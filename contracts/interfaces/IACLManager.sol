// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

import "./IDecryptionManager.sol";

/**
 * @title IACLManager.
 * @notice Interface of the ACLManager contract which aggregates all ACLs from all blockchains.
 */
interface IACLManager {
    /**
     * @notice A struct that contains the handle and the 128-PBS representation of a ciphertext.
     * @dev 128-PBS ciphertext is used for some optimizations in contrast with 64-bit ciphertext.
     */
    struct CtHandleCiphertext128Pair {
        uint256 ctHandle;
        bytes ciphertext128;
    }

    /// @notice Allows an account address to access the given ciphertext handle over the chainId.
    /// @param chainId The chainId of the blockchain the decryption access belongs to.
    /// @param ctHandle The handle of the ciphertext allowing for decryption.
    /// @param allowedAddress The address of the account receiving decryption access.
    function allowUserDecrypt(uint256 chainId, uint256 ctHandle, address allowedAddress) external;

    /// @notice Allows access to the given ciphertext handle for public decryption.
    /// @param ctHandle The handle of the ciphertext allowing for decryption.
    function allowPublicDecrypt(uint256 ctHandle) external;

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
    /// @param chainId The chainId of the blockchain tied to the requested handle and PBS ciphertext pairs.
    /// @param userAddress The address of the user tied to the requested handle and PBS ciphertext pairs.
    /// @param ctHandleContractPairs The ciphertext handles and their associated contract addresses to retrieve.
    /// @return A list of handle and PBS ciphertext pairs for the given chainId, user address and ciphertext handles.
    function getUserCiphertexts(
        uint256 chainId,
        address userAddress,
        IDecryptionManager.CtHandleContractPair[] calldata ctHandleContractPairs
    ) external view returns (CtHandleCiphertext128Pair[] calldata);

    /// @notice Returns the handles and PBS ciphertext pairs for the given ciphertext handles.
    /// @param ctHandles The ciphertext handles to retrieve if public decryption is allowed.
    /// @return A list of handle and PBS ciphertext pairs for the given ciphertext handles.
    function getPublicCiphertexts(
        uint256[] calldata ctHandles
    ) external view returns (CtHandleCiphertext128Pair[] calldata);
}
