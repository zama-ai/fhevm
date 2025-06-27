// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { ContextStatus } from "../shared/Enums.sol";

/**
 * @title Interface for the InputVerification contract.
 * @dev The InputVerification contract handles Zero-Knowledge Proof of Knowledge (ZKPoK) verifications for inputs.
 */
interface IInputVerification {
    /**
     * @notice Emitted when a ZK Proof verification is started.
     * @param zkProofId The ID of the ZK Proof.
     * @param contextId The context ID.
     * @param contractChainId The host chain's chain ID of the contract requiring the ZK Proof verification.
     * @param contractAddress The address of the dapp requiring the ZK Proof verification.
     * @param userAddress The address of the user providing the input.
     * @param ciphertextWithZKProof The combination of the ciphertext (plain text signed with user PK) and the ZK Proof.
     */
    event VerifyProofRequest(
        uint256 indexed zkProofId,
        uint256 indexed contextId,
        uint256 indexed contractChainId,
        address contractAddress,
        address userAddress,
        bytes ciphertextWithZKProof
    );

    /**
     * @notice Emitted once a correct ZK Proof verification is completed.
     * @param zkProofId The ID of the ZK Proof.
     * @param ctHandles The coprocessor's computed ciphertext handles.
     * @param signatures The coprocessor's signature.
     */
    event VerifyProofResponse(uint256 indexed zkProofId, bytes32[] ctHandles, bytes[] signatures);

    /**
     * @notice Error indicating that the coprocessor context is no longer valid for verifying the ZK Proof.
     * A context is valid if it is active or suspended.
     * @param zkProofId The ID of the ZK Proof.
     * @param contextId The context ID of the coprocessor.
     * @param contextStatus The status of the coprocessor context.
     */
    error InvalidCoprocessorContextProofVerification(uint256 zkProofId, uint256 contextId, ContextStatus contextStatus);

    /**
     * @notice Error indicating that the coprocessor context is no longer valid for rejecting the ZK Proof.
     * A context is valid if it is active or suspended.
     * @param zkProofId The ID of the ZK Proof.
     * @param contextId The context ID of the coprocessor.
     * @param contextStatus The status of the coprocessor context.
     */
    error InvalidCoprocessorContextProofRejection(uint256 zkProofId, uint256 contextId, ContextStatus contextStatus);

    /**
     * @notice Emitted once an ZK Proof verification is rejected.
     * @param zkProofId The ID of the ZK Proof.
     */
    event RejectProofResponse(uint256 indexed zkProofId);

    /**
     * @notice Error indicating that the coprocessor has already verified the ZKPoK.
     * @param zkProofId The ID of the ZKPoK.
     * @param txSender The transaction sender address of the coprocessor that has already verified.
     * @param signer The signer address of the coprocessor that has already verified.
     */
    error CoprocessorAlreadyVerified(uint256 zkProofId, address txSender, address signer);

    /**
     * @notice Error indicating that the coprocessor has already rejected the ZKPoK.
     * @param zkProofId The ID of the ZKPoK.
     * @param txSender The transaction sender address of the coprocessor that has already rejected.
     * @param signer The signer address of the coprocessor that has already rejected.
     */
    error CoprocessorAlreadyRejected(uint256 zkProofId, address txSender, address signer);

    /**
     * @notice Error indicating that the ZK Proof has not been verified.
     * @param zkProofId The ID of the ZK Proof.
     */
    error ProofNotVerified(uint256 zkProofId);

    /**
     * @notice Error indicating that the ZK Proof has not been rejected.
     * @param zkProofId The ID of the ZK Proof.
     */
    error ProofNotRejected(uint256 zkProofId);

    /**
     * @notice Requests the verification of a ZK Proof.
     * @param contractChainId The ID of the blockchain the contract belongs to.
     * @param contractAddress The address of the dapp the input is used for.
     * @param userAddress The address of the user providing the input.
     * @param ciphertextWithZKProof The combination of the ciphertext (plain text signed with user PK) and the ZK Proof.
     */
    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextWithZKProof
    ) external;

    /**
     * @notice Responds to a correct ZK Proof verification request.
     * @param zkProofId The ID of the requested ZK Proof.
     * @param ctHandles The coprocessor's computed ciphertext handles.
     * @param signature The coprocessor's signature.
     */
    function verifyProofResponse(uint256 zkProofId, bytes32[] calldata ctHandles, bytes calldata signature) external;

    /**
     * @notice Rejects an incorrect ZK Proof verification request.
     * @dev This function does not ask for a signature as we only propagate an incorrect proof for
     * tracking purposes, so there is no real need to verify the signature anywhere else. Besides, we can
     * easily verify the sender's identity through `msg.sender`.
     *
     * @param zkProofId The ID of the requested ZK Proof.
     */
    function rejectProofResponse(uint256 zkProofId) external;

    /**
     * @notice Checks that a ZK Proof has been verified.
     * @param zkProofId The ID of the ZK Proof.
     */
    function checkProofVerified(uint256 zkProofId) external view;

    /**
     * @notice Checks that a ZK Proof has been rejected.
     * @param zkProofId The ID of the ZK Proof.
     */
    function checkProofRejected(uint256 zkProofId) external view;

    /**
     * @notice Returns the versions of the InputVerification contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}
