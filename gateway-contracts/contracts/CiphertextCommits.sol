// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import {
    gatewayConfigAddress,
    kmsManagementAddress,
    coprocessorContextsAddress
} from "../addresses/GatewayAddresses.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/ICiphertextCommits.sol";
import "./interfaces/IGatewayConfig.sol";
import { ICoprocessorContexts } from "./interfaces/ICoprocessorContexts.sol";
import "./interfaces/IKmsManagement.sol";
import "./shared/UUPSUpgradeableEmptyProxy.sol";
import "./shared/GatewayConfigChecks.sol";
import "./libraries/HandleOps.sol";
import "./shared/Pausable.sol";
import { ContextChecks } from "./shared/ContextChecks.sol";

/**
 * @title CiphertextCommits smart contract
 * @dev See {ICiphertextCommits}.
 */
contract CiphertextCommits is
    ICiphertextCommits,
    Ownable2StepUpgradeable,
    UUPSUpgradeableEmptyProxy,
    GatewayConfigChecks,
    Pausable,
    ContextChecks
{
    /// @notice The address of the GatewayConfig contract, used for fetching information about host chains.
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /// @notice The address of the CoprocessorContexts contract, used for fetching information about coprocessors.
    ICoprocessorContexts private constant COPROCESSOR_CONTEXTS = ICoprocessorContexts(coprocessorContextsAddress);

    /// @notice The address of the KmsManagement contract, used for fetching information about the current key.
    IKmsManagement private constant KMS_MANAGEMENT = IKmsManagement(kmsManagementAddress);

    /// @dev The following constants are used for versioning the contract. They are made private
    /// @dev in order to force derived contracts to consider a different version. Note that
    /// @dev they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "CiphertextCommits";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 4;
    uint256 private constant PATCH_VERSION = 0;

    /// Constant used for making sure the version number using in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 5;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:fhevm_gateway.storage.CiphertextCommits
    struct CiphertextCommitsStorage {
        /// @notice The regular ciphertext digests tied to the ciphertext handle.
        mapping(bytes32 ctHandle => bytes32 ctDigest) _ciphertextDigests;
        /// @notice The SNS ciphertext digests tied to the ciphertext handle.
        mapping(bytes32 ctHandle => bytes32 snsCtDigest) _snsCiphertextDigests;
        /// @notice The key IDs used for generating the ciphertext.
        /// @dev It's necessary in case new keys are generated: we need to know what key to use for using a ciphertext.
        mapping(bytes32 ctHandle => uint256 keyId) _keyIds;
        /// @notice DEPRECATED
        mapping(bytes32 ctHandle => uint256 chainId) _chainIds; // DEPRECATED
        /// @notice The mapping of already added ciphertexts tied to the given handle.
        mapping(bytes32 ctHandle => bool isAdded) _isCiphertextMaterialAdded;
        /// @notice The counter of confirmations received for a ciphertext to be added.
        mapping(bytes32 addCiphertextHash => uint8 counter) _addCiphertextHashCounters;
        // prettier-ignore
        /// @notice The mapping of the coprocessor transaction senders that have already added the ciphertext handle.
        mapping(bytes32 ctHandle => mapping(address coprocessorTxSenderAddress => bool hasAdded)) 
            _alreadyAddedCoprocessorTxSenders;
        // ----------------------------------------------------------------------------------------------
        // Transaction sender addresses from consensus state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The coprocessor transaction senders involved in a consensus for a ciphertext material addition.
        mapping(bytes32 addCiphertextHash => address[] coprocessorTxSenderAddresses) _coprocessorTxSenderAddresses;
        /// @notice The digest of the ciphertext material addition that reached consensus for a handle.
        mapping(bytes32 ctHandle => bytes32 addCiphertextHash) _ctHandleConsensusHash;
        /// @notice The coprocessor context ID associated to the add ciphertext
        mapping(bytes32 addCiphertextHash => uint256 contextId) addCiphertextContextId;
        /// @notice The S3 bucket URLs that have reached consensus for a ciphertext material addition.
        mapping(bytes32 addCiphertextHash => string[] coprocessorS3BucketUrls) consensusCoprocessorS3BucketUrls;
    }

    /// @dev Storage location has been computed using the following command:
    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.CiphertextCommits")) - 1)) &
    /// @dev ~bytes32(uint256(0xff))
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
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __Ownable_init(owner());
        __Pausable_init();
    }

    /**
     * @notice Re-initializes the contract from V3.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV4() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /// @notice See {ICiphertextCommits-addCiphertextMaterial}.
    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 keyId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) external virtual refreshCoprocessorContextStatuses {
        // Extract the chainId from the ciphertext handle
        uint256 chainId = HandleOps.extractChainId(ctHandle);

        // Check that the associated host chain is registered
        GATEWAY_CONFIG.checkHostChainIsRegistered(chainId);

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
        } else if (!COPROCESSOR_CONTEXTS.isCoprocessorContextActiveOrSuspended(contextId)) {
            ContextStatus contextStatus = COPROCESSOR_CONTEXTS.getCoprocessorContextStatus(contextId);
            revert InvalidCoprocessorContextAddCiphertext(ctHandle, contextId, contextStatus);
        }

        // Only accept coprocessor transaction senders from the same context
        COPROCESSOR_CONTEXTS.checkIsCoprocessorTxSenderFromContext(contextId, msg.sender);

        // Check if the coprocessor transaction sender has already added the ciphertext handle.
        if ($._alreadyAddedCoprocessorTxSenders[ctHandle][msg.sender]) {
            revert CoprocessorAlreadyAdded(ctHandle, msg.sender);
        }

        // Check if the received key ID is the latest activated.
        // TODO: Revisit the following line accordingly with key life-cycles issue
        // See: https://github.com/zama-ai/fhevm-gateway/issues/90
        // TODO: Re-enable this check once keys are generated through the Gateway
        // KMS_MANAGEMENT.checkCurrentKeyId(keyId);

        $._addCiphertextHashCounters[addCiphertextHash]++;

        // It is ok to only the handle can be considered here as a handle should only be added once
        // in the contract anyway
        $._alreadyAddedCoprocessorTxSenders[ctHandle][msg.sender] = true;

        // Store the coprocessor transaction sender address for the ciphertext material addition
        // It's important to consider the hash and not the handle to make sure we only gather the
        // transaction senders associated to the same ciphertext material addition. This allows to
        // be able to retrieve all the transaction senders involved in a consensus
        // In particular, this means that a "late" (see right below) valid coprocessor transaction
        // sender address will still be added in the list
        $._coprocessorTxSenderAddresses[addCiphertextHash].push(msg.sender);

        // Get and store the coprocessor's S3 bucket URL for the ciphertext material addition
        // The comment above regarding the coprocessor transaction sender addresses also applies here.
        string memory coprocessorS3BucketUrl = COPROCESSOR_CONTEXTS
            .getCoprocessorFromContext(contextId, msg.sender)
            .s3BucketUrl;
        $.consensusCoprocessorS3BucketUrls[addCiphertextHash].push(coprocessorS3BucketUrl);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        // Besides, consensus only considers the coprocessors of the same context
        if (
            !$._isCiphertextMaterialAdded[ctHandle] &&
            _isConsensusReached(contextId, $._addCiphertextHashCounters[addCiphertextHash])
        ) {
            $._ciphertextDigests[ctHandle] = ciphertextDigest;
            $._snsCiphertextDigests[ctHandle] = snsCiphertextDigest;
            $._keyIds[ctHandle] = keyId;

            // A ciphertext handle should only be added once, ever
            $._isCiphertextMaterialAdded[ctHandle] = true;

            // A "late" valid coprocessor could still see its transaction sender address be added to
            // the list after consensus. This variable is here to be able to retrieve this list later
            // by only knowing the handle, since a consensus can only happen once per handle
            $._ctHandleConsensusHash[ctHandle] = addCiphertextHash;

            emit AddCiphertextMaterial(ctHandle, ciphertextDigest, snsCiphertextDigest, contextId);
        }
    }

    /// @notice See {ICiphertextCommits-getCiphertextMaterials}.
    function getCiphertextMaterials(
        bytes32[] calldata ctHandles
    ) external view virtual returns (CiphertextMaterial[] memory ctMaterials) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        ctMaterials = new CiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            // Check that the consensus has been reached
            checkCiphertextMaterial(ctHandles[i]);

            ctMaterials[i] = CiphertextMaterial(
                ctHandles[i],
                $._keyIds[ctHandles[i]],
                $._ciphertextDigests[ctHandles[i]]
            );
        }

        return ctMaterials;
    }

    /// @notice See {ICiphertextCommits-getSnsCiphertextMaterials}.
    function getSnsCiphertextMaterials(
        bytes32[] calldata ctHandles
    ) external view virtual returns (SnsCiphertextMaterial[] memory snsCtMaterials) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        snsCtMaterials = new SnsCiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            // Check that the consensus has been reached
            checkCiphertextMaterial(ctHandles[i]);

            snsCtMaterials[i] = SnsCiphertextMaterial(
                ctHandles[i],
                $._keyIds[ctHandles[i]],
                $._snsCiphertextDigests[ctHandles[i]]
            );
        }

        return snsCtMaterials;
    }

    /**
     * @dev See {ICiphertextCommits-getConsensusTxSenders}.
     * The list remains empty until the consensus is reached.
     */
    function getConsensusTxSenders(bytes32 ctHandle) external view virtual returns (address[] memory) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();

        // Get the unique hash associated to the handle in order to retrieve the list of transaction
        // sender address that participated in the consensus
        // This digest remains the default value (0x0) until the consensus is reached.
        bytes32 addCiphertextHash = $._ctHandleConsensusHash[ctHandle];

        // If the consensus has been reached but the hash is 0x0, it means that the handle has been
        // added in V1: the handle was used to retrieve the list of transaction sender addresses
        // instead of the hash
        // We therefore consider this in order to be backward compatible.
        // DEPRECATED: to remove before mainnet
        // See https://github.com/zama-ai/fhevm-internal/issues/381
        if (addCiphertextHash == bytes32(0) && $._isCiphertextMaterialAdded[ctHandle]) {
            return $._coprocessorTxSenderAddresses[ctHandle];
        }

        return $._coprocessorTxSenderAddresses[addCiphertextHash];
    }

    function getConsensusS3BucketUrls(bytes32[] calldata ctHandles) external view virtual returns (string[][] memory) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        string[][] memory s3BucketUrls = new string[][](ctHandles.length);
        for (uint256 i = 0; i < ctHandles.length; i++) {
            // Check that the consensus has been reached
            checkCiphertextMaterial(ctHandles[i]);

            // Get the unique hash associated to the handle in order to retrieve the list of transaction
            // sender address that participated in the consensus
            // This digest is null (0x0) for version V1.
            bytes32 addCiphertextHash = $._ctHandleConsensusHash[ctHandles[i]];

            // Get the list of S3 bucket URLs that have reached consensus for the ciphertext
            // This list is empty for versions < V4.
            s3BucketUrls[i] = $.consensusCoprocessorS3BucketUrls[addCiphertextHash];

            // If the consensus has been reached but the list is empty, it means that the handle has been
            // added in versions < V4 and s3 bucket URLs were not stored in the contract
            // We therefore consider this in order to be backward compatible.
            // DEPRECATED: to remove before mainnet
            // See https://github.com/zama-ai/fhevm-internal/issues/381
            if (s3BucketUrls[i].length == 0) {
                // If the consensus has been reached but the hash is 0x0, it means that the handle has been
                // added in V1: the handle was used to retrieve the list of transaction sender addresses
                // instead of the hash
                // We therefore consider this in order to be backward compatible.
                // DEPRECATED: to remove before mainnet
                // See https://github.com/zama-ai/fhevm-internal/issues/381
                address[] memory coprocessorTxSenderAddresses;
                if (addCiphertextHash == bytes32(0)) {
                    coprocessorTxSenderAddresses = $._coprocessorTxSenderAddresses[ctHandles[i]];
                } else {
                    coprocessorTxSenderAddresses = $._coprocessorTxSenderAddresses[addCiphertextHash];
                }

                // Get the list of S3 bucket URL for each coprocessor from the first context (`contextId=1`),
                // as versions < V4 were only using a single context.
                // DEPRECATED: to remove before mainnet
                // See https://github.com/zama-ai/fhevm-internal/issues/381
                string[] memory s3BucketUrlsPerCoprocessor = new string[](coprocessorTxSenderAddresses.length);
                for (uint256 j = 0; j < coprocessorTxSenderAddresses.length; j++) {
                    string memory s3BucketUrl = COPROCESSOR_CONTEXTS
                        .getCoprocessorFromContext(1, coprocessorTxSenderAddresses[j])
                        .s3BucketUrl;
                    s3BucketUrlsPerCoprocessor[j] = s3BucketUrl;
                }
                s3BucketUrls[i] = s3BucketUrlsPerCoprocessor;
            }
        }
        return s3BucketUrls;
    }

    /// @notice See {ICiphertextCommits-getVersion}.
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

    /// @notice See {ICiphertextCommits-checkCiphertextMaterial}.
    function checkCiphertextMaterial(bytes32 ctHandle) public view virtual {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        if (!$._isCiphertextMaterialAdded[ctHandle]) {
            revert CiphertextMaterialNotFound(ctHandle);
        }
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
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
        uint256 consensusThreshold = COPROCESSOR_CONTEXTS.getCoprocessorMajorityThresholdFromContext(contextId);
        return coprocessorCounter >= consensusThreshold;
    }

    /**
     * @dev Returns the CiphertextCommits storage location.
     * Note that this function is internal but not virtual: derived contracts should be able to
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
