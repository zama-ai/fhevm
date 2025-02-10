// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/**
 * @title Interface for the HTTPZ contract
 * @notice The HTTPZ contract is responsible for being the point of truth for all contracts and
 * components from the Gateway L2.
 * @dev The HTTPZ contract contains:
 * - the list of KMS nodes used exclusively by this Gateway L2
 * - the list of coprocessors used exclusively by this Gateway L2
 * - the list of networks using this Gateway L2
 * - the FHE parameters to consider when generating public materials (FHE keys, CRS)
 * - the FHE keys used by ciphertexts (including the current key to use)
 *
 * The HTTPZ contract is owned by a DAO governance contract that can be used for initialization, as
 * well as updating the FHE parameters.
 * The HTTPZ contract is also managed by administrators that can add KMS nodes, coprocessors,
 * networks and generate (and activate) FHE keys.
 * Most of the functions are restricted to the DAO governance contract or administrators, although
 * some view functions are accessible to everyone (ex: getting the current FHE key Id).
 * Some functions are also restricted to KMS connectors (contracts representing each KMS node)
 * or coprocessors.
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

    // TODO: To be defined : https://github.com/zama-ai/gateway-l2/issues/50
    /// @notice Struct that represents FHE parameters
    /// @dev FHE parameters are used for FHE key, CRS (Common Reference String) and KSK
    /// @dev (Key Switching Key) generation, as well as for the FHE key and KSK preprocessing steps
    struct FheParams {
        /// @notice Placeholder for FHE parameters
        string dummy;
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

    /// @notice Emitted to trigger a key generation preprocessing
    /// @param fheParams The FHE parameters to use
    event PreprocessKeygenRequest(FheParams fheParams);

    /// @notice Emitted when the key generation preprocessing is completed
    /// @param preKeyId The preprocessed key ID
    event PreprocessKeygenResponse(uint256 preKeyId);

    /// @notice Emitted to trigger a KSK generation preprocessing
    /// @param fheParams The FHE parameters to use
    event PreprocessKskgenRequest(FheParams fheParams);

    /// @notice Emitted when the KSK generation preprocessing is completed
    /// @param preKskId The preprocessed KSK ID
    event PreprocessKskgenResponse(uint256 preKskId);

    /// @notice Emitted to trigger a key generation
    /// @param preKeyId The preprocessed key ID
    /// @param fheParams The FHE parameters to use
    event KeygenRequest(uint256 preKeyId, FheParams fheParams);

    /// @notice Emitted when the key generation is completed
    /// @param keygenId The generated key ID
    event KeygenResponse(uint256 keygenId);

    /// @notice Emitted to trigger a CRS (Common Reference String) generation
    /// @param preCrsId The preprocessed CRS ID
    /// @param fheParams The FHE parameters to use
    event CrsgenRequest(uint256 preCrsId, FheParams fheParams);

    /// @notice Emitted when the CRS generation is completed
    /// @param crsId The generated CRS ID
    event CrsgenResponse(uint256 crsId);

    /// @notice Emitted to trigger a KSK generation
    /// @param preKskId The preprocessed KSK ID
    /// @param sourceKeyId The key ID to key switch from
    /// @param destKeyId The key ID to key switch to
    /// @param fheParams The FHE parameters to use
    event KskgenRequest(uint256 preKskId, uint256 sourceKeyId, uint256 destKeyId, FheParams fheParams);

    /// @notice Emitted when the KSK generation is completed
    /// @param kskId The generated KSK ID
    event KskgenResponse(uint256 kskId);

    /// @notice Emitted to activate the key in coprocessors
    /// @param keyId The key ID
    event ActivateKeyRequest(uint256 keyId);

    /// @notice Emitted when the key has been activated in all coprocessors
    /// @param keyId The key ID
    event ActivateKeyResponse(uint256 keyId);

    /// @notice Emitted when the FHE parameters have been updated
    /// @param newFheParams The new FHE parameters
    event UpdateFheParams(FheParams newFheParams);

    /// @notice Error thrown when a key generation preprocessing step is already ongoing
    error PreprocessKeygenAlreadyOngoing();

    /// @notice Error thrown when a key generation preprocessing step is not ongoing
    error PreprocessKeygenNotOngoing();

    /// @notice Error thrown when a key generation preprocessing ID is 0
    error PreprocessKeyIdNull();

    /// @notice Error thrown when a KMS node has already responded to a key generation preprocessing step
    error PreprocessKeygenKmsNodeAlreadyResponded(uint256 preKeyId);

    /// @notice Error thrown when a KSK generation preprocessing step is already ongoing
    error PreprocessKskgenAlreadyOngoing();

    /// @notice Error thrown when a KSK generation preprocessing step is not ongoing
    error PreprocessKskgenNotOngoing();

    /// @notice Error thrown when a KSK generation preprocessing ID is 0
    error PreprocessKskIdNull();

    /// @notice Error thrown when a KMS node has already responded to a KSK generation preprocessing step
    error PreprocessKskgenKmsNodeAlreadyResponded(uint256 preKskId);

    /// @notice Error thrown when a key generation step is already ongoing
    error KeygenAlreadyOngoing();

    /// @notice Error thrown when a key generation step is not ongoing
    error KeygenNotOngoing();

    /// @notice Error thrown when a key generation ID is 0
    error KeyIdNull();

    /// @notice Error thrown when a KMS node has already responded to a key generation step
    error KeygenKmsNodeAlreadyResponded(uint256 keyId);

    /// @notice Error thrown when a key generation step requires preprocessing
    error KeygenRequiresPreprocessing();

    /// @notice Error thrown when a CRS generation step is already ongoing
    error CrsgenAlreadyOngoing();

    /// @notice Error thrown when a CRS generation step is not ongoing
    error CrsgenNotOngoing();

    /// @notice Error thrown when a CRS generation ID is 0
    error CrsIdNull();

    /// @notice Error thrown when a KMS node has already responded to a CRS generation step
    error CrsgenKmsNodeAlreadyResponded(uint256 crsId);

    /// @notice Error thrown when a KSK generation step is already ongoing
    error KskgenAlreadyOngoing();

    /// @notice Error thrown when a KSK generation step is not ongoing
    error KskgenNotOngoing();

    /// @notice Error thrown when a KSK generation source key ID is 0
    error KskgenSourceKeyIdNull();

    /// @notice Error thrown when a KSK generation destination key ID is 0
    error KskgenDestKeyIdNull();

    /// @notice Error thrown when a KSK generation step requires preprocessing
    error KskgenRequiresPreprocessing();

    /// @notice Error thrown when a KSK generation ID is 0
    error KskIdNull();

    /// @notice Error thrown when a KMS node has already responded to a KSK generation step
    error KskgenKmsNodeAlreadyResponded(uint256 kskId);

    /// @notice Error thrown when a key activation step requires a key generation step
    error ActivateKeyRequiresKeygen(uint256 currentKeyId, uint256 pendingKeyId);

    /// @notice Error thrown when a key activation step requires a KSK generation step
    error ActivateKeyRequiresKskgen(uint256 currentKeyId, uint256 pendingKeyId);

    /// @notice Error thrown when a key activation step is already ongoing
    error ActivateKeyAlreadyOngoing();

    /// @notice Error thrown when a key activation step is not ongoing
    error ActivateKeyNotOngoing();

    /// @notice Error thrown when a key activation step key ID is 0
    error ActivateKeyKeyIdNull();

    /// @notice Error thrown when a KMS node has already responded to a key activation step
    error ActivateKeyKmsNodeAlreadyResponded(uint256 keyId);

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
