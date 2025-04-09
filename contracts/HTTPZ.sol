// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./interfaces/IHTTPZ.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title HTTPZ contract
 * @dev See {IHTTPZ}.
 * @dev Add/remove methods will be added in the future for KMS nodes, coprocessors and networks.
 * @dev See https://github.com/zama-ai/httpz-gateway/issues/98 for more details.
 */
contract HTTPZ is IHTTPZ, Ownable2StepUpgradeable, UUPSUpgradeable {
    /// @notice The maximum chain ID.
    uint256 public constant MAX_CHAIN_ID = type(uint64).max;

    /// @notice The contract's metadata
    string private constant CONTRACT_NAME = "HTTPZ";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:httpz_gateway.storage.HTTPZ
    struct HTTPZStorage {
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
        /// @notice The networks' registered status
        mapping(uint256 chainId => bool isRegistered) _isNetworkRegistered;
        /// @notice The protocol's metadata
        ProtocolMetadata protocolMetadata;
        /// @notice The KMS nodes' metadata
        mapping(address kmsTxSenderAddress => KmsNode kmsNode) kmsNodes;
        /// @notice The KMS nodes' transaction sender address list
        address[] kmsTxSenderAddresses;
        /// @notice The KMS nodes' signer address list
        address[] kmsSignerAddresses;
        /// @notice The KMS' threshold to consider for majority vote or reconstruction. For a set ot `n`
        /// @notice KMS nodes, the threshold `t` must verify `3t < n`.
        uint256 kmsThreshold;
        /// @notice The coprocessors' metadata
        mapping(address coprocessorTxSenderAddress => Coprocessor coprocessor) coprocessors;
        /// @notice The coprocessors' transaction sender address list
        address[] coprocessorTxSenderAddresses;
        /// @notice The coprocessors' signer address list
        address[] coprocessorSignerAddresses;
        /// @notice The networks' metadata
        Network[] networks;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("httpz_gateway.storage.HTTPZ")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant HTTPZ_STORAGE_LOCATION =
        0x827176a45e1aad1f3a6539fee60c06126c40427b4849e7301bf2cf0f1f8e9500;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract
    /// @param initialPauser Pauser address
    /// @param initialMetadata Metadata of the protocol
    /// @param initialKmsThreshold The KMS threshold. Must verify `3t < n` for `n` KMS nodes.
    /// @param initialKmsNodes List of KMS nodes
    /// @param initialCoprocessors List of coprocessors
    function initialize(
        address initialPauser,
        ProtocolMetadata memory initialMetadata,
        uint256 initialKmsThreshold,
        KmsNode[] memory initialKmsNodes,
        Coprocessor[] memory initialCoprocessors
    ) public reinitializer(2) {
        __Ownable_init(owner());

        HTTPZStorage storage $ = _getHTTPZStorage();
        $.protocolMetadata = initialMetadata;

        /// @dev Register the pauser
        $.pauser = initialPauser;

        uint256 nParties = initialKmsNodes.length;

        /// @dev Check that this KMS node's threshold is valid. For a set of `n` KMS nodes, the
        /// @dev threshold `t` must verify `0 <= t <= n`.
        if (initialKmsThreshold > nParties) {
            revert KmsThresholdTooHigh(initialKmsThreshold, nParties);
        }

        /// @dev Set the KMS threshold.
        $.kmsThreshold = initialKmsThreshold;

        /// @dev Register the KMS nodes
        for (uint256 i = 0; i < nParties; i++) {
            $._isKmsTxSender[initialKmsNodes[i].txSenderAddress] = true;
            $.kmsNodes[initialKmsNodes[i].txSenderAddress] = initialKmsNodes[i];
            $.kmsTxSenderAddresses.push(initialKmsNodes[i].txSenderAddress);
            $._isKmsSigner[initialKmsNodes[i].signerAddress] = true;
            $.kmsSignerAddresses.push(initialKmsNodes[i].signerAddress);
        }

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

    /// @dev See {IHTTPZ-updatePauser}.
    function updatePauser(address newPauser) external virtual onlyOwner {
        if (newPauser == address(0)) {
            revert InvalidNullPauser();
        }
        HTTPZStorage storage $ = _getHTTPZStorage();
        $.pauser = newPauser;
        emit UpdatePauser(newPauser);
    }

    /// @dev See {IHTTPZ-updateKmsThreshold}.
    function updateKmsThreshold(uint256 newKmsThreshold) external virtual onlyOwner {
        HTTPZStorage storage $ = _getHTTPZStorage();
        if (newKmsThreshold > $.kmsTxSenderAddresses.length) {
            revert KmsThresholdTooHigh(newKmsThreshold, $.kmsTxSenderAddresses.length);
        }

        $.kmsThreshold = newKmsThreshold;
        emit UpdateKmsThreshold(newKmsThreshold);
    }

    /// @dev See {IHTTPZ-checkIsPauser}.
    function checkIsPauser(address pauserAddress) external view virtual {
        HTTPZStorage storage $ = _getHTTPZStorage();
        if ($.pauser != pauserAddress) {
            revert NotPauser(pauserAddress);
        }
    }

    /// @dev See {IHTTPZ-checkIsKmsTxSender}.
    function checkIsKmsTxSender(address txSenderAddress) external view virtual {
        HTTPZStorage storage $ = _getHTTPZStorage();
        if (!$._isKmsTxSender[txSenderAddress]) {
            revert NotKmsTxSender(txSenderAddress);
        }
    }

    /// @dev See {IHTTPZ-checkIsKmsSigner}.
    function checkIsKmsSigner(address signerAddress) external view virtual {
        HTTPZStorage storage $ = _getHTTPZStorage();
        if (!$._isKmsSigner[signerAddress]) {
            revert NotKmsSigner(signerAddress);
        }
    }

    /// @dev See {IHTTPZ-checkIsCoprocessorTxSender}.
    function checkIsCoprocessorTxSender(address txSenderAddress) external view virtual {
        HTTPZStorage storage $ = _getHTTPZStorage();
        if (!$._isCoprocessorTxSender[txSenderAddress]) {
            revert NotCoprocessorTxSender(txSenderAddress);
        }
    }

    /// @dev See {IHTTPZ-checkIsCoprocessorSigner}.
    function checkIsCoprocessorSigner(address signerAddress) external view virtual {
        HTTPZStorage storage $ = _getHTTPZStorage();
        if (!$._isCoprocessorSigner[signerAddress]) {
            revert NotCoprocessorSigner(signerAddress);
        }
    }

    /// @dev See {IHTTPZ-checkNetworkIsRegistered}.
    function checkNetworkIsRegistered(uint256 chainId) external view virtual {
        HTTPZStorage storage $ = _getHTTPZStorage();
        if (!$._isNetworkRegistered[chainId]) {
            revert NetworkNotRegistered(chainId);
        }
    }

    /// @dev See {IHTTPZ-getProtocolMetadata}.
    function getProtocolMetadata() external view virtual returns (ProtocolMetadata memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.protocolMetadata;
    }

    /// @dev See {IHTTPZ-getKmsThreshold}.
    function getKmsThreshold() external view virtual returns (uint256) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.kmsThreshold;
    }

    /// @dev See {IHTTPZ-getKmsMajorityThreshold}.
    function getKmsMajorityThreshold() external view virtual returns (uint256) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.kmsThreshold + 1;
    }

    /// @dev See {IHTTPZ-getKmsReconstructionThreshold}.
    function getKmsReconstructionThreshold() external view virtual returns (uint256) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return 2 * $.kmsThreshold + 1;
    }

    /// @dev See {IHTTPZ-getCoprocessorMajorityThreshold}.
    function getCoprocessorMajorityThreshold() external view virtual returns (uint256) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.coprocessorTxSenderAddresses.length / 2 + 1;
    }

    /// @dev See {IHTTPZ-getKmsNode}.
    function getKmsNode(address kmsTxSenderAddress) external view virtual returns (KmsNode memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.kmsNodes[kmsTxSenderAddress];
    }

    /// @dev See {IHTTPZ-getKmsTxSenders}.
    function getKmsTxSenders() external view virtual returns (address[] memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.kmsTxSenderAddresses;
    }

    /// @dev See {IHTTPZ-getKmsSigners}.
    function getKmsSigners() external view virtual returns (address[] memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.kmsSignerAddresses;
    }

    /// @dev See {IHTTPZ-getCoprocessor}.
    function getCoprocessor(address coprocessorTxSenderAddress) external view virtual returns (Coprocessor memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.coprocessors[coprocessorTxSenderAddress];
    }

    /// @dev See {IHTTPZ-getCoprocessorTxSenders}.
    function getCoprocessorTxSenders() external view virtual returns (address[] memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.coprocessorTxSenderAddresses;
    }

    /// @dev See {IHTTPZ-getCoprocessorSigners}.
    function getCoprocessorSigners() external view virtual returns (address[] memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.coprocessorSignerAddresses;
    }

    /// @dev See {IHTTPZ-getNetwork}.
    function getNetwork(uint256 index) external view virtual returns (Network memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.networks[index];
    }

    /// @dev See {IHTTPZ-getNetworks}.
    function getNetworks() external view virtual returns (Network[] memory) {
        HTTPZStorage storage $ = _getHTTPZStorage();
        return $.networks;
    }

    /// @dev See {IHTTPZ-addNetwork}.
    function addNetwork(Network calldata network) external virtual {
        if (network.chainId == 0) {
            revert InvalidNullChainId();
        }
        if (network.chainId > MAX_CHAIN_ID) {
            revert ChainIdNotUint64(network.chainId);
        }

        HTTPZStorage storage $ = _getHTTPZStorage();
        if ($._isNetworkRegistered[network.chainId]) {
            revert NetworkAlreadyRegistered(network.chainId);
        }

        $.networks.push(network);
        $._isNetworkRegistered[network.chainId] = true;
        emit AddNetwork(network);
    }

    /// @notice Returns the versions of the HTTPZ contract in SemVer format.
    /// @dev This is conventionally used for upgrade features.
    function getVersion() public pure virtual returns (string memory) {
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
     * @dev Returns the HTTPZ storage location.
     */
    function _getHTTPZStorage() internal pure returns (HTTPZStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := HTTPZ_STORAGE_LOCATION
        }
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}
}
