// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

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
 * The HTTPZ contract is also managed by administrators that can add KMS nodes, coprocessors and
 * networks.
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
    }

    /// @notice Struct that represents a coprocessor
    struct Coprocessor {
        /// @notice Address of the coprocessor's connector
        address connectorAddress;
        /// @notice Identity of the coprocessor (its public signature key)
        bytes identity;
    }

    /// @notice Struct that represents a network
    struct Network {
        /// @notice Chain ID of the network (unique identifier)
        uint256 chainId;
        /// @notice Address where the HTTPZ library contract is deployed
        address httpzLibrary;
        /// @notice Address where the ACL contract is deployed
        address acl;
        /// @notice Name of the network
        string name;
        /// @notice Website of the network
        string website;
    }

    /// @notice Emitted when the HTTPZ initialization is completed
    /// @param protocolMetadata Metadata of the protocol
    /// @param admins List of admin addresses
    event Initialization(ProtocolMetadata protocolMetadata, address[] admins);

    /// @notice Emitted to trigger the initialization of KMS nodes
    /// @param identities List of KMS nodes' identities
    event KmsNodesInit(bytes[] identities);

    /// @notice Emitted when all KMS nodes are ready
    /// @param identities List of KMS nodes' identities
    event KmsServiceReady(bytes[] identities);

    /// @notice Emitted to trigger the initialization of coprocessors
    /// @param identities List of coprocessors' identities
    event CoprocessorsInit(bytes[] identities);

    /// @notice Emitted when all coprocessors are ready
    /// @param identities List of coprocessors' identities
    event CoprocessorServiceReady(bytes[] identities);

    /// @notice Emitted when a network has been added
    /// @param chainId The chain ID of the network
    event AddNetwork(uint256 chainId);

    /// @notice Error thrown when KMS nodes are not set
    error KmsNodesNotSet();

    /// @notice Error thrown when coprocessors are not set
    error CoprocessorsNotSet();

    /// @notice Initialize the protocol
    /// @dev This function can only be called once by the owner
    /// @param protocolMetadata Metadata of the protocol
    /// @param admins List of admin addresses
    function initialize(ProtocolMetadata calldata protocolMetadata, address[] calldata admins) external;

    /// @notice Add KMS nodes
    /// @dev This function can only be called by an administrator
    /// @param initialKmsNodes List of KMS nodes to add
    function addKmsNodes(KmsNode[] calldata initialKmsNodes) external;

    /// @notice Mark a KMS node as ready
    /// @dev This function can only be called by a KMS connector
    /// @param signedNodes Signed nodes to verify readiness
    /// @param keychainDaAddress Address of the KMS node's keychain DA
    function kmsNodeReady(bytes calldata signedNodes, address keychainDaAddress) external;

    /// @notice Add coprocessors
    /// @dev This function can only be called by an administrator
    /// @param initialCoprocessors List of coprocessors to add
    function addCoprocessors(Coprocessor[] calldata initialCoprocessors) external;

    /// @notice Mark a coprocessor as ready
    /// @dev This function can only be called by a coprocessor
    /// @param coprocessorDaAddress Address of the coprocessor's DA
    function coprocessorReady(address coprocessorDaAddress) external;

    /// @notice Add a network
    /// @dev This function can only be called by an administrator
    /// @param network The network to add
    function addNetwork(Network calldata network) external;

    /// @notice Check if an address is an administrator
    /// @param adminAddress The address to check
    /// @return True if the address is an administrator, false otherwise
    function isAdmin(address adminAddress) external view returns (bool);

    /// @notice Check if an address is a registered KMS node
    /// @param kmsNodeAddress The address to check
    /// @return True if the address is a registered KMS node, false otherwise
    function isKmsNode(address kmsNodeAddress) external view returns (bool);

    /// @notice Check if an address is a registered coprocessor
    /// @param coprocessorAddress The address to check
    /// @return True if the address is a registered coprocessor, false otherwise
    function isCoprocessor(address coprocessorAddress) external view returns (bool);

    /// @notice Check if a chain ID corresponds to a registered network
    /// @param chainId The chain ID to check
    /// @return True if the chain ID corresponds to a registered network, false otherwise
    function isNetwork(uint256 chainId) external view returns (bool);

    /// @notice Get the number of KMS nodes
    /// @return The number of KMS nodes
    function getKmsNodesCount() external view returns (uint256);

    /// @notice Get the number of coprocessors
    /// @return The number of coprocessors
    function getCoprocessorsCount() external view returns (uint256);
}
