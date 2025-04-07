// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../shared/Structs.sol";

/**
 * @title Interface for the HTTPZ contract
 * @notice The HTTPZ contract is responsible for being a point of truth for all contracts and
 * components from the HTTPZ Gateway.
 * @dev The HTTPZ contract contains:
 * - the list of KMS nodes used exclusively by this HTTPZ Gateway
 * - the list of coprocessors used exclusively by this HTTPZ Gateway
 * - the list of networks using this HTTPZ Gateway
 *
 * The HTTPZ contract has an owner and a pauser.
 * The owner can call some restricted functions, such as adding or removing KMS nodes, coprocessors
 * and networks.
 * The pauser can pause all contracts.
 * Some view functions are accessible to everyone (ex: getting the number of KMS nodes).
 */
interface IHTTPZ {
    /// @notice Emitted when the HTTPZ initialization is completed
    /// @param pauser Pauser address
    /// @param metadata Metadata of the protocol
    /// @param kmsThreshold The KMS threshold
    /// @param kmsNodes List of KMS nodes
    /// @param coprocessors List of coprocessors
    event Initialization(
        address pauser,
        ProtocolMetadata metadata,
        uint256 kmsThreshold,
        KmsNode[] kmsNodes,
        Coprocessor[] coprocessors
    );

    /// @notice Emitted when the pauser address has been updated
    /// @param newPauser The new pauser address
    event UpdatePauser(address newPauser);

    /// @notice Emitted when the KMS threshold has been updated
    /// @param newKmsThreshold The new KMS threshold
    event UpdateKmsThreshold(uint256 newKmsThreshold);

    /// @notice Emitted when a new network metadata is added
    /// @param network The new network metadata
    event AddNetwork(Network network);

    /// @notice Error emitted when the pauser address is the null address
    error InvalidNullPauser();

    /// @notice Error emitted when the KMS threshold is too high with respect to the number of KMS nodes
    /// @notice For a set of `n` KMS nodes, the threshold `t` must verify `0 <= t <= n`.
    /// @param threshold The threshold
    /// @param nParties The number of KMS nodes
    error KmsThresholdTooHigh(uint256 threshold, uint256 nParties);

    /// @notice Error emitted when an address is not the pauser
    /// @param pauserAddress The address that is not the pauser
    error NotPauser(address pauserAddress);

    /// @notice Error emitted when an address is not a KMS transaction sender
    /// @param txSenderAddress The address that is not a KMS transaction sender
    error NotKmsTxSender(address txSenderAddress);

    /// @notice Error emitted when an address is not a KMS signer
    /// @param signerAddress The address that is not a KMS signer
    error NotKmsSigner(address signerAddress);

    /// @notice Error emitted when an address is not a coprocessor transaction sender
    /// @param txSenderAddress The address that is not a coprocessor transaction sender
    error NotCoprocessorTxSender(address txSenderAddress);

    /// @notice Error emitted when an address is not a coprocessor signer
    /// @param signerAddress The address that is not a coprocessor signer
    error NotCoprocessorSigner(address signerAddress);

    /// @notice Error emitted when a network is not registered
    /// @param chainId The chain ID of the network
    error NetworkNotRegistered(uint256 chainId);

    /// @notice Error emitted when trying to add a network that is already registered.
    error NetworkAlreadyRegistered(uint256 chainId);

    /// @notice Error indicating that a null chain ID is not allowed.
    error InvalidNullChainId();

    /// @notice Error indicating that a chain ID is not represented by a uint64.
    /// @param chainId The chain ID
    error ChainIdNotUint64(uint256 chainId);

    /// @notice Update the pauser address
    /// @param newPauser The new pauser address
    function updatePauser(address newPauser) external;

    /// @notice Update the KMS threshold
    /// @dev The new threshold must verify `0 <= t <= n`, with `n` the number of KMS nodes currently registered
    /// @param newKmsThreshold The new KMS threshold
    function updateKmsThreshold(uint256 newKmsThreshold) external;

    /// @notice Check if an address is the pauser
    /// @param pauserAddress The address to check
    function checkIsPauser(address pauserAddress) external view;

    /// @notice Check if an address is a registered KMS transaction sender
    /// @param kmsTxSenderAddress The address to check
    function checkIsKmsTxSender(address kmsTxSenderAddress) external view;

    /// @notice Check if an address is a registered KMS signer
    /// @param signerAddress The address to check
    function checkIsKmsSigner(address signerAddress) external view;

    /// @notice Check if an address is a registered coprocessor transaction sender
    /// @param coprocessorTxSenderAddress The address to check
    function checkIsCoprocessorTxSender(address coprocessorTxSenderAddress) external view;

    /// @notice Check if an address is a registered coprocessor signer
    /// @param signerAddress The address to check
    function checkIsCoprocessorSigner(address signerAddress) external view;

    /// @notice Check if a chain ID corresponds to a registered network
    /// @param chainId The chain ID to check
    function checkNetworkIsRegistered(uint256 chainId) external view;

    /// @notice Get the protocol's metadata
    /// @return The protocol's metadata
    function getProtocolMetadata() external view returns (ProtocolMetadata memory);

    /// @notice Get the KMS vote threshold
    /// @return The KMS vote threshold
    function getKmsThreshold() external view returns (uint256);

    /// @notice Get the KMS majority vote threshold
    /// @return The KMS majority vote threshold
    function getKmsMajorityThreshold() external view returns (uint256);

    /// @notice Get the KMS reconstruction threshold
    /// @return The KMS reconstruction threshold
    function getKmsReconstructionThreshold() external view returns (uint256);

    /// @notice Get the coprocessor majority threshold
    /// @return The coprocessor majority threshold
    function getCoprocessorMajorityThreshold() external view returns (uint256);

    /// @notice Get the metadata of the KMS node with the given transaction sender address
    /// @return The KMS node's metadata
    function kmsNodes(address kmsTxSenderAddress) external view returns (KmsNode memory);

    /// @notice Get the address of the KMS transaction sender with the given index
    /// @return The KMS transaction sender's address
    function kmsTxSenderAddresses(uint256 index) external view returns (address);

    /// @notice Get the list of all KMS nodes' transaction sender addresses currently registered
    /// @return The list of KMS nodes' transaction sender addresses
    function getAllKmsTxSenderAddresses() external view returns (address[] memory);

    /// @notice Get the metadata of the coprocessor with the given transaction sender address
    /// @return The coprocessor's metadata
    function coprocessors(address coprocessorTxSenderAddress) external view returns (Coprocessor memory);

    /// @notice Get the address of the coprocessor transaction sender with the given index
    /// @return The coprocessor transaction sender's address
    function coprocessorTxSenderAddresses(uint256 index) external view returns (address);

    /// @notice Get the list of all coprocessors' transaction sender addresses currently registered
    /// @return The list of coprocessors' transaction sender addresses
    function getAllCoprocessorTxSenderAddresses() external view returns (address[] memory);

    /// @notice Get the metadata of the network with the given index
    /// @return The network's metadata
    function networks(uint256 index) external view returns (Network memory);

    /// @notice Add a new Network metadata to the HTTPZ contract.
    /// @dev The associated chain ID must be non-zero and representable by a uint64.
    /// @param network The new network metadata to include.
    function addNetwork(Network calldata network) external;
}
