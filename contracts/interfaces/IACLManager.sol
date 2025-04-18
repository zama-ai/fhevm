// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./IDecryptionManager.sol";
/**
 * @title IACLManager.
 * @notice Interface of the ACLManager contract which aggregates all ACLs from all blockchains.
 */
interface IACLManager {
    /// @notice Emitted when an account is allowed to use a ciphertext handle
    /// @param ctHandle The ciphertext handle that the account is allowed to use
    /// @param accountAddress The address of the account allowed to use the ciphertext handle
    event AllowAccount(bytes32 indexed ctHandle, address accountAddress);

    /// @notice Emitted when a public decryption is allowed for a ciphertext handle
    /// @param ctHandle The ciphertext handle that is allowed for public decryption
    event AllowPublicDecrypt(bytes32 indexed ctHandle);

    /// @notice Emitted when an account delegates its access to another account
    /// @param chainId The contracts' host chainId.
    /// @param delegationAccounts The delegator and the delegated addresses.
    /// @param contractAddresses The addresses of the delegated contracts.
    event DelegateAccount(uint256 indexed chainId, DelegationAccounts delegationAccounts, address[] contractAddresses);

    /// @notice Error indicating that the coprocessor has already allowed access to the ciphertext.
    error CoprocessorAlreadyAllowed(address coprocessor, bytes32 ctHandle);

    /// @notice Error indicating that the coprocessor has already delegated access to another account.
    /// @param coprocessor The address of the coprocessor that has already confirm delegation.
    /// @param chainId The contracts' host chainId.
    /// @param delegationAccounts The delegator and the delegated addresses.
    /// @param contractAddresses The addresses of the contracts that the coprocessor has already delegated.
    error CoprocessorAlreadyDelegated(
        address coprocessor,
        uint256 chainId,
        DelegationAccounts delegationAccounts,
        address[] contractAddresses
    );

    /// @notice Error indicating that the account address is not allowed to use the ciphertext handle.
    error AccountNotAllowedToUseCiphertext(bytes32 ctHandle, address accountAddress);

    /// @notice Error indicating that the ciphertext handle is not allowed for public decryption.
    error PublicDecryptNotAllowed(bytes32 ctHandle);

    /// @notice Error indicating that the contract addresses list is empty.
    error EmptyContractAddresses();

    /// @notice Error indicating that the number of delegation contracts requested exceeds the maximum allowed.
    error ContractsMaxLengthExceeded(uint8 maxLength, uint256 actualLength);

    /// @notice Error indicating that the account has not been fully delegated.
    /// @param chainId The contracts's host chainId.
    /// @param delegationAccounts The delegator and the delegated addresses.
    /// @param contractAddress The address of the delegated contract.
    error AccountNotDelegated(uint256 chainId, DelegationAccounts delegationAccounts, address contractAddress);

    /// @notice Allows access to the ciphertext handle for public decryption.
    /// @param ctHandle The ciphertext handle.
    function allowPublicDecrypt(bytes32 ctHandle) external;

    /// @notice Allows an account to access a ciphertext handle.
    /// @param ctHandle The ciphertext handle.
    /// @param accountAddress The account's address.
    function allowAccount(bytes32 ctHandle, address accountAddress) external;

    /// @notice Delegates the access to the delegated and contract addresses.
    /// @param chainId The contracts' host chainId.
    /// @param delegationAccounts The delegator and the delegated addresses.
    /// @param contractAddresses The contract addresses to delegate access to.
    function delegateAccount(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external;

    /// @notice Checks that the ciphertext handle is allowed for public decryption.
    /// @param ctHandle The ciphertext handle.
    function checkPublicDecryptAllowed(bytes32 ctHandle) external view;

    /// @notice Checks that the account is allowed to use the ciphertext handle.
    /// @param ctHandle The ciphertext handle.
    /// @param accountAddress The address of the account.
    function checkAccountAllowed(bytes32 ctHandle, address accountAddress) external view;

    /// @notice Checks that the delegator has delegated access to the delegate and contracts addresses.
    /// @param chainId The contracts' host chainId.
    /// @param delegationAccounts The delegator and the delegated addresses.
    /// @param contractAddresses The delegated contract addresses.
    function checkAccountDelegated(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external view;
}
