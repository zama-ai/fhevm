// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Impl} from "../Impl.sol";
import {IDstApp} from "./IConfidentialBridge.sol";
import {ConfidentialOAppCore} from "./ConfidentialOAppCore.sol";

/**
 * @title   ConfidentialOAppReceiver
 * @notice  Receive side of a confidential omnichain app. Implements the bridge's
 *          {IDstApp-onConfidentialBridgeReceived} callback and enforces the two checks every
 *          receiver needs:
 *            1. the caller is the trusted local `ConfidentialBridge`;
 *            2. `(srcEid, srcApp)` is a trusted peer (see {ConfidentialOAppCore-isPeer}).
 *          Subclasses implement {_onReceiveHandles} and receive the derived destination handles
 *          as raw `bytes32`, wrapping each to its known encrypted type (e.g.
 *          `euint64.wrap(handles[i])`). The host bridge has already granted transient ACL
 *          allowance for every handle before this is invoked.
 * @dev     The trusted bridge is resolved from the ACL — the same source the send path
 *          (`FHE.bridge`) uses — so inbound authentication and outbound routing can never
 *          diverge. Requires the app to have configured the ACL via `FHE.setCoprocessor(...)`.
 */
abstract contract ConfidentialOAppReceiver is ConfidentialOAppCore, IDstApp {
    error OnlyConfidentialBridge(address caller);
    error UntrustedPeer(uint32 srcEid, bytes32 srcApp);

    /**
     * @notice The trusted local ConfidentialBridge — the only authorized
     *         `onConfidentialBridgeReceived` caller. Resolved from the ACL, the same source the
     *         `FHE.bridge` send path uses.
     * @dev    Reverts `Impl.BridgeNotConfigured` if the ACL reports no bridge for this chain.
     */
    function confidentialBridge() public view returns (address) {
        return address(Impl.getConfidentialBridge());
    }

    /**
     * @inheritdoc IDstApp
     * @dev Authenticates caller + peer, then dispatches to {_onReceiveHandles}. `srcHandleList`
     *      is intentionally not forwarded — it carries opaque source-chain handles that are not
     *      usable locally.
     */
    function onConfidentialBridgeReceived(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata /* srcHandleList */,
        bytes32[] calldata dstHandleList,
        bytes32 guid
    ) external override {
        if (msg.sender != confidentialBridge()) revert OnlyConfidentialBridge(msg.sender);
        if (!isPeer(srcEid, srcApp)) revert UntrustedPeer(srcEid, srcApp);

        _onReceiveHandles(srcEid, srcApp, payload, dstHandleList, guid);
    }

    /**
     * @notice App hook: handle a bridged payload with the derived destination handles.
     * @param srcEid    Source endpoint id.
     * @param srcApp    Source app (bytes32; for EVM, a left-padded address).
     * @param payload   Opaque app payload as encoded by the source app.
     * @param handles   Derived destination handles as raw `bytes32`, aligned by index with the
     *                  source list. Wrap each to its known type, e.g. `euint64.wrap(handles[i])`.
     *                  Transient ACL allowance has already been granted to this contract.
     * @param guid      LayerZero message GUID of the transfer; useful for correlation or
     *                  idempotency. Apps that don't need it can ignore the parameter.
     */
    function _onReceiveHandles(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata handles,
        bytes32 guid
    ) internal virtual;
}
