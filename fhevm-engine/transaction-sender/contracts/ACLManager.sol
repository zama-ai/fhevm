// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @title ACLManager smart contract
/// @dev source: github.com/zama-ai/gateway-l2/blob/main/contracts/ACLManager.sol
/// @notice This contract is a mock of the ACLManager contract from L2.
contract ACLManager {
    function allowAccount(
        uint256 chainId,
        uint256 ctHandle,
        address accountAddress
    ) public {}
}
