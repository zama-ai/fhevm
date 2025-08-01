// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract MultichainAclMock {
    event AllowAccount(bytes32 indexed ctHandle, address accountAddress);

    event AllowPublicDecrypt(bytes32 indexed ctHandle);

    event DelegateAccount(uint256 indexed chainId, DelegationAccounts delegationAccounts, address[] contractAddresses);

    function allowPublicDecrypt(bytes32 ctHandle, bytes calldata /* unusedVariable */) external {
        emit AllowPublicDecrypt(ctHandle);
    }

    function allowAccount(bytes32 ctHandle, address accountAddress, bytes calldata /* unusedVariable */) external {
        emit AllowAccount(ctHandle, accountAddress);
    }

    function delegateAccount(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external {
        emit DelegateAccount(chainId, delegationAccounts, contractAddresses);
    }
}
