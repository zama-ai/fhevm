// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {EIP712UpgradeableCrossChain} from "./shared/EIP712UpgradeableCrossChain.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {IProtocolConfig} from "./interfaces/IProtocolConfig.sol";
import {protocolConfigAdd} from "../addresses/FHEVMHostAddresses.sol";

/**
 * @title   KMSVerifier.
 * @notice  KMSVerifier (Key Management System Verifier) is a contract that allows the management of signers and provides
 *          signature verification functions.
 * @dev     The contract uses EIP712UpgradeableCrossChain for cryptographic operations and is deployed using an UUPS proxy.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract KMSVerifier is UUPSUpgradeableEmptyProxy, EIP712UpgradeableCrossChain, ACLOwnable {
    /// @notice Returned if the recovered KMS signer is not a valid KMS signer.
    /// @param invalidSigner Address of the invalid signer.
    error KMSInvalidSigner(address invalidSigner);

    /// @notice Returned if the deserializing of the decryption proof fails.
    error DeserializingDecryptionProofFail();

    /// @notice Returned if the deserializing of the extra data fails.
    error DeserializingExtraDataFail();

    /// @notice Returned if the decryption proof is empty.
    error EmptyDecryptionProof();

    /// @notice                 Returned if the number of signatures is inferior to the threshold.
    /// @param numSignatures    Number of signatures.
    error KMSSignatureThresholdNotReached(uint256 numSignatures);

    /// @notice Returned if the number of signatures is equal to 0.
    error KMSZeroSignature();

    /// @notice Returned if the extra data version is unsupported.
    /// @param version The unsupported version byte.
    error UnsupportedExtraDataVersion(uint8 version);

    /// @notice Returned if the hardwired ProtocolConfig is not deployed or not initialized.
    error ProtocolConfigNotReady();

    /// @notice The typed data structure for the EIP712 signature to validate in public decryption responses.
    /// @dev The name of this struct is not relevant for the signature validation, only the one defined
    /// @dev EIP712_PUBLIC_DECRYPT_TYPE is, but we keep it the same for clarity.
    struct PublicDecryptVerification {
        /// @notice The handles of the ciphertexts that have been decrypted.
        bytes32[] ctHandles;
        /// @notice The decrypted result of the public decryption.
        bytes decryptedResult;
        /// @notice Generic bytes metadata for versioned payloads.
        bytes extraData;
    }

    /// @notice Decryption result type.
    string public constant EIP712_PUBLIC_DECRYPT_TYPE =
        "PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)";

    /// @notice Decryption result typehash.
    bytes32 public constant DECRYPTION_RESULT_TYPEHASH = keccak256(bytes(EIP712_PUBLIC_DECRYPT_TYPE));

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "KMSVerifier";

    /// @notice Name of the source contract for which original EIP712 was destinated.
    string private constant CONTRACT_NAME_SOURCE = "Decryption";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 3;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @custom:storage-location erc7201:fhevm.storage.KMSVerifier
    /// @dev Deprecated legacy storage retained for upgrade safety. Reads now delegate to ProtocolConfig.
    struct KMSVerifierStorage {
        mapping(address => bool) isSigner;
        address[] signers;
        uint256 threshold;
        uint256 currentKmsContextId;
        mapping(uint256 => address[]) contextSigners;
        mapping(uint256 => mapping(address => bool)) contextIsSigner;
        mapping(uint256 => uint256) contextThreshold;
        mapping(uint256 => bool) destroyedContexts;
    }

    /// @dev Shared between `initializeFromEmptyProxy` and `reinitializeV3`.
    uint64 private constant REINITIALIZER_VERSION = 4;

    /// @notice Canonical ProtocolConfig used for context reads.
    IProtocolConfig private constant PROTOCOL_CONFIG = IProtocolConfig(protocolConfigAdd);

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.KMSVerifier")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant KMSVerifierStorageLocation =
        0x7e81a744be86773af8644dd7304fa1dc9350ccabf16cfcaa614ddb78b4ce8900;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Re-initializes the contract.
     * @param verifyingContractSource The Decryption contract address from the Gateway chain.
     * @param chainIDSource The chain id of the Gateway chain.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        address verifyingContractSource,
        uint64 chainIDSource
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        _requireProtocolConfigReady();
        __EIP712_init(CONTRACT_NAME_SOURCE, "1", verifyingContractSource, chainIDSource);
    }

    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV3() public virtual onlyACLOwner reinitializer(REINITIALIZER_VERSION) {
        _requireProtocolConfigReady();
    }

    /**
     * @notice                  Verifies multiple signatures for a given handlesList and a given decryptedResult.
     * @dev                     Calls verifySignaturesDigest internally.
     * @param handlesList       The list of handles, which where requested to be decrypted.
     * @param decryptedResult   A bytes array representing the abi-encoding of all requested decrypted values.
     * @param decryptionProof   Decryption proof containing KMS signatures and extra data.
     * @return isVerified       true if enough provided signatures are valid, false otherwise.
     */
    function verifyDecryptionEIP712KMSSignatures(
        bytes32[] memory handlesList,
        bytes memory decryptedResult,
        bytes memory decryptionProof
    ) public virtual returns (bool) {
        if (decryptionProof.length == 0) {
            revert EmptyDecryptionProof();
        }

        /// @dev The decryptionProof is the numSigners + kmsSignatures + extraData (1 + 65*numSigners + extraData bytes)
        uint256 numSigners = uint256(uint8(decryptionProof[0]));

        /// @dev The extraData is the rest of the decryptionProof bytes after the numSigners + signatures.
        uint256 extraDataOffset = 1 + 65 * numSigners;

        /// @dev Check that the decryptionProof is long enough to contain at least the numSigners + kmsSignatures.
        if (decryptionProof.length < extraDataOffset) {
            revert DeserializingDecryptionProofFail();
        }

        bytes[] memory signatures = new bytes[](numSigners);
        for (uint256 j = 0; j < numSigners; j++) {
            signatures[j] = new bytes(65);
            for (uint256 i = 0; i < 65; i++) {
                signatures[j][i] = decryptionProof[1 + 65 * j + i];
            }
        }

        /// @dev Extract the extraData from the decryptionProof.
        uint256 extraDataSize = decryptionProof.length - extraDataOffset;
        bytes memory extraData = new bytes(extraDataSize);
        for (uint i = 0; i < extraDataSize; i++) {
            extraData[i] = decryptionProof[extraDataOffset + i];
        }

        PublicDecryptVerification memory publicDecryptVerification = PublicDecryptVerification(
            handlesList,
            decryptedResult,
            extraData
        );
        bytes32 digest = _hashDecryptionResult(publicDecryptVerification);

        uint256 kmsContextId = _extractKmsContextId(extraData);
        return _verifySignaturesDigestForContext(digest, signatures, kmsContextId);
    }

    /**
     * @notice          Returns the list of KMS signers.
     * @dev             If there are too many signers, it could be out-of-gas.
     * @return signers  List of signers.
     */
    function getKmsSigners() public view virtual returns (address[] memory) {
        return PROTOCOL_CONFIG.getKmsSigners();
    }

    /**
     * @notice              Returns whether the account address is a valid KMS signer.
     * @param account       Account address.
     * @return isSigner     Whether the account is a valid KMS signer.
     */
    function isSigner(address account) public view virtual returns (bool) {
        return PROTOCOL_CONFIG.isKmsSigner(account);
    }

    /**
     * @notice              Returns the current KMS context ID.
     * @return contextId    The current KMS context ID.
     */
    function getCurrentKmsContextId() public view virtual returns (uint256) {
        return PROTOCOL_CONFIG.getCurrentKmsContextId();
    }

    /**
     * @notice              Returns the list of signers for a given KMS context.
     * @dev                 Reverts if the context doesn't exist or has been destroyed.
     * @param kmsContextId  The context ID.
     * @return signers      The list of signers for the context.
     */
    function getSignersForKmsContext(uint256 kmsContextId) public view virtual returns (address[] memory) {
        return PROTOCOL_CONFIG.getKmsSignersForContext(kmsContextId);
    }

    /**
     * @notice              Resolves extraData into the context-specific signers and threshold.
     * @dev                 Parses the version-tagged extraData to extract the context ID, validates
     *                      that the context exists and is not destroyed, then returns the corresponding
     *                      signer set and threshold. Reverts on invalid extraData, non-existent, or
     *                      destroyed contexts.
     * @param extraData     The extra data bytes from the decryption proof.
     * @return signers      The list of signers for the resolved context.
     * @return threshold    The threshold for the resolved context.
     */
    function getContextSignersAndThresholdFromExtraData(
        bytes calldata extraData
    ) external view virtual returns (address[] memory signers, uint256 threshold) {
        uint256 kmsContextId = _extractKmsContextId(extraData);
        return (
            PROTOCOL_CONFIG.getKmsSignersForContext(kmsContextId),
            PROTOCOL_CONFIG.getPublicDecryptionThresholdForContext(kmsContextId)
        );
    }

    /**
     * @notice        Getter for the name and version of the contract.
     * @return string Name and the version of the contract.
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
     * @notice          Cleans a hashmap in transient storage.
     * @dev             This is important to keep composability in the context of account abstraction.
     * @param keys      An array of keys to cleanup from transient storage.
     * @param maxIndex  The biggest index to take into account from the array - assumed to be less or equal to keys.length.
     */
    function _cleanTransientHashMap(address[] memory keys, uint256 maxIndex) internal virtual {
        for (uint256 j = 0; j < maxIndex; j++) {
            _tstore(keys[j], 0);
        }
    }

    /**
     * @notice          Writes to transient storage.
     * @dev             Uses inline assembly to access the Transient Storage's _tstore operation.
     * @param location  The address used as key where transient storage of the contract is written at.
     * @param value     An uint256 stored at location key in transient storage of the contract.
     */
    function _tstore(address location, uint256 value) internal virtual {
        assembly {
            tstore(location, value)
        }
    }

    /**
     * @notice              Extracts the KMS context ID from extra data.
     * @param extraData     The extra data bytes from the decryption proof.
     * @return contextId    The extracted KMS context ID.
     */
    function _extractKmsContextId(bytes memory extraData) internal view virtual returns (uint256) {
        // v0 (0x00 prefix or empty): uses the current context. Trailing bytes are
        // ignored for forward-compatibility with potential v0 extensions.
        if (extraData.length == 0 || uint8(extraData[0]) == 0x00) {
            return PROTOCOL_CONFIG.getCurrentKmsContextId();
        }
        uint8 version = uint8(extraData[0]);
        if (version == 0x01) {
            // v1 (0x01 prefix): reads a 32-byte context ID starting at byte 1.
            // Trailing bytes after byte 33 are ignored for forward-compatibility
            // with potential v1 extensions.
            if (extraData.length < 33) {
                revert DeserializingExtraDataFail();
            }
            uint256 contextId;
            // Memory layout: [32-byte length][version byte][32-byte contextId][...]
            // mload(add(extraData, 33)) reads 32 bytes starting at offset 1 (after version byte).
            assembly {
                contextId := mload(add(extraData, 33))
            }
            return contextId;
        }
        revert UnsupportedExtraDataVersion(version);
    }

    /**
     * @notice              Verifies multiple signatures for a given message using context-aware verification.
     * @param digest        The hash of the message that was signed by all signers.
     * @param signatures    An array of signatures to verify.
     * @param kmsContextId  The KMS context ID to verify against.
     * @return isVerified   true if enough provided signatures are valid, false otherwise.
     */
    function _verifySignaturesDigestForContext(
        bytes32 digest,
        bytes[] memory signatures,
        uint256 kmsContextId
    ) internal virtual returns (bool) {
        uint256 numSignatures = signatures.length;
        if (numSignatures == 0) {
            revert KMSZeroSignature();
        }

        uint256 threshold = PROTOCOL_CONFIG.getPublicDecryptionThresholdForContext(kmsContextId);
        if (numSignatures < threshold) {
            revert KMSSignatureThresholdNotReached(numSignatures);
        }

        address[] memory recoveredSigners = new address[](numSignatures);
        uint256 uniqueValidCount;
        for (uint256 i = 0; i < numSignatures; i++) {
            address signerRecovered = _recoverSigner(digest, signatures[i]);
            if (!PROTOCOL_CONFIG.isKmsSignerForContext(kmsContextId, signerRecovered)) {
                revert KMSInvalidSigner(signerRecovered);
            }
            if (!_tload(signerRecovered)) {
                recoveredSigners[uniqueValidCount] = signerRecovered;
                uniqueValidCount++;
                _tstore(signerRecovered, 1);
            }
            if (uniqueValidCount >= threshold) {
                _cleanTransientHashMap(recoveredSigners, uniqueValidCount);
                return true;
            }
        }
        _cleanTransientHashMap(recoveredSigners, uniqueValidCount);
        return false;
    }

    /**
     * @dev Reverts unless the hardwired ProtocolConfig is deployed and initialized.
     *      An uninitialized ProtocolConfig returns 0 for the counter; a bad address
     *      reverts the external call via Solidity's implicit extcodesize check.
     */
    function _requireProtocolConfigReady() internal view virtual {
        if (PROTOCOL_CONFIG.getCurrentKmsContextId() == 0) {
            revert ProtocolConfigNotReady();
        }
    }

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}

    /**
     * @notice                  Hashes the decryption result.
     * @param decRes            Decryption result.
     * @return hashTypedData    Hash typed data.
     */
    function _hashDecryptionResult(PublicDecryptVerification memory decRes) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        DECRYPTION_RESULT_TYPEHASH,
                        keccak256(abi.encodePacked(decRes.ctHandles)),
                        keccak256(decRes.decryptedResult),
                        keccak256(abi.encodePacked(decRes.extraData))
                    )
                )
            );
    }

    /**
     * @notice           Reads transient storage.
     * @dev              Uses inline assembly to access the Transient Storage's tload operation.
     * @param location   The address used as key where transient storage of the contract is read at.
     * @return value     true if value stored at the given location is non-null, false otherwise.
     */
    function _tload(address location) internal view virtual returns (bool value) {
        assembly {
            value := tload(location)
        }
    }

    /**
     * @notice          Recovers the signer's address from a `signature` and a `message` digest.
     * @dev             It utilizes ECDSA for actual address recovery. It does not support contract signature (EIP-1271).
     * @param message   The hash of the message that was signed.
     * @param signature The signature to verify.
     * @return signer   The address that supposedly signed the message.
     */
    function _recoverSigner(bytes32 message, bytes memory signature) internal pure virtual returns (address) {
        address signerRecovered = ECDSA.recover(message, signature);
        return signerRecovered;
    }
}
