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
     * @notice The operator's thresholds.
     */
    struct Thresholds {
        /// @notice The MPC threshold
        uint256 mpcThreshold;
        /// @notice The threshold to consider for public decryption consensus
        uint256 publicDecryptionThreshold;
        /// @notice The threshold to consider for user decryption consensus
        uint256 userDecryptionThreshold;
        /// @notice The threshold to consider for KMS generation consensus
        uint256 kmsGenThreshold;
        /// @notice The threshold to consider for coprocessor consensus
        uint256 coprocessorThreshold;
    }

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
    uint256 private constant MINOR_VERSION = 3;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     * This constant does not represent the number of time a specific contract have been upgraded,
     * as a contract deployed from version VX will have a REINITIALIZER_VERSION > 2.
     */
    uint64 private constant REINITIALIZER_VERSION = 4;

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
        // KMS nodes state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The KMS nodes' transaction sender addresses
        mapping(address kmsTxSenderAddress => bool isTxSender) isKmsTxSender;
        /// @notice The KMS nodes' signer addresses
        mapping(address kmsSignerAddress => bool isSigner) isKmsSigner;
        /// @notice The KMS nodes' metadata
        mapping(address kmsTxSenderAddress => KmsNode kmsNode) kmsNodes;
        /// @notice The KMS nodes' transaction sender address list
        address[] kmsTxSenderAddresses;
        /// @notice The KMS nodes' signer address list
        address[] kmsSignerAddresses;
        /// @notice The MPC threshold
        uint256 mpcThreshold;
        /// @notice The threshold to consider for public decryption consensus
        uint256 publicDecryptionThreshold;
        /// @notice The threshold to consider for user decryption consensus
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
        /// @notice The threshold to consider for the KMS public material (FHE key, CRS) generation consensus.
        uint256 kmsGenThreshold;
        // ----------------------------------------------------------------------------------------------
        // Coprocessor threshold state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The threshold to consider for coprocessor consensus
        uint256 coprocessorThreshold;
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
        ProtocolMetadata calldata initialMetadata,
        Thresholds calldata initialThresholds,
        KmsNode[] calldata initialKmsNodes,
        Coprocessor[] calldata initialCoprocessors,
        Custodian[] calldata initialCustodians
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __Ownable_init(owner());

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        $.protocolMetadata = initialMetadata;

        // Set the KMS nodes and their thresholds
        _setKmsNodes(
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
            initialMetadata,
            initialThresholds.mpcThreshold,
            initialKmsNodes,
            initialCoprocessors,
            initialCustodians
        );
    }

    /**
     * @notice Re-initializes the contract from V2.
     * @dev Define a `reinitializeVX` function once the contract needs to be upgraded.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV3(KmsNode[] calldata newKmsNodes) public virtual reinitializer(REINITIALIZER_VERSION) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        updateKmsNodes(
            newKmsNodes,
            $.mpcThreshold,
            $.publicDecryptionThreshold,
            $.userDecryptionThreshold,
            $.kmsGenThreshold
        );
        emit ReinitializeGatewayConfigV3(newKmsNodes);
    }

    /**
     * @notice See {IGatewayConfig-isPauser}.
     */
    function isPauser(address account) public view virtual returns (bool) {
        return PAUSER_SET.isPauser(account);
    }

    /**
     * @notice See {IGatewayConfig-updateKmsNodes}.
     */
    function updateKmsNodes(
        KmsNode[] calldata newKmsNodes,
        uint256 newMpcThreshold,
        uint256 newPublicDecryptionThreshold,
        uint256 newUserDecryptionThreshold,
        uint256 newKmsGenThreshold
    ) public virtual onlyOwner {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        // Remove the old KMS nodes
        uint256 oldKmsTxSenderAddressesLength = $.kmsTxSenderAddresses.length;
        for (uint256 i = 0; i < oldKmsTxSenderAddressesLength; i++) {
            $.isKmsTxSender[$.kmsTxSenderAddresses[i]] = false;
            delete $.kmsNodes[$.kmsTxSenderAddresses[i]];
        }
        uint256 oldKmsSignerAddressesLength = $.kmsSignerAddresses.length;
        for (uint256 i = 0; i < oldKmsSignerAddressesLength; i++) {
            $.isKmsSigner[$.kmsSignerAddresses[i]] = false;
        }

        delete $.kmsTxSenderAddresses;
        delete $.kmsSignerAddresses;

        // Set the new KMS nodes and their thresholds
        _setKmsNodes(
            newKmsNodes,
            newMpcThreshold,
            newPublicDecryptionThreshold,
            newUserDecryptionThreshold,
            newKmsGenThreshold
        );

        emit UpdateKmsNodes(
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
            delete $.coprocessors[$.coprocessorTxSenderAddresses[i]];
        }

        uint256 oldCoprocessorSignerAddressesLength = $.coprocessorSignerAddresses.length;
        for (uint256 i = 0; i < oldCoprocessorSignerAddressesLength; i++) {
            $.isCoprocessorSigner[$.coprocessorSignerAddresses[i]] = false;
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
            delete $.custodians[$.custodianTxSenderAddresses[i]];
        }

        uint256 oldCustodianSignerAddressesLength = $.custodianSignerAddresses.length;
        for (uint256 i = 0; i < oldCustodianSignerAddressesLength; i++) {
            $.isCustodianSigner[$.custodianSignerAddresses[i]] = false;
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
        _setMpcThreshold(newMpcThreshold);
        emit UpdateMpcThreshold(newMpcThreshold);
    }

    /**
     * @notice See {IGatewayConfig-updatePublicDecryptionThreshold}.
     */
    function updatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold) external virtual onlyOwner {
        _setPublicDecryptionThreshold(newPublicDecryptionThreshold);
        emit UpdatePublicDecryptionThreshold(newPublicDecryptionThreshold);
    }

    /**
     * @notice See {IGatewayConfig-updateUserDecryptionThreshold}.
     */
    function updateUserDecryptionThreshold(uint256 newUserDecryptionThreshold) external virtual onlyOwner {
        _setUserDecryptionThreshold(newUserDecryptionThreshold);
        emit UpdateUserDecryptionThreshold(newUserDecryptionThreshold);
    }

    /**
     * @notice See {IGatewayConfig-updateKmsGenThreshold}.
     */
    function updateKmsGenThreshold(uint256 newKmsGenThreshold) external virtual onlyOwner {
        _setKmsGenThreshold(newKmsGenThreshold);
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
        return $.isKmsTxSender[txSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-isKmsSigner}.
     */
    function isKmsSigner(address signerAddress) external view virtual returns (bool) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.isKmsSigner[signerAddress];
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
        return $.mpcThreshold;
    }

    /**
     * @notice See {IGatewayConfig-getPublicDecryptionThreshold}.
     */
    function getPublicDecryptionThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.publicDecryptionThreshold;
    }

    /**
     * @notice See {IGatewayConfig-getUserDecryptionThreshold}.
     */
    function getUserDecryptionThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.userDecryptionThreshold;
    }

    /**
     * @notice See {IGatewayConfig-getKmsGenThreshold}.
     */
    function getKmsGenThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsGenThreshold;
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
        return $.kmsNodes[kmsTxSenderAddress];
    }

    /**
     * @notice See {IGatewayConfig-getKmsTxSenders}.
     */
    function getKmsTxSenders() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsTxSenderAddresses;
    }

    /**
     * @notice See {IGatewayConfig-getKmsSigners}.
     */
    function getKmsSigners() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsSignerAddresses;
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
     * @notice Sets the KMS nodes and their thresholds.
     * @param newKmsNodes The new KMS nodes.
     * @param newMpcThreshold The new MPC threshold.
     * @param newPublicDecryptionThreshold The new public decryption threshold.
     * @param newUserDecryptionThreshold The new user decryption threshold.
     * @param newKmsGenThreshold The new key and CRS generation threshold.
     */
    function _setKmsNodes(
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

        // Register the new KMS nodes
        for (uint256 i = 0; i < newKmsNodes.length; i++) {
            $.isKmsTxSender[newKmsNodes[i].txSenderAddress] = true;
            $.kmsNodes[newKmsNodes[i].txSenderAddress] = newKmsNodes[i];
            $.kmsTxSenderAddresses.push(newKmsNodes[i].txSenderAddress);
            $.isKmsSigner[newKmsNodes[i].signerAddress] = true;
            $.kmsSignerAddresses.push(newKmsNodes[i].signerAddress);
        }

        // Setting the thresholds should be done after the KMS nodes have been registered as the functions
        // reading the `kmsSignerAddresses` array.
        _setMpcThreshold(newMpcThreshold);
        _setPublicDecryptionThreshold(newPublicDecryptionThreshold);
        _setUserDecryptionThreshold(newUserDecryptionThreshold);
        _setKmsGenThreshold(newKmsGenThreshold);
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
            $.isCoprocessorTxSender[newCoprocessors[i].txSenderAddress] = true;
            $.coprocessors[newCoprocessors[i].txSenderAddress] = newCoprocessors[i];
            $.coprocessorTxSenderAddresses.push(newCoprocessors[i].txSenderAddress);
            $.isCoprocessorSigner[newCoprocessors[i].signerAddress] = true;
            $.coprocessorSignerAddresses.push(newCoprocessors[i].signerAddress);
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
            $.custodians[newCustodians[i].txSenderAddress] = newCustodians[i];
            $.custodianTxSenderAddresses.push(newCustodians[i].txSenderAddress);
            $.isCustodianTxSender[newCustodians[i].txSenderAddress] = true;
            $.custodianSignerAddresses.push(newCustodians[i].signerAddress);
            $.isCustodianSigner[newCustodians[i].signerAddress] = true;
        }
    }

    /**
     * @notice Sets the MPC threshold.
     * @param newMpcThreshold The new MPC threshold.
     */
    function _setMpcThreshold(uint256 newMpcThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddresses.length;

        // Check that the MPC threshold `t` is valid. It must verify:
        // - `t >= 0` : it is already a uint256 so this is always true
        // - `t < n` : it should be strictly less than the number of registered KMS nodes
        if (newMpcThreshold >= nKmsNodes) {
            revert InvalidHighMpcThreshold(newMpcThreshold, nKmsNodes);
        }

        $.mpcThreshold = newMpcThreshold;
    }

    /**
     * @notice Sets the public decryption threshold.
     * @param newPublicDecryptionThreshold The new public decryption threshold.
     */
    function _setPublicDecryptionThreshold(uint256 newPublicDecryptionThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddresses.length;

        // Check that the public decryption threshold `t` is valid. It must verify:
        // - `t >= 1` : the public decryption consensus should require at least one vote
        // - `t <= n` : it should be less than the number of registered KMS nodes
        if (newPublicDecryptionThreshold == 0) {
            revert InvalidNullPublicDecryptionThreshold();
        }
        if (newPublicDecryptionThreshold > nKmsNodes) {
            revert InvalidHighPublicDecryptionThreshold(newPublicDecryptionThreshold, nKmsNodes);
        }

        $.publicDecryptionThreshold = newPublicDecryptionThreshold;
    }

    /**
     * @notice Sets the user decryption threshold.
     * @param newUserDecryptionThreshold The new user decryption threshold.
     */
    function _setUserDecryptionThreshold(uint256 newUserDecryptionThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddresses.length;

        // Check that the user decryption threshold `t` is valid. It must verify:
        // - `t >= 1` : the user decryption consensus should require at least one vote
        // - `t <= n` : it should be less than the number of registered KMS nodes
        if (newUserDecryptionThreshold == 0) {
            revert InvalidNullUserDecryptionThreshold();
        }
        if (newUserDecryptionThreshold > nKmsNodes) {
            revert InvalidHighUserDecryptionThreshold(newUserDecryptionThreshold, nKmsNodes);
        }

        $.userDecryptionThreshold = newUserDecryptionThreshold;
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
        // - `t <= n` : it should be less than the number of registered coprocessors
        if (newCoprocessorThreshold == 0) {
            revert InvalidNullCoprocessorThreshold();
        }
        if (newCoprocessorThreshold > nCoprocessors) {
            revert InvalidHighCoprocessorThreshold(newCoprocessorThreshold, nCoprocessors);
        }

        $.coprocessorThreshold = newCoprocessorThreshold;
    }

    /**
     * @notice Sets the key and CRS generation threshold.
     * @param newKmsGenThreshold The new key and CRS generation threshold.
     */
    function _setKmsGenThreshold(uint256 newKmsGenThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddresses.length;

        // Check that the key and CRS generation threshold `t` is valid. It must verify:
        // - `t >= 1` : the key and CRS generation consensus should require at least one vote
        // - `t <= n` : it should be less than the number of registered KMS nodes
        if (newKmsGenThreshold == 0) {
            revert InvalidNullKmsGenThreshold();
        }
        if (newKmsGenThreshold > nKmsNodes) {
            revert InvalidHighKmsGenThreshold(newKmsGenThreshold, nKmsNodes);
        }

        $.kmsGenThreshold = newKmsGenThreshold;
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
