// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { gatewayConfigAddress } from "../addresses/GatewayAddresses.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { IInputVerificationRegistry } from "./interfaces/IInputVerificationRegistry.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayConfigChecks } from "./shared/GatewayConfigChecks.sol";
import { GatewayOwnable } from "./shared/GatewayOwnable.sol";
import { ProtocolPaymentUtils } from "./shared/ProtocolPaymentUtils.sol";
import { Pausable } from "./shared/Pausable.sol";

/**
 * @title InputVerificationRegistry contract
 * @notice See {IInputVerificationRegistry}.
 * @dev V2 Design: This contract handles input verification request registration only.
 * Unlike V1 InputVerification.sol:
 * - Does NOT receive full ciphertext payloads (only commitment hash)
 * - Does NOT handle response processing (done off-chain via Coprocessor HTTP API)
 * - Does NOT store ciphertexts (Coprocessors store them after verification)
 * - Relayer computes commitment = keccak256(ciphertext || ZKPoK), posts commitment only
 * - Relayer broadcasts full payload directly to Coprocessors via HTTP
 * - Coprocessors verify commitment matches before processing
 */
contract InputVerificationRegistry is
    IInputVerificationRegistry,
    UUPSUpgradeableEmptyProxy,
    GatewayOwnable,
    GatewayConfigChecks,
    ProtocolPaymentUtils,
    Pausable
{
    /**
     * @notice The address of the GatewayConfig contract.
     */
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /**
     * @dev The following constants are used for versioning the contract.
     */
    string private constant CONTRACT_NAME = "InputVerificationRegistry";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for the `reinitializer` modifier.
     */
    uint64 private constant REINITIALIZER_VERSION = 1;

    /**
     * @notice Request ID counter base for input verification requests.
     * @dev Format: [0000 0011 | counter_1..31] for uniqueness across request types.
     * Different from decryption request IDs (0x01xx and 0x02xx prefixes).
     */
    uint256 private constant INPUT_VERIFICATION_COUNTER_BASE = 0x0300000000000000000000000000000000000000000000000000000000000000;

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.InputVerificationRegistry
    struct InputVerificationRegistryStorage {
        /// @notice Counter for input verification requests.
        uint256 requestCounter;
        /// @notice Mapping from requestId to request details.
        mapping(uint256 => InputVerificationRequest) requests;
    }

    /**
     * @notice Storage struct for input verification requests.
     */
    struct InputVerificationRequest {
        bytes32 commitment;
        address userAddress;
        uint256 contractChainId;
        address contractAddress;
        uint256 fee;
        uint256 timestamp;
    }

    /**
     * @dev Storage location computed using:
     * keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.InputVerificationRegistry")) - 1))
     * & ~bytes32(uint256(0xff))
     */
    bytes32 private constant INPUT_VERIFICATION_REGISTRY_STORAGE_LOCATION =
        0xa2c7a90bb5b79766e8d1a00bb5b79766e8d1a00bb5b79766e8d1a00bb5b79700;

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
        __Pausable_init();

        InputVerificationRegistryStorage storage $ = _getInputVerificationRegistryStorage();
        $.requestCounter = INPUT_VERIFICATION_COUNTER_BASE;
    }

    /**
     * @notice See {IInputVerificationRegistry-registerInputVerification}.
     */
    function registerInputVerification(
        bytes32 commitment,
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata userSignature
    ) external payable virtual whenNotPaused returns (uint256) {
        // Validate inputs
        if (commitment == bytes32(0)) {
            revert EmptyCommitment();
        }
        if (contractChainId == 0) {
            revert InvalidChainId();
        }
        if (contractAddress == address(0)) {
            revert EmptyContractAddress();
        }
        if (userAddress == address(0)) {
            revert EmptyUserAddress();
        }
        if (userSignature.length == 0) {
            revert EmptyUserSignature();
        }

        InputVerificationRegistryStorage storage $ = _getInputVerificationRegistryStorage();

        // Generate unique request ID
        $.requestCounter++;
        uint256 requestId = $.requestCounter;

        // Collect fee for input verification
        _collectInputVerificationFee(msg.sender);

        // Store request details
        $.requests[requestId] = InputVerificationRequest({
            commitment: commitment,
            userAddress: userAddress,
            contractChainId: contractChainId,
            contractAddress: contractAddress,
            fee: msg.value,
            timestamp: block.timestamp
        });

        emit InputVerificationRegistered(
            requestId,
            commitment,
            userAddress,
            contractChainId,
            contractAddress,
            userSignature,
            block.timestamp
        );

        return requestId;
    }

    /**
     * @notice See {IInputVerificationRegistry-getRequest}.
     */
    function getRequest(uint256 requestId) external view virtual returns (
        bytes32 commitment,
        address userAddress,
        uint256 contractChainId,
        address contractAddress,
        uint256 fee,
        uint256 timestamp
    ) {
        InputVerificationRegistryStorage storage $ = _getInputVerificationRegistryStorage();
        InputVerificationRequest storage request = $.requests[requestId];
        return (
            request.commitment,
            request.userAddress,
            request.contractChainId,
            request.contractAddress,
            request.fee,
            request.timestamp
        );
    }

    /**
     * @notice See {IInputVerificationRegistry-getVersion}.
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
     * @notice Returns the InputVerificationRegistry storage location.
     */
    function _getInputVerificationRegistryStorage() internal pure returns (InputVerificationRegistryStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := INPUT_VERIFICATION_REGISTRY_STORAGE_LOCATION
        }
    }
}
