// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IDstApp} from "../../../contracts/bridge/interfaces/IDstApp.sol";

/**
 * @notice Test-only destination app that records the last onConfidentialBridgeReceived call so tests can
 *         assert HandlesReceiver dispatched the expected arguments. Optionally reverts
 *         to exercise the lzCompose revert-handling path.
 */
contract MockDstApp is IDstApp {
    struct LastCall {
        uint32 srcEid;
        bytes32 srcApp;
        bytes payload;
        bytes32[] srcHandleList;
        bytes32[] dstHandleList;
        bytes32 guid;
        bool wasCalled;
    }

    LastCall private _last;
    bool public shouldRevert;

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    function onConfidentialBridgeReceived(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata srcHandleList,
        bytes32[] calldata dstHandleList,
        bytes32 guid
    ) external override {
        if (shouldRevert) revert("MockDstApp: revert requested");
        _last = LastCall({
            srcEid: srcEid,
            srcApp: srcApp,
            payload: payload,
            srcHandleList: srcHandleList,
            dstHandleList: dstHandleList,
            guid: guid,
            wasCalled: true
        });
    }

    function lastCall() external view returns (LastCall memory) {
        return _last;
    }
}
