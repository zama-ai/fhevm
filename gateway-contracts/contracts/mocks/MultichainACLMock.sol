// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract MultichainACLMock {
    event AllowPublicDecrypt(bytes32 indexed ctHandle, address coprocessorTxSender, bytes extraData);

    event AllowPublicDecryptConsensus(bytes32 indexed ctHandle, bytes extraData);

    event AllowAccount(bytes32 indexed ctHandle, address accountAddress, address coprocessorTxSender, bytes extraData);

    event AllowAccountConsensus(bytes32 indexed ctHandle, address accountAddress, bytes extraData);

    event DelegateUserDecryption(
        uint256 indexed chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 expirationDate
    );

    event DelegateUserDecryptionConsensus(
        uint256 indexed chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 oldExpirationDate,
        uint64 newExpirationDate
    );

    event RevokeUserDecryptionDelegation(
        uint256 indexed chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter
    );

    event RevokeUserDecryptionDelegationConsensusReached(
        uint256 indexed chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 oldExpirationDate
    );

    function allowPublicDecrypt(bytes32 ctHandle, bytes calldata extraData) external {
        address coprocessorTxSender;

        emit AllowPublicDecrypt(ctHandle, coprocessorTxSender, extraData);

        emit AllowPublicDecryptConsensus(ctHandle, extraData);
    }

    function allowAccount(bytes32 ctHandle, address accountAddress, bytes calldata extraData) external {
        address coprocessorTxSender;

        emit AllowAccount(ctHandle, accountAddress, coprocessorTxSender, extraData);

        emit AllowAccountConsensus(ctHandle, accountAddress, extraData);
    }

    function delegateUserDecryption(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 expirationDate
    ) external {
        uint64 oldExpirationDate;
        uint64 newExpirationDate;

        emit DelegateUserDecryption(chainId, delegator, delegate, contractAddress, delegationCounter, expirationDate);

        emit DelegateUserDecryptionConsensus(
            chainId,
            delegator,
            delegate,
            contractAddress,
            delegationCounter,
            oldExpirationDate,
            newExpirationDate
        );
    }

    function revokeUserDecryptionDelegation(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 expirationDate
    ) external {
        uint64 oldExpirationDate;

        emit RevokeUserDecryptionDelegation(chainId, delegator, delegate, contractAddress, delegationCounter);

        emit RevokeUserDecryptionDelegationConsensusReached(
            chainId,
            delegator,
            delegate,
            contractAddress,
            delegationCounter,
            oldExpirationDate
        );
    }
}
