// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { IPauserSet } from "./interfaces/IPauserSet.sol";
import { decryptionAddress, inputVerificationAddress, pauserSetAddress } from "../addresses/GatewayAddresses.sol";
import { Decryption } from "./Decryption.sol";
import { InputVerification } from "./InputVerification.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { Pausable } from "./shared/Pausable.sol";
import { ProtocolMetadata, HostChain, Coprocessor, Custodian, KmsNode } from "./shared/Structs.sol";

/**
 * @title GatewayConfig contract
 * @notice See {IGatewayConfig}.
 * @dev Add/remove methods will be added in the future for KMS nodes, coprocessors and host chains.
 * See https://github.com/zama-ai/fhevm-gateway/issues/98 for more details.
 */
contract GatewayConfig is IGatewayConfig, Ownable2StepUpgradeable, UUPSUpgradeableEmptyProxy {
    /**
     * @notice The maximum chain ID.
     */
    uint256 internal constant MAX_CHAIN_ID = type(uint64).max;

    // ----------------------------------------------------------------------------------------------
    // Contract information:
    // ----------------------------------------------------------------------------------------------

    /**
     * @dev The following constants are used for versioning the contract. They are made private
     * in order to force derived contracts to consider a different version. Note that
     * they can still define their own private constants with the same name.
     */
    string private constant CONTRACT_NAME = "GatewayConfig";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 5;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     * This constant does not represent the number of time a specific contract have been upgraded,
     * as a contract deployed from version VX will have a REINITIALIZER_VERSION > 2.
     */
    uint64 private constant REINITIALIZER_VERSION = 6;

    /**
     * @notice The address of the all gateway contracts
     */
    Decryption private constant DECRYPTION = Decryption(decryptionAddress);
    InputVerification private constant INPUT_VERIFICATION = InputVerification(inputVerificationAddress);
    IPauserSet private constant PAUSER_SET = IPauserSet(pauserSetAddress);

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.GatewayConfig
    struct GatewayConfigStorage {
        // ----------------------------------------------------------------------------------------------
        // Protocol metadata state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The protocol's metadata
        ProtocolMetadata protocolMetadata;
        // ----------------------------------------------------------------------------------------------
        // Deprecated KMS nodes state variables (replaced by per-context mappings below):
        // @dev These fields must remain to preserve the storage layout for UUPS proxy upgrades.
        // ----------------------------------------------------------------------------------------------
        /// @dev Deprecated. Use `isKmsContextTxSender` instead.
        mapping(address kmsTxSenderAddress => bool isTxSender) isKmsTxSender;
        /// @dev Deprecated. Use `isKmsContextSigner` instead.
        mapping(address kmsSignerAddress => bool isSigner) isKmsSigner;
        /// @dev Deprecated. Use `kmsContextNodes` instead.
        mapping(address kmsTxSenderAddress => KmsNode kmsNode) kmsNodes;
        /// @dev Deprecated. Use `kmsContextTxSenderAddresses` instead.
        address[] kmsTxSenderAddresses;
        /// @dev Deprecated. Use `kmsContextSignerAddresses` instead.
        address[] kmsSignerAddresses;
        /// @dev Deprecated. Use `kmsContextMpcThreshold` instead.
        uint256 mpcThreshold;
        /// @dev Deprecated. Use `kmsContextPublicDecryptionThreshold` instead.
        uint256 publicDecryptionThreshold;
        /// @dev Deprecated. Use `kmsContextUserDecryptionThreshold` instead.
        uint256 userDecryptionThreshold;
        // ----------------------------------------------------------------------------------------------
        // Coprocessors state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The coprocessors' transaction sender addresses
        mapping(address coprocessorTxSenderAddress => bool isTxSender) isCoprocessorTxSender;
        /// @notice The coprocessors' signer addresses
        mapping(address coprocessorSignerAddress => bool isSigner) isCoprocessorSigner;
        /// @notice The coprocessors' metadata
        mapping(address coprocessorTxSenderAddress => Coprocessor coprocessor) coprocessors;
        /// @notice The coprocessors' transaction sender address list
        address[] coprocessorTxSenderAddresses;
        /// @notice The coprocessors' signer address list
        address[] coprocessorSignerAddresses;
        // ----------------------------------------------------------------------------------------------
        // Host chain state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The host chains' registered status
        mapping(uint256 chainId => bool isRegistered) isHostChainRegistered;
        /// @notice The host chains' metadata
        HostChain[] hostChains;
        // ----------------------------------------------------------------------------------------------
        // Custodians state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The custodians' metadata
        mapping(address custodianTxSenderAddress => Custodian custodian) custodians;
        /// @notice The custodians' transaction sender address list
        address[] custodianTxSenderAddresses;
        /// @notice The custodians' signer address list
        address[] custodianSignerAddresses;
        /// @notice The custodians' transaction sender addresses
        mapping(address custodianTxSenderAddress => bool isTxSender) isCustodianTxSender;
        /// @notice The custodians' signer addresses
        mapping(address custodianSignerAddress => bool isSigner) isCustodianSigner;
        /// @dev Deprecated. Use `kmsContextKmsGenThreshold` instead.
        uint256 kmsGenThreshold;
        // ----------------------------------------------------------------------------------------------
        // Coprocessor threshold state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The threshold to consider for coprocessor consensus
        uint256 coprocessorThreshold;
        // ----------------------------------------------------------------------------------------------
        // KMS context state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The current KMS context ID
        uint256 currentKmsContextId;
        /// @notice The KMS nodes' transaction sender addresses per context
        mapping(uint256 contextId => mapping(address kmsTxSenderAddress => bool isTxSender)) isKmsContextTxSender;
        /// @notice The KMS nodes' signer addresses per context
        mapping(uint256 contextId => mapping(address kmsSignerAddress => bool isSigner)) isKmsContextSigner;
        /// @notice The KMS nodes' metadata per context
        mapping(uint256 contextId => mapping(address kmsTxSenderAddress => KmsNode kmsNode)) kmsContextNodes;
        /// @notice The KMS nodes' transaction sender address list per context
        mapping(uint256 contextId => address[]) kmsContextTxSenderAddresses;
        /// @notice The KMS nodes' signer address list per context
        mapping(uint256 contextId => address[]) kmsContextSignerAddresses;
        /// @notice The public decryption threshold per context
        mapping(uint256 contextId => uint256) kmsContextPublicDecryptionThreshold;
        /// @notice The user decryption threshold per context
        mapping(uint256 contextId => uint256) kmsContextUserDecryptionThreshold;
        /// @notice The MPC threshold per context
        mapping(uint256 contextId => uint256) kmsContextMpcThreshold;
        /// @notice The key and CRS generation threshold per context
        mapping(uint256 contextId => uint256) kmsContextKmsGenThreshold;
    }

    /**
     * @dev Storage location has been computed using the following command:
     * keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.GatewayConfig")) - 1))
     * & ~bytes32(uint256(0xff))
     */
    bytes32 private constant GATEWAY_CONFIG_STORAGE_LOCATION =
        0x86d3070a8993f6b209bee6185186d38a07fce8bbd97c750d934451b72f35b400;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    modifier onlyPauser() {
        if (!isPauser(msg.sender)) {
            revert NotPauser(msg.sender);
        }
        _;
    }

    /**
     * @notice Initializes the contract
     * @dev This function needs to be public in order to be called by the UUPS proxy.
     * @param initialMetadata Metadata of the protocol
     * @param initialThresholds The operator thresholds
     * @param initialKmsNodes List of KMS nodes
     * @param initialCoprocessors List of coprocessors
     * @param initialCustodians List of custodians
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        uint256 initialKmsContextId,
        ProtocolMetadata calldata initialMetadata,
        Thresholds calldata initialThresholds,
        KmsNode[] calldata initialKmsNodes,
        Coprocessor[] calldata initialCoprocessors,
        Custodian[] calldata initialCustodians
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __Ownable_init(owner());
        if (initialKmsContextId == 0) {
            revert InvalidNullKmsContextId();
        }

        // Using scoped block to avoid stack depth error
        {
            GatewayConfigStorage storage $ = _getGatewayConfigStorage();
            $.protocolMetadata = initialMetadata;
            $.currentKmsContextId = initialKmsContextId;
        }

        // Initialize the KMS context with the KMS nodes and all thresholds
        _setKmsContext(
            initialKmsContextId,
            initialKmsNodes,
            initialThresholds.mpcThreshold,
            initialThresholds.publicDecryptionThreshold,
            initialThresholds.userDecryptionThreshold,
            initialThresholds.kmsGenThreshold
        );

        // Set the coprocessors and their threshold
        _setCoprocessors(initialCoprocessors, initialThresholds.coprocessorThreshold);

        // Set the custodians
        _setCustodians(initialCustodians);

        emit InitializeGatewayConfig(
            initialKmsContextId,
            initialMetadata,
            initialThresholds,
            initialKmsNodes,
            initialCoprocessors,
            initialCustodians
        );
    }

    /**
     * @notice Re-initializes the contract from V4.
     * @dev Define a `reinitializeVX` function once the contract needs to be upgraded.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV5(uint256 initialKmsContextId) public virtual reinitializer(REINITIALIZER_VERSION) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        if (initialKmsContextId == 0) {
            revert InvalidNullKmsContextId();
        }

        // Migrate existing global KMS nodes to the initial KMS context ID
        uint256 nKmsNodes = $.kmsTxSenderAddresses.length;
        for (uint256 i = 0; i < nKmsNodes; i++) {
            address txSenderAddr = $.kmsTxSenderAddresses[i];
            address signerAddr = $.kmsSignerAddresses[i];

            $.isKmsContextTxSender[initialKmsContextId][txSenderAddr] = true;
            $.isKmsContextSigner[initialKmsContextId][signerAddr] = true;
            $.kmsContextNodes[initialKmsContextId][txSenderAddr] = $.kmsNodes[txSenderAddr];
            $.kmsContextTxSenderAddresses[initialKmsContextId].push(txSenderAddr);
            $.kmsContextSignerAddresses[initialKmsContextId].push(signerAddr);
        }

        // Migrate all thresholds
        $.kmsContextMpcThreshold[initialKmsContextId] = $.mpcThreshold;
        $.kmsContextPublicDecryptionThreshold[initialKmsContextId] = $.publicDecryptionThreshold;
        $.kmsContextUserDecryptionThreshold[initialKmsContextId] = $.userDecryptionThreshold;
        $.kmsContextKmsGenThreshold[initialKmsContextId] = $.kmsGenThreshold;

        // Set the current context ID
        $.currentKmsContextId = initialKmsContextId;
    }

    /**
     * @notice See {IGatewayConfig-isPauser}.
     */
    function isPauser(address account) public view virtual returns (bool) {
        return PAUSER_SET.isPauser(account);
    }

    /**
     * @notice See {IGatewayConfig-updateKmsContext}.
     */
    function updateKmsContext(
        uint256 contextId,
        KmsNode[] calldata newKmsNodes,
        uint256 newMpcThreshold,
        uint256 newPublicDecryptionThreshold,
        uint256 newUserDecryptionThreshold,
        uint256 newKmsGenThreshold
    ) public virtual onlyOwner {
        if (contextId == 0) {
            revert InvalidNullKmsContextId();
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Validate contextId is strictly greater than the current one
        if (contextId <= $.currentKmsContextId) {
            revert KmsContextAlreadyRegistered(contextId, $.currentKmsContextId);
        }

        // Set the new context-indexed KMS nodes and all thresholds
        _setKmsContext(
            contextId,
            newKmsNodes,
            newMpcThreshold,
            newPublicDecryptionThreshold,
            newUserDecryptionThreshold,
            newKmsGenThreshold
        );

        // Update the current KMS context ID
        $.currentKmsContextId = contextId;

        emit UpdateKmsContext(
            contextId,
            newKmsNodes,
            newMpcThreshold,
            newPublicDecryptionThreshold,
            newUserDecryptionThreshold,
            newKmsGenThreshold
        );
    }

    /**
     * @notice See {IGatewayConfig-updateCoprocessors}.
     */
    function updateCoprocessors(
        Coprocessor[] calldata newCoprocessors,
        uint256 newCoprocessorThreshold
    ) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Remove the old coprocessors
        uint256 oldCoprocessorTxSenderAddressesLength = $.coprocessorTxSenderAddresses.length;
        for (uint256 i = 0; i < oldCoprocessorTxSenderAddressesLength; i++) {
            $.isCoprocessorTxSender[$.coprocessorTxSenderAddresses[i]] = false;
            $.isCoprocessorSigner[$.coprocessorSignerAddresses[i]] = false;
            delete $.coprocessors[$.coprocessorTxSenderAddresses[i]];
        }

        delete $.coprocessorTxSenderAddresses;
        delete $.coprocessorSignerAddresses;

        // Set the new coprocessors and their threshold
        _setCoprocessors(newCoprocessors, newCoprocessorThreshold);

        emit UpdateCoprocessors(newCoprocessors, newCoprocessorThreshold);
    }

    /**
     * @notice See {IGatewayConfig-updateCustodians}.
     */
    function updateCustodians(Custodian[] calldata newCustodians) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Remove the old custodians
        uint256 oldCustodianTxSenderAddressesLength = $.custodianTxSenderAddresses.length;
        for (uint256 i = 0; i < oldCustodianTxSenderAddressesLength; i++) {
            $.isCustodianTxSender[$.custodianTxSenderAddresses[i]] = false;
            $.isCustodianSigner[$.custodianSignerAddresses[i]] = false;
            delete $.custodians[$.custodianTxSenderAddresses[i]];
        }

        delete $.custodianTxSenderAddresses;
        delete $.custodianSignerAddresses;

        // Set the new custodians
        _setCustodians(newCustodians);

        emit UpdateCustodians(newCustodians);
    }

    /**
     * @notice See {IGatewayConfig-updateMpcThreshold}.
     */
    function updateMpcThreshold(uint256 newMpcThreshold) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        _setMpcThreshold($.currentKmsContextId, newMpcThreshold);
        emit UpdateMpcThreshold(newMpcThreshold);
    }

    /**
     * @notice See {IGatewayConfig-updatePublicDecryptionThreshold}.
     */
    function updatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        _setPublicDecryptionThreshold($.currentKmsContextId, newPublicDecryptionThreshold);
        emit UpdatePublicDecryptionThreshold(newPublicDecryptionThreshold);
    }

    /**
     * @notice See {IGatewayConfig-updateUserDecryptionThreshold}.
     */
    function updateUserDecryptionThreshold(uint256 newUserDecryptionThreshold) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        _setUserDecryptionThreshold($.currentKmsContextId, newUserDecryptionThreshold);
        emit UpdateUserDecryptionThreshold(newUserDecryptionThreshold);
    }

    /**
     * @notice See {IGatewayConfig-updateKmsGenThreshold}.
     */
    function updateKmsGenThreshold(uint256 newKmsGenThreshold) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        _setKmsGenThreshold($.currentKmsContextId, newKmsGenThreshold);
        emit UpdateKmsGenThreshold(newKmsGenThreshold);
    }

    /**
     * @notice See {IGatewayConfig-updateCoprocessorThreshold}.
     */
    function updateCoprocessorThreshold(uint256 newCoprocessorThreshold) external virtual onlyOwner {
        _setCoprocessorThreshold(newCoprocessorThreshold);
        emit UpdateCoprocessorThreshold(newCoprocessorThreshold);
    }

    /**
     * @notice See {IGatewayConfig-addHostChain}.
     */
    function addHostChain(HostChain calldata hostChain) external virtual onlyOwner {
        if (hostChain.chainId == 0) {
            revert InvalidNullChainId();
        }
        if (hostChain.chainId > MAX_CHAIN_ID) {
            revert ChainIdNotUint64(hostChain.chainId);
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if ($.isHostChainRegistered[hostChain.chainId]) {
            revert HostChainAlreadyRegistered(hostChain.chainId);
        }

        $.hostChains.push(hostChain);
        $.isHostChainRegistered[hostChain.chainId] = true;
        emit AddHostChain(hostChain);
    }

    /**
     * @notice See {IGatewayConfig-pauseAllGatewayContracts}.
     * Contracts that are technically pausable but do not provide any pausable functions are not
     * paused. If at least one of the contracts is already paused, the function will revert.
     */
    function pauseAllGatewayContracts() external virtual onlyPauser {
        DECRYPTION.pause();
        INPUT_VERIFICATION.pause();
        emit PauseAllGatewayContracts();
    }

    /**
     * @notice See {IGatewayConfig-unpauseAllGatewayContracts}.
     * If at least one of the contracts is not paused, the function will revert.
     */
    function unpauseAllGatewayContracts() external virtual onlyOwner {
        DECRYPTION.unpause();
        INPUT_VERIFICATION.unpause();
        emit UnpauseAllGatewayContracts();
    }

    /**
     * @notice See {IGatewayConfig-isKmsTxSender}.
     */
    function isKmsTxSender(address txSenderAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isKmsContextTxSender[$.currentKmsContextId][txSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-isKmsSigner}.
     */
    function isKmsSigner(address signerAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isKmsContextSigner[$.currentKmsContextId][signerAddress];
    }

    /**
     * @notice See {IGatewayConfig-isCoprocessorTxSender}.
     */
    function isCoprocessorTxSender(address txSenderAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isCoprocessorTxSender[txSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-isCoprocessorSigner}.
     */
    function isCoprocessorSigner(address signerAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isCoprocessorSigner[signerAddress];
    }

    /**
     * @notice See {IGatewayConfig-isCustodianTxSender}.
     */
    function isCustodianTxSender(address txSenderAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isCustodianTxSender[txSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-isCustodianSigner}.
     */
    function isCustodianSigner(address signerAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isCustodianSigner[signerAddress];
    }

    /**
     * @notice See {IGatewayConfig-isHostChainRegistered}.
     */
    function isHostChainRegistered(uint256 chainId) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isHostChainRegistered[chainId];
    }

    /**
     * @notice See {IGatewayConfig-getProtocolMetadata}.
     */
    function getProtocolMetadata() external view virtual returns (ProtocolMetadata memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.protocolMetadata;
    }

    /**
     * @notice See {IGatewayConfig-getMpcThreshold}.
     */
    function getMpcThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextMpcThreshold[$.currentKmsContextId];
    }

    /**
     * @notice See {IGatewayConfig-getKmsGenThreshold}.
     */
    function getKmsGenThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextKmsGenThreshold[$.currentKmsContextId];
    }

    /**
     * @notice See {IGatewayConfig-getCoprocessorMajorityThreshold}.
     */
    function getCoprocessorMajorityThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessorThreshold;
    }

    /**
     * @notice See {IGatewayConfig-getKmsNode}.
     */
    function getKmsNode(address kmsTxSenderAddress) external view virtual returns (KmsNode memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextNodes[$.currentKmsContextId][kmsTxSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-getKmsTxSenders}.
     */
    function getKmsTxSenders() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextTxSenderAddresses[$.currentKmsContextId];
    }

    /**
     * @notice See {IGatewayConfig-getKmsSigners}.
     */
    function getKmsSigners() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextSignerAddresses[$.currentKmsContextId];
    }

    /**
     * @notice See {IGatewayConfig-getCoprocessor}.
     */
    function getCoprocessor(address coprocessorTxSenderAddress) external view virtual returns (Coprocessor memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessors[coprocessorTxSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-getCoprocessorTxSenders}.
     */
    function getCoprocessorTxSenders() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessorTxSenderAddresses;
    }

    /**
     * @notice See {IGatewayConfig-getCoprocessorSigners}.
     */
    function getCoprocessorSigners() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessorSignerAddresses;
    }

    /**
     * @notice See {IGatewayConfig-getHostChain}.
     */
    function getHostChain(uint256 index) external view virtual returns (HostChain memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.hostChains[index];
    }

    /**
     * @notice See {IGatewayConfig-getHostChains}.
     */
    function getHostChains() external view virtual returns (HostChain[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.hostChains;
    }

    /**
     * @notice See {IGatewayConfig-getCustodian}.
     */
    function getCustodian(address custodianTxSenderAddress) external view virtual returns (Custodian memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.custodians[custodianTxSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-getCustodianTxSenders}.
     */
    function getCustodianTxSenders() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.custodianTxSenderAddresses;
    }

    /**
     * @notice See {IGatewayConfig-getCustodianSigners}.
     */
    function getCustodianSigners() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.custodianSignerAddresses;
    }

    /**
     * @notice See {IGatewayConfig-isKmsContextTxSender}.
     */
    function isKmsContextTxSender(uint256 contextId, address txSenderAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isKmsContextTxSender[contextId][txSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-isKmsContextSigner}.
     */
    function isKmsContextSigner(uint256 contextId, address signerAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isKmsContextSigner[contextId][signerAddress];
    }

    /**
     * @notice See {IGatewayConfig-getKmsContextNode}.
     */
    function getKmsContextNode(
        uint256 contextId,
        address kmsTxSenderAddress
    ) external view virtual returns (KmsNode memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextNodes[contextId][kmsTxSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-getKmsContextTxSenders}.
     */
    function getKmsContextTxSenders(uint256 contextId) external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextTxSenderAddresses[contextId];
    }

    /**
     * @notice See {IGatewayConfig-getKmsContextSigners}.
     */
    function getKmsContextSigners(uint256 contextId) external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextSignerAddresses[contextId];
    }

    /**
     * @notice See {IGatewayConfig-getCurrentKmsContextId}.
     */
    function getCurrentKmsContextId() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.currentKmsContextId;
    }

    /**
     * @notice See {IGatewayConfig-getKmsContextPublicDecryptionThreshold}.
     */
    function getKmsContextPublicDecryptionThreshold(uint256 contextId) external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextPublicDecryptionThreshold[contextId];
    }

    /**
     * @notice See {IGatewayConfig-getKmsContextUserDecryptionThreshold}.
     */
    function getKmsContextUserDecryptionThreshold(uint256 contextId) external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsContextUserDecryptionThreshold[contextId];
    }

    /**
     * @notice See {IGatewayConfig-getVersion}.
     */
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
     * @notice Sets the KMS context: nodes and all thresholds for a given context ID.
     * @param contextId The context ID.
     * @param newKmsNodes The new KMS nodes.
     * @param newMpcThreshold The new MPC threshold for this context.
     * @param newPublicDecryptionThreshold The new public decryption threshold for this context.
     * @param newUserDecryptionThreshold The new user decryption threshold for this context.
     * @param newKmsGenThreshold The new key and CRS generation threshold for this context.
     */
    function _setKmsContext(
        uint256 contextId,
        KmsNode[] calldata newKmsNodes,
        uint256 newMpcThreshold,
        uint256 newPublicDecryptionThreshold,
        uint256 newUserDecryptionThreshold,
        uint256 newKmsGenThreshold
    ) internal virtual {
        if (newKmsNodes.length == 0) {
            revert EmptyKmsNodes();
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Register the new KMS nodes for this context
        for (uint256 i = 0; i < newKmsNodes.length; i++) {
            address newKmsTxSenderAddress = newKmsNodes[i].txSenderAddress;
            address newKmsSignerAddress = newKmsNodes[i].signerAddress;

            // Check for KMS transaction sender and signer duplicates within this context
            if ($.isKmsContextTxSender[contextId][newKmsTxSenderAddress]) {
                revert KmsTxSenderAlreadyRegistered(newKmsTxSenderAddress);
            }
            if ($.isKmsContextSigner[contextId][newKmsSignerAddress]) {
                revert KmsSignerAlreadyRegistered(newKmsSignerAddress);
            }

            // Register transaction sender
            $.isKmsContextTxSender[contextId][newKmsTxSenderAddress] = true;
            $.kmsContextTxSenderAddresses[contextId].push(newKmsTxSenderAddress);

            // Register KMS node
            $.kmsContextNodes[contextId][newKmsTxSenderAddress] = newKmsNodes[i];

            // Register signer
            $.isKmsContextSigner[contextId][newKmsSignerAddress] = true;
            $.kmsContextSignerAddresses[contextId].push(newKmsSignerAddress);
        }

        // Setting the thresholds should be done after the KMS nodes have been registered
        // as the functions validate against the context's node count.
        _setMpcThreshold(contextId, newMpcThreshold);
        _setPublicDecryptionThreshold(contextId, newPublicDecryptionThreshold);
        _setUserDecryptionThreshold(contextId, newUserDecryptionThreshold);
        _setKmsGenThreshold(contextId, newKmsGenThreshold);
    }

    /**
     * @notice Sets the coprocessors and their threshold.
     * @param newCoprocessors The new coprocessors.
     * @param newCoprocessorThreshold The new coprocessor threshold.
     */
    function _setCoprocessors(
        Coprocessor[] calldata newCoprocessors,
        uint256 newCoprocessorThreshold
    ) internal virtual {
        if (newCoprocessors.length == 0) {
            revert EmptyCoprocessors();
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Register the new coprocessors
        for (uint256 i = 0; i < newCoprocessors.length; i++) {
            address newCoprocessorTxSenderAddress = newCoprocessors[i].txSenderAddress;
            address newCoprocessorSignerAddress = newCoprocessors[i].signerAddress;

            // Check for coprocessor transaction sender and signer duplicates
            if ($.isCoprocessorTxSender[newCoprocessorTxSenderAddress]) {
                revert CoprocessorTxSenderAlreadyRegistered(newCoprocessorTxSenderAddress);
            }
            if ($.isCoprocessorSigner[newCoprocessorSignerAddress]) {
                revert CoprocessorSignerAlreadyRegistered(newCoprocessorSignerAddress);
            }

            // Register coprocessor transaction sender
            $.isCoprocessorTxSender[newCoprocessorTxSenderAddress] = true;
            $.coprocessorTxSenderAddresses.push(newCoprocessorTxSenderAddress);

            // Register coprocessor
            $.coprocessors[newCoprocessorTxSenderAddress] = newCoprocessors[i];

            // Register coprocessor signer
            $.isCoprocessorSigner[newCoprocessorSignerAddress] = true;
            $.coprocessorSignerAddresses.push(newCoprocessorSignerAddress);
        }

        // Setting the coprocessor threshold should be done after the coprocessors have been
        // registered as the functions reading the `coprocessorSignerAddresses` array.
        _setCoprocessorThreshold(newCoprocessorThreshold);
    }

    /**
     * @notice Sets the custodians.
     * @param newCustodians The new custodians.
     */
    function _setCustodians(Custodian[] calldata newCustodians) internal virtual {
        if (newCustodians.length == 0) {
            revert EmptyCustodians();
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Register the new custodians
        for (uint256 i = 0; i < newCustodians.length; i++) {
            address newCustodianTxSenderAddress = newCustodians[i].txSenderAddress;
            address newCustodianSignerAddress = newCustodians[i].signerAddress;

            // Check for custodian transaction sender and signer duplicates
            if ($.isCustodianTxSender[newCustodianTxSenderAddress]) {
                revert CustodianTxSenderAlreadyRegistered(newCustodianTxSenderAddress);
            }
            if ($.isCustodianSigner[newCustodianSignerAddress]) {
                revert CustodianSignerAlreadyRegistered(newCustodianSignerAddress);
            }

            // Register custodian transaction sender
            $.isCustodianTxSender[newCustodianTxSenderAddress] = true;
            $.custodianTxSenderAddresses.push(newCustodianTxSenderAddress);

            // Register custodian
            $.custodians[newCustodianTxSenderAddress] = newCustodians[i];

            // Register custodian signer
            $.isCustodianSigner[newCustodianSignerAddress] = true;
            $.custodianSignerAddresses.push(newCustodianSignerAddress);
        }
    }

    /**
     * @notice Sets the MPC threshold for a given context.
     * @param contextId The context ID.
     * @param newMpcThreshold The new MPC threshold.
     */
    function _setMpcThreshold(uint256 contextId, uint256 newMpcThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsContextSignerAddresses[contextId].length;

        // Check that the MPC threshold `t` is valid. It must verify:
        // - `t >= 0` : it is already a uint256 so this is always true
        // - `t < n` : it should be strictly less than the number of registered KMS nodes
        if (newMpcThreshold >= nKmsNodes) {
            revert InvalidHighMpcThreshold(newMpcThreshold, nKmsNodes);
        }

        $.kmsContextMpcThreshold[contextId] = newMpcThreshold;
    }

    /**
     * @notice Sets the public decryption threshold for a given context.
     * @param contextId The context ID.
     * @param newPublicDecryptionThreshold The new public decryption threshold.
     */
    function _setPublicDecryptionThreshold(uint256 contextId, uint256 newPublicDecryptionThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsContextSignerAddresses[contextId].length;

        // Check that the public decryption threshold `t` is valid. It must verify:
        // - `t >= 1` : the public decryption consensus should require at least one vote
        // - `t <= n` : it should be less than or equal to the number of registered KMS nodes
        if (newPublicDecryptionThreshold == 0) {
            revert InvalidNullPublicDecryptionThreshold();
        }
        if (newPublicDecryptionThreshold > nKmsNodes) {
            revert InvalidHighPublicDecryptionThreshold(newPublicDecryptionThreshold, nKmsNodes);
        }

        $.kmsContextPublicDecryptionThreshold[contextId] = newPublicDecryptionThreshold;
    }

    /**
     * @notice Sets the user decryption threshold for a given context.
     * @param contextId The context ID.
     * @param newUserDecryptionThreshold The new user decryption threshold.
     */
    function _setUserDecryptionThreshold(uint256 contextId, uint256 newUserDecryptionThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsContextSignerAddresses[contextId].length;

        // Check that the user decryption threshold `t` is valid. It must verify:
        // - `t >= 1` : the user decryption consensus should require at least one vote
        // - `t <= n` : it should be less than or equal to the number of registered KMS nodes
        if (newUserDecryptionThreshold == 0) {
            revert InvalidNullUserDecryptionThreshold();
        }
        if (newUserDecryptionThreshold > nKmsNodes) {
            revert InvalidHighUserDecryptionThreshold(newUserDecryptionThreshold, nKmsNodes);
        }

        $.kmsContextUserDecryptionThreshold[contextId] = newUserDecryptionThreshold;
    }

    /**
     * @notice Sets the coprocessor threshold.
     * @param newCoprocessorThreshold The new coprocessor threshold.
     */
    function _setCoprocessorThreshold(uint256 newCoprocessorThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nCoprocessors = $.coprocessorSignerAddresses.length;

        // Check that the coprocessor threshold `t` is valid. It must verify:
        // - `t >= 1` : the coprocessor consensus should require at least one vote
        // - `t <= n` : it should be less than or equal to the number of registered coprocessors
        if (newCoprocessorThreshold == 0) {
            revert InvalidNullCoprocessorThreshold();
        }
        if (newCoprocessorThreshold > nCoprocessors) {
            revert InvalidHighCoprocessorThreshold(newCoprocessorThreshold, nCoprocessors);
        }

        $.coprocessorThreshold = newCoprocessorThreshold;
    }

    /**
     * @notice Sets the key and CRS generation threshold for a given context.
     * @param contextId The context ID.
     * @param newKmsGenThreshold The new key and CRS generation threshold.
     */
    function _setKmsGenThreshold(uint256 contextId, uint256 newKmsGenThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsContextSignerAddresses[contextId].length;

        // Check that the key and CRS generation threshold `t` is valid. It must verify:
        // - `t >= 1` : the key and CRS generation consensus should require at least one vote
        // - `t <= n` : it should be less than or equal to the number of registered KMS nodes
        if (newKmsGenThreshold == 0) {
            revert InvalidNullKmsGenThreshold();
        }
        if (newKmsGenThreshold > nKmsNodes) {
            revert InvalidHighKmsGenThreshold(newKmsGenThreshold, nKmsNodes);
        }

        $.kmsContextKmsGenThreshold[contextId] = newKmsGenThreshold;
    }

    /**
     * @notice Checks if the sender is authorized to upgrade the contract and reverts otherwise.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /**
     * @notice Returns the GatewayConfig storage location.
     * @dev Note that this function is internal but not virtual: derived contracts should be able to
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
