// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IDstApp} from "./IDstApp.sol";
import {ConfidentialOAppCore} from "./ConfidentialOAppCore.sol";

/**
 * @title   ConfidentialOAppReceiver
 * @notice  Receive side of a confidential omnichain app. Implements the destination app's
 *          {IDstApp-onConfidentialBridgeReceived} callback and enforces the two checks every
 *          receiver needs for security:
 *            1. the caller is the trusted local `ConfidentialBridge`.
 *            2. `(srcEid, srcApp)` is a trusted peer.
 *          Subclasses implement {_onReceiveHandles} and receive the derived destination handles
 *          as raw `bytes32`, wrapping each to its known encrypted type (e.g.
 *          `euint64.wrap(dstHandleList[i])`). The local bridge has already granted transient ACL
 *          allowance for every `dstHandleList[i]` before this callback is invoked.
 * @dev     The trusted bridge is resolved from the ACL, so the app must have configured the ACL via `FHE.setCoprocessor(...)`.
 */
abstract contract ConfidentialOAppReceiver is ConfidentialOAppCore, IDstApp {
    /// @notice The callback was not invoked by the trusted local `ConfidentialBridge`.
    /// @param caller The unauthorized `msg.sender` that attempted the call.
    error OnlyConfidentialBridge(address caller);

    /// @notice The `srcApp` does not match the peer configured for `srcEid`, so the message is not trusted.
    /// @param srcEid The LayerZero endpoint id of the source chain.
    /// @param srcApp The source app that sent the message (bytes32; for EVM, a left-padded address).
    error OnlyPeer(uint32 srcEid, bytes32 srcApp);

    /**
     * @inheritdoc IDstApp
     * @dev Enforces the two receiver security checks before dispatching to {_onReceiveHandles}:
     *      reverts {OnlyConfidentialBridge} if `msg.sender` is not the local `ConfidentialBridge`
     *      (resolved from the ACL), and {UntrustedPeer} if `srcApp` is not the configured peer for `srcEid`.
     */
    function onConfidentialBridgeReceived(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata srcHandleList,
        bytes32[] calldata dstHandleList,
        bytes32 guid
    ) external virtual {
        if (msg.sender != LZConfidentialBridgeAddress()) revert OnlyConfidentialBridge(msg.sender);
        // Ensure that the sender matches the expected peer for the source endpoint.
        if (_getPeerOrRevert(srcEid) != srcApp) revert OnlyPeer(srcEid, srcApp);

        _onReceiveHandles(srcEid, srcApp, payload, srcHandleList, dstHandleList, guid);
    }

    /**
     * @notice App hook: handle a bridged payload with the derived destination handles. Called by
     *         {onConfidentialBridgeReceived} only after the caller and peer have been authenticated.
     * @param srcEid        Source LayerZero endpoint id.
     * @param srcApp        Source app (bytes32; for EVM, a left-padded address).
     * @param payload       Opaque app payload as encoded by the source app.
     * @param srcHandleList Source-chain handles as raw `bytes32`, in the order the source app sent them.
     *                      Treat as opaque: they are not usable on this chain.
     * @param dstHandleList Derived destination handles as raw `bytes32`, aligned by index with
     *                      `srcHandleList`. Wrap each to its known type, e.g. `euint64.wrap(dstHandleList[i])`.
     *                      Transient ACL allowance has already been granted to this contract for each `dstHandleList[i]`.
     * @param guid          LayerZero message GUID of the transfer; useful for correlation or
     *                      analytics. Apps that don't need it can ignore the parameter.
     */
    function _onReceiveHandles(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata srcHandleList,
        bytes32[] calldata dstHandleList,
        bytes32 guid
    ) internal virtual;
}
