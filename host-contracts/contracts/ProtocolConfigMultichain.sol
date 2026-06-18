// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IProtocolConfigCommon} from "./interfaces/IProtocolConfigCommon.sol";
import {IProtocolConfigMultichain} from "./interfaces/IProtocolConfigMultichain.sol";
import {KmsNode, KmsNodeParams, PcrValues} from "./shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE} from "./shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title ProtocolConfigMultichain
 * @notice Mirrors canonical KMS context data on non-Ethereum host chains.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ProtocolConfigMultichain is IProtocolConfigMultichain, UUPSUpgradeableEmptyProxy, ACLOwnable {
    string private constant CONTRACT_NAME = "ProtocolConfigMultichain";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 2;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Shared between `initializeFromEmptyProxy` and `reinitializeV2`.
    uint64 private constant REINITIALIZER_VERSION = 3;

    /// @notice Upper bound on the KMS committee size and on every per-context threshold.
    /// @dev Driven by the proof format consumed in
    ///      `KMSVerifier.verifyDecryptionEIP712KMSSignatures`, which encodes the signature count
    ///      in a single byte (`uint8(decryptionProof[0])`). A context mirrored above this bound
    ///      cannot ever satisfy verification, so the limit is enforced at mirror time to reject the
    ///      misconfiguration loudly rather than silently bricking the context.
    uint256 private constant MAX_KMS_SIGNERS = type(uint8).max;

    /// @custom:storage-location erc7201:fhevm.storage.ProtocolConfig
    struct ProtocolConfigMultichainStorage {
        /// @notice The latest mirrored KMS context ID, which reads resolve against.
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
        /// @dev The SDK derives the MPC threshold from the MPC nodes it knows about instead of reading this value.
        mapping(uint256 contextId => uint256) mpcThresholdForContext;
        /// @notice Whether a context has been destroyed.
        mapping(uint256 contextId => bool) destroyedContexts;
        /// @notice Canonical source provenance per mirrored context.
        mapping(uint256 contextId => MirroredContextSource) mirroredContextSources;
    }

    /// @dev Same ERC-7201 namespace as ProtocolConfig v0.1.0, preserving the legacy storage prefix.
    bytes32 private constant PROTOCOL_CONFIG_STORAGE_LOCATION =
        0x80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200;

    function _getProtocolConfigStorage() internal pure returns (ProtocolConfigMultichainStorage storage $) {
        assembly {
            $.slot := PROTOCOL_CONFIG_STORAGE_LOCATION
        }
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Fresh deploy initializer: seeds the first mirrored context from a canonical snapshot.
     * @dev Seeds the exact canonical active `contextId` rather than allocating one locally.
     * @param initialContextId The canonical active context ID to mirror.
     * @param initialKmsNodeParams The initial KMS node set, including MPC metadata.
     * @param initialThresholds The initial thresholds.
     * @param softwareVersion The KMS software version of the canonical context.
     * @param pcrValues Accepted enclave PCR values of the canonical context.
     * @param source The canonical source provenance (`sourceProtocolConfig` must be non-zero).
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        uint256 initialContextId,
        KmsNodeParams[] calldata initialKmsNodeParams,
        KmsThresholds calldata initialThresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues,
        MirroredContextSource calldata source
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        if (initialContextId < KMS_CONTEXT_COUNTER_BASE + 1) {
            revert InvalidKmsContext(initialContextId);
        }
        _storeMirroredKmsContext(
            initialContextId,
            initialKmsNodeParams,
            initialThresholds,
            softwareVersion,
            pcrValues,
            source
        );
    }

    /**
     * @notice In-place upgrade reinitializer for existing `ProtocolConfig v0.1.0` proxies (e.g. Polygon).
     * @dev Backfills canonical source provenance for the already-stored current context, leaving the
     *      legacy v0.1.0 context storage intact.
     * @param source The canonical source provenance (`sourceProtocolConfig` must be non-zero).
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2(MirroredContextSource calldata source) public virtual reinitializer(REINITIALIZER_VERSION) {
        if (source.sourceProtocolConfig == address(0)) {
            revert InvalidSourceProtocolConfig();
        }

        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        uint256 contextId = $.currentKmsContextId;
        _requireValidContext(contextId);
        $.mirroredContextSources[contextId] = source;
    }

    /// @inheritdoc IProtocolConfigMultichain
    function mirrorKmsContext(
        uint256 contextId,
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues,
        MirroredContextSource calldata source
    ) external virtual onlyACLOwner {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        uint256 currentKmsContextId = $.currentKmsContextId;
        if (contextId <= currentKmsContextId) {
            revert NonIncreasingKmsContextId(contextId, currentKmsContextId);
        }
        _storeMirroredKmsContext(contextId, kmsNodeParams, thresholds, softwareVersion, pcrValues, source);
    }

    /// @inheritdoc IProtocolConfigMultichain
    function mirrorPublicDecryptionThreshold(uint256 contextId, uint256 threshold) external virtual onlyACLOwner {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        _requireValidContext(contextId);
        _checkThreshold("publicDecryption", threshold, $.kmsNodesForContext[contextId].length);
        $.publicDecryptionThresholdForContext[contextId] = threshold;
        emit MirrorPublicDecryptionThreshold(contextId, threshold);
    }

    /// @inheritdoc IProtocolConfigMultichain
    function mirrorUserDecryptionThreshold(uint256 contextId, uint256 threshold) external virtual onlyACLOwner {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        _requireValidContext(contextId);
        _checkThreshold("userDecryption", threshold, $.kmsNodesForContext[contextId].length);
        $.userDecryptionThresholdForContext[contextId] = threshold;
        emit MirrorUserDecryptionThreshold(contextId, threshold);
    }

    /// @inheritdoc IProtocolConfigMultichain
    function mirrorKmsGenThreshold(uint256 contextId, uint256 threshold) external virtual onlyACLOwner {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        _requireValidContext(contextId);
        _checkThreshold("kmsGen", threshold, $.kmsNodesForContext[contextId].length);
        $.kmsGenThresholdForContext[contextId] = threshold;
        emit MirrorKmsGenThreshold(contextId, threshold);
    }

    /// @inheritdoc IProtocolConfigMultichain
    function mirrorMpcThreshold(uint256 contextId, uint256 threshold) external virtual onlyACLOwner {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        _requireValidContext(contextId);
        _checkThreshold("mpc", threshold, $.kmsNodesForContext[contextId].length);
        $.mpcThresholdForContext[contextId] = threshold;
        emit MirrorMpcThreshold(contextId, threshold);
    }

    /// @inheritdoc IProtocolConfigMultichain
    function mirrorKmsContextDestruction(
        uint256 contextId,
        MirroredContextSource calldata source
    ) external virtual onlyACLOwner {
        if (source.sourceProtocolConfig == address(0)) {
            revert InvalidSourceProtocolConfig();
        }
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        if (contextId == $.currentKmsContextId) {
            revert CurrentKmsContextCannotBeDestroyed(contextId);
        }
        _requireValidContext(contextId);
        $.destroyedContexts[contextId] = true;
        emit MirrorKmsContextDestroyed(
            contextId,
            source.sourceChainId,
            source.sourceBlockNumber,
            source.sourceProtocolConfig
        );
    }

    /// @inheritdoc IProtocolConfigCommon
    function getCurrentKmsContextId() external view virtual returns (uint256) {
        return _getProtocolConfigStorage().currentKmsContextId;
    }

    /// @inheritdoc IProtocolConfigCommon
    function isValidKmsContext(uint256 kmsContextId) external view virtual returns (bool) {
        return _isValidKmsContext(kmsContextId);
    }

    /// @inheritdoc IProtocolConfigCommon
    function getKmsSigners() external view virtual returns (address[] memory) {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        return $.kmsSignerAddressesForContext[$.currentKmsContextId];
    }

    /// @inheritdoc IProtocolConfigCommon
    function getKmsSignersForContext(uint256 kmsContextId) external view virtual returns (address[] memory) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsSignerAddressesForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfigCommon
    function isKmsSigner(address signer) external view virtual returns (bool) {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        return $.isKmsSignerForContext[$.currentKmsContextId][signer];
    }

    /// @inheritdoc IProtocolConfigCommon
    function isKmsSignerForContext(uint256 kmsContextId, address signer) external view virtual returns (bool) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().isKmsSignerForContext[kmsContextId][signer];
    }

    /// @inheritdoc IProtocolConfigCommon
    function getKmsNodesForContext(uint256 kmsContextId) external view virtual returns (KmsNode[] memory) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsNodesForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfigCommon
    function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view virtual returns (bool) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().isKmsTxSenderForContext[kmsContextId][txSender];
    }

    /// @inheritdoc IProtocolConfigCommon
    function getKmsNodeForContext(
        uint256 kmsContextId,
        address txSender
    ) external view virtual returns (KmsNode memory) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsNodeByTxSenderForContext[kmsContextId][txSender];
    }

    /// @inheritdoc IProtocolConfigCommon
    function getPublicDecryptionThreshold() external view virtual returns (uint256) {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        return $.publicDecryptionThresholdForContext[$.currentKmsContextId];
    }

    /// @inheritdoc IProtocolConfigCommon
    function getPublicDecryptionThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().publicDecryptionThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfigCommon
    function getUserDecryptionThreshold() external view virtual returns (uint256) {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        return $.userDecryptionThresholdForContext[$.currentKmsContextId];
    }

    /// @inheritdoc IProtocolConfigCommon
    function getUserDecryptionThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().userDecryptionThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfigCommon
    function getKmsGenThreshold() external view virtual returns (uint256) {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        return $.kmsGenThresholdForContext[$.currentKmsContextId];
    }

    /// @inheritdoc IProtocolConfigCommon
    function getKmsGenThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsGenThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfigCommon
    function getMpcThreshold() external view virtual returns (uint256) {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        return $.mpcThresholdForContext[$.currentKmsContextId];
    }

    /// @inheritdoc IProtocolConfigCommon
    function getMpcThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().mpcThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfigMultichain
    function getMirroredContextSource(
        uint256 contextId
    ) external view virtual returns (MirroredContextSource memory source) {
        _requireValidContext(contextId);
        return _getProtocolConfigStorage().mirroredContextSources[contextId];
    }

    /// @inheritdoc IProtocolConfigCommon
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

    /**
     * @dev Stores the mirrored context at `contextId`, records its canonical source provenance, and
     *      emits `MirrorKmsContext`.
     */
    function _storeMirroredKmsContext(
        uint256 contextId,
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues,
        MirroredContextSource calldata source
    ) internal virtual {
        if (source.sourceProtocolConfig == address(0)) {
            revert InvalidSourceProtocolConfig();
        }
        if (kmsNodeParams.length == 0) {
            revert EmptyKmsNodes();
        }
        if (kmsNodeParams.length > MAX_KMS_SIGNERS) {
            revert KmsSignerSetExceedsProofFormatLimit(kmsNodeParams.length, MAX_KMS_SIGNERS);
        }

        _checkThreshold("publicDecryption", thresholds.publicDecryption, kmsNodeParams.length);
        _checkThreshold("userDecryption", thresholds.userDecryption, kmsNodeParams.length);
        _checkThreshold("kmsGen", thresholds.kmsGen, kmsNodeParams.length);
        _checkThreshold("mpc", thresholds.mpc, kmsNodeParams.length);

        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        $.currentKmsContextId = contextId;

        for (uint256 i = 0; i < kmsNodeParams.length; i++) {
            KmsNodeParams calldata params = kmsNodeParams[i];
            _storeKmsNode(
                contextId,
                KmsNode({
                    txSenderAddress: params.txSenderAddress,
                    signerAddress: params.signerAddress,
                    ipAddress: params.ipAddress,
                    storageUrl: params.storageUrl
                })
            );
        }

        $.publicDecryptionThresholdForContext[contextId] = thresholds.publicDecryption;
        $.userDecryptionThresholdForContext[contextId] = thresholds.userDecryption;
        $.kmsGenThresholdForContext[contextId] = thresholds.kmsGen;
        $.mpcThresholdForContext[contextId] = thresholds.mpc;

        $.mirroredContextSources[contextId] = source;
        emit MirrorKmsContext(
            contextId,
            kmsNodeParams,
            thresholds,
            softwareVersion,
            pcrValues,
            source.sourceChainId,
            source.sourceBlockNumber,
            source.sourceProtocolConfig
        );
    }

    function _storeKmsNode(uint256 contextId, KmsNode memory node) internal virtual {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
        if (node.txSenderAddress == address(0)) {
            revert KmsNodeNullTxSender();
        }
        if (node.signerAddress == address(0)) {
            revert KmsNodeNullSigner();
        }
        if ($.isKmsTxSenderForContext[contextId][node.txSenderAddress]) {
            revert KmsTxSenderAlreadyRegistered(node.txSenderAddress);
        }
        if ($.isKmsSignerForContext[contextId][node.signerAddress]) {
            revert KmsSignerAlreadyRegistered(node.signerAddress);
        }

        $.kmsNodesForContext[contextId].push(node);
        $.isKmsTxSenderForContext[contextId][node.txSenderAddress] = true;
        $.isKmsSignerForContext[contextId][node.signerAddress] = true;
        $.kmsNodeByTxSenderForContext[contextId][node.txSenderAddress] = node;
        $.kmsSignerAddressesForContext[contextId].push(node.signerAddress);
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
     * @dev Returns true if the context was mirrored, has not been destroyed, and is at most the
     *      latest mirrored context ID.
     */
    function _isValidKmsContext(uint256 kmsContextId) internal view virtual returns (bool) {
        ProtocolConfigMultichainStorage storage $ = _getProtocolConfigStorage();
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
