// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {HCULimit} from "../contracts/HCULimit.sol";

/**
 * An `HCULimit` that keeps every per-transaction and per-block HCU charge, but drops ONLY the sequential
 * depth cap.
 *
 * Production reverts when a result handle's derived depth exceeds the configured limit. That limit exists to
 * bound real FHE work; in a test it mostly punishes long end-to-end flows whose orchestration is heavier than
 * the individual calls they are actually validating. Each override below keeps the base contract's depth
 * PROPAGATION rule (`opHCU + max(input depths)`) and its transaction accounting, and simply stores the deeper
 * handle instead of reverting.
 *
 * It lives here rather than in a consumer because it reaches into `HCULimit`'s internal accounting API —
 * knowledge that changes with the protocol version, like everything else in this package. Installed via
 * `FhevmStack.disableHCUDepthLimit()`, which upgrades the HCULimit proxy in place.
 */
contract HCULimitNoDepthCap is HCULimit {
    function _adjustAndCheckFheTransactionLimitOneOp(uint256 opHCU, address caller, bytes32 op1, bytes32 result)
        internal
        virtual
        override
    {
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);
        _setHCUForHandle(result, opHCU + _getHCUForHandle(op1));
    }

    function _adjustAndCheckFheTransactionLimitTwoOps(
        uint256 opHCU,
        address caller,
        bytes32 op1,
        bytes32 op2,
        bytes32 result
    ) internal virtual override {
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);
        _setHCUForHandle(result, opHCU + _maxDepth(_getHCUForHandle(op1), _getHCUForHandle(op2)));
    }

    function _adjustAndCheckFheTransactionLimitThreeOps(
        uint256 opHCU,
        address caller,
        bytes32 op1,
        bytes32 op2,
        bytes32 op3,
        bytes32 result
    ) internal virtual override {
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);
        uint256 deepest = _maxDepth(_getHCUForHandle(op1), _maxDepth(_getHCUForHandle(op2), _getHCUForHandle(op3)));
        _setHCUForHandle(result, opHCU + deepest);
    }

    function _maxDepth(uint256 a, uint256 b) private pure returns (uint256) {
        return a >= b ? a : b;
    }
}
