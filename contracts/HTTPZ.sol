// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

import "./interfaces/IHTTPZ.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title HTTPZ contract
 * @dev See {IHTTPZ}.
 * @dev Add/remove methods will be added in the future for admins, KMS nodes, coprocessors and networks.
 * @dev See https://github.com/zama-ai/gateway-l2/issues/98 for more details.
 */
contract HTTPZ is IHTTPZ, Ownable2Step, AccessControl {
    /// @notice The protocol's metadata
    ProtocolMetadata public protocolMetadata;

    /// @notice The admin role. For example, only admins can add KMS nodes, coprocessors and networks
    /// @notice (HTTPZ contract), trigger public material generation or set/update FHE parameters
    /// @notice (in Key Manager).
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");

    /// @notice The KMS nodes' metadata
    KmsNode[] public kmsNodes;
    /// @notice The KMS' threshold to consider for majority vote or reconstruction. For a set ot `n`
    /// @notice KMS nodes, the threshold `t` must verify `3t < n`.
    uint256 public kmsThreshold;
    /// @notice The KMS node role. For example, only KMS nodes can send response transactions during
    /// @notice public material generation (in Key Manager) or decryption (in Decryption Manager).
    bytes32 public constant KMS_NODE_ROLE = keccak256("KMS_NODE_ROLE");

    /// @notice The coprocessors' metadata
    Coprocessor[] public coprocessors;
    /// @notice The coprocessor role. For example, only coprocessors can send response transactions
    /// @notice during key activation (in Key Manager) or ZK Proof verification (in ZKPoK Manager).
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
        ProtocolMetadata calldata initialMetadata,
        address[] calldata initialAdmins,
        uint256 initialKmsThreshold,
        KmsNode[] calldata initialKmsNodes,
        Coprocessor[] calldata initialCoprocessors
    ) external virtual onlyOwner {
        protocolMetadata = initialMetadata;

        /// @dev Register the admins
        for (uint256 i = 0; i < initialAdmins.length; i++) {
            _grantRole(ADMIN_ROLE, initialAdmins[i]);
        }

        uint256 nParties = initialKmsNodes.length;

        /// @dev Check that this KMS node's threshold is valid. For a set ot `n` KMS nodes, the
        /// @dev threshold `t` must verify `3t < n`.
        if (3 * initialKmsThreshold >= nParties) {
            revert KmsThresholdTooHigh(initialKmsThreshold, nParties);
        }

        /// @dev Set the KMS threshold.
        kmsThreshold = initialKmsThreshold;

        /// @dev Register the KMS nodes
        for (uint256 i = 0; i < nParties; i++) {
            _grantRole(KMS_NODE_ROLE, initialKmsNodes[i].connectorAddress);
            kmsNodes.push(initialKmsNodes[i]);
        }

        /// @dev Register the coprocessors
        for (uint256 i = 0; i < initialCoprocessors.length; i++) {
            _grantRole(COPROCESSOR_ROLE, initialCoprocessors[i].connectorAddress);
            coprocessors.push(initialCoprocessors[i]);
        }

        emit Initialization(initialMetadata, initialAdmins, initialKmsThreshold, initialKmsNodes, initialCoprocessors);
    }

    /// @dev See {IHTTPZ-addNetwork}.
    function addNetwork(Network calldata network) external virtual onlyRole(ADMIN_ROLE) {
        networks.push(network);
        _isNetworkRegistered[network.chainId] = true;

        emit AddNetwork(network.chainId);
    }

    /// @dev See {IHTTPZ-updateKmsThreshold}.
    function updateKmsThreshold(uint256 newKmsThreshold) external virtual onlyRole(ADMIN_ROLE) {
        if (3 * newKmsThreshold >= kmsNodes.length) {
            revert KmsThresholdTooHigh(newKmsThreshold, kmsNodes.length);
        }

        kmsThreshold = newKmsThreshold;
        emit UpdateKmsThreshold(newKmsThreshold);
    }

    /// @dev See {IHTTPZ-isAdmin}.
    function isAdmin(address adminAddress) external view virtual returns (bool) {
        return hasRole(ADMIN_ROLE, adminAddress);
    }

    /// @dev See {IHTTPZ-isKmsNode}.
    function isKmsNode(address kmsNodeAddress) external view virtual returns (bool) {
        return hasRole(KMS_NODE_ROLE, kmsNodeAddress);
    }

    /// @dev See {IHTTPZ-isCoprocessor}.
    function isCoprocessor(address coprocessorAddress) external view virtual returns (bool) {
        return hasRole(COPROCESSOR_ROLE, coprocessorAddress);
    }

    /// @dev See {IHTTPZ-isNetwork}.
    function isNetwork(uint256 chainId) external view virtual returns (bool) {
        return _isNetworkRegistered[chainId];
    }

    /// @dev See {IHTTPZ-getKmsMajorityThreshold}.
    function getKmsMajorityThreshold() external view virtual returns (uint256) {
        return kmsThreshold + 1;
    }

    /// @dev See {IHTTPZ-getKmsReconstructionThreshold}.
    function getKmsReconstructionThreshold() external view virtual returns (uint256) {
        return 2 * kmsThreshold + 1;
    }

    /// @dev See {IHTTPZ-getCoprocessorMajorityThreshold}.
    function getCoprocessorMajorityThreshold() external view virtual returns (uint256) {
        return coprocessors.length / 2 + 1;
    }

    /// @notice Returns the versions of the HTTPZ contract in SemVer format.
    /// @dev This is conventionally used for upgrade features.
    function getVersion() public pure virtual returns (string memory) {
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
