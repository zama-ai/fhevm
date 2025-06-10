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
import { ProtocolMetadata, HostChain, CoprocessorV1, Custodian, KmsNode } from "./shared/Structs.sol";

/**
 * @title GatewayConfig contract
 * @notice See {IGatewayConfig}.
 * @dev Add/remove methods will be added in the future for KMS nodes, host chains.
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
    uint256 private constant MINOR_VERSION = 2;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     * This constant does not represent the number of time a specific contract have been upgraded,
     * as a contract deployed from version VX will have a REINITIALIZER_VERSION > 2.
     */
    uint64 private constant REINITIALIZER_VERSION = 3;

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
        // Coprocessors state variables (DEPRECATED):
        // ----------------------------------------------------------------------------------------------
        /// @notice DEPRECATED: coprocessors are stored in `CoprocessorContexts` contract
        mapping(address coprocessorTxSenderAddress => bool isTxSender) isCoprocessorTxSender; // DEPRECATED
        /// @notice DEPRECATED: coprocessors are stored in `CoprocessorContexts` contract
        mapping(address coprocessorSignerAddress => bool isSigner) isCoprocessorSigner; // DEPRECATED
        /// @notice DEPRECATED: coprocessors are stored in `CoprocessorContexts` contract
        mapping(address coprocessorTxSenderAddress => CoprocessorV1 coprocessor) coprocessors; // DEPRECATED
        /// @notice DEPRECATED: coprocessors are stored in `CoprocessorContexts` contract
        address[] coprocessorTxSenderAddresses; // DEPRECATED
        /// @notice DEPRECATED: coprocessors are stored in `CoprocessorContexts` contract
        address[] coprocessorSignerAddresses; // DEPRECATED
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
     * @param initialMpcThreshold The MPC threshold
     * @param initialPublicDecryptionThreshold The public decryption threshold
     * @param initialUserDecryptionThreshold The user decryption threshold
     * @param initialKmsGenThreshold The KMS generation threshold
     * @param initialKmsNodes List of KMS nodes
     * @param initialCustodians List of custodians
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        ProtocolMetadata memory initialMetadata,
        uint256 initialMpcThreshold,
        uint256 initialPublicDecryptionThreshold,
        uint256 initialUserDecryptionThreshold,
        uint256 initialKmsGenThreshold,
        KmsNode[] memory initialKmsNodes,
        Custodian[] memory initialCustodians
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __Ownable_init(owner());

        if (initialKmsNodes.length == 0) {
            revert EmptyKmsNodes();
        }

        if (initialCustodians.length == 0) {
            revert EmptyCustodians();
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        $.protocolMetadata = initialMetadata;

        // Register the KMS nodes
        for (uint256 i = 0; i < initialKmsNodes.length; i++) {
            $.isKmsTxSender[initialKmsNodes[i].txSenderAddress] = true;
            $.kmsNodes[initialKmsNodes[i].txSenderAddress] = initialKmsNodes[i];
            $.kmsTxSenderAddresses.push(initialKmsNodes[i].txSenderAddress);
            $.isKmsSigner[initialKmsNodes[i].signerAddress] = true;
            $.kmsSignerAddresses.push(initialKmsNodes[i].signerAddress);
        }

        // Setting the threshold should be done after the KMS nodes have been registered as the functions
        // reading the `kmsSignerAddresses` array.
        _setMpcThreshold(initialMpcThreshold);
        _setPublicDecryptionThreshold(initialPublicDecryptionThreshold);
        _setUserDecryptionThreshold(initialUserDecryptionThreshold);
        _setKmsGenThreshold(initialKmsGenThreshold);

        // Register the custodians
        for (uint256 i = 0; i < initialCustodians.length; i++) {
            $.custodians[initialCustodians[i].txSenderAddress] = initialCustodians[i];
            $.custodianTxSenderAddresses.push(initialCustodians[i].txSenderAddress);
            $.isCustodianTxSender[initialCustodians[i].txSenderAddress] = true;
            $.custodianSignerAddresses.push(initialCustodians[i].signerAddress);
            $.isCustodianSigner[initialCustodians[i].signerAddress] = true;
        }

        emit InitializeGatewayConfig(initialMetadata, initialMpcThreshold, initialKmsNodes, initialCustodians);
    }

    /**
     * @notice Re-initializes the contract from V1.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice See {IGatewayConfig-isPauser}.
     */
    function isPauser(address account) public view virtual returns (bool) {
        return PAUSER_SET.isPauser(account);
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
