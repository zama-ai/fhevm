// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {
    KmsNode,
    KmsContext,
    Coprocessor,
    HostChain,
    ProtocolMetadata,
    KmsConfiguration,
    DecryptionThresholds
} from "../shared/Structs.sol";
import { ContextStatus } from "../shared/Enums.sol";
/**
 * @title Interface for the GatewayConfig contract.
 * @notice The GatewayConfig contract is responsible for being a point of truth for all contracts and
 * components from the fhevm Gateway.
 * @dev In particular, the GatewayConfig contract contains:
 * - the list of KMS nodes used exclusively by this fhevm Gateway
 * - the list of coprocessors used exclusively by this fhevm Gateway
 * - the list of host chains using this fhevm Gateway
 *
 * The GatewayConfig contract has an owner and a pauser.
 * The owner can call some restricted functions, such as adding or removing KMS nodes, coprocessors
 * and host chains.
 * The pauser can pause all contracts.
 * Some view functions are accessible to everyone (ex: getting the number of KMS nodes).
 */
interface IGatewayConfig {
    /**
     * @notice Emitted when the GatewayConfig initialization is completed.
     * @param pauser Pauser address.
     * @param metadata Metadata of the protocol.
     * @param kmsConfiguration KMS configuration parameters.
     * @param coprocessors List of coprocessors.
     */
    event Initialization(
        address pauser,
        ProtocolMetadata metadata,
        KmsConfiguration kmsConfiguration,
        Coprocessor[] coprocessors
    );

    /**
     * @notice Emitted when the pauser address has been updated.
     * @param newPauser The new pauser address.
     */
    event UpdatePauser(address newPauser);

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

    event UpdateKmsContextGenerationBlockPeriod(uint256 newKmsContextGenerationBlockPeriod);
    event UpdateKmsContextSuspensionBlockPeriod(uint256 newKmsContextSuspensionBlockPeriod);

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

    event InvalidateKeyResharing(uint256 kmsContextId);

    event DeactivateKmsContext(uint256 kmsContextId);

    event CompromiseKmsContext(uint256 kmsContextId);

    /**
     * @notice Emitted when a new KMS context has been registered.
     * @param activeKmsContext The current active KMS context.
     * @param newKmsContext The new KMS context.
     */
    event NewKmsContext(KmsContext activeKmsContext, KmsContext newKmsContext);

    event DestroyKmsContext(uint256 kmsContextId);

    event SuspendKmsContext(uint256 kmsContextId);

    event ActivateKmsContext(uint256 kmsContextId);

    /**
     * @notice Emitted when a new KMS context is being pre-activated.
     * @param newKmsContext The new KMS context.
     * @param preActivationBlockNumber The block number at which the KMS context will be activated.
     */
    event StartKmsContextPreActivation(KmsContext newKmsContext, uint256 preActivationBlockNumber);

    /**
     * @notice Emitted when a new host chain has been registered.
     * @param hostChain The new host chain metadata.
     */
    event AddHostChain(HostChain hostChain);

    /// @notice Error emitted when the pauser address is the null address.
    error InvalidNullPauser();

    /// @notice Error emitted when the KMS nodes list is empty.
    error EmptyKmsNodes();

    /// @notice Error emitted when the coprocessors list is empty.
    error EmptyCoprocessors();

    /**
     * @notice Error emitted when the MPC threshold is greater or equal to the number of KMS nodes
     * within the KMS context.
     * @param kmsContextId The KMS context ID.
     * @param mpcThreshold The MPC threshold.
     * @param nKmsNodes The number of KMS nodes.
     */
    error InvalidHighMpcThreshold(uint256 kmsContextId, uint256 mpcThreshold, uint256 nKmsNodes);

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

    /**
     * @notice Error emitted when an address is not the pauser.
     * @param pauserAddress The address that is not the pauser.
     */
    error NotPauser(address pauserAddress);

    /**
     * @notice Error emitted when an address is not a KMS transaction sender from the active context.
     * @param txSenderAddress The address to check.
     */
    error NotActiveKmsTxSender(address txSenderAddress);

    /**
     * @notice Error emitted when an address is not a KMS transaction sender from a context.
     * @param kmsContextId The KMS context ID.
     * @param txSenderAddress The address to check.
     */
    error NotKmsTxSenderFromContext(uint256 kmsContextId, address txSenderAddress);

    /**
     * @notice Error emitted when an address is not a KMS signer from the active context.
     * @param signerAddress The address to check.
     */
    error NotActiveKmsSigner(address signerAddress);

    /**
     * @notice Error emitted when an address is not a KMS signer from a context.
     * @param kmsContextId The KMS context ID.
     * @param signerAddress The address to check.
     */
    error NotKmsSignerFromContext(uint256 kmsContextId, address signerAddress);

    /**
     * @notice Error emitted when an address is not a coprocessor transaction sender.
     * @param txSenderAddress The address that is not a coprocessor transaction sender.
     */
    error NotCoprocessorTxSender(address txSenderAddress);

    /**
     * @notice Error emitted when an transaction sender address is not associated with a registered KMS node within.
     * @param kmsContextId The KMS context ID.
     * @param kmsTxSenderAddress The transaction sender address that is not associated with a registered KMS node.
     */
    error NotKmsNode(uint256 kmsContextId, address kmsTxSenderAddress);

    error KmsNodeAlreadyValidatedKeyResharing(uint256 kmsContextId, address kmsSigner);

    error KmsContextNotGenerating(uint256 kmsContextId);

    error NoSuspendedKmsContext();

    error NumberOfKmsNodesChanged(uint256 activeKmsNodesLength, uint256 newKmsNodesLength);

    error SuspendedKmsContextOngoing(uint256 suspendedContextId);

    error KmsContextNotInitialized(uint256 kmsContextId);

    error CompromiseActiveKmsContextNotAllowed(uint256 kmsContextId);
    error DestroyActiveKmsContextNotAllowed(uint256 kmsContextId);

    /*C
     * @notice Error emitted when an address is not a coprocessor signer.
     * @param signerAddress The address that is not a coprocessor signer.
     */
    error NotCoprocessorSigner(address signerAddress);

    /**
     * @notice Error emitted when a host chain is not registered.
     * @param chainId The host chain's chain ID.
     */
    error HostChainNotRegistered(uint256 chainId);

    /**
     * @notice Error emitted when trying to add a host chain that is already registered.
     * @param chainId The host chain's chain ID that is already registered.
     */
    error HostChainAlreadyRegistered(uint256 chainId);

    /// @notice Error indicating that a null chain ID is not allowed.
    error InvalidNullChainId();

    /**
     * @notice Error indicating that a chain ID is not represented by a uint64.
     * @param chainId The ID of the host chain that is not a valid uint64.
     */
    error ChainIdNotUint64(uint256 chainId);

    function getKmsContext(uint256 kmsContextId) external view returns (KmsContext memory);
    function getActiveKmsContextId() external view returns (uint256);
    function getSuspendedKmsContextId() external view returns (uint256);

    function getActiveKmsContext() external view returns (KmsContext memory);

    function getKmsNodes() external view returns (KmsNode[] memory);

    /**
     * @notice Update the pauser address.
     * @param newPauser The new pauser address.
     */
    function updatePauser(address newPauser) external;

    /**
     * @notice Add a new KMS context to the GatewayConfig contract.
     * @param preActivationBlockPeriod The pre-activation block period.
     * @param softwareVersion The software version.
     * @param reshareKeys Whether to reshare keys.
     * @param mpcThreshold The MPC threshold.
     * @param kmsNodes The set of KMS nodes representing the KMS context.
     */
    function addKmsContext(
        uint256 preActivationBlockPeriod,
        bytes8 softwareVersion,
        bool reshareKeys,
        uint256 mpcThreshold,
        KmsNode[] calldata kmsNodes,
        DecryptionThresholds calldata decryptionThresholds
    ) external;

    function validateKeyResharing(uint256 kmsContextId, bytes calldata signature) external;

    function refreshKmsContextStatuses() external;

    function compromiseKmsContext(uint256 kmsContextId) external;

    function destroyKmsContext(uint256 kmsContextId) external;

    function moveSuspendedKmsContextToActive() external;

    /**
     * @notice Add a new host chain metadata to the GatewayConfig contract.
     * @dev The associated chain ID must be non-zero and representable by a uint64.
     * @param hostChain The new host chain metadata to include.
     */
    function addHostChain(HostChain calldata hostChain) external;

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
     * @notice Check if an address is the pauser.
     * @param pauserAddress The address to check.
     */
    function checkIsPauser(address pauserAddress) external view;

    /**
     * @notice Check if an address is a registered KMS transaction sender from a context.
     * @param kmsContextId The KMS context ID.
     * @param txSenderAddress The address to check.
     */
    function checkIsKmsTxSenderFromContext(uint256 kmsContextId, address txSenderAddress) external view;

    /**
     * @notice Check if an address is a registered KMS signer from a context.
     * @param kmsContextId The KMS context ID.
     * @param signerAddress The address to check.
     */
    function checkIsKmsSignerFromContext(uint256 kmsContextId, address signerAddress) external view;

    /**
     * @notice Check if an address is a registered coprocessor transaction sender.
     * @param coprocessorTxSenderAddress The address to check.
     */
    function checkIsCoprocessorTxSender(address coprocessorTxSenderAddress) external view;

    /**
     * @notice Check if an address is a registered coprocessor signer.
     * @param signerAddress The address to check.
     */
    function checkIsCoprocessorSigner(address signerAddress) external view;

    /**
     * @notice Check if a chain ID corresponds to a registered host chain.
     * @param chainId The chain ID to check.
     */
    function checkHostChainIsRegistered(uint256 chainId) external view;

    /**
     * @notice Get the pauser's address.
     * @return The address of the pauser.
     */
    function getPauser() external view returns (address);

    /**
     * @notice Get the protocol's metadata.
     * @return The protocol's metadata.
     */
    function getProtocolMetadata() external view returns (ProtocolMetadata memory);

    /**
     *  @notice Get the MPC threshold.
     *  @return The MPC threshold.
     */
    function getMpcThreshold() external view returns (uint256);

    /**
     * @notice Get the public decryption threshold.
     * @param kmsContextId The KMS context ID.
     * @return The public decryption threshold.
     */
    function getPublicDecryptionThresholdFromContext(uint256 kmsContextId) external view returns (uint256);

    /**
     * @notice Get the public decryption threshold.
     * @return The public decryption threshold.
     */
    function getPublicDecryptionThreshold() external view returns (uint256);

    /**
     * @notice Get the user decryption threshold.
     * @param kmsContextId The KMS context ID.
     * @return The user decryption threshold.
     */
    function getUserDecryptionThresholdFromContext(uint256 kmsContextId) external view returns (uint256);

    /**
     * @notice Get the user decryption threshold.
     * @return The user decryption threshold.
     */
    function getUserDecryptionThreshold() external view returns (uint256);

    /**
     * @notice Get the coprocessor majority threshold.
     * @return The coprocessor majority threshold.
     */
    function getCoprocessorMajorityThreshold() external view returns (uint256);

    /**
     * @notice Get the infos of the KMS node associated to the transaction sender within a KMS context.
     * @param kmsContextId The KMS context ID.
     * @param kmsTxSenderAddress The signer address of the KMS node to get.
     * @return The KMS node's metadata.
     */
    function getKmsNodeFromContext(
        uint256 kmsContextId,
        address kmsTxSenderAddress
    ) external view returns (KmsNode memory);

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

    function getKmsContextStatus(uint256 kmsContextId) external view returns (ContextStatus);

    /**
     * @notice Get the metadata of the coprocessor with the given transaction sender address.
     * @return The coprocessor's metadata.
     */
    function getCoprocessor(address coprocessorTxSenderAddress) external view returns (Coprocessor memory);

    /**
     * @notice Get the list of all coprocessors' transaction sender addresses currently registered.
     * @return The list of coprocessors' transaction sender addresses.
     */
    function getCoprocessorTxSenders() external view returns (address[] memory);

    /**
     * @notice Get the list of all coprocessors' signer addresses currently registered.
     * @return The list of coprocessors' signer addresses.
     */
    function getCoprocessorSigners() external view returns (address[] memory);

    /**
     * @notice Get the metadata of the host chain with the given index.
     * @return The host chain's metadata.
     */
    function getHostChain(uint256 index) external view returns (HostChain memory);

    /**
     * @notice Get the metadata of all the registered host chains.
     * @return The host chains' metadata.
     */
    function getHostChains() external view returns (HostChain[] memory);

    /**
     * @notice Returns the versions of the GatewayConfig contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}
