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
 * All state-changing functions have been removed as part of RFC 013 (Move Key Generation
 * to Ethereum). This contract remains deployed for historical queries of previously
 * generated keys and CRS materials.
 */
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
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier
     * is identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     */
    uint64 private constant REINITIALIZER_VERSION = 6;

    // ----------------------------------------------------------------------------------------------
    // Contract storage:
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.KMSGeneration
    struct KMSGenerationStorage {
        // ----------------------------------------------------------------------------------------------
        // Common consensus variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice Whether a KMS node has signed for a response
        /// @Deprecated. No longer written to.
        mapping(uint256 requestId => mapping(address kmsSigner => bool hasSigned)) kmsHasSignedForResponse;
        /// @notice Whether a request has reached consensus
        mapping(uint256 requestId => bool hasConsensusAlreadyBeenReached) isRequestDone;
        /// @notice The KMS transaction sender addresses that propagated valid signatures for a request
        mapping(uint256 requestId => mapping(bytes32 digest => address[] kmsTxSenderAddresses)) consensusTxSenderAddresses;
        /// @notice The digest of the signed struct on which consensus was reached for a request
        mapping(uint256 requestId => bytes32 digest) consensusDigest;
        // ----------------------------------------------------------------------------------------------
        // Pre-processing keygen state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The number of preprocessing keygen, used to generate the prepKeygenIds.
        /// @Deprecated. No longer written to.
        uint256 prepKeygenCounter;
        // ----------------------------------------------------------------------------------------------
        // Keygen state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The number of keygen, used to generate the keyIds.
        /// @Deprecated. No longer written to.
        uint256 keyCounter;
        /// @notice Bidirectional mapping between preprocessing request IDs and key IDs
        mapping(uint256 id => uint256 pairedId) keygenIdPairs;
        /// @notice The digests of the generated keys
        mapping(uint256 keyId => KeyDigest[] keyDigests) keyDigests;
        /// @notice The ID of the currently active key
        /// @Deprecated. No longer written to.
        uint256 activeKeyId;
        // ----------------------------------------------------------------------------------------------
        // Crsgen state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The number of crsgen, used to generate the crsIds.
        /// @Deprecated. No longer written to.
        uint256 crsCounter;
        /// @notice The max bit length used for the CRS generation
        /// @Deprecated. No longer written to.
        mapping(uint256 crsId => uint256 maxBitLength) crsMaxBitLength;
        /// @notice The digests of the generated CRS
        mapping(uint256 crsId => bytes crsDigest) crsDigests;
        /// @notice The ID of the currently active CRS
        /// @Deprecated. No longer written to.
        uint256 activeCrsId;
        // ----------------------------------------------------------------------------------------------
        // Parameters variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The parameters type used for the request
        mapping(uint256 requestId => ParamsType paramsType) requestParamsType;
        /// @notice The number of key resharing, used to generate the keyReshareIds.
        /// @Deprecated. No longer written to.
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
     * @notice Initializes the contract.
     * @dev This function needs to be public in order to be called by the UUPS proxy.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __EIP712_init(CONTRACT_NAME, "1");
    }

    /**
     * @notice Re-initializes the contract to the view-only version.
     * @dev This is an empty reinitializer that marks the upgrade to the view-only implementation.
     * No state changes are needed since the storage layout is preserved.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV5() public virtual reinitializer(REINITIALIZER_VERSION) {}

    // ----------------------------------------------------------------------------------------------
    // View functions (historical state access):
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice See {IKMSGeneration-getKeyParamsType}.
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
