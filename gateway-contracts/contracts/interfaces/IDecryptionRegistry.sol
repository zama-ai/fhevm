// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the DecryptionRegistry contract.
 * @notice The DecryptionRegistry contract handles decryption request registration.
 * @dev V2 Design: Unlike V1 Decryption.sol, this contract:
 * - Does NOT handle response processing (done off-chain via KMS HTTP API)
 * - Does NOT depend on CiphertextCommits (ciphertexts fetched from Coprocessor API)
 * - Does NOT depend on MultichainACL (ACL checked on Host Chain by KMS)
 * - Events emit handles only (not full SnsCiphertextMaterial[])
 */
interface IDecryptionRegistry {
    // ============================================
    // Events
    // ============================================

    /**
     * @notice Emitted when a user decryption is requested.
     * @param requestId The unique identifier for the decryption request.
     * @param handles The ciphertext handles to decrypt.
     * @param contractAddresses The contract addresses associated with each handle for ACL lookup.
     * @param userAddress The address of the user requesting decryption.
     * @param publicKey The user's public key for re-encryption.
     * @param signature The EIP-712 user signature authorizing the request.
     * @param chainId The host chain ID where the handles originated.
     * @param timestamp The block timestamp when the request was made.
     */
    event UserDecryptionRequested(
        uint256 indexed requestId,
        bytes32[] handles,
        address[] contractAddresses,
        address indexed userAddress,
        bytes publicKey,
        bytes signature,
        uint256 chainId,
        uint256 timestamp
    );

    /**
     * @notice Emitted when a public decryption is requested.
     * @param requestId The unique identifier for the decryption request.
     * @param handles The ciphertext handles to decrypt.
     * @param contractAddresses The contract addresses associated with each handle for ACL lookup.
     * @param chainId The host chain ID where the handles originated.
     * @param timestamp The block timestamp when the request was made.
     */
    event PublicDecryptionRequested(
        uint256 indexed requestId,
        bytes32[] handles,
        address[] contractAddresses,
        uint256 chainId,
        uint256 timestamp
    );

    // ============================================
    // Errors
    // ============================================

    /**
     * @notice Error indicating that handles and contractAddresses arrays have different lengths.
     * @param handlesLength The length of the handles array.
     * @param contractAddressesLength The length of the contractAddresses array.
     */
    error HandleContractAddressLengthMismatch(uint256 handlesLength, uint256 contractAddressesLength);

    /**
     * @notice Error indicating that the handles array is empty.
     */
    error EmptyHandles();

    /**
     * @notice Error indicating that the public key is empty.
     */
    error EmptyPublicKey();

    /**
     * @notice Error indicating that the signature is empty.
     */
    error EmptySignature();

    // ============================================
    // Functions
    // ============================================

    /**
     * @notice Request a user decryption of the given handles.
     * @dev INVARIANT: handles.length == contractAddresses.length
     * Each handles[i] is associated with contractAddresses[i] for ACL lookup.
     * @param handles The ciphertext handles to decrypt.
     * @param contractAddresses The contract addresses associated with each handle.
     * @param publicKey The user's public key for re-encryption.
     * @param signature The EIP-712 signature authorizing the request.
     * @return requestId The unique identifier for the decryption request.
     */
    function requestUserDecryption(
        bytes32[] calldata handles,
        address[] calldata contractAddresses,
        address userAddress,
        bytes calldata publicKey,
        bytes calldata signature
    ) external payable returns (uint256 requestId);

    /**
     * @notice Request a public decryption of the given handles.
     * @dev INVARIANT: handles.length == contractAddresses.length
     * Each handles[i] is associated with contractAddresses[i] for ACL lookup.
     * @param handles The ciphertext handles to decrypt.
     * @param contractAddresses The contract addresses associated with each handle.
     * @return requestId The unique identifier for the decryption request.
     */
    function requestPublicDecryption(
        bytes32[] calldata handles,
        address[] calldata contractAddresses
    ) external payable returns (uint256 requestId);

    /**
     * @notice Returns the version of the DecryptionRegistry contract in SemVer format.
     */
    function getVersion() external pure returns (string memory);
}
