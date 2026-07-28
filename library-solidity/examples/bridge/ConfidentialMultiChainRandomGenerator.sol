// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

import "../../lib/FHE.sol";
import {CoprocessorSetup} from "../CoprocessorSetup.sol";

import {ConfidentialOApp} from "../../lib/bridge/ConfidentialOApp.sol";

/**
 * @title ConfidentialMultiChainRandomGenerator
 * @notice Minimal example app, built on top of the {ConfidentialOApp} base (a LayerZero-enabled
 *         ConfidentialBridge omnichain app), that bridges a list of FHE handles from one chain to
 *         another. To keep usage simple, the caller does not have to supply (and pre-authorize)
 *         the handles: the contract mints `countHandles` fresh encrypted values on-chain via
 *         `FHE.randEuint32`, grants itself ACL allowance on each, and bridges that freshly
 *         generated list.
 *
 * @dev    An instance is deployed on each supported chain and wired with its remote peers via
 *         {ConfidentialOAppCore-setPeer}. Because every instance embeds both the send path
 *         ({generateAndSendHandlesList}) and the receive path (inherited
 *         {ConfidentialOAppReceiver-onConfidentialBridgeReceived}, which dispatches to
 *         {_onReceiveHandles}), the same deployment can bridge in both directions:
 *         chain A → chain B and chain B → chain A.
 *
 *         Authorization model:
 *         - Outbound: the ConfidentialBridge enforces that *this contract* holds ACL allowance on
 *           every bridged handle. The handles are generated here and immediately `FHE.allowThis`'d,
 *           so the check always passes — no caller-side setup is required. The owner is also
 *           granted ACL allowance on every handle, so they can later `userDecrypt` them.
 *         - Inbound: {ConfidentialOAppReceiver} only accepts calls from the local ConfidentialBridge
 *           and only from a `(srcEid, srcApp)` pair the owner has registered as a peer. The owner is
 *           granted ACL allowance on every received handle so they can later `userDecrypt` them.
 */
contract ConfidentialMultiChainRandomGenerator is ConfidentialOApp {
    /// @notice Emitted on a successful outbound bridge, carrying the encrypted handles sent.
    event HandlesListSent(uint32 indexed dstEid, bytes32 indexed dstApp, bytes32[] handlesListSent, bytes32 guid);

    /// @notice Emitted when a bridged list of handles has been received and recorded,
    ///         carrying the destination-chain handles and the originating LayerZero GUID.
    event HandlesListReceived(
        uint32 indexed srcEid,
        bytes32 indexed srcApp,
        euint32[] handlesListReceived,
        bytes32 guid
    );

    /// @notice A non-zero handle count is required to bridge.
    error EmptyHandleList();

    /// @dev Per-delivery commitment, keyed by the LayerZero GUID of the inbound message.
    ///      Stores `keccak256(abi.encode(srcHandleList, dstHandleList, payload))` instead
    ///      of the full arrays/payload, so the receive hook only writes one 32-byte slot
    ///      per delivery. This keeps the destination-side `lzCompose` cost (and therefore
    ///      the required `lzComposeGas`) to a minimum.
    mapping(bytes32 guid => bytes32 resultHash) public resultBridgedHash;

    /**
     * @param _owner Initial owner (can configure peers).
     */
    constructor(address _owner) Ownable(_owner) {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    }

    /**
     * @notice Generate `countHandles` fresh encrypted values and bridge their handles to
     *         the peer registered for `dstEid`.
     * @dev    Each handle is produced by `FHE.randEuint32` and granted ACL allowance to
     *         this contract via `FHE.allowThis` (and to the app `owner()`), so the
     *         ConfidentialBridge's `isAllowed(handle, this)` check passes without any
     *         caller-side setup. Bridging to an eid with no configured peer reverts with
     *         {IConfidentialOAppCore-NoPeer}. Pass exactly the quoted native fee as `msg.value`
     *         to cover the LayerZero native fee (see {quoteGenerateAndSendHandlesList}).
     *         `customPayload` is opaque, app-defined data forwarded verbatim to the peer's
     *         receive hook on the destination chain; this contract does not interpret it.
     * @param dstEid        LayerZero endpoint id of the destination chain.
     * @param countHandles  Number of random handles to generate and bridge. Capped by the
     *                      bridge's `MAX_HANDLES` (set to 32 by default).
     * @param customPayload Opaque app-level payload delivered to the destination peer; not
     *                      interpreted here. Its length contributes to the message fee.
     * @param lzComposeGas  Gas budget for the destination-side `lzCompose` ({_onReceiveHandles}).
     * @return guid         The LayerZero message guid (also emitted in {HandlesListSent}).
     * @return nonce        The LayerZero message nonce.
     */
    function generateAndSendHandlesList(
        uint32 dstEid,
        uint256 countHandles,
        bytes memory customPayload,
        uint64 lzComposeGas
    ) external payable returns (bytes32 guid, uint64 nonce) {
        if (countHandles == 0) revert EmptyHandleList();

        bytes32[] memory handleList = _generateHandles(countHandles);

        // The peer is resolved (and `NoPeer`-checked) internally; the bridge builds
        // execution options from its lzReceiveGas formula (sized by handle count and payload length) and `lzComposeGas`.
        (guid, nonce) = _sendHandlesToPeer(dstEid, customPayload, handleList, lzComposeGas);

        emit HandlesListSent(dstEid, _getPeerOrRevert(dstEid), handleList, guid);
    }

    /**
     * @notice Quote the LayerZero native fee for a {generateAndSendHandlesList} call,
     *         without sending.
     * @dev    The fee depends only on the message size and `lzComposeGas` — a function of
     *         `countHandles` and the `customPayload` length (not the handles nor the payload
     *         values) — so this view can quote without actually generating handles. Reverts
     *         with {IConfidentialOAppCore-NoPeer} if no peer is configured for `dstEid`.
     * @param dstEid        LayerZero endpoint id of the destination chain.
     * @param countHandles  Number of handles the matching {generateAndSendHandlesList} call
     *                      would bridge.
     * @param customPayload The same opaque payload that would be passed to
     *                      {generateAndSendHandlesList}; its length determines the fee.
     * @param lzComposeGas  Gas budget for the destination-side `lzCompose`.
     * @return nativeFee    The LayerZero native fee to forward as `msg.value`.
     */
    function quoteGenerateAndSendHandlesList(
        uint32 dstEid,
        uint256 countHandles,
        bytes memory customPayload,
        uint64 lzComposeGas
    ) external view returns (uint256 nativeFee) {
        // Mirror the real send: the fee is measured by `customPayload` and the number of
        // handles. `_quoteSendHandlesToPeer` builds a matching list of null placeholder
        // handles internally, pricing identically to the real call.
        nativeFee = _quoteSendHandlesToPeer(dstEid, customPayload, countHandles, lzComposeGas);
    }

    /**
     * @notice Record a received list of handles once {ConfidentialOAppReceiver} has authenticated
     *         the inbound bridge message (trusted local ConfidentialBridge caller and trusted
     *         `(srcEid, srcApp)` peer). The bridge has already granted this contract transient ACL
     *         allowance on every entry of `dstHandleList`.
     * @dev    The `payload` is opaque, app-defined data forwarded by the source peer (see
     *         {generateAndSendHandlesList}); it is not interpreted here and is only folded into the
     *         per-delivery commitment stored in {resultBridgedHash} (keyed by `guid`). For every
     *         derived handle we grant *persistent* ACL allowance to both this contract
     *         (`FHE.allowThis`) and the app `owner()` (`FHE.allow`), so the owner can later
     *         `userDecrypt` the destination handles.
     */
    function _onReceiveHandles(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata srcHandleList,
        bytes32[] calldata dstHandleList,
        bytes32 guid
    ) internal override {
        // Commit to the delivery with a single SSTORE rather than persisting the full
        // arrays/payload, keeping the lzCompose gas cost low. The pre-image is available
        // off-chain via the `HandlesListReceived` event.
        resultBridgedHash[guid] = keccak256(abi.encode(srcHandleList, dstHandleList, payload));

        // Re-type the received destination handles and grant persistent decryption rights
        // to this contract and the initiating user. These handles are usable on this
        // chain and have already been transiently ACL-allowed to this contract by the
        // bridge, which is what lets us re-grant them here.
        uint256 n = dstHandleList.length;
        euint32[] memory handlesListReceived = new euint32[](n);
        for (uint256 i = 0; i < n; i++) {
            euint32 dstHandle = euint32.wrap(dstHandleList[i]);
            FHE.allowThis(dstHandle);
            FHE.allow(dstHandle, owner());
            handlesListReceived[i] = dstHandle;
        }

        emit HandlesListReceived(srcEid, srcApp, handlesListReceived, guid);
    }

    /// @dev Mint `count` random encrypted 32-bit values, granting persistent ACL
    ///      allowance on each to this contract and to the app `owner()`, so that the owner
    ///      could later `userDecrypt` them. Returns the handles in their raw `bytes32` form
    ///      for the bridge `send` call.
    function _generateHandles(uint256 count) internal returns (bytes32[] memory handleList) {
        handleList = new bytes32[](count);
        for (uint256 i = 0; i < count; i++) {
            euint32 value = FHE.randEuint32();
            FHE.allowThis(value);
            FHE.allow(value, owner());
            handleList[i] = euint32.unwrap(value);
        }
    }
}
