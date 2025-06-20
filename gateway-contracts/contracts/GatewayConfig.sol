// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/IGatewayConfig.sol";
import "./shared/UUPSUpgradeableEmptyProxy.sol";
import "./shared/Pausable.sol";

/**
 * @title GatewayConfig contract
 * @dev See {IGatewayConfig}.
 * @dev Add/remove methods will be added in the future for KMS nodes, host chains.
 * @dev See https://github.com/zama-ai/fhevm-gateway/issues/98 for more details.
 */
contract GatewayConfig is IGatewayConfig, Ownable2StepUpgradeable, UUPSUpgradeableEmptyProxy, Pausable {
    /// @notice The maximum chain ID.
    uint256 internal constant MAX_CHAIN_ID = type(uint64).max;

    /// @dev The following constants are used for versioning the contract. They are made private
    /// @dev in order to force derived contracts to consider a different version. Note that
    /// @dev they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "GatewayConfig";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 2;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:fhevm_gateway.storage.GatewayConfig
    struct GatewayConfigStorage {
        // ----------------------------------------------------------------------------------------------
        // Pauser state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The pauser's address
        address pauser;
        // ----------------------------------------------------------------------------------------------
        // Transaction sender and signer state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The KMS nodes' transaction sender addresses
        mapping(address kmsTxSenderAddress => bool isKmsTxSender) _isKmsTxSender;
        /// @notice The KMS nodes' signer addresses
        mapping(address kmsSignerAddress => bool isKmsSigner) _isKmsSigner;
        /// @notice The coprocessors' transaction sender addresses
        mapping(address coprocessorTxSenderAddress => bool isCoprocessorTxSender) _isCoprocessorTxSender; // deprecated
        /// @notice The coprocessors' signer addresses
        mapping(address coprocessorSignerAddress => bool isCoprocessorSigner) _isCoprocessorSigner; // deprecated
        // ----------------------------------------------------------------------------------------------
        // Host chains state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The host chains' registered status
        mapping(uint256 chainId => bool isRegistered) _isHostChainRegistered;
        // ----------------------------------------------------------------------------------------------
        // Protocol metadata state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The protocol's metadata
        ProtocolMetadata protocolMetadata;
        // ----------------------------------------------------------------------------------------------
        // KMS nodes state variables:
        // ----------------------------------------------------------------------------------------------
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
        // Coprocessors state variables (deprecated):
        // ----------------------------------------------------------------------------------------------
        /// @notice The coprocessors' metadata
        mapping(address coprocessorTxSenderAddress => Coprocessor coprocessor) coprocessors; // deprecated
        /// @notice The coprocessors' transaction sender address list
        address[] coprocessorTxSenderAddresses; // deprecated
        /// @notice The coprocessors' signer address list
        address[] coprocessorSignerAddresses; // deprecated
        // ----------------------------------------------------------------------------------------------
        // Host chains state variables:
        // ----------------------------------------------------------------------------------------------
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
        mapping(address custodianTxSenderAddress => bool isCustodianTxSender) _isCustodianTxSender;
        /// @notice The custodians' signer addresses
        mapping(address custodianSignerAddress => bool isCustodianSigner) _isCustodianSigner;
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

    /// @notice Initializes the contract
    /// @dev This function needs to be public in order to be called by the UUPS proxy.
    /// @param initialPauser Pauser address
    /// @param initialMetadata Metadata of the protocol
    /// @param initialMpcThreshold The MPC threshold
    /// @param initialPublicDecryptionThreshold The public decryption threshold
    /// @param initialUserDecryptionThreshold The user decryption threshold
    /// @param initialKmsNodes List of KMS nodes
    /// @param initialCustodians List of custodians
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        address initialPauser,
        ProtocolMetadata memory initialMetadata,
        uint256 initialMpcThreshold,
        uint256 initialPublicDecryptionThreshold,
        uint256 initialUserDecryptionThreshold,
        KmsNode[] memory initialKmsNodes,
        Custodian[] memory initialCustodians
    ) public virtual onlyFromEmptyProxy reinitializer(3) {
        __Ownable_init(owner());
        __Pausable_init();

        if (initialPauser == address(0)) {
            revert InvalidNullPauser();
        }

        if (initialKmsNodes.length == 0) {
            revert EmptyKmsNodes();
        }

        if (initialCustodians.length == 0) {
            revert EmptyCustodians();
        }

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        $.protocolMetadata = initialMetadata;

        /// @dev Register the pauser
        $.pauser = initialPauser;

        /// @dev Register the KMS nodes
        for (uint256 i = 0; i < initialKmsNodes.length; i++) {
            $._isKmsTxSender[initialKmsNodes[i].txSenderAddress] = true;
            $.kmsNodes[initialKmsNodes[i].txSenderAddress] = initialKmsNodes[i];
            $.kmsTxSenderAddresses.push(initialKmsNodes[i].txSenderAddress);
            $._isKmsSigner[initialKmsNodes[i].signerAddress] = true;
            $.kmsSignerAddresses.push(initialKmsNodes[i].signerAddress);
        }

        /// @dev Setting the threshold should be done after the KMS nodes have been registered as the functions
        /// @dev reading the `kmsSignerAddresses` array.
        _setMpcThreshold(initialMpcThreshold);
        _setPublicDecryptionThreshold(initialPublicDecryptionThreshold);
        _setUserDecryptionThreshold(initialUserDecryptionThreshold);

        /// @dev Register the custodians
        for (uint256 i = 0; i < initialCustodians.length; i++) {
            $.custodians[initialCustodians[i].txSenderAddress] = initialCustodians[i];
            $.custodianTxSenderAddresses.push(initialCustodians[i].txSenderAddress);
            $._isCustodianTxSender[initialCustodians[i].txSenderAddress] = true;
            $.custodianSignerAddresses.push(initialCustodians[i].signerAddress);
            $._isCustodianSigner[initialCustodians[i].signerAddress] = true;
        }

        emit InitializeGatewayConfig(
            initialPauser,
            initialMetadata,
            initialMpcThreshold,
            initialKmsNodes,
            initialCustodians
        );
    }

    /// @notice Reinitializes the contract with custodians.
    function reinitializeV2(Custodian[] memory custodians) external reinitializer(3) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();

        if (custodians.length == 0) {
            revert EmptyCustodians();
        }

        /// @dev Register the custodians
        for (uint256 i = 0; i < custodians.length; i++) {
            $.custodians[custodians[i].txSenderAddress] = custodians[i];
            $.custodianTxSenderAddresses.push(custodians[i].txSenderAddress);
            $._isCustodianTxSender[custodians[i].txSenderAddress] = true;
            $.custodianSignerAddresses.push(custodians[i].signerAddress);
            $._isCustodianSigner[custodians[i].signerAddress] = true;
        }

        emit ReinitializeGatewayConfigV2(custodians);
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

    /// @dev See {IGatewayConfig-updateMpcThreshold}.
    function updateMpcThreshold(uint256 newMpcThreshold) external virtual onlyOwner whenNotPaused {
        _setMpcThreshold(newMpcThreshold);
        emit UpdateMpcThreshold(newMpcThreshold);
    }

    /// @dev See {IGatewayConfig-updatePublicDecryptionThreshold}.
    function updatePublicDecryptionThreshold(
        uint256 newPublicDecryptionThreshold
    ) external virtual onlyOwner whenNotPaused {
        _setPublicDecryptionThreshold(newPublicDecryptionThreshold);
        emit UpdatePublicDecryptionThreshold(newPublicDecryptionThreshold);
    }

    /// @dev See {IGatewayConfig-updateUserDecryptionThreshold}.
    function updateUserDecryptionThreshold(
        uint256 newUserDecryptionThreshold
    ) external virtual onlyOwner whenNotPaused {
        _setUserDecryptionThreshold(newUserDecryptionThreshold);
        emit UpdateUserDecryptionThreshold(newUserDecryptionThreshold);
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

    /// @dev See {IGatewayConfig-checkIsKmsTxSender}.
    function checkIsKmsTxSender(address txSenderAddress) external view virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$._isKmsTxSender[txSenderAddress]) {
            revert NotKmsTxSender(txSenderAddress);
        }
    }

    /// @dev See {IGatewayConfig-checkIsKmsSigner}.
    function checkIsKmsSigner(address signerAddress) external view virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$._isKmsSigner[signerAddress]) {
            revert NotKmsSigner(signerAddress);
        }
    }

    /// @dev See {IGatewayConfig-checkIsCustodianTxSender}.
    function checkIsCustodianTxSender(address txSenderAddress) external view virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$._isCustodianTxSender[txSenderAddress]) {
            revert NotCustodianTxSender(txSenderAddress);
        }
    }

    /// @dev See {IGatewayConfig-checkIsCustodianSigner}.
    function checkIsCustodianSigner(address signerAddress) external view virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        if (!$._isCustodianSigner[signerAddress]) {
            revert NotCustodianSigner(signerAddress);
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
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.mpcThreshold;
    }

    /// @dev See {IGatewayConfig-getPublicDecryptionThreshold}.
    function getPublicDecryptionThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.publicDecryptionThreshold;
    }

    /// @dev See {IGatewayConfig-getUserDecryptionThreshold}.
    function getUserDecryptionThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.userDecryptionThreshold;
    }

    /// @dev See {IGatewayConfig-getKmsNode}.
    function getKmsNode(address kmsTxSenderAddress) external view virtual returns (KmsNode memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsNodes[kmsTxSenderAddress];
    }

    /// @dev See {IGatewayConfig-getKmsTxSenders}.
    function getKmsTxSenders() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsTxSenderAddresses;
    }

    /// @dev See {IGatewayConfig-getKmsSigners}.
    function getKmsSigners() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsSignerAddresses;
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

    /// @dev See {IGatewayConfig-getCustodian}.
    function getCustodian(address custodianTxSenderAddress) external view virtual returns (Custodian memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.custodians[custodianTxSenderAddress];
    }

    /// @dev See {IGatewayConfig-getCustodianTxSenders}.
    function getCustodianTxSenders() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.custodianTxSenderAddresses;
    }

    /// @dev See {IGatewayConfig-getCustodianSigners}.
    function getCustodianSigners() external view virtual returns (address[] memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.custodianSignerAddresses;
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

    /**
     * @dev Sets the MPC threshold.
     * @param newMpcThreshold The new MPC threshold.
     */
    function _setMpcThreshold(uint256 newMpcThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddresses.length;

        /// @dev Check that the MPC threshold `t` is valid. It must verify:
        /// @dev - `t >= 0` : it is already a uint256 so this is always true
        /// @dev - `t < n` : it should be strictly less than the number of registered KMS nodes
        if (newMpcThreshold >= nKmsNodes) {
            revert InvalidHighMpcThreshold(newMpcThreshold, nKmsNodes);
        }

        $.mpcThreshold = newMpcThreshold;
    }

    /**
     * @dev Sets the public decryption threshold.
     * @param newPublicDecryptionThreshold The new public decryption threshold.
     */
    function _setPublicDecryptionThreshold(uint256 newPublicDecryptionThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddresses.length;

        /// @dev Check that the public decryption threshold `t` is valid. It must verify:
        /// @dev - `t >= 1` : the public decryption consensus should require at least one vote
        /// @dev - `t <= n` : it should be less than the number of registered KMS nodes
        if (newPublicDecryptionThreshold == 0) {
            revert InvalidNullPublicDecryptionThreshold();
        }
        if (newPublicDecryptionThreshold > nKmsNodes) {
            revert InvalidHighPublicDecryptionThreshold(newPublicDecryptionThreshold, nKmsNodes);
        }

        $.publicDecryptionThreshold = newPublicDecryptionThreshold;
    }

    /**
     * @dev Sets the user decryption threshold.
     * @param newUserDecryptionThreshold The new user decryption threshold.
     */
    function _setUserDecryptionThreshold(uint256 newUserDecryptionThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddresses.length;

        /// @dev Check that the user decryption threshold `t` is valid. It must verify:
        /// @dev - `t >= 1` : the user decryption consensus should require at least one vote
        /// @dev - `t <= n` : it should be less than the number of registered KMS nodes
        if (newUserDecryptionThreshold == 0) {
            revert InvalidNullUserDecryptionThreshold();
        }
        if (newUserDecryptionThreshold > nKmsNodes) {
            revert InvalidHighUserDecryptionThreshold(newUserDecryptionThreshold, nKmsNodes);
        }

        $.userDecryptionThreshold = newUserDecryptionThreshold;
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

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
