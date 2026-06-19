// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Ownable, Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {MessagingFee, MessagingReceipt} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";

import "../../lib/FHE.sol";
import {CoprocessorSetup} from "../../lib/CoprocessorSetup.sol";

import {ConfidentialBridge} from "../../contracts/bridge/ConfidentialBridge.sol";
import {IDstApp} from "../../contracts/bridge/interfaces/IDstApp.sol";

/**
 * @title HandlesListConfidentialOApp
 * @notice Minimal example app that bridges a list of FHE handles from one chain to
 *         another using the {ConfidentialBridge}. To keep usage simple, the caller does
 *         not have to supply (and pre-authorize) the handles: the contract mints
 *         `countHandles` fresh encrypted values on-chain via `FHE.randEuint32`, grants
 *         itself ACL allowance on each, and bridges that freshly generated list.
 *
 * @dev    An instance is deployed on each supported chain and wired with its remote
 *         peers via {setPeer}. Because every instance embeds both the send path
 *         (`generateAndSendHandlesList`) and the receive path
 *         (`onConfidentialBridgeReceived`), the same deployment can bridge in both
 *         directions: chain A → chain B and chain B → chain A.
 *
 *         Authorization model:
 *         - Outbound: the {ConfidentialBridge} enforces that *this contract* holds ACL
 *           allowance on every bridged handle (the bridge checks
 *           `isAllowed(handle, msg.sender)` and `msg.sender` is this contract). The
 *           handles are generated here and immediately `FHE.allowThis`'d, so the check
 *           always passes — no caller-side setup is required.
 *         - Inbound: `onConfidentialBridgeReceived` only accepts calls from the local {ConfidentialBridge}
 *           and only from a `(srcEid, srcApp)` pair the owner has registered as a peer.
 */
contract HandlesListConfidentialOApp is Ownable2Step, IDstApp {
    /// @notice Emitted on a successful outbound bridge, carrying the encrypted handles sent.
    event HandlesListConfidentialOAppSent(
        uint32 indexed dstEid,
        bytes32 indexed dstApp,
        euint32[] handlesListSent,
        bytes32 guid
    );

    /// @notice Emitted when a bridged list of handles has been received and recorded,
    ///         carrying the destination-chain handles and the originating LayerZero GUID.
    event HandlesListConfidentialOAppReceived(
        uint32 indexed srcEid,
        bytes32 indexed srcApp,
        euint32[] handlesListReceived,
        bytes32 guid
    );

    /// @notice Emitted when a remote peer is configured (or cleared) for an eid.
    event PeerSet(uint32 indexed eid, bytes32 indexed peer);

    /// @notice Inbound `(srcEid, srcApp)` does not match the registered peer.
    error UntrustedPeer(uint32 srcEid, bytes32 srcApp);

    /// @notice No peer configured for the requested destination eid.
    error PeerNotSet(uint32 dstEid);

    /// @notice `onConfidentialBridgeReceived` caller is not the local {ConfidentialBridge}.
    error OnlyConfidentialBridge(address caller);

    /// @notice A non-zero handle count is required to bridge.
    error EmptyHandleList();

    /// @notice ConfidentialBridge on this chain. Used both to dispatch outbound sends and
    ///         to authenticate inbound `onConfidentialBridgeReceived` calls (the bridge is its own lzCompose
    ///         dispatcher, so the bridge address is the only authorized `onConfidentialBridgeReceived` caller).
    ConfidentialBridge public immutable confidentialBridge;

    /// @dev Canonical peer app on each remote chain, keyed by eid. A single peer per eid
    ///      serves both directions: outbound `generateAndSendHandlesList` dispatches to
    ///      `_peers[dstEid]` and inbound `onConfidentialBridgeReceived` rejects any
    ///      `(srcEid, srcApp)` that doesn't match
    ///      `_peers[srcEid]`. Stored as `bytes32` to support non-EVM peers; for EVM peers
    ///      pass `bytes32(uint256(uint160(remoteAddress)))`.
    mapping(uint32 eid => bytes32 peer) private _peers;

    /// @dev The last list of source-chain handles received via `onConfidentialBridgeReceived` (opaque).
    bytes32[] private _lastReceivedSrcHandleList;

    /// @dev The last list of destination-chain handles received via `onConfidentialBridgeReceived`. These are
    ///      usable on this chain and have been ACL-allowed to this contract by the bridge.
    bytes32[] private _lastReceivedDstHandleList;

    /// @dev The last app-level payload received via `onConfidentialBridgeReceived`.
    bytes private _lastReceivedPayload;

    constructor(address _confidentialBridge, address _owner) Ownable(_owner) {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
        confidentialBridge = ConfidentialBridge(_confidentialBridge);
    }

    /**
     * @notice Generate `countHandles` fresh encrypted values and bridge their handles to
     *         the peer registered for `dstEid`.
     * @dev    Each handle is produced by `FHE.randEuint32` and granted ACL allowance to
     *         this contract via `FHE.allowThis`, so the {ConfidentialBridge}'s
     *         `isAllowed(handle, this)` check passes without any caller-side setup. Pass
     *         enough `msg.value` to cover the LayerZero native fee
     *         (see {quoteGenerateAndSendHandlesList}).
     *
     *         The bridge payload is the abi-encoded `msg.sender`: the destination instance
     *         decodes it in {onConfidentialBridgeReceived} and re-grants decryption rights
     *         on the derived handles to that original caller, so the initiating user can
     *         later `userDecrypt` the destination handles.
     * @param dstEid        LayerZero endpoint id of the destination chain.
     * @param countHandles  Number of random handles to generate and bridge. Capped by the
     *                      bridge's `MAX_HANDLES`.
     * @param lzComposeGas  Gas budget for the destination-side `lzCompose`
     *                      (`onConfidentialBridgeReceived`).
     * @return receipt      LayerZero messaging receipt (includes the GUID used in events).
     */
    function generateAndSendHandlesList(
        uint32 dstEid,
        uint256 countHandles,
        uint128 lzComposeGas
    ) external payable returns (MessagingReceipt memory receipt) {
        if (countHandles == 0) revert EmptyHandleList();

        bytes32 dstApp = _peers[dstEid];
        if (dstApp == bytes32(0)) revert PeerNotSet(dstEid);

        (euint32[] memory handles, bytes32[] memory handleList) = _generateHandles(countHandles);

        // Carry the initiating user so the destination instance can re-grant them
        // decryption rights on the derived handles (see onConfidentialBridgeReceived).
        bytes memory payload = abi.encode(msg.sender);

        // Empty `options` lets the bridge build defaults from its lzReceiveGas formula
        // (sized by handle count) and `lzComposeGas`.
        receipt = confidentialBridge.send{value: msg.value}(dstEid, dstApp, payload, handleList, lzComposeGas, "");

        emit HandlesListConfidentialOAppSent(dstEid, dstApp, handles, receipt.guid);
    }

    /**
     * @notice Quote the LayerZero native fee for a {generateAndSendHandlesList} call,
     *         without sending.
     * @dev    The fee depends only on the message size and options, both of which are a
     *         function of `countHandles` (not the handle values), so this view can quote
     *         without actually generating handles. Reverts with {PeerNotSet} when no peer
     *         is configured for `dstEid`.
     */
    function quoteGenerateAndSendHandlesList(
        uint32 dstEid,
        uint256 countHandles,
        uint128 lzComposeGas
    ) external view returns (MessagingFee memory fee) {
        bytes32 dstApp = _peers[dstEid];
        if (dstApp == bytes32(0)) revert PeerNotSet(dstEid);
        // Mirror the real send: the payload is the abi-encoded caller (32 bytes) and the
        // message is otherwise measured by `handleList.length`. A placeholder address and
        // zero handles of the right length quote identically to the real call.
        bytes memory payload = abi.encode(msg.sender);
        bytes32[] memory placeholder = new bytes32[](countHandles);
        fee = confidentialBridge.quote(dstEid, address(this), dstApp, payload, placeholder, lzComposeGas, "");
    }

    /**
     * @notice ConfidentialBridge dispatches here in lzCompose with the derived handles.
     * @dev    Authentication: caller must be the local {ConfidentialBridge}, and
     *         `(srcEid, srcApp)` must match a peer registered via {setPeer}. The bridge
     *         has already granted this contract transient ACL allowance on every entry
     *         of `dstHandleList`.
     *
     *         The `payload` is the abi-encoded address of the user who initiated the
     *         transfer on the source chain (see {generateAndSendHandlesList}). For every
     *         derived handle we grant *persistent* ACL allowance to both this contract
     *         (`FHE.allowThis`) and that user (`FHE.allow`), so the user can later
     *         `userDecrypt` the destination handles.
     */
    function onConfidentialBridgeReceived(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata srcHandleList,
        bytes32[] calldata dstHandleList,
        bytes32 guid
    ) external override {
        if (msg.sender != address(confidentialBridge)) revert OnlyConfidentialBridge(msg.sender);
        bytes32 trustedPeer = _peers[srcEid];
        if (trustedPeer == bytes32(0) || trustedPeer != srcApp) revert UntrustedPeer(srcEid, srcApp);

        address originalSender = abi.decode(payload, (address));

        _lastReceivedSrcHandleList = srcHandleList;
        _lastReceivedDstHandleList = dstHandleList;
        _lastReceivedPayload = payload;

        // Re-type the received destination handles and grant persistent decryption rights
        // to this contract and the initiating user. These handles are usable on this
        // chain and have already been transiently ACL-allowed to this contract by the
        // bridge, which is what lets us re-grant them here.
        uint256 n = dstHandleList.length;
        euint32[] memory handlesListReceived = new euint32[](n);
        for (uint256 i = 0; i < n; i++) {
            euint32 dstHandle = euint32.wrap(dstHandleList[i]);
            FHE.allowThis(dstHandle);
            FHE.allow(dstHandle, originalSender);
            handlesListReceived[i] = dstHandle;
        }

        emit HandlesListConfidentialOAppReceived(srcEid, srcApp, handlesListReceived, guid);
    }

    /// @notice Configure the canonical peer app on the chain at `eid`. Must be set before
    ///         this contract can `generateAndSendHandlesList` to that eid or accept a
    ///         delivery from it.
    ///         Pass `bytes32(0)` to clear a peer.
    /// @param eid  LayerZero endpoint id of the remote chain.
    /// @param peer Peer app on the remote chain as bytes32. EVM peers: pass
    ///        `bytes32(uint256(uint160(remoteAddress)))`.
    function setPeer(uint32 eid, bytes32 peer) external onlyOwner {
        _peers[eid] = peer;
        emit PeerSet(eid, peer);
    }

    /// @notice Returns the configured peer app on the chain at `eid` (bytes32(0) if unset).
    function peers(uint32 eid) external view returns (bytes32) {
        return _peers[eid];
    }

    /// @notice The list of source-chain handles from the most recent `onConfidentialBridgeReceived`.
    function lastReceivedSrcHandleList() external view returns (bytes32[] memory) {
        return _lastReceivedSrcHandleList;
    }

    /// @notice The list of destination-chain handles from the most recent `onConfidentialBridgeReceived`.
    function lastReceivedDstHandleList() external view returns (bytes32[] memory) {
        return _lastReceivedDstHandleList;
    }

    /// @notice The app-level payload from the most recent `onConfidentialBridgeReceived`.
    function lastReceivedPayload() external view returns (bytes memory) {
        return _lastReceivedPayload;
    }

    /// @dev Mint `count` random encrypted 32-bit values, granting this contract persistent
    ///      ACL allowance on each so the bridge's source-side allowance check passes.
    ///      Returns both the typed `euint32` handles (for the outbound event) and their
    ///      raw `bytes32` form (for the bridge call).
    function _generateHandles(
        uint256 count
    ) internal returns (euint32[] memory handles, bytes32[] memory handleList) {
        handles = new euint32[](count);
        handleList = new bytes32[](count);
        for (uint256 i = 0; i < count; i++) {
            euint32 value = FHE.randEuint32();
            FHE.allowThis(value);
            FHE.allow(value, msg.sender);
            handles[i] = value;
            handleList[i] = euint32.unwrap(value);
        }
    }
}
