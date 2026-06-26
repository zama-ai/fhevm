// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {OAppReceiverUpgradeable, Origin} from "@layerzerolabs/oapp-evm-upgradeable/contracts/oapp/OAppReceiverUpgradeable.sol";
import {ILayerZeroComposer} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroComposer.sol";

import {ACL} from "../ACL.sol";
import {aclAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {ACLOwnable} from "../shared/ACLOwnable.sol";
import {HANDLE_VERSION} from "../shared/Constants.sol";
import {FheType} from "../shared/FheType.sol";
import {BridgeEvents} from "./BridgeEvents.sol";
import {IDstApp} from "./interfaces/IDstApp.sol";

/**
 * @title HandlesReceiver
 * @notice Destination-side mixin for confidential handle bridging: LayerZero V2 OApp
 *         + composer that acts as the destination-chain trust anchor.
 *
 *         Operation is split across two transactions to keep bridge state independent
 *         of app-level outcomes:
 *
 *         1. `_lzReceive` (inbound LZ message handling): derives a new destination
 *            handle for each source handle in the message, emits one `HandleBridged`
 *            event per handle (the destination-side trust anchor), and forwards the
 *            payload + handle lists to itself via `sendCompose`. No persistent storage
 *            is written between `lzReceive` and `lzCompose`.
 *
 *         2. `lzCompose` (a separate transaction with the executor as caller): grants
 *            transient ACL allowance to the destination app for each derived handle,
 *            then calls `DstApp.onConfidentialBridgeReceived(...)`. If the app reverts, the lzCompose
 *            transaction reverts and the transient grants do not land — but the bridge
 *            state from step 1 is already committed, and the coprocessor's association
 *            is unaffected. lzCompose could be retried independently.
 *
 * @dev    Handle derivation uses
 *         `DstHandle = Hash(domain_sep, srcHandle, aclAdd, dstChainId, prevBlockHash, timestamp)`
 *         followed by metadata embedding (chain id, FheType byte from srcHandle, and
 *         the destination chain's HANDLE_VERSION).
 *
 * @dev    Abstract: the {ConfidentialBridge} concrete contract derives from this and
 *         from {HandlesSender}, and is the only deployed contract. The OApp endpoint
 *         and ownership are initialized by the derived constructor — this contract
 *         intentionally provides none.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
abstract contract HandlesReceiver is OAppReceiverUpgradeable, ILayerZeroComposer, ACLOwnable, BridgeEvents {
    /// @notice Returned when `lzCompose` is invoked by an unauthorized caller.
    error NotLzEndpoint(address caller);

    /// @notice Returned when the compose message claims a `from` address other than this contract.
    error UnexpectedComposeOrigin(address from);

    /// @notice Domain separator for destination handle derivation.
    bytes8 private constant BRIDGE_DERIVATION_DOMAIN_SEPARATOR = "FHE_brdg";

    /// @notice ACL contract on this (destination) chain.
    ACL private constant ACL_CONTRACT = ACL(aclAdd);

    error WrongChainIdInDstHandle();

    /// @notice Returned when byte 21 of `dstHandle` is not the `0xff` computation marker.
    error WrongIndexByteInDstHandle();

    /// @notice Returned when byte 31 of `dstHandle` does not match this chain's `HANDLE_VERSION`.
    error WrongHandleVersionInDstHandle();

    /// @notice Returned when byte 30 of `dstHandle` decodes to an `FheType` outside the
    ///         set supported by `grantFallbackPlaintext`.
    error UnsupportedFheTypeInDstHandle();

    /// @notice Returned when `plaintext` does not fit in the bit width implied by the
    ///         `FheType` encoded in `dstHandle`.
    error PlaintextOutOfRange();

    /// @notice OApp version tuple. HandlesReceiver is receive-only: sender side is `0`.
    /// @dev    Virtual so the combined {ConfidentialBridge} can return `(1, 2)`.
    function oAppVersion() public pure virtual override returns (uint64 senderVersion, uint64 receiverVersion) {
        return (0, 2);
    }

    /**
     * @notice Authorizes the coprocessor to associate the trivial encryption of `plaintext`
     *         with `dstHandle` as a fallback, when the normal pair of bridge events did not
     *         settle (missed/invalid event, or missing source ciphertext).
     *
     * @dev    This grants a permission, not an assertion: if a node already has a real
     *         association for `dstHandle` (from a matched pair of bridge events), it
     *         keeps that. Otherwise, it may use the trivial encryption of `plaintext`.
     *         Coprocessor nodes then run consensus on `dstHandle` and settle on the
     *         majority ciphertext. Only affects this destination chain.
     * @dev    Assumes the ACL owner would never call this method twice with the same
     *         `dstHandle`; if it does, the coprocessor treats only the first
     *         `FallbackGrantedPlaintext` event as the source of truth.
     */
    function grantFallbackPlaintext(bytes32 dstHandle, uint256 plaintext) external onlyACLOwner {
        // Bytes 22-29 must encode this chain's id (matches `_appendMetadataToPrehandle`).
        uint256 extractedChainId = uint256(
            dstHandle & 0x00000000000000000000000000000000000000000000ffffffffffffffff0000
        ) >> 16;
        if (extractedChainId != block.chainid) revert WrongChainIdInDstHandle();

        // Byte 21 is the index/marker byte; bridged handles always set it to 0xff.
        if (uint8(dstHandle[21]) != 0xff) revert WrongIndexByteInDstHandle();

        // Byte 31 carries the destination chain's HANDLE_VERSION.
        if (uint8(dstHandle[31]) != HANDLE_VERSION) revert WrongHandleVersionInDstHandle();

        // Byte 30 is the FheType.
        FheType fheType = FheType(uint8(dstHandle[30]));
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint160)) +
            (1 << uint8(FheType.Uint256));
        if ((1 << uint8(fheType)) & supportedTypes == 0) revert UnsupportedFheTypeInDstHandle();

        // Plaintext must fit in the FheType's bit width.
        _checkPlaintextFits(plaintext, fheType);

        emit FallbackGrantedPlaintext(dstHandle, plaintext);
    }

    /// @dev Reverts if `plaintext` cannot be represented in the bit width of `fheType`.
    ///      Caller is responsible for ensuring `fheType` is in the supported allowlist.
    function _checkPlaintextFits(uint256 plaintext, FheType fheType) internal pure virtual {
        if (fheType == FheType.Bool) {
            if (plaintext > 1) revert PlaintextOutOfRange();
        } else if (fheType == FheType.Uint8) {
            if (plaintext > type(uint8).max) revert PlaintextOutOfRange();
        } else if (fheType == FheType.Uint16) {
            if (plaintext > type(uint16).max) revert PlaintextOutOfRange();
        } else if (fheType == FheType.Uint32) {
            if (plaintext > type(uint32).max) revert PlaintextOutOfRange();
        } else if (fheType == FheType.Uint64) {
            if (plaintext > type(uint64).max) revert PlaintextOutOfRange();
        } else if (fheType == FheType.Uint128) {
            if (plaintext > type(uint128).max) revert PlaintextOutOfRange();
        } else if (fheType == FheType.Uint160) {
            if (plaintext > type(uint160).max) revert PlaintextOutOfRange();
        }
        // Uint256: no upper-bound check needed (uint256 is the wire type).
    }

    /**
     * @notice LayerZero compose entry-point. Invoked by the executor in a separate
     *         transaction after `_lzReceive` has dispatched a compose message.
     *
     * @dev    Authenticates: msg.sender == LayerZero endpoint, and the compose
     *         message originated from this contract (`from == address(this)`). Then
     *         grants transient ACL allowance for each derived handle to the
     *         destination app and invokes its `onConfidentialBridgeReceived` callback, forwarding the
     *         LayerZero `guid` so the app can correlate the delivery with the source send.
     *
     *         Conforms to ILayerZeroComposer; unused parameters (`executor`, `extraData`)
     *         are still part of the interface.
     */
    function lzCompose(
        address from,
        bytes32 guid,
        bytes calldata message,
        address /* executor */,
        bytes calldata /* extraData */
    ) external payable override {
        if (msg.sender != address(endpoint)) revert NotLzEndpoint(msg.sender);
        if (from != address(this)) revert UnexpectedComposeOrigin(from);

        // Decode + grant + dispatch in a separate frame: threading the LayerZero `guid`
        // through to `onConfidentialBridgeReceived` alongside the six decoded fields trips the without-via-ir
        // 16-slot stack limit if kept inline.
        _grantAndDispatch(message, guid);
    }

    /// @dev Decodes the compose message, grants transient ACL allowance for each derived
    ///      handle to the destination app, and invokes its `onConfidentialBridgeReceived` with the LayerZero
    ///      `guid`. Extracted from {lzCompose} to keep the calling frame within the
    ///      16-slot stack limit on without-via-ir builds.
    function _grantAndDispatch(bytes calldata message, bytes32 guid) internal virtual {
        // Wire format: both srcApp and dstApp are bytes32 (see HandlesSender._dispatch
        // for the rationale). On EVM, srcApp is a zero-padded address; dstApp must
        // also fit in 20 bytes for the local IDstApp dispatch — non-EVM destinations
        // never reach this lzCompose path because they run their own (non-Solidity)
        // bridge implementation.
        (
            uint32 srcEid,
            bytes32 srcApp,
            bytes32 dstApp,
            bytes memory payload,
            bytes32[] memory srcHandleList,
            bytes32[] memory dstHandleList
        ) = abi.decode(message, (uint32, bytes32, bytes32, bytes, bytes32[], bytes32[]));

        address dstAppEvm = address(uint160(uint256(dstApp)));
        uint256 nHandles = dstHandleList.length;
        for (uint256 i = 0; i < nHandles; i++) {
            ACL_CONTRACT.allowTransient(dstHandleList[i], dstAppEvm);
        }

        IDstApp(dstAppEvm).onConfidentialBridgeReceived(srcEid, srcApp, payload, srcHandleList, dstHandleList, guid);
    }

    /**
     * @dev Inbound LayerZero message handler. Derives destination handles, emits
     *      `HandleBridged` events, and dispatches a compose message back to self.
     */
    function _lzReceive(
        Origin calldata origin,
        bytes32 guid,
        bytes calldata message,
        address /* executor */,
        bytes calldata /* extraData */
    ) internal override {
        _handleInbound(origin.srcEid, guid, message);
    }

    /**
     * @dev Inbound flow: decode message, derive destination handles, emit events,
     *      dispatch compose-to-self with the augmented payload.
     */
    function _handleInbound(uint32 srcEid, bytes32 guid, bytes calldata message) internal virtual {
        // Wire format mirrors HandlesSender._dispatch: srcApp+dstApp as bytes32.
        (bytes32 srcApp, bytes32 dstApp, bytes memory payload, bytes32[] memory srcHandleList) = abi.decode(
            message,
            (bytes32, bytes32, bytes, bytes32[])
        );

        bytes32[] memory dstHandleList = _deriveAndEmit(dstApp, srcHandleList, guid);

        // Dispatch a compose message back to self. The HandlesReceiver is also the
        // compose target, configured via LayerZero options on the source side.
        endpoint.sendCompose(
            address(this),
            guid,
            0,
            abi.encode(srcEid, srcApp, dstApp, payload, srcHandleList, dstHandleList)
        );
    }

    /**
     * @dev Per-handle derivation + event emission. Extracted from `_lzReceive` to
     *      keep the calling frame's stack within the 16-slot limit (the without-via-ir
     *      build trips stack-too-deep when this is inlined).
     */
    function _deriveAndEmit(
        bytes32 dstApp,
        bytes32[] memory srcHandleList,
        bytes32 guid
    ) internal virtual returns (bytes32[] memory dstHandleList) {
        uint256 n = srcHandleList.length;
        dstHandleList = new bytes32[](n);
        bytes32 prevBlockHash = blockhash(block.number - 1);
        // HandleBridged.receiverDapp is an EVM-local address (the event fires on this
        // chain). Convert from the bytes32 wire field for the emit.
        address dstAppEvm = address(uint160(uint256(dstApp)));
        for (uint256 i = 0; i < n; i++) {
            bytes32 srcHandle = srcHandleList[i];
            bytes32 dstHandle = _deriveDstHandle(srcHandle, prevBlockHash);
            dstHandleList[i] = dstHandle;
            emit HandleBridged(dstAppEvm, srcHandle, dstHandle, guid);
        }
    }

    /**
     * @dev Derives the destination handle
     *      then embeds metadata: chain id (this chain) in bytes 22-29, FheType from the
     *      source handle in byte 30 (so the handle type is preserved across the bridge),
     *      and HANDLE_VERSION in byte 31.
     *
     *      The 0xff byte at byte 21 marks the handle as not originating from input
     *      verification (matches the FHEVMExecutor's computation-handle convention),
     *      distinguishing bridged handles from user-input handles on this chain.
     */
    function _deriveDstHandle(bytes32 srcHandle, bytes32 prevBlockHash) internal view returns (bytes32 result) {
        result = keccak256(
            abi.encodePacked(
                BRIDGE_DERIVATION_DOMAIN_SEPARATOR,
                srcHandle,
                ACL_CONTRACT,
                block.chainid,
                prevBlockHash,
                block.timestamp
            )
        );

        // Clear bytes 21-31 in preparation for metadata embedding.
        result = result & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        // Byte 21 = 0xff for non-input (i.e. computation) marker, matches FHEVMExecutor pattern.
        result = result | (bytes32(uint256(0xff)) << 80);
        // Bytes 22-29 = chain id of this (destination) chain.
        result = result | (bytes32(uint256(uint64(block.chainid))) << 16);
        // Byte 30 = FheType byte copied from the source handle (preserves the type).
        uint256 fheTypeByte = uint256(uint8(srcHandle[30]));
        result = result | (bytes32(fheTypeByte) << 8);
        // Byte 31 = HANDLE_VERSION on this chain.
        result = result | bytes32(uint256(HANDLE_VERSION));
    }
}
