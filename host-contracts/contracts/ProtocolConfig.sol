// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IProtocolConfig} from "./interfaces/IProtocolConfig.sol";
import {KmsNode, ChainUpgradeWindow, CoprocessorContext} from "./shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE, COPROC_CONTEXT_COUNTER_BASE} from "./shared/Constants.sol";
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
    uint64 private constant REINITIALIZER_VERSION = 3;

    /// @notice Upper bound on the KMS committee size and on every per-context threshold.
    /// @dev Driven by the proof format consumed in
    ///      `KMSVerifier.verifyDecryptionEIP712KMSSignatures`, which encodes the signature count
    ///      in a single byte (`uint8(decryptionProof[0])`). A context registered above this
    ///      bound cannot ever satisfy verification, so the limit is enforced at registration time
    ///      to reject the misconfiguration loudly rather than silently bricking the context.
    uint256 private constant MAX_KMS_SIGNERS = type(uint8).max;

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
        mapping(uint256 contextId => mapping(address txSender => bool isRegistered)) isKmsTxSenderForContext;
        /// @notice Signer lookup per context.
        mapping(uint256 contextId => mapping(address signer => bool isRegistered)) isKmsSignerForContext;
        /// @notice KmsNode by tx sender per context.
        mapping(uint256 contextId => mapping(address txSender => KmsNode node)) kmsNodeByTxSenderForContext;
        /// @notice Signer addresses per context, in insertion order.
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
        /// @notice Current coprocessor context ID counter.
        uint256 currentCoprocessorContextId;
        /// @notice Stored coprocessor context records.
        mapping(uint256 contextId => CoprocessorContext) coprocessorContexts;
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
        $.currentCoprocessorContextId = COPROC_CONTEXT_COUNTER_BASE;
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
        $.currentCoprocessorContextId = COPROC_CONTEXT_COUNTER_BASE;
        _defineKmsContext(existingKmsNodes, existingThresholds);
    }

    /**
     * @notice Reinitializer for proxies previously initialized at version 2 (KMS-context only).
     *         Seeds `currentCoprocessorContextId` at the namespace base so the first
     *         `defineNewCoprocessorContext` call lands at `COPROC_CONTEXT_COUNTER_BASE + 1`.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV3() public virtual reinitializer(REINITIALIZER_VERSION) {
        _getProtocolConfigStorage().currentCoprocessorContextId = COPROC_CONTEXT_COUNTER_BASE;
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

    /// @inheritdoc IProtocolConfig
    function defineNewCoprocessorContext(
        string calldata softwareVersion,
        ChainUpgradeWindow[] calldata chainUpgradeWindows,
        uint64 gwStartBlock
    ) external virtual onlyACLOwner {
        if (bytes(softwareVersion).length == 0) {
            revert EmptySoftwareVersion();
        }
        if (chainUpgradeWindows.length == 0) {
            revert EmptyChainUpgradeWindows();
        }
        if (gwStartBlock == 0) {
            revert ZeroGwStartBlock();
        }

        for (uint256 i = 0; i < chainUpgradeWindows.length; i++) {
            ChainUpgradeWindow calldata cw = chainUpgradeWindows[i];
            if (cw.chainId == 0) {
                revert ZeroChainId();
            }
            if (cw.startBlock > cw.endBlock) {
                revert InvalidBlockWindow(cw.chainId, cw.startBlock, cw.endBlock);
            }
            for (uint256 j = 0; j < i; j++) {
                if (chainUpgradeWindows[j].chainId == cw.chainId) {
                    revert DuplicateChainId(cw.chainId);
                }
            }
        }

        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 newCoprocessorContextId = ++$.currentCoprocessorContextId;

        CoprocessorContext storage ctx = $.coprocessorContexts[newCoprocessorContextId];
        ctx.softwareVersion = softwareVersion;
        ctx.gwStartBlock = gwStartBlock;
        ctx.activatedAtBlock = uint64(block.number);
        for (uint256 i = 0; i < chainUpgradeWindows.length; i++) {
            ctx.chainUpgradeWindows.push(chainUpgradeWindows[i]);
        }

        emit NewCoprocessorContext(newCoprocessorContextId, softwareVersion, chainUpgradeWindows, gwStartBlock);
    }

    /// @inheritdoc IProtocolConfig
    function destroyCoprocessorContext(uint256 coprocessorContextId) external virtual onlyACLOwner {
        // Unlike `destroyKmsContext`, no "current cannot be destroyed" guard — the protocol
        // does not maintain a single "current coprocessor context" pointer.
        if (!_isValidCoprocessorContext(coprocessorContextId)) {
            revert InvalidCoprocessorContext(coprocessorContextId);
        }

        _getProtocolConfigStorage().coprocessorContexts[coprocessorContextId].destroyed = true;
        emit CoprocessorContextDestroyed(coprocessorContextId);
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
    function getCurrentCoprocessorContextId() external view virtual returns (uint256) {
        return _getProtocolConfigStorage().currentCoprocessorContextId;
    }

    /// @inheritdoc IProtocolConfig
    function getCoprocessorContext(
        uint256 coprocessorContextId
    ) external view virtual returns (CoprocessorContext memory) {
        if (!_isValidCoprocessorContext(coprocessorContextId)) {
            revert InvalidCoprocessorContext(coprocessorContextId);
        }
        return _getProtocolConfigStorage().coprocessorContexts[coprocessorContextId];
    }

    /// @inheritdoc IProtocolConfig
    function isValidCoprocessorContext(uint256 coprocessorContextId) external view virtual returns (bool) {
        return _isValidCoprocessorContext(coprocessorContextId);
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
        if (kmsNodes.length > MAX_KMS_SIGNERS) {
            revert KmsSignerSetExceedsProofFormatLimit(kmsNodes.length, MAX_KMS_SIGNERS);
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
        if (value > MAX_KMS_SIGNERS) revert ThresholdExceedsProofFormatLimit(name, value, MAX_KMS_SIGNERS);
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
     * @dev Checks whether a coprocessor context ID is in range, has windows, and is not destroyed.
     */
    function _isValidCoprocessorContext(uint256 coprocessorContextId) internal view virtual returns (bool) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        CoprocessorContext storage ctx = $.coprocessorContexts[coprocessorContextId];
        return
            coprocessorContextId >= COPROC_CONTEXT_COUNTER_BASE + 1 &&
            coprocessorContextId <= $.currentCoprocessorContextId &&
            ctx.chainUpgradeWindows.length != 0 &&
            !ctx.destroyed;
    }

    /**
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
