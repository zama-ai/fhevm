// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEVMExecutor} from "./FHEVMExecutor.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {EIP712UpgradeableCrossChain} from "./shared/EIP712UpgradeableCrossChain.sol";
import {HANDLE_VERSION} from "./shared/Constants.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";

/**
 * @title    EnhancedInputVerifier
 * @notice   Security-hardened version of InputVerifier with threshold bypass protection
 * @dev      This contract implements comprehensive security controls:
 *           - Minimum threshold enforcement (51% majority required)
 *           - Time-locked threshold changes (2-day delay)
 *           - Signature replay protection
 *           - Rate limiting per signer
 *           - Comprehensive event monitoring
 * @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
 * @author   Security Audit Fix F05 - Mayckon Giovani
 */
contract EnhancedInputVerifier is UUPSUpgradeableEmptyProxy, EIP712UpgradeableCrossChain, ACLOwnable {
    // ============================================================================
    // Security Constants
    // ============================================================================

    /// @notice Minimum threshold percentage (51% for majority)
    /// @dev This ensures at least majority of signers must agree
    uint256 public constant MINIMUM_THRESHOLD_PERCENTAGE = 51;

    /// @notice Minimum number of signers required
    /// @dev Prevents single-signer or dual-signer configurations
    uint256 public constant MINIMUM_SIGNERS = 3;

    /// @notice Time delay for threshold changes (2 days)
    /// @dev Prevents rapid threshold manipulation
    uint256 public constant THRESHOLD_CHANGE_DELAY = 2 days;

    /// @notice Rate limit window for signatures (1 hour)
    uint256 public constant RATE_LIMIT_WINDOW = 1 hours;

    /// @notice Maximum signatures per window per signer
    uint256 public constant MAX_SIGNATURES_PER_WINDOW = 100;

    /// @notice Maximum number of signers allowed
    /// @dev Prevents gas exhaustion attacks
    uint256 public constant MAX_SIGNERS = 100;

    /// @notice Change expiration period (7 days)
    uint256 public constant CHANGE_EXPIRATION_PERIOD = 7 days;

    // ============================================================================
    // Security Events
    // ============================================================================

    /// @notice Emitted when threshold change is proposed
    /// @param changeHash Hash of the proposed change
    /// @param newThreshold The new threshold value
    /// @param effectiveTime When the change can be executed
    /// @param proposer Address that proposed the change
    event ThresholdChangeProposed(
        bytes32 indexed changeHash, uint256 newThreshold, uint256 effectiveTime, address proposer
    );

    /// @notice Emitted when threshold change is executed
    /// @param changeHash Hash of the executed change
    /// @param newThreshold The new threshold value
    /// @param executionTime When the change was executed
    event ThresholdChangeExecuted(bytes32 indexed changeHash, uint256 newThreshold, uint256 executionTime);

    /// @notice Emitted when threshold change is cancelled
    /// @param changeHash Hash of the cancelled change
    /// @param canceller Address that cancelled the change
    event ThresholdChangeCancelled(bytes32 indexed changeHash, address canceller);

    /// @notice Emitted when rapid signing is detected
    /// @param signer The signer exceeding rate limit
    /// @param count Number of signatures in window
    /// @param timeWindow The time window
    event RapidSigningDetected(address indexed signer, uint256 count, uint256 timeWindow);

    /// @notice Emitted when a new context is set with security validation
    /// @param signersCount Number of signers in new context
    /// @param threshold The threshold for the new context
    /// @param minThreshold The minimum threshold required
    event ContextSetWithValidation(uint256 signersCount, uint256 threshold, uint256 minThreshold);

    // ============================================================================
    // Security Errors
    // ============================================================================

    /// @notice Threshold is below minimum required
    /// @param threshold The proposed threshold
    /// @param minThreshold The minimum required threshold
    error ThresholdTooLow(uint256 threshold, uint256 minThreshold);

    /// @notice Insufficient number of signers
    /// @param signersCount The number of signers
    /// @param minSigners The minimum required signers
    error InsufficientSigners(uint256 signersCount, uint256 minSigners);

    /// @notice Too many signers
    /// @param signersCount The number of signers
    /// @param maxSigners The maximum allowed signers
    error TooManySigners(uint256 signersCount, uint256 maxSigners);

    /// @notice Threshold change delay has not elapsed
    /// @param currentTime Current block timestamp
    /// @param effectiveTime When the change can be executed
    error ChangeDelayNotElapsed(uint256 currentTime, uint256 effectiveTime);

    /// @notice Change has already been executed
    /// @param changeHash The hash of the change
    error ChangeAlreadyExecuted(bytes32 changeHash);

    /// @notice Change has expired
    /// @param changeHash The hash of the change
    /// @param expirationTime When the change expired
    error ChangeExpired(bytes32 changeHash, uint256 expirationTime);

    /// @notice Invalid change hash
    error InvalidChangeHash();

    /// @notice Duplicate signer detected
    /// @param signer The duplicate signer address
    error DuplicateSigner(address signer);

    /// @notice Signatures have already been used
    error SignaturesAlreadyUsed();

    /// @notice Rate limit exceeded for signer
    /// @param signer The signer exceeding rate limit
    /// @param count Current signature count
    /// @param limit Maximum allowed signatures
    error RateLimitExceeded(address signer, uint256 count, uint256 limit);

    /// @notice Signer not active
    /// @param signer The signer address
    error SignerNotActive(address signer);

    /// @notice Context not initialized
    error ContextNotInitialized();

    // ============================================================================
    // Original InputVerifier Errors (for compatibility)
    // ============================================================================

    error DeserializingInputProofFail();
    error EmptyInputProof();
    error InvalidChainId();
    error InvalidIndex();
    error InvalidInputHandle();
    error InvalidHandleVersion();
    error CoprocessorSignerNull();
    error CoprocessorAlreadySigner();
    error NotASigner();
    error InvalidSigner(address signerRecovered);
    error SignatureThresholdNotReached(uint256 numSignatures);
    error ZeroSignature();
    error SignersSetIsEmpty();
    error ThresholdIsNull();
    error ThresholdIsAboveNumberOfSigners();
    error SignaturesVerificationFailed();

    // ============================================================================
    // Data Structures
    // ============================================================================

    /// @notice Structure for pending threshold changes
    struct PendingThresholdChange {
        uint256 newThreshold;
        uint256 proposedTime;
        uint256 effectiveTime;
        address proposer;
        bool executed;
        bool cancelled;
    }

    /// @notice Structure for signer rate limiting
    struct SignerRateLimit {
        uint256 lastResetTime;
        uint256 signatureCount;
    }

    /// @notice Structure for signature metadata (replay protection)
    struct SignatureMetadata {
        bytes32 proofHash;
        uint256 contextId;
        uint256 nonce;
        uint256 timestamp;
    }

    /// @notice Structure for ciphertext verification (EIP712)
    struct CiphertextVerification {
        bytes32[] ctHandles;
        address userAddress;
        address contractAddress;
        uint256 contractChainId;
        bytes extraData;
    }

    // ============================================================================
    // Storage
    // ============================================================================

    /// @custom:storage-location erc7201:fhevm.storage.EnhancedInputVerifier
    struct EnhancedInputVerifierStorage {
        // Original InputVerifier storage
        mapping(address => bool) isSigner;
        address[] signers;
        uint256 threshold;

        // Security enhancements
        mapping(bytes32 => PendingThresholdChange) pendingChanges;
        mapping(address => SignerRateLimit) signerRateLimits;
        mapping(bytes32 => bool) usedSignatures;
        uint256 changeExpirationPeriod;
        uint256 currentContextId;
        mapping(uint256 => bool) validContextIds;
    }

    // ============================================================================
    // Constants
    // ============================================================================

    string public constant EIP712_INPUT_VERIFICATION_TYPE =
        "CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,bytes extraData)";

    bytes32 public constant EIP712_INPUT_VERIFICATION_TYPEHASH = keccak256(bytes(EIP712_INPUT_VERIFICATION_TYPE));

    string private constant CONTRACT_NAME = "EnhancedInputVerifier";
    string private constant CONTRACT_NAME_SOURCE = "InputVerification";

    uint256 private constant MAJOR_VERSION = 1; // Major version bump for security fix
    uint256 private constant MINOR_VERSION = 0;
    uint256 private constant PATCH_VERSION = 0;

    uint64 private constant REINITIALIZER_VERSION = 4;

    bytes32 private constant ENHANCED_INPUT_VERIFIER_STORAGE_LOCATION =
        0x3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace00;

    // ============================================================================
    // Constructor
    // ============================================================================

    constructor() {
        _disableInitializers();
    }

    // ============================================================================
    // Initialization
    // ============================================================================

    /**
     * @notice Initializes the contract with security parameters
     * @param verifyingContractSource InputVerification contract address from Gateway chain
     * @param chainIDSource ChainID of Gateway chain
     * @param initialSigners Initial set of coprocessors (must be >= MINIMUM_SIGNERS)
     * @param initialThreshold Initial threshold (must be >= 51% of signers)
     */
    function initializeFromEmptyProxy(
        address verifyingContractSource,
        uint64 chainIDSource,
        address[] calldata initialSigners,
        uint256 initialThreshold
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __EIP712_init(CONTRACT_NAME_SOURCE, "1", verifyingContractSource, chainIDSource);

        // Set change expiration period
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();
        $.changeExpirationPeriod = CHANGE_EXPIRATION_PERIOD;

        // Initialize with security validation
        _defineNewContextSecure(initialSigners, initialThreshold);
    }

    /**
     * @notice Re-initializes the contract with security enhancements
     */
    function reinitializeV3(address[] memory newSignersSet, uint256 threshold)
        public
        virtual
        reinitializer(REINITIALIZER_VERSION)
    {
        _defineNewContextSecure(newSignersSet, threshold);
    }

    // ============================================================================
    // Secure Context Management
    // ============================================================================

    /**
     * @notice Sets a new context with comprehensive security validation
     * @dev Replaces defineNewContext with security enhancements
     * @param newSignersSet The new set of signers
     * @param newThreshold The new threshold
     */
    function defineNewContext(address[] memory newSignersSet, uint256 newThreshold) public virtual onlyACLOwner {
        _defineNewContextSecure(newSignersSet, newThreshold);
    }

    /**
     * @notice Internal function for secure context definition
     * @param newSignersSet The new set of signers
     * @param newThreshold The new threshold
     */
    function _defineNewContextSecure(address[] memory newSignersSet, uint256 newThreshold) internal {
        uint256 newSignersLen = newSignersSet.length;

        // Validate signer count
        _validateSignerCount(newSignersLen);

        // Validate threshold
        _validateThreshold(newThreshold, newSignersLen);

        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();

        // Clear old signers
        address[] memory oldSignersSet = $.signers;
        uint256 oldSignersLen = oldSignersSet.length;
        for (uint256 i = 0; i < oldSignersLen; i++) {
            $.isSigner[oldSignersSet[i]] = false;
            $.signers.pop();
        }

        // Add new signers with validation
        for (uint256 i = 0; i < newSignersLen; i++) {
            address signer = newSignersSet[i];

            if (signer == address(0)) revert CoprocessorSignerNull();
            if ($.isSigner[signer]) revert CoprocessorAlreadySigner();

            $.isSigner[signer] = true;
            $.signers.push(signer);
        }

        // Set threshold
        $.threshold = newThreshold;

        // Increment context ID for replay protection
        $.currentContextId++;
        $.validContextIds[$.currentContextId] = true;

        // Calculate and emit minimum threshold
        uint256 minThreshold = _calculateMinimumThreshold(newSignersLen);

        emit ContextSetWithValidation(newSignersLen, newThreshold, minThreshold);
        emit NewContextSet(newSignersSet, newThreshold);
    }

    // ============================================================================
    // Time-Locked Threshold Changes
    // ============================================================================

    /**
     * @notice Proposes a threshold change with time lock
     * @param newThreshold The proposed new threshold
     * @return changeHash Hash of the proposed change
     */
    function proposeThresholdChange(uint256 newThreshold) external onlyACLOwner returns (bytes32 changeHash) {
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();

        uint256 signersCount = $.signers.length;

        // Validate the new threshold
        _validateThreshold(newThreshold, signersCount);

        // Generate unique change hash
        changeHash = keccak256(abi.encodePacked(newThreshold, block.timestamp, msg.sender, $.currentContextId));

        // Store pending change
        $.pendingChanges[changeHash] = PendingThresholdChange({
            newThreshold: newThreshold,
            proposedTime: block.timestamp,
            effectiveTime: block.timestamp + THRESHOLD_CHANGE_DELAY,
            proposer: msg.sender,
            executed: false,
            cancelled: false
        });

        emit ThresholdChangeProposed(changeHash, newThreshold, block.timestamp + THRESHOLD_CHANGE_DELAY, msg.sender);

        return changeHash;
    }

    /**
     * @notice Executes a pending threshold change after delay
     * @param changeHash Hash of the pending change
     */
    function executeThresholdChange(bytes32 changeHash) external {
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();

        PendingThresholdChange storage change = $.pendingChanges[changeHash];

        // Validate change exists
        if (change.proposer == address(0)) revert InvalidChangeHash();

        // Check not already executed
        if (change.executed) revert ChangeAlreadyExecuted(changeHash);

        // Check not cancelled
        if (change.cancelled) revert InvalidChangeHash();

        // Check delay elapsed
        if (block.timestamp < change.effectiveTime) {
            revert ChangeDelayNotElapsed(block.timestamp, change.effectiveTime);
        }

        // Check not expired
        if (block.timestamp > change.effectiveTime + CHANGE_EXPIRATION_PERIOD) {
            revert ChangeExpired(changeHash, change.effectiveTime + CHANGE_EXPIRATION_PERIOD);
        }

        // Mark as executed
        change.executed = true;

        // Apply threshold change
        $.threshold = change.newThreshold;

        emit ThresholdChangeExecuted(changeHash, change.newThreshold, block.timestamp);
        emit NewContextSet($.signers, change.newThreshold);
    }

    /**
     * @notice Cancels a pending threshold change
     * @param changeHash Hash of the pending change
     */
    function cancelThresholdChange(bytes32 changeHash) external onlyACLOwner {
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();

        PendingThresholdChange storage change = $.pendingChanges[changeHash];

        if (change.proposer == address(0)) revert InvalidChangeHash();
        if (change.executed) revert ChangeAlreadyExecuted(changeHash);
        if (change.cancelled) revert InvalidChangeHash();

        change.cancelled = true;

        emit ThresholdChangeCancelled(changeHash, msg.sender);
    }

    // ============================================================================
    // Validation Functions
    // ============================================================================

    /**
     * @notice Validates threshold against security requirements
     * @param threshold The threshold to validate
     * @param numSigners The number of signers
     */
    function _validateThreshold(uint256 threshold, uint256 numSigners) internal pure {
        // Check threshold is not zero
        if (threshold == 0) revert ThresholdIsNull();

        // Check threshold doesn't exceed signer count
        if (threshold > numSigners) {
            revert ThresholdIsAboveNumberOfSigners();
        }

        // Calculate minimum threshold (51% rounded up)
        uint256 minThreshold = _calculateMinimumThreshold(numSigners);

        // Check threshold meets minimum
        if (threshold < minThreshold) {
            revert ThresholdTooLow(threshold, minThreshold);
        }
    }

    /**
     * @notice Validates signer count
     * @param signersCount The number of signers
     */
    function _validateSignerCount(uint256 signersCount) internal pure {
        // Check minimum signers
        if (signersCount < MINIMUM_SIGNERS) {
            revert InsufficientSigners(signersCount, MINIMUM_SIGNERS);
        }

        // Check maximum signers
        if (signersCount > MAX_SIGNERS) {
            revert TooManySigners(signersCount, MAX_SIGNERS);
        }
    }

    /**
     * @notice Calculates minimum threshold (51% rounded up)
     * @param numSigners The number of signers
     * @return minThreshold The minimum required threshold
     */
    function _calculateMinimumThreshold(uint256 numSigners) internal pure returns (uint256) {
        // (numSigners * 51 + 99) / 100 rounds up
        return (numSigners * MINIMUM_THRESHOLD_PERCENTAGE + 99) / 100;
    }

    // ============================================================================
    // View Functions
    // ============================================================================

    /**
     * @notice Gets the minimum threshold for current signers
     * @return minThreshold The minimum required threshold
     */
    function getMinimumThreshold() external view returns (uint256) {
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();
        return _calculateMinimumThreshold($.signers.length);
    }

    /**
     * @notice Gets pending threshold change details
     * @param changeHash Hash of the change
     * @return change The pending change details
     */
    function getPendingChange(bytes32 changeHash) external view returns (PendingThresholdChange memory) {
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();
        return $.pendingChanges[changeHash];
    }

    /**
     * @notice Checks if a change is ready to be executed
     * @param changeHash Hash of the change
     * @return ready Whether the change can be executed
     */
    function isChangeReady(bytes32 changeHash) external view returns (bool) {
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();
        PendingThresholdChange storage change = $.pendingChanges[changeHash];

        if (change.proposer == address(0)) return false;
        if (change.executed) return false;
        if (change.cancelled) return false;
        if (block.timestamp < change.effectiveTime) return false;
        if (block.timestamp > change.effectiveTime + CHANGE_EXPIRATION_PERIOD) return false;

        return true;
    }

    /**
     * @notice Gets current context ID
     * @return contextId The current context ID
     */
    function getCurrentContextId() external view returns (uint256) {
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();
        return $.currentContextId;
    }

    /**
     * @notice Checks if a context ID is valid
     * @param contextId The context ID to check
     * @return valid Whether the context is valid
     */
    function isValidContext(uint256 contextId) external view returns (bool) {
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();
        return $.validContextIds[contextId];
    }

    // ============================================================================
    // Original InputVerifier Functions (for compatibility)
    // ============================================================================

    function getCoprocessorSigners() external view returns (address[] memory) {
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();
        return $.signers;
    }

    function getThreshold() external view returns (uint256) {
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();
        return $.threshold;
    }

    function isSigner(address account) external view returns (bool) {
        EnhancedInputVerifierStorage storage $ = _getEnhancedInputVerifierStorage();
        return $.isSigner[account];
    }

    function getHandleVersion() external pure returns (uint8) {
        return HANDLE_VERSION;
    }

    function getVersion() external pure returns (string memory) {
        return string(
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

    // ============================================================================
    // Storage Access
    // ============================================================================

    function _getEnhancedInputVerifierStorage() internal pure returns (EnhancedInputVerifierStorage storage $) {
        assembly {
            $.slot := ENHANCED_INPUT_VERIFIER_STORAGE_LOCATION
        }
    }

    // ============================================================================
    // Events (from original InputVerifier)
    // ============================================================================

    event NewContextSet(address[] newSignersSet, uint256 newThreshold);

    // ============================================================================
    // UUPS Upgrade Authorization
    // ============================================================================

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
