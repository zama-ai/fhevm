// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./interfaces/IHTTPZ.sol";
import { AccessControlUpgradeable } from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title HTTPZ contract
 * @dev See {IHTTPZ}.
 * @dev Add/remove methods will be added in the future for admins, KMS nodes, coprocessors and networks.
 * @dev See https://github.com/zama-ai/gateway-l2/issues/98 for more details.
 */
contract HTTPZ is IHTTPZ, AccessControlUpgradeable, Ownable2StepUpgradeable, UUPSUpgradeable {
    /// @notice The admin role. For example, only admins can update the KMS threshold (HTTPZ contract)
    /// @notice trigger public material generation or set/update FHE parameters (in Key Manager).
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");

    /// @notice The KMS node role. For example, only KMS nodes can send response transactions during
    /// @notice public material generation (in Key Manager) or decryption (in Decryption Manager).
    bytes32 public constant KMS_NODE_ROLE = keccak256("KMS_NODE_ROLE");

    /// @notice The coprocessor role. For example, only coprocessors can send response transactions
    /// @notice during key activation (in Key Manager) or ZK Proof verification (in ZKPoK Manager).
    bytes32 public constant COPROCESSOR_ROLE = keccak256("COPROCESSOR_ROLE");

    /// @notice The contract's metadata
    string private constant CONTRACT_NAME = "HTTPZ";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:httpz_gateway.storage.HTTPZ
    struct HTTPZStorage {
        /// @notice The protocol's metadata
        ProtocolMetadata protocolMetadata;
        /// @notice The KMS nodes' metadata
        mapping(address kmsNodeAddress => KmsNode kmsNode) kmsNodes;
        /// @notice The KMS nodes' addresses
        address[] kmsNodeAddresses;
        /// @notice The KMS' threshold to consider for majority vote or reconstruction. For a set of `n`
        /// @notice KMS nodes, the threshold `t` must verify `3t < n`.
        uint256 kmsThreshold;
        /// @notice The coprocessors' metadata
        mapping(address coprocessorAddress => Coprocessor coprocessor) coprocessors;
        /// @notice The coprocessors' addresses
        address[] coprocessorAddresses;
        /// @notice The networks' metadata
        Network[] networks;
        /// @notice The networks' registered status
        mapping(uint256 chainId => bool isRegistered) _isNetworkRegistered;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("httpz_gateway.storage.HTTPZ")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant HTTPZ_STORAGE_LOCATION =
        0x827176a45e1aad1f3a6539fee60c06126c40427b4849e7301bf2cf0f1f8e9500;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract
    /// @param initialMetadata Metadata of the protocol
    /// @param initialAdmins List of admin addresses
    /// @param initialKmsThreshold The KMS threshold. Must verify `3t < n` for `n` KMS nodes.
    /// @param initialKmsNodes List of KMS nodes
    /// @param initialCoprocessors List of coprocessors
    /// @param initialNetworks List of networks
    function initialize(
        ProtocolMetadata memory initialMetadata,
        address[] memory initialAdmins,
        uint256 initialKmsThreshold,
        KmsNode[] memory initialKmsNodes,
        Coprocessor[] memory initialCoprocessors,
        Network[] memory initialNetworks
    ) public reinitializer(2) {
        __Ownable_init(msg.sender);

        HTTPZStorage storage $ = _getHTTPZStorage();
        $.protocolMetadata = initialMetadata;

        /// @dev Register the admins
        for (uint256 i = 0; i < initialAdmins.length; i++) {
            _grantRole(ADMIN_ROLE, initialAdmins[i]);
        }

        uint256 nParties = initialKmsNodes.length;

        /// @dev Check that this KMS node's threshold is valid. For a set of `n` KMS nodes, the
        /// @dev threshold `t` must verify `3t < n`.
        if (3 * initialKmsThreshold >= nParties) {
            revert KmsThresholdTooHigh(initialKmsThreshold, nParties);
        }

        /// @dev Set the KMS threshold.
        $.kmsThreshold = initialKmsThreshold;

        /// @dev Register the KMS nodes
        for (uint256 i = 0; i < nParties; i++) {
            _grantRole(KMS_NODE_ROLE, initialKmsNodes[i].connectorAddress);
            $.kmsNodes[initialKmsNodes[i].connectorAddress] = initialKmsNodes[i];
            $.kmsNodeAddresses.push(initialKmsNodes[i].connectorAddress);
        }

        /// @dev Register the coprocessors
        for (uint256 i = 0; i < initialCoprocessors.length; i++) {
            _grantRole(COPROCESSOR_ROLE, initialCoprocessors[i].transactionSenderAddress);
            $.coprocessors[initialCoprocessors[i].transactionSenderAddress] = initialCoprocessors[i];
            $.coprocessorAddresses.push(initialCoprocessors[i].transactionSenderAddress);
        }

        /// @dev Register the networks
        for (uint256 i = 0; i < initialNetworks.length; i++) {
            $.networks.push(initialNetworks[i]);
            $._isNetworkRegistered[initialNetworks[i].chainId] = true;
        }

        emit Initialization(
            initialMetadata,
            initialAdmins,
            initialKmsThreshold,
            initialKmsNodes,
            initialCoprocessors,
            initialNetworks
        );
    }

    /// @dev See {IHTTPZ-updateKmsThreshold}.
    function updateKmsThreshold(uint256 newKmsThreshold) external virtual onlyRole(ADMIN_ROLE) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        if (3 * newKmsThreshold >= $.kmsNodeAddresses.length) {
            revert KmsThresholdTooHigh(newKmsThreshold, $.kmsNodeAddresses.length);
        }

        $.kmsThreshold = newKmsThreshold;
        emit UpdateKmsThreshold(newKmsThreshold);
    }

    /// @dev See {IHTTPZ-checkIsAdmin}.
    function checkIsAdmin(address adminAddress) external view virtual {
        _checkRole(ADMIN_ROLE, adminAddress);
    }

    /// @dev See {IHTTPZ-checkIsKmsNode}.
    function checkIsKmsNode(address kmsNodeAddress) external view virtual {
        _checkRole(KMS_NODE_ROLE, kmsNodeAddress);
    }

    /// @dev See {IHTTPZ-checkIsCoprocessor}.
    function checkIsCoprocessor(address coprocessorAddress) external view virtual {
        _checkRole(COPROCESSOR_ROLE, coprocessorAddress);
    }

    /// @dev See {IHTTPZ-checkNetworkIsRegistered}.
    function checkNetworkIsRegistered(uint256 chainId) external view virtual {
        HTTPZStorage storage $ = _getHTTPZStorage();
        if (!$._isNetworkRegistered[chainId]) {
            revert NetworkNotRegistered(chainId);
        }
    }

    /// @dev See {IHTTPZ-getProtocolMetadata}.
    function getProtocolMetadata() external view virtual returns (ProtocolMetadata memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.protocolMetadata;
    }

    /// @dev See {IHTTPZ-getKmsThreshold}.
    function getKmsThreshold() external view virtual returns (uint256) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.kmsThreshold;
    }

    /// @dev See {IHTTPZ-getKmsMajorityThreshold}.
    function getKmsMajorityThreshold() external view virtual returns (uint256) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.kmsThreshold + 1;
    }

    /// @dev See {IHTTPZ-getKmsReconstructionThreshold}.
    function getKmsReconstructionThreshold() external view virtual returns (uint256) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return 2 * $.kmsThreshold + 1;
    }

    /// @dev See {IHTTPZ-getCoprocessorMajorityThreshold}.
    function getCoprocessorMajorityThreshold() external view virtual returns (uint256) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.coprocessorAddresses.length / 2 + 1;
    }

    /// @dev See {IHTTPZ-kmsNodes}.
    function kmsNodes(address addr) external view virtual returns (KmsNode memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.kmsNodes[addr];
    }

    /// @dev See {IHTTPZ-kmsNodeAddresses}.
    function kmsNodeAddresses(uint256 index) external view virtual returns (address) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.kmsNodeAddresses[index];
    }

    /// @dev See {IHTTPZ-getAllKmsNodeAddresses}.
    function getAllKmsNodeAddresses() external view virtual returns (address[] memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.kmsNodeAddresses;
    }

    /// @dev See {IHTTPZ-coprocessors}.
    function coprocessors(address addr) external view virtual returns (Coprocessor memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.coprocessors[addr];
    }

    /// @dev See {IHTTPZ-coprocessorAddresses}.
    function coprocessorAddresses(uint256 index) external view virtual returns (address) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.coprocessorAddresses[index];
    }

    /// @dev See {IHTTPZ-getAllCoprocessorAddresses}.
    function getAllCoprocessorAddresses() external view virtual returns (address[] memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.coprocessorAddresses;
    }

    /// @dev See {IHTTPZ-networks}.
    function networks(uint256 index) external view virtual returns (Network memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.networks[index];
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

    /**
     * @dev Returns the HTTPZ storage location.
     */
    function _getHTTPZStorage() internal pure returns (HTTPZStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := HTTPZ_STORAGE_LOCATION
        }
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}
}
