// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigBase} from "./ProtocolConfigBase.sol";
import {IProtocolConfig} from "./interfaces/IProtocolConfig.sol";
import {IProtocolConfigBase} from "./interfaces/IProtocolConfigBase.sol";
import {IKMSGeneration} from "./interfaces/IKMSGeneration.sol";
import {KmsContextAnchor, KmsThresholds, KmsNode, KmsNodeParams, PcrValues, ChainUpgradeWindow} from "./shared/Structs.sol";
import {EPOCH_COUNTER_BASE, KMS_CONTEXT_COUNTER_BASE} from "./shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title ProtocolConfig
 * @notice Manages KMS node sets, thresholds, and context lifecycle on the canonical host chain.
 * @dev Ethereum is the canonical host and the single source of truth: the context/epoch lifecycle
 *      (`defineNewKmsContextAndEpoch` / `defineNewEpochForCurrentKmsContext`, then
 *      `confirmKmsContextCreation` / `confirmEpochActivation`) runs only there, alongside
 *      `KMSGeneration`. Every other host chain runs `ProtocolConfigReplica` instead.
 *      Anyone may submit a KMS signer's EIP-712 lifecycle confirmation. The contract checks the
 *      recovered signer, not the caller. The signatures exist so that another chain can check a
 *      lifecycle step under this contract's EIP-712 domain (RFC 037). `ProtocolConfigReplica` does
 *      not check them.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ProtocolConfig is IProtocolConfig, ProtocolConfigBase, UUPSUpgradeableEmptyProxy, ACLOwnable {
    // -----------------------------------------------------------------------------------------
    // Contract information
    // -----------------------------------------------------------------------------------------

    string private constant CONTRACT_NAME = "ProtocolConfig";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 4;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Shared between `initializeFromEmptyProxy` and `reinitializeV4`.
    uint64 private constant REINITIALIZER_VERSION = 5;

    // -----------------------------------------------------------------------------------------
    // EIP-712 type hashes
    //
    // Used to recover the KMS signer from the four KMS lifecycle confirmations.
    // -----------------------------------------------------------------------------------------

    /// @dev Hash of the EIP-712 domain separator type.
    bytes32 private constant EIP712_DOMAIN_TYPE_HASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");

    /// @dev Hash of the KeyDigest type, used to hash the key digests into `epochMaterialHash`.
    bytes32 private constant EIP712_KEY_DIGEST_TYPE_HASH = keccak256("KeyDigest(uint8 keyType,bytes digest)");

    /// @dev Hash of the ContextCreationConfirmation type.
    bytes32 private constant EIP712_CONTEXT_CREATION_TYPE_HASH =
        keccak256(
            "ContextCreationConfirmation(uint256 previousContextId,uint256 newContextId,bytes32 nodeConfigHash,bytes extraData)"
        );

    /// @dev Hash of the EpochActivationConfirmation type.
    bytes32 private constant EIP712_EPOCH_ACTIVATION_TYPE_HASH =
        keccak256(
            "EpochActivationConfirmation(uint256 contextId,uint256 previousEpochId,uint256 epochId,bytes32 epochMaterialHash,bytes extraData)"
        );

    /// @dev Hash of the ContextDestructionConfirmation type.
    bytes32 private constant EIP712_CONTEXT_DESTRUCTION_TYPE_HASH =
        keccak256(
            "ContextDestructionConfirmation(uint256 destroyedContextId,uint256[] destroyedEpochIds,bytes extraData)"
        );

    /// @dev Hash of the EpochDestructionConfirmation type.
    bytes32 private constant EIP712_EPOCH_DESTRUCTION_TYPE_HASH =
        keccak256("EpochDestructionConfirmation(uint256 destroyedEpochId,bytes extraData)");

    // -----------------------------------------------------------------------------------------
    // ERC-7201 namespaced storage
    // -----------------------------------------------------------------------------------------

    /// @custom:storage-location erc7201:fhevm.storage.ProtocolConfigCanonical
    /// @dev Canonical-only state added in v0.4.0. It is outside the base namespace, so
    ///      `ProtocolConfigReplica` does not carry it. The epoch activation confirmations stay in base
    ///      storage, because `reinitializeV4` leaves only v0.3.0 confirmations of settled epochs there.
    ///      `destroyedContexts` is also canonical-only but stays in base storage, because it existed
    ///      before v0.4.0. So a context destroyed before the upgrade can still collect destruction
    ///      confirmations, but an epoch destroyed before the upgrade cannot.
    struct ProtocolConfigCanonicalStorage {
        /// @notice Hash of the stored node set and thresholds, signed in ContextCreationConfirmation.
        mapping(uint256 contextId => bytes32) nodeConfigHashForContext;
        /// @notice Context creation confirmations per signer (one digest per signer per context).
        mapping(uint256 contextId => mapping(address signer => bool confirmed)) contextCreationConfirmedBySigner;
        /// @notice Previous-committee context creation confirmations grouped by digest.
        mapping(uint256 contextId => mapping(bytes32 digest => uint256 confirmations)) contextCreationPreviousConfirmationCountForDigest;
        /// @notice New-committee context creation confirmations grouped by digest.
        mapping(uint256 contextId => mapping(bytes32 digest => uint256 confirmations)) contextCreationNewConfirmationCountForDigest;
        /// @notice Context destruction confirmations per signer.
        mapping(uint256 contextId => mapping(address signer => bool confirmed)) contextDestructionConfirmedBySigner;
        /// @notice Epoch destruction confirmations per signer.
        mapping(uint256 epochId => mapping(address signer => bool confirmed)) epochDestructionConfirmedBySigner;
        /// @notice Whether an epoch was destroyed by destroyKmsEpoch.
        mapping(uint256 epochId => bool) destroyedEpochs;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.ProtocolConfigCanonical")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PROTOCOL_CONFIG_CANONICAL_STORAGE_LOCATION =
        0x98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c00;

    function _getProtocolConfigCanonicalStorage() internal pure returns (ProtocolConfigCanonicalStorage storage $) {
        assembly {
            $.slot := PROTOCOL_CONFIG_CANONICAL_STORAGE_LOCATION
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
     * @dev When deploying a fresh ProtocolConfig on Ethereum (canonical host), this function is called.
     * @param initialKmsNodeParams The initial KMS node set, including MPC metadata.
     * @param initialThresholds The initial thresholds.
     * @param softwareVersion The KMS software version expected for the context.
     * @param pcrValues Accepted enclave PCR values for the context.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        KmsNodeParams[] calldata initialKmsNodeParams,
        KmsThresholds calldata initialThresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        _storeAndActivateKmsContextAndEpoch(
            newContextId,
            EPOCH_COUNTER_BASE + 1,
            initialKmsNodeParams,
            initialThresholds
        );

        $.contextAnchors[newContextId] = KmsContextAnchor({
            emissionBlockNumber: block.number,
            contextInfoHash: keccak256(abi.encode(initialKmsNodeParams, initialThresholds, softwareVersion, pcrValues))
        });
        emit NewKmsContext(
            newContextId,
            KMS_CONTEXT_COUNTER_BASE,
            initialKmsNodeParams,
            initialThresholds,
            softwareVersion,
            pcrValues
        );
    }

    /**
     * @notice Re-initializes the contract from V3.
     * @dev Reverts while a lifecycle operation is in flight, so no v0.3.0 epoch activation
     *      confirmation can count toward a v0.4.0 quorum.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV4() public virtual reinitializer(REINITIALIZER_VERSION) {
        _checkNoKmsLifecycleOperationInFlight();
    }

    // -----------------------------------------------------------------------------------------
    // State-changing functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc IProtocolConfig
    /// @dev Context-switch: governance opens a new signer set as Pending; its first epoch is created
    ///      once creation is confirmed (see confirmKmsContextCreation).
    /// @dev Reverts while another lifecycle operation is in flight. Settle the in-flight one first
    ///      by completing it (confirmKmsContextCreation then confirmEpochActivation) or by
    ///      destroying it (destroyKmsContext).
    function defineNewKmsContextAndEpoch(
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) external virtual onlyACLOwner {
        _checkNoKmsLifecycleOperationInFlight();
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        // Store the new signer set as Pending. Its first epoch is created once the creation quorum
        // is reached, so the (context, epoch) pair announced to connectors is correct by construction.
        uint256 previousContextId = $.latestActiveKmsContextId;
        uint256 contextId = $.currentKmsContextId + 1;
        _storeKmsContext(contextId, kmsNodeParams, thresholds);
        $.contextState[contextId] = ContextState.Pending;

        // Commit to the stored node set and thresholds. Both committees sign it in ContextCreationConfirmation.
        KmsNode[] memory nodes = $.kmsNodesForContext[contextId];
        _getProtocolConfigCanonicalStorage().nodeConfigHashForContext[contextId] = keccak256(
            abi.encode(nodes, thresholds)
        );

        // Store context anchor and emit NewKmsContext event.
        $.contextAnchors[contextId] = KmsContextAnchor({
            emissionBlockNumber: block.number,
            contextInfoHash: keccak256(abi.encode(kmsNodeParams, thresholds, softwareVersion, pcrValues))
        });
        emit NewKmsContext(contextId, previousContextId, kmsNodeParams, thresholds, softwareVersion, pcrValues);
    }

    /// @inheritdoc IProtocolConfig
    /// @dev Same-set resharing: governance opens a new Pending epoch under the active context, no signer change.
    /// @dev Reverts while another lifecycle operation is in flight. Settle the in-flight one first
    ///      by completing it (confirmEpochActivation).
    function defineNewEpochForCurrentKmsContext() external virtual onlyACLOwner {
        _checkNoKmsLifecycleOperationInFlight();
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 latestActiveKmsContextId = $.latestActiveKmsContextId;
        uint256 epochId = _createPendingEpoch(latestActiveKmsContextId);

        // NewKmsEpoch: `previousContextId` equals `kmsContextId` because same-set resharing keeps the
        // context. `materialBlockNumber` is the last block before this request, where connectors read
        // the previous key/CRS material.
        emit NewKmsEpoch(
            latestActiveKmsContextId,
            epochId,
            latestActiveKmsContextId,
            $.latestActiveEpochId,
            block.number - 1
        );
    }

    /// @inheritdoc IProtocolConfig
    /// @dev Signers of the previous and the new committee confirm a context switch, each committee
    ///      against its own threshold. The contract counts confirmations per digest, so all signatures
    ///      of a completed quorum share one `extraData`.
    function confirmKmsContextCreation(
        uint256 kmsContextId,
        bytes calldata signature,
        bytes calldata extraData
    ) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        ProtocolConfigCanonicalStorage storage canonical$ = _getProtocolConfigCanonicalStorage();
        if ($.contextState[kmsContextId] != ContextState.Pending) {
            revert KmsContextNotPending(kmsContextId);
        }

        // The signer must belong to the outgoing or incoming committee and confirm only once.
        uint256 previousContextId = $.latestActiveKmsContextId;
        bytes32 digest = _hashTypedData(
            keccak256(
                abi.encode(
                    EIP712_CONTEXT_CREATION_TYPE_HASH,
                    previousContextId,
                    kmsContextId,
                    canonical$.nodeConfigHashForContext[kmsContextId],
                    keccak256(extraData)
                )
            )
        );
        address signer = ECDSA.recover(digest, signature);
        bool isPreviousSigner = $.isKmsSignerForContext[previousContextId][signer];
        bool isNewSigner = $.isKmsSignerForContext[kmsContextId][signer];
        if (!isPreviousSigner && !isNewSigner) {
            revert KmsContextCreationUnauthorized(signer, kmsContextId);
        }
        if (canonical$.contextCreationConfirmedBySigner[kmsContextId][signer]) {
            revert KmsContextCreationAlreadyConfirmed(signer, kmsContextId);
        }
        canonical$.contextCreationConfirmedBySigner[kmsContextId][signer] = true;

        // Count the confirmation separately for the split quorum. A signer in both committees
        // counts toward both.
        if (isPreviousSigner) {
            ++canonical$.contextCreationPreviousConfirmationCountForDigest[kmsContextId][digest];
        }
        if (isNewSigner) {
            ++canonical$.contextCreationNewConfirmationCountForDigest[kmsContextId][digest];
        }

        emit KmsContextCreationConfirmation(kmsContextId, signer, signature, extraData);

        // Context creation quorum: all new nodes and (n - t) previous nodes confirmed the same digest.
        if (
            canonical$.contextCreationNewConfirmationCountForDigest[kmsContextId][digest] !=
            $.kmsNodesForContext[kmsContextId].length
        ) {
            return;
        }
        // The previous committee has `n` nodes. At most `t` of them (its MPC threshold) can be faulty,
        // which means crashed, offline, or malicious. The quorum must be:
        //   - more than `t`, so faulty nodes can never approve a switch on their own.
        //   - at most `n - t`, because if `t` nodes stay silent only `n - t` confirmations ever
        //     arrive. Anything higher lets a dead node block the switch forever.
        // `n - t` satisfies `n - t >= t + 1` under the `n >= 3t + 1` topology the KMS core
        // requires. The contract itself does not enforce the topology.
        // The quorum is floored at 1, so the degenerate `t = n` config cannot make it zero.
        uint256 previousQuorum = $.kmsNodesForContext[previousContextId].length -
            $.mpcThresholdForContext[previousContextId];
        if (
            canonical$.contextCreationPreviousConfirmationCountForDigest[kmsContextId][digest] >=
            (previousQuorum > 0 ? previousQuorum : 1)
        ) {
            $.contextState[kmsContextId] = ContextState.Created;
            // Create the confirmed context's first epoch here, pairing it with the context by construction.
            uint256 epochId = _createPendingEpoch(kmsContextId);
            // Connectors must read previous key/CRS material from the last block before this epoch request.
            emit NewKmsEpoch(kmsContextId, epochId, previousContextId, $.latestActiveEpochId, block.number - 1);
        }
    }

    /// @inheritdoc IProtocolConfig
    /// @dev Final step of both flows Context-switch and Same-set resharing:
    ///      new-context signers attest reshared keys/CRS with full quorum activates the epoch.
    function confirmEpochActivation(
        uint256 epochId,
        EpochKeyResult[] calldata keys,
        EpochCrsResult[] calldata crsList,
        bytes calldata signature,
        bytes calldata extraData
    ) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.epochState[epochId] != EpochState.Pending) {
            revert InvalidKmsEpoch(epochId);
        }
        uint256 contextId = $.contextForEpoch[epochId];
        {
            (address signer, bytes32 epochMaterialHash, bytes32 digest) = _verifyEpochActivation(
                contextId,
                epochId,
                keys,
                crsList,
                signature,
                extraData
            );
            bool allSignersAgreed = _recordEpochConfirmation(epochId, contextId, signer, digest);
            emit EpochActivationConfirmation(epochId, signer, epochMaterialHash, signature, extraData);
            if (!allSignersAgreed) {
                return;
            }
        }

        // All signers agreed on `epochId`, so promote `contextId` and `epochId` to Active.
        $.contextState[contextId] = ContextState.Active;
        $.latestActiveKmsContextId = contextId;
        _activateEpoch(epochId, contextId);

        KmsNode[] storage nodes = $.kmsNodesForContext[contextId];
        string[] memory urls = new string[](nodes.length);
        for (uint256 i = 0; i < nodes.length; i++) {
            urls[i] = nodes[i].storageUrl;
        }
        emit ActivateEpoch(contextId, epochId, keys, crsList, urls);
    }

    /// @inheritdoc IProtocolConfig
    function destroyKmsContext(uint256 kmsContextId) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        if (kmsContextId == $.latestActiveKmsContextId) {
            revert LatestActiveKmsContextCannotBeDestroyed(kmsContextId);
        }
        if (!_isLiveKmsContext(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }

        // Mark the context destroyed and clear its epoch (if any).
        $.destroyedContexts[kmsContextId] = true;
        $.contextState[kmsContextId] = ContextState.None;
        // A confirmed switch created this context's epoch as the latest-issued one: clear it whether
        // it is still Pending or already Active, so no live epoch is left pointing at a destroyed
        // context. A switch destroyed before its creation quorum has no epoch yet, so there is
        // nothing to clear and the ownership check below is false.
        uint256 latestEpochId = $.epochCounter;
        if ($.contextForEpoch[latestEpochId] == kmsContextId) {
            _clearEpoch(latestEpochId);
        }

        emit KmsContextDestroyed(kmsContextId);
    }

    /// @inheritdoc IProtocolConfig
    function destroyKmsEpoch(uint256 epochId) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        if (epochId == $.latestActiveEpochId) {
            revert LatestActiveKmsEpochCannotBeDestroyed(epochId);
        }

        // Destroyable cases, everything else reverts:
        // - Active: retire a superseded epoch so its old shares stop being served.
        // - Pending under an Active context: abort a stuck same-set rotation. Activation
        //   confirmations are one-shot per signer, so a divergent vote can never reach unanimity
        //   and the epoch would otherwise stay Pending forever — with its context still current,
        //   neither destroyKmsContext() (reverts for the latest-active context) nor completion
        //   could ever settle it. The pending epoch of an in-flight context switch (context
        //   Pending/Created) is settled by destroyKmsContext() instead, which clears its paired
        //   epoch itself.
        EpochState state = $.epochState[epochId];
        bool isEpochActive = state == EpochState.Active;
        bool isEpochPendingUnderActiveContext = state == EpochState.Pending &&
            $.contextState[$.contextForEpoch[epochId]] == ContextState.Active;
        if (!isEpochActive && !isEpochPendingUnderActiveContext) {
            revert InvalidKmsEpoch(epochId);
        }

        _clearEpoch(epochId);
        _getProtocolConfigCanonicalStorage().destroyedEpochs[epochId] = true;
        emit KmsEpochDestroyed(epochId);
    }

    /// @inheritdoc IProtocolConfig
    /// @dev Collects signatures only. The target is `n - t` of them over one digest, but no contract
    ///      checks it. This contract keeps no quorum and accepts signers of the active committee until
    ///      the next rotation.
    function confirmKmsContextDestruction(
        uint256 destroyedContextId,
        uint256[] calldata destroyedEpochIds,
        bytes calldata signature,
        bytes calldata extraData
    ) external virtual {
        if (!_getProtocolConfigStorage().destroyedContexts[destroyedContextId]) {
            revert KmsContextNotDestroyed(destroyedContextId);
        }

        address signer = _confirmDestruction(
            _getProtocolConfigCanonicalStorage().contextDestructionConfirmedBySigner[destroyedContextId],
            destroyedContextId,
            keccak256(
                abi.encode(
                    EIP712_CONTEXT_DESTRUCTION_TYPE_HASH,
                    destroyedContextId,
                    keccak256(abi.encodePacked(destroyedEpochIds)),
                    keccak256(extraData)
                )
            ),
            signature
        );

        emit KmsContextDestructionConfirmed(destroyedContextId, destroyedEpochIds, signer, signature, extraData);
    }

    /// @inheritdoc IProtocolConfig
    /// @dev Collects signatures only, like confirmKmsContextDestruction. An epoch cleared by
    ///      destroyKmsContext is confirmed through confirmKmsContextDestruction instead.
    function confirmKmsEpochDestruction(
        uint256 destroyedEpochId,
        bytes calldata signature,
        bytes calldata extraData
    ) external virtual {
        ProtocolConfigCanonicalStorage storage canonical$ = _getProtocolConfigCanonicalStorage();
        if (!canonical$.destroyedEpochs[destroyedEpochId]) {
            revert KmsEpochNotDestroyed(destroyedEpochId);
        }

        address signer = _confirmDestruction(
            canonical$.epochDestructionConfirmedBySigner[destroyedEpochId],
            destroyedEpochId,
            keccak256(abi.encode(EIP712_EPOCH_DESTRUCTION_TYPE_HASH, destroyedEpochId, keccak256(extraData))),
            signature
        );

        emit KmsEpochDestructionConfirmed(destroyedEpochId, signer, signature, extraData);
    }

    /// @inheritdoc IProtocolConfig
    function proposeCoprocessorUpgrade(
        uint256 proposalId,
        string calldata softwareVersion,
        ChainUpgradeWindow[] calldata chainUpgradeWindows,
        uint64 gwStartBlock
    ) external virtual onlyACLOwner {
        if (proposalId == 0) {
            revert InvalidProposalId();
        }
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

        emit CoprocessorUpgradeProposed(proposalId, softwareVersion, chainUpgradeWindows, gwStartBlock);
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

    /// @inheritdoc IProtocolConfig
    function getKmsContextAnchor(
        uint256 contextId
    ) external view virtual returns (uint256 emissionBlockNumber, bytes32 contextInfoHash) {
        if (!_kmsContextExists(contextId)) {
            revert InvalidKmsContext(contextId);
        }
        KmsContextAnchor memory anchor = _getProtocolConfigStorage().contextAnchors[contextId];
        return (anchor.emissionBlockNumber, anchor.contextInfoHash);
    }

    // -----------------------------------------------------------------------------------------
    // Internal
    // -----------------------------------------------------------------------------------------

    /**
     * @dev Verifies one signer's activation submission for `epochId` under `contextId` and returns
     *      its signer, `epochMaterialHash`, and EpochActivationConfirmation digest. The aggregate
     *      signature binds the key and CRS results through `epochMaterialHash` and must recover to
     *      a signer of the context's committee.
     */
    function _verifyEpochActivation(
        uint256 contextId,
        uint256 epochId,
        EpochKeyResult[] calldata keys,
        EpochCrsResult[] calldata crsList,
        bytes calldata signature,
        bytes calldata extraData
    ) internal view virtual returns (address signer, bytes32 epochMaterialHash, bytes32 digest) {
        if (keys.length == 0 || crsList.length == 0) {
            revert EmptyEpochActivationResults(epochId);
        }
        epochMaterialHash = _hashEpochMaterial(keys, crsList);
        digest = _hashTypedData(
            keccak256(
                abi.encode(
                    EIP712_EPOCH_ACTIVATION_TYPE_HASH,
                    contextId,
                    _getProtocolConfigStorage().latestActiveEpochId,
                    epochId,
                    epochMaterialHash,
                    keccak256(extraData)
                )
            )
        );
        signer = ECDSA.recover(digest, signature);
        if (!_getProtocolConfigStorage().isKmsSignerForContext[contextId][signer]) {
            revert EpochActivationUnauthorized(signer, epochId);
        }
    }

    /**
     * @dev Returns the `epochMaterialHash` of the key and CRS results.
     */
    function _hashEpochMaterial(
        EpochKeyResult[] calldata keys,
        EpochCrsResult[] calldata crsList
    ) internal pure virtual returns (bytes32) {
        bytes32[] memory keyHashes = new bytes32[](keys.length);
        for (uint256 i = 0; i < keys.length; i++) {
            IKMSGeneration.KeyDigest[] calldata keyDigests = keys[i].keyDigests;
            bytes32[] memory keyDigestHashes = new bytes32[](keyDigests.length);
            for (uint256 j = 0; j < keyDigests.length; j++) {
                keyDigestHashes[j] = keccak256(
                    abi.encode(EIP712_KEY_DIGEST_TYPE_HASH, keyDigests[j].keyType, keccak256(keyDigests[j].digest))
                );
            }
            keyHashes[i] = keccak256(
                abi.encode(keys[i].prepKeygenId, keys[i].keyId, keccak256(abi.encodePacked(keyDigestHashes)))
            );
        }

        bytes32[] memory crsHashes = new bytes32[](crsList.length);
        for (uint256 i = 0; i < crsList.length; i++) {
            crsHashes[i] = keccak256(abi.encode(crsList[i].crsId, crsList[i].maxBitLength, crsList[i].crsDigest));
        }

        return keccak256(abi.encode(keyHashes, crsHashes));
    }

    /**
     * @dev Records `signer`'s confirmation of `digest`. Returns whether every signer of `contextId`
     *      confirmed it.
     *      The contract requires unanimity by design. One divergent epochMaterialHash or extraData splits
     *      the vote, and no digest reaches quorum. Each signer confirms only once, so a divergent vote
     *      cannot change. The epoch stays Pending until governance settles it with destroyKmsEpoch()
     *      and re-triggers the rotation.
     */
    function _recordEpochConfirmation(
        uint256 epochId,
        uint256 contextId,
        address signer,
        bytes32 digest
    ) internal virtual returns (bool complete) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.epochActivationConfirmedBySigner[epochId][signer]) {
            revert EpochActivationAlreadyConfirmed(signer, epochId);
        }
        $.epochActivationConfirmedBySigner[epochId][signer] = true;
        uint256 digestCount = ++$.epochActivationConfirmationCountForDigest[epochId][digest];
        complete = digestCount == $.kmsSignerAddressesForContext[contextId].length;
    }

    /**
     * @dev Recovers the signer of a destruction confirmation, requires it in the active committee,
     *      and records its one confirmation in `confirmedBySigner`.
     */
    function _confirmDestruction(
        mapping(address signer => bool confirmed) storage confirmedBySigner,
        uint256 destroyedId,
        bytes32 structHash,
        bytes calldata signature
    ) internal virtual returns (address signer) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        signer = ECDSA.recover(_hashTypedData(structHash), signature);
        if (!$.isKmsSignerForContext[$.latestActiveKmsContextId][signer]) {
            revert KmsDestructionUnauthorized(signer, destroyedId);
        }
        if (confirmedBySigner[signer]) {
            revert KmsDestructionAlreadyConfirmed(signer, destroyedId);
        }
        confirmedBySigner[signer] = true;
    }

    function _hashTypedData(bytes32 structHash) internal view virtual returns (bytes32) {
        bytes32 domainSeparator = keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPE_HASH,
                keccak256(bytes(CONTRACT_NAME)),
                keccak256(bytes("1")),
                block.chainid,
                address(this)
            )
        );
        return keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
    }

    /**
     * @dev Reverts when a context switch or epoch rotation is still settling. In flight means the
     *      latest-issued context is Pending/Created or the latest-issued epoch is Pending. Destroyed
     *      entries are None and do not count, so the destroy paths reopen the gate.
     */
    function _checkNoKmsLifecycleOperationInFlight() internal view virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        ContextState latestContextState = $.contextState[$.currentKmsContextId];
        bool contextInFlight = latestContextState == ContextState.Pending || latestContextState == ContextState.Created;
        bool epochPending = $.epochState[$.epochCounter] == EpochState.Pending;
        if (contextInFlight || epochPending) {
            revert KmsLifecycleOperationInFlight($.currentKmsContextId, $.epochCounter);
        }
    }

    /**
     * @dev Creates the next epoch in Pending state under `contextId` and returns its ID.
     *      Callers own any `NewKmsEpoch` event emission.
     */
    function _createPendingEpoch(uint256 contextId) internal virtual returns (uint256 epochId) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        epochId = ++$.epochCounter;
        $.epochState[epochId] = EpochState.Pending;
        $.contextForEpoch[epochId] = contextId;
    }

    /**
     * @dev Clears `epochId` back to None and drops its context link. Callers own the preconditions
     *      and any event emission.
     */
    function _clearEpoch(uint256 epochId) internal virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        $.epochState[epochId] = EpochState.None;
        delete $.contextForEpoch[epochId];
    }

    /**
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
