// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title IInputVerification
 * @dev Interface of the InputVerification contract for Zero-Knowledge Proof of Knowledge (ZKPoK)
 * verifications.
 *
 * This interface expose functions that are meant to process a ZK Proof verification asynchronously
 * by interacting with the coprocessors.
 */
interface IInputVerification {
    /**
     * @notice Emitted when a ZK Proof verification is started
     * @dev This event is meant to be listened by a coprocessor
     * @param zkProofId The ID of the ZK Proof
     * @param contractChainId The chainId of the contract requiring the ZK Proof verification
     * @param contractAddress The address of the dapp requiring the ZK Proof verification
     * @param userAddress The address of the user providing the input
     * @param ciphertextWithZKProof The combination of the ciphertext (plain text signed with user PK) and the ZK Proof
     */
    event VerifyProofRequest(
        uint256 indexed zkProofId,
        uint256 indexed contractChainId,
        address contractAddress,
        address userAddress,
        bytes ciphertextWithZKProof
    );

    /**
     * @notice Emitted once a correct ZK Proof verification is completed
     * @param zkProofId The ID of the ZK Proof
     * @param ctHandles The coprocessor's computed ciphertext handles
     * @param signatures The coprocessor's signature
     */
    event VerifyProofResponse(uint256 indexed zkProofId, bytes32[] ctHandles, bytes[] signatures);

    /**
     * @notice Emitted once an ZK Proof verification is rejected
     * @param zkProofId The ID of the ZK Proof
     */
    event RejectProofResponse(uint256 indexed zkProofId);

    /**
     * @notice Error indicating that the coprocessor signer has already responded to
     * the ZK Proof verification request (either by verifying or rejecting the proof)
     * @param zkProofId The ID of the ZK Proof
     * @param signer The address of the coprocessor signer that has already responded
     */
    error CoprocessorSignerAlreadyResponded(uint256 zkProofId, address signer);

    /**
     * @notice Error indicating that the coprocessor signer has already signed the ZK Proof verification
     * @param zkProofId The ID of the ZK Proof
     * @param signer The address of the coprocessor signer that has already signed
     */
    error CoprocessorSignerAlreadySigned(uint256 zkProofId, address signer);

    /**
     * @notice Error indicating that the ZK Proof has not been verified
     * @param zkProofId The ID of the ZK Proof
     */
    error ProofNotVerified(uint256 zkProofId);

    /**
     * @notice Error indicating that the ZK Proof has not been rejected
     * @param zkProofId The ID of the ZK Proof
     */
    error ProofNotRejected(uint256 zkProofId);

    /**
     * @notice Requests the verification of a ZK Proof
     * @param contractChainId The chainId of the blockchain the contract belongs to
     * @param contractAddress The address of the dapp the input is used for
     * @param userAddress The address of the user providing the input
     * @param ciphertextWithZKProof The combination of the ciphertext (plain text signed with user PK) and the ZK Proof
     */
    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextWithZKProof
    ) external;

    /**
     * @notice Responds to a correct ZK Proof verification request
     * @dev This function checks that the message was signed by a coprocessor
     * @param zkProofId The ID of the requested ZK Proof
     * @param ctHandles The coprocessor's computed ciphertext handles
     * @param signature The coprocessor's signature
     */
    function verifyProofResponse(uint256 zkProofId, bytes32[] calldata ctHandles, bytes calldata signature) external;

    /**
     * @notice Rejects an incorrect ZK Proof verification request
     * @dev This function checks that the message sender is a coprocessor
     * @dev This function does not ask for a signature as we only propagate an incorrect proof for
     * @dev tracking purposes, so there is no real need to verify the signature anywhere else. Besides,
     * @dev we can easily verify the sender's identity through `msg.sender`.
     * @param zkProofId The ID of the requested ZK Proof
     */
    function rejectProofResponse(uint256 zkProofId) external;

    /**
     * @notice Checks that a ZK Proof has been verified
     * @param zkProofId The ID of the ZK Proof
     */
    function checkProofVerified(uint256 zkProofId) external view;

    /**
     * @notice Checks that a ZK Proof has been rejected
     * @param zkProofId The ID of the ZK Proof
     */
    function checkProofRejected(uint256 zkProofId) external view;

    /// @notice Returns the versions of the InputVerification contract in SemVer format.
    /// @dev This is conventionally used for upgrade features.
    function getVersion() external pure returns (string memory);
}
