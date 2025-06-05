// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./interfaces/IKmsContexts.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import { EIP712Upgradeable } from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./shared/Pausable.sol";
import { ContextLifecycle } from "./libraries/ContextLifecycle.sol";
import { KmsContext, KmsBlockPeriods, DecryptionThresholds } from "./shared/Structs.sol";
import { ContextStatus } from "./shared/Enums.sol";

/**
 * @title KmsContexts contract
 * @dev See {IKmsContexts}.
 * @dev Add/remove methods will be added in the future for KMS nodes, coprocessors and host chains.
 * @dev See https://github.com/zama-ai/fhevm-gateway/issues/98 for more details.
 */
contract KmsContexts is IKmsContexts, EIP712Upgradeable, Ownable2StepUpgradeable, UUPSUpgradeable, Pausable {
    /// @notice The typed data structure for the EIP712 signature to validate in key resharing responses.
    /// @dev The name of this struct is not relevant for the signature validation, only the one defined
    /// @dev EIP712_KEY_RESHARING_TYPE is, but we keep it the same for clarity.
    struct KeyResharingVerification {
        /// @notice The KMS context ID
        uint256 kmsContextId;
    }

    /// @notice The definition of the KeyResharingVerification structure typed data.
    string private constant EIP712_KEY_RESHARING_TYPE = "KeyResharingVerification(uint256 kmsContextId)";

    /// @notice The hash of the KeyResharingVerification structure typed data definition used for signature validation.
    bytes32 private constant EIP712_KEY_RESHARING_TYPE_HASH = keccak256(bytes(EIP712_KEY_RESHARING_TYPE));

    /// @dev The following constants are used for versioning the contract. They are made private
    /// @dev in order to force derived contracts to consider a different version. Note that
    /// @dev they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "KmsContexts";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:fhevm_gateway.storage.KmsContexts
    struct KmsContextsStorage {
        /// @notice The KMS context lifecycle
        ContextLifecycle.ContextLifecycleStorage kmsContextLifecycle;
        uint256 kmsContextGenerationBlockPeriod;
        mapping(uint256 kmsContextId => uint256 kmsContextPreActivationBlockPeriod) kmsContextPreActivationBlockPeriod;
        uint256 kmsContextSuspensionBlockPeriod;
        /// @notice The KMS contexts
        mapping(uint256 kmsContextId => KmsContext kmsContext) kmsContexts;
        /// @notice The number of KMS contexts
        uint256 kmsContextCount;
        /// @notice Wether a KMS node is done with key resharing
        mapping(uint256 kmsContextId => mapping(address kmsSignerAddress => bool doneKeyResharing)) kmsNodeDoneKeyResharing;
        mapping(uint256 kmsContextId => uint256 activationBlockNumber) kmsContextActivationBlockNumber;
        /// @notice Verified signatures for key resharing responses
        mapping(uint256 kmsContextId => bytes[] verifiedSignatures) verifiedKeyResharingSignatures;
        /// @notice The KMS nodes' metadata
        mapping(uint256 kmsContextId => mapping(address kmsTxSenderAddress => KmsNode kmsNode)) kmsNodes;
        /// @notice The KMS nodes' transaction sender addresses
        mapping(uint256 kmsContextId => mapping(address kmsTxSenderAddress => bool isKmsTxSender)) isKmsTxSender;
        /// @notice The KMS nodes' signer addresses
        mapping(uint256 kmsContextId => mapping(address kmsSignerAddress => bool isKmsSigner)) isKmsSigner;
        /// @notice The KMS nodes' transaction sender address list
        mapping(uint256 kmsContextId => address[] kmsTxSenderAddresses) kmsTxSenderAddresses;
        /// @notice The KMS nodes' signer address list
        mapping(uint256 kmsContextId => address[] kmsSignerAddresses) kmsSignerAddresses;
        mapping(uint256 kmsContextId => uint256 generationBlockNumber) kmsContextGenerationBlockNumber;
        mapping(uint256 kmsContextId => uint256 preActivationBlockNumber) kmsContextPreActivationBlockNumber;
        mapping(uint256 kmsContextId => uint256 suspensionBlockNumber) kmsContextSuspensionBlockNumber;
        /// @notice The public decryption threshold per KMS context
        mapping(uint256 kmsContextId => uint256 threshold) publicDecryptionThreshold;
        /// @notice The user decryption threshold per KMS context
        mapping(uint256 kmsContextId => uint256 threshold) userDecryptionThreshold;
    }

    /// @dev Storage location has been computed using the following command:
    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.KmsContexts")) - 1))
    /// @dev & ~bytes32(uint256(0xff))
    bytes32 private constant KMS_CONTEXTS_STORAGE_LOCATION =
        0x7d8159810a7ebf944e8fa93cc4fbd1cade6c71f8b0b86b37187ac7991777b100;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    modifier ensureKmsContextInitialized(uint256 kmsContextId) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        if ($.kmsContexts[kmsContextId].contextId == 0) {
            revert KmsContextNotInitialized(kmsContextId);
        }
        _;
    }

    /// @notice Initializes the contract
    /// @dev This function needs to be public in order to be called by the UUPS proxy.
    /// @param initialKmsConfiguration KMS configuration parameters
    function initialize(KmsConfiguration calldata initialKmsConfiguration) public virtual reinitializer(2) {
        __EIP712_init(CONTRACT_NAME, "1");
        __Ownable_init(owner());
        __Pausable_init();

        if (initialKmsConfiguration.kmsNodes.length == 0) {
            revert EmptyKmsNodes();
        }

        // The first KMS context is the initial KMS context and thus does not have a previous context
        KmsContext memory newKmsContext = _addKmsContext(
            0,
            initialKmsConfiguration.softwareVersion,
            initialKmsConfiguration.mpcThreshold,
            initialKmsConfiguration.kmsNodes,
            initialKmsConfiguration.decryptionThresholds
        );

        KmsContextsStorage storage $ = _getKmsContextsStorage();

        ContextLifecycle.setGenerating($.kmsContextLifecycle, newKmsContext.contextId);
        ContextLifecycle.setPreActivation($.kmsContextLifecycle, newKmsContext.contextId);
        ContextLifecycle.setActive($.kmsContextLifecycle, newKmsContext.contextId);

        _setKmsBlockPeriods(initialKmsConfiguration.blockPeriods, newKmsContext.contextId);

        emit Initialization(initialKmsConfiguration);
    }

    /// @dev See {IKmsContexts-checkIsKmsTxSenderFromContext}.
    function checkIsKmsTxSenderFromContext(
        uint256 kmsContextId,
        address txSenderAddress
    ) public view virtual ensureKmsContextInitialized(kmsContextId) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        if (!$.isKmsTxSender[kmsContextId][txSenderAddress]) {
            revert NotKmsTxSenderFromContext(kmsContextId, txSenderAddress);
        }
    }

    /// @dev See {IKmsContexts-checkIsKmsSignerFromContext}.
    function checkIsKmsSignerFromContext(
        uint256 kmsContextId,
        address signerAddress
    ) public view virtual ensureKmsContextInitialized(kmsContextId) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        if (!$.isKmsSigner[kmsContextId][signerAddress]) {
            revert NotKmsSignerFromContext(kmsContextId, signerAddress);
        }
    }

    /**
     * @dev See {IKmsContexts-getActiveKmsContextId}.
     * There should always be an active KMS context defined in the gateway, as we do not allow
     * to manually set active KMS contexts to `Compromised` or `Destroyed` states
     */
    function getActiveKmsContextId() public view virtual returns (uint256) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.kmsContextLifecycle.activeContextId;
    }

    /**
     * @dev See {IKmsContexts-getSuspendedKmsContextId}.
     */
    function getSuspendedKmsContextId() public view virtual returns (uint256) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.kmsContextLifecycle.suspendedContextId;
    }

    /**
     * @dev See {IKmsContexts-getActiveKmsContext}.
     * There should always be an active KMS context defined in the gateway, as we do not allow
     * to manually set active KMS contexts to `Compromised` or `Destroyed` states
     */
    function getActiveKmsContext() public view virtual returns (KmsContext memory) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        uint256 activeContextId = getActiveKmsContextId();
        return $.kmsContexts[activeContextId];
    }

    /// @dev See {IKmsContexts-updatePublicDecryptionThreshold}.
    function updatePublicDecryptionThreshold(
        uint256 newPublicDecryptionThreshold
    ) external virtual onlyOwner whenNotPaused {
        KmsContext memory activeKmsContext = getActiveKmsContext();
        _setPublicDecryptionThreshold(
            activeKmsContext.contextId,
            newPublicDecryptionThreshold,
            activeKmsContext.kmsNodes.length
        );
        emit UpdatePublicDecryptionThreshold(newPublicDecryptionThreshold);
    }

    /// @dev See {IKmsContexts-updateUserDecryptionThreshold}.
    function updateUserDecryptionThreshold(
        uint256 newUserDecryptionThreshold
    ) external virtual onlyOwner whenNotPaused {
        KmsContext memory activeKmsContext = getActiveKmsContext();
        _setUserDecryptionThreshold(
            activeKmsContext.contextId,
            newUserDecryptionThreshold,
            activeKmsContext.kmsNodes.length
        );
        emit UpdateUserDecryptionThreshold(newUserDecryptionThreshold);
    }

    /// @dev See {IKmsContexts-updateKmsContextGenerationBlockPeriod}.
    function updateKmsContextGenerationBlockPeriod(
        uint256 newKmsContextGenerationBlockPeriod
    ) external virtual onlyOwner whenNotPaused {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        $.kmsContextGenerationBlockPeriod = newKmsContextGenerationBlockPeriod;
        emit UpdateKmsContextGenerationBlockPeriod(newKmsContextGenerationBlockPeriod);
    }

    /// @dev See {IKmsContexts-updateKmsContextSuspensionBlockPeriod}.
    function updateKmsContextSuspensionBlockPeriod(
        uint256 newKmsContextSuspensionBlockPeriod
    ) external virtual onlyOwner whenNotPaused {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        $.kmsContextSuspensionBlockPeriod = newKmsContextSuspensionBlockPeriod;
        emit UpdateKmsContextSuspensionBlockPeriod(newKmsContextSuspensionBlockPeriod);
    }

    /// @dev See {IKmsContexts-addKmsContext}.
    function addKmsContext(
        uint256 preActivationBlockPeriod,
        bytes8 softwareVersion,
        bool reshareKeys,
        uint256 mpcThreshold,
        KmsNode[] calldata kmsNodes,
        DecryptionThresholds calldata decryptionThresholds
    ) external virtual onlyOwner {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        KmsContext memory activeKmsContext = getActiveKmsContext();
        uint256 activeKmsNodesLength = activeKmsContext.kmsNodes.length;
        uint256 newKmsNodesLength = kmsNodes.length;

        // Changing the number of KMS nodes is currently not allowed as the KMS does not support
        // resharing between different numbers of KMS nodes yet
        // See https://github.com/zama-ai/fhevm/issues/134
        if (newKmsNodesLength != activeKmsNodesLength) {
            revert NumberOfKmsNodesChanged(activeKmsNodesLength, newKmsNodesLength);
        }

        // Do not allow adding a new KMS context if there is a suspended KMS context ongoing
        uint256 suspendedContextId = getSuspendedKmsContextId();
        if (suspendedContextId != 0) {
            revert SuspendedKmsContextOngoing(suspendedContextId);
        }

        KmsContext memory newKmsContext = _addKmsContext(
            activeKmsContext.contextId,
            softwareVersion,
            mpcThreshold,
            kmsNodes,
            decryptionThresholds
        );

        // Get the current active KMS context

        // Emit the `NewKmsContext` event in any case
        emit NewKmsContext(activeKmsContext, newKmsContext);

        // If the `reshareKeys` flag is set or if the number of KMS nodes has changed, a key resharing is triggered
        // TODO: We should not trigger key resharing in case of parties having too many compromised parties
        // See: https://github.com/zama-ai/fhevm-gateway/issues/393
        // TODO: We should also trigger key resharing if the number of KMS nodes has changed once
        // this is supported by the KMS
        // See https://github.com/zama-ai/fhevm/issues/134
        if (reshareKeys) {
            ContextLifecycle.setGenerating($.kmsContextLifecycle, newKmsContext.contextId);

            // Store the pre-activation block period that will be taken into account once the key resharing is validated
            $.kmsContextPreActivationBlockPeriod[newKmsContext.contextId] = preActivationBlockPeriod;

            // Set the generation block number until which the key resharing needs to be validated by all KMS nodes
            uint256 generationBlockNumber = block.number + $.kmsContextGenerationBlockPeriod;
            $.kmsContextGenerationBlockNumber[newKmsContext.contextId] = generationBlockNumber;

            emit StartKeyResharing(activeKmsContext, newKmsContext, generationBlockNumber);
        } else {
            // Otherwise, set the KMS context directly to the pre-activation state
            _preActivateKmsContext(newKmsContext, preActivationBlockPeriod);
        }
    }

    function validateKeyResharing(uint256 kmsContextId, bytes calldata signature) external virtual {
        // Only accept KMS transaction senders from the associated context
        checkIsKmsTxSenderFromContext(kmsContextId, msg.sender);

        KmsContextsStorage storage $ = _getKmsContextsStorage();

        // Key resharing can only be validated if the KMS context is being generated
        if (!ContextLifecycle.isGenerating($.kmsContextLifecycle, kmsContextId)) {
            revert KmsContextNotGenerating(kmsContextId);
        }

        /// @dev Initialize the KeyResharingVerification structure for the signature validation.
        KeyResharingVerification memory keyResharingVerification = KeyResharingVerification(kmsContextId);

        /// @dev Compute the digest of the KeyResharingVerification structure.
        bytes32 digest = _hashKeyResharingVerification(keyResharingVerification);

        /// @dev Recover the signer address from the signature and validate that it corresponds to a
        /// @dev KMS node that has not already validated the key resharing.
        _validateKeyResharingEIP712Signature(kmsContextId, digest, signature);

        /// @dev Store the signature for the key resharing.
        /// @dev This list is then used to check the consensus.
        bytes[] storage verifiedSignatures = $.verifiedKeyResharingSignatures[kmsContextId];
        verifiedSignatures.push(signature);

        KmsContext memory newKmsContext = $.kmsContexts[kmsContextId];
        if (_isConsensusReachedKeyResharing(newKmsContext, verifiedSignatures.length)) {
            uint256 preActivationBlockPeriod = $.kmsContextPreActivationBlockPeriod[kmsContextId];
            _preActivateKmsContext(newKmsContext, preActivationBlockPeriod);
            emit ValidateKeyResharing(newKmsContext);
        }
    }

    function refreshKmsContextStatuses() external virtual whenNotPaused {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        uint256 generatingContextId = $.kmsContextLifecycle.generatingContextId;

        if (generatingContextId != 0) {
            if (block.number > $.kmsContextGenerationBlockNumber[generatingContextId]) {
                emit InvalidateKeyResharing(generatingContextId);

                ContextLifecycle.setDestroyed($.kmsContextLifecycle, generatingContextId);
                emit DestroyKmsContext(generatingContextId);
            }
        }

        uint256 preActivationContextId = $.kmsContextLifecycle.preActivationContextId;

        if (preActivationContextId != 0) {
            if (block.number > $.kmsContextPreActivationBlockNumber[preActivationContextId]) {
                uint256 activeContextId = getActiveKmsContextId();
                ContextLifecycle.setSuspended($.kmsContextLifecycle, activeContextId);
                emit SuspendKmsContext(activeContextId);

                ContextLifecycle.setActive($.kmsContextLifecycle, preActivationContextId);
                emit ActivateKmsContext(preActivationContextId);
            }
        }

        uint256 suspendedContextId = getSuspendedKmsContextId();

        if (suspendedContextId != 0) {
            if (block.number > $.kmsContextSuspensionBlockNumber[suspendedContextId]) {
                ContextLifecycle.setDeactivated($.kmsContextLifecycle, suspendedContextId);
                emit DeactivateKmsContext(suspendedContextId);
            }
        }
    }

    function compromiseKmsContext(
        uint256 kmsContextId
    ) external virtual onlyOwner ensureKmsContextInitialized(kmsContextId) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        // Do not allow compromising an active KMS context in order to ensure that the gateway can
        // always provide an active KMS context
        // If too many parties are compromised for this KMS context, then the relevant functions
        // should be paused instead
        if (ContextLifecycle.isActive($.kmsContextLifecycle, kmsContextId)) {
            revert CompromiseActiveKmsContextNotAllowed(kmsContextId);
        }

        ContextLifecycle.setCompromised($.kmsContextLifecycle, kmsContextId);
        emit CompromiseKmsContext(kmsContextId);
    }

    function destroyKmsContext(
        uint256 kmsContextId
    ) external virtual onlyOwner ensureKmsContextInitialized(kmsContextId) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        // Do not allow destroying an active KMS context in order to ensure that the gateway can
        // always provide an active KMS context
        if (ContextLifecycle.isActive($.kmsContextLifecycle, kmsContextId)) {
            revert DestroyActiveKmsContextNotAllowed(kmsContextId);
        }

        ContextLifecycle.setDestroyed($.kmsContextLifecycle, kmsContextId);
        emit DestroyKmsContext(kmsContextId);
    }

    function moveSuspendedKmsContextToActive() external virtual onlyOwner {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        uint256 suspendedContextId = getSuspendedKmsContextId();
        if (suspendedContextId == 0) {
            revert NoSuspendedKmsContext();
        }

        uint256 activeContextId = getActiveKmsContextId();
        ContextLifecycle.setDeactivated($.kmsContextLifecycle, activeContextId);
        emit DeactivateKmsContext(activeContextId);

        ContextLifecycle.setActive($.kmsContextLifecycle, suspendedContextId);
        emit ActivateKmsContext(suspendedContextId);
    }

    /// @dev See {IKmsContexts-getPublicDecryptionThresholdFromContext}.
    function getPublicDecryptionThresholdFromContext(
        uint256 kmsContextId
    ) external view virtual ensureKmsContextInitialized(kmsContextId) returns (uint256) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.publicDecryptionThreshold[kmsContextId];
    }

    /// @dev See {IKmsContexts-getUserDecryptionThresholdFromContext}.
    function getUserDecryptionThresholdFromContext(
        uint256 kmsContextId
    ) external view virtual ensureKmsContextInitialized(kmsContextId) returns (uint256) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.userDecryptionThreshold[kmsContextId];
    }

    /// @dev See {IKmsContexts-getKmsContextGenerationBlockPeriod}.
    function getKmsContextGenerationBlockPeriod() external view virtual returns (uint256) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.kmsContextGenerationBlockPeriod;
    }

    /// @dev See {IKmsContexts-getKmsContextSuspensionBlockPeriod}.
    function getKmsContextSuspensionBlockPeriod() external view virtual returns (uint256) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.kmsContextSuspensionBlockPeriod;
    }

    /// @dev See {IKmsContexts-getKmsNode}.
    function getKmsNode(address kmsTxSenderAddress) external view virtual returns (KmsNode memory) {
        uint256 activeKmsContextId = getActiveKmsContextId();
        return _getKmsNodeFromContext(activeKmsContextId, kmsTxSenderAddress);
    }

    /// @dev See {IKmsContexts-getKmsNodes}.
    function getKmsNodes() external view virtual returns (KmsNode[] memory) {
        return getActiveKmsContext().kmsNodes;
    }

    /// @dev See {IKmsContexts-getKmsTxSenders}.
    function getKmsTxSenders() external view virtual returns (address[] memory) {
        uint256 activeKmsContextId = getActiveKmsContextId();
        return _getKmsTxSendersFromContext(activeKmsContextId);
    }

    /// @dev See {IKmsContexts-getKmsSigners}.
    function getKmsSigners() external view virtual returns (address[] memory) {
        uint256 activeKmsContextId = getActiveKmsContextId();
        return _getKmsSignersFromContext(activeKmsContextId);
    }

    function getKmsContextStatus(uint256 kmsContextId) external view virtual returns (ContextStatus) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return ContextLifecycle.getContextStatus($.kmsContextLifecycle, kmsContextId);
    }

    /// @dev See {IKmsContexts-getVersion}.
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

    function _setKmsBlockPeriods(KmsBlockPeriods calldata kmsBlockPeriods, uint256 kmsContextId) internal virtual {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        $.kmsContextPreActivationBlockPeriod[kmsContextId] = kmsBlockPeriods.preActivationBlockPeriod;
        $.kmsContextGenerationBlockPeriod = kmsBlockPeriods.generationBlockPeriod;
        $.kmsContextSuspensionBlockPeriod = kmsBlockPeriods.suspensionBlockPeriod;
    }

    /**
     * @dev Sets the MPC threshold for a KMS context.
     * @param kmsContextId The KMS context ID
     * @param newMpcThreshold The new MPC threshold.
     * @param nKmsNodes The number of KMS nodes associated to this context
     */
    function _setMpcThreshold(uint256 kmsContextId, uint256 newMpcThreshold, uint256 nKmsNodes) internal virtual {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        /// @dev Check that the MPC threshold `t` is valid. It must verify:
        /// @dev - `t >= 0` : this is always true as it's an uint256
        /// @dev - `t < n` : it must be strictly less than the number of registered KMS nodes
        if (newMpcThreshold >= nKmsNodes) {
            revert InvalidHighMpcThreshold(kmsContextId, newMpcThreshold, nKmsNodes);
        }

        $.kmsContexts[kmsContextId].mpcThreshold = newMpcThreshold;
    }

    /**
     * @dev Sets the public decryption threshold for a KMS context.
     * @param kmsContextId The KMS context ID
     * @param publicDecryptionThreshold The public decryption threshold.
     * @param nKmsNodes The number of KMS nodes associated to this context
     */
    function _setPublicDecryptionThreshold(
        uint256 kmsContextId,
        uint256 publicDecryptionThreshold,
        uint256 nKmsNodes
    ) internal virtual {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        /// @dev Check that the public decryption threshold `t` is valid. It must verify:
        /// @dev - `t >= 1` : the public decryption consensus should require at least one vote
        /// @dev - `t <= n` : it must be less than the number of registered KMS nodes
        if (publicDecryptionThreshold == 0) {
            revert InvalidNullPublicDecryptionThreshold();
        }
        if (publicDecryptionThreshold > nKmsNodes) {
            revert InvalidHighPublicDecryptionThreshold(publicDecryptionThreshold, nKmsNodes);
        }

        $.publicDecryptionThreshold[kmsContextId] = publicDecryptionThreshold;
    }

    /**
     * @dev Sets the user decryption threshold for a KMS context.
     * @param kmsContextId The KMS context ID
     * @param userDecryptionThreshold The user decryption threshold.
     * @param nKmsNodes The number of KMS nodes associated to this context
     */
    function _setUserDecryptionThreshold(
        uint256 kmsContextId,
        uint256 userDecryptionThreshold,
        uint256 nKmsNodes
    ) internal virtual {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        /// @dev Check that the user decryption threshold `t` is valid. It must verify:
        /// @dev - `t >= 1` : the user decryption consensus should require at least one vote
        /// @dev - `t <= n` : it must be less than the number of registered KMS nodes
        if (userDecryptionThreshold == 0) {
            revert InvalidNullUserDecryptionThreshold();
        }
        if (userDecryptionThreshold > nKmsNodes) {
            revert InvalidHighUserDecryptionThreshold(userDecryptionThreshold, nKmsNodes);
        }

        $.userDecryptionThreshold[kmsContextId] = userDecryptionThreshold;
    }

    /**
     * @dev Sets the decryption thresholds for a KMS context.
     * @param kmsContextId The KMS context ID
     * @param decryptionThresholds The decryption thresholds.
     * @param nKmsNodes The number of KMS nodes associated to this context
     */
    function _setDecryptionThresholds(
        uint256 kmsContextId,
        DecryptionThresholds calldata decryptionThresholds,
        uint256 nKmsNodes
    ) internal virtual {
        _setPublicDecryptionThreshold(kmsContextId, decryptionThresholds.publicDecryptionThreshold, nKmsNodes);
        _setUserDecryptionThreshold(kmsContextId, decryptionThresholds.userDecryptionThreshold, nKmsNodes);
    }

    function _addKmsContext(
        uint256 previousKmsContextId,
        bytes8 softwareVersion,
        uint256 mpcThreshold,
        KmsNode[] calldata kmsNodes,
        DecryptionThresholds calldata decryptionThresholds
    ) internal virtual returns (KmsContext memory) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        // A KMS context ID is never null
        $.kmsContextCount++;
        uint256 kmsContextId = $.kmsContextCount;

        // Solidity doesn't support directly copying complex data structures like KmsNodes (array
        // of structs), so we need to instead create the struct field by field
        $.kmsContexts[kmsContextId].contextId = kmsContextId;
        $.kmsContexts[kmsContextId].previousContextId = previousKmsContextId;
        $.kmsContexts[kmsContextId].softwareVersion = softwareVersion;
        _setMpcThreshold(kmsContextId, mpcThreshold, kmsNodes.length);

        // Then, we need copy each KMS node struct one by one
        for (uint256 i = 0; i < kmsNodes.length; i++) {
            $.kmsContexts[kmsContextId].kmsNodes.push(kmsNodes[i]);
        }

        // Register several mappings for faster lookups
        for (uint256 i = 0; i < kmsNodes.length; i++) {
            $.kmsNodes[kmsContextId][kmsNodes[i].txSenderAddress] = kmsNodes[i];
            $.isKmsTxSender[kmsContextId][kmsNodes[i].txSenderAddress] = true;
            $.kmsTxSenderAddresses[kmsContextId].push(kmsNodes[i].txSenderAddress);
            $.isKmsSigner[kmsContextId][kmsNodes[i].signerAddress] = true;
            $.kmsSignerAddresses[kmsContextId].push(kmsNodes[i].signerAddress);
        }

        _setDecryptionThresholds(kmsContextId, decryptionThresholds, kmsNodes.length);

        return $.kmsContexts[kmsContextId];
    }

    /**
     * @notice Validates the EIP712 signature for key resharing.
     * @param kmsContextId The decryption request ID.
     * @param digest The hashed EIP712 struct.
     * @param signature The signature to validate.
     */
    function _validateKeyResharingEIP712Signature(
        uint256 kmsContextId,
        bytes32 digest,
        bytes calldata signature
    ) internal virtual {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        address signer = ECDSA.recover(digest, signature);

        /// @dev Check that the signer is a KMS signer from the KMS context.
        checkIsKmsSignerFromContext(kmsContextId, signer);

        /// @dev Check that the signer has not already validated the key resharing.
        if ($.kmsNodeDoneKeyResharing[kmsContextId][signer]) {
            revert KmsNodeAlreadyValidatedKeyResharing(kmsContextId, signer);
        }

        $.kmsNodeDoneKeyResharing[kmsContextId][signer] = true;
    }

    function _preActivateKmsContext(KmsContext memory kmsContext, uint256 preActivationBlockPeriod) internal virtual {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        uint256 preActivationBlockNumber = block.number + preActivationBlockPeriod;
        $.kmsContextPreActivationBlockNumber[kmsContext.contextId] = preActivationBlockNumber;

        emit PreActivateKmsContext(kmsContext, preActivationBlockNumber);
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /// @notice Computes the hash of a given KeyResharingVerification structured data
    /// @param keyResharingVerification The KeyResharingVerification structure
    /// @return The hash of the KeyResharingVerification structure
    function _hashKeyResharingVerification(
        KeyResharingVerification memory keyResharingVerification
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(abi.encode(EIP712_KEY_RESHARING_TYPE_HASH, keyResharingVerification.kmsContextId))
            );
    }

    /**
     * @notice Returns the KMS node from the KMS context associated to the transaction sender address.
     * @param kmsContextId The KMS context ID.
     * @param kmsTxSenderAddress The KMS transaction sender address.
     * @return The KMS node.
     */
    function _getKmsNodeFromContext(
        uint256 kmsContextId,
        address kmsTxSenderAddress
    ) internal view virtual ensureKmsContextInitialized(kmsContextId) returns (KmsNode memory) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        KmsNode memory kmsNode = $.kmsNodes[kmsContextId][kmsTxSenderAddress];
        if (kmsNode.txSenderAddress == address(0)) {
            revert NotKmsNode(kmsContextId, kmsTxSenderAddress);
        }
        return kmsNode;
    }

    /**
     * @notice Returns the KMS transaction senders from the KMS context.
     * @param kmsContextId The KMS context ID.
     * @return The KMS transaction senders.
     */
    function _getKmsTxSendersFromContext(
        uint256 kmsContextId
    ) internal view virtual ensureKmsContextInitialized(kmsContextId) returns (address[] memory) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.kmsTxSenderAddresses[kmsContextId];
    }

    /**
     * @notice Returns the KMS signers from the KMS context.
     * @param kmsContextId The KMS context ID.
     * @return The KMS signers.
     */
    function _getKmsSignersFromContext(
        uint256 kmsContextId
    ) internal view virtual ensureKmsContextInitialized(kmsContextId) returns (address[] memory) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.kmsSignerAddresses[kmsContextId];
    }

    /**
     * @notice Checks if the consensus is reached among the KMS context's nodes.
     * @dev The consensus is reached when all the KMS context's nodes have validated the key resharing.
     * @param kmsContext The KMS context
     * @param kmsCounter The number of KMS context's nodes that agreed
     * @return Whether the consensus is reached
     */
    function _isConsensusReachedKeyResharing(
        KmsContext memory kmsContext,
        uint256 kmsCounter
    ) internal view virtual returns (bool) {
        uint256 consensusThreshold = kmsContext.kmsNodes.length;
        return kmsCounter >= consensusThreshold;
    }

    /**
     * @dev Returns the KmsContexts storage location.
     * Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getKmsContextsStorage() internal pure returns (KmsContextsStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := KMS_CONTEXTS_STORAGE_LOCATION
        }
    }
}
