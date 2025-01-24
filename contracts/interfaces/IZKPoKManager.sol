// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/**
 * @title IZKPoKManager
 * @dev Interface of the ZKPoKManager contract for Zero-Knowledge Proof of Knowledge (ZKPoK)
 * verifications.
 *
 * This interface expose two functions that are meant to process a ZKProof verification asynchronously.
 * The first function is called by the fhEVM Relayer to start the verification process, and the second
 * function is called by the Coprocessors that process the verification.
 */
interface IZKPoKManager {
    /// @notice Emitted when a ZKProof verification ID is created
    /// @dev This event is meant to be listened by the fhEVM Relayer
    event ZKProofId(uint256 indexed zkProofId);

    /// @notice Emitted when a ZKProof verification is started
    /// @dev This event is meant to be listened by the Coprocessor
    event VerifyProofRequest(
        uint256 indexed keychainId,
        uint256 indexed zkProofId,
        uint256 indexed chainId,
        address contractAddress,
        address userAddress,
        bytes ctProofHandle
    );

    /// @notice Emitted once a ZKProof verification is completed
    /// @dev This event is meant to be listened by the fhEVM Relayer
    event VerifyProofResponse(uint256 indexed zkProofId, bytes32[] handles, bytes[] signatures);

    /// @notice Error indicating that a given chain ID is not attached to the keychain ID
    error NetworkNotRegisteredInKeychain();

    /// @notice Starts the ZKProof verification
    /// @dev This function is called by the fhEVM Relayer
    /// @param keychainId The ID of the keychain used
    /// @param chainId The network's chain ID
    /// @param contractAddress The address of the dapp the input is used for
    /// @param userAddress The address of the user providing the input
    /// @param ctProofHandle The Coprocessor DA handle of the ciphertext and proof to be verified
    function verifyProofRequest(
        uint256 keychainId,
        uint256 chainId,
        address contractAddress,
        address userAddress,
        bytes calldata ctProofHandle
    ) external;

    /// @notice Processes ZKProof verification responses and validates the verification completion
    /// @dev This function is called by the Coprocessor
    /// @param zkProofId The ID of the requested ZKProof verification
    /// @param handles The Coprocessor's computed handles
    /// @param signature The Coprocessor's signature
    function verifyProofResponse(uint256 zkProofId, bytes32[] calldata handles, bytes calldata signature) external;
}
