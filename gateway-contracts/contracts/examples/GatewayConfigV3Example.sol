// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "../shared/Pausable.sol";

contract GatewayConfigV3Example is Ownable2StepUpgradeable, UUPSUpgradeable, Pausable {
    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "GatewayConfig";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 1001;
    uint256 private constant MINOR_VERSION = 0;
    uint256 private constant PATCH_VERSION = 0;

    struct ProtocolMetadataV2 {
        string name;
        string website;
        string newField;
    }

    /// @custom:storage-location erc7201:fhevm_gateway.storage.GatewayConfig
    struct GatewayConfigStorage {
        /// @notice DEPRECATED, use protocolMetadataV2 instead
        ProtocolMetadata protocolMetadata; // DEPRECATED
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
        // ----------------------------------------------------------------------------------------------
        // Protocol metadata state variables:
        // ----------------------------------------------------------------------------------------------
        // New state variables added in the upgraded version
        ProtocolMetadataV2 protocolMetadataV2;
    }

    bytes32 private constant GATEWAY_CONFIG_STORAGE_LOCATION =
        0x86d3070a8993f6b209bee6185186d38a07fce8bbd97c750d934451b72f35b400;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @custom:oz-upgrades-validate-as-initializer
    function initialize(string calldata newField) public virtual reinitializer(1000) {
        __Ownable_init(owner());
        __Pausable_init();

        // Execute the migration logic to set the new field in the protocol metadata
        _migrate(newField);
    }

    function getProtocolMetadata() external view virtual returns (ProtocolMetadataV2 memory) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.protocolMetadataV2;
    }

    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
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

    function _migrate(string calldata newField) private {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        $.protocolMetadataV2 = ProtocolMetadataV2({
            name: $.protocolMetadata.name,
            website: $.protocolMetadata.website,
            newField: newField
        });
    }

    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override {}

    function _getGatewayConfigStorage() internal pure returns (GatewayConfigStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := GATEWAY_CONFIG_STORAGE_LOCATION
        }
    }
}
