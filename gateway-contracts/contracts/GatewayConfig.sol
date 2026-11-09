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
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract GatewayConfig is IGatewayConfig, Ownable2StepUpgradeable, UUPSUpgradeableEmptyProxy {
    /**
     * @notice The maximum chain ID.
     */
    uint256 internal constant MAX_CHAIN_ID = type(uint64).max;

    /**
     * @notice Upper bound on the KMS committee size and on every per-context threshold.
     * @dev Mirrors `ProtocolConfig.MAX_KMS_SIGNERS` on the host side. Driven by the proof format
     *      consumed in `KMSVerifier.verifyDecryptionEIP712KMSSignatures`, which encodes the
     *      signature count in a single byte (`uint8(decryptionProof[0])`). Both registries cap
     *      at registration time so a misconfiguration is rejected loudly on either side.
     */
    uint256 private constant MAX_KMS_SIGNERS = type(uint8).max;

    /**
     * @notice Upper bound on the coprocessor set size.
     * @dev Mirrors `ProtocolConfig.MAX_COPROCESSORS`. The input proof encodes the signature count in
     *      a single byte.
     */
    uint256 private constant MAX_COPROCESSORS = type(uint8).max;

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
    uint256 private constant MINOR_VERSION = 8;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     * This constant does not represent the number of time a specific contract have been upgraded,
     * as a contract deployed from version VX will have a REINITIALIZER_VERSION > 2.
     */
    uint64 private constant REINITIALIZER_VERSION = 10;

    /**
     * @notice The addresses of all gateway contracts
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
        /// @dev Deprecated. Use `isKmsTxSenderForContext` instead.
        mapping(address kmsTxSenderAddress => bool isTxSender) isKmsTxSender;
        /// @dev Deprecated. Use `isKmsSignerForContext` instead.
        mapping(address kmsSignerAddress => bool isSigner) isKmsSigner;
        /// @dev Deprecated. Use `kmsNodesForContext` instead.
        mapping(address kmsTxSenderAddress => KmsNode kmsNode) kmsNodes;
        /// @dev Deprecated. Use `kmsTxSenderAddressesForContext` instead.
        address[] kmsTxSenderAddresses;
        /// @dev Deprecated. Use `kmsSignerAddressesForContext` instead.
        address[] kmsSignerAddresses;
        /// @dev Deprecated. Use `mpcThresholdForContext` instead.
        uint256 mpcThreshold;
        /// @dev Deprecated. Use `publicDecryptionThresholdForContext` instead.
        uint256 publicDecryptionThreshold;
        /// @dev Deprecated. Use `userDecryptionThresholdForContext` instead.
        uint256 userDecryptionThreshold;
        /// @dev Deprecated. Use `isCoprocessorTxSenderForContext` instead.
        mapping(address coprocessorTxSenderAddress => bool isTxSender) isCoprocessorTxSender;
        /// @dev Deprecated. Use `isCoprocessorSignerForContext` instead.
        mapping(address coprocessorSignerAddress => bool isSigner) isCoprocessorSigner;
        /// @dev Deprecated. Use `coprocessorsForContext` instead.
        mapping(address coprocessorTxSenderAddress => Coprocessor coprocessor) coprocessors;
        /// @dev Deprecated. Use `coprocessorTxSenderAddressesForContext` instead.
        address[] coprocessorTxSenderAddresses;
        /// @dev Deprecated. Use `coprocessorSignerAddressesForContext` instead.
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
        /// @dev Deprecated. Use `kmsGenThresholdForContext` instead.
        uint256 kmsGenThreshold;
        /// @dev Deprecated. Use `coprocessorThresholdForContext` instead.
        uint256 coprocessorThreshold;
        // ----------------------------------------------------------------------------------------------
        // KMS context state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The current KMS context ID
        uint256 currentKmsContextId;
        /// @notice The KMS nodes' transaction sender addresses per context
        mapping(uint256 contextId => mapping(address kmsTxSenderAddress => bool isTxSender)) isKmsTxSenderForContext;
        /// @notice The KMS nodes' signer addresses per context
        mapping(uint256 contextId => mapping(address kmsSignerAddress => bool isSigner)) isKmsSignerForContext;
        /// @notice The KMS nodes' metadata per context
        mapping(uint256 contextId => mapping(address kmsTxSenderAddress => KmsNode kmsNode)) kmsNodesForContext;
        /// @notice The KMS nodes' transaction sender address list per context
        mapping(uint256 contextId => address[]) kmsTxSenderAddressesForContext;
        /// @notice The KMS nodes' signer address list per context
        mapping(uint256 contextId => address[]) kmsSignerAddressesForContext;
        /// @notice The public decryption threshold per context
        mapping(uint256 contextId => uint256) publicDecryptionThresholdForContext;
        /// @notice The user decryption threshold per context
        mapping(uint256 contextId => uint256) userDecryptionThresholdForContext;
        /// @notice The MPC threshold per context
        mapping(uint256 contextId => uint256) mpcThresholdForContext;
        /// @notice The key and CRS generation threshold per context
        mapping(uint256 contextId => uint256) kmsGenThresholdForContext;
        /// @notice Whether a KMS context has been destroyed.
        /// @dev A destroyed context cannot satisfy any decryption verification: its signers and
        ///      tx senders are reported as not registered, and its KMS-node lookups return zero.
        mapping(uint256 contextId => bool isDestroyed) destroyedKmsContexts;
        /// @notice Whether a registered host chain has been disabled.
        mapping(uint256 chainId => bool isDisabled) disabledHostChains;
        /// @dev Deprecated. The priority coprocessor feature was removed, so coprocessor consensus is
        ///      always threshold-based. Kept to preserve the storage layout, and zeroed by
        ///      `reinitializeV9`.
        address priorityCoprocessorTxSender;
        // ----------------------------------------------------------------------------------------------
        // Coprocessor context state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The current coprocessor context ID
        uint256 currentCoprocessorContextId;
        /// @notice The coprocessors' transaction sender addresses per context
        mapping(uint256 contextId => mapping(address coprocessorTxSenderAddress => bool isTxSender)) isCoprocessorTxSenderForContext;
        /// @notice The coprocessors' signer addresses per context
        mapping(uint256 contextId => mapping(address coprocessorSignerAddress => bool isSigner)) isCoprocessorSignerForContext;
        /// @notice The coprocessors' metadata per context
        mapping(uint256 contextId => mapping(address coprocessorTxSenderAddress => Coprocessor coprocessor)) coprocessorsForContext;
        /// @notice The coprocessors' transaction sender address list per context
        mapping(uint256 contextId => address[]) coprocessorTxSenderAddressesForContext;
        /// @notice The coprocessors' signer address list per context
        mapping(uint256 contextId => address[]) coprocessorSignerAddressesForContext;
        /// @notice The coprocessor majority threshold per context
        mapping(uint256 contextId => uint256) coprocessorThresholdForContext;
        /// @notice Whether a coprocessor context has been destroyed.
        mapping(uint256 contextId => bool isDestroyed) destroyedCoprocessorContexts;
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
     * @param initialKmsContextId ID of the first KMS context
     * @param initialCoprocessorContextId ID of the first coprocessor context
     * @param initialMetadata Metadata of the protocol
     * @param initialThresholds The operator thresholds
     * @param initialKmsNodes List of KMS nodes
     * @param initialCoprocessors List of coprocessors
     * @param initialCustodians List of custodians
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        uint256 initialKmsContextId,
        uint256 initialCoprocessorContextId,
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
        if (initialCoprocessorContextId == 0) {
            revert InvalidNullCoprocessorContextId();
        }

        // Using scoped block to avoid stack depth error
        {
            GatewayConfigStorage storage $ = _getGatewayConfigStorage();
            $.protocolMetadata = initialMetadata;
            $.currentKmsContextId = initialKmsContextId;
            $.currentCoprocessorContextId = initialCoprocessorContextId;
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

        // Initialize the coprocessor context with the coprocessors and their threshold
        _setCoprocessorContext(
            initialCoprocessorContextId,
            initialCoprocessors,
            initialThresholds.coprocessorThreshold
        );

        // Set the custodians
        _setCustodians(initialCustodians);

        emit InitializeGatewayConfig(
            initialKmsContextId,
            initialCoprocessorContextId,
            initialMetadata,
            initialThresholds,
            initialKmsNodes,
            initialCoprocessors,
            initialCustodians
        );
    }

    /**
     * @notice Re-initializes the contract from V8.
     * @dev Clears the deprecated priority coprocessor slot, so the upgrade itself makes coprocessor
     *      consensus threshold-based without depending on a prior ops call.
     * @dev UPGRADE ORDER: batch the `upgradeToAndCall` calls of `GatewayConfig`, `CiphertextCommits` and
     *      `InputVerification` in one proposal. This implementation drops `getPriorityCoprocessorTxSender()`,
     *      which the deployed `CiphertextCommits` and `InputVerification` call on every response, and the new
     *      `InputVerification` reads coprocessor contexts, which only this implementation serves.
     * @dev Every host-chain `InputVerifier` must already accept the full gateway coprocessor signer set
     *      at the gateway threshold. Under priority mode the gateway emitted a single signature, so a
     *      host still pinned to one signer would reject the bundles this upgrade makes unconditional.
     * @dev Requests the former priority sender already responded to, without reaching consensus, become
     *      finalizable again: their count resumes against the coprocessor threshold.
     * @dev Stores the registered coprocessor set and threshold as the first coprocessor context.
     * @param initialCoprocessorContextId ID of the coprocessor context that `ProtocolConfig` defines with this
     *        same set.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV9(uint256 initialCoprocessorContextId) public virtual reinitializer(REINITIALIZER_VERSION) {
        if (initialCoprocessorContextId == 0) {
            revert InvalidNullCoprocessorContextId();
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        delete $.priorityCoprocessorTxSender;

        uint256 contextId = initialCoprocessorContextId;
        address[] storage txSenderAddresses = $.coprocessorTxSenderAddresses;
        uint256 coprocessorCount = txSenderAddresses.length;
        for (uint256 i = 0; i < coprocessorCount; i++) {
            Coprocessor storage coprocessor = $.coprocessors[txSenderAddresses[i]];
            $.isCoprocessorTxSenderForContext[contextId][coprocessor.txSenderAddress] = true;
            $.isCoprocessorSignerForContext[contextId][coprocessor.signerAddress] = true;
            $.coprocessorsForContext[contextId][coprocessor.txSenderAddress] = coprocessor;
            $.coprocessorTxSenderAddressesForContext[contextId].push(coprocessor.txSenderAddress);
            $.coprocessorSignerAddressesForContext[contextId].push(coprocessor.signerAddress);
        }
        $.coprocessorThresholdForContext[contextId] = $.coprocessorThreshold;
        $.currentCoprocessorContextId = contextId;
    }

    /**
     * @notice See {IGatewayConfig-isPauser}.
     */
    function isPauser(address account) public view virtual returns (bool) {
        return PAUSER_SET.isPauser(account);
    }

    /**
     * @notice See {IGatewayConfig-updateKmsContext}.
     * @dev WARNING: This mutates KMS context state read by `Decryption` without enforcing
     *      an on-chain pause. Operators must follow the runbook to avoid racing in-flight
     *      decryption requests when rotating contexts.
     */
    function updateKmsContext(
        uint256 newContextId,
        KmsNode[] calldata newKmsNodes,
        uint256 newMpcThreshold,
        uint256 newPublicDecryptionThreshold,
        uint256 newUserDecryptionThreshold,
        uint256 newKmsGenThreshold
    ) public virtual onlyOwner {
        if (newContextId == 0) {
            revert InvalidNullKmsContextId();
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Validate newContextId is strictly greater than the current one
        if (newContextId <= $.currentKmsContextId) {
            revert KmsContextAlreadyRegistered(newContextId, $.currentKmsContextId);
        }

        // Set the new context-indexed KMS nodes and all thresholds
        _setKmsContext(
            newContextId,
            newKmsNodes,
            newMpcThreshold,
            newPublicDecryptionThreshold,
            newUserDecryptionThreshold,
            newKmsGenThreshold
        );

        // Update the current KMS context ID
        $.currentKmsContextId = newContextId;

        emit UpdateKmsContext(
            newContextId,
            newKmsNodes,
            newMpcThreshold,
            newPublicDecryptionThreshold,
            newUserDecryptionThreshold,
            newKmsGenThreshold
        );
    }

    /**
     * @notice See {IGatewayConfig-destroyKmsContext}.
     * @dev WARNING: This mutates KMS context state read by `Decryption` without enforcing
     *      an on-chain pause. Operators must follow the runbook to avoid racing in-flight
     *      decryption requests when destroying contexts.
     */
    function destroyKmsContext(uint256 kmsContextId) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        if (kmsContextId == $.currentKmsContextId) {
            revert CurrentKmsContextCannotBeDestroyed(kmsContextId);
        }
        // Context must exist and not be already destroyed.
        _requireValidKmsContext(kmsContextId);

        $.destroyedKmsContexts[kmsContextId] = true;
        emit DestroyKmsContext(kmsContextId);
    }

    /**
     * @notice See {IGatewayConfig-updateCoprocessors}.
     * @dev Applies live: new input verification requests use the new context, and requests in progress
     *      complete under the context they recorded.
     */
    function updateCoprocessors(
        uint256 newContextId,
        Coprocessor[] calldata newCoprocessors,
        uint256 newCoprocessorThreshold
    ) external virtual onlyOwner {
        if (newContextId == 0) {
            revert InvalidNullCoprocessorContextId();
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Validate newContextId is strictly greater than the current one
        if (newContextId <= $.currentCoprocessorContextId) {
            revert CoprocessorContextAlreadyRegistered(newContextId, $.currentCoprocessorContextId);
        }

        // Set the new context-indexed coprocessors and threshold
        _setCoprocessorContext(newContextId, newCoprocessors, newCoprocessorThreshold);

        // Update the current coprocessor context ID
        $.currentCoprocessorContextId = newContextId;

        emit UpdateCoprocessors(newContextId, newCoprocessors, newCoprocessorThreshold);
    }

    /**
     * @notice See {IGatewayConfig-destroyCoprocessorContext}.
     */
    function destroyCoprocessorContext(uint256 coprocessorContextId) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        if (coprocessorContextId == $.currentCoprocessorContextId) {
            revert CurrentCoprocessorContextCannotBeDestroyed(coprocessorContextId);
        }
        // Context must exist and not be already destroyed.
        _requireValidCoprocessorContext(coprocessorContextId);

        $.destroyedCoprocessorContexts[coprocessorContextId] = true;
        emit DestroyCoprocessorContext(coprocessorContextId);
    }

    /**
     * @notice See {IGatewayConfig-updateCustodians}.
     */
    function updateCustodians(Custodian[] calldata newCustodians) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Remove the old custodians
        uint256 oldCustodianTxSenderAddressesLength = $.custodianTxSenderAddresses.length;
        for (uint256 i = 0; i < oldCustodianTxSenderAddressesLength; i++) {
            delete $.isCustodianTxSender[$.custodianTxSenderAddresses[i]];
            delete $.isCustodianSigner[$.custodianSignerAddresses[i]];
            delete $.custodians[$.custodianTxSenderAddresses[i]];
        }

        delete $.custodianTxSenderAddresses;
        delete $.custodianSignerAddresses;

        // Set the new custodians
        _setCustodians(newCustodians);

        emit UpdateCustodians(newCustodians);
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
        emit AddHostChain(hostChain.chainId, hostChain);
    }

    /**
     * @notice See {IGatewayConfig-disableHostChain}.
     */
    function disableHostChain(uint256 chainId) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$.isHostChainRegistered[chainId]) {
            revert HostChainNotRegistered(chainId);
        }
        if ($.disabledHostChains[chainId]) {
            revert HostChainAlreadyDisabled(chainId);
        }
        $.disabledHostChains[chainId] = true;
        emit DisableHostChain(chainId);
    }

    /**
     * @notice See {IGatewayConfig-enableHostChain}.
     */
    function enableHostChain(uint256 chainId) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$.isHostChainRegistered[chainId]) {
            revert HostChainNotRegistered(chainId);
        }
        if (!$.disabledHostChains[chainId]) {
            revert HostChainAlreadyEnabled(chainId);
        }
        $.disabledHostChains[chainId] = false;
        emit EnableHostChain(chainId);
    }

    /**
     * @notice See {IGatewayConfig-removeHostChain}.
     */
    function removeHostChain(uint256 chainId) external virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$.isHostChainRegistered[chainId]) {
            revert HostChainNotRegistered(chainId);
        }
        // The chain must be disabled first so the action is deliberate and cannot pull a chain
        // out from under in-flight requests.
        if (!$.disabledHostChains[chainId]) {
            revert HostChainNotDisabled(chainId);
        }

        // Swap-and-pop the entry out of the `hostChains` array.
        uint256 len = $.hostChains.length;
        for (uint256 i = 0; i < len; i++) {
            if ($.hostChains[i].chainId == chainId) {
                if (i != len - 1) {
                    $.hostChains[i] = $.hostChains[len - 1];
                }
                $.hostChains.pop();
                break;
            }
        }

        // Reset the registration state so a fresh `addHostChain(chainId)` later starts from a clean slate.
        delete $.isHostChainRegistered[chainId];
        delete $.disabledHostChains[chainId];

        emit RemoveHostChain(chainId);
    }

    /**
     * @notice See {IGatewayConfig-pauseAllGatewayContracts}.
     * Contracts that are technically pausable but do not provide any pausable functions are not
     * paused. If all of the contracts are already paused, the function will revert.
     */
    function pauseAllGatewayContracts() external virtual onlyPauser {
        bool isDecryptionPaused = DECRYPTION.paused();
        bool isInputVerificationPaused = INPUT_VERIFICATION.paused();

        if (isDecryptionPaused && isInputVerificationPaused) {
            revert AllGatewayContractsAlreadyPaused();
        }

        if (!isDecryptionPaused) {
            DECRYPTION.pause();
        }

        if (!isInputVerificationPaused) {
            INPUT_VERIFICATION.pause();
        }

        emit PauseAllGatewayContracts();
    }

    /**
     * @notice See {IGatewayConfig-unpauseAllGatewayContracts}.
     * If none of the contracts are paused, the function will revert.
     */
    function unpauseAllGatewayContracts() external virtual onlyOwner {
        bool isDecryptionPaused = DECRYPTION.paused();
        bool isInputVerificationPaused = INPUT_VERIFICATION.paused();

        if (!isDecryptionPaused && !isInputVerificationPaused) {
            revert AllGatewayContractsAlreadyUnpaused();
        }

        if (isDecryptionPaused) {
            DECRYPTION.unpause();
        }

        if (isInputVerificationPaused) {
            INPUT_VERIFICATION.unpause();
        }

        emit UnpauseAllGatewayContracts();
    }

    /**
     * @notice See {IGatewayConfig-isKmsTxSender}.
     */
    function isKmsTxSender(address txSenderAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isKmsTxSenderForContext[$.currentKmsContextId][txSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-isKmsSigner}.
     */
    function isKmsSigner(address signerAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isKmsSignerForContext[$.currentKmsContextId][signerAddress];
    }

    /**
     * @notice See {IGatewayConfig-isCoprocessorTxSender}.
     */
    function isCoprocessorTxSender(address txSenderAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isCoprocessorTxSenderForContext[$.currentCoprocessorContextId][txSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-isCoprocessorSigner}.
     */
    function isCoprocessorSigner(address signerAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isCoprocessorSignerForContext[$.currentCoprocessorContextId][signerAddress];
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
     * @notice See {IGatewayConfig-isHostChainDisabled}.
     */
    function isHostChainDisabled(uint256 chainId) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.disabledHostChains[chainId];
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
        return $.mpcThresholdForContext[$.currentKmsContextId];
    }

    /**
     * @notice See {IGatewayConfig-getKmsGenThreshold}.
     */
    function getKmsGenThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsGenThresholdForContext[$.currentKmsContextId];
    }

    /**
     * @notice See {IGatewayConfig-getCoprocessorMajorityThreshold}.
     */
    function getCoprocessorMajorityThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessorThresholdForContext[$.currentCoprocessorContextId];
    }

    /**
     * @notice See {IGatewayConfig-getKmsNode}.
     */
    function getKmsNode(address kmsTxSenderAddress) external view virtual returns (KmsNode memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsNodesForContext[$.currentKmsContextId][kmsTxSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-getKmsTxSenders}.
     */
    function getKmsTxSenders() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsTxSenderAddressesForContext[$.currentKmsContextId];
    }

    /**
     * @notice See {IGatewayConfig-getKmsSigners}.
     */
    function getKmsSigners() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsSignerAddressesForContext[$.currentKmsContextId];
    }

    /**
     * @notice See {IGatewayConfig-getCoprocessor}.
     */
    function getCoprocessor(address coprocessorTxSenderAddress) external view virtual returns (Coprocessor memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessorsForContext[$.currentCoprocessorContextId][coprocessorTxSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-getCoprocessorTxSenders}.
     */
    function getCoprocessorTxSenders() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessorTxSenderAddressesForContext[$.currentCoprocessorContextId];
    }

    /**
     * @notice See {IGatewayConfig-getCoprocessorSigners}.
     */
    function getCoprocessorSigners() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessorSignerAddressesForContext[$.currentCoprocessorContextId];
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
     * @notice See {IGatewayConfig-isKmsTxSenderForContext}.
     * @dev Reverts with `InvalidKmsContext` if the context is unknown or destroyed, so the
     *      on-chain verification flow rejects signatures referencing them.
     */
    function isKmsTxSenderForContext(uint256 contextId, address txSenderAddress) external view virtual returns (bool) {
        _requireValidKmsContext(contextId);
        return _getGatewayConfigStorage().isKmsTxSenderForContext[contextId][txSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-isKmsSignerForContext}.
     * @dev Reverts with `InvalidKmsContext` if the context is unknown or destroyed, so the
     *      on-chain verification flow rejects signatures referencing them.
     */
    function isKmsSignerForContext(uint256 contextId, address signerAddress) external view virtual returns (bool) {
        _requireValidKmsContext(contextId);
        return _getGatewayConfigStorage().isKmsSignerForContext[contextId][signerAddress];
    }

    /**
     * @notice See {IGatewayConfig-getKmsNodeForContext}.
     * @dev Reverts with `InvalidKmsContext` if the context is unknown or destroyed.
     */
    function getKmsNodeForContext(
        uint256 contextId,
        address kmsTxSenderAddress
    ) external view virtual returns (KmsNode memory) {
        _requireValidKmsContext(contextId);
        return _getGatewayConfigStorage().kmsNodesForContext[contextId][kmsTxSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-getKmsTxSendersForContext}.
     * @dev Reverts with `InvalidKmsContext` if the context is unknown or destroyed.
     */
    function getKmsTxSendersForContext(uint256 contextId) external view virtual returns (address[] memory) {
        _requireValidKmsContext(contextId);
        return _getGatewayConfigStorage().kmsTxSenderAddressesForContext[contextId];
    }

    /**
     * @notice See {IGatewayConfig-getKmsSignersForContext}.
     * @dev Reverts with `InvalidKmsContext` if the context is unknown or destroyed.
     */
    function getKmsSignersForContext(uint256 contextId) external view virtual returns (address[] memory) {
        _requireValidKmsContext(contextId);
        return _getGatewayConfigStorage().kmsSignerAddressesForContext[contextId];
    }

    /**
     * @notice See {IGatewayConfig-getCurrentKmsContextId}.
     */
    function getCurrentKmsContextId() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.currentKmsContextId;
    }

    /**
     * @notice See {IGatewayConfig-isValidKmsContext}.
     */
    function isValidKmsContext(uint256 contextId) external view virtual returns (bool) {
        return _isValidKmsContext(contextId);
    }

    /**
     * @notice See {IGatewayConfig-getPublicDecryptionThresholdForContext}.
     * @dev Reverts with `InvalidKmsContext` if the context is unknown or destroyed.
     */
    function getPublicDecryptionThresholdForContext(uint256 contextId) external view virtual returns (uint256) {
        _requireValidKmsContext(contextId);
        return _getGatewayConfigStorage().publicDecryptionThresholdForContext[contextId];
    }

    /**
     * @notice See {IGatewayConfig-getUserDecryptionThresholdForContext}.
     * @dev Reverts with `InvalidKmsContext` if the context is unknown or destroyed.
     */
    function getUserDecryptionThresholdForContext(uint256 contextId) external view virtual returns (uint256) {
        _requireValidKmsContext(contextId);
        return _getGatewayConfigStorage().userDecryptionThresholdForContext[contextId];
    }

    /**
     * @notice See {IGatewayConfig-getCurrentCoprocessorContextId}.
     */
    function getCurrentCoprocessorContextId() external view virtual returns (uint256) {
        return _getGatewayConfigStorage().currentCoprocessorContextId;
    }

    /**
     * @notice See {IGatewayConfig-isValidCoprocessorContext}.
     */
    function isValidCoprocessorContext(uint256 contextId) external view virtual returns (bool) {
        return _isValidCoprocessorContext(contextId);
    }

    /**
     * @notice See {IGatewayConfig-isCoprocessorTxSenderForContext}.
     * @dev Reverts with `InvalidCoprocessorContext` if the context is unknown or destroyed.
     */
    function isCoprocessorTxSenderForContext(
        uint256 contextId,
        address txSenderAddress
    ) external view virtual returns (bool) {
        _requireValidCoprocessorContext(contextId);
        return _getGatewayConfigStorage().isCoprocessorTxSenderForContext[contextId][txSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-isCoprocessorSignerForContext}.
     * @dev Reverts with `InvalidCoprocessorContext` if the context is unknown or destroyed.
     */
    function isCoprocessorSignerForContext(
        uint256 contextId,
        address signerAddress
    ) external view virtual returns (bool) {
        _requireValidCoprocessorContext(contextId);
        return _getGatewayConfigStorage().isCoprocessorSignerForContext[contextId][signerAddress];
    }

    /**
     * @notice See {IGatewayConfig-getCoprocessorForContext}.
     * @dev Reverts with `InvalidCoprocessorContext` if the context is unknown or destroyed.
     */
    function getCoprocessorForContext(
        uint256 contextId,
        address coprocessorTxSenderAddress
    ) external view virtual returns (Coprocessor memory) {
        _requireValidCoprocessorContext(contextId);
        return _getGatewayConfigStorage().coprocessorsForContext[contextId][coprocessorTxSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-getCoprocessorMajorityThresholdForContext}.
     * @dev Reverts with `InvalidCoprocessorContext` if the context is unknown or destroyed.
     */
    function getCoprocessorMajorityThresholdForContext(uint256 contextId) external view virtual returns (uint256) {
        _requireValidCoprocessorContext(contextId);
        return _getGatewayConfigStorage().coprocessorThresholdForContext[contextId];
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
        if (newKmsNodes.length > MAX_KMS_SIGNERS) {
            revert KmsSignerSetExceedsProofFormatLimit(newKmsNodes.length, MAX_KMS_SIGNERS);
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Register the new KMS nodes for this context
        for (uint256 i = 0; i < newKmsNodes.length; i++) {
            address newKmsTxSenderAddress = newKmsNodes[i].txSenderAddress;
            address newKmsSignerAddress = newKmsNodes[i].signerAddress;

            // Check for KMS transaction sender and signer duplicates within this context
            if ($.isKmsTxSenderForContext[contextId][newKmsTxSenderAddress]) {
                revert KmsTxSenderAlreadyRegistered(newKmsTxSenderAddress);
            }
            if ($.isKmsSignerForContext[contextId][newKmsSignerAddress]) {
                revert KmsSignerAlreadyRegistered(newKmsSignerAddress);
            }

            // Register transaction sender
            $.isKmsTxSenderForContext[contextId][newKmsTxSenderAddress] = true;
            $.kmsTxSenderAddressesForContext[contextId].push(newKmsTxSenderAddress);

            // Register KMS node
            $.kmsNodesForContext[contextId][newKmsTxSenderAddress] = newKmsNodes[i];

            // Register signer
            $.isKmsSignerForContext[contextId][newKmsSignerAddress] = true;
            $.kmsSignerAddressesForContext[contextId].push(newKmsSignerAddress);
        }

        // Setting the thresholds should be done after the KMS nodes have been registered
        // as the functions validate against the context's node count.
        _setMpcThreshold(contextId, newMpcThreshold);
        _setPublicDecryptionThreshold(contextId, newPublicDecryptionThreshold);
        _setUserDecryptionThreshold(contextId, newUserDecryptionThreshold);
        _setKmsGenThreshold(contextId, newKmsGenThreshold);
    }

    /**
     * @notice Sets the coprocessor context: coprocessors and majority threshold for a given context ID.
     * @param contextId The context ID.
     * @param newCoprocessors The new coprocessors.
     * @param newCoprocessorThreshold The new majority threshold for this context.
     */
    function _setCoprocessorContext(
        uint256 contextId,
        Coprocessor[] calldata newCoprocessors,
        uint256 newCoprocessorThreshold
    ) internal virtual {
        uint256 coprocessorCount = newCoprocessors.length;
        if (coprocessorCount == 0) {
            revert EmptyCoprocessors();
        }
        if (coprocessorCount > MAX_COPROCESSORS) {
            revert CoprocessorSetExceedsProofFormatLimit(coprocessorCount, MAX_COPROCESSORS);
        }
        // More than half, so that two conflicting outputs cannot both reach the threshold.
        if (newCoprocessorThreshold <= coprocessorCount / 2 || newCoprocessorThreshold > coprocessorCount) {
            revert InvalidCoprocessorMajorityThreshold(newCoprocessorThreshold, coprocessorCount);
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        for (uint256 i = 0; i < coprocessorCount; i++) {
            address newCoprocessorTxSenderAddress = newCoprocessors[i].txSenderAddress;
            address newCoprocessorSignerAddress = newCoprocessors[i].signerAddress;

            // Check for coprocessor transaction sender and signer duplicates within this context
            if ($.isCoprocessorTxSenderForContext[contextId][newCoprocessorTxSenderAddress]) {
                revert CoprocessorTxSenderAlreadyRegistered(newCoprocessorTxSenderAddress);
            }
            if ($.isCoprocessorSignerForContext[contextId][newCoprocessorSignerAddress]) {
                revert CoprocessorSignerAlreadyRegistered(newCoprocessorSignerAddress);
            }

            $.isCoprocessorTxSenderForContext[contextId][newCoprocessorTxSenderAddress] = true;
            $.coprocessorTxSenderAddressesForContext[contextId].push(newCoprocessorTxSenderAddress);
            $.coprocessorsForContext[contextId][newCoprocessorTxSenderAddress] = newCoprocessors[i];
            $.isCoprocessorSignerForContext[contextId][newCoprocessorSignerAddress] = true;
            $.coprocessorSignerAddressesForContext[contextId].push(newCoprocessorSignerAddress);
        }

        $.coprocessorThresholdForContext[contextId] = newCoprocessorThreshold;
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
     * @notice Returns whether a KMS context ID has been created and not destroyed.
     * @dev A context is considered created once `kmsSignerAddressesForContext[contextId].length > 0`,
     *      which happens in `_setKmsContext` (invoked by `updateKmsContext` and the initializer).
     */
    function _isValidKmsContext(uint256 contextId) internal view returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsSignerAddressesForContext[contextId].length != 0 && !$.destroyedKmsContexts[contextId];
    }

    /**
     * @notice Reverts with `InvalidKmsContext` if the given context is unknown or destroyed.
     * @dev Used by all external per-context views and setters so that callers cannot read or
     *      mutate state of a context that does not exist or has been destroyed.
     */
    function _requireValidKmsContext(uint256 contextId) internal view {
        if (!_isValidKmsContext(contextId)) {
            revert InvalidKmsContext(contextId);
        }
    }

    /**
     * @notice Returns whether a coprocessor context ID has been created and not destroyed.
     * @dev A context is considered created once `coprocessorSignerAddressesForContext[contextId].length > 0`,
     *      which happens in `_setCoprocessorContext` (invoked by `updateCoprocessors` and the initializer)
     *      and in `reinitializeV9`.
     */
    function _isValidCoprocessorContext(uint256 contextId) internal view returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return
            $.coprocessorSignerAddressesForContext[contextId].length != 0 && !$.destroyedCoprocessorContexts[contextId];
    }

    /**
     * @notice Reverts with `InvalidCoprocessorContext` if the given context is unknown or destroyed.
     * @dev Used by all external per-context views so that callers cannot read state of a context that
     *      does not exist or has been destroyed.
     */
    function _requireValidCoprocessorContext(uint256 contextId) internal view {
        if (!_isValidCoprocessorContext(contextId)) {
            revert InvalidCoprocessorContext(contextId);
        }
    }

    /**
     * @notice Sets the MPC threshold for a given context.
     * @param contextId The context ID.
     * @param newMpcThreshold The new MPC threshold.
     */
    function _setMpcThreshold(uint256 contextId, uint256 newMpcThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddressesForContext[contextId].length;

        // Check that the MPC threshold `t` is valid. It must verify:
        // - `t >= 0` : it is already a uint256 so this is always true
        // - `t < n` : it should be strictly less than the number of registered KMS nodes
        if (newMpcThreshold > MAX_KMS_SIGNERS) {
            revert ThresholdExceedsProofFormatLimit("mpc", newMpcThreshold, MAX_KMS_SIGNERS);
        }
        if (newMpcThreshold >= nKmsNodes) {
            revert InvalidHighMpcThreshold(newMpcThreshold, nKmsNodes);
        }

        $.mpcThresholdForContext[contextId] = newMpcThreshold;
    }

    /**
     * @notice Sets the public decryption threshold for a given context.
     * @param contextId The context ID.
     * @param newPublicDecryptionThreshold The new public decryption threshold.
     */
    function _setPublicDecryptionThreshold(uint256 contextId, uint256 newPublicDecryptionThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddressesForContext[contextId].length;

        // Check that the public decryption threshold `t` is valid. It must verify:
        // - `t >= 1` : the public decryption consensus should require at least one vote
        // - `t <= n` : it should be less than or equal to the number of registered KMS nodes
        if (newPublicDecryptionThreshold == 0) {
            revert InvalidNullPublicDecryptionThreshold();
        }
        if (newPublicDecryptionThreshold > MAX_KMS_SIGNERS) {
            revert ThresholdExceedsProofFormatLimit("publicDecryption", newPublicDecryptionThreshold, MAX_KMS_SIGNERS);
        }
        if (newPublicDecryptionThreshold > nKmsNodes) {
            revert InvalidHighPublicDecryptionThreshold(newPublicDecryptionThreshold, nKmsNodes);
        }

        $.publicDecryptionThresholdForContext[contextId] = newPublicDecryptionThreshold;
    }

    /**
     * @notice Sets the user decryption threshold for a given context.
     * @param contextId The context ID.
     * @param newUserDecryptionThreshold The new user decryption threshold.
     */
    function _setUserDecryptionThreshold(uint256 contextId, uint256 newUserDecryptionThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddressesForContext[contextId].length;

        // Check that the user decryption threshold `t` is valid. It must verify:
        // - `t >= 1` : the user decryption consensus should require at least one vote
        // - `t <= n` : it should be less than or equal to the number of registered KMS nodes
        if (newUserDecryptionThreshold == 0) {
            revert InvalidNullUserDecryptionThreshold();
        }
        if (newUserDecryptionThreshold > MAX_KMS_SIGNERS) {
            revert ThresholdExceedsProofFormatLimit("userDecryption", newUserDecryptionThreshold, MAX_KMS_SIGNERS);
        }
        if (newUserDecryptionThreshold > nKmsNodes) {
            revert InvalidHighUserDecryptionThreshold(newUserDecryptionThreshold, nKmsNodes);
        }

        $.userDecryptionThresholdForContext[contextId] = newUserDecryptionThreshold;
    }

    /**
     * @notice Sets the key and CRS generation threshold for a given context.
     * @param contextId The context ID.
     * @param newKmsGenThreshold The new key and CRS generation threshold.
     */
    function _setKmsGenThreshold(uint256 contextId, uint256 newKmsGenThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddressesForContext[contextId].length;

        // Check that the key and CRS generation threshold `t` is valid. It must verify:
        // - `t >= 1` : the key and CRS generation consensus should require at least one vote
        // - `t <= n` : it should be less than or equal to the number of registered KMS nodes
        if (newKmsGenThreshold == 0) {
            revert InvalidNullKmsGenThreshold();
        }
        if (newKmsGenThreshold > MAX_KMS_SIGNERS) {
            revert ThresholdExceedsProofFormatLimit("kmsGen", newKmsGenThreshold, MAX_KMS_SIGNERS);
        }
        if (newKmsGenThreshold > nKmsNodes) {
            revert InvalidHighKmsGenThreshold(newKmsGenThreshold, nKmsNodes);
        }

        $.kmsGenThresholdForContext[contextId] = newKmsGenThreshold;
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
