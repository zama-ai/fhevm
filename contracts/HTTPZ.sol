// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

import "./interfaces/IHTTPZ.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

/// @title HTTPZ contract
/// @dev See {IHTTPZ}.
contract HTTPZ is IHTTPZ, Ownable2Step, AccessControl {
    /// @notice The protocol's metadata
    ProtocolMetadata public protocolMetadata;

    /// @notice The admin role. Only admins can add KMS nodes, coprocessors and networks, as well
    /// @notice as trigger FHE key, CRS and KSK generations
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");

    /// @notice The KMS nodes' metadata
    KmsNode[] public kmsNodes;
    /// @notice The KMS nodes' identities (public signature keys)
    bytes[] public kmsNodeIdentities;
    /// @notice The KMS nodes' signed nodes
    mapping(address kmsNodeConnector => bytes signedNodes) public kmsNodeSignedNodes;
    /// @notice The keychain DA addresses (one per KMS node)
    mapping(address kmsNodeConnector => address keychainDaAddress) public keychainDaAddresses;
    /// @notice The number of KMS nodes that are marked as ready
    uint256 private _kmsNodeReadyCounter;
    /// @notice The pending KMS node role. Only pending KMS nodes can mark KMS nodes as ready
    bytes32 public constant PENDING_KMS_NODE_ROLE = keccak256("PENDING_KMS_NODE_ROLE");
    /// @notice The KMS node role. Only KMS nodes can respond to FHE key, CRS and KSK generations
    bytes32 public constant KMS_NODE_ROLE = keccak256("KMS_NODE_ROLE");

    /// @notice The coprocessors' metadata
    Coprocessor[] public coprocessors;
    /// @notice The coprocessors' identities (public signature keys)
    bytes[] public coprocessorIdentities;
    /// @notice The coprocessor DA addresses (one per coprocessor)
    mapping(address coprocessorConnector => address coprocessorDaAddress) public coprocessorDaAddresses;
    /// @notice The number of coprocessors that are marked as ready
    uint256 private _coprocessorReadyCounter;
    /// @notice The pending coprocessor role. Only pending coprocessors can mark coprocessors as ready
    bytes32 public constant PENDING_COPROCESSOR_ROLE = keccak256("PENDING_COPROCESSOR_ROLE");
    /// @notice The coprocessor role. Only coprocessors can respond to key activation
    bytes32 public constant COPROCESSOR_ROLE = keccak256("COPROCESSOR_ROLE");

    /// @notice The networks' metadata
    Network[] public networks;
    /// @notice The networks' registered status
    mapping(uint256 chainId => bool isRegistered) private _isNetworkRegistered;

    /// @notice The contract's metadata
    string private constant CONTRACT_NAME = "HTTPZ";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    constructor() Ownable(msg.sender) {}

    /// @dev See {IHTTPZ-initialize}.
    function initialize(
        ProtocolMetadata calldata initialProtocolMetadata,
        address[] calldata admins
    ) external onlyOwner {
        protocolMetadata = initialProtocolMetadata;

        for (uint256 i = 0; i < admins.length; i++) {
            _grantRole(ADMIN_ROLE, admins[i]);
        }

        emit Initialization(protocolMetadata, admins);
    }

    /// @dev See {IHTTPZ-addKmsNodes}.
    function addKmsNodes(KmsNode[] calldata initialKmsNodes) external onlyRole(ADMIN_ROLE) {
        for (uint256 i = 0; i < initialKmsNodes.length; i++) {
            _grantRole(PENDING_KMS_NODE_ROLE, initialKmsNodes[i].connectorAddress);
            kmsNodes.push(initialKmsNodes[i]);
            kmsNodeIdentities.push(initialKmsNodes[i].identity);
        }

        emit KmsNodesInit(kmsNodeIdentities);
    }

    /// @dev See {IHTTPZ-kmsNodeReady}.
    function kmsNodeReady(
        bytes calldata signedNodes,
        address keychainDaAddress
    ) external onlyRole(PENDING_KMS_NODE_ROLE) {
        _grantRole(KMS_NODE_ROLE, msg.sender);

        /// @dev A KMS node can only be ready once
        _revokeRole(PENDING_KMS_NODE_ROLE, msg.sender);

        kmsNodeSignedNodes[msg.sender] = signedNodes;
        keychainDaAddresses[msg.sender] = keychainDaAddress;
        _kmsNodeReadyCounter++;

        /// @dev Emit the event when all KMS nodes are ready
        if (_kmsNodeReadyCounter == kmsNodes.length) {
            emit KmsServiceReady(kmsNodeIdentities);
        }
    }

    /// @dev See {IHTTPZ-addCoprocessors}.
    function addCoprocessors(Coprocessor[] calldata initialCoprocessors) external onlyRole(ADMIN_ROLE) {
        for (uint256 i = 0; i < initialCoprocessors.length; i++) {
            _grantRole(PENDING_COPROCESSOR_ROLE, initialCoprocessors[i].connectorAddress);
            coprocessors.push(initialCoprocessors[i]);
            coprocessorIdentities.push(initialCoprocessors[i].identity);
        }

        emit CoprocessorsInit(coprocessorIdentities);
    }

    /// @dev See {IHTTPZ-coprocessorReady}.
    function coprocessorReady(address coprocessorDaAddress) external onlyRole(PENDING_COPROCESSOR_ROLE) {
        _grantRole(COPROCESSOR_ROLE, msg.sender);

        /// @dev A coprocessor can only be ready once
        _revokeRole(PENDING_COPROCESSOR_ROLE, msg.sender);

        coprocessorDaAddresses[msg.sender] = coprocessorDaAddress;
        _coprocessorReadyCounter++;

        /// @dev Emit the event when all coprocessors are ready
        if (_coprocessorReadyCounter == coprocessors.length) {
            emit CoprocessorServiceReady(coprocessorIdentities);
        }
    }

    /// @dev See {IHTTPZ-addNetwork}.
    function addNetwork(Network calldata network) external onlyRole(ADMIN_ROLE) {
        networks.push(network);
        _isNetworkRegistered[network.chainId] = true;

        emit AddNetwork(network.chainId);
    }

    /// @dev See {IHTTPZ-isKmsNode}.
    function isKmsNode(address kmsNodeAddress) external view returns (bool) {
        return hasRole(KMS_NODE_ROLE, kmsNodeAddress);
    }

    /// @dev See {IHTTPZ-isCoprocessor}.
    function isCoprocessor(address coprocessorAddress) external view returns (bool) {
        return hasRole(COPROCESSOR_ROLE, coprocessorAddress);
    }

    /// @dev See {IHTTPZ-isNetwork}.
    function isNetwork(uint256 chainId) external view returns (bool) {
        return _isNetworkRegistered[chainId];
    }

    /// @notice Returns the versions of the HTTPZ contract in SemVer format.
    /// @dev This is conventionally used for upgrade features.
    function getVersion() public pure returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    " v",
                    Strings.toString(MAJOR_VERSION),
                    ".",
                    Strings.toString(MINOR_VERSION),
                    ".",
                    Strings.toString(PATCH_VERSION)
                )
            );
    }
}
