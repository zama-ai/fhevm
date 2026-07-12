// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @dev Test-only HCULimit implementation that keeps the base contract's per-transaction
/// HCU accounting but skips the sequential-depth guard. The production implementation
/// reverts when a result handle would exceed the depth cap derived from its inputs;
/// these overrides intentionally store the deeper handle instead so multi-step tests
/// can exercise long end-to-end flows without patching the vendored dependency.

import {HCULimit} from "../host-contracts/contracts/HCULimit.sol";

contract HCULimitNoDepthCap is HCULimit {
    /// @dev Diff from base: still charges `opHCU` against the transaction (and block cap),
    /// but does not compare the derived handle depth against the configured depth limit.
    function _adjustAndCheckFheTransactionLimitOneOp(uint256 opHCU, address caller, bytes32 op1, bytes32 result)
        internal
        virtual
        override
    {
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);
        _setHCUForHandle(result, opHCU + _getHCUForHandle(op1));
    }

    /// @dev Diff from base: preserves the same depth propagation rule (`opHCU + max(input depths)`)
    /// while removing the revert that normally blocks deeper intermediate handles.
    function _adjustAndCheckFheTransactionLimitTwoOps(
        uint256 opHCU,
        address caller,
        bytes32 op1,
        bytes32 op2,
        bytes32 result
    ) internal virtual override {
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);
        _setHCUForHandle(result, opHCU + _maxHandleHcu(op1, op2));
    }

    /// @dev Diff from base: keeps the same three-input depth calculation as HCULimit,
    /// but stores the computed depth unconditionally instead of enforcing the depth cap.
    function _adjustAndCheckFheTransactionLimitThreeOps(
        uint256 opHCU,
        address caller,
        bytes32 op1,
        bytes32 op2,
        bytes32 op3,
        bytes32 result
    ) internal virtual override {
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);
        uint256 maxInputHcu = _maxHcu(_getHCUForHandle(op1), _maxHcu(_getHCUForHandle(op2), _getHCUForHandle(op3)));
        _setHCUForHandle(result, opHCU + maxInputHcu);
    }

    /// @dev Helper local to the relaxed implementation; the base contract has a similar
    /// private utility, so this wrapper keeps the override logic readable without relying on it.
    function _maxHandleHcu(bytes32 op1, bytes32 op2) private view returns (uint256) {
        uint256 hcu1 = _getHCUForHandle(op1);
        uint256 hcu2 = _getHCUForHandle(op2);
        return hcu1 >= hcu2 ? hcu1 : hcu2;
    }

    /// @dev Small numeric max helper used when composing three-input handle depths.
    function _maxHcu(uint256 hcu1, uint256 hcu2) private pure returns (uint256) {
        return hcu1 >= hcu2 ? hcu1 : hcu2;
    }
}
