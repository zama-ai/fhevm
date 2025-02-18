// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

import "./IDecryptionManager.sol";
import "../shared/Structs.sol";

/**
 * @title IACLManager.
 * @notice Interface of the ACLManager contract which aggregates all ACLs from all blockchains.
 */
interface IACLManager {
    /// @notice Emitted when a user is allowed to decrypt a given ciphertext handle
    /// @param ctHandle The ciphertext handle that the user is allowed to decrypt
    /// @param allowedAddress The address of the user allowed to decrypt the ciphertext
    event AllowUserDecrypt(uint256 indexed ctHandle, address allowedAddress);

    /// @notice Emitted when a public decryption is allowed for a given ciphertext handle
    /// @param ctHandle The ciphertext handle that is allowed for public decryption
    event AllowPublicDecrypt(uint256 indexed ctHandle);

    /// @notice Emitted when a user delegates the decryption access to another account
    /// @param chainId The chainId of the blockchain the allowed contract addresses belongs to.
    /// @param delegator The address of the current permission owner.
    /// @param delegatee The address of the access permission receiver.
    /// @param allowedContract The address of the contract being delegated to decrypt.
    event DelegateAccount(uint256 indexed chainId, address delegator, address delegatee, address allowedContract);

    /// @notice Error indicating that the sender is not a valid Coprocessor.
    error InvalidCoprocessorSender(address sender);

    /// @notice Error indicating that the ciphertext handle is not associated to the given network.
    error CiphertextHandleNotOnNetwork(uint256 ctHandle, uint256 chainId);

    /// @notice Error indicating that the given coprocessor has already allowed the ciphertext decryption.
    error CoprocessorHasAlreadyAllowed(address coprocessor, uint256 ctHandle);

    /// @notice Error indicating that the given coprocessor has already delegated the decryption access.
    error CoprocessorHasAlreadyDelegated(address coprocessor);

    /// @notice Error indicating that the given ciphertext handle is not allowed for user decryption.
    error UserDecryptNotAllowed(uint256 ctHandle, address userAddress);

    /// @notice Error indicating that the given ciphertext handle is not allowed for public decryption.
    error PublicDecryptNotAllowed(uint256 ctHandle);

    /// @notice Error indicating that the number of handles requested exceeds the maximum allowed.
    error TooManyContractsRequested(uint8 maxNumberExpected, uint256 actualNumber);

    /// @notice Allows an account address to access the given ciphertext handle over the chainId.
    /// @param chainId The chainId of the blockchain the ciphertext handle is associated with.
    /// @param ctHandle The handle of the ciphertext allowing for decryption.
    /// @param allowedAddress The address of the account receiving decryption access.
    function allowUserDecrypt(uint256 chainId, uint256 ctHandle, address allowedAddress) external;

    /// @notice Allows access to the given ciphertext handle for public decryption.
    /// @param chainId The chainId of the blockchain the ciphertext handle is associated with.
    /// @param ctHandle The handle of the ciphertext allowing for decryption.
    function allowPublicDecrypt(uint256 chainId, uint256 ctHandle) external;

    /// @notice Delegates the decryption access to the given delegatee for the given chainId and allowed contracts.
    /// @param chainId The chainId of the blockchain the allowed contracts addresses belongs to.
    /// @param delegator The address of the current permission owner.
    /// @param delegatee The address of the access permission receiver.
    /// @param allowedContracts The addresses of the contracts being delegated to decrypt.
    function delegateAccount(
        uint256 chainId,
        address delegator,
        address delegatee,
        address[] calldata allowedContracts
    ) external;

    /// @notice Returns the handles and PBS ciphertext pairs for the given chainId, user address and ciphertext handles.
    /// @param userAddress The address of the user tied to the requested handle and PBS ciphertext pairs.
    /// @param ctHandleContractPairs The ciphertext handles and their associated contract addresses to retrieve.
    /// @return A list of handle and PBS ciphertext pairs for the given chainId, user address and ciphertext handles.
    function getUserCiphertexts(
        address userAddress,
        IDecryptionManager.CtHandleContractPair[] calldata ctHandleContractPairs
    ) external view returns (CiphertextMaterial[] calldata);

    /// @notice Returns the handles and PBS ciphertext pairs for the given ciphertext handles.
    /// @param ctHandles The ciphertext handles to retrieve if public decryption is allowed.
    /// @return A list of handle and PBS ciphertext pairs for the given ciphertext handles.
    function getPublicCiphertexts(uint256[] calldata ctHandles) external view returns (CiphertextMaterial[] calldata);

    /// @notice Indicates if the decryption access to the given delegatee is already delegated.
    /// @param chainId The chainId of the blockchain the allowed contracts addresses belongs to.
    /// @param delegator The address of the current permission owner.
    /// @param delegatee The address of the access permission receiver.
    /// @param allowedContracts The addresses of the contracts delegated to decrypt.
    function isAccountDelegated(
        uint256 chainId,
        address delegator,
        address delegatee,
        address[] calldata allowedContracts
    ) external view returns (bool);
}
