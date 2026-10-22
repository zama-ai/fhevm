// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigBase} from "./ProtocolConfigBase.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "./interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "./interfaces/IProtocolConfigBase.sol";
import {KmsThresholds, KmsNode, KmsNodeParams} from "./shared/Structs.sol";
import {EPOCH_COUNTER_BASE, KMS_CONTEXT_COUNTER_BASE} from "./shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {EIP712UpgradeableCrossChain} from "./shared/EIP712UpgradeableCrossChain.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title ProtocolConfigReplica
 * @notice Read-replica of the canonical ProtocolConfig on non-canonical host chains.
 * @dev Ethereum is the canonical host and the single source of truth. Anyone may apply a canonical
 *      lifecycle step with the KMS confirmation signatures the canonical collected (RFC 037). The
 *      replica verifies them under the canonical's EIP-712 domain. The owner mirror functions stay
 *      as the recovery path.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ProtocolConfigReplica is
    IProtocolConfigReplica,
    ProtocolConfigBase,
    UUPSUpgradeableEmptyProxy,
    EIP712UpgradeableCrossChain,
    ACLOwnable
{
    // -----------------------------------------------------------------------------------------
    // Contract information
    // -----------------------------------------------------------------------------------------

    string private constant CONTRACT_NAME = "ProtocolConfigReplica";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 4;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Shared between `initializeFromCanonical` and `reinitializeV4`. Existing replica proxies
    ///      run the v0.15 `ProtocolConfig`, which is at initialized version 4.
    uint64 private constant REINITIALIZER_VERSION = 5;

    /// @dev EIP-712 domain name of the canonical ProtocolConfig, which the KMS signers sign under.
    string private constant CANONICAL_EIP712_NAME = "ProtocolConfig";

    // -----------------------------------------------------------------------------------------
    // ERC-7201 namespaced storage
    // -----------------------------------------------------------------------------------------

    /// @custom:storage-location erc7201:fhevm.storage.ProtocolConfigReplica
    /// @dev Replica-only state. Destroyed contexts reuse the base `destroyedContexts`.
    struct ProtocolConfigReplicaStorage {
        /// @notice Whether an epoch was destroyed.
        mapping(uint256 epochId => bool) destroyedEpochs;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.ProtocolConfigReplica")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PROTOCOL_CONFIG_REPLICA_STORAGE_LOCATION =
        0x5932c2e02d79e9df5f53cf8ed1f7255b6df5ec10ecc32fb34353e5743a23e200;

    function _getProtocolConfigReplicaStorage() internal pure returns (ProtocolConfigReplicaStorage storage $) {
        assembly {
            $.slot := PROTOCOL_CONFIG_REPLICA_STORAGE_LOCATION
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
     * @notice Canonical mirror initializer: seeds a non-canonical host from Ethereum's active state.
     * @dev Preserves both the canonical context ID and canonical epoch ID. This is the bootstrap path.
     *      Later canonical updates use the submit functions, or the mirror functions as recovery.
     * @param canonicalContextId The active Ethereum KMS context ID to preserve.
     * @param canonicalEpochId The active Ethereum epoch ID to preserve.
     * @param canonicalKmsNodeParams The active Ethereum KMS node set, including MPC metadata.
     * @param canonicalThresholds The active Ethereum thresholds.
     * @param canonicalChainId The chain ID of the canonical ProtocolConfig, in its EIP-712 domain.
     * @param canonicalProtocolConfig The canonical ProtocolConfig address, in its EIP-712 domain.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromCanonical(
        uint256 canonicalContextId,
        uint256 canonicalEpochId,
        KmsNodeParams[] calldata canonicalKmsNodeParams,
        KmsThresholds calldata canonicalThresholds,
        uint64 canonicalChainId,
        address canonicalProtocolConfig
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        if (canonicalContextId < KMS_CONTEXT_COUNTER_BASE + 1) {
            revert InvalidKmsContext(canonicalContextId);
        }
        if (canonicalEpochId < EPOCH_COUNTER_BASE + 1) {
            revert InvalidKmsEpoch(canonicalEpochId);
        }

        _storeAndActivateKmsContextAndEpoch(
            canonicalContextId,
            canonicalEpochId,
            canonicalKmsNodeParams,
            canonicalThresholds
        );
        __EIP712_init(CANONICAL_EIP712_NAME, "1", canonicalProtocolConfig, canonicalChainId);
    }

    /**
     * @notice Re-initializes a replica proxy upgraded from `ProtocolConfig`.
     * @dev See `REINITIALIZER_VERSION`.
     * @param canonicalChainId The chain ID of the canonical ProtocolConfig, in its EIP-712 domain.
     * @param canonicalProtocolConfig The canonical ProtocolConfig address, in its EIP-712 domain.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV4(
        uint64 canonicalChainId,
        address canonicalProtocolConfig
    ) public virtual reinitializer(REINITIALIZER_VERSION) {
        __EIP712_init(CANONICAL_EIP712_NAME, "1", canonicalProtocolConfig, canonicalChainId);
    }

    // -----------------------------------------------------------------------------------------
    // Submit functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc IProtocolConfigReplica
    function submitKmsContextSwitch(
        KmsContextSubmission calldata context,
        KmsEpochSubmission calldata epoch
    ) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 previousContextId = $.latestActiveKmsContextId;
        // An older ID counts as applied. A stored ID above the active one does not. An owner mirror
        // back to a lower ID can leave such an ID behind, and the submission replaces its committee.
        if (context.contextId <= previousContextId) {
            revert KmsContextSwitchAlreadyApplied(context.contextId);
        }
        // The owner marks a forged or canonically destroyed ID destroyed before the replica reaches it.
        if ($.destroyedContexts[context.contextId]) {
            revert KmsContextIdDestroyed(context.contextId);
        }
        if (_getProtocolConfigReplicaStorage().destroyedEpochs[epoch.epochId]) {
            revert KmsEpochIdDestroyed(epoch.epochId);
        }
        if (context.previousContextId != previousContextId) {
            revert KmsContextLineageMismatch(context.previousContextId, previousContextId);
        }
        if (epoch.previousEpochId != $.latestActiveEpochId) {
            revert KmsEpochLineageMismatch(epoch.previousEpochId, $.latestActiveEpochId);
        }
        if (epoch.contextId != context.contextId) {
            revert InvalidKmsContext(epoch.contextId);
        }

        // Store the new committee first, because the epoch signatures verify against it.
        _replaceKmsContext(context.contextId, context.nodes, context.thresholds);
        _verifyQuorum(
            _hashContextCreationStruct(context),
            context.signatures,
            previousContextId,
            _faultTolerantQuorum(previousContextId)
        );
        _verifyQuorum(_hashEpochActivationStruct(epoch), epoch.signatures, context.contextId, context.nodes.length);

        _activateKmsContextAndEpoch(context.contextId, epoch.epochId);
        emit KmsContextSwitchSubmitted(context.contextId, epoch.epochId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function submitKmsKeyResharing(KmsEpochSubmission calldata epoch) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 activeEpochId = $.latestActiveEpochId;
        if (epoch.epochId <= activeEpochId) {
            revert KmsKeyResharingAlreadyApplied(epoch.epochId);
        }
        // The owner marks a forged or canonically destroyed ID destroyed before the replica reaches it.
        if (_getProtocolConfigReplicaStorage().destroyedEpochs[epoch.epochId]) {
            revert KmsEpochIdDestroyed(epoch.epochId);
        }
        uint256 contextId = $.latestActiveKmsContextId;
        if (epoch.contextId != contextId) {
            revert KmsContextLineageMismatch(epoch.contextId, contextId);
        }
        if (epoch.previousEpochId != activeEpochId) {
            revert KmsEpochLineageMismatch(epoch.previousEpochId, activeEpochId);
        }

        _verifyQuorum(
            _hashEpochActivationStruct(epoch),
            epoch.signatures,
            contextId,
            $.kmsSignerAddressesForContext[contextId].length
        );

        $.epochCounter = epoch.epochId;
        _activateEpoch(epoch.epochId, contextId);
        emit KmsKeyResharingSubmitted(contextId, epoch.epochId);
    }

    /// @inheritdoc IProtocolConfigReplica
    /// @dev The ahead check runs first, then the destroy checks, then the signature checks. So an ID
    ///      at or above the active one reverts with `KmsContextDestructionAheadOfReplica`, whatever
    ///      committee signed it. A replay after a rotation reverts with `KmsContextAlreadyDestroyed`.
    function submitKmsContextDestruction(
        uint256 destroyedContextId,
        bytes[] calldata signatures,
        bytes calldata extraData
    ) external virtual {
        uint256 activeContextId = _getProtocolConfigStorage().latestActiveKmsContextId;
        if (destroyedContextId >= activeContextId) {
            revert KmsContextDestructionAheadOfReplica(destroyedContextId, activeContextId);
        }
        _destroyKmsContext(destroyedContextId);
        _verifyDestructionQuorum(
            keccak256(abi.encode(EIP712_CONTEXT_DESTRUCTION_TYPE_HASH, destroyedContextId, keccak256(extraData))),
            signatures
        );
    }

    /// @inheritdoc IProtocolConfigReplica
    /// @dev The checks run in the same order as in `submitKmsContextDestruction`.
    function submitKmsEpochDestruction(
        uint256 destroyedEpochId,
        bytes[] calldata signatures,
        bytes calldata extraData
    ) external virtual {
        uint256 activeEpochId = _getProtocolConfigStorage().latestActiveEpochId;
        if (destroyedEpochId >= activeEpochId) {
            revert KmsEpochDestructionAheadOfReplica(destroyedEpochId, activeEpochId);
        }
        _destroyKmsEpoch(destroyedEpochId);
        _verifyDestructionQuorum(
            keccak256(abi.encode(EIP712_EPOCH_DESTRUCTION_TYPE_HASH, destroyedEpochId, keccak256(extraData))),
            signatures
        );
    }

    // -----------------------------------------------------------------------------------------
    // Mirror functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc IProtocolConfigReplica
    function mirrorKmsContextAndEpoch(
        uint256 contextId,
        uint256 epochId,
        KmsNode[] calldata nodes,
        KmsThresholds calldata thresholds
    ) external virtual onlyACLOwner {
        delete _getProtocolConfigStorage().destroyedContexts[contextId];
        delete _getProtocolConfigReplicaStorage().destroyedEpochs[epochId];
        _replaceKmsContext(contextId, nodes, thresholds);
        _activateKmsContextAndEpoch(contextId, epochId);
        emit MirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds);
    }

    /// @inheritdoc IProtocolConfigReplica
    function mirrorKmsEpoch(uint256 contextId, uint256 epochId) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if (contextId != $.latestActiveKmsContextId || !_isLiveKmsContext(contextId)) {
            revert InvalidKmsContext(contextId);
        }

        delete _getProtocolConfigReplicaStorage().destroyedEpochs[epochId];
        $.epochCounter = epochId;
        _activateEpoch(epochId, contextId);
        emit MirrorKmsEpoch(contextId, epochId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function mirrorKmsContextDestruction(uint256 destroyedContextId) external virtual onlyACLOwner {
        _destroyKmsContext(destroyedContextId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function mirrorKmsEpochDestruction(uint256 destroyedEpochId) external virtual onlyACLOwner {
        _destroyKmsEpoch(destroyedEpochId);
    }

    /// @inheritdoc IProtocolConfigBase
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
     * @dev Stores `nodes` and `thresholds` under `contextId`. Clears the committee already stored
     *      under that ID first, so none of its signers stays a member.
     */
    function _replaceKmsContext(
        uint256 contextId,
        KmsNode[] calldata nodes,
        KmsThresholds calldata thresholds
    ) private {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        KmsNode[] storage storedNodes = $.kmsNodesForContext[contextId];
        for (uint256 i = 0; i < storedNodes.length; i++) {
            address txSender = storedNodes[i].txSenderAddress;
            delete $.isKmsTxSenderForContext[contextId][txSender];
            delete $.isKmsSignerForContext[contextId][storedNodes[i].signerAddress];
            delete $.kmsNodeByTxSenderForContext[contextId][txSender];
        }
        delete $.kmsNodesForContext[contextId];
        delete $.kmsSignerAddressesForContext[contextId];

        _checkKmsThresholds(nodes.length, thresholds);
        $.publicDecryptionThresholdForContext[contextId] = thresholds.publicDecryption;
        $.userDecryptionThresholdForContext[contextId] = thresholds.userDecryption;
        $.kmsGenThresholdForContext[contextId] = thresholds.kmsGen;
        $.mpcThresholdForContext[contextId] = thresholds.mpc;
        for (uint256 i = 0; i < nodes.length; i++) {
            _storeKmsNode(contextId, nodes[i]);
        }

        // Every store on a replica activates, so the counter follows the active context.
        $.currentKmsContextId = contextId;
    }

    /**
     * @dev Sets `contextId` Active and activates `epochId` under it, as the latest active pair.
     */
    function _activateKmsContextAndEpoch(uint256 contextId, uint256 epochId) private {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        $.contextState[contextId] = ContextState.Active;
        $.latestActiveKmsContextId = contextId;
        $.epochCounter = epochId;
        _activateEpoch(epochId, contextId);
    }

    /**
     * @dev Marks `destroyedContextId` destroyed. The epochs of a destroyed context stop being valid
     *      for it, because `isValidEpochForContext` also requires a valid context.
     */
    function _destroyKmsContext(uint256 destroyedContextId) private {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.destroyedContexts[destroyedContextId]) {
            revert KmsContextAlreadyDestroyed(destroyedContextId);
        }
        if (destroyedContextId == $.latestActiveKmsContextId) {
            revert LatestActiveKmsContextCannotBeDestroyed(destroyedContextId);
        }

        $.destroyedContexts[destroyedContextId] = true;
        $.contextState[destroyedContextId] = ContextState.None;
        emit KmsContextDestroyed(destroyedContextId);
    }

    /**
     * @dev Clears `destroyedEpochId` and marks it destroyed.
     */
    function _destroyKmsEpoch(uint256 destroyedEpochId) private {
        ProtocolConfigReplicaStorage storage replica$ = _getProtocolConfigReplicaStorage();
        if (replica$.destroyedEpochs[destroyedEpochId]) {
            revert KmsEpochAlreadyDestroyed(destroyedEpochId);
        }
        if (destroyedEpochId == _getProtocolConfigStorage().latestActiveEpochId) {
            revert LatestActiveKmsEpochCannotBeDestroyed(destroyedEpochId);
        }

        _clearEpoch(destroyedEpochId);
        replica$.destroyedEpochs[destroyedEpochId] = true;
        emit KmsEpochDestroyed(destroyedEpochId);
    }

    /**
     * @dev Requires at least `quorum` signatures over `structHash` under the canonical domain. Every
     *      signature must recover to a distinct signer of the committee of `contextId`.
     */
    function _verifyQuorum(
        bytes32 structHash,
        bytes[] calldata signatures,
        uint256 contextId,
        uint256 quorum
    ) private view {
        if (signatures.length < quorum) {
            revert KmsSignatureQuorumNotReached(signatures.length, quorum);
        }
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        bytes32 digest = _hashTypedDataV4(structHash);
        address[] memory signers = new address[](signatures.length);
        for (uint256 i = 0; i < signatures.length; i++) {
            address signer = ECDSA.recover(digest, signatures[i]);
            if (!$.isKmsSignerForContext[contextId][signer]) {
                revert KmsSignerNotInCommittee(signer, contextId);
            }
            // A committee has at most 255 signers, so the quadratic scan stays small.
            for (uint256 j = 0; j < i; j++) {
                if (signers[j] == signer) {
                    revert DuplicateKmsSigner(signer);
                }
            }
            signers[i] = signer;
        }
    }

    /**
     * @dev Requires the `n - t` quorum of the active committee over the destruction `structHash`.
     */
    function _verifyDestructionQuorum(bytes32 structHash, bytes[] calldata signatures) private view {
        uint256 activeContextId = _getProtocolConfigStorage().latestActiveKmsContextId;
        _verifyQuorum(structHash, signatures, activeContextId, _faultTolerantQuorum(activeContextId));
    }

    /**
     * @dev Returns the `n - t` quorum of the committee of `contextId`, floored at 1. Same rule as the
     *      previous-committee quorum of `ProtocolConfig.confirmKmsContextCreation`.
     */
    function _faultTolerantQuorum(uint256 contextId) private view returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 quorum = $.kmsNodesForContext[contextId].length - $.mpcThresholdForContext[contextId];
        return quorum > 0 ? quorum : 1;
    }

    /**
     * @dev Struct hash of the ContextCreationConfirmation that `context` carries. The replica
     *      recomputes `nodeConfigHash` from the submitted nodes and thresholds.
     */
    function _hashContextCreationStruct(KmsContextSubmission calldata context) private pure returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EIP712_CONTEXT_CREATION_TYPE_HASH,
                    context.previousContextId,
                    context.contextId,
                    keccak256(abi.encode(context.nodes, context.thresholds)),
                    keccak256(context.extraData)
                )
            );
    }

    /**
     * @dev Struct hash of the EpochActivationConfirmation that `epoch` carries.
     */
    function _hashEpochActivationStruct(KmsEpochSubmission calldata epoch) private pure returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EIP712_EPOCH_ACTIVATION_TYPE_HASH,
                    epoch.contextId,
                    epoch.previousEpochId,
                    epoch.epochId,
                    epoch.epochMaterialHash,
                    keccak256(epoch.extraData)
                )
            );
    }

    /**
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
