// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title IZKPoKManager
 * @dev Interface of the ZKPoKManager contract for Zero-Knowledge Proof of Knowledge (ZKPoK)
 * verifications.
 *
 * This interface expose two functions that are meant to process a ZK Proof verification asynchronously.
 * The first function is called by the fhEVM Relayer to start the verification process, and the second
 * function is called by the Coprocessors that process the verification.
 */
interface IZKPoKManager {
    /// @notice Emitted when a ZK Proof verification is started
    /// @dev This event is meant to be listened by the Coprocessor
    /// @param zkProofId The ID of the ZK Proof
    /// @param contractChainId The chainId of the contract requiring the ZK Proof verification
    /// @param contractAddress The address of the dapp requiring the ZK Proof verification
    /// @param userAddress The address of the user providing the input
    /// @param ciphertextWithZKProof The combination of the ciphertext (plain text signed with user PK) and the ZK Proof
    event VerifyProofRequest(
        uint256 indexed zkProofId,
        uint256 indexed contractChainId,
        address contractAddress,
        address userAddress,
        bytes ciphertextWithZKProof
    );

    /// @notice Emitted once a ZK Proof verification is completed
    /// @dev This event is meant to be listened by the fhEVM Relayer
    /// @param zkProofId The ID of the ZK Proof
    /// @param ctHandles The Coprocessor's computed ciphertext handles
    /// @param signatures The Coprocessor's signature
    event VerifyProofResponse(uint256 indexed zkProofId, bytes32[] ctHandles, bytes[] signatures);

    /// @notice Error indicating that the Coprocessor has already signed its ZK Proof verification response
    /// @param zkProofId The ID of the ZK Proof
    /// @param signer The address of the Coprocessor signer that has already signed
    error CoprocessorHasAlreadySigned(uint256 zkProofId, address signer);

    /// @notice Requests the verification of a ZK Proof
    /// @dev This function is called by the fhEVM Relayer
    /// @param contractChainId The chainId of the blockchain the contract belongs to
    /// @param contractAddress The address of the dapp the input is used for
    /// @param userAddress The address of the user providing the input
    /// @param ciphertextWithZKProof The combination of the ciphertext (plain text signed with user PK) and the ZK Proof
    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextWithZKProof
    ) external;

    /// @notice Responds to a ZK Proof verification request
    /// @dev This function is called by the Coprocessor
    /// @param zkProofId The ID of the requested ZK Proof
    /// @param ctHandles The Coprocessor's computed ciphertext handles
    /// @param signature The Coprocessor's signature
    function verifyProofResponse(uint256 zkProofId, bytes32[] calldata ctHandles, bytes calldata signature) external;

    /// @notice Indicates if a given ZK Proof is already verified
    /// @param zkProofId The ID of the ZK Proof
    /// @return Whether the ZK Proof is verified
    function isProofVerified(uint256 zkProofId) external view returns (bool);
}
