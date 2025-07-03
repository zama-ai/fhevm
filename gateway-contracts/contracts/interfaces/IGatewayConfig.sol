// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {
    KmsNode,
    Coprocessor,
    Custodian,
    HostChain,
    ProtocolMetadata,
    DecryptionThresholds
} from "../shared/Structs.sol";
import { ContextStatus } from "../shared/Enums.sol";
/**
 * @title Interface for the GatewayConfig contract.
 * @notice The GatewayConfig contract is responsible for being a point of truth for all contracts and
 * components from the fhevm Gateway.
 * @dev In particular, the GatewayConfig contract contains:
 * - the list of coprocessors used exclusively by this fhevm Gateway
 * - the list of host chains using this fhevm Gateway
 *
 * The GatewayConfig contract has an owner and a pauser.
 * The owner can call some restricted functions, such as adding or removing coprocessors
 * and host chains.
 * The pauser can pause all contracts.
 * Some view functions are accessible to everyone (ex: getting the number of coprocessors).
 */
interface IGatewayConfig {
    /**
     * @notice Emitted when the GatewayConfig initialization is completed.
     * @param pauser Pauser address.
     * @param metadata Metadata of the protocol.
     * @param coprocessors List of coprocessors.
     * @param custodians List of custodians.
     */
    event InitializeGatewayConfig(
        address pauser,
        ProtocolMetadata metadata,
        Coprocessor[] coprocessors,
        Custodian[] custodians
    );

    /**
     * @notice Emitted when the GatewayConfigV2 reinitialization is completed.
     * @param custodians List of custodians.
     */
    event ReinitializeGatewayConfigV2(Custodian[] custodians);

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

    /**
     * @notice Emitted when a new host chain has been registered.
     * @param hostChain The new host chain metadata.
     */
    event AddHostChain(HostChain hostChain);

    /// @notice Error emitted when the pauser address is the null address.
    error InvalidNullPauser();

    /// @notice Error emitted when the coprocessors list is empty.
    error EmptyCoprocessors();

    /// @notice Error emitted when the custodians list is empty.
    error EmptyCustodians();

    /**
     * @notice Error emitted when an address is not the pauser.
     * @param pauserAddress The address that is not the pauser.
     */
    error NotPauser(address pauserAddress);

    /**
     * @notice Error emitted when an address is not a coprocessor transaction sender.
     * @param txSenderAddress The address that is not a coprocessor transaction sender.
     */
    error NotCoprocessorTxSender(address txSenderAddress);

    /*
     * @notice Error emitted when an address is not a coprocessor signer.
     * @param signerAddress The address that is not a coprocessor signer.
     */
    error NotCoprocessorSigner(address signerAddress);

    /**
     * @notice Error emitted when an address is not a custodian transaction sender.
     * @param txSenderAddress The address that is not a custodian transaction sender.
     */
    error NotCustodianTxSender(address txSenderAddress);

    /*
     * @notice Error emitted when an address is not a custodian signer.
     * @param signerAddress The address that is not a custodian signer.
     */
    error NotCustodianSigner(address signerAddress);

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

    /**
     * @notice Update the pauser address.
     * @param newPauser The new pauser address.
     */
    function updatePauser(address newPauser) external;

    /**
     * @notice Add a new host chain metadata to the GatewayConfig contract.
     * @dev The associated chain ID must be non-zero and representable by a uint64.
     * @param hostChain The new host chain metadata to include.
     */
    function addHostChain(HostChain calldata hostChain) external;

    /**
     * @notice Check if an address is the pauser.
     * @param pauserAddress The address to check.
     */
    function checkIsPauser(address pauserAddress) external view;

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
     * @notice Check if an address is a registered custodian transaction sender.
     * @param txSenderAddress The address to check.
     */
    function checkIsCustodianTxSender(address txSenderAddress) external view;

    /**
     * @notice Check if an address is a registered custodian signer.
     * @param signerAddress The address to check.
     */
    function checkIsCustodianSigner(address signerAddress) external view;

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
     * @notice Get the coprocessor majority threshold.
     * @return The coprocessor majority threshold.
     */
    function getCoprocessorMajorityThreshold() external view returns (uint256);

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
