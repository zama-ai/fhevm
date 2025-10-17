// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract MultichainACLMock {
    event AllowAccount(bytes32 indexed ctHandle, address accountAddress);

    event AllowPublicDecrypt(bytes32 indexed ctHandle);

    event DelegateUserDecryption(uint256 indexed chainId, address delegator, address delegate, address contractAddress);

    event RevokeUserDecryption(uint256 indexed chainId, address delegator, address delegate, address contractAddress);

    function allowPublicDecrypt(bytes32 ctHandle, bytes calldata /* unusedVariable */) external {
        emit AllowPublicDecrypt(ctHandle);
    }

    function allowAccount(bytes32 ctHandle, address accountAddress, bytes calldata /* unusedVariable */) external {
        emit AllowAccount(ctHandle, accountAddress);
    }

    function delegateUserDecryption(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 expiryDate,
        uint64 delegationCounter
    ) external {
        emit DelegateUserDecryption(chainId, delegator, delegate, contractAddress);
    }

    function revokeUserDecryption(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 expiryDate,
        uint64 delegationCounter
    ) external {
        emit RevokeUserDecryption(chainId, delegator, delegate, contractAddress);
    }
}
