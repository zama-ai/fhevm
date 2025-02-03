// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/**
 * @title ICiphertextStorage
 * @notice Interface of the CiphertextStorage contract that stores the ciphertexts allowed/requested for decryption.
 * @dev Normal 64-bit ciphertexts are only used for computations while 128-PBS ciphertexts are used for KMS concerns.
 */
interface ICiphertextStorage {
    /// @notice Checks if the ciphertext for the given handle is stored in the contract.
    /// @param ctHandle The handle of the ciphertext.
    function hasCiphertext(uint256 ctHandle) external view returns (bool);

    /// @notice Retrieves the list of 128-PBS ciphertexts for the given handles.
    /// @param ctHandle The list of handles of the requested ciphertexts.
    /// @return ciphertext128s A list of 128-PBS ciphertexts (128-bit).
    function getCiphertexts(uint256[] calldata ctHandle) external returns (bytes[] memory ciphertext128s);

    /// @notice Adds a new ciphertext to the storage.
    /// @param ctHandle The handle of the storing ciphertext.
    /// @param ciphertext64 The normal ciphertext (64-bit) to be stored.
    /// @param ciphertext128 The 128-PBS ciphertext (128-bit) to be stored.
    /// @param keyId The ID of the key under the ciphertext has been generated.
    function addCiphertext(
        uint256 ctHandle,
        bytes memory ciphertext64,
        bytes memory ciphertext128,
        uint256 keyId
    ) external;
}
