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

    /// @notice Emitted when a public decryption is allowed for a given ciphertext handle
    /// @param ctHandle The ciphertext handle that is allowed for public decryption
    event AllowPublicDecrypt(uint256 indexed ctHandle);

    /// @notice Emitted when an account delegates its access to another account
    /// @param chainId The chainId of the blockchain the allowed contract addresses belongs to.
    /// @param delegator The address of the current permission owner.
    /// @param delegatee The address of the access permission receiver.
    /// @param contractAddresses The addresses of the delegatedcontracts.
    event DelegateAccount(uint256 indexed chainId, address delegator, address delegatee, address[] contractAddresses);

    /// @notice Error indicating that the given coprocessor has already allowed access to the ciphertext.
    error CoprocessorAlreadyAllowed(address coprocessor, uint256 ctHandle);

    /// @notice Error indicating that the given coprocessor has already delegated access to another account.
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

    /// @dev Error indicating that the given user address has been found within the list of contract
    /// @dev addresses during account access check
    error UserAddressInContractAddresses(address userAddress);

    /// @notice Error indicating that the given user is not allowed to use the ciphertext handle.
    error UserNotAllowedToUseCiphertext(uint256 ctHandle, address userAddress);

    /// @notice Error indicating that the given contract is not allowed to use the ciphertext handle.
    error ContractNotAllowedToUseCiphertext(uint256 ctHandle, address contractAddress);

    /// @notice Error indicating that the given ciphertext handle is not allowed for public decryption.
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

    /// @notice Allows access to the given ciphertext handle for public decryption.
    /// @param chainId The network's chainId where the ciphertext handle belongs to.
    /// @param ctHandle The ciphertext handle.
    function allowPublicDecrypt(uint256 chainId, uint256 ctHandle) external;

    /// @notice Delegates the decryption access to the given delegatee for the given chainId and allowed contracts.
    /// @param chainId The network's chainId where the allowed contracts addresses belongs to.
    /// @param delegator The address of the current permission owner.
    /// @param delegatee The address of the access permission receiver.
    /// @param contractAddresses The addresses of the contracts being delegated to decrypt.
    function delegateAccount(
        uint256 chainId,
        address delegator,
        address delegatee,
        address[] calldata contractAddresses
    ) external;

    /// @notice Checks if the user and the given contracts are allowed to decrypt all the given ciphertext handles.
    /// @param userAddress The address of the user.
    /// @param ctHandleContractPairs The ciphertext handles and their associated contract addresses.
    function checkAccountAllowed(
        address userAddress,
        CtHandleContractPair[] calldata ctHandleContractPairs
    ) external view;

    /// @notice Checks if the public decryption is allowed for all the given ciphertext handles.
    /// @param ctHandles The ciphertext handles.
    function checkPublicDecryptAllowed(uint256[] calldata ctHandles) external view;

    /// @notice Indicates if the decryption access to the given delegatee is already delegated.
    /// @param chainId The chainId of the blockchain the allowed contracts addresses belongs to.
    /// @param delegator The address of the current permission owner.
    /// @param delegatee The address of the access permission receiver.
    /// @param contractAddresses The addresses of the contracts delegated to decrypt.
    function checkAccountDelegated(
        uint256 chainId,
        address delegator,
        address delegatee,
        address[] calldata contractAddresses
    ) external view;
}
