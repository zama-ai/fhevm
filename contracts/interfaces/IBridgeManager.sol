// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @title An interface for the bridge manager
/// @notice The bridge manager is responsible for managing bridging ciphertexts between networks
/// @dev Request functions are callable by any user or the relayer
/// @dev Response functions are only callable by the KMS Connectors
interface IBridgeManager {
    /// @notice Emitted when a bridge request is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param bridgeId The bridge request's unique ID
    event BridgeId(uint256 indexed bridgeId);

    /// @notice Emitted when a bridge request is made
    /// @dev This event is meant to be listened by the KMS Connectors
    /// @param keychainId The keychain's unique ID
    /// @param bridgeId The bridge request's unique ID
    /// @param chainIdFrom The chain ID of the source network
    /// @param chainIdTo The chain ID of the destination network
    /// @param ciphertextHandles The ciphertext handles to bridge
    /// @param userAddress The user's address
    /// @param userSignature The user's signature
    event BridgeRequest(
        uint256 indexed keychainId,
        uint256 indexed bridgeId,
        uint256 indexed chainIdFrom,
        uint256 chainIdTo,
        uint256[] ciphertextHandles,
        address userAddress,
        address userSignature
    );

    /// @notice Emitted when a bridge response is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param bridgeId The bridge request's unique ID
    /// @param chainIdTo The chain ID of the destination network
    /// @param ciphertextHandles The ciphertext handles that can be bridged
    /// @param userAddress The user's address
    /// @param attestations The coprocessor's attestations
    /// @param signatures The coprocessor's signatures
    event BridgeResponse(
        uint256 indexed bridgeId,
        uint256 chainIdTo,
        uint256[] ciphertextHandles,
        address userAddress,
        bytes[] attestations,
        bytes[] signatures
    );

    /// @notice Requests a bridge of ciphertexts between two networks
    /// @dev This function can be called by a user or relayer
    /// @param keychainId The keychain's unique ID
    /// @param chainIdFrom The chain ID of the source network
    /// @param chainIdTo The chain ID of the destination network
    /// @param ciphertextHandles The ciphertext handles to bridge
    /// @param userAddress The user's address
    /// @param userSignature The user's signature
    function bridgeRequest(
        uint256 keychainId,
        uint256 chainIdFrom,
        uint256 chainIdTo,
        uint256[] calldata ciphertextHandles,
        address userAddress,
        address userSignature
    ) external;

    /// @notice Responds to a bridge request
    /// @dev This function can be called by the KMS Connectors
    /// @param bridgeId The bridge request's unique ID
    /// @param chainIdTo The chain ID of the destination network
    /// @param ciphertextHandles The ciphertext handles that can be bridged
    /// @param userAddress The user's address
    /// @param attestation The coprocessor's attestation
    /// @param signature The coprocessor's signature
    function bridgeResponse(
        uint256 bridgeId,
        uint256 chainIdTo,
        uint256[] calldata ciphertextHandles,
        address userAddress,
        bytes calldata attestation,
        bytes calldata signature
    ) external;
}
