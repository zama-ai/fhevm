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
 *      lifecycle step with the KMS confirmation signatures the canonical collected (RFC 037). The
 *      replica verifies them under the canonical's EIP-712 domain. The owner mirror functions stay
 *      as the fallback path, and `recoverKmsState` is the recovery path.
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

        _storeAndActivateKmsContextAndEpoch(canonicalContextId, canonicalEpochId, canonicalNodes, canonicalThresholds);
        _initCanonicalDomain(canonicalChainId, canonicalProtocolConfig);
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
        // This check also stops a write over the active committee before the signature checks.
        if (context.contextId <= previousContextId) {
            revert KmsContextSwitchAlreadyApplied(context.contextId);
        }
        uint256 previousEpochId = $.latestActiveEpochId;
        if (epoch.epochId <= previousEpochId) {
            revert NonIncreasingEpochId(epoch.epochId, previousEpochId);
        }
        if ($.destroyedContexts[context.contextId]) {
            revert KmsContextIdDestroyed(context.contextId);
        }
        if ($.destroyedEpochs[epoch.epochId]) {
            revert KmsEpochIdDestroyed(epoch.epochId);
        }

        // Store the new committee first, because the epoch signatures verify against it.
        _replaceKmsContext(context.contextId, context.nodes, context.thresholds);
        _verifyQuorum(
            _contextCreationStructHash(
                previousContextId,
                context.contextId,
                _nodeConfigHash(context.nodes, context.thresholds),
                context.extraData
            ),
            context.signatures,
            previousContextId,
            _faultTolerantQuorum(previousContextId)
        );
        _verifyQuorum(
            _epochActivationStructHash(
                context.contextId,
                previousEpochId,
                epoch.epochId,
                epoch.epochMaterialHash,
                epoch.extraData
            ),
            epoch.signatures,
            context.contextId,
            context.nodes.length
        );

        _activateWithCounters(context.contextId, epoch.epochId);
        emit KmsContextSwitchSubmitted(context.contextId, epoch.epochId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function submitKmsKeyResharing(KmsEpochSubmission calldata epoch) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 activeEpochId = $.latestActiveEpochId;
        if (epoch.epochId <= activeEpochId) {
            revert KmsKeyResharingAlreadyApplied(epoch.epochId);
        }
        if ($.destroyedEpochs[epoch.epochId]) {
            revert KmsEpochIdDestroyed(epoch.epochId);
        }

        uint256 contextId = $.latestActiveKmsContextId;
        _verifyQuorum(
            _epochActivationStructHash(
                contextId,
                activeEpochId,
                epoch.epochId,
                epoch.epochMaterialHash,
                epoch.extraData
            ),
            epoch.signatures,
            contextId,
            $.kmsSignerAddressesForContext[contextId].length
        );

        _activateWithCounters(contextId, epoch.epochId);
        emit KmsKeyResharingSubmitted(contextId, epoch.epochId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function submitKmsContextDestruction(
        uint256 destroyedContextId,
        bytes[] calldata signatures,
        bytes calldata extraData
    ) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 activeContextId = $.latestActiveKmsContextId;
        if (destroyedContextId >= activeContextId) {
            revert KmsContextDestructionAheadOfReplica(destroyedContextId, activeContextId);
        }
        if ($.destroyedContexts[destroyedContextId]) {
            revert KmsContextAlreadyDestroyed(destroyedContextId);
        }
        _verifyDestructionQuorum(_contextDestructionStructHash(destroyedContextId, extraData), signatures);
        _markKmsContextDestroyed(destroyedContextId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function submitKmsEpochDestruction(
        uint256 destroyedEpochId,
        bytes[] calldata signatures,
        bytes calldata extraData
    ) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 activeEpochId = $.latestActiveEpochId;
        if (destroyedEpochId >= activeEpochId) {
            revert KmsEpochDestructionAheadOfReplica(destroyedEpochId, activeEpochId);
        }
        if ($.destroyedEpochs[destroyedEpochId]) {
            revert KmsEpochAlreadyDestroyed(destroyedEpochId);
        }
        _verifyDestructionQuorum(_epochDestructionStructHash(destroyedEpochId, extraData), signatures);
        _markKmsEpochDestroyed(destroyedEpochId);
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
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 activeContextId = $.latestActiveKmsContextId;
        if (contextId <= activeContextId) {
            revert NonIncreasingKmsContextId(contextId, activeContextId);
        }
        uint256 activeEpochId = $.latestActiveEpochId;
        if (epochId <= activeEpochId) {
            revert NonIncreasingEpochId(epochId, activeEpochId);
        }

        _replaceAndActivateKmsContextAndEpoch(contextId, epochId, nodes, thresholds);
    }

    /// @inheritdoc IProtocolConfigReplica
    function mirrorKmsEpoch(uint256 contextId, uint256 epochId) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if (contextId != $.latestActiveKmsContextId || !_isLiveKmsContext(contextId)) {
            revert InvalidKmsContext(contextId);
        }
        uint256 activeEpochId = $.latestActiveEpochId;
        if (epochId <= activeEpochId) {
            revert NonIncreasingEpochId(epochId, activeEpochId);
        }

        delete $.destroyedEpochs[epochId];
        _activateWithCounters(contextId, epochId);
        emit MirrorKmsEpoch(contextId, epochId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function recoverKmsState(
        uint256 contextId,
        uint256 epochId,
        KmsNode[] calldata nodes,
        KmsThresholds calldata thresholds,
        uint256[] calldata destroyedContextIds,
        uint256[] calldata destroyedEpochIds
    ) external virtual onlyACLOwner {
        // Activate first, so a listed ID that was active can be destroyed and the restored pair cannot.
        _replaceAndActivateKmsContextAndEpoch(contextId, epochId, nodes, thresholds);
        for (uint256 i = 0; i < destroyedContextIds.length; i++) {
            _destroyKmsContext(destroyedContextIds[i]);
        }
        for (uint256 i = 0; i < destroyedEpochIds.length; i++) {
            _destroyKmsEpoch(destroyedEpochIds[i]);
        }
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
     * @dev Requires a nonzero canonical chain ID and address, then sets the canonical EIP-712 domain.
     */
    function _initCanonicalDomain(uint64 canonicalChainId, address canonicalProtocolConfig) private {
        if (canonicalChainId == 0 || canonicalProtocolConfig == address(0)) {
            revert InvalidCanonicalDomain(canonicalChainId, canonicalProtocolConfig);
        }
        __EIP712_init(EIP712_NAME, "1", canonicalProtocolConfig, canonicalChainId);
    }

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
        KmsCommitteeEntry[] storage storedNodes = $.kmsNodesForContext[contextId];
        for (uint256 i = 0; i < storedNodes.length; i++) {
            address txSender = storedNodes[i].txSenderAddress;
            delete $.isKmsTxSenderForContext[contextId][txSender];
            delete $.isKmsSignerForContext[contextId][storedNodes[i].signerAddress];
            delete $.kmsNodeByTxSenderForContext[contextId][txSender];
        }
        delete $.kmsNodesForContext[contextId];
        delete $.kmsSignerAddressesForContext[contextId];

        _storeKmsCommittee(contextId, nodes, thresholds);
    }

    /**
     * @dev Clears the destroyed flags of `contextId` and `epochId`, stores the committee under
     *      `contextId`, and activates the pair. No ordering check.
     */
    function _replaceAndActivateKmsContextAndEpoch(
        uint256 contextId,
        uint256 epochId,
        KmsNode[] calldata nodes,
        KmsThresholds calldata thresholds
    ) private {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        delete $.destroyedContexts[contextId];
        delete $.destroyedEpochs[epochId];
        _replaceKmsContext(contextId, nodes, thresholds);
        _activateWithCounters(contextId, epochId);
        emit MirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds);
    }

    /**
     * @dev Marks `destroyedContextId` destroyed. Reverts if it is already destroyed or is the active context.
     */
    function _destroyKmsContext(uint256 destroyedContextId) private {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.destroyedContexts[destroyedContextId]) {
            revert KmsContextAlreadyDestroyed(destroyedContextId);
        }
        if (destroyedContextId == $.latestActiveKmsContextId) {
            revert LatestActiveKmsContextCannotBeDestroyed(destroyedContextId);
        }

        _markKmsContextDestroyed(destroyedContextId);
    }

    /**
     * @dev Marks `destroyedEpochId` destroyed. Reverts if it is already destroyed or is the active epoch.
     */
    function _destroyKmsEpoch(uint256 destroyedEpochId) private {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.destroyedEpochs[destroyedEpochId]) {
            revert KmsEpochAlreadyDestroyed(destroyedEpochId);
        }
        if (destroyedEpochId == $.latestActiveEpochId) {
            revert LatestActiveKmsEpochCannotBeDestroyed(destroyedEpochId);
        }

        _markKmsEpochDestroyed(destroyedEpochId);
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
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
