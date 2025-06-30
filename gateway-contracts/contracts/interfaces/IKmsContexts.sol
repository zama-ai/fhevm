// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {
    KmsNode,
    KmsContext,
    Coprocessor,
    HostChain,
    ProtocolMetadata,
    DecryptionThresholds,
    KmsBlockPeriods
} from "../shared/Structs.sol";
import { ContextStatus } from "../shared/Enums.sol";
/**
 * @title Interface for the KmsContexts contract.
 * @notice The KmsContexts contract is responsible for being a point of truth for all contracts and
 * components from the fhevm Gateway.
 * @dev In particular, the KmsContexts contract contains:
 * - the list of KMS nodes used exclusively by this fhevm Gateway
 * - the list of coprocessors used exclusively by this fhevm Gateway
 * - the list of host chains using this fhevm Gateway
 *
 * The KmsContexts contract has an owner.
 * The owner can call some restricted functions, such as adding or removing KMS nodes, coprocessors
 * and host chains.
 * Some view functions are accessible to everyone (ex: getting the number of KMS nodes).
 */
interface IKmsContexts {
    /**
     * @notice Emitted when the KmsContexts initialization is completed.
     * @param decryptionThresholds The decryption thresholds for the KMS context
     * @param softwareVersion The software version of the KMS context
     * @param mpcThreshold The MPC threshold for the KMS context
     * @param kmsNodes The KMS nodes for the KMS context
     */
    event InitializeKmsContexts(
        DecryptionThresholds decryptionThresholds,
        bytes8 softwareVersion,
        uint256 mpcThreshold,
        KmsNode[] kmsNodes
    );

    /**
     * @notice Emitted when the public decryption threshold has been updated.
     * @param newPublicDecryptionThreshold The new public decryption threshold.
     */
    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);

    /**
     * @notice Emitted when the user decryption threshold has been updated.
     * @param newUserDecryptionThreshold The new user decryption threshold.
     */
    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);

    /**
     * @notice Emitted when a new KMS context has been registered.
     * @param activeKmsContext The current active KMS context.
     * @param newKmsContext The new KMS context.
     * @param blockPeriods The block periods.
     */
    event NewKmsContext(KmsContext activeKmsContext, KmsContext newKmsContext, KmsBlockPeriods blockPeriods);

    /**
     * @notice Emitted when a key resharing needs to be done among the new KMS nodes.
     * @param activeKmsContext The current active KMS context.
     * @param newKmsContext The new KMS context.
     * @param generationBlockNumber The block number at which the key resharing will be invalidated if
     * all the KMS nodes have not validated the key resharing.
     */
    event StartKeyResharing(KmsContext activeKmsContext, KmsContext newKmsContext, uint256 generationBlockNumber);

    /**
     * @notice Emitted when a key resharing has been validated by all the KMS nodes.
     * @param newKmsContext The new KMS context.
     */
    event ValidateKeyResharing(KmsContext newKmsContext);

    event InvalidateKeyResharing(uint256 contextId);

    /**
     * @notice Emitted when a new KMS context is being pre-activated.
     * @param newKmsContext The new KMS context.
     * @param preActivationBlockNumber The block number at which the KMS context will be activated.
     */
    event PreActivateKmsContext(KmsContext newKmsContext, uint256 preActivationBlockNumber);

    event ActivateKmsContext(uint256 contextId);

    event SuspendKmsContext(uint256 contextId);

    event CompromiseKmsContext(uint256 contextId);

    event DeactivateKmsContext(uint256 contextId);

    event DestroyKmsContext(uint256 contextId);

    error KmsContextNotInitialized(uint256 contextId);

    /// @notice Error emitted when the KMS nodes list is empty.
    error EmptyKmsNodes();

    /**
     * @notice Error emitted when an address is not a KMS transaction sender from a context.
     * @param contextId The KMS context ID.
     * @param txSenderAddress The address to check.
     */
    error NotKmsTxSenderFromContext(uint256 contextId, address txSenderAddress);

    /**
     * @notice Error emitted when an address is not a KMS signer from a context.
     * @param contextId The KMS context ID.
     * @param signerAddress The address to check.
     */
    error NotKmsSignerFromContext(uint256 contextId, address signerAddress);

    /**
     * @notice Error emitted when the MPC threshold is greater or equal to the number of KMS nodes
     * within the KMS context.
     * @param contextId The KMS context ID.
     * @param mpcThreshold The MPC threshold.
     * @param nKmsNodes The number of KMS nodes.
     */
    error InvalidHighMpcThreshold(uint256 contextId, uint256 mpcThreshold, uint256 nKmsNodes);

    /// @notice Error emitted when the public decryption threshold is null.
    error InvalidNullPublicDecryptionThreshold();

    /// @notice Error emitted when the public decryption threshold is strictly greater than the number of KMS nodes.
    /// @param publicDecryptionThreshold The public decryption threshold.
    /// @param nKmsNodes The number of KMS nodes.
    error InvalidHighPublicDecryptionThreshold(uint256 publicDecryptionThreshold, uint256 nKmsNodes);

    /// @notice Error emitted when the user decryption threshold is null.
    error InvalidNullUserDecryptionThreshold();

    /// @notice Error emitted when the user decryption threshold is strictly greater than the number of KMS nodes.
    /// @param userDecryptionThreshold The user decryption threshold.
    /// @param nKmsNodes The number of KMS nodes.
    error InvalidHighUserDecryptionThreshold(uint256 userDecryptionThreshold, uint256 nKmsNodes);

    error NumberOfKmsNodesChanged(uint256 activeKmsNodesLength, uint256 newKmsNodesLength);

    error SuspendedKmsContextOngoing(uint256 suspendedContextId);

    error CompromiseActiveKmsContextNotAllowed(uint256 contextId);

    error DestroyActiveKmsContextNotAllowed(uint256 contextId);

    error KmsContextNotGenerating(uint256 contextId);

    error KmsNodeAlreadyValidatedKeyResharing(uint256 contextId, address kmsSigner);

    error NoSuspendedKmsContext();

    /**
     * @notice Error emitted when an transaction sender address is not associated with a registered KMS node within a context.
     * @param contextId The KMS context ID.
     * @param kmsTxSenderAddress The transaction sender address that is not associated with a registered KMS node.
     */
    error NotKmsNodeFromContext(uint256 contextId, address kmsTxSenderAddress);

    /**
     * @notice Check if an address is a registered KMS transaction sender from a context.
     * @param contextId The KMS context ID.
     * @param txSenderAddress The address to check.
     */
    function checkIsKmsTxSenderFromContext(uint256 contextId, address txSenderAddress) external view;

    /**
     * @notice Check if an address is a registered KMS signer from a context.
     * @param contextId The KMS context ID.
     * @param signerAddress The address to check.
     */
    function checkIsKmsSignerFromContext(uint256 contextId, address signerAddress) external view;

    function getActiveKmsContextId() external view returns (uint256);

    function getSuspendedKmsContextId() external view returns (uint256);

    function getActiveKmsContext() external view returns (KmsContext memory);

    /**
     * @notice Update the public decryption threshold.
     * @dev The new threshold must verify `1 <= t <= n`, with `n` the number of KMS nodes currently registered.
     * @param newPublicDecryptionThreshold The new public decryption threshold.
     */
    function updatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold) external;

    /**
     * @notice Update the user decryption threshold.
     * @dev The new threshold must verify `1 <= t <= n`, with `n` the number of KMS nodes currently registered.
     * @param newUserDecryptionThreshold The new user decryption threshold.
     */
    function updateUserDecryptionThreshold(uint256 newUserDecryptionThreshold) external;

    /**
     * @notice Add a new KMS context to the KmsContexts contract.
     * @param softwareVersion The software version.
     * @param reshareKeys Whether to reshare keys.
     * @param mpcThreshold The MPC threshold.
     * @param kmsNodes The set of KMS nodes representing the KMS context.
     * @param kmsBlockPeriods The block periods.
     * @param decryptionThresholds The decryption thresholds.
     */
    function addKmsContext(
        bytes8 softwareVersion,
        bool reshareKeys,
        uint256 mpcThreshold,
        KmsNode[] calldata kmsNodes,
        KmsBlockPeriods calldata kmsBlockPeriods,
        DecryptionThresholds calldata decryptionThresholds
    ) external;

    function validateKeyResharing(uint256 contextId, bytes calldata signature) external;

    function refreshKmsContextStatuses() external;

    function compromiseKmsContext(uint256 contextId) external;

    function destroyKmsContext(uint256 contextId) external;

    function moveSuspendedKmsContextToActive() external;

    /**
     * @notice Get the public decryption threshold.
     * @param contextId The KMS context ID.
     * @return The public decryption threshold.
     */
    function getPublicDecryptionThresholdFromContext(uint256 contextId) external view returns (uint256);

    /**
     * @notice Get the user decryption threshold.
     * @param contextId The KMS context ID.
     * @return The user decryption threshold.
     */
    function getUserDecryptionThresholdFromContext(uint256 contextId) external view returns (uint256);

    function getKmsContextGenerationBlockPeriod() external view returns (uint256);

    function getKmsContextSuspensionBlockPeriod() external view returns (uint256);

    /**
     * @notice Get the infos of the KMS node associated to the transaction sender within the active KMS context.
     * @param kmsTxSenderAddress The signer address of the KMS node to get.
     * @return The KMS node's metadata.
     */
    function getKmsNode(address kmsTxSenderAddress) external view returns (KmsNode memory);

    /**
     * @notice Get the list of all KMS nodes' transaction sender addresses currently registered.
     * @return The list of KMS nodes' transaction sender addresses.
     */
    function getKmsTxSenders() external view returns (address[] memory);

    /**
     * @notice Get the list of all KMS nodes' signer addresses currently registered.
     * @return The list of KMS nodes' signer addresses.
     */
    function getKmsSigners() external view returns (address[] memory);

    function getKmsContextStatus(uint256 contextId) external view returns (ContextStatus);

    /**
     * @notice Returns the versions of the KmsContexts contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}
