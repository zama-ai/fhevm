// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./interfaces/IGatewayConfig.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title GatewayConfig contract
 * @dev See {IGatewayConfig}.
 * @dev Add/remove methods will be added in the future for KMS nodes, coprocessors and host chains.
 * @dev See https://github.com/zama-ai/fhevm-gateway/issues/98 for more details.
 */
contract GatewayConfig is IGatewayConfig, Ownable2StepUpgradeable, UUPSUpgradeable {
    /// @notice The maximum chain ID.
    uint256 internal constant MAX_CHAIN_ID = type(uint64).max;

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
        /// @notice The KMS nodes' transaction sender addresses
        mapping(address kmsTxSenderAddress => bool isKmsTxSender) _isKmsTxSender;
        /// @notice The KMS nodes' signer addresses
        mapping(address kmsSignerAddress => bool isKmsSigner) _isKmsSigner;
        /// @notice The coprocessors' transaction sender addresses
        mapping(address coprocessorTxSenderAddress => bool isCoprocessorTxSender) _isCoprocessorTxSender;
        /// @notice The coprocessors' signer addresses
        mapping(address coprocessorSignerAddress => bool isCoprocessorSigner) _isCoprocessorSigner;
        /// @notice The host chains' registered status
        mapping(uint256 chainId => bool isRegistered) _isHostChainRegistered;
        /// @notice The protocol's metadata
        ProtocolMetadata protocolMetadata;
        /// @notice The KMS nodes' metadata
        mapping(address kmsTxSenderAddress => KmsNode kmsNode) kmsNodes;
        /// @notice The KMS nodes' transaction sender address list
        address[] kmsTxSenderAddresses;
        /// @notice The KMS nodes' signer address list
        address[] kmsSignerAddresses;
        /// @notice The KMS threshold parameter to consider for decryption consensus
        uint256 kmsThreshold;
        /// @notice The coprocessors' metadata
        mapping(address coprocessorTxSenderAddress => Coprocessor coprocessor) coprocessors;
        /// @notice The coprocessors' transaction sender address list
        address[] coprocessorTxSenderAddresses;
        /// @notice The coprocessors' signer address list
        address[] coprocessorSignerAddresses;
        /// @notice The host chains' metadata
        HostChain[] hostChains;
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
    /// @param initialKmsThreshold The KMS threshold parameter
    /// @param initialKmsNodes List of KMS nodes
    /// @param initialCoprocessors List of coprocessors
    function initialize(
        address initialPauser,
        ProtocolMetadata memory initialMetadata,
        uint256 initialKmsThreshold,
        KmsNode[] memory initialKmsNodes,
        Coprocessor[] memory initialCoprocessors
    ) public virtual reinitializer(2) {
        __Ownable_init(owner());

        if (initialPauser == address(0)) {
            revert InvalidNullPauser();
        }

        if (initialKmsNodes.length == 0) {
            revert EmptyKmsNodes();
        }

        if (initialCoprocessors.length == 0) {
            revert EmptyCoprocessors();
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

        /// @dev Set the KMS threshold.
        /// @dev This should be done after the KMS nodes have been registered as the function reads
        /// @dev the `kmsSignerAddresses` array.
        _setKmsThreshold(initialKmsThreshold);

        /// @dev Register the coprocessors
        for (uint256 i = 0; i < initialCoprocessors.length; i++) {
            $._isCoprocessorTxSender[initialCoprocessors[i].txSenderAddress] = true;
            $.coprocessors[initialCoprocessors[i].txSenderAddress] = initialCoprocessors[i];
            $.coprocessorTxSenderAddresses.push(initialCoprocessors[i].txSenderAddress);
            $._isCoprocessorSigner[initialCoprocessors[i].signerAddress] = true;
            $.coprocessorSignerAddresses.push(initialCoprocessors[i].signerAddress);
        }

        emit Initialization(initialPauser, initialMetadata, initialKmsThreshold, initialKmsNodes, initialCoprocessors);
    }

    /// @dev See {IGatewayConfig-updatePauser}.
    function updatePauser(address newPauser) external virtual onlyOwner {
        if (newPauser == address(0)) {
            revert InvalidNullPauser();
        }
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        $.pauser = newPauser;
        emit UpdatePauser(newPauser);
    }

    /// @dev See {IGatewayConfig-updateKmsThreshold}.
    function updateKmsThreshold(uint256 newKmsThreshold) external virtual onlyOwner {
        _setKmsThreshold(newKmsThreshold);
        emit UpdateKmsThreshold(newKmsThreshold);
    }

    /// @dev See {IGatewayConfig-addHostChain}.
    function addHostChain(HostChain calldata hostChain) external virtual onlyOwner {
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

    /// @dev See {IGatewayConfig-getProtocolMetadata}.
    function getProtocolMetadata() external view virtual returns (ProtocolMetadata memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.protocolMetadata;
    }

    /// @dev See {IGatewayConfig-getKmsThreshold}.
    function getKmsThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsThreshold;
    }

    /// @dev See {IGatewayConfig-getKmsMajorityThreshold}.
    function getKmsMajorityThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.kmsThreshold + 1;
    }

    /// @dev See {IGatewayConfig-getKmsReconstructionThreshold}.
    function getKmsReconstructionThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return 2 * $.kmsThreshold + 1;
    }

    /// @dev See {IGatewayConfig-getCoprocessorMajorityThreshold}.
    function getCoprocessorMajorityThreshold() external view virtual returns (uint256) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.coprocessorTxSenderAddresses.length / 2 + 1;
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

    /**
     * @dev Sets the KMS threshold.
     * @param newKmsThreshold The new KMS threshold.
     */
    function _setKmsThreshold(uint256 newKmsThreshold) internal virtual {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        uint256 nKmsNodes = $.kmsSignerAddresses.length;

        /// @dev Check that the KMS threshold `t` is valid. It must verify:
        /// @dev - `t >= 0` : it is already a uint256 so this is always true
        /// @dev - `2t + 1 <= n` : the response consensus should not require more than `n` votes,
        /// @dev with `n` being the number of registered KMS nodes
        if (2 * newKmsThreshold + 1 > nKmsNodes) {
            revert InvalidHighKmsThreshold(newKmsThreshold, nKmsNodes);
        }

        $.kmsThreshold = newKmsThreshold;
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
