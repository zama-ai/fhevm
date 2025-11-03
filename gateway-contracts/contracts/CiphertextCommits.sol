// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import { gatewayConfigAddress, kmsGenerationAddress } from "../addresses/GatewayAddresses.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { ICiphertextCommits } from "./interfaces/ICiphertextCommits.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { IKMSGeneration } from "./interfaces/IKMSGeneration.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayConfigChecks } from "./shared/GatewayConfigChecks.sol";
import { GatewayOwnable } from "./shared/GatewayOwnable.sol";
import { CiphertextMaterial, SnsCiphertextMaterial } from "./shared/Structs.sol";

/**
 * @title CiphertextCommits smart contract
 * @notice See {ICiphertextCommits}.
 */
contract CiphertextCommits is ICiphertextCommits, UUPSUpgradeableEmptyProxy, GatewayOwnable, GatewayConfigChecks {
    /**
     * @notice The address of the GatewayConfig contract, used for fetching information about coprocessors.
     */
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /**
     * @notice The address of the KMSGeneration contract, used for fetching information about the current key.
     */
    IKMSGeneration private constant KMS_GENERATION = IKMSGeneration(kmsGenerationAddress);

    /**
     * @notice The domain separator for the add ciphertext hash.
     */
    bytes32 private constant ADD_CIPHERTEXT_DOMAIN_SEPARATOR_HASH =
        keccak256(bytes("CiphertextCommits.addCiphertextMaterial"));

    /**
     * @dev The following constants are used for versioning the contract. They are made private
     * in order to force derived contracts to consider a different version. Note that
     * they can still define their own private constants with the same name.
     */
    string private constant CONTRACT_NAME = "CiphertextCommits";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     * This constant does not represent the number of time a specific contract have been upgraded,
     * as a contract deployed from version VX will have a REINITIALIZER_VERSION > 2.
     */
    uint64 private constant REINITIALIZER_VERSION = 2;

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
     * @dev Define a `reinitializeVX` function once the contract needs to be upgraded.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    // function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice See {ICiphertextCommits-addCiphertextMaterial}.
     */
    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 keyId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) external virtual onlyCoprocessorTxSender onlyHandleFromRegisteredHostChain(ctHandle) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();

        // Check if the coprocessor transaction sender has already added the ciphertext handle.
        if ($.alreadyAddedCoprocessorTxSenders[ctHandle][msg.sender]) {
            revert CoprocessorAlreadyAdded(ctHandle, msg.sender);
        }

        // The addCiphertextHash is the hash of all received input arguments which means that multiple
        // Coprocessors can only have a consensus on a ciphertext material with the same information.
        // This hash is used to differentiate different calls to the function, in particular when
        // tracking the consensus on the received ciphertext material.
        // Note that chainId is not included in the hash because it is already contained in the ctHandle.
        bytes32 addCiphertextHash = _getAddCiphertextHash(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);
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
        if (
            !$.isCiphertextMaterialAdded[ctHandle] &&
            _isConsensusReached($.addCiphertextHashCounters[addCiphertextHash])
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

            emit AddCiphertextMaterial(
                ctHandle,
                ciphertextDigest,
                snsCiphertextDigest,
                $.coprocessorTxSenderAddresses[addCiphertextHash]
            );
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

            // Get the unique hash associated to the handle and use it to get the list of coprocessor
            // transaction sender address that were involved in the consensus
            bytes32 addCiphertextHash = $.ctHandleConsensusHash[ctHandles[i]];
            address[] memory coprocessorTxSenderAddresses = $.coprocessorTxSenderAddresses[addCiphertextHash];

            ctMaterials[i] = CiphertextMaterial(
                ctHandles[i],
                $.keyIds[ctHandles[i]],
                $.ciphertextDigests[ctHandles[i]],
                coprocessorTxSenderAddresses
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

            // Get the unique hash associated to the handle and use it to get the list of coprocessor
            // transaction sender address that were involved in the consensus
            bytes32 addCiphertextHash = $.ctHandleConsensusHash[ctHandles[i]];
            address[] memory coprocessorTxSenderAddresses = $.coprocessorTxSenderAddresses[addCiphertextHash];

            snsCtMaterials[i] = SnsCiphertextMaterial(
                ctHandles[i],
                $.keyIds[ctHandles[i]],
                $.snsCiphertextDigests[ctHandles[i]],
                coprocessorTxSenderAddresses
            );
        }

        return snsCtMaterials;
    }

    /**
     * @notice See {ICiphertextCommits-getAddCiphertextMaterialConsensusTxSenders}.
     * The list remains empty until the consensus is reached.
     */
    function getAddCiphertextMaterialConsensusTxSenders(
        bytes32 ctHandle
    ) external view virtual returns (address[] memory) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();

        // Get the unique hash associated to the handle in order to retrieve the list of transaction
        // sender address that participated in the consensus
        // This digest remains the default value (0x0) until the consensus is reached.
        bytes32 addCiphertextHash = $.ctHandleConsensusHash[ctHandle];

        return $.coprocessorTxSenderAddresses[addCiphertextHash];
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
     * @notice Checks if the consensus is reached among the Coprocessors.
     * @param coprocessorCounter The number of coprocessors that agreed
     * @return Whether the consensus is reached
     */
    function _isConsensusReached(uint256 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = GATEWAY_CONFIG.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }

    /**
     * @notice Returns the hash of a add ciphertext hash.
     */
    function _getAddCiphertextHash(
        bytes32 ctHandle,
        uint256 keyId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) internal pure virtual returns (bytes32) {
        return
            keccak256(
                abi.encode(ADD_CIPHERTEXT_DOMAIN_SEPARATOR_HASH, ctHandle, keyId, ciphertextDigest, snsCiphertextDigest)
            );
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
