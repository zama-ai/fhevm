// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @title MultichainAcl smart contract
/// @dev source: github.com/zama-ai/fhevm-gateway/blob/main/contracts/MultichainAcl.sol
/// @notice This contract is a mock of the MultichainAcl contract from L2.
contract MultichainAcl {
    function allowAccount(bytes32 ctHandle, address accountAddress) public {}

    function allowPublicDecrypt(bytes32 ctHandle) public {}
}
