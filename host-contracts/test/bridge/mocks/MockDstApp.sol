// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IDstApp} from "../../../contracts/bridge/interfaces/IDstApp.sol";

/**
 * @notice Test-only destination app that records the last onReceive call so tests can
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
        bool wasCalled;
    }

    LastCall private _last;
    bool public shouldRevert;

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    function onReceive(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata srcHandleList,
        bytes32[] calldata dstHandleList
    ) external override {
        if (shouldRevert) revert("MockDstApp: revert requested");
        _last = LastCall({
            srcEid: srcEid,
            srcApp: srcApp,
            payload: payload,
            srcHandleList: srcHandleList,
            dstHandleList: dstHandleList,
            wasCalled: true
        });
    }

    function lastCall() external view returns (LastCall memory) {
        return _last;
    }
}
