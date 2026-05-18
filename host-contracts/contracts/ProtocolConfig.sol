// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IProtocolConfig} from "./interfaces/IProtocolConfig.sol";
import {KmsNode} from "./shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE} from "./shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title ProtocolConfig
 * @notice Manages KMS node sets, thresholds, and context lifecycle on the Ethereum host chain.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ProtocolConfig is IProtocolConfig, UUPSUpgradeableEmptyProxy, ACLOwnable {
    // -----------------------------------------------------------------------------------------
    // Contract information
    // -----------------------------------------------------------------------------------------

    string private constant CONTRACT_NAME = "ProtocolConfig";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Shared between `initializeFromEmptyProxy` and `initializeFromMigration`.
    uint64 private constant REINITIALIZER_VERSION = 2;

    // -----------------------------------------------------------------------------------------
    // ERC-7201 namespaced storage
    // -----------------------------------------------------------------------------------------

    /// @custom:storage-location erc7201:fhevm.storage.ProtocolConfig
    struct ProtocolConfigStorage {
        /// @notice Current KMS context ID counter.
        uint256 currentKmsContextId;
        /// @notice KMS nodes per context.
        mapping(uint256 contextId => KmsNode[]) kmsNodesForContext;
        /// @notice Tx sender lookup per context.
        mapping(uint256 contextId => mapping(address => bool)) isKmsTxSenderForContext;
        /// @notice Signer lookup per context.
        mapping(uint256 contextId => mapping(address => bool)) isKmsSignerForContext;
        /// @notice KmsNode by tx sender per context.
        mapping(uint256 contextId => mapping(address => KmsNode)) kmsNodeByTxSenderForContext;
        /// @notice Signer addresses per context (for ordered iteration).
        mapping(uint256 contextId => address[]) kmsSignerAddressesForContext;
        /// @notice Public decryption threshold per context.
        mapping(uint256 contextId => uint256) publicDecryptionThresholdForContext;
        /// @notice User decryption threshold per context.
        mapping(uint256 contextId => uint256) userDecryptionThresholdForContext;
        /// @notice KmsGen threshold per context.
        mapping(uint256 contextId => uint256) kmsGenThresholdForContext;
        /// @notice MPC threshold per context.
        mapping(uint256 contextId => uint256) mpcThresholdForContext;
        /// @notice Whether a context has been destroyed.
        mapping(uint256 contextId => bool) destroyedContexts;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.ProtocolConfig")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PROTOCOL_CONFIG_STORAGE_LOCATION =
        0x80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200;

    function _getProtocolConfigStorage() internal pure returns (ProtocolConfigStorage storage $) {
        assembly {
            $.slot := PROTOCOL_CONFIG_STORAGE_LOCATION
        }
    }

    // -----------------------------------------------------------------------------------------
    // Constructor
    // -----------------------------------------------------------------------------------------

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    // -----------------------------------------------------------------------------------------
    // Initialization
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Fresh deploy initializer: creates the first KMS context.
     * @param initialKmsNodes The initial KMS node set.
     * @param initialThresholds The initial thresholds.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        KmsNode[] calldata initialKmsNodes,
        KmsThresholds calldata initialThresholds
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        $.currentKmsContextId = KMS_CONTEXT_COUNTER_BASE;
        _defineKmsContext(initialKmsNodes, initialThresholds);
    }

    /**
     * @notice Migration initializer: seeds the migrated context from an existing KMSVerifier state.
     * @param existingContextId The context ID from the old KMSVerifier to preserve. The counter is
     *        seeded to `existingContextId - 1` so that `_defineKmsContext` increments to the exact
     *        old ID, preserving context continuity for downstream readers.
     * @param existingKmsNodes The existing KMS node set to migrate.
     * @param existingThresholds The existing thresholds to migrate.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromMigration(
        uint256 existingContextId,
        KmsNode[] calldata existingKmsNodes,
        KmsThresholds calldata existingThresholds
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        if (existingContextId < KMS_CONTEXT_COUNTER_BASE + 1) {
            revert InvalidKmsContext(existingContextId);
        }

        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        // Seed counter so _defineKmsContext's ++counter lands on the original context ID
        $.currentKmsContextId = existingContextId - 1;
        _defineKmsContext(existingKmsNodes, existingThresholds);
    }

    // -----------------------------------------------------------------------------------------
    // State-changing functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc IProtocolConfig
    function defineNewKmsContext(
        KmsNode[] calldata kmsNodes,
        KmsThresholds calldata thresholds
    ) external virtual onlyACLOwner {
        _defineKmsContext(kmsNodes, thresholds);
    }

    /// @inheritdoc IProtocolConfig
    function destroyKmsContext(uint256 kmsContextId) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        if (kmsContextId == $.currentKmsContextId) {
            revert CurrentKmsContextCannotBeDestroyed(kmsContextId);
        }
        if (!_isValidKmsContext(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }

        $.destroyedContexts[kmsContextId] = true;
        emit KmsContextDestroyed(kmsContextId);
    }

    // -----------------------------------------------------------------------------------------
    // View functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc IProtocolConfig
    function getCurrentKmsContextId() external view virtual returns (uint256) {
        return _getProtocolConfigStorage().currentKmsContextId;
    }

    /// @inheritdoc IProtocolConfig
    function isValidKmsContext(uint256 kmsContextId) external view virtual returns (bool) {
        return _isValidKmsContext(kmsContextId);
    }

    /// @inheritdoc IProtocolConfig
    function getKmsSigners() external view virtual returns (address[] memory) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.kmsSignerAddressesForContext[$.currentKmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getKmsSignersForContext(uint256 kmsContextId) external view virtual returns (address[] memory) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsSignerAddressesForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function isKmsSigner(address signer) external view virtual returns (bool) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.isKmsSignerForContext[$.currentKmsContextId][signer];
    }

    /// @inheritdoc IProtocolConfig
    function isKmsSignerForContext(uint256 kmsContextId, address signer) external view virtual returns (bool) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().isKmsSignerForContext[kmsContextId][signer];
    }

    /// @inheritdoc IProtocolConfig
    function getKmsNodesForContext(uint256 kmsContextId) external view virtual returns (KmsNode[] memory) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsNodesForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view virtual returns (bool) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().isKmsTxSenderForContext[kmsContextId][txSender];
    }

    /// @inheritdoc IProtocolConfig
    function getKmsNodeForContext(
        uint256 kmsContextId,
        address txSender
    ) external view virtual returns (KmsNode memory) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsNodeByTxSenderForContext[kmsContextId][txSender];
    }

    /// @inheritdoc IProtocolConfig
    function getPublicDecryptionThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.publicDecryptionThresholdForContext[$.currentKmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getPublicDecryptionThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().publicDecryptionThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getUserDecryptionThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.userDecryptionThresholdForContext[$.currentKmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getUserDecryptionThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().userDecryptionThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getKmsGenThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.kmsGenThresholdForContext[$.currentKmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getKmsGenThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsGenThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getMpcThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.mpcThresholdForContext[$.currentKmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getMpcThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().mpcThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getVersion() external pure virtual returns (string memory) {
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

    // -----------------------------------------------------------------------------------------
    // Internal
    // -----------------------------------------------------------------------------------------

    /**
     * @dev Creates a new KMS context, validates nodes and thresholds, and stores them.
     */
    function _defineKmsContext(
        KmsNode[] calldata kmsNodes,
        KmsThresholds calldata thresholds
    ) internal virtual returns (uint256 newContextId) {
        if (kmsNodes.length == 0) {
            revert EmptyKmsNodes();
        }

        _validateThresholds(thresholds, kmsNodes.length);

        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        newContextId = ++$.currentKmsContextId;

        for (uint256 i = 0; i < kmsNodes.length; i++) {
            KmsNode calldata node = kmsNodes[i];

            if (node.txSenderAddress == address(0)) {
                revert KmsNodeNullTxSender();
            }
            if (node.signerAddress == address(0)) {
                revert KmsNodeNullSigner();
            }
            if ($.isKmsTxSenderForContext[newContextId][node.txSenderAddress]) {
                revert KmsTxSenderAlreadyRegistered(node.txSenderAddress);
            }
            if ($.isKmsSignerForContext[newContextId][node.signerAddress]) {
                revert KmsSignerAlreadyRegistered(node.signerAddress);
            }

            $.kmsNodesForContext[newContextId].push(node);
            $.isKmsTxSenderForContext[newContextId][node.txSenderAddress] = true;
            $.isKmsSignerForContext[newContextId][node.signerAddress] = true;
            $.kmsNodeByTxSenderForContext[newContextId][node.txSenderAddress] = node;
            $.kmsSignerAddressesForContext[newContextId].push(node.signerAddress);
        }

        $.publicDecryptionThresholdForContext[newContextId] = thresholds.publicDecryption;
        $.userDecryptionThresholdForContext[newContextId] = thresholds.userDecryption;
        $.kmsGenThresholdForContext[newContextId] = thresholds.kmsGen;
        $.mpcThresholdForContext[newContextId] = thresholds.mpc;

        emit NewKmsContext(newContextId, kmsNodes, thresholds);
    }

    /**
     * @dev Validates that thresholds are non-zero and not exceeding the node count.
     */
    function _validateThresholds(KmsThresholds calldata thresholds, uint256 nodeCount) internal pure virtual {
        _checkThreshold("publicDecryption", thresholds.publicDecryption, nodeCount);
        _checkThreshold("userDecryption", thresholds.userDecryption, nodeCount);
        _checkThreshold("kmsGen", thresholds.kmsGen, nodeCount);
        _checkThreshold("mpc", thresholds.mpc, nodeCount);
    }

    /**
     * @dev Validates a single threshold: must be non-zero and at most nodeCount.
     */
    function _checkThreshold(string memory name, uint256 value, uint256 nodeCount) internal pure {
        if (value == 0) revert InvalidNullThreshold(name);
        if (value > nodeCount) revert InvalidHighThreshold(name, value, nodeCount);
    }

    /**
     * @dev Checks whether a context ID is in range, has nodes, and is not destroyed.
     */
    function _isValidKmsContext(uint256 kmsContextId) internal view virtual returns (bool) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        // A valid context must be in the allocated range and have at least one stored node.
        // The node check also keeps migration gap IDs invalid when initializeFromMigration
        // preserves a legacy context ID above BASE + 1.
        return
            kmsContextId >= KMS_CONTEXT_COUNTER_BASE + 1 &&
            kmsContextId <= $.currentKmsContextId &&
            $.kmsNodesForContext[kmsContextId].length != 0 &&
            !$.destroyedContexts[kmsContextId];
    }

    function _requireValidContext(uint256 kmsContextId) internal view virtual {
        if (!_isValidKmsContext(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }
    }

    /**
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
