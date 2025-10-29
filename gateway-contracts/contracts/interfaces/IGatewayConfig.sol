// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { ProtocolMetadata, KmsNode, Coprocessor, Custodian, HostChain } from "../shared/Structs.sol";

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
     * @param metadata Metadata of the protocol.
     * @param mpcThreshold The MPC threshold.
     * @param kmsNodes List of KMS nodes.
     * @param coprocessors List of coprocessors.
     * @param custodians List of custodians.
     */
    event InitializeGatewayConfig(
        ProtocolMetadata metadata,
        uint256 mpcThreshold,
        KmsNode[] kmsNodes,
        Coprocessor[] coprocessors,
        Custodian[] custodians
    );

    /**
     * @notice Emitted when the GatewayConfig is re-initialized from V3.
     * @param newKmsNodes The new KMS nodes.
     */
    event ReinitializeGatewayConfigV3(KmsNode[] newKmsNodes);

    /**
     * @notice Emitted when the KMS nodes have been updated.
     * @param newKmsNodes The new KMS nodes.
     * @param newMpcThreshold The new MPC threshold.
     * @param newPublicDecryptionThreshold The new public decryption threshold.
     * @param newUserDecryptionThreshold The new user decryption threshold.
     * @param newKmsGenThreshold The new key and CRS generation threshold.
     */
    event UpdateKmsNodes(
        KmsNode[] newKmsNodes,
        uint256 newMpcThreshold,
        uint256 newPublicDecryptionThreshold,
        uint256 newUserDecryptionThreshold,
        uint256 newKmsGenThreshold
    );

    /**
     * @notice Emitted when the coprocessors have been updated.
     * @param newCoprocessors The new coprocessors.
     * @param newCoprocessorThreshold The new coprocessor threshold.
     */
    event UpdateCoprocessors(Coprocessor[] newCoprocessors, uint256 newCoprocessorThreshold);

    /**
     * @notice Emitted when the custodians have been updated.
     * @param newCustodians The new custodians.
     */
    event UpdateCustodians(Custodian[] newCustodians);

    /**
     * @notice Emitted when the MPC threshold has been updated.
     * @param newMpcThreshold The new MPC threshold.
     */
    event UpdateMpcThreshold(uint256 newMpcThreshold);

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
     * @notice Emitted when the key and CRS generation threshold has been updated.
     * @param newKmsGenThreshold The new key and CRS generation threshold.
     */
    event UpdateKmsGenThreshold(uint256 newKmsGenThreshold);

    /**
     * @notice Emitted when the coprocessor threshold has been updated.
     * @param newCoprocessorThreshold The new coprocessor threshold.
     */
    event UpdateCoprocessorThreshold(uint256 newCoprocessorThreshold);

    /**
     * @notice Emitted when a new host chain has been registered.
     * @param hostChain The new host chain metadata.
     */
    event AddHostChain(HostChain hostChain);

    /**
     * @notice Error indicating that the given account is not a pauser.
     * @param account The address of the account.
     */
    error NotPauser(address account);

    /**
     * @notice Error emitted when the KMS nodes list is empty.
     */
    error EmptyKmsNodes();

    /**
     * @notice Error emitted when the coprocessors list is empty.
     */
    error EmptyCoprocessors();

    /**
     * @notice Error emitted when the custodians list is empty.
     */
    error EmptyCustodians();

    /**
     * @notice Error emitted when the MPC threshold is greater or equal to the number of KMS nodes.
     * @param mpcThreshold The MPC threshold.
     * @param nKmsNodes The number of KMS nodes.
     */
    error InvalidHighMpcThreshold(uint256 mpcThreshold, uint256 nKmsNodes);

    /**
     * @notice Error emitted when the public decryption threshold is null.
     */
    error InvalidNullPublicDecryptionThreshold();

    /**
     * @notice Error emitted when the public decryption threshold is strictly greater than the number of KMS nodes.
     * @param publicDecryptionThreshold The public decryption threshold.
     * @param nKmsNodes The number of KMS nodes.
     */
    error InvalidHighPublicDecryptionThreshold(uint256 publicDecryptionThreshold, uint256 nKmsNodes);

    /**
     * @notice Error emitted when the user decryption threshold is null.
     */
    error InvalidNullUserDecryptionThreshold();

    /**
     * @notice Error emitted when the user decryption threshold is strictly greater than the number of KMS nodes.
     * @param userDecryptionThreshold The user decryption threshold.
     * @param nKmsNodes The number of KMS nodes.
     */
    error InvalidHighUserDecryptionThreshold(uint256 userDecryptionThreshold, uint256 nKmsNodes);

    /**
     * @notice Error emitted when the key and CRS generation threshold is null.
     */
    error InvalidNullKmsGenThreshold();

    /**
     * @notice Error emitted when the key and CRS generation threshold is strictly greater than the number of KMS nodes.
     * @param kmsGenThreshold The key and CRS generation threshold.
     * @param nKmsNodes The number of KMS nodes.
     */
    error InvalidHighKmsGenThreshold(uint256 kmsGenThreshold, uint256 nKmsNodes);

    /**
     * @notice Error emitted when the coprocessor threshold is null.
     */
    error InvalidNullCoprocessorThreshold();

    /**
     * @notice Error emitted when the coprocessor threshold is strictly greater than the number of coprocessors.
     * @param coprocessorThreshold The coprocessor threshold.
     * @param nCoprocessors The number of coprocessors.
     */
    error InvalidHighCoprocessorThreshold(uint256 coprocessorThreshold, uint256 nCoprocessors);

    /**
     * @notice Emitted when all the pausable gateway contracts are paused.
     */
    event PauseAllGatewayContracts();

    /**
     * @notice Emitted when all the pausable gateway contracts are unpaused.
     */
    event UnpauseAllGatewayContracts();

    /**
     * @notice Error emitted when trying to add a host chain that is already registered.
     * @param chainId The host chain's chain ID that is already registered.
     */
    error HostChainAlreadyRegistered(uint256 chainId);

    /**
     * @notice Error indicating that a null chain ID is not allowed.
     */
    error InvalidNullChainId();

    /**
     * @notice Error indicating that a chain ID is not represented by a uint64.
     * @param chainId The ID of the host chain that is not a valid uint64.
     */
    error ChainIdNotUint64(uint256 chainId);

    /**
     * @notice Update the list of KMS nodes and their thresholds.
     * @dev ⚠️ This function should be used with caution as it can lead to unexpected behavior in
     * some requests and the contracts should first be paused. It will be deprecated in the future.
     * @param newKmsNodes The new KMS nodes.
     * @param newMpcThreshold The new MPC threshold.
     * @param newPublicDecryptionThreshold The new public decryption threshold.
     * @param newUserDecryptionThreshold The new user decryption threshold.
     * @param newKmsGenThreshold The new key and CRS generation threshold.
     */
    function updateKmsNodes(
        KmsNode[] calldata newKmsNodes,
        uint256 newMpcThreshold,
        uint256 newPublicDecryptionThreshold,
        uint256 newUserDecryptionThreshold,
        uint256 newKmsGenThreshold
    ) external;

    /**
     * @notice Update the list of coprocessors and their threshold.
     * @dev ⚠️ This function should be used with caution as it can lead to unexpected behavior in
     * some requests and the contracts should first be paused. It will be deprecated in the future.
     * @param newCoprocessors The new coprocessors.
     * @param newCoprocessorThreshold The new coprocessor threshold.
     */
    function updateCoprocessors(Coprocessor[] calldata newCoprocessors, uint256 newCoprocessorThreshold) external;

    /**
     * @notice Update the list of custodians.
     * @dev ⚠️ This function should be used with caution. It will be deprecated in the future.
     * @param newCustodians The new custodians.
     */
    function updateCustodians(Custodian[] calldata newCustodians) external;

    /**
     * @notice Update the MPC threshold.
     * @dev The new threshold must verify `0 <= t < n`, with `n` the number of KMS nodes currently registered.
     * @param newMpcThreshold The new MPC threshold.
     */
    function updateMpcThreshold(uint256 newMpcThreshold) external;

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
     * @notice Update the key and CRS generation threshold.
     * @dev The new threshold must verify `1 <= t <= n`, with `n` the number of KMS nodes currently registered.
     * @param newKmsGenThreshold The new key and CRS generation threshold.
     */
    function updateKmsGenThreshold(uint256 newKmsGenThreshold) external;

    /**
     * @notice Update the coprocessor threshold.
     * @dev The new threshold must verify `1 <= t <= n`, with `n` the number of coprocessors currently registered.
     * @param newCoprocessorThreshold The new coprocessor threshold.
     */
    function updateCoprocessorThreshold(uint256 newCoprocessorThreshold) external;

    /**
     * @notice Add a new host chain metadata to the GatewayConfig contract.
     * @dev The associated chain ID must be non-zero and representable by a uint64.
     * @param hostChain The new host chain metadata to include.
     */
    function addHostChain(HostChain calldata hostChain) external;

    /**
     * @notice Pause all pausable gateway contracts.
     */
    function pauseAllGatewayContracts() external;

    /**
     * @notice Unpause all pausable gateway contracts.
     */
    function unpauseAllGatewayContracts() external;

    /**
     * @notice Indicates if an address is a registered KMS transaction sender.
     * @param kmsTxSenderAddress The address to check.
     */
    function isKmsTxSender(address kmsTxSenderAddress) external view returns (bool);

    /**
     * @notice Indicates if an address is a registered KMS signer.
     * @param signerAddress The address to check.
     */
    function isKmsSigner(address signerAddress) external view returns (bool);

    /**
     * @notice Indicates if an address is a registered coprocessor transaction sender.
     * @param coprocessorTxSenderAddress The address to check.
     */
    function isCoprocessorTxSender(address coprocessorTxSenderAddress) external view returns (bool);

    /**
     * @notice Indicates if an address is a registered coprocessor signer.
     * @param signerAddress The address to check.
     */
    function isCoprocessorSigner(address signerAddress) external view returns (bool);

    /**
     * @notice Indicates if an address is a registered custodian transaction sender.
     * @param txSenderAddress The address to check.
     */
    function isCustodianTxSender(address txSenderAddress) external view returns (bool);

    /**
     * @notice Indicates if an address is a registered custodian signer.
     * @param signerAddress The address to check.
     */
    function isCustodianSigner(address signerAddress) external view returns (bool);

    /**
     * @notice Indicates if a chain ID corresponds to a registered host chain.
     * @param chainId The chain ID to check.
     */
    function isHostChainRegistered(uint256 chainId) external view returns (bool);

    /**
     * @notice Check if the account is a pauser.
     * @return Whether or not the account is a pauser.
     */
    function isPauser(address account) external view returns (bool);

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
     * @return The public decryption threshold.
     */
    function getPublicDecryptionThreshold() external view returns (uint256);

    /**
     * @notice Get the user decryption threshold.
     * @return The user decryption threshold.
     */
    function getUserDecryptionThreshold() external view returns (uint256);

    /**
     * @notice Get the key and CRS generation threshold
     * @return The key and CRS generation threshold.
     */
    function getKmsGenThreshold() external view returns (uint256);

    /**
     * @notice Get the coprocessor majority threshold
     * @return The coprocessor majority threshold.
     */
    function getCoprocessorMajorityThreshold() external view returns (uint256);

    /**
     * @notice Get the metadata of the KMS node with the given transaction sender address.
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
     * @notice Get the metadata of the custodian with the given transaction sender address.
     * @return The custodian's metadata.
     */
    function getCustodian(address custodianTxSender) external view returns (Custodian memory);

    /**
     * @notice Get the list of all custodians' transaction sender addresses currently registered.
     * @return The list of custodians' transaction sender addresses.
     */
    function getCustodianTxSenders() external view returns (address[] memory);

    /**
     * @notice Get the list of all custodians' signer addresses currently registered.
     * @return The list of custodians' signer addresses.
     */
    function getCustodianSigners() external view returns (address[] memory);

    /**
     * @notice Returns the versions of the GatewayConfig contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}
