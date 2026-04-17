// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { IKMSGeneration } from "./interfaces/IKMSGeneration.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { gatewayConfigAddress } from "../addresses/GatewayAddresses.sol";
import { EIP712Upgradeable } from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayOwnable } from "./shared/GatewayOwnable.sol";

/**
 * @title KMSGeneration contract (view-only)
 * @notice View-only implementation of KMSGeneration for the Gateway chain.
 * All state-changing functions have been removed after the move of KMSGeneration to Ethereum.
 * This contract remains deployed for historical queries of previously generated keys and CRS
 * materials. Fresh deployments are no longer supported: only `reinitializeV5` is provided to
 * upgrade existing proxies to this view-only implementation.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
/// @custom:oz-upgrades-unsafe-allow missing-initializer
contract KMSGeneration is IKMSGeneration, EIP712Upgradeable, UUPSUpgradeableEmptyProxy, GatewayOwnable {
    // ----------------------------------------------------------------------------------------------
    // Other contract references:
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice The address of the GatewayConfig contract, used for resolving KMS node storage URLs.
     */
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    // ----------------------------------------------------------------------------------------------
    // Contract information:
    // ----------------------------------------------------------------------------------------------

    /**
     * @dev The following constants are used for versioning the contract. They are made private
     * in order to force derived contracts to consider a different version. Note that
     * they can still define their own private constants with the same name.
     */
    string private constant CONTRACT_NAME = "KMSGeneration";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 5;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Version used in the `reinitializer` modifier of the `reinitializeVX` method.
     */
    uint64 private constant REINITIALIZER_VERSION = 6;

    // ----------------------------------------------------------------------------------------------
    // Contract storage:
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     * @dev @deprecated. No longer written to. Preserved for historical queries and storage layout
     * compatibility after the move of KMSGeneration to Ethereum.
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.KMSGeneration
    struct KMSGenerationStorage {
        // ----------------------------------------------------------------------------------------------
        // Common consensus variables:
        // ----------------------------------------------------------------------------------------------
        mapping(uint256 requestId => mapping(address kmsSigner => bool hasSigned)) kmsHasSignedForResponse;
        mapping(uint256 requestId => bool hasConsensusAlreadyBeenReached) isRequestDone;
        mapping(uint256 requestId => mapping(bytes32 digest => address[] kmsTxSenderAddresses)) consensusTxSenderAddresses;
        mapping(uint256 requestId => bytes32 digest) consensusDigest;
        // ----------------------------------------------------------------------------------------------
        // Pre-processing keygen state variables:
        // ----------------------------------------------------------------------------------------------
        uint256 prepKeygenCounter;
        // ----------------------------------------------------------------------------------------------
        // Keygen state variables:
        // ----------------------------------------------------------------------------------------------
        uint256 keyCounter;
        mapping(uint256 id => uint256 pairedId) keygenIdPairs;
        mapping(uint256 keyId => KeyDigest[] keyDigests) keyDigests;
        uint256 activeKeyId;
        // ----------------------------------------------------------------------------------------------
        // Crsgen state variables:
        // ----------------------------------------------------------------------------------------------
        uint256 crsCounter;
        mapping(uint256 crsId => uint256 maxBitLength) crsMaxBitLength;
        mapping(uint256 crsId => bytes crsDigest) crsDigests;
        uint256 activeCrsId;
        // ----------------------------------------------------------------------------------------------
        // Parameters variables:
        // ----------------------------------------------------------------------------------------------
        mapping(uint256 requestId => ParamsType paramsType) requestParamsType;
        uint256 keyReshareCounter;
    }

    /**
     * @dev Storage location has been computed using the following command:
     * keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.KMSGeneration")) - 1))
     * & ~bytes32(uint256(0xff))
     */
    bytes32 private constant KMS_GENERATION_STORAGE_LOCATION =
        0x0b8fdb1f0a6356dd20a6cbc6f9668fac23b85f96575d10e333e603faa794ac00;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Re-initializes the contract to the view-only version.
     * @dev This is an empty reinitializer that marks the upgrade to the view-only implementation.
     * No state changes are needed since the storage layout is preserved.
     * Fresh deployments are no longer supported: KMSGeneration now lives on Ethereum and this
     * contract only remains on the Gateway chain to serve historical queries on existing proxies.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV5() public virtual reinitializer(REINITIALIZER_VERSION) {}

    // ----------------------------------------------------------------------------------------------
    // View functions (historical state access):
    // ----------------------------------------------------------------------------------------------
    // All functions below only expose state that was written before KMSGeneration was moved to
    // Ethereum. For up-to-date keys, CRS and protocol parameters, query the ProtocolConfig and
    // KMSGeneration contracts deployed on Ethereum instead.
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice See {IKMSGeneration-getKeyParamsType}.
     * @dev Historical access only. For current parameters, query ProtocolConfig / KMSGeneration on Ethereum.
     */
    function getKeyParamsType(uint256 keyId) external view virtual returns (ParamsType) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        if (!$.isRequestDone[keyId]) {
            revert KeyNotGenerated(keyId);
        }

        // Get the prepKeygenId associated to the keyId
        uint256 prepKeygenId = $.keygenIdPairs[keyId];

        return $.requestParamsType[prepKeygenId];
    }

    /**
     * @notice See {IKMSGeneration-getCrsParamsType}.
     * @dev Historical access only. For current parameters, query ProtocolConfig / KMSGeneration on Ethereum.
     */
    function getCrsParamsType(uint256 crsId) external view virtual returns (ParamsType) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        if (!$.isRequestDone[crsId]) {
            revert CrsNotGenerated(crsId);
        }

        return $.requestParamsType[crsId];
    }

    /**
     * @notice See {IKMSGeneration-getConsensusTxSenders}.
     * The returned list remains empty until the consensus is reached.
     * @dev Historical access only. For current consensus data, query KMSGeneration on Ethereum.
     */
    function getConsensusTxSenders(uint256 requestId) external view virtual returns (address[] memory) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Get the unique digest associated to the request in order to retrieve the list of
        // KMS transaction sender addresses that were involved in the associated consensus
        // This digest remains the default value (0x0) until the consensus is reached, meaning
        // that the returned list remains empty until then.
        // Each requestId is unique across all request types.
        bytes32 digest = $.consensusDigest[requestId];

        return $.consensusTxSenderAddresses[requestId][digest];
    }

    /**
     * @notice See {IKMSGeneration-getKeyMaterials}.
     * @dev Historical access only. For current key materials, query KMSGeneration on Ethereum.
     */
    function getKeyMaterials(uint256 keyId) external view virtual returns (string[] memory, KeyDigest[] memory) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        if (!$.isRequestDone[keyId]) {
            revert KeyNotGenerated(keyId);
        }
        bytes32 digest = $.consensusDigest[keyId];
        address[] memory consensusTxSenders = $.consensusTxSenderAddresses[keyId][digest];
        uint256 consensusTxSendersLength = consensusTxSenders.length;

        string[] memory consensusUrls = new string[](consensusTxSendersLength);
        for (uint256 i = 0; i < consensusTxSendersLength; i++) {
            consensusUrls[i] = GATEWAY_CONFIG.getKmsNode(consensusTxSenders[i]).storageUrl;
        }

        return (consensusUrls, $.keyDigests[keyId]);
    }

    /**
     * @notice See {IKMSGeneration-getCrsMaterials}.
     * @dev Historical access only. For current CRS materials, query KMSGeneration on Ethereum.
     */
    function getCrsMaterials(uint256 crsId) external view virtual returns (string[] memory, bytes memory) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        if (!$.isRequestDone[crsId]) {
            revert CrsNotGenerated(crsId);
        }
        bytes32 digest = $.consensusDigest[crsId];
        address[] memory consensusTxSenders = $.consensusTxSenderAddresses[crsId][digest];
        uint256 consensusTxSendersLength = consensusTxSenders.length;

        string[] memory consensusUrls = new string[](consensusTxSendersLength);
        for (uint256 i = 0; i < consensusTxSendersLength; i++) {
            consensusUrls[i] = GATEWAY_CONFIG.getKmsNode(consensusTxSenders[i]).storageUrl;
        }

        return (consensusUrls, $.crsDigests[crsId]);
    }

    /**
     * @notice See {IKMSGeneration-getVersion}.
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

    // ----------------------------------------------------------------------------------------------
    // Internal functions:
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice Checks if the sender is authorized to upgrade the contract and reverts otherwise.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}

    /**
     * @notice Returns the KMSGeneration storage location.
     * @dev Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getKMSGenerationStorage() internal pure returns (KMSGenerationStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := KMS_GENERATION_STORAGE_LOCATION
        }
    }
}
