// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ICiphertextRoots} from "./interfaces/ICiphertextRoots.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {protocolConfigAdd} from "../addresses/FHEVMHostAddresses.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @dev ProtocolConfig's coprocessor context reads (RFC 039), declared here until
 *      `IProtocolConfigBase` carries them; then delete this and import it.
 */
interface ICoprocessorContexts {
    function getCurrentCoprocessorContextId() external view returns (uint256);
    function isValidCoprocessorContext(uint256 coprocessorContextId) external view returns (bool);
    function getCoprocessorMajorityThresholdForContext(uint256 coprocessorContextId) external view returns (uint256);
    function isCoprocessorTxSenderForContext(
        uint256 coprocessorContextId,
        address txSender
    ) external view returns (bool);
}

/**
 * @title CiphertextRoots contract
 * @notice See {ICiphertextRoots}.
 * @dev Append-only: no function, the DAO's included, modifies or deletes a posted root, a recorded
 *      epoch root or an epoch length that a recorded root was built under. An upgrade must preserve them.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract CiphertextRoots is ICiphertextRoots, UUPSUpgradeableEmptyProxy, ACLOwnable {
    // -----------------------------------------------------------------------------------------
    // Contract information
    // -----------------------------------------------------------------------------------------

    string private constant CONTRACT_NAME = "CiphertextRoots";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Version used by `initializeFromEmptyProxy`'s `reinitializer` modifier.
    uint64 private constant REINITIALIZER_VERSION = 2;

    // -----------------------------------------------------------------------------------------
    // Other contract references
    // -----------------------------------------------------------------------------------------

    /// @notice The ProtocolConfig contract, source of the coprocessor contexts.
    ICoprocessorContexts private constant PROTOCOL_CONFIG = ICoprocessorContexts(protocolConfigAdd);

    // -----------------------------------------------------------------------------------------
    // ERC-7201 namespaced storage
    // -----------------------------------------------------------------------------------------

    /// @custom:storage-location erc7201:fhevm.storage.CiphertextRoots
    struct CiphertextRootsStorage {
        /// @notice Epoch schedule per host chain. No epoch length means no schedule.
        mapping(uint64 chainId => ICiphertextRoots.EpochSchedule schedule) epochSchedules;
        /// @notice Distinct roots posted per (host chain, epoch), in first-post order.
        mapping(uint64 chainId => mapping(uint256 epoch => bytes32[] roots)) postedRoots;
        /// @notice Senders of each posted root per (host chain, epoch), in post order.
        mapping(uint64 chainId => mapping(uint256 epoch => mapping(bytes32 root => address[] senders))) rootSenders;
        /// @notice Whether a sender already posted for a (host chain, epoch).
        mapping(uint64 chainId => mapping(uint256 epoch => mapping(address sender => bool posted))) hasPosted;
        /// @notice Recorded root per (host chain, epoch).
        mapping(uint64 chainId => mapping(uint256 epoch => bytes32 root)) epochRoots;
        /// @notice Whether a (host chain, epoch) has a recorded root.
        mapping(uint64 chainId => mapping(uint256 epoch => bool recorded)) epochRootRecorded;
        /// @notice The highest epoch with a recorded root per host chain.
        mapping(uint64 chainId => uint256 epoch) highestRecordedEpoch;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.CiphertextRoots")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant CIPHERTEXT_ROOTS_STORAGE_LOCATION =
        0x3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f12000;

    /// @dev The coprocessor contexts whose tx senders may post, resolved from ProtocolConfig once per call.
    struct Contexts {
        /// @notice The current coprocessor context.
        uint256 currentId;
        /// @notice The coprocessor context the current one replaced: the current ID minus one.
        uint256 previousId;
        /// @notice Whether ProtocolConfig still reports the previous context valid.
        bool previousValid;
        /// @notice The majority threshold of the current context.
        uint256 currentThreshold;
        /// @notice The majority threshold of the previous context, zero if it is not valid.
        uint256 previousThreshold;
    }

    // -----------------------------------------------------------------------------------------
    // Constructor and initialization
    // -----------------------------------------------------------------------------------------

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract from the empty proxy.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    // solhint-disable-next-line no-empty-blocks
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {}

    // -----------------------------------------------------------------------------------------
    // State-changing functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc ICiphertextRoots
    function setEpochSchedule(uint64 chainId, uint256 startBlock, uint256 epochLength) external virtual onlyACLOwner {
        if (chainId == 0) {
            revert InvalidChainId();
        }
        if (epochLength == 0) {
            revert InvalidEpochLength();
        }
        CiphertextRootsStorage storage $ = _getCiphertextRootsStorage();
        EpochSchedule storage schedule = $.epochSchedules[chainId];
        if (schedule.epochLengths.length != 0) {
            revert EpochScheduleAlreadySet(chainId);
        }
        schedule.startBlock = startBlock;
        schedule.epochLengths.push(EpochLength({fromEpoch: 0, epochLength: epochLength}));
        emit EpochScheduleSet(chainId, startBlock, epochLength);
    }

    /// @inheritdoc ICiphertextRoots
    function setEpochLength(uint64 chainId, uint256 fromEpoch, uint256 epochLength) external virtual onlyACLOwner {
        if (epochLength == 0) {
            revert InvalidEpochLength();
        }
        CiphertextRootsStorage storage $ = _getCiphertextRootsStorage();
        _checkEpochSchedule($, chainId);
        EpochLength[] storage epochLengths = $.epochSchedules[chainId].epochLengths;
        uint256 minFromEpoch = epochLengths[epochLengths.length - 1].fromEpoch;
        uint256 highestRecordedEpoch = $.highestRecordedEpoch[chainId];
        if (highestRecordedEpoch > minFromEpoch) {
            minFromEpoch = highestRecordedEpoch;
        }
        minFromEpoch++;
        if (fromEpoch < minFromEpoch) {
            revert FromEpochTooLow(chainId, fromEpoch, minFromEpoch);
        }
        epochLengths.push(EpochLength({fromEpoch: fromEpoch, epochLength: epochLength}));
        emit EpochLengthSet(chainId, fromEpoch, epochLength);
    }

    /// @inheritdoc ICiphertextRoots
    function removeEpochLength(uint64 chainId, uint256 fromEpoch) external virtual onlyACLOwner {
        CiphertextRootsStorage storage $ = _getCiphertextRootsStorage();
        _checkEpochSchedule($, chainId);
        EpochLength[] storage epochLengths = $.epochSchedules[chainId].epochLengths;
        uint256 count = epochLengths.length;
        // The first epoch length, from epoch 0, is never removed: the search starts after it.
        uint256 i = 1;
        while (i < count && epochLengths[i].fromEpoch != fromEpoch) {
            i++;
        }
        if (i == count) {
            revert EpochLengthNotFound(chainId, fromEpoch);
        }
        if (fromEpoch <= $.highestRecordedEpoch[chainId]) {
            revert EpochLengthInUse(chainId, fromEpoch);
        }
        for (; i + 1 < count; i++) {
            epochLengths[i] = epochLengths[i + 1];
        }
        epochLengths.pop();
        emit EpochLengthRemoved(chainId, fromEpoch);
    }

    /// @inheritdoc ICiphertextRoots
    function postRoot(uint64 chainId, uint256 epoch, bytes32 root) external virtual {
        CiphertextRootsStorage storage $ = _getCiphertextRootsStorage();
        _checkEpochSchedule($, chainId);
        if ($.hasPosted[chainId][epoch][msg.sender]) {
            revert RootAlreadyPosted(chainId, epoch, msg.sender);
        }
        Contexts memory contexts = _loadContexts();
        if (!_isCoprocessorTxSender(contexts, msg.sender)) {
            revert NotCoprocessorTxSender(msg.sender);
        }

        // Record a root that a context switch already brought to a threshold before adding an entry
        // for another root, so the first root to reach a threshold wins whatever triggers the count.
        // With a single root, the count after the append finds it.
        bytes32[] storage roots = $.postedRoots[chainId][epoch];
        address[] storage senders = $.rootSenders[chainId][epoch][root];
        bool recorded = $.epochRootRecorded[chainId][epoch];
        bool otherRootPosted = senders.length == 0 ? roots.length > 0 : roots.length > 1;
        if (!recorded && otherRootPosted) {
            recorded = _recordFirstQualifyingRoot($, chainId, epoch, contexts);
        }

        $.hasPosted[chainId][epoch][msg.sender] = true;
        if (senders.length == 0) {
            roots.push(root);
        }
        senders.push(msg.sender);
        emit RootPosted(chainId, epoch, msg.sender, root);

        // Only this root's count changed, so only it can have reached a threshold.
        if (!recorded) {
            (bool reached, uint256 contextId) = _reachesMajorityThreshold(contexts, senders);
            if (reached) {
                _recordEpochRoot($, chainId, epoch, root, contextId);
            }
        }
    }

    /// @inheritdoc ICiphertextRoots
    function recordEpochRoot(uint64 chainId, uint256 epoch) external virtual {
        CiphertextRootsStorage storage $ = _getCiphertextRootsStorage();
        _checkEpochSchedule($, chainId);
        if ($.epochRootRecorded[chainId][epoch]) {
            revert EpochRootAlreadyRecorded(chainId, epoch);
        }
        if (!_recordFirstQualifyingRoot($, chainId, epoch, _loadContexts())) {
            revert NoRootReachesThreshold(chainId, epoch);
        }
    }

    // -----------------------------------------------------------------------------------------
    // View functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc ICiphertextRoots
    function getEpochSchedule(uint64 chainId) external view virtual returns (EpochSchedule memory) {
        CiphertextRootsStorage storage $ = _getCiphertextRootsStorage();
        _checkEpochSchedule($, chainId);
        return $.epochSchedules[chainId];
    }

    /// @inheritdoc ICiphertextRoots
    function getEpochRoot(uint64 chainId, uint256 epoch) external view virtual returns (bytes32 root, bool recorded) {
        CiphertextRootsStorage storage $ = _getCiphertextRootsStorage();
        return ($.epochRoots[chainId][epoch], $.epochRootRecorded[chainId][epoch]);
    }

    /// @inheritdoc ICiphertextRoots
    function getPostedRoots(uint64 chainId, uint256 epoch) external view virtual returns (bytes32[] memory) {
        return _getCiphertextRootsStorage().postedRoots[chainId][epoch];
    }

    /// @inheritdoc ICiphertextRoots
    function getRootSenders(
        uint64 chainId,
        uint256 epoch,
        bytes32 root
    ) external view virtual returns (address[] memory) {
        return _getCiphertextRootsStorage().rootSenders[chainId][epoch][root];
    }

    /// @inheritdoc ICiphertextRoots
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

    // -----------------------------------------------------------------------------------------
    // Internal
    // -----------------------------------------------------------------------------------------

    function _checkEpochSchedule(CiphertextRootsStorage storage $, uint64 chainId) internal view virtual {
        if ($.epochSchedules[chainId].epochLengths.length == 0) {
            revert NoEpochSchedule(chainId);
        }
    }

    /**
     * @dev The previous context is the current ID minus one (RFC 039). It counts only while
     *      ProtocolConfig still reports it valid, i.e. until governance destroys it, and every read
     *      of it is gated on that, as a destroyed context's set and threshold stay readable.
     */
    function _loadContexts() internal view virtual returns (Contexts memory contexts) {
        contexts.currentId = PROTOCOL_CONFIG.getCurrentCoprocessorContextId();
        if (contexts.currentId == 0) {
            revert NoCoprocessorContext();
        }
        contexts.currentThreshold = PROTOCOL_CONFIG.getCoprocessorMajorityThresholdForContext(contexts.currentId);
        contexts.previousId = contexts.currentId - 1;
        contexts.previousValid = PROTOCOL_CONFIG.isValidCoprocessorContext(contexts.previousId);
        if (contexts.previousValid) {
            contexts.previousThreshold = PROTOCOL_CONFIG.getCoprocessorMajorityThresholdForContext(contexts.previousId);
        }
    }

    function _isCoprocessorTxSender(Contexts memory contexts, address sender) internal view virtual returns (bool) {
        if (PROTOCOL_CONFIG.isCoprocessorTxSenderForContext(contexts.currentId, sender)) {
            return true;
        }
        return contexts.previousValid && PROTOCOL_CONFIG.isCoprocessorTxSenderForContext(contexts.previousId, sender);
    }

    /**
     * @dev Counts a root's senders by membership read now, not at post time, so entries posted
     *      before a context switch count under the new context too; a sender in both contexts counts
     *      for both. Returns the context whose threshold is reached, the current one if both are.
     *      A context whose threshold exceeds the sender count is skipped.
     */
    function _reachesMajorityThreshold(
        Contexts memory contexts,
        address[] storage senders
    ) internal view virtual returns (bool reached, uint256 coprocessorContextId) {
        uint256 senderCount = senders.length;
        bool currentReachable = senderCount >= contexts.currentThreshold;
        bool previousReachable = contexts.previousValid && senderCount >= contexts.previousThreshold;
        if (!currentReachable && !previousReachable) {
            return (false, 0);
        }
        uint256 currentCount;
        uint256 previousCount;
        for (uint256 i = 0; i < senderCount; i++) {
            address sender = senders[i];
            if (currentReachable && PROTOCOL_CONFIG.isCoprocessorTxSenderForContext(contexts.currentId, sender)) {
                currentCount++;
            }
            if (previousReachable && PROTOCOL_CONFIG.isCoprocessorTxSenderForContext(contexts.previousId, sender)) {
                previousCount++;
            }
        }
        if (currentReachable && currentCount >= contexts.currentThreshold) {
            return (true, contexts.currentId);
        }
        if (previousReachable && previousCount >= contexts.previousThreshold) {
            return (true, contexts.previousId);
        }
        return (false, 0);
    }

    /**
     * @dev Records the first posted root, in first-post order, that reaches a majority threshold
     *      under the given contexts. Returns whether one was recorded.
     */
    function _recordFirstQualifyingRoot(
        CiphertextRootsStorage storage $,
        uint64 chainId,
        uint256 epoch,
        Contexts memory contexts
    ) internal virtual returns (bool) {
        bytes32[] storage roots = $.postedRoots[chainId][epoch];
        for (uint256 i = 0; i < roots.length; i++) {
            bytes32 root = roots[i];
            (bool reached, uint256 contextId) = _reachesMajorityThreshold(
                contexts,
                $.rootSenders[chainId][epoch][root]
            );
            if (reached) {
                _recordEpochRoot($, chainId, epoch, root, contextId);
                return true;
            }
        }
        return false;
    }

    function _recordEpochRoot(
        CiphertextRootsStorage storage $,
        uint64 chainId,
        uint256 epoch,
        bytes32 root,
        uint256 coprocessorContextId
    ) internal virtual {
        $.epochRoots[chainId][epoch] = root;
        $.epochRootRecorded[chainId][epoch] = true;
        if (epoch > $.highestRecordedEpoch[chainId]) {
            $.highestRecordedEpoch[chainId] = epoch;
        }
        emit EpochRootRecorded(chainId, epoch, root, coprocessorContextId);
    }

    /**
     * @notice Returns the CiphertextRoots storage location.
     * @dev Internal but not virtual: a derived contract that changes the storage struct must define
     *      its own getter rather than override the storage location.
     */
    function _getCiphertextRootsStorage() internal pure returns (CiphertextRootsStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := CIPHERTEXT_ROOTS_STORAGE_LOCATION
        }
    }

    /**
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
