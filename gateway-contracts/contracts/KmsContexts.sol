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
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { KmsContext, KmsBlockPeriods, DecryptionThresholds } from "./shared/Structs.sol";
import { ContextStatus } from "./shared/Enums.sol";

/**
 * @title KmsContexts contract
 * @dev See {IKmsContexts}.
 * @dev Add/remove methods will be added in the future for KMS nodes, coprocessors and host chains.
 * @dev See https://github.com/zama-ai/fhevm-gateway/issues/98 for more details.
 */
contract KmsContexts is IKmsContexts, EIP712Upgradeable, Ownable2StepUpgradeable, UUPSUpgradeableEmptyProxy, Pausable {
    /// @notice The typed data structure for the EIP712 signature to validate in key resharing responses.
    /// @dev The name of this struct is not relevant for the signature validation, only the one defined
    /// @dev EIP712_KEY_RESHARING_TYPE is, but we keep it the same for clarity.
    struct KeyResharingVerification {
        /// @notice The KMS context ID
        uint256 contextId;
    }

    /// @notice The definition of the KeyResharingVerification structure typed data.
    string private constant EIP712_KEY_RESHARING_TYPE = "KeyResharingVerification(uint256 contextId)";

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
        uint256 kmsContextSuspensionBlockPeriod;
        /// @notice The KMS contexts
        mapping(uint256 contextId => KmsContext kmsContext) kmsContexts;
        /// @notice The number of KMS contexts
        uint256 kmsContextCount;
        /// @notice Wether a KMS node is done with key resharing
        mapping(uint256 contextId => mapping(address kmsSignerAddress => bool doneKeyResharing)) kmsNodeDoneKeyResharing;
        /// @notice Verified signatures for key resharing responses
        mapping(uint256 contextId => bytes[] verifiedSignatures) verifiedKeyResharingSignatures;
        /// @notice The KMS nodes' metadata
        mapping(uint256 contextId => mapping(address kmsTxSenderAddress => KmsNode kmsNode)) kmsNodes;
        /// @notice The KMS nodes' transaction sender addresses
        mapping(uint256 contextId => mapping(address kmsTxSenderAddress => bool isKmsTxSender)) isKmsTxSender;
        /// @notice The KMS nodes' signer addresses
        mapping(uint256 contextId => mapping(address kmsSignerAddress => bool isKmsSigner)) isKmsSigner;
        /// @notice The KMS nodes' transaction sender address list
        mapping(uint256 contextId => address[] kmsTxSenderAddresses) kmsTxSenderAddresses;
        /// @notice The KMS nodes' signer address list
        mapping(uint256 contextId => address[] kmsSignerAddresses) kmsSignerAddresses;
        mapping(uint256 contextId => uint256 preActivationBlockNumber) kmsContextPreActivationBlockNumber;
        mapping(uint256 contextId => uint256 activationBlockNumber) kmsContextActivationBlockNumber;
        mapping(uint256 contextId => uint256 deactivatedBlockNumber) kmsContextDeactivatedBlockNumber;
        mapping(uint256 contextId => uint256 kmsContextPreActivationBlockPeriod) kmsContextPreActivationBlockPeriod;
        mapping(uint256 contextId => uint256 suspendedBlockPeriod) kmsContextSuspendedBlockPeriod;
        /// @notice The public decryption threshold per KMS context
        mapping(uint256 contextId => uint256 threshold) publicDecryptionThreshold;
        /// @notice The user decryption threshold per KMS context
        mapping(uint256 contextId => uint256 threshold) userDecryptionThreshold;
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

    modifier ensureContextInitialized(uint256 contextId) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        if ($.kmsContexts[contextId].contextId == 0) {
            revert KmsContextNotInitialized(contextId);
        }
        _;
    }

    /// @notice Initializes the contract
    /// @dev This function needs to be public in order to be called by the UUPS proxy.
    /// @param initialDecryptionThresholds The decryption thresholds for the KMS context
    /// @param initialSoftwareVersion The software version of the KMS context
    /// @param initialMpcThreshold The MPC threshold for the KMS context
    /// @param initialKmsNodes The KMS nodes for the KMS context
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        bytes8 initialSoftwareVersion,
        uint256 initialMpcThreshold,
        KmsNode[] calldata initialKmsNodes,
        DecryptionThresholds calldata initialDecryptionThresholds
    ) public virtual onlyFromEmptyProxy reinitializer(2) {
        __EIP712_init(CONTRACT_NAME, "1");
        __Ownable_init(owner());
        __Pausable_init();

        // The first KMS context is the initial KMS context and thus does not have a previous context
        KmsContext memory newKmsContext = _addKmsContext(
            0,
            initialSoftwareVersion,
            initialMpcThreshold,
            initialKmsNodes,
            initialDecryptionThresholds
        );

        KmsContextsStorage storage $ = _getKmsContextsStorage();

        // It is exceptionally allowed to set the active context directly at initialization. In other
        // cases, the context must be pre-activated first.
        ContextLifecycle.setActive($.kmsContextLifecycle, newKmsContext.contextId);

        emit InitializeKmsContexts(
            initialDecryptionThresholds,
            initialSoftwareVersion,
            initialMpcThreshold,
            initialKmsNodes
        );
    }

    /// @dev See {IKmsContexts-checkIsKmsTxSenderFromContext}.
    function checkIsKmsTxSenderFromContext(
        uint256 contextId,
        address txSenderAddress
    ) public view virtual ensureContextInitialized(contextId) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        if (!$.isKmsTxSender[contextId][txSenderAddress]) {
            revert NotKmsTxSenderFromContext(contextId, txSenderAddress);
        }
    }

    /// @dev See {IKmsContexts-checkIsKmsSignerFromContext}.
    function checkIsKmsSignerFromContext(
        uint256 contextId,
        address signerAddress
    ) public view virtual ensureContextInitialized(contextId) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        if (!$.isKmsSigner[contextId][signerAddress]) {
            revert NotKmsSignerFromContext(contextId, signerAddress);
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

    /// @dev See {IKmsContexts-addKmsContext}.
    function addKmsContext(
        bytes8 softwareVersion,
        bool reshareKeys,
        uint256 mpcThreshold,
        KmsNode[] calldata kmsNodes,
        KmsBlockPeriods calldata blockPeriods,
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

        // Store the suspended block period for the previous KMS context
        // This value will be considered once the new KMS context is activated and the old one
        // is suspended
        $.kmsContextSuspendedBlockPeriod[activeKmsContext.contextId] = blockPeriods.suspendedBlockPeriod;

        // Emit the `NewKmsContext` event in any case
        emit NewKmsContext(activeKmsContext, newKmsContext, blockPeriods);

        ContextLifecycle.setGenerating($.kmsContextLifecycle, newKmsContext.contextId);

        // If the `reshareKeys` flag is set or if the number of KMS nodes has changed, a key resharing is triggered
        // TODO: We should not trigger key resharing in case of parties having too many compromised parties
        // See: https://github.com/zama-ai/fhevm-gateway/issues/393
        // TODO: We should also trigger key resharing if the number of KMS nodes has changed once
        // this is supported by the KMS
        // See https://github.com/zama-ai/fhevm/issues/134
        if (reshareKeys) {
            // Store the pre-activation block period that will be taken into account once the key resharing is validated
            $.kmsContextPreActivationBlockPeriod[newKmsContext.contextId] = blockPeriods.preActivationBlockPeriod;

            // Set the generation block number until which the key resharing needs to be validated by all KMS nodes
            uint256 preActivationBlockNumber = block.number + blockPeriods.generationBlockPeriod;
            $.kmsContextPreActivationBlockNumber[newKmsContext.contextId] = preActivationBlockNumber;

            emit StartKeyResharing(activeKmsContext, newKmsContext, preActivationBlockNumber);
        } else {
            // Otherwise, set the KMS context directly to the pre-activation state
            _preActivateKmsContext(newKmsContext, blockPeriods.preActivationBlockPeriod);
        }
    }

    function validateKeyResharing(uint256 contextId, bytes calldata signature) external virtual {
        // Only accept KMS transaction senders from the associated context
        checkIsKmsTxSenderFromContext(contextId, msg.sender);

        KmsContextsStorage storage $ = _getKmsContextsStorage();

        // Key resharing can only be validated if the KMS context is being generated
        if (!ContextLifecycle.isGenerating($.kmsContextLifecycle, contextId)) {
            revert KmsContextNotGenerating(contextId);
        }

        /// @dev Initialize the KeyResharingVerification structure for the signature validation.
        KeyResharingVerification memory keyResharingVerification = KeyResharingVerification(contextId);

        /// @dev Compute the digest of the KeyResharingVerification structure.
        bytes32 digest = _hashKeyResharingVerification(keyResharingVerification);

        /// @dev Recover the signer address from the signature and validate that it corresponds to a
        /// @dev KMS node that has not already validated the key resharing.
        _validateKeyResharingEIP712Signature(contextId, digest, signature);

        /// @dev Store the signature for the key resharing.
        /// @dev This list is then used to check the consensus.
        bytes[] storage verifiedSignatures = $.verifiedKeyResharingSignatures[contextId];
        verifiedSignatures.push(signature);

        KmsContext memory newKmsContext = $.kmsContexts[contextId];
        if (_isConsensusReachedKeyResharing(newKmsContext, verifiedSignatures.length)) {
            uint256 preActivationBlockPeriod = $.kmsContextPreActivationBlockPeriod[contextId];
            _preActivateKmsContext(newKmsContext, preActivationBlockPeriod);
            emit ValidateKeyResharing(newKmsContext);
        }
    }

    function refreshKmsContextStatuses() external virtual whenNotPaused {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        uint256 generatingContextId = $.kmsContextLifecycle.generatingContextId;

        if (generatingContextId != 0 && block.number >= $.kmsContextPreActivationBlockNumber[generatingContextId]) {
            emit InvalidateKeyResharing(generatingContextId);

            ContextLifecycle.setDestroyed($.kmsContextLifecycle, generatingContextId);
            emit DestroyKmsContext(generatingContextId);
        }

        uint256 preActivationContextId = $.kmsContextLifecycle.preActivationContextId;

        if (preActivationContextId != 0 && block.number >= $.kmsContextActivationBlockNumber[preActivationContextId]) {
            uint256 activeContextId = getActiveKmsContextId();

            // Define the deactivation block number for the current active KMS context
            uint256 deactivatedBlockNumber = block.number + $.kmsContextSuspendedBlockPeriod[activeContextId];
            $.kmsContextDeactivatedBlockNumber[activeContextId] = deactivatedBlockNumber;

            ContextLifecycle.setSuspended($.kmsContextLifecycle, activeContextId);
            emit SuspendKmsContext(activeContextId);

            ContextLifecycle.setActive($.kmsContextLifecycle, preActivationContextId);
            emit ActivateKmsContext(preActivationContextId);
        }

        uint256 suspendedContextId = getSuspendedKmsContextId();

        if (suspendedContextId != 0) {
            if (block.number > $.kmsContextDeactivatedBlockNumber[suspendedContextId]) {
                ContextLifecycle.setDeactivated($.kmsContextLifecycle, suspendedContextId);
                emit DeactivateKmsContext(suspendedContextId);
            }
        }
    }

    function compromiseKmsContext(uint256 contextId) external virtual onlyOwner ensureContextInitialized(contextId) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        // Do not allow compromising an active KMS context in order to ensure that the gateway can
        // always provide an active KMS context
        // If too many parties are compromised for this KMS context, then the relevant functions
        // should be paused instead
        if (ContextLifecycle.isActive($.kmsContextLifecycle, contextId)) {
            revert CompromiseActiveKmsContextNotAllowed(contextId);
        }

        ContextLifecycle.setCompromised($.kmsContextLifecycle, contextId);
        emit CompromiseKmsContext(contextId);
    }

    function destroyKmsContext(uint256 contextId) external virtual onlyOwner ensureContextInitialized(contextId) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        // Do not allow destroying an active KMS context in order to ensure that the gateway can
        // always provide an active KMS context
        if (ContextLifecycle.isActive($.kmsContextLifecycle, contextId)) {
            revert DestroyActiveKmsContextNotAllowed(contextId);
        }

        ContextLifecycle.setDestroyed($.kmsContextLifecycle, contextId);
        emit DestroyKmsContext(contextId);
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
        uint256 contextId
    ) external view virtual ensureContextInitialized(contextId) returns (uint256) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.publicDecryptionThreshold[contextId];
    }

    /// @dev See {IKmsContexts-getUserDecryptionThresholdFromContext}.
    function getUserDecryptionThresholdFromContext(
        uint256 contextId
    ) external view virtual ensureContextInitialized(contextId) returns (uint256) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.userDecryptionThreshold[contextId];
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
        uint256 activeContextId = getActiveKmsContextId();
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        KmsNode memory kmsNode = $.kmsNodes[activeContextId][kmsTxSenderAddress];
        if (kmsNode.txSenderAddress == address(0)) {
            revert NotKmsNodeFromContext(activeContextId, kmsTxSenderAddress);
        }
        return kmsNode;
    }

    /// @dev See {IKmsContexts-getKmsTxSenders}.
    function getKmsTxSenders() external view virtual returns (address[] memory) {
        uint256 activeContextId = getActiveKmsContextId();
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.kmsTxSenderAddresses[activeContextId];
    }

    /// @dev See {IKmsContexts-getKmsSigners}.
    function getKmsSigners() external view virtual returns (address[] memory) {
        uint256 activeContextId = getActiveKmsContextId();
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return $.kmsSignerAddresses[activeContextId];
    }

    function getKmsContextStatus(uint256 contextId) external view virtual returns (ContextStatus) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        return ContextLifecycle.getContextStatus($.kmsContextLifecycle, contextId);
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

    /**
     * @dev Sets the MPC threshold for a KMS context.
     * @param contextId The KMS context ID
     * @param newMpcThreshold The new MPC threshold.
     * @param nKmsNodes The number of KMS nodes associated to this context
     */
    function _setMpcThreshold(uint256 contextId, uint256 newMpcThreshold, uint256 nKmsNodes) internal virtual {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        /// @dev Check that the MPC threshold `t` is valid. It must verify:
        /// @dev - `t >= 0` : this is always true as it's an uint256
        /// @dev - `t < n` : it must be strictly less than the number of registered KMS nodes
        if (newMpcThreshold >= nKmsNodes) {
            revert InvalidHighMpcThreshold(contextId, newMpcThreshold, nKmsNodes);
        }

        $.kmsContexts[contextId].mpcThreshold = newMpcThreshold;
    }

    /**
     * @dev Sets the public decryption threshold for a KMS context.
     * @param contextId The KMS context ID
     * @param publicDecryptionThreshold The public decryption threshold.
     * @param nKmsNodes The number of KMS nodes associated to this context
     */
    function _setPublicDecryptionThreshold(
        uint256 contextId,
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

        $.publicDecryptionThreshold[contextId] = publicDecryptionThreshold;
    }

    /**
     * @dev Sets the user decryption threshold for a KMS context.
     * @param contextId The KMS context ID
     * @param userDecryptionThreshold The user decryption threshold.
     * @param nKmsNodes The number of KMS nodes associated to this context
     */
    function _setUserDecryptionThreshold(
        uint256 contextId,
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

        $.userDecryptionThreshold[contextId] = userDecryptionThreshold;
    }

    /**
     * @dev Sets the decryption thresholds for a KMS context.
     * @param contextId The KMS context ID
     * @param decryptionThresholds The decryption thresholds.
     * @param nKmsNodes The number of KMS nodes associated to this context
     */
    function _setDecryptionThresholds(
        uint256 contextId,
        DecryptionThresholds calldata decryptionThresholds,
        uint256 nKmsNodes
    ) internal virtual {
        _setPublicDecryptionThreshold(contextId, decryptionThresholds.publicDecryptionThreshold, nKmsNodes);
        _setUserDecryptionThreshold(contextId, decryptionThresholds.userDecryptionThreshold, nKmsNodes);
    }

    function _addKmsContext(
        uint256 previousContextId,
        bytes8 softwareVersion,
        uint256 mpcThreshold,
        KmsNode[] calldata kmsNodes,
        DecryptionThresholds calldata decryptionThresholds
    ) internal virtual returns (KmsContext memory) {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        if (kmsNodes.length == 0) {
            revert EmptyKmsNodes();
        }

        // A KMS context ID is never null
        $.kmsContextCount++;
        uint256 contextId = $.kmsContextCount;

        // Solidity doesn't support directly copying complex data structures like KmsNodes (array
        // of structs), so we need to instead create the struct field by field
        $.kmsContexts[contextId].contextId = contextId;
        $.kmsContexts[contextId].previousContextId = previousContextId;
        $.kmsContexts[contextId].softwareVersion = softwareVersion;
        _setMpcThreshold(contextId, mpcThreshold, kmsNodes.length);

        // Then, we need copy each KMS node struct one by one
        for (uint256 i = 0; i < kmsNodes.length; i++) {
            $.kmsContexts[contextId].kmsNodes.push(kmsNodes[i]);
        }

        // Register several mappings for faster lookups
        for (uint256 i = 0; i < kmsNodes.length; i++) {
            $.kmsNodes[contextId][kmsNodes[i].txSenderAddress] = kmsNodes[i];
            $.isKmsTxSender[contextId][kmsNodes[i].txSenderAddress] = true;
            $.kmsTxSenderAddresses[contextId].push(kmsNodes[i].txSenderAddress);
            $.isKmsSigner[contextId][kmsNodes[i].signerAddress] = true;
            $.kmsSignerAddresses[contextId].push(kmsNodes[i].signerAddress);
        }

        _setDecryptionThresholds(contextId, decryptionThresholds, kmsNodes.length);

        return $.kmsContexts[contextId];
    }

    /**
     * @notice Validates the EIP712 signature for key resharing.
     * @param contextId The decryption request ID.
     * @param digest The hashed EIP712 struct.
     * @param signature The signature to validate.
     */
    function _validateKeyResharingEIP712Signature(
        uint256 contextId,
        bytes32 digest,
        bytes calldata signature
    ) internal virtual {
        KmsContextsStorage storage $ = _getKmsContextsStorage();
        address signer = ECDSA.recover(digest, signature);

        /// @dev Check that the signer is a KMS signer from the KMS context.
        checkIsKmsSignerFromContext(contextId, signer);

        /// @dev Check that the signer has not already validated the key resharing.
        if ($.kmsNodeDoneKeyResharing[contextId][signer]) {
            revert KmsNodeAlreadyValidatedKeyResharing(contextId, signer);
        }

        $.kmsNodeDoneKeyResharing[contextId][signer] = true;
    }

    function _preActivateKmsContext(KmsContext memory kmsContext, uint256 preActivationBlockPeriod) internal virtual {
        KmsContextsStorage storage $ = _getKmsContextsStorage();

        ContextLifecycle.setPreActivation($.kmsContextLifecycle, kmsContext.contextId);

        uint256 activationBlockNumber = block.number + preActivationBlockPeriod;
        $.kmsContextActivationBlockNumber[kmsContext.contextId] = activationBlockNumber;

        emit PreActivateKmsContext(kmsContext, activationBlockNumber);
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
            _hashTypedDataV4(keccak256(abi.encode(EIP712_KEY_RESHARING_TYPE_HASH, keyResharingVerification.contextId)));
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
