// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigBase} from "./ProtocolConfigBase.sol";
import {IProtocolConfig} from "./interfaces/IProtocolConfig.sol";
import {IProtocolConfigBase} from "./interfaces/IProtocolConfigBase.sol";
import {IKMSGeneration} from "./interfaces/IKMSGeneration.sol";
import {KmsContextAnchor, KmsThresholds, KmsNode, KmsNodeParams, PcrValues, ChainUpgradeWindow} from "./shared/Structs.sol";
import {EPOCH_COUNTER_BASE, EXTRA_DATA_V2, KMS_CONTEXT_COUNTER_BASE} from "./shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title ProtocolConfig
 * @notice Manages KMS node sets, thresholds, and context lifecycle on the host chains.
 * @dev Ethereum is the canonical host and the single source of truth: the context/epoch lifecycle
 *      (`defineNewKmsContextAndEpoch` / `defineNewEpochForCurrentKmsContext`, then
 *      `confirmKmsContextCreation` / `confirmEpochActivation`) runs only there, alongside
 *      `KMSGeneration`. Every other host chain (e.g. Polygon) runs `ProtocolConfigReplica` instead.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ProtocolConfig is IProtocolConfig, ProtocolConfigBase, UUPSUpgradeableEmptyProxy, ACLOwnable {
    // -----------------------------------------------------------------------------------------
    // Contract information
    // -----------------------------------------------------------------------------------------

    string private constant CONTRACT_NAME = "ProtocolConfig";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 3;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Shared between `initializeFromEmptyProxy` and `reinitializeV3`.
    uint64 private constant REINITIALIZER_VERSION = 4;

    // -----------------------------------------------------------------------------------------
    // EIP-712 type hashes
    //
    // Used to recover the KMS signer from the keygen/CRS attestations supplied to
    // `confirmEpochActivation`.
    // -----------------------------------------------------------------------------------------

    /// @dev Hash of the EIP-712 domain separator type.
    bytes32 private constant EIP712_DOMAIN_TYPE_HASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");

    /// @dev Hash of the KeyDigest type, the nested struct referenced by KeygenVerification.
    bytes32 private constant EIP712_KEY_DIGEST_TYPE_HASH = keccak256("KeyDigest(uint8 keyType,bytes digest)");

    /// @dev Hash of the KeygenVerification type. The nested KeyDigest type is appended as
    ///      EIP-712 requires nested struct types to be declared inline with the primary type.
    bytes32 private constant EIP712_KEYGEN_TYPE_HASH =
        keccak256(
            "KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)"
        );

    /// @dev Hash of the CrsgenVerification type.
    bytes32 private constant EIP712_CRSGEN_TYPE_HASH =
        keccak256("CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)");

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
     * @notice Re-initializes the contract from V2.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV3() public virtual reinitializer(REINITIALIZER_VERSION) {}

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

        // Cache the number of previous-committee confirmations confirmKmsContextCreation requires.
        // The previous committee has `n` nodes, of which at most `t` (its MPC threshold) are assumed
        // faulty — crashed, offline, or malicious. The quorum must be:
        //   - more than `t`, so faulty nodes can never approve a switch on their own;
        //   - at most `n - t`, because if `t` nodes stay silent only `n - t` confirmations ever
        //     arrive — anything higher lets a dead node block the switch forever.
        // `n - t` satisfies `n - t >= t + 1` under the `n = 3t + 1` topology the KMS core
        // requires; the contract itself does not enforce the topology.
        // Floored at 1 so the degenerate `t = n` config cannot make the quorum zero.
        uint256 previousQuorum = $.kmsNodesForContext[previousContextId].length -
            $.mpcThresholdForContext[previousContextId];
        $.contextCreationPreviousTxSenderThreshold[contextId] = previousQuorum > 0 ? previousQuorum : 1;

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
    /// @dev Context-switch: previous+new committee tx senders confirm on split-threshold quorum.
    function confirmKmsContextCreation(uint256 kmsContextId) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.contextState[kmsContextId] != ContextState.Pending) {
            revert KmsContextNotPending(kmsContextId);
        }

        // Caller must belong to the outgoing or incoming committee and confirm only once.
        uint256 previousContextId = $.latestActiveKmsContextId;
        bool isPreviousTxSender = $.isKmsTxSenderForContext[previousContextId][msg.sender];
        bool isNewTxSender = $.isKmsTxSenderForContext[kmsContextId][msg.sender];
        if (!isPreviousTxSender && !isNewTxSender) {
            revert KmsContextCreationUnauthorized(msg.sender, kmsContextId);
        }
        if ($.contextCreationConfirmedByTxSender[kmsContextId][msg.sender]) {
            revert KmsContextCreationAlreadyConfirmed(msg.sender, kmsContextId);
        }

        // Record the confirmation and counts separately for the split quorum.
        $.contextCreationConfirmedByTxSender[kmsContextId][msg.sender] = true;
        if (isPreviousTxSender) {
            ++$.contextCreationPreviousTxSenderConfirmationCount[kmsContextId];
        }
        if (isNewTxSender) {
            ++$.contextCreationNewTxSenderConfirmationCount[kmsContextId];
        }

        emit KmsContextCreationConfirmation(kmsContextId, msg.sender, isPreviousTxSender, isNewTxSender);

        // Context creation quorum: all new nodes and (n - t) previous nodes confirmed.
        if (
            $.contextCreationNewTxSenderConfirmationCount[kmsContextId] == $.kmsNodesForContext[kmsContextId].length &&
            $.contextCreationPreviousTxSenderConfirmationCount[kmsContextId] >=
            $.contextCreationPreviousTxSenderThreshold[kmsContextId]
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
        EpochCrsResult[] calldata crsList
    ) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        // Validate epoch activation: Verify EIP-712 keygen/CRS attestations and derive the consensus hash all signers must agree on.
        if ($.epochState[epochId] != EpochState.Pending) {
            revert InvalidKmsEpoch(epochId);
        }

        uint256 contextId = $.contextForEpoch[epochId];
        if (!$.isKmsTxSenderForContext[contextId][msg.sender]) {
            revert EpochActivationUnauthorized(msg.sender, epochId);
        }

        // Activation requires one key and one CRS attestation from the signer. An empty array skips its loop
        // below, so the vote would be recorded without checking that attestation.
        if (keys.length == 0 || crsList.length == 0) {
            revert EmptyEpochActivationAttestation(epochId);
        }

        address signer = $.kmsNodeByTxSenderForContext[contextId][msg.sender].signerAddress;
        bytes32 dataHash;
        {
            bytes memory extraData = abi.encodePacked(EXTRA_DATA_V2, contextId, epochId);

            bytes32[] memory keyHashes = new bytes32[](keys.length);
            for (uint256 i = 0; i < keys.length; i++) {
                bytes32 keyDigestsHash = _hashKeyDigests(keys[i].keyDigests);
                bytes32 digest = _hashKeygenVerification(
                    keys[i].prepKeygenId,
                    keys[i].keyId,
                    keyDigestsHash,
                    extraData
                );
                _requireExpectedSigner(signer, digest, keys[i].signature);
                keyHashes[i] = keccak256(abi.encode(keys[i].prepKeygenId, keys[i].keyId, keyDigestsHash));
            }

            bytes32[] memory crsHashes = new bytes32[](crsList.length);
            for (uint256 i = 0; i < crsList.length; i++) {
                bytes32 digest = _hashCrsgenVerification(
                    crsList[i].crsId,
                    crsList[i].maxBitLength,
                    crsList[i].crsDigest,
                    extraData
                );
                _requireExpectedSigner(signer, digest, crsList[i].signature);
                crsHashes[i] = keccak256(abi.encode(crsList[i].crsId, crsList[i].maxBitLength, crsList[i].crsDigest));
            }

            dataHash = keccak256(abi.encode(keyHashes, crsHashes));
        }

        // Confirm epoch activation: add this signer's vote under that hash, activate the epoch once all signers agree.
        // Record one confirmation per signer, counted by data hash so quorum requires all signers on the same result.
        // Unanimity is required by design: a single divergent dataHash splits the vote so no hash reaches quorum.
        // Confirmations are one-shot per signer, so a divergent vote can never converge — the epoch stays
        // Pending until governance settles it with destroyKmsEpoch() and re-triggers the rotation.
        if ($.epochActivationConfirmedBySigner[epochId][signer]) {
            revert EpochActivationAlreadyConfirmed(signer, epochId);
        }
        $.epochActivationConfirmedBySigner[epochId][signer] = true;
        uint256 digestCount = ++$.epochActivationConfirmationCountForDigest[epochId][dataHash];

        emit EpochActivationConfirmation(epochId, signer, dataHash);

        // All signers agreed, promote context and epoch to Active.
        if (digestCount == $.kmsSignerAddressesForContext[contextId].length) {
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

        // Mark the context destroyed, clear its epoch (if any), and wipe context-creation bookkeeping.
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
        delete $.contextCreationPreviousTxSenderThreshold[kmsContextId];
        delete $.contextCreationNewTxSenderConfirmationCount[kmsContextId];
        delete $.contextCreationPreviousTxSenderConfirmationCount[kmsContextId];

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
        emit KmsEpochDestroyed(epochId);
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

    // -----------------------------------------------------------------------------------------
    // Internal
    // -----------------------------------------------------------------------------------------

    function _requireExpectedSigner(
        address expectedSigner,
        bytes32 digest,
        bytes calldata signature
    ) internal view virtual {
        address recoveredSigner = ECDSA.recover(digest, signature);
        if (recoveredSigner != expectedSigner) {
            revert EpochActivationSignerDoesNotMatchTxSender(recoveredSigner, msg.sender);
        }
    }

    function _hashKeygenVerification(
        uint256 prepKeygenId,
        uint256 keyId,
        bytes32 keyDigestsHash,
        bytes memory extraData
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedData(
                keccak256(
                    abi.encode(EIP712_KEYGEN_TYPE_HASH, prepKeygenId, keyId, keyDigestsHash, keccak256(extraData))
                )
            );
    }

    function _hashKeyDigests(IKMSGeneration.KeyDigest[] calldata keyDigests) internal pure virtual returns (bytes32) {
        bytes32[] memory keyDigestHashes = new bytes32[](keyDigests.length);
        for (uint256 i = 0; i < keyDigests.length; i++) {
            keyDigestHashes[i] = keccak256(
                abi.encode(EIP712_KEY_DIGEST_TYPE_HASH, keyDigests[i].keyType, keccak256(keyDigests[i].digest))
            );
        }
        return keccak256(abi.encodePacked(keyDigestHashes));
    }

    function _hashCrsgenVerification(
        uint256 crsId,
        uint256 maxBitLength,
        bytes calldata crsDigest,
        bytes memory extraData
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedData(
                keccak256(
                    abi.encode(
                        EIP712_CRSGEN_TYPE_HASH,
                        crsId,
                        maxBitLength,
                        keccak256(abi.encodePacked(crsDigest)),
                        keccak256(extraData)
                    )
                )
            );
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
