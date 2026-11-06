// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title   IConfidentialBridge
 */
interface IConfidentialBridge {
    /**
     * @notice Re-points the bridge's LayerZero endpoint delegate to the current ACL owner.
     * @dev    Must be restricted to the ACL contract.
     */
    function syncDelegate() external;
}
