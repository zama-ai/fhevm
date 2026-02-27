// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {EIP712UpgradeableCrossChain} from "./shared/EIP712UpgradeableCrossChain.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";

/**
 * @title   KMSVerifier.
 * @notice  KMSVerifier (Key Management System Verifier) is a contract that allows the management of signers and provides
 *          signature verification functions.
 * @dev     The contract uses EIP712UpgradeableCrossChain for cryptographic operations and is deployed using an UUPS proxy.
 */
contract KMSVerifier is UUPSUpgradeableEmptyProxy, EIP712UpgradeableCrossChain, ACLOwnable {
    /// @notice Returned if the KMS signer to add is already a signer.
    error KMSAlreadySigner();

    /// @notice Returned if the recovered KMS signer is not a valid KMS signer.
    /// @param invalidSigner Address of the invalid signer.
    error KMSInvalidSigner(address invalidSigner);

    /// @notice Returned if the deserializing of the decryption proof fails.
    error DeserializingDecryptionProofFail();

    /// @notice Returned if the deserializing of the extra data fails.
    error DeserializingExtraDataFail();

    /// @notice Returned if the decryption proof is empty.
    error EmptyDecryptionProof();

    /// @notice Returned if the KMS signer to add is the null address.
    error KMSSignerNull();

    /// @notice                 Returned if the number of signatures is inferior to the threshold.
    /// @param numSignatures    Number of signatures.
    error KMSSignatureThresholdNotReached(uint256 numSignatures);

    /// @notice Returned if the number of signatures is equal to 0.
    error KMSZeroSignature();

    /// @notice Returned if the signers set is empty.
    error SignersSetIsEmpty();

    /// @notice Returned if the chosen threshold is null.
    error ThresholdIsNull();

    /// @notice Threshold is above number of signers.
    error ThresholdIsAboveNumberOfSigners();

    /// @notice Returned if the KMS context does not exist or has been destroyed.
    /// @param kmsContextId The non-existent context ID.
    error InvalidKMSContext(uint256 kmsContextId);

    /// @notice Returned if attempting to destroy the current context.
    /// @param kmsContextId The current context ID.
    error CurrentKMSContextCannotBeDestroyed(uint256 kmsContextId);

    /// @notice Returned if the extra data version is unsupported.
    /// @param version The unsupported version byte.
    error UnsupportedExtraDataVersion(uint8 version);

    /// @notice         Emitted when a context is set or changed.
    /// @param kmsContextId      The context ID.
    /// @param newKmsSignersSet   The set of new KMS signers.
    /// @param newThreshold   The new threshold set by the owner.
    event NewContextSet(uint256 indexed kmsContextId, address[] newKmsSignersSet, uint256 newThreshold);

    /// @notice         Emitted when a KMS context is destroyed.
    /// @param kmsContextId      The destroyed context ID.
    event KMSContextDestroyed(uint256 indexed kmsContextId);

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
    uint256 private constant MINOR_VERSION = 2;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @notice Base value for KMS context IDs. Format: [0x07 type tag | 31 counter bytes].
    /// @dev See KMSRequestCounters on Gateway for the shared counter scheme.
    uint256 private constant KMS_CONTEXT_COUNTER_BASE = uint256(0x07) << 248;

    /// @custom:storage-location erc7201:fhevm.storage.KMSVerifier
    struct KMSVerifierStorage {
        /// @dev Deprecated. Use `contextIsSigner` instead.
        mapping(address => bool) isSigner;
        /// @dev Deprecated. Use `contextSigners` instead.
        address[] signers;
        /// @dev Deprecated. Use `contextThreshold` instead.
        uint256 threshold;
        /// @notice Current KMS context ID counter.
        uint256 currentKmsContextId;
        /// @notice Ordered signer list per context ID.
        mapping(uint256 => address[]) contextSigners;
        /// @notice Fast signer membership check per context ID.
        mapping(uint256 => mapping(address => bool)) contextIsSigner;
        /// @notice Required signature threshold per context ID.
        mapping(uint256 => uint256) contextThreshold;
        /// @notice Whether a context ID has been destroyed.
        mapping(uint256 => bool) destroyedContexts;
    }

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 3;

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
     * @param initialSigners The list of initial KMS signers, should be non-empty and contain unique addresses, otherwise initialization will fail.
     * @param initialThreshold Initial threshold, should be non-null and less or equal to the initialSigners length.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        address verifyingContractSource,
        uint64 chainIDSource,
        address[] calldata initialSigners,
        uint256 initialThreshold
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __EIP712_init(CONTRACT_NAME_SOURCE, "1", verifyingContractSource, chainIDSource);
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        $.currentKmsContextId = KMS_CONTEXT_COUNTER_BASE;
        defineNewContext(initialSigners, initialThreshold);
    }

    /**
     * @notice Re-initializes the contract from V1 to V2 with context-aware KMS support.
     * @dev Migrates existing signers into the first context. The legacy mapping retains its pre-migration
     *      values, which should be considered stale after this call.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2() public virtual onlyACLOwner reinitializer(REINITIALIZER_VERSION) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        $.currentKmsContextId = KMS_CONTEXT_COUNTER_BASE;
        defineNewContext($.signers, $.threshold);
    }

    /**
     * @notice          Sets a new context (i.e. new set of unique signers and new threshold).
     * @dev             Only the owner can set a new context.
     * @param newSignersSet   The new set of signers to be set. This array should not be empty and without duplicates nor null values.
     * @param newThreshold    The threshold to be set. Threshold should be non-null and less than the number of signers.
     */
    function defineNewContext(address[] memory newSignersSet, uint256 newThreshold) public virtual onlyACLOwner {
        if (newSignersSet.length == 0) {
            revert SignersSetIsEmpty();
        }

        KMSVerifierStorage storage $ = _getKMSVerifierStorage();

        $.currentKmsContextId++;

        _setContextSigners($.currentKmsContextId, newSignersSet);
        _setContextThreshold($.currentKmsContextId, newThreshold);

        emit NewContextSet($.currentKmsContextId, newSignersSet, newThreshold);
    }

    /**
     * @notice              Destroys a KMS context, preventing it from being used for verification.
     * @dev                 Only the owner can destroy a context. The current context cannot be destroyed.
     * @param kmsContextId  The ID of the context to destroy.
     */
    function destroyKmsContext(uint256 kmsContextId) public virtual onlyACLOwner {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        if (kmsContextId == $.currentKmsContextId) {
            revert CurrentKMSContextCannotBeDestroyed(kmsContextId);
        }
        if (!_isValidKmsContext(kmsContextId)) {
            revert InvalidKMSContext(kmsContextId);
        }
        $.destroyedContexts[kmsContextId] = true;

        emit KMSContextDestroyed(kmsContextId);
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
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.contextSigners[$.currentKmsContextId];
    }

    /**
     * @notice              Get the threshold for signature.
     * @return threshold    Threshold for signature verification.
     */
    function getThreshold() public view virtual returns (uint256) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.contextThreshold[$.currentKmsContextId];
    }

    /**
     * @notice              Returns whether the account address is a valid KMS signer.
     * @param account       Account address.
     * @return isSigner     Whether the account is a valid KMS signer.
     */
    function isSigner(address account) public view virtual returns (bool) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.contextIsSigner[$.currentKmsContextId][account];
    }

    /**
     * @notice              Returns the current KMS context ID.
     * @return contextId    The current KMS context ID.
     */
    function getCurrentKmsContextId() public view virtual returns (uint256) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.currentKmsContextId;
    }

    /**
     * @notice              Returns the list of signers for a given KMS context.
     * @param kmsContextId  The context ID.
     * @return signers      The list of signers for the context, or empty if context doesn't exist or is destroyed.
     */
    function getSignersForKmsContext(uint256 kmsContextId) public view virtual returns (address[] memory) {
        if (!_isValidKmsContext(kmsContextId)) {
            return new address[](0);
        }
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.contextSigners[kmsContextId];
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
        if (!_isValidKmsContext(kmsContextId)) {
            revert InvalidKMSContext(kmsContextId);
        }
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return ($.contextSigners[kmsContextId], $.contextThreshold[kmsContextId]);
    }

    /**
     * @notice              Validates whether a KMS context exists and is not destroyed.
     * @param kmsContextId  The context ID.
     * @return isValid      true if the context exists and is not destroyed, false otherwise.
     */
    function isValidKmsContext(uint256 kmsContextId) public view virtual returns (bool) {
        return _isValidKmsContext(kmsContextId);
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
     * @notice              Validates and writes a threshold to context-aware storage.
     * @dev                 Validates against contextSigners[contextId].length.
     * @param contextId     The context to update.
     * @param threshold_    The threshold to set. Must be non-zero and <= context signer count.
     */
    function _setContextThreshold(uint256 contextId, uint256 threshold_) internal virtual {
        if (threshold_ == 0) revert ThresholdIsNull();
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        if (threshold_ > $.contextSigners[contextId].length) {
            revert ThresholdIsAboveNumberOfSigners();
        }
        $.contextThreshold[contextId] = threshold_;
    }

    /**
     * @notice              Checks whether a KMS context ID is within the allocated range.
     * @param kmsContextId  The context ID to check.
     * @return inRange      true if the context ID is within the valid range.
     */
    function _contextIdInRange(uint256 kmsContextId) internal view virtual returns (bool) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return kmsContextId >= KMS_CONTEXT_COUNTER_BASE + 1 && kmsContextId <= $.currentKmsContextId;
    }

    /**
     * @notice              Checks whether a KMS context ID is within the allocated range and not destroyed.
     * @param kmsContextId  The context ID to check.
     * @return              true if the context ID is in range and not destroyed.
     */
    function _isValidKmsContext(uint256 kmsContextId) internal view virtual returns (bool) {
        return _contextIdInRange(kmsContextId) && !_getKMSVerifierStorage().destroyedContexts[kmsContextId];
    }

    /**
     * @notice              Validates and stores context signers for a given context ID.
     * @dev                 Reverts on null or duplicate addresses. Must only be called once
     *                      per contextId (appends without clearing).
     * @param contextId     The context ID.
     * @param signersList   The list of signers.
     */
    function _setContextSigners(uint256 contextId, address[] memory signersList) internal virtual {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        for (uint256 i = 0; i < signersList.length; i++) {
            address signer = signersList[i];
            if (signer == address(0)) {
                revert KMSSignerNull();
            }
            if ($.contextIsSigner[contextId][signer]) {
                revert KMSAlreadySigner();
            }
            $.contextSigners[contextId].push(signer);
            $.contextIsSigner[contextId][signer] = true;
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
            KMSVerifierStorage storage $ = _getKMSVerifierStorage();
            return $.currentKmsContextId;
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

        KMSVerifierStorage storage $ = _getKMSVerifierStorage();

        if (!_isValidKmsContext(kmsContextId)) {
            revert InvalidKMSContext(kmsContextId);
        }

        uint256 threshold = $.contextThreshold[kmsContextId];
        if (numSignatures < threshold) {
            revert KMSSignatureThresholdNotReached(numSignatures);
        }

        address[] memory recoveredSigners = new address[](numSignatures);
        uint256 uniqueValidCount;
        for (uint256 i = 0; i < numSignatures; i++) {
            address signerRecovered = _recoverSigner(digest, signatures[i]);
            if (!$.contextIsSigner[kmsContextId][signerRecovered]) {
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
     * @dev Returns the KMSVerifier storage location.
     */
    function _getKMSVerifierStorage() internal pure returns (KMSVerifierStorage storage $) {
        assembly {
            $.slot := KMSVerifierStorageLocation
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
