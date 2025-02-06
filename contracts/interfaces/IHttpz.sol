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
        /// @notice Identity of the KMS node (its public signature key)
        bytes identity;
        /// @notice Gateway address for the KMS node
        address gateway;
        /// @notice IP address of the KMS node
        string ipAddress;
    }

    /// @notice Struct that represents a coprocessor
    struct Coprocessor {
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

    /// @notice Emitted to trigger the initialization of a KMS node
    event KmsNodeInit();

    /// @notice Emitted when all KMS nodes are ready
    event KmsReady();

    /// @notice Emitted to trigger the initialization of a coprocessor
    event CoprocessorInit();

    /// @notice Emitted when all coprocessors are ready
    event CoprocessorReady();

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
    event PreprocessKskRequest(FheParams fheParams);

    /// @notice Emitted when the KSK generation preprocessing is completed
    /// @param preKskId The preprocessed KSK ID
    event PreprocessKskResponse(uint256 preKskId);

    /// @notice Emitted to trigger a key generation
    /// @param preKeyId The preprocessed key ID
    /// @param fheParams The FHE parameters to use
    event KeygenRequest(uint256 preKeyId, FheParams fheParams);

    /// @notice Emitted when the key generation is completed
    /// @param keygenId The generated key ID
    event KeygenResponse(uint256 keygenId);

    /// @notice Emitted to trigger a CRS (Common Reference String) generation
    /// @param fheParams The FHE parameters to use
    event CrsgenRequest(FheParams fheParams);

    /// @notice Emitted when the CRS generation is completed
    /// @param crsId The generated CRS ID
    event CrsgenResponse(uint256 crsId);

    /// @notice Emitted to trigger a KSK generation
    /// @param preKskId The preprocessed KSK ID
    /// @param sourceKeyId The key ID to key switch from
    /// @param destKeyId The key ID to key switch to
    /// @param fheParams The FHE parameters to use
    event KskRequest(uint256 preKskId, uint256 sourceKeyId, uint256 destKeyId, FheParams fheParams);

    /// @notice Emitted when the KSK generation is completed
    /// @param kskId The generated KSK ID
    event KskResponse(uint256 kskId);

    /// @notice Emitted to activate the key in coprocessors
    /// @param keyId The key ID
    event ActivateKeyRequest(uint256 keyId);

    /// @notice Emitted when the key has been activated in all coprocessors
    /// @param keyId The key ID
    event ActivateKeyResponse(uint256 keyId);

    /// @notice Emitted when the FHE parameters have been updated
    /// @param newFheParams The new FHE parameters
    event UpdateFheParams(FheParams newFheParams);

    /// @notice Initialize the protocol
    /// @dev This function can only be called once by the owner
    /// @param protocolMetadata Metadata of the protocol
    /// @param admins List of admin addresses
    function initialize(ProtocolMetadata calldata protocolMetadata, address[] calldata admins) external;

    /// @notice Add KMS nodes
    /// @dev This function can only be called by an administrator
    /// @param kmsNodes List of KMS nodes to add
    function addKmsNodes(KmsNode[] calldata kmsNodes) external;

    /// @notice Mark a KMS node as ready
    /// @dev This function can only be called by a KMS connector
    /// @param signature Signature to verify readiness
    function kmsNodeReady(bytes calldata signature) external;

    /// @notice Add coprocessors
    /// @dev This function can only be called by an administrator
    /// @param coprocessors List of coprocessors to add
    function addCoprocessors(Coprocessor[] calldata coprocessors) external;

    /// @notice Mark a coprocessor as ready
    /// @dev This function can only be called by a coprocessor
    function coprocessorReady() external;

    /// @notice Add a network
    /// @dev This function can only be called by an administrator
    /// @param network The network to add
    function addNetwork(Network calldata network) external;

    /// @notice Trigger a key generation preprocessing
    /// @dev This function can only be called by an administrator
    function preprocessKeygenRequest() external;

    /// @notice Handle the response of a key generation preprocessing
    /// @dev This function can only be called by a KMS connector
    /// @param preKeyId The preprocessed key ID
    function preprocessKeygenResponse(uint256 preKeyId) external;

    /// @notice Trigger a KSK generation preprocessing
    /// @dev This function can only be called by an administrator
    function preprocessKskRequest() external;

    /// @notice Handle the response of a KSK generation preprocessing
    /// @dev This function can only be called by a KMS connector
    /// @param preKskId The preprocessed KSK ID
    function preprocessKskResponse(uint256 preKskId) external;

    /// @notice Trigger a key generation
    /// @dev This function can only be called by an administrator
    /// @param preKeyId The preprocessed key ID
    function keygenRequest(uint256 preKeyId) external;

    /// @notice Handle the response of a key generation
    /// @dev This function can only be called by a KMS connector
    /// @param keyId The generated key ID
    function keygenResponse(uint256 keyId) external;

    /// @notice Trigger a CRS generation
    /// @dev This function can only be called by an administrator
    function crsgenRequest() external;

    /// @notice Handle the response of a CRS generation
    /// @dev This function can only be called by a KMS connector
    /// @param crsId The generated CRS ID
    function crsgenResponse(uint256 crsId) external;

    /// @notice Trigger a KSK generation
    /// @dev This function can only be called by an administrator
    /// @param preKskId The preprocessed KSK ID
    /// @param sourceKeyId The key ID to key switch from
    /// @param destKeyId The key ID to key switch to
    function kskRequest(uint256 preKskId, uint256 sourceKeyId, uint256 destKeyId) external;

    /// @notice Handle the response of a KSK generation
    /// @dev This function can only be called by a KMS connector
    /// @param kskId The generated KSK ID
    function kskResponse(uint256 kskId) external;

    /// @notice Activate the key in coprocessors
    /// @dev This function can only be called by an administrator
    /// @dev A key can only be activated if a key switch key from the current key to this key has
    /// @dev already been generated
    /// @param keyId The key ID
    function activateKeyRequest(uint256 keyId) external;

    /// @notice Handle the response of a key activation
    /// @dev This function can only be called by a coprocessor
    /// @param keyId The key ID
    function activateKeyResponse(uint256 keyId) external;

    /// @notice Update the FHE params
    /// @dev This function can only be called by the owner
    /// @param newFheParams The new FHE params
    function updateFheParams(FheParams memory newFheParams) external;

    /// @notice Get the current key ID
    /// @dev The current key is the latest generated key that has been activated
    /// @return The current key ID
    function getCurrentKeyId() external view returns (uint256);

    // TODO: May not be needed if contracts are made pausable
    // https://github.com/zama-ai/gateway-l2/issues/51
    /// @notice Check if a given key ID is the current one
    /// @dev The current key is the latest generated key that has been activated
    /// @param keyId The key ID to check
    /// @return True if the key ID is the current one, false otherwise
    function isCurrentKeyId(uint256 keyId) external view returns (bool);

    /// @notice Get all KMS nodes
    /// @return List of all KMS nodes
    function getKmsNodes() external view returns (KmsNode[] calldata);

    /// @notice Get all the KMS nodes' identities
    /// @dev An identity is the public signature key of a KMS node
    /// @return List of all the KMS nodes' identities
    function getKmsIdentities() external view returns (bytes[] calldata);

    /// @notice Get all coprocessors
    /// @return List of all coprocessors
    function getCoprocessors() external view returns (Coprocessor[] calldata);

    /// @notice Get all the coprocessors' identities
    /// @dev An identity is the public signature key of a coprocessor
    /// @return List of all the coprocessors' identities
    function getCoprocessorIdentities() external view returns (bytes[] calldata);

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

    /// @notice Get the FHE parameters currently used
    /// @return The FHE parameters
    function getFheParams() external view returns (FheParams memory);
}
