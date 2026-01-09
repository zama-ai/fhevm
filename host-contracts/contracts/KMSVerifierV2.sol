// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {EIP712UpgradeableCrossChain} from "./shared/EIP712UpgradeableCrossChain.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";

/**
 * @title   KMSVerifierV2.
 * @notice  KMSVerifierV2 (Key Management System Verifier V2) is a contract that allows the management of signers
 *          with epoch-based grace period support for seamless key rotations.
 * @dev     The contract supports a transition period during which both the current and previous epoch signers
 *          are valid. This enables safe key rotations without service interruption.
 *
 *          Epoch Grace Period State Machine:
 *          ┌────────────────┐   defineNewContext()   ┌─────────────────────┐
 *          │ EPOCH_N_ACTIVE │───────────────────────>│ TRANSITION_PERIOD   │
 *          │                │                        │ (grace period)      │
 *          │ signers: [A,B] │                        │                     │
 *          │ threshold: 2   │                        │ current: [C,D]      │
 *          └────────────────┘                        │ previous: [A,B]     │
 *                                                    │ BOTH valid          │
 *                                                    └──────────┬──────────┘
 *                                                               │
 *                                                               │ grace period expires
 *                                                               ▼
 *                                                    ┌─────────────────────┐
 *                                                    │ EPOCH_N+1_ACTIVE    │
 *                                                    │                     │
 *                                                    │ signers: [C,D]      │
 *                                                    │ threshold: 2        │
 *                                                    │ previous cleared    │
 *                                                    └─────────────────────┘
 */
contract KMSVerifierV2 is UUPSUpgradeableEmptyProxy, EIP712UpgradeableCrossChain, ACLOwnable {
    /// @notice Returned if the KMS signer to add is already a signer.
    error KMSAlreadySigner();

    /// @notice Returned if the recovered KMS signer is not a valid KMS signer.
    /// @param invalidSigner Address of the invalid signer.
    error KMSInvalidSigner(address invalidSigner);

    /// @notice Returned if the deserializing of the decryption proof fails.
    error DeserializingDecryptionProofFail();

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

    /// @notice Returned if grace period duration is zero.
    error GracePeriodDurationIsZero();

    /// @notice         Emitted when a new context is set, starting a transition period.
    /// @param epochId          The new epoch ID.
    /// @param newKmsSignersSet The set of new KMS signers.
    /// @param newThreshold     The new threshold set by the owner.
    /// @param gracePeriodEnd   Timestamp when the grace period ends.
    event NewContextSet(uint256 indexed epochId, address[] newKmsSignersSet, uint256 newThreshold, uint256 gracePeriodEnd);

    /// @notice         Emitted when the grace period expires and previous epoch is cleared.
    /// @param epochId  The epoch ID that is now exclusively active.
    event GracePeriodEnded(uint256 indexed epochId);

    /// @notice The typed data structure for the EIP712 signature to validate in public decryption responses.
    struct PublicDecryptVerification {
        /// @notice The handles of the ciphertexts that have been decrypted.
        bytes32[] ctHandles;
        /// @notice The decrypted result of the public decryption.
        bytes decryptedResult;
        /// @notice Generic bytes metadata for versioned payloads.
        bytes extraData;
    }

    /// @notice Struct to hold epoch-specific signer data.
    struct EpochSigners {
        mapping(address => bool) isSigner;
        address[] signers;
        uint256 threshold;
        bool isActive;
    }

    /// @notice Decryption result type.
    string public constant EIP712_PUBLIC_DECRYPT_TYPE =
        "PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)";

    /// @notice Decryption result typehash.
    bytes32 public constant DECRYPTION_RESULT_TYPEHASH = keccak256(bytes(EIP712_PUBLIC_DECRYPT_TYPE));

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "KMSVerifierV2";

    /// @notice Name of the source contract for which original EIP712 was destinated.
    string private constant CONTRACT_NAME_SOURCE = "Decryption";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 2;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 0;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @notice Default grace period duration (1 hour).
    uint256 public constant DEFAULT_GRACE_PERIOD_DURATION = 1 hours;

    /// @custom:storage-location erc7201:fhevm.storage.KMSVerifierV2
    struct KMSVerifierV2Storage {
        /// @notice Current epoch ID (increments on each defineNewContext call).
        uint256 currentEpochId;
        /// @notice Timestamp when the grace period ends (0 if not in transition).
        uint256 gracePeriodEnd;
        /// @notice Duration of grace periods.
        uint256 gracePeriodDuration;
        /// @notice Current epoch signers.
        mapping(address => bool) currentIsSigner;
        address[] currentSigners;
        uint256 currentThreshold;
        /// @notice Previous epoch signers (only valid during grace period).
        mapping(address => bool) previousIsSigner;
        address[] previousSigners;
        uint256 previousThreshold;
    }

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 2;

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.KMSVerifierV2")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant KMSVerifierV2StorageLocation =
        0x8b9e5c5c1b9e5c5c1b9e5c5c1b9e5c5c1b9e5c5c1b9e5c5c1b9e5c5c1b9e5c00;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract from an empty proxy.
     * @param verifyingContractSource The Decryption contract address from the Gateway chain.
     * @param chainIDSource The chain id of the Gateway chain.
     * @param initialSigners The list of initial KMS signers, should be non-empty and contain unique addresses.
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
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        $.gracePeriodDuration = DEFAULT_GRACE_PERIOD_DURATION;
        _setCurrentEpoch(initialSigners, initialThreshold);
        $.currentEpochId = 1;
        emit NewContextSet(1, initialSigners, initialThreshold, 0);
    }

    /**
     * @notice          Sets a new context (i.e. new set of unique signers and new threshold).
     * @dev             Only the owner can set a new context. This starts a grace period during which
     *                  both the new and previous signers are valid.
     * @param newSignersSet   The new set of signers to be set. Should not be empty, without duplicates or null values.
     * @param newThreshold    The threshold to be set. Should be non-null and <= number of signers.
     */
    function defineNewContext(address[] memory newSignersSet, uint256 newThreshold) public virtual onlyACLOwner {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();

        // First, move current epoch to previous (if not in grace period, clear previous first)
        _clearPreviousEpoch();
        _moveToPreviousEpoch();

        // Set new current epoch
        _setCurrentEpoch(newSignersSet, newThreshold);

        // Increment epoch ID and set grace period
        $.currentEpochId++;
        $.gracePeriodEnd = block.timestamp + $.gracePeriodDuration;

        emit NewContextSet($.currentEpochId, newSignersSet, newThreshold, $.gracePeriodEnd);
    }

    /**
     * @notice          Sets a threshold (i.e. the minimum number of valid signatures required).
     * @dev             Only the owner can set a threshold. This does NOT trigger a grace period.
     * @param threshold The threshold to be set. Should be non-null and <= number of signers.
     */
    function setThreshold(uint256 threshold) public virtual onlyACLOwner {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        _validateThreshold(threshold, $.currentSigners.length);
        $.currentThreshold = threshold;
        emit NewContextSet($.currentEpochId, $.currentSigners, threshold, $.gracePeriodEnd);
    }

    /**
     * @notice                  Sets the grace period duration for future context changes.
     * @param newDuration       The new grace period duration in seconds.
     */
    function setGracePeriodDuration(uint256 newDuration) public virtual onlyACLOwner {
        if (newDuration == 0) {
            revert GracePeriodDurationIsZero();
        }
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        $.gracePeriodDuration = newDuration;
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

        uint256 numSigners = uint256(uint8(decryptionProof[0]));
        uint256 extraDataOffset = 1 + 65 * numSigners;

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

        return _verifySignaturesDigest(digest, signatures);
    }

    /**
     * @notice          Returns whether the account is a valid signer (checks both epochs during grace period).
     * @param account   Account address to check.
     * @return          true if the account is a valid signer in current or (during grace period) previous epoch.
     */
    function isValidSigner(address account) public view virtual returns (bool) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        
        // Always check current epoch
        if ($.currentIsSigner[account]) {
            return true;
        }
        
        // During grace period, also check previous epoch
        if (_isInGracePeriod()) {
            return $.previousIsSigner[account];
        }
        
        return false;
    }

    /**
     * @notice          Returns whether the account is a signer in the current epoch only.
     * @param account   Account address.
     * @return          Whether the account is a signer in the current epoch.
     */
    function isCurrentSigner(address account) public view virtual returns (bool) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        return $.currentIsSigner[account];
    }

    /**
     * @notice          Returns whether the account is a signer in the previous epoch.
     * @param account   Account address.
     * @return          Whether the account is a signer in the previous epoch (only meaningful during grace period).
     */
    function isPreviousSigner(address account) public view virtual returns (bool) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        return $.previousIsSigner[account];
    }

    /**
     * @notice          Returns the effective threshold (minimum of current and previous during grace period).
     * @return          The effective threshold for signature verification.
     */
    function getEffectiveThreshold() public view virtual returns (uint256) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        
        if (_isInGracePeriod() && $.previousThreshold > 0) {
            // During grace period, use the minimum threshold for safety
            return $.currentThreshold < $.previousThreshold ? $.currentThreshold : $.previousThreshold;
        }
        
        return $.currentThreshold;
    }

    /**
     * @notice          Returns the current epoch's threshold.
     * @return          Current epoch threshold.
     */
    function getThreshold() public view virtual returns (uint256) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        return $.currentThreshold;
    }

    /**
     * @notice          Returns the previous epoch's threshold.
     * @return          Previous epoch threshold (0 if not in grace period or no previous epoch).
     */
    function getPreviousThreshold() public view virtual returns (uint256) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        return $.previousThreshold;
    }

    /**
     * @notice          Returns the list of current KMS signers.
     * @return          List of current signers.
     */
    function getKmsSigners() public view virtual returns (address[] memory) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        return $.currentSigners;
    }

    /**
     * @notice          Returns the list of previous KMS signers.
     * @return          List of previous signers (empty if not in grace period).
     */
    function getPreviousKmsSigners() public view virtual returns (address[] memory) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        return $.previousSigners;
    }

    /**
     * @notice          Returns the current epoch ID.
     * @return          Current epoch ID.
     */
    function getCurrentEpochId() public view virtual returns (uint256) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        return $.currentEpochId;
    }

    /**
     * @notice          Returns the grace period end timestamp.
     * @return          Grace period end timestamp (0 if not in grace period).
     */
    function getGracePeriodEnd() public view virtual returns (uint256) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        return $.gracePeriodEnd;
    }

    /**
     * @notice          Returns the grace period duration.
     * @return          Grace period duration in seconds.
     */
    function getGracePeriodDuration() public view virtual returns (uint256) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        return $.gracePeriodDuration;
    }

    /**
     * @notice          Returns whether the contract is currently in a grace period.
     * @return          true if in grace period, false otherwise.
     */
    function isInGracePeriod() public view virtual returns (bool) {
        return _isInGracePeriod();
    }

    /**
     * @notice        Getter for the name and version of the contract.
     * @return        Name and the version of the contract.
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

    // ============ Internal Functions ============

    /**
     * @notice          Checks if we're currently in a grace period.
     * @return          true if in grace period.
     */
    function _isInGracePeriod() internal view virtual returns (bool) {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();
        return $.gracePeriodEnd > 0 && block.timestamp < $.gracePeriodEnd;
    }

    /**
     * @notice          Sets the current epoch signers and threshold.
     * @param signers   Array of signer addresses.
     * @param threshold Threshold for signatures.
     */
    function _setCurrentEpoch(address[] memory signers, uint256 threshold) internal virtual {
        uint256 signersLen = signers.length;
        if (signersLen == 0) {
            revert SignersSetIsEmpty();
        }
        _validateThreshold(threshold, signersLen);

        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();

        // Clear current signers first
        uint256 currentLen = $.currentSigners.length;
        for (uint256 i = 0; i < currentLen; i++) {
            $.currentIsSigner[$.currentSigners[i]] = false;
        }
        delete $.currentSigners;

        // Add new signers
        for (uint256 i = 0; i < signersLen; i++) {
            address signer = signers[i];
            if (signer == address(0)) {
                revert KMSSignerNull();
            }
            if ($.currentIsSigner[signer]) {
                revert KMSAlreadySigner();
            }
            $.currentIsSigner[signer] = true;
            $.currentSigners.push(signer);
        }
        $.currentThreshold = threshold;
    }

    /**
     * @notice          Moves current epoch to previous epoch storage.
     */
    function _moveToPreviousEpoch() internal virtual {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();

        // Copy current to previous
        uint256 currentLen = $.currentSigners.length;
        for (uint256 i = 0; i < currentLen; i++) {
            address signer = $.currentSigners[i];
            $.previousIsSigner[signer] = true;
            $.previousSigners.push(signer);
        }
        $.previousThreshold = $.currentThreshold;
    }

    /**
     * @notice          Clears the previous epoch storage.
     */
    function _clearPreviousEpoch() internal virtual {
        KMSVerifierV2Storage storage $ = _getKMSVerifierV2Storage();

        uint256 prevLen = $.previousSigners.length;
        for (uint256 i = 0; i < prevLen; i++) {
            $.previousIsSigner[$.previousSigners[i]] = false;
        }
        delete $.previousSigners;
        $.previousThreshold = 0;
    }

    /**
     * @notice          Validates that threshold is valid for the given number of signers.
     * @param threshold Threshold to validate.
     * @param numSigners Number of signers.
     */
    function _validateThreshold(uint256 threshold, uint256 numSigners) internal pure virtual {
        if (threshold == 0) {
            revert ThresholdIsNull();
        }
        if (threshold > numSigners) {
            revert ThresholdIsAboveNumberOfSigners();
        }
    }

    /**
     * @notice              Verifies multiple signatures for a given digest at the effective threshold.
     * @param digest        The hash of the message that was signed by all signers.
     * @param signatures    An array of signatures to verify.
     * @return isVerified   true if enough provided signatures are valid, false otherwise.
     */
    function _verifySignaturesDigest(bytes32 digest, bytes[] memory signatures) internal virtual returns (bool) {
        uint256 numSignatures = signatures.length;

        if (numSignatures == 0) {
            revert KMSZeroSignature();
        }

        uint256 threshold = getEffectiveThreshold();

        if (numSignatures < threshold) {
            revert KMSSignatureThresholdNotReached(numSignatures);
        }

        address[] memory recoveredSigners = new address[](numSignatures);
        uint256 uniqueValidCount;
        for (uint256 i = 0; i < numSignatures; i++) {
            address signerRecovered = _recoverSigner(digest, signatures[i]);
            // Use isValidSigner which checks both epochs during grace period
            if (!isValidSigner(signerRecovered)) {
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
     * @notice          Cleans a hashmap in transient storage.
     * @param keys      An array of keys to cleanup from transient storage.
     * @param maxIndex  The biggest index to take into account from the array.
     */
    function _cleanTransientHashMap(address[] memory keys, uint256 maxIndex) internal virtual {
        for (uint256 j = 0; j < maxIndex; j++) {
            _tstore(keys[j], 0);
        }
    }

    /**
     * @notice          Writes to transient storage.
     * @param location  The address used as key.
     * @param value     Value to store.
     */
    function _tstore(address location, uint256 value) internal virtual {
        assembly {
            tstore(location, value)
        }
    }

    /**
     * @notice           Reads transient storage.
     * @param location   The address used as key.
     * @return value     true if value is non-null, false otherwise.
     */
    function _tload(address location) internal view virtual returns (bool value) {
        assembly {
            value := tload(location)
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
     * @notice          Recovers the signer's address from a signature and a message digest.
     * @param message   The hash of the message that was signed.
     * @param signature The signature to verify.
     * @return signer   The address that signed the message.
     */
    function _recoverSigner(bytes32 message, bytes memory signature) internal pure virtual returns (address) {
        address signerRecovered = ECDSA.recover(message, signature);
        return signerRecovered;
    }

    /**
     * @dev Returns the KMSVerifierV2 storage location.
     */
    function _getKMSVerifierV2Storage() internal pure returns (KMSVerifierV2Storage storage $) {
        assembly {
            $.slot := KMSVerifierV2StorageLocation
        }
    }

    /**
     * @notice          Legacy function for backward compatibility - checks if address is a signer.
     * @param account   Account address.
     * @return          Whether the account is a valid signer.
     */
    function isSigner(address account) public view virtual returns (bool) {
        return isValidSigner(account);
    }
}
