// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./IDecryptionManager.sol";
import "../shared/Structs.sol";

/**
 * @title IACLManager.
 * @notice Interface of the ACLManager contract which aggregates all ACLs from all blockchains.
 */
interface IACLManager {
    /// @notice Emitted when an account is allowed to use a ciphertext handle
    /// @param ctHandle The ciphertext handle that the account is allowed to use
    /// @param accountAddress The address of the account allowed to use the ciphertext handle
    event AllowAccount(uint256 indexed ctHandle, address accountAddress);

    /// @notice Emitted when a public decryption is allowed for a ciphertext handle
    /// @param ctHandle The ciphertext handle that is allowed for public decryption
    event AllowPublicDecrypt(uint256 indexed ctHandle);

    /// @notice Emitted when an account delegates its access to another account
    /// @param chainId The chainId of the blockchain the allowed contract addresses belongs to.
    /// @param delegator The address of the current permission owner.
    /// @param delegatee The address of the access permission receiver.
    /// @param contractAddresses The addresses of the delegated contracts.
    event DelegateAccount(uint256 indexed chainId, address delegator, address delegatee, address[] contractAddresses);

    /// @notice Error indicating that the coprocessor has already allowed access to the ciphertext.
    error CoprocessorAlreadyAllowed(address coprocessor, uint256 ctHandle);

    /// @notice Error indicating that the coprocessor has already delegated access to another account.
    /// @param coprocessor The address of the coprocessor that has already confirm delegation.
    /// @param chainId The chainId of the blockchain the delegatee contract addresses belongs to.
    /// @param delegator The address of the permission owner.
    /// @param delegatee The address of the account checking the delegation.
    /// @param contractAddresses The addresses of the contracts that the coprocessor has already delegated.
    error CoprocessorAlreadyDelegated(
        address coprocessor,
        uint256 chainId,
        address delegator,
        address delegatee,
        address[] contractAddresses
    );

    /// @dev Error indicating that the account address has been found within the list of contract
    /// @dev addresses during account access check
    error AccountAddressInContractAddresses(address accountAddress);

    /// @notice Error indicating that the account address is not allowed to use the ciphertext handle.
    error AccountNotAllowedToUseCiphertext(uint256 ctHandle, address accountAddress);

    /// @notice Error indicating that the contract is not allowed to use the ciphertext handle.
    error ContractNotAllowedToUseCiphertext(uint256 ctHandle, address contractAddress);

    /// @notice Error indicating that the ciphertext handle is not allowed for public decryption.
    error PublicDecryptNotAllowed(uint256 ctHandle);

    /// @notice Error indicating that the number of delegation contracts requested exceeds the maximum allowed.
    error ContractsMaxLengthExceeded(uint8 maxLength, uint256 actualLength);

    /// @notice Error indicating that the account has not been fully delegated.
    /// @param chainId The chainId of the blockchain the contract address belongs to.
    /// @param delegator The address of the permission owner.
    /// @param delegatee The address of the account checking the delegation.
    /// @param contractAddress The address of the delegated contract.
    error AccountNotDelegated(uint256 chainId, address delegator, address delegatee, address contractAddress);

    /// @notice Allows an account to access a ciphertext handle over a chainId.
    /// @param chainId The network's chainId where the ciphertext handle belongs to.
    /// @param ctHandle The ciphertext handle.
    /// @param accountAddress The account's address.
    function allowAccount(uint256 chainId, uint256 ctHandle, address accountAddress) external;

    /// @notice Allows access to the ciphertext handle for public decryption.
    /// @param chainId The network's chainId where the ciphertext handle belongs to.
    /// @param ctHandle The ciphertext handle.
    function allowPublicDecrypt(uint256 chainId, uint256 ctHandle) external;

    /// @notice Delegates the access to the delegatee and contracts.
    /// @param chainId The network's chainId where the contracts addresses belongs to.
    /// @param delegator The address of the current permission owner.
    /// @param delegatee The address of the access permission receiver.
    /// @param contractAddresses The contract addresses to delegate access to.
    function delegateAccount(
        uint256 chainId,
        address delegator,
        address delegatee,
        address[] calldata contractAddresses
    ) external;

    /// @notice Checks that the account and the contracts are allowed to use all the ciphertext handles.
    /// @param accountAddress The address of the account.
    /// @param ctHandleContractPairs The ciphertext handles and their associated contract addresses.
    function checkAccountAllowed(
        address accountAddress,
        CtHandleContractPair[] calldata ctHandleContractPairs
    ) external view;

    /// @notice Checks that the public decryption is allowed for all the ciphertext handles.
    /// @param ctHandles The ciphertext handles.
    function checkPublicDecryptAllowed(uint256[] calldata ctHandles) external view;

    /// @notice Checks delegator has delegated access to the delegatee and contracts.
    /// @param chainId The chainId of the blockchain the allowed contracts addresses belongs to.
    /// @param delegator The address of the current permission owner.
    /// @param delegatee The address of the access permission receiver.
    /// @param contractAddresses The delegated contract addresses.
    function checkAccountDelegated(
        uint256 chainId,
        address delegator,
        address delegatee,
        address[] calldata contractAddresses
    ) external view;

    /// @notice Return whether the account is allowed to use the ciphertext handle or not.
    /// @return Whether the account is allowed to use the ciphertext handle or not.
    function allowedAccounts(uint256 ctHandle, address accountAddress) external view returns (bool);

    /// @notice Return whether the ciphertext can be publicly decrypted or not.
    /// @return Whether the ciphertext can be publicly decrypted or not.
    function allowedPublicDecrypts(uint256 ctHandle) external view returns (bool);
}
