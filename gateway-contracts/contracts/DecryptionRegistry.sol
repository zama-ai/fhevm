// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { gatewayConfigAddress } from "../addresses/GatewayAddresses.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { IDecryptionRegistry } from "./interfaces/IDecryptionRegistry.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayConfigChecks } from "./shared/GatewayConfigChecks.sol";
import { GatewayOwnable } from "./shared/GatewayOwnable.sol";
import { ProtocolPaymentUtils } from "./shared/ProtocolPaymentUtils.sol";
import { Pausable } from "./shared/Pausable.sol";

/**
 * @title DecryptionRegistry contract
 * @notice See {IDecryptionRegistry}.
 * @dev V2 Design: This contract handles decryption request registration only.
 * Unlike V1 Decryption.sol:
 * - Does NOT handle response processing (done off-chain via KMS HTTP API)
 * - Does NOT depend on CiphertextCommits (ciphertexts fetched from Coprocessor API)
 * - Does NOT depend on MultichainACL (ACL checked on Host Chain by KMS)
 * - Events emit handles only (not full SnsCiphertextMaterial[])
 */
contract DecryptionRegistry is
    IDecryptionRegistry,
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
    string private constant CONTRACT_NAME = "DecryptionRegistry";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for the `reinitializer` modifier.
     */
    uint64 private constant REINITIALIZER_VERSION = 2;

    /**
     * @notice Request ID counter base for user decryption requests.
     * @dev Format: [0000 0010 | counter_1..31] for uniqueness across request types.
     */
    uint256 private constant USER_DECRYPT_COUNTER_BASE = 0x0200000000000000000000000000000000000000000000000000000000000000;

    /**
     * @notice Request ID counter base for public decryption requests.
     * @dev Format: [0000 0001 | counter_1..31] for uniqueness across request types.
     */
    uint256 private constant PUBLIC_DECRYPT_COUNTER_BASE = 0x0100000000000000000000000000000000000000000000000000000000000000;

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.DecryptionRegistry
    struct DecryptionRegistryStorage {
        /// @notice Counter for user decryption requests (including delegated).
        uint256 userDecryptionCounter;
        /// @notice Counter for public decryption requests.
        uint256 publicDecryptionCounter;
    }

    /**
     * @dev Storage location computed using:
     * keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.DecryptionRegistry")) - 1))
     * & ~bytes32(uint256(0xff))
     */
    bytes32 private constant DECRYPTION_REGISTRY_STORAGE_LOCATION =
        0xb1c7a90bb5b79766e8d1a00bb5b79766e8d1a00bb5b79766e8d1a00bb5b79700;

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

        DecryptionRegistryStorage storage $ = _getDecryptionRegistryStorage();
        $.userDecryptionCounter = USER_DECRYPT_COUNTER_BASE;
        $.publicDecryptionCounter = PUBLIC_DECRYPT_COUNTER_BASE;
    }

    /**
     * @notice See {IDecryptionRegistry-requestUserDecryption}.
     */
    function requestUserDecryption(
        bytes32[] calldata handles,
        address[] calldata contractAddresses,
        address userAddress,
        bytes calldata publicKey,
        bytes calldata signature
    ) external payable virtual whenNotPaused returns (uint256) {
        // Validate inputs
        if (handles.length == 0) {
            revert EmptyHandles();
        }
        if (handles.length != contractAddresses.length) {
            revert HandleContractAddressLengthMismatch(handles.length, contractAddresses.length);
        }
        if (publicKey.length == 0) {
            revert EmptyPublicKey();
        }
        if (signature.length == 0) {
            revert EmptySignature();
        }

        DecryptionRegistryStorage storage $ = _getDecryptionRegistryStorage();

        // Generate unique request ID
        $.userDecryptionCounter++;
        uint256 requestId = $.userDecryptionCounter;

        // Collect fee for user decryption
        _collectUserDecryptionFee(msg.sender);

        // Extract chainId from the first handle (all handles should be from same chain)
        uint256 chainId = _extractChainIdFromHandle(handles[0]);

        emit UserDecryptionRequested(
            requestId,
            handles,
            contractAddresses,
            userAddress,
            publicKey,
            signature,
            chainId,
            block.timestamp
        );

        return requestId;
    }

    /**
     * @notice See {IDecryptionRegistry-requestPublicDecryption}.
     */
    function requestPublicDecryption(
        bytes32[] calldata handles,
        address[] calldata contractAddresses
    ) external payable virtual whenNotPaused returns (uint256) {
        // Validate inputs
        if (handles.length == 0) {
            revert EmptyHandles();
        }
        if (handles.length != contractAddresses.length) {
            revert HandleContractAddressLengthMismatch(handles.length, contractAddresses.length);
        }

        DecryptionRegistryStorage storage $ = _getDecryptionRegistryStorage();

        // Generate unique request ID
        $.publicDecryptionCounter++;
        uint256 requestId = $.publicDecryptionCounter;

        // Collect fee for public decryption
        _collectPublicDecryptionFee(msg.sender);

        // Extract chainId from the first handle (all handles should be from same chain)
        uint256 chainId = _extractChainIdFromHandle(handles[0]);

        emit PublicDecryptionRequested(
            requestId,
            handles,
            contractAddresses,
            chainId,
            block.timestamp
        );

        return requestId;
    }

    /**
     * @notice See {IDecryptionRegistry-getVersion}.
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
     * @notice Extracts the chain ID from a ciphertext handle.
     * @param handle The ciphertext handle.
     * @return chainId The extracted chain ID.
     */
    function _extractChainIdFromHandle(bytes32 handle) internal pure returns (uint256) {
        // Chain ID is stored in bytes 2-9 of the handle (8 bytes = uint64)
        return uint256(uint64(uint256(handle) >> 184));
    }

    /**
     * @notice Checks if the sender is authorized to upgrade the contract and reverts otherwise.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}

    /**
     * @notice Returns the DecryptionRegistry storage location.
     */
    function _getDecryptionRegistryStorage() internal pure returns (DecryptionRegistryStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := DECRYPTION_REGISTRY_STORAGE_LOCATION
        }
    }
}
