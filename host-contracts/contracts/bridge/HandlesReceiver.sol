// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {OAppReceiver, Origin} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/OAppReceiver.sol";
import {ILayerZeroComposer} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroComposer.sol";

import {ACL} from "../ACL.sol";
import {aclAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {HANDLE_VERSION} from "../shared/Constants.sol";
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
 *            then calls `DstApp.onReceive(...)`. If the app reverts, the lzCompose
 *            transaction reverts and the transient grants do not land — but the bridge
 *            state from step 1 is already committed, and the coprocessor's association
 *            is unaffected. LayerZero retries lzCompose independently.
 *
 * @dev    Handle derivation uses
 *         `DstHandle = Hash(domain_sep, srcHandle, dstChainId, prevBlockHash, guid)`
 *         followed by metadata embedding (chain id, FheType byte from srcHandle, and
 *         the destination chain's HANDLE_VERSION). Including `prevBlockHash` limits
 *         an adversary's collision-search window on the destination chain to a single
 *         block (see RFC 008 §Security Considerations — Colliding handles).
 *
 * @dev    Abstract: the {ConfidentialBridge} concrete contract derives from this and
 *         from {HandlesSender}, and is the only deployed artifact. The OApp endpoint
 *         and ownership are initialized by the derived constructor — this contract
 *         intentionally provides none.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
abstract contract HandlesReceiver is OAppReceiver, ILayerZeroComposer, BridgeEvents {
    /// @notice Returned when `lzCompose` is invoked by an unauthorized caller.
    error NotLzEndpoint(address caller);

    error WrongChainId();

    /// @notice Returned when the compose message claims a `from` address other than this contract.
    /// @dev    Defense in depth: only HandlesReceiver itself dispatches compose messages.
    error UnexpectedComposeOrigin(address from);

    /// @notice Domain separator for destination handle derivation.
    bytes8 private constant BRIDGE_DERIVATION_DOMAIN_SEPARATOR = "FHE_brdg";

    /// @notice ACL contract on this (destination) chain.
    ACL private constant ACL_CONTRACT = ACL(aclAdd);

    error WrongChainIdInDstHandle();

    /// @notice OApp version tuple. HandlesReceiver is receive-only: sender side is `0`.
    /// @dev    Virtual so the combined {ConfidentialBridge} can return `(1, 2)`.
    function oAppVersion() public pure virtual override returns (uint64 senderVersion, uint64 receiverVersion) {
        return (0, 2);
    }

    /**
     * @notice Authorizes the coprocessor to associate `ciphertextHash`'s ciphertext with
     *         `dstHandle` as a fallback, when the normal pair of bridge events did not
     *         settle (missed/invalid event, or missing source ciphertext).
     *
     * @dev    This grants a permission, not an assertion: if a node already has a real
     *         association for `dstHandle` (from a matched pair of bridge events), it
     *         keeps that. Otherwise, it may use the ciphertext matching `ciphertextHash`.
     *         Coprocessor nodes then run consensus on `dstHandle` and settle on the
     *         majority ciphertext. Only affects this destination chain.
     * @dev    Assumes owner would never call this method twice with same `dstHandle`,
     *         otherwise in case of such error, coprocessor should consider only first
     *         `FallbackGrantedPlainText` event is source of truth.
     */
    function grantFallbackPlainText(bytes32 dstHandle, uint256 plainText) external onlyOwner {
        uint256 extractedChainId = uint256(
            dstHandle & 0x00000000000000000000000000000000000000000000ffffffffffffffff0000
        ) >> 16;
        if (extractedChainId != block.chainid) revert WrongChainIdInDstHandle();
        // TODO: add other checks on dstHandle and plainText, such as index byte, version, range of cleartext, fheType validity.
        emit FallbackGrantedPlainText(dstHandle, plainText);
    }

    /**
     * @notice LayerZero compose entry-point. Invoked by the executor in a separate
     *         transaction after `_lzReceive` has dispatched a compose message.
     *
     * @dev    Authenticates: msg.sender == LayerZero endpoint, and the compose
     *         message originated from this contract (`from == address(this)`). Then
     *         grants transient ACL allowance for each derived handle to the
     *         destination app and invokes its `onReceive` callback.
     *
     *         Conforms to ILayerZeroComposer; unused parameters (`guid`, `executor`,
     *         `extraData`) are still part of the interface.
     */
    function lzCompose(
        address from,
        bytes32 /* guid */,
        bytes calldata message,
        address /* executor */,
        bytes calldata /* extraData */
    ) external payable override {
        if (msg.sender != address(endpoint)) revert NotLzEndpoint(msg.sender);
        if (from != address(this)) revert UnexpectedComposeOrigin(from);

        (
            uint32 srcEid,
            address srcApp,
            address dstApp,
            bytes memory payload,
            bytes32[] memory srcHandleList,
            bytes32[] memory dstHandleList
        ) = abi.decode(message, (uint32, address, address, bytes, bytes32[], bytes32[]));

        uint256 nHandles = dstHandleList.length;
        for (uint256 i = 0; i < nHandles; i++) {
            ACL_CONTRACT.allowTransient(dstHandleList[i], dstApp);
        }

        IDstApp(dstApp).onReceive(srcEid, srcApp, payload, srcHandleList, dstHandleList);
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
        // Thin forwarder so `_lzReceive`'s frame holds only the three primitives needed
        // by the inbound handler. Inlining the decode + sendCompose here trips
        // stack-too-deep when compiling without --via-ir.
        _handleInbound(origin.srcEid, guid, message);
    }

    /**
     * @dev Inbound flow: decode message, derive destination handles, emit events,
     *      dispatch compose-to-self with the augmented payload.
     */
    function _handleInbound(uint32 srcEid, bytes32 guid, bytes calldata message) private {
        (address srcApp, address dstApp, bytes memory payload, bytes32[] memory srcHandleList) = abi.decode(
            message,
            (address, address, bytes, bytes32[])
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
        address dstApp,
        bytes32[] memory srcHandleList,
        bytes32 guid
    ) private returns (bytes32[] memory dstHandleList) {
        uint256 n = srcHandleList.length;
        dstHandleList = new bytes32[](n);
        bytes32 prevBlockHash = blockhash(block.number - 1);
        for (uint256 i = 0; i < n; i++) {
            bytes32 srcHandle = srcHandleList[i];
            bytes32 dstHandle = _deriveDstHandle(srcHandle, prevBlockHash, guid);
            dstHandleList[i] = dstHandle;
            emit HandleBridged(dstApp, srcHandle, dstHandle, guid);
        }
    }

    /**
     * @dev Derives the destination handle as
     *      Hash(BRIDGE_DERIVATION_DOMAIN_SEPARATOR, srcHandle, dstChainId, prevBlockHash, guid),
     *      then embeds metadata: chain id (this chain) in bytes 22-29, FheType from the
     *      source handle in byte 30 (so the handle type is preserved across the bridge),
     *      and HANDLE_VERSION in byte 31.
     *
     *      The 0xff byte at byte 21 marks the handle as not originating from input
     *      verification (matches the FHEVMExecutor's computation-handle convention),
     *      distinguishing bridged handles from user-input handles on this chain.
     */
    function _deriveDstHandle(
        bytes32 srcHandle,
        bytes32 prevBlockHash,
        bytes32 guid
    ) internal view returns (bytes32 result) {
        result = keccak256(
            abi.encodePacked(BRIDGE_DERIVATION_DOMAIN_SEPARATOR, srcHandle, block.chainid, prevBlockHash, guid)
        );

        // Clear bytes 21-31 in preparation for metadata embedding.
        result = result & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        // Byte 21 = 0xff (non-input/computation marker, matches FHEVMExecutor pattern).
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
