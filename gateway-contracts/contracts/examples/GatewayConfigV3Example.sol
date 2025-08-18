// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { Pausable } from "../shared/Pausable.sol";
import { ProtocolMetadata, KmsNodeV1, KmsNodeV2, Coprocessor, Custodian, HostChain } from "../shared/Structs.sol";

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
        address pauser;
        mapping(address kmsTxSenderAddress => bool isKmsTxSender) _isKmsTxSender;
        mapping(address kmsSignerAddress => bool isKmsSigner) _isKmsSigner;
        mapping(address coprocessorTxSenderAddress => bool isCoprocessorTxSender) _isCoprocessorTxSender;
        mapping(address coprocessorSignerAddress => bool isCoprocessorSigner) _isCoprocessorSigner;
        mapping(uint256 chainId => bool isRegistered) _isHostChainRegistered;
        ProtocolMetadata protocolMetadata; // deprecated, use protocolMetadataV2 instead
        mapping(address kmsTxSenderAddress => KmsNodeV1 kmsNode) kmsNodes;
        address[] kmsTxSenderAddresses;
        address[] kmsSignerAddresses;
        uint256 mpcThreshold;
        uint256 publicDecryptionThreshold;
        uint256 userDecryptionThreshold;
        mapping(address coprocessorTxSenderAddress => Coprocessor coprocessor) coprocessors;
        address[] coprocessorTxSenderAddresses;
        address[] coprocessorSignerAddresses;
        HostChain[] hostChains;
        mapping(address custodianTxSenderAddress => Custodian custodian) custodians;
        address[] custodianTxSenderAddresses;
        address[] custodianSignerAddresses;
        mapping(address custodianTxSenderAddress => bool isCustodianTxSender) _isCustodianTxSender;
        mapping(address custodianSignerAddress => bool isCustodianSigner) _isCustodianSigner;
        mapping(address kmsTxSenderAddress => KmsNodeV2 kmsNodeV2) kmsNodesV2;
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
