// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import {
    gatewayConfigAddress,
    kmsGenerationAddress,
    coprocessorContextsAddress
} from "../addresses/GatewayAddresses.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { ICiphertextCommits } from "./interfaces/ICiphertextCommits.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { IKMSGeneration } from "./interfaces/IKMSGeneration.sol";
import { ICoprocessorContexts } from "./interfaces/ICoprocessorContexts.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayConfigChecks } from "./shared/GatewayConfigChecks.sol";
import { GatewayOwnable } from "./shared/GatewayOwnable.sol";
import { CiphertextMaterial, SnsCiphertextMaterial } from "./shared/Structs.sol";
import { ContextStatus } from "./shared/Enums.sol";
import { ContextChecks } from "./shared/ContextChecks.sol";

/**
 * @title CiphertextCommits smart contract
 * @notice See {ICiphertextCommits}.
 */
contract CiphertextCommits is
    ICiphertextCommits,
    UUPSUpgradeableEmptyProxy,
    GatewayOwnable,
    GatewayConfigChecks,
    ContextChecks
{
    /**
     * @notice The address of the GatewayConfig contract, used for fetching information about coprocessors.
     */
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /**
     * @notice The address of the KMSGeneration contract, used for fetching information about the current key.
     */
    IKMSGeneration private constant KMS_GENERATION = IKMSGeneration(kmsGenerationAddress);

    /// @notice The address of the CoprocessorContexts contract, used for fetching information about coprocessors.
    ICoprocessorContexts private constant COPROCESSOR_CONTEXTS = ICoprocessorContexts(coprocessorContextsAddress);

    /**
     * @dev The following constants are used for versioning the contract. They are made private
     * in order to force derived contracts to consider a different version. Note that
     * they can still define their own private constants with the same name.
     */
    string private constant CONTRACT_NAME = "CiphertextCommits";
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
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.CiphertextCommits
    struct CiphertextCommitsStorage {
        // ----------------------------------------------------------------------------------------------
        // Ciphertext material state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The key IDs used for generating the ciphertext.
        mapping(bytes32 ctHandle => uint256 keyId) keyIds;
        /// @notice The regular ciphertext digests tied to the ciphertext handle.
        mapping(bytes32 ctHandle => bytes32 ctDigest) ciphertextDigests;
        /// @notice The SNS ciphertext digests tied to the ciphertext handle.
        mapping(bytes32 ctHandle => bytes32 snsCtDigest) snsCiphertextDigests;
        // ----------------------------------------------------------------------------------------------
        // Consensus state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The mapping of already added ciphertexts tied to the given handle.
        mapping(bytes32 ctHandle => bool isAdded) isCiphertextMaterialAdded;
        /// @notice The counter of confirmations received for a ciphertext to be added.
        mapping(bytes32 addCiphertextHash => uint256 counter) addCiphertextHashCounters;
        /// @notice The digest of the ciphertext material addition that reached consensus for a handle.
        mapping(bytes32 ctHandle => bytes32 addCiphertextHash) ctHandleConsensusHash;
        // ----------------------------------------------------------------------------------------------
        // Transaction sender addresses state variables:
        // ----------------------------------------------------------------------------------------------
        // prettier-ignore
        /// @notice The mapping of the coprocessor transaction senders that have already added the ciphertext handle.
        mapping(bytes32 ctHandle => mapping(address coprocessorTxSenderAddress => bool hasAdded)) 
            alreadyAddedCoprocessorTxSenders;
        /// @notice The coprocessor transaction senders involved in a consensus for a ciphertext material addition.
        mapping(bytes32 addCiphertextHash => address[] coprocessorTxSenderAddresses) coprocessorTxSenderAddresses;
        // ----------------------------------------------------------------------------------------------
        // Coprocessor context state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The coprocessor context ID associated to the add ciphertext
        mapping(bytes32 addCiphertextHash => uint256 contextId) addCiphertextContextId;
    }

    /**
     * @dev Storage location has been computed using the following command:
     * keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.CiphertextCommits")) - 1))
     * & ~bytes32(uint256(0xff))
     */
    bytes32 private constant CIPHERTEXT_COMMITS_STORAGE_LOCATION =
        0xf41c60ea5b83c8f19b663613ffdd3fa441a59933b8a4fdf4da891b38433d1a00;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract.
     * @dev This function needs to be public in order to be called by the UUPS proxy.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice Re-initializes the contract from V1.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /// @notice See {ICiphertextCommits-addCiphertextMaterial}.
    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 keyId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) external virtual onlyHandleFromRegisteredHostChain(ctHandle) refreshCoprocessorContextStatuses {
        // The addCiphertextHash is the hash of all received input arguments which means that multiple
        // Coprocessors can only have a consensus on a ciphertext material with the same information.
        // This hash is used to differentiate different calls to the function, in particular when
        // tracking the consensus on the received ciphertext material.
        // Note that chainId is not included in the hash because it is already contained in the ctHandle.
        bytes32 addCiphertextHash = keccak256(abi.encode(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest));

        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();

        // Get the context ID from the input verification context ID mapping
        // This ID may be 0 (invalid) if this is the first addCiphertextMaterial call for this
        // addCiphertextHash (see right below)
        uint256 contextId = $.addCiphertextContextId[addCiphertextHash];

        // If the context ID is null, get the active coprocessor context's ID and associate it to
        // this ciphertext material addition
        if (contextId == 0) {
            contextId = COPROCESSOR_CONTEXTS.getActiveCoprocessorContextId();
            $.addCiphertextContextId[addCiphertextHash] = contextId;

            // Else, that means a coprocessor already started to add the ciphertext material
            // and we need to check that the context is active or suspended
            // If it is not, that means the context is no longer valid for this operation and we revert
        } else if (!COPROCESSOR_CONTEXTS.isCoprocessorContextOperating(contextId)) {
            ContextStatus contextStatus = COPROCESSOR_CONTEXTS.getCoprocessorContextStatus(contextId);
            revert InvalidCoprocessorContextAddCiphertext(ctHandle, contextId, contextStatus);
        }

        // Only accept coprocessor transaction senders from the same context
        _checkIsCoprocessorTxSender(contextId, msg.sender);

        // Check if the coprocessor transaction sender has already added the ciphertext handle.
        if ($.alreadyAddedCoprocessorTxSenders[ctHandle][msg.sender]) {
            revert CoprocessorAlreadyAdded(ctHandle, msg.sender);
        }

        $.addCiphertextHashCounters[addCiphertextHash]++;

        // Associate the handle to coprocessor context ID 1 to anticipate their introduction in V2.
        // Only set the context ID if it hasn't been set yet to avoid multiple identical SSTOREs.
        if ($.addCiphertextContextId[addCiphertextHash] == 0) {
            $.addCiphertextContextId[addCiphertextHash] = 1;
        }

        // It is ok to only the handle can be considered here as a handle should only be added once
        // in the contract anyway
        $.alreadyAddedCoprocessorTxSenders[ctHandle][msg.sender] = true;

        // Store the coprocessor transaction sender address for the ciphertext material addition
        // It's important to consider the hash and not the handle to make sure we only gather the
        // transaction senders associated to the same ciphertext material addition. This allows to
        // be able to retrieve all the transaction senders involved in a consensus
        // In particular, this means that a "late" (see right below) valid coprocessor transaction
        // sender address will still be added in the list
        $.coprocessorTxSenderAddresses[addCiphertextHash].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        // Besides, consensus only considers the coprocessors of the same context
        if (
            !$.isCiphertextMaterialAdded[ctHandle] &&
            _isConsensusReached(contextId, $.addCiphertextHashCounters[addCiphertextHash])
        ) {
            $.ciphertextDigests[ctHandle] = ciphertextDigest;
            $.snsCiphertextDigests[ctHandle] = snsCiphertextDigest;
            $.keyIds[ctHandle] = keyId;

            // A ciphertext handle should only be added once, ever
            $.isCiphertextMaterialAdded[ctHandle] = true;

            // A "late" valid coprocessor could still see its transaction sender address be added to
            // the list after consensus. This variable is here to be able to retrieve this list later
            // by only knowing the handle, since a consensus can only happen once per handle
            $.ctHandleConsensusHash[ctHandle] = addCiphertextHash;

            emit AddCiphertextMaterial(ctHandle, ciphertextDigest, snsCiphertextDigest, contextId);
        }
    }

    /**
     * @notice See {ICiphertextCommits-isCiphertextMaterialAdded}.
     */
    function isCiphertextMaterialAdded(bytes32 ctHandle) public view virtual returns (bool) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        return $.isCiphertextMaterialAdded[ctHandle];
    }

    /**
     * @notice See {ICiphertextCommits-getCiphertextMaterials}.
     */
    function getCiphertextMaterials(
        bytes32[] calldata ctHandles
    ) external view virtual returns (CiphertextMaterial[] memory ctMaterials) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        ctMaterials = new CiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            // Check that the consensus has been reached
            if (!isCiphertextMaterialAdded(ctHandles[i])) {
                revert CiphertextMaterialNotFound(ctHandles[i]);
            }

            ctMaterials[i] = CiphertextMaterial(
                ctHandles[i],
                $.keyIds[ctHandles[i]],
                $.ciphertextDigests[ctHandles[i]]
            );
        }

        return ctMaterials;
    }

    /**
     * @notice See {ICiphertextCommits-getSnsCiphertextMaterials}.
     */
    function getSnsCiphertextMaterials(
        bytes32[] calldata ctHandles
    ) external view virtual returns (SnsCiphertextMaterial[] memory snsCtMaterials) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        snsCtMaterials = new SnsCiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            // Check that the consensus has been reached
            if (!isCiphertextMaterialAdded(ctHandles[i])) {
                revert CiphertextMaterialNotFound(ctHandles[i]);
            }

            snsCtMaterials[i] = SnsCiphertextMaterial(
                ctHandles[i],
                $.keyIds[ctHandles[i]],
                $.snsCiphertextDigests[ctHandles[i]]
            );
        }

        return snsCtMaterials;
    }

    /**
     * @notice See {ICiphertextCommits-getConsensusTxSenders}.
     * The list remains empty until the consensus is reached.
     */
    function getConsensusTxSenders(bytes32 ctHandle) external view virtual returns (address[] memory) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();

        // Get the unique hash associated to the handle in order to retrieve the list of transaction
        // sender address that participated in the consensus
        // This digest remains the default value (0x0) until the consensus is reached.
        bytes32 addCiphertextHash = $.ctHandleConsensusHash[ctHandle];

        return $.coprocessorTxSenderAddresses[addCiphertextHash];
    }

    /**
     * @notice See {ICiphertextCommits-getConsensusStorageUrls}.
     */
    function getConsensusStorageUrls(bytes32[] calldata ctHandles) external view virtual returns (string[][] memory) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        string[][] memory consensusStorageUrls = new string[][](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            // Check that the consensus has been reached
            if (!isCiphertextMaterialAdded(ctHandles[i])) {
                revert CiphertextMaterialNotFound(ctHandles[i]);
            }

            // Get the unique hash associated to the handle in order to retrieve the list of transaction
            // sender address that participated in the consensus
            // This digest is null (0x0) for version V1.
            bytes32 addCiphertextHash = $.ctHandleConsensusHash[ctHandles[i]];

            // Get the transaction sender addresses and the context ID associated to the consensus
            // If the consensus has been reached but the hash is 0x0, it means that the handle has been
            // added in V1: the handle was used to retrieve the list of transaction sender addresses
            // instead of the hash, under the first context (`contextId=1`).
            // We therefore consider this in order to be backward compatible.
            // DEPRECATED: to remove in next state reset
            // See https://github.com/zama-ai/fhevm-internal/issues/471
            address[] memory coprocessorTxSenderAddresses;
            uint256 contextId;
            if (addCiphertextHash == bytes32(0)) {
                coprocessorTxSenderAddresses = $.coprocessorTxSenderAddresses[ctHandles[i]];
                contextId = 1;
            } else {
                coprocessorTxSenderAddresses = $.coprocessorTxSenderAddresses[addCiphertextHash];
                contextId = $.addCiphertextContextId[addCiphertextHash];
            }

            // Get the list of storage URLs associated to the transaction sender addresses
            string[] memory coprocessorStorageUrls = new string[](coprocessorTxSenderAddresses.length);
            for (uint256 j = 0; j < coprocessorTxSenderAddresses.length; j++) {
                coprocessorStorageUrls[j] = COPROCESSOR_CONTEXTS
                    .getCoprocessor(contextId, coprocessorTxSenderAddresses[j])
                    .storageUrl;
            }
            consensusStorageUrls[i] = coprocessorStorageUrls;
        }

        return consensusStorageUrls;
    }

    /**
     * @notice See {ICiphertextCommits-getVersion}.
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
     * @notice Checks if the sender is authorized to upgrade the contract and reverts otherwise.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}

    /**
     * @notice Checks if the consensus is reached among the coprocessors from the same context.
     * @param contextId The coprocessor context ID
     * @param coprocessorCounter The number of coprocessors that agreed
     * @return Whether the consensus is reached
     */
    function _isConsensusReached(uint256 contextId, uint256 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = COPROCESSOR_CONTEXTS.getCoprocessorMajorityThreshold(contextId);
        return coprocessorCounter >= consensusThreshold;
    }

    /**
     * @notice Returns the CiphertextCommits storage location.
     * @dev Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getCiphertextCommitsStorage() internal pure returns (CiphertextCommitsStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := CIPHERTEXT_COMMITS_STORAGE_LOCATION
        }
    }
}
