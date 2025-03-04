// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the HTTPZ contract
 * @notice The HTTPZ contract is responsible for being a point of truth for all contracts and
 * components from the Gateway L2.
 * @dev The HTTPZ contract contains:
 * - the list of KMS nodes used exclusively by this Gateway L2
 * - the list of coprocessors used exclusively by this Gateway L2
 * - the list of networks using this Gateway L2
 *
 * The HTTPZ contract is owned by a DAO governance contract that can be used for initialization.
 * The HTTPZ contract is also managed by administrators that will be allowed to update the state.
 * Some view functions are accessible to everyone (ex: getting the number of KMS nodes).
 */
interface IHTTPZ {
    /// @notice Struct that contains metadata about the protocol
    struct ProtocolMetadata {
        /// @notice Name of the protocol
        string name;
        /// @notice Website of the protocol
        string website;
    }

    /// @notice Struct that represents a KMS (Key Management Service) node
    struct KmsNode {
        /// @notice Address of the KMS node's connector
        address connectorAddress;
        /// @notice Identity of the KMS node (its public signature key)
        bytes identity;
        /// @notice IP address of the KMS node
        string ipAddress;
        /// @notice Address of the KMS node's DA (data availability, an S3 bucket)
        string daAddress;
        /// @notice The TLS certificate to consider for core-to-core communication
        bytes tlsCertificate;
    }

    /// @notice Struct that represents a coprocessor
    struct Coprocessor {
        /// @notice Address of the the coprocessor's transaction sender
        address transactionSenderAddress;
        /// @notice Identity of the coprocessor (its public signature key)
        bytes identity;
        /// @notice Address of the coprocessor's DA (data availability, an S3 bucket)
        string daAddress;
    }

    /// @notice Struct that represents a network
    struct Network {
        /// @notice Chain ID of the network (unique identifier)
        uint256 chainId;
        /// @notice Address where the HTTPZ library contract is deployed
        address httpzExecutor;
        /// @notice Address where the ACL contract is deployed
        address aclAddress;
        /// @notice Name of the network
        string name;
        /// @notice Website of the network
        string website;
    }

    /// @notice Emitted when the HTTPZ initialization is completed
    /// @param metadata Metadata of the protocol
    /// @param admins List of admin addresses
    /// @param kmsThreshold The KMS threshold
    /// @param kmsNodes List of KMS nodes
    /// @param coprocessors List of coprocessors
    /// @param networks List of networks
    event Initialization(
        ProtocolMetadata metadata,
        address[] admins,
        uint256 kmsThreshold,
        KmsNode[] kmsNodes,
        Coprocessor[] coprocessors,
        Network[] networks
    );

    /// @notice Emitted when the KMS threshold has been updated
    /// @param newKmsThreshold The new KMS threshold
    event UpdateKmsThreshold(uint256 newKmsThreshold);

    /// @notice Error emitted when the KMS threshold is too high with respect to the number of KMS nodes
    /// @notice For a set of `n` KMS nodes, the threshold `t` must verify `3t < n`.
    /// @param threshold The threshold
    /// @param nParties The number of KMS nodes
    error KmsThresholdTooHigh(uint256 threshold, uint256 nParties);

    /// @notice Error emitted when a network is not registered
    /// @param chainId The chain ID of the network
    error NetworkNotRegistered(uint256 chainId);

    /// @notice Update the KMS threshold
    /// @dev This function can only be called by an administrator
    /// @dev The new threshold must verify `3t < n`, with `n` the number of KMS nodes currently registered
    /// @param newKmsThreshold The new KMS threshold
    function updateKmsThreshold(uint256 newKmsThreshold) external;

    /// @notice Check if an address is an administrator
    /// @param adminAddress The address to check
    function checkIsAdmin(address adminAddress) external view;

    /// @notice Check if an address is a registered KMS node
    /// @param kmsNodeAddress The address to check
    function checkIsKmsNode(address kmsNodeAddress) external view;

    /// @notice Check if an address is a registered coprocessor
    /// @param coprocessorAddress The address to check
    function checkIsCoprocessor(address coprocessorAddress) external view;

    /// @notice Check if a chain ID corresponds to a registered network
    /// @param chainId The chain ID to check
    function checkNetworkIsRegistered(uint256 chainId) external view;

    /// @notice Get the KMS majority vote threshold
    /// @return The KMS majority vote threshold
    function getKmsMajorityThreshold() external view returns (uint256);

    /// @notice Get the KMS reconstruction threshold
    /// @return The KMS reconstruction threshold
    function getKmsReconstructionThreshold() external view returns (uint256);

    /// @notice Get the coprocessor majority threshold
    /// @return The coprocessor majority threshold
    function getCoprocessorMajorityThreshold() external view returns (uint256);
}
