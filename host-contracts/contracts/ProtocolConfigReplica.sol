// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigBase} from "./ProtocolConfigBase.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "./interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "./interfaces/IProtocolConfigBase.sol";
import {KmsThresholds, KmsNode} from "./shared/Structs.sol";
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
 *      lifecycle step with the KMS confirmation signatures the canonical collected. The
 *      replica verifies them under the canonical's EIP-712 domain. The owner mirror functions stay
 *      as the fallback and recovery path.
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
    uint256 private constant MINOR_VERSION = 3;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Shared between `initializeFromCanonical` and `reinitializeV3`.
    uint64 private constant REINITIALIZER_VERSION = 4;

    /// @dev EIP-712 domain name and version of the canonical ProtocolConfig, which the KMS signers sign under.
    string private constant EIP712_NAME = "ProtocolConfig";
    string private constant EIP712_VERSION = "1";

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
     *      Later canonical updates use the submit functions or the owner mirror functions.
     * @param canonicalContextId The active Ethereum KMS context ID to preserve.
     * @param canonicalEpochId The active Ethereum epoch ID to preserve.
     * @param canonicalNodes The active Ethereum KMS node set, including each `connectorUrl`.
     * @param canonicalThresholds The active Ethereum thresholds.
     * @param canonicalChainId The chain ID of the canonical ProtocolConfig, in its EIP-712 domain.
     * @param canonicalProtocolConfig The canonical ProtocolConfig address, in its EIP-712 domain.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromCanonical(
        uint256 canonicalContextId,
        uint256 canonicalEpochId,
        KmsNode[] calldata canonicalNodes,
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

        _storeKmsNodesAndThresholds(canonicalContextId, canonicalNodes, canonicalThresholds);
        _activateKmsContextAndEpoch(canonicalContextId, canonicalEpochId);
        _initCanonicalDomain(canonicalChainId, canonicalProtocolConfig);
    }

    /**
     * @notice Re-initializes a replica proxy upgraded from a v0.14 `ProtocolConfig`, which initialized at version 3.
     * @param canonicalChainId The chain ID of the canonical ProtocolConfig, in its EIP-712 domain.
     * @param canonicalProtocolConfig The canonical ProtocolConfig address, in its EIP-712 domain.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV3(
        uint64 canonicalChainId,
        address canonicalProtocolConfig
    ) public virtual reinitializer(REINITIALIZER_VERSION) {
        _initCanonicalDomain(canonicalChainId, canonicalProtocolConfig);
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
        // This check also stops _replaceKmsContext from overwriting the active committee.
        if (context.contextId <= previousContextId) {
            revert KmsContextSwitchAlreadyApplied(context.contextId);
        }
        if ($.contextState[context.contextId] == ContextState.Destroyed) {
            revert DestroyedKmsContextCannotBeActivated(context.contextId);
        }

        _verifyActiveCommitteeQuorum(
            _contextCreationStructHash(
                previousContextId,
                context.contextId,
                _nodeConfigHash(context.nodes, context.thresholds),
                context.extraData
            ),
            context.signatures
        );
        _replaceKmsContext(context.contextId, context.nodes, context.thresholds);
        _applyKmsEpoch(context.contextId, epoch);
        emit KmsContextSwitchSubmitted(context.contextId, epoch.epochId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function submitKmsKeyResharing(KmsEpochSubmission calldata epoch) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if (epoch.epochId <= $.latestActiveEpochId) {
            revert KmsKeyResharingAlreadyApplied(epoch.epochId);
        }

        uint256 contextId = $.latestActiveKmsContextId;
        _applyKmsEpoch(contextId, epoch);
        emit KmsKeyResharingSubmitted(contextId, epoch.epochId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function submitKmsContextDestruction(
        uint256 destroyedContextId,
        bytes[] calldata signatures,
        bytes calldata extraData
    ) external virtual {
        _checkKmsContextDestroyable(destroyedContextId);
        if (destroyedContextId > _getProtocolConfigStorage().latestActiveKmsContextId) {
            revert KmsContextDestructionAheadOfReplica(destroyedContextId);
        }
        _verifyActiveCommitteeQuorum(_contextDestructionStructHash(destroyedContextId, extraData), signatures);
        _destroyKmsContext(destroyedContextId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function submitKmsEpochDestruction(
        uint256 destroyedEpochId,
        bytes[] calldata signatures,
        bytes calldata extraData
    ) external virtual {
        _checkKmsEpochDestroyable(destroyedEpochId);
        if (destroyedEpochId > _getProtocolConfigStorage().latestActiveEpochId) {
            revert KmsEpochDestructionAheadOfReplica(destroyedEpochId);
        }
        _verifyActiveCommitteeQuorum(_epochDestructionStructHash(destroyedEpochId, extraData), signatures);
        _destroyKmsEpoch(destroyedEpochId);
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

        _activateKmsContextAndEpoch(contextId, epochId);
        emit MirrorKmsEpoch(contextId, epochId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function mirrorKmsContextDestruction(uint256 destroyedContextId) external virtual onlyACLOwner {
        _checkKmsContextDestroyable(destroyedContextId);
        _destroyKmsContext(destroyedContextId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function mirrorKmsEpochDestruction(uint256 destroyedEpochId) external virtual onlyACLOwner {
        _checkKmsEpochDestroyable(destroyedEpochId);
        _destroyKmsEpoch(destroyedEpochId);
    }

    /// @inheritdoc IProtocolConfigBase
    /// @dev A replica allocates no context IDs, so it returns the active context ID.
    function getCurrentKmsContextIdCounter() external view virtual returns (uint256) {
        return _getProtocolConfigStorage().latestActiveKmsContextId;
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
     * @dev Requires a nonzero canonical chain ID and address, then sets the canonical EIP-712 domain.
     */
    function _initCanonicalDomain(uint64 canonicalChainId, address canonicalProtocolConfig) private {
        if (canonicalChainId == 0 || canonicalProtocolConfig == address(0)) {
            revert InvalidCanonicalDomain(canonicalChainId, canonicalProtocolConfig);
        }
        __EIP712_init(EIP712_NAME, EIP712_VERSION, canonicalProtocolConfig, canonicalChainId);
    }

    /**
     * @dev Stores `nodes` and `thresholds` under `contextId`, replacing any committee stored there.
     */
    function _replaceKmsContext(
        uint256 contextId,
        KmsNode[] calldata nodes,
        KmsThresholds calldata thresholds
    ) private {
        // Clear the stored committee first, so none of its signers stays a member.
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        KmsCommitteeEntry[] storage storedNodes = $.kmsNodesForContext[contextId];
        for (uint256 i = 0; i < storedNodes.length; i++) {
            address txSender = storedNodes[i].txSenderAddress;
            delete $.isKmsTxSenderForContext[contextId][txSender];
            delete $.isKmsSignerForContext[contextId][storedNodes[i].signerAddress];
            delete $.kmsNodeByTxSenderForContext[contextId][txSender];
        }
        delete $.kmsNodesForContext[contextId];
        delete $.kmsSignerAddressesForContext[contextId];

        _storeKmsNodesAndThresholds(contextId, nodes, thresholds);
    }

    /**
     * @dev Rejects a destroyed `epoch`, verifies its activation under `contextId` after the active
     *      epoch, then activates the pair.
     */
    function _applyKmsEpoch(uint256 contextId, KmsEpochSubmission calldata epoch) private {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.epochState[epoch.epochId] == EpochState.Destroyed) {
            revert DestroyedKmsEpochCannotBeActivated(epoch.epochId);
        }
        _verifyQuorum(
            _epochActivationStructHash(
                contextId,
                $.latestActiveEpochId,
                epoch.epochId,
                epoch.epochMaterialHash,
                epoch.extraData
            ),
            epoch.signatures,
            contextId,
            _kmsCommitteeSize(contextId)
        );
        _activateKmsContextAndEpoch(contextId, epoch.epochId);
    }

    function _destroyKmsContext(uint256 destroyedContextId) private {
        _getProtocolConfigStorage().contextState[destroyedContextId] = ContextState.Destroyed;
        emit KmsContextDestroyed(destroyedContextId);
    }

    function _destroyKmsEpoch(uint256 destroyedEpochId) private {
        _getProtocolConfigStorage().epochState[destroyedEpochId] = EpochState.Destroyed;
        emit KmsEpochDestroyed(destroyedEpochId);
    }

    /**
     * @dev Reverts if `destroyedContextId` is already destroyed or is the active context.
     */
    function _checkKmsContextDestroyable(uint256 destroyedContextId) private view {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.contextState[destroyedContextId] == ContextState.Destroyed) {
            revert KmsContextAlreadyDestroyed(destroyedContextId);
        }
        if (destroyedContextId == $.latestActiveKmsContextId) {
            revert LatestActiveKmsContextCannotBeDestroyed(destroyedContextId);
        }
    }

    /**
     * @dev Reverts if `destroyedEpochId` is already destroyed or is the active epoch.
     */
    function _checkKmsEpochDestroyable(uint256 destroyedEpochId) private view {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.epochState[destroyedEpochId] == EpochState.Destroyed) {
            revert KmsEpochAlreadyDestroyed(destroyedEpochId);
        }
        if (destroyedEpochId == $.latestActiveEpochId) {
            revert LatestActiveKmsEpochCannotBeDestroyed(destroyedEpochId);
        }
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
     * @dev Requires the `n - t` quorum of the active committee over `structHash`.
     */
    function _verifyActiveCommitteeQuorum(bytes32 structHash, bytes[] calldata signatures) private view {
        uint256 activeContextId = _getProtocolConfigStorage().latestActiveKmsContextId;
        _verifyQuorum(structHash, signatures, activeContextId, _faultTolerantQuorum(activeContextId));
    }

    /**
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
