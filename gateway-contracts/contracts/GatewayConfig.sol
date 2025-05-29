// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./interfaces/IGatewayConfig.sol";
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
 * @title GatewayConfig contract
 * @dev See {IGatewayConfig}.
 * @dev Add/remove methods will be added in the future for KMS nodes, coprocessors and host chains.
 * @dev See https://github.com/zama-ai/fhevm-gateway/issues/98 for more details.
 */
contract GatewayConfig is IGatewayConfig, Ownable2StepUpgradeable, UUPSUpgradeable, EIP712Upgradeable, Pausable {
    /// @notice The typed data structure for the EIP712 signature to validate in key resharing responses.
    /// @dev The name of this struct is not relevant for the signature validation, only the one defined
    /// @dev EIP712_KEY_RESHARING_TYPE is, but we keep it the same for clarity.
    struct KeyResharingVerification {
        /// @notice The KMS context ID
        uint256 kmsContextId;
    }

    /// @notice The maximum chain ID.
    uint256 internal constant MAX_CHAIN_ID = type(uint64).max;

    /// @notice The definition of the KeyResharingVerification structure typed data.
    string private constant EIP712_KEY_RESHARING_TYPE = "KeyResharingVerification(uint256 kmsContextId)";

    /// @notice The hash of the KeyResharingVerification structure typed data definition used for signature validation.
    bytes32 private constant EIP712_KEY_RESHARING_TYPE_HASH = keccak256(bytes(EIP712_KEY_RESHARING_TYPE));

    /// @dev The following constants are used for versioning the contract. They are made private
    /// @dev in order to force derived contracts to consider a different version. Note that
    /// @dev they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "GatewayConfig";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:fhevm_gateway.storage.GatewayConfig
    struct GatewayConfigStorage {
        /// @notice The pauser's address
        address pauser;
        /// @notice The protocol's metadata
        ProtocolMetadata protocolMetadata;
        // ----------------------------------------------------------------------------------------------
        // KMS state variables:
        // ----------------------------------------------------------------------------------------------
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
        // ----------------------------------------------------------------------------------------------
        // Coprocessor state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The coprocessors' transaction sender addresses
        mapping(address coprocessorTxSenderAddress => bool isCoprocessorTxSender) _isCoprocessorTxSender;
        /// @notice The coprocessors' signer addresses
        mapping(address coprocessorSignerAddress => bool isCoprocessorSigner) _isCoprocessorSigner;
        /// @notice The coprocessors' metadata
        mapping(address coprocessorTxSenderAddress => Coprocessor coprocessor) coprocessors;
        /// @notice The coprocessors' transaction sender address list
        address[] coprocessorTxSenderAddresses;
        /// @notice The coprocessors' signer address list
        address[] coprocessorSignerAddresses;
        // ----------------------------------------------------------------------------------------------
        // Host chain state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The host chains' metadata
        HostChain[] hostChains;
        /// @notice The host chains' registered status
        mapping(uint256 chainId => bool isRegistered) _isHostChainRegistered;
    }

    /// @dev Storage location has been computed using the following command:
    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.GatewayConfig")) - 1))
    /// @dev & ~bytes32(uint256(0xff))
    bytes32 private constant GATEWAY_CONFIG_STORAGE_LOCATION =
        0x86d3070a8993f6b209bee6185186d38a07fce8bbd97c750d934451b72f35b400;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    modifier ensureKmsContextInitialized(uint256 kmsContextId) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if ($.kmsContexts[kmsContextId].contextId == 0) {
            revert KmsContextNotInitialized(kmsContextId);
        }
        _;
    }

    /// @notice Initializes the contract
    /// @dev This function needs to be public in order to be called by the UUPS proxy.
    /// @param initialPauser Pauser address
    /// @param initialMetadata Metadata of the protocol
    /// @param initialKmsConfiguration KMS configuration parameters
    /// @param initialCoprocessors List of coprocessors
    function initialize(
        address initialPauser,
        ProtocolMetadata calldata initialMetadata,
        KmsConfiguration calldata initialKmsConfiguration,
        Coprocessor[] calldata initialCoprocessors
    ) public virtual reinitializer(2) {
        __Ownable_init(owner());
        __EIP712_init(CONTRACT_NAME, "1");
        __Pausable_init();

        if (initialPauser == address(0)) {
            revert InvalidNullPauser();
        }

        if (initialKmsConfiguration.kmsNodes.length == 0) {
            revert EmptyKmsNodes();
        }

        if (initialCoprocessors.length == 0) {
            revert EmptyCoprocessors();
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        $.protocolMetadata = initialMetadata;

        /// @dev Register the pauser
        $.pauser = initialPauser;

        // The key resharing flag is set to false at initialization as there is no previous KMS context
        // to reshare from
        KmsContext memory newKmsContext = _addKmsContext(
            initialKmsConfiguration.softwareVersion,
            false,
            initialKmsConfiguration.mpcThreshold,
            initialKmsConfiguration.kmsNodes
        );

        ContextLifecycle.setActive($.kmsContextLifecycle, newKmsContext.contextId);

        _setKmsBlockPeriods(initialKmsConfiguration.blockPeriods, newKmsContext.contextId);

        _setDecryptionThresholds(newKmsContext, initialKmsConfiguration.decryptionThresholds);

        /// @dev Register the coprocessors
        for (uint256 i = 0; i < initialCoprocessors.length; i++) {
            $._isCoprocessorTxSender[initialCoprocessors[i].txSenderAddress] = true;
            $.coprocessors[initialCoprocessors[i].txSenderAddress] = initialCoprocessors[i];
            $.coprocessorTxSenderAddresses.push(initialCoprocessors[i].txSenderAddress);
            $._isCoprocessorSigner[initialCoprocessors[i].signerAddress] = true;
            $.coprocessorSignerAddresses.push(initialCoprocessors[i].signerAddress);
        }

        emit Initialization(initialPauser, initialMetadata, initialKmsConfiguration, initialCoprocessors);
    }

    /// @dev See {IGatewayConfig-checkIsKmsTxSenderFromContext}.
    function checkIsKmsTxSenderFromContext(
        uint256 kmsContextId,
        address txSenderAddress
    ) public view virtual ensureKmsContextInitialized(kmsContextId) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$.isKmsTxSender[kmsContextId][txSenderAddress]) {
            revert NotKmsTxSenderFromContext(kmsContextId, txSenderAddress);
        }
    }

    /// @dev See {IGatewayConfig-checkIsKmsSignerFromContext}.
    function checkIsKmsSignerFromContext(
        uint256 kmsContextId,
        address signerAddress
    ) public view virtual ensureKmsContextInitialized(kmsContextId) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$.isKmsSigner[kmsContextId][signerAddress]) {
            revert NotKmsSignerFromContext(kmsContextId, signerAddress);
        }
    }

    /// @dev See {IGatewayConfig-getKmsContext}.
    function getKmsContext(
        uint256 kmsContextId
    ) public view virtual ensureKmsContextInitialized(kmsContextId) returns (KmsContext memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContexts[kmsContextId];
    }

    /**
     * @dev See {IGatewayConfig-getActiveKmsContextId}.
     * There should always be an active KMS context defined in the gateway, as we do not allow
     * to manually set active KMS contexts to `Compromised` or `Destroyed` states
     */
    function getActiveKmsContextId() public view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextLifecycle.activeContextId;
    }

    /**
     * @dev See {IGatewayConfig-getSuspendedKmsContextId}.
     */
    function getSuspendedKmsContextId() public view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextLifecycle.suspendedContextId;
    }

    /**
     * @dev See {IGatewayConfig-getActiveKmsContext}.
     * There should always be an active KMS context defined in the gateway, as we do not allow
     * to manually set active KMS contexts to `Compromised` or `Destroyed` states
     */
    function getActiveKmsContext() public view virtual returns (KmsContext memory) {
        uint256 activeContextId = getActiveKmsContextId();
        return getKmsContext(activeContextId);
    }

    /// @dev See {IGatewayConfig-getKmsNodes}.
    function getKmsNodes() public view virtual returns (KmsNode[] memory) {
        return getActiveKmsContext().kmsNodes;
    }

    /// @dev See {IGatewayConfig-getKmsNodeFromContext}.
    function getKmsNodeFromContext(
        uint256 kmsContextId,
        address kmsTxSenderAddress
    ) public view virtual ensureKmsContextInitialized(kmsContextId) returns (KmsNode memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        KmsNode memory kmsNode = $.kmsNodes[kmsContextId][kmsTxSenderAddress];
        if (kmsNode.txSenderAddress == address(0)) {
            revert NotKmsNode(kmsContextId, kmsTxSenderAddress);
        }
        return kmsNode;
    }

    /// @dev See {IGatewayConfig-getKmsTxSendersFromContext}.
    function getKmsTxSendersFromContext(
        uint256 kmsContextId
    ) public view virtual ensureKmsContextInitialized(kmsContextId) returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsTxSenderAddresses[kmsContextId];
    }

    /// @dev See {IGatewayConfig-getKmsSignersFromContext}.
    function getKmsSignersFromContext(
        uint256 kmsContextId
    ) public view virtual ensureKmsContextInitialized(kmsContextId) returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsSignerAddresses[kmsContextId];
    }

    /// @dev See {IGatewayConfig-getPublicDecryptionThresholdFromContext}.
    function getPublicDecryptionThresholdFromContext(
        uint256 kmsContextId
    ) public view virtual ensureKmsContextInitialized(kmsContextId) returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.publicDecryptionThreshold[kmsContextId];
    }

    /// @dev See {IGatewayConfig-getUserDecryptionThresholdFromContext}.
    function getUserDecryptionThresholdFromContext(
        uint256 kmsContextId
    ) public view virtual ensureKmsContextInitialized(kmsContextId) returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.userDecryptionThreshold[kmsContextId];
    }

    /// @dev See {IGatewayConfig-updatePauser}.
    function updatePauser(address newPauser) external virtual onlyOwner whenNotPaused {
        if (newPauser == address(0)) {
            revert InvalidNullPauser();
        }
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        $.pauser = newPauser;
        emit UpdatePauser(newPauser);
    }

    /// @dev See {IGatewayConfig-updatePublicDecryptionThreshold}.
    function updatePublicDecryptionThreshold(
        uint256 newPublicDecryptionThreshold
    ) external virtual onlyOwner whenNotPaused {
        KmsContext memory activeKmsContext = getActiveKmsContext();
        _setPublicDecryptionThreshold(activeKmsContext, newPublicDecryptionThreshold);
        emit UpdatePublicDecryptionThreshold(newPublicDecryptionThreshold);
    }

    /// @dev See {IGatewayConfig-updateUserDecryptionThreshold}.
    function updateUserDecryptionThreshold(
        uint256 newUserDecryptionThreshold
    ) external virtual onlyOwner whenNotPaused {
        KmsContext memory activeKmsContext = getActiveKmsContext();
        _setUserDecryptionThreshold(activeKmsContext, newUserDecryptionThreshold);
        emit UpdateUserDecryptionThreshold(newUserDecryptionThreshold);
    }

    /// @dev See {IGatewayConfig-updateKmsContextGenerationBlockPeriod}.
    function updateKmsContextGenerationBlockPeriod(
        uint256 newKmsContextGenerationBlockPeriod
    ) external virtual onlyOwner whenNotPaused {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        $.kmsContextGenerationBlockPeriod = newKmsContextGenerationBlockPeriod;
        emit UpdateKmsContextGenerationBlockPeriod(newKmsContextGenerationBlockPeriod);
    }

    /// @dev See {IGatewayConfig-updateKmsContextSuspensionBlockPeriod}.
    function updateKmsContextSuspensionBlockPeriod(
        uint256 newKmsContextSuspensionBlockPeriod
    ) external virtual onlyOwner whenNotPaused {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        $.kmsContextSuspensionBlockPeriod = newKmsContextSuspensionBlockPeriod;
        emit UpdateKmsContextSuspensionBlockPeriod(newKmsContextSuspensionBlockPeriod);
    }

    /// @dev See {IGatewayConfig-addKmsContext}.
    function addKmsContext(
        uint256 preActivationBlockPeriod,
        bytes calldata softwareVersion,
        bool reshareKeys,
        uint256 mpcThreshold,
        KmsNode[] calldata kmsNodes,
        DecryptionThresholds calldata decryptionThresholds
    ) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        KmsContext memory newKmsContext = _addKmsContext(softwareVersion, reshareKeys, mpcThreshold, kmsNodes);

        _setDecryptionThresholds(newKmsContext, decryptionThresholds);

        // Get the current active KMS context
        KmsContext memory activeKmsContext = getActiveKmsContext();

        // Emit the `NewKmsContext` event in any case
        emit NewKmsContext(activeKmsContext, newKmsContext);

        // If the `reshareKeys` flag is set or if the number of KMS nodes has changed, a key resharing is triggered
        // TODO: We should not trigger key resharing in case of parties having too many compromised parties
        // See: https://github.com/zama-ai/fhevm-gateway/issues/393
        if (reshareKeys || (activeKmsContext.kmsNodes.length != newKmsContext.kmsNodes.length)) {
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

    function validateKeyResharing(
        uint256 kmsContextId,
        bytes calldata signature
    ) external virtual ensureKmsContextInitialized(kmsContextId) {
        // Only accept KMS transaction senders from the associated context
        checkIsKmsTxSenderFromContext(kmsContextId, msg.sender);

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

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

        if (_isConsensusReachedKeyResharing(verifiedSignatures.length)) {
            KmsContext memory newKmsContext = getKmsContext(kmsContextId);
            uint256 preActivationBlockPeriod = $.kmsContextPreActivationBlockPeriod[kmsContextId];
            _preActivateKmsContext(newKmsContext, preActivationBlockPeriod);
            emit ValidateKeyResharing(newKmsContext);
        }
    }

    function refreshKmsContextStatuses() external virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

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
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
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
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Do not allow destroying an active KMS context in order to ensure that the gateway can
        // always provide an active KMS context
        if (ContextLifecycle.isActive($.kmsContextLifecycle, kmsContextId)) {
            revert DestroyActiveKmsContextNotAllowed(kmsContextId);
        }

        ContextLifecycle.setDestroyed($.kmsContextLifecycle, kmsContextId);
        emit DestroyKmsContext(kmsContextId);
    }

    function moveSuspendedKmsContextToActive() external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
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

    /// @dev See {IGatewayConfig-addHostChain}.
    function addHostChain(HostChain calldata hostChain) external virtual onlyOwner whenNotPaused {
        if (hostChain.chainId == 0) {
            revert InvalidNullChainId();
        }
        if (hostChain.chainId > MAX_CHAIN_ID) {
            revert ChainIdNotUint64(hostChain.chainId);
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if ($._isHostChainRegistered[hostChain.chainId]) {
            revert HostChainAlreadyRegistered(hostChain.chainId);
        }

        $.hostChains.push(hostChain);
        $._isHostChainRegistered[hostChain.chainId] = true;
        emit AddHostChain(hostChain);
    }

    /// @dev See {IGatewayConfig-checkIsPauser}.
    function checkIsPauser(address pauserAddress) external view virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if ($.pauser != pauserAddress) {
            revert NotPauser(pauserAddress);
        }
    }

    /// @dev See {IGatewayConfig-checkIsCoprocessorTxSender}.
    function checkIsCoprocessorTxSender(address txSenderAddress) external view virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$._isCoprocessorTxSender[txSenderAddress]) {
            revert NotCoprocessorTxSender(txSenderAddress);
        }
    }

    /// @dev See {IGatewayConfig-checkIsCoprocessorSigner}.
    function checkIsCoprocessorSigner(address signerAddress) external view virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$._isCoprocessorSigner[signerAddress]) {
            revert NotCoprocessorSigner(signerAddress);
        }
    }

    /// @dev See {IGatewayConfig-checkHostChainIsRegistered}.
    function checkHostChainIsRegistered(uint256 chainId) external view virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$._isHostChainRegistered[chainId]) {
            revert HostChainNotRegistered(chainId);
        }
    }

    /// @dev See {IGatewayConfig-getPauser}.
    function getPauser() external view virtual returns (address) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.pauser;
    }

    /// @dev See {IGatewayConfig-getProtocolMetadata}.
    function getProtocolMetadata() external view virtual returns (ProtocolMetadata memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.protocolMetadata;
    }

    /// @dev See {IGatewayConfig-getMpcThreshold}.
    function getMpcThreshold() external view virtual returns (uint256) {
        KmsContext memory activeKmsContext = getActiveKmsContext();
        return activeKmsContext.mpcThreshold;
    }

    /// @dev See {IGatewayConfig-getPublicDecryptionThreshold}.
    function getPublicDecryptionThreshold() external view virtual returns (uint256) {
        uint256 activeKmsContextId = getActiveKmsContextId();
        return getPublicDecryptionThresholdFromContext(activeKmsContextId);
    }

    /// @dev See {IGatewayConfig-getUserDecryptionThreshold}.
    function getUserDecryptionThreshold() external view virtual returns (uint256) {
        uint256 activeKmsContextId = getActiveKmsContextId();
        return getUserDecryptionThresholdFromContext(activeKmsContextId);
    }

    /// @dev See {IGatewayConfig-getCoprocessorMajorityThreshold}.
    function getCoprocessorMajorityThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessorTxSenderAddresses.length / 2 + 1;
    }

    /// @dev See {IGatewayConfig-getKmsContextGenerationBlockPeriod}.
    function getKmsContextGenerationBlockPeriod() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextGenerationBlockPeriod;
    }

    /// @dev See {IGatewayConfig-getKmsContextSuspensionBlockPeriod}.
    function getKmsContextSuspensionBlockPeriod() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextSuspensionBlockPeriod;
    }

    /// @dev See {IGatewayConfig-getKmsNode}.
    function getKmsNode(address kmsTxSenderAddress) external view virtual returns (KmsNode memory) {
        uint256 activeKmsContextId = getActiveKmsContextId();
        return getKmsNodeFromContext(activeKmsContextId, kmsTxSenderAddress);
    }

    /// @dev See {IGatewayConfig-getKmsTxSenders}.
    function getKmsTxSenders() external view virtual returns (address[] memory) {
        uint256 activeKmsContextId = getActiveKmsContextId();
        return getKmsTxSendersFromContext(activeKmsContextId);
    }

    /// @dev See {IGatewayConfig-getKmsSigners}.
    function getKmsSigners() external view virtual returns (address[] memory) {
        uint256 activeKmsContextId = getActiveKmsContextId();
        return getKmsSignersFromContext(activeKmsContextId);
    }

    function getKmsContextStatus(uint256 kmsContextId) external view virtual returns (ContextStatus) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return ContextLifecycle.getContextStatus($.kmsContextLifecycle, kmsContextId);
    }

    /// @dev See {IGatewayConfig-getCoprocessor}.
    function getCoprocessor(address coprocessorTxSenderAddress) external view virtual returns (Coprocessor memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessors[coprocessorTxSenderAddress];
    }

    /// @dev See {IGatewayConfig-getCoprocessorTxSenders}.
    function getCoprocessorTxSenders() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessorTxSenderAddresses;
    }

    /// @dev See {IGatewayConfig-getCoprocessorSigners}.
    function getCoprocessorSigners() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessorSignerAddresses;
    }

    /// @dev See {IGatewayConfig-getHostChain}.
    function getHostChain(uint256 index) external view virtual returns (HostChain memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.hostChains[index];
    }

    /// @dev See {IGatewayConfig-getHostChains}.
    function getHostChains() external view virtual returns (HostChain[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.hostChains;
    }

    /// @dev See {IGatewayConfig-getVersion}.
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
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
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
    function _setMpcThreshold(
        uint256 kmsContextId,
        uint256 newMpcThreshold,
        uint256 nKmsNodes
    ) internal virtual ensureKmsContextInitialized(kmsContextId) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

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
     * @param kmsContext The KMS context
     * @param publicDecryptionThreshold The public decryption threshold.
     */
    function _setPublicDecryptionThreshold(
        KmsContext memory kmsContext,
        uint256 publicDecryptionThreshold
    ) internal virtual ensureKmsContextInitialized(kmsContext.contextId) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        /// @dev Check that the public decryption threshold `t` is valid. It must verify:
        /// @dev - `t >= 1` : the public decryption consensus should require at least one vote
        /// @dev - `t <= n` : it must be less than the number of registered KMS nodes
        if (publicDecryptionThreshold == 0) {
            revert InvalidNullPublicDecryptionThreshold();
        }
        uint256 nKmsNodes = kmsContext.kmsNodes.length;
        if (publicDecryptionThreshold > nKmsNodes) {
            revert InvalidHighPublicDecryptionThreshold(publicDecryptionThreshold, nKmsNodes);
        }

        $.publicDecryptionThreshold[kmsContext.contextId] = publicDecryptionThreshold;
    }

    /**
     * @dev Sets the user decryption threshold for a KMS context.
     * @param kmsContext The KMS context
     * @param userDecryptionThreshold The user decryption threshold.
     */
    function _setUserDecryptionThreshold(
        KmsContext memory kmsContext,
        uint256 userDecryptionThreshold
    ) internal virtual ensureKmsContextInitialized(kmsContext.contextId) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        /// @dev Check that the user decryption threshold `t` is valid. It must verify:
        /// @dev - `t >= 1` : the user decryption consensus should require at least one vote
        /// @dev - `t <= n` : it must be less than the number of registered KMS nodes
        if (userDecryptionThreshold == 0) {
            revert InvalidNullUserDecryptionThreshold();
        }
        uint256 nKmsNodes = kmsContext.kmsNodes.length;
        if (userDecryptionThreshold > nKmsNodes) {
            revert InvalidHighUserDecryptionThreshold(userDecryptionThreshold, nKmsNodes);
        }

        $.userDecryptionThreshold[kmsContext.contextId] = userDecryptionThreshold;
    }

    /**
     * @dev Sets the decryption thresholds for a KMS context.
     * @param kmsContext The KMS context
     * @param decryptionThresholds The decryption thresholds.
     */
    function _setDecryptionThresholds(
        KmsContext memory kmsContext,
        DecryptionThresholds calldata decryptionThresholds
    ) internal virtual {
        _setPublicDecryptionThreshold(kmsContext, decryptionThresholds.publicDecryptionThreshold);
        _setUserDecryptionThreshold(kmsContext, decryptionThresholds.userDecryptionThreshold);
    }

    function _addKmsContext(
        bytes calldata softwareVersion,
        bool reshareKeys,
        uint256 mpcThreshold,
        KmsNode[] calldata kmsNodes
    ) internal virtual returns (KmsContext memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // A KMS context ID is never null
        $.kmsContextCount++;
        uint256 kmsContextId = $.kmsContextCount;

        // Solidity doesn't support directly copying complex data structures like KmsNodes (array
        // of structs), so we need to instead create the struct field by field
        $.kmsContexts[kmsContextId].contextId = kmsContextId;
        $.kmsContexts[kmsContextId].softwareVersion = softwareVersion;
        $.kmsContexts[kmsContextId].reshareKeys = reshareKeys;
        _setMpcThreshold(kmsContextId, kmsNodes.length, mpcThreshold);

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
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
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
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        uint256 preActivationBlockNumber = block.number + preActivationBlockPeriod;
        $.kmsContextPreActivationBlockNumber[kmsContext.contextId] = preActivationBlockNumber;

        emit StartKmsContextPreActivation(kmsContext, preActivationBlockNumber);
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
     * @notice Checks if the consensus is reached among the KMS nodes.
     * @dev The consensus is reached when all the KMS nodes have validated the key resharing.
     * @param kmsCounter The number of KMS nodes that agreed
     * @return Whether the consensus is reached
     */
    function _isConsensusReachedKeyResharing(uint256 kmsCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = getKmsNodes().length;
        return kmsCounter >= consensusThreshold;
    }

    /**
     * @dev Returns the GatewayConfig storage location.
     * Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getGatewayConfigStorage() internal pure returns (GatewayConfigStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := GATEWAY_CONFIG_STORAGE_LOCATION
        }
    }
}
