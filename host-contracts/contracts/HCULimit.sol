// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {aclAdd} from "../addresses/FHEVMHostAddresses.sol";
import {ACL} from "./ACL.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {HCULimitOperators} from "./HCULimitOperators.sol";

/**
 * @title HCULimit
 * @notice Controls admission to HCU-charged FHE computation and enforces block and transaction HCU limits.
 * @dev The contract is designed to be used with the FHEVMExecutor contract.
 * The computation allowlist controls which dapps may compute while activated. Membership grants no
 * exemption from HCU limits. The independent block HCU whitelist exempts callers only from the global
 * block cap and meter; transaction and depth limits still apply. A caller on the block HCU whitelist
 * must also be on the computation allowlist while allowlist is activated.
 */
/// @dev This contract was migrated from Ownable2StepUpgradeable to ACLOwnable.
/// Deployed proxies retain residual `_owner` and `_pendingOwner` values in the
/// Ownable2StepUpgradeable EIP-7201 storage namespace. These slots are unused
/// by ACLOwnable and have no effect on contract behavior.
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract HCULimit is UUPSUpgradeableEmptyProxy, ACLOwnable, HCULimitOperators {
    /// @notice Returned if the block exceeds the maximum allowed homomorphic complexity units.
    error HCUBlockLimitExceeded();

    /// @notice Returned if the address is already block HCU whitelisted.
    error AlreadyBlockHCUWhitelisted(address account);

    /// @notice Returned if the address is not block HCU whitelisted.
    error NotBlockHCUWhitelisted(address account);

    /// @notice Returned if a computation caller is absent from the active allowlist.
    error CallerNotAllowlisted(address caller);

    /// @notice Returned if the sender is neither the ACL owner nor an active allowlister.
    error CallerNotACLOwnerOrAllowlister(address caller);

    /// @notice Returned if the address is not allowlisted.
    error NotAllowlisted(address account);

    /// @notice Returned if an allowlist admission or removal batch is empty.
    error EmptyBatch();

    /// @notice Returned if an allowlister deadline is not in the future.
    error InvalidAllowlisterDeadline(uint256 deadline);

    /// @notice Returned if an account without a stored allowlister grant tries to renounce.
    error NotAllowlister(address account);

    /// @notice Returned if an addition uses the zero address.
    error InvalidZeroAddress();

    /// @notice Returned if the transaction exceeds the maximum allowed homomorphic complexity units.
    error HCUTransactionLimitExceeded();

    /// @notice Returned if the transaction exceeds the maximum allowed depth of homomorphic complexity units.
    error HCUTransactionDepthLimitExceeded();

    /// @notice Returned if hcuPerBlock < maxHCUPerTx.
    error HCUPerBlockBelowMaxPerTx();

    /// @notice Returned if a handle is the zero handle.
    /// @dev Handle slot zero is reserved for the transaction HCU accumulator.
    error InvalidZeroHandle();

    /// @notice Returned when the caller is not the ACL contract.
    error CallerMustBeACLContract();

    /// @notice Returned when only one of the cross-transaction depth limit and its decay is zero.
    error InvalidCrossTxDepthLimit();

    /// @notice Returned if a nonzero cross-transaction depth limit is below maxHCUDepthPerTx.
    error CrossTxDepthBelowMaxDepthPerTx();

    /// @notice Returned when an operation's depth carried across transactions exceeds the limit.
    /// @param retryAfterBlocks Blocks of decay after which this operation fits, if nothing else extends its chain.
    ///        Later operations of the same transaction can need more, so clients simulate again after waiting.
    error HCUCrossTxDepthLimitExceeded(uint256 retryAfterBlocks);

    /// @notice Emitted when the global block HCU cap is updated.
    /// @param hcuPerBlock New global block HCU cap.
    event HCUPerBlockSet(uint48 hcuPerBlock);

    /// @notice Emitted when the per-transaction HCU depth limit is updated.
    /// @param maxHCUDepthPerTx New depth limit.
    event MaxHCUDepthPerTxSet(uint48 maxHCUDepthPerTx);

    /// @notice Emitted when the per-transaction HCU limit is updated.
    /// @param maxHCUPerTx New transaction limit.
    event MaxHCUPerTxSet(uint48 maxHCUPerTx);

    /// @notice Emitted when a caller is added to the block-cap whitelist.
    /// @param account Caller address that was whitelisted.
    event BlockHCUWhitelistAdded(address indexed account);

    /// @notice Emitted when a caller is removed from the block-cap whitelist.
    /// @param account Caller address that was removed from the whitelist.
    event BlockHCUWhitelistRemoved(address indexed account);

    /// @notice Emitted when computation allowlist activation is set.
    event AllowlistActivationChanged(bool activated);

    /// @notice Emitted for each dapp admitted by the ACL owner or an active allowlister.
    event AllowlistAdded(address indexed account, address indexed addedBy);

    /// @notice Emitted for each dapp removed from the computation allowlist.
    event AllowlistRemoved(address indexed account);

    /// @notice Emitted when an allowlister deadline is granted or replaced.
    event AllowlisterAdded(address indexed account, uint256 deadline);

    /// @notice Emitted when a nonzero stored allowlister deadline is cleared.
    event AllowlisterRemoved(address indexed account);

    /// @notice Emitted when the cross-transaction depth limit is updated.
    event CrossTxDepthLimitSet(uint48 maxDepth, uint48 decayPerBlock);

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "HCULimit";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 6;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Per-handle transient word: bit 255 marks the depths as known, bits 128-254 hold the
    ///      depth carried across transactions and bits 0-127 the depth within this transaction.
    uint256 private constant DEPTH_KNOWN = 1 << 255;

    /// @custom:storage-location erc7201:fhevm.storage.HCULimit
    /// @dev The first five uint48 fields pack into a single 256-bit slot (5 × 48 = 240 bits).
    struct HCULimitStorage {
        /// @notice Maximum homomorphic complexity units per block for callers outside the block HCU whitelist.
        uint48 globalHCUCapPerBlock;
        /// @notice Used HCU in the current block for callers outside the block HCU whitelist.
        uint48 usedBlockHCU;
        /// @notice Last seen block number for the block meter.
        uint48 lastSeenBlockNumber;
        /// @notice Maximum sequential HCU depth per transaction.
        uint48 maxHCUDepthPerTx;
        /// @notice Maximum total HCU per transaction.
        uint48 maxHCUPerTx;
        /// @notice Callers exempt from the global block HCU cap and meter.
        /// @dev Membership grants no computation admission or exemption from transaction and depth limits.
        mapping(address account => bool isWhitelisted) blockHCUWhitelist;
        /// @notice Dapps admitted to HCU-charged computation while the allowlist is active.
        /// @dev Membership grants no exemption from HCU limits.
        mapping(address account => bool allowed) isAllowlisted;
        /// @notice Delegated allowlister deadlines as Unix timestamps in seconds.
        mapping(address account => uint256 deadline) allowlisters;
        /// @notice Whether only allowlisted dapps may request HCU-charged computation.
        bool isAllowlistActivated;
        /// @notice Maximum depth carried across transactions; 0 disables the check.
        uint48 maxCrossTxDepth;
        /// @notice Carried depth the coprocessors are assumed to drain per block.
        uint48 crossTxDepthDecayPerBlock;
    }

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 7;

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.HCULimit")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant HCU_LIMIT_STORAGE_LOCATION =
        0xc13af6c514bff8997f30c90003baa82bd02aad978179d1ce58d85c4319ad6500;

    function _getHCULimitStorage() internal pure virtual returns (HCULimitStorage storage $) {
        assembly {
            $.slot := HCU_LIMIT_STORAGE_LOCATION
        }
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract.
     * @param hcuCapPerBlock Initial global HCU cap per block.
     * @param maxHCUDepthPerTx Maximum sequential HCU depth per transaction.
     * @param maxHCUPerTx Maximum total HCU per transaction.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        uint48 hcuCapPerBlock,
        uint48 maxHCUDepthPerTx,
        uint48 maxHCUPerTx
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        _setHCUPerBlock(hcuCapPerBlock);
        _setMaxHCUPerTx(maxHCUPerTx);
        _setMaxHCUDepthPerTx(maxHCUDepthPerTx);
    }

    /**
     * @notice Re-initializes the contract from V4 (the 0.14.x releases) or V5 (the computation allowlist), leaving the
     *         cross-transaction depth limit disabled.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV6() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice Sets the block-level HCU limit for callers outside the block HCU whitelist.
     * @param hcuPerBlock New block-level cap.
     */
    function setHCUPerBlock(uint48 hcuPerBlock) external onlyACLOwner {
        _setHCUPerBlock(hcuPerBlock);
    }

    /**
     * @notice Sets the per-transaction HCU depth limit.
     * @param maxHCUDepthPerTx New depth limit.
     */
    function setMaxHCUDepthPerTx(uint48 maxHCUDepthPerTx) external onlyACLOwner {
        _setMaxHCUDepthPerTx(maxHCUDepthPerTx);
    }

    /**
     * @notice Sets the per-transaction HCU limit.
     * @param maxHCUPerTx New transaction limit.
     */
    function setMaxHCUPerTx(uint48 maxHCUPerTx) external onlyACLOwner {
        _setMaxHCUPerTx(maxHCUPerTx);
    }

    /**
     * @notice Exempts one caller from the global block HCU cap and meter.
     * @dev An active computation allowlist and transaction and depth limits still apply to this caller.
     * @param account Caller to add to the block HCU whitelist.
     */
    function addToBlockHCUWhitelist(address account) external onlyACLOwner {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.blockHCUWhitelist[account]) revert AlreadyBlockHCUWhitelisted(account);
        $.blockHCUWhitelist[account] = true;
        emit BlockHCUWhitelistAdded(account);
    }

    /**
     * @notice Removes one caller's exemption from the global block HCU cap and meter.
     * @dev Does not change computation allowlist membership.
     * @param account Caller to remove from the block HCU whitelist.
     */
    function removeFromBlockHCUWhitelist(address account) external onlyACLOwner {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if (!$.blockHCUWhitelist[account]) revert NotBlockHCUWhitelisted(account);
        $.blockHCUWhitelist[account] = false;
        emit BlockHCUWhitelistRemoved(account);
    }

    /**
     * @notice Activates or deactivates the computation allowlist without clearing membership or deadlines.
     * @dev When active, callers on the block HCU whitelist must also be on the computation allowlist.
     * @param activated Whether computation requires allowlist membership.
     */
    function setAllowlistActivated(bool activated) external onlyACLOwner {
        _getHCULimitStorage().isAllowlistActivated = activated;
        emit AllowlistActivationChanged(activated);
    }

    /**
     * @notice Atomically admits dapps, including addresses approved before deployment.
     * @dev Admission grants no exemption from HCU limits or membership in the block HCU whitelist.
     * Existing members and repeated addresses are skipped without emitting events.
     * @param accounts Nonempty list of nonzero dapp addresses to admit.
     */
    function addToAllowlist(address[] calldata accounts) external {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if (msg.sender != Ownable2StepUpgradeable(aclAdd).owner() && $.allowlisters[msg.sender] <= block.timestamp) {
            revert CallerNotACLOwnerOrAllowlister(msg.sender);
        }
        if (accounts.length == 0) revert EmptyBatch();
        for (uint256 i = 0; i < accounts.length; ++i) {
            address account = accounts[i];
            if (account == address(0)) revert InvalidZeroAddress();
            if ($.isAllowlisted[account]) continue;
            $.isAllowlisted[account] = true;
            emit AllowlistAdded(account, msg.sender);
        }
    }

    /**
     * @notice Atomically removes dapps from the computation allowlist.
     * @dev While allowlist is activated, removed dapps cannot compute even if on the block HCU whitelist.
     * @param accounts Nonempty list of allowlisted dapp addresses to remove.
     */
    function removeFromAllowlist(address[] calldata accounts) external onlyACLOwner {
        if (accounts.length == 0) revert EmptyBatch();
        HCULimitStorage storage $ = _getHCULimitStorage();
        for (uint256 i = 0; i < accounts.length; ++i) {
            address account = accounts[i];
            if (!$.isAllowlisted[account]) revert NotAllowlisted(account);
            $.isAllowlisted[account] = false;
            emit AllowlistRemoved(account);
        }
    }

    /**
     * @notice Grants or replaces an allowlister deadline, including renewal of expired grants.
     * @param account Nonzero account allowed to admit dapps until the deadline.
     * @param deadline Future Unix timestamp in seconds; authority expires at this timestamp.
     */
    function addAllowlister(address account, uint256 deadline) external onlyACLOwner {
        if (account == address(0)) revert InvalidZeroAddress();
        if (deadline <= block.timestamp) revert InvalidAllowlisterDeadline(deadline);
        _getHCULimitStorage().allowlisters[account] = deadline;
        emit AllowlisterAdded(account, deadline);
    }

    /**
     * @notice Clears an allowlister grant without removing previously admitted dapps.
     * @dev Clears active or expired deadlines. An absent grant succeeds without emitting an event.
     * @param account Account whose allowlister grant is cleared.
     */
    function removeAllowlister(address account) external onlyACLOwner {
        _removeAllowlister(account);
    }

    /// @notice Clears the sender's stored allowlister grant, active or expired, without removing previously admitted dapps.
    function renounceAllowlister() external {
        if (_getHCULimitStorage().allowlisters[msg.sender] == 0) revert NotAllowlister(msg.sender);
        _removeAllowlister(msg.sender);
    }

    function _removeAllowlister(address account) private {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.allowlisters[account] == 0) return;
        delete $.allowlisters[account];
        emit AllowlisterRemoved(account);
    }

    /**
     * @notice Sets the cross-transaction depth limit and how much carried depth decays per block.
     * @dev (0, 0) disables it. Decay must be nonzero when enabled, or a chain that reaches the limit stays
     *      blocked forever. The limit must be at least maxHCUDepthPerTx, so a transaction whose inputs carry no
     *      depth is never refused. Permissions granted while disabled carry no depth. A new decay rate also
     *      applies to depths already stored.
     */
    function setCrossTxDepthLimit(uint48 maxDepth, uint48 decayPerBlock) external onlyACLOwner {
        if ((maxDepth == 0) != (decayPerBlock == 0)) revert InvalidCrossTxDepthLimit();
        HCULimitStorage storage $ = _getHCULimitStorage();
        if (maxDepth != 0) {
            if (maxDepth < $.maxHCUDepthPerTx) revert CrossTxDepthBelowMaxDepthPerTx();
            // Every operation on a stored handle reads ACL.getStoredDepth: refuse to enable before ACL has it.
            ACL(aclAdd).getStoredDepth(bytes32(0), address(0));
        }
        $.maxCrossTxDepth = maxDepth;
        $.crossTxDepthDecayPerBlock = decayPerBlock;
        emit CrossTxDepthLimitSet(maxDepth, decayPerBlock);
    }

    /**
     * @notice Caches `handle`'s carried depth for the rest of this transaction and returns it. If this
     *         transaction has not seen the handle yet, the depth comes from `holder`'s permission.
     *         Returns 0 when disabled.
     */
    function cacheDepth(bytes32 handle, address holder) external returns (uint48) {
        if (msg.sender != aclAdd) revert CallerMustBeACLContract();
        if (_getHCULimitStorage().maxCrossTxDepth == 0) return 0;
        return uint48(_cacheDepths(handle, holder) >> 128);
    }

    /**
     * @notice Checks the HCU depth for an operation with no ciphertext operands.
     */
    function _adjustAndCheckFheTransactionLimitZeroOp(
        uint256 opCost,
        address caller,
        bytes32 result
    ) internal virtual override {
        _updateAndVerifyHCUTransactionLimit(_work(opCost), caller);
        _setResultDepths(result, _latency(opCost), 0);
    }

    /**
     * @notice Adjusts the sequential HCU for the transaction.
     */
    function _adjustAndCheckFheTransactionLimitOneOp(
        uint256 opCost,
        address caller,
        bytes32 op1,
        bytes32 result
    ) internal virtual override {
        _updateAndVerifyHCUTransactionLimit(_work(opCost), caller);
        _setResultDepths(result, _latency(opCost), _cacheDepths(op1, caller));
    }

    /**
     * @notice Adjusts the current HCU for the transaction.
     */
    function _adjustAndCheckFheTransactionLimitTwoOps(
        uint256 opCost,
        address caller,
        bytes32 op1,
        bytes32 op2,
        bytes32 result
    ) internal virtual override {
        _updateAndVerifyHCUTransactionLimit(_work(opCost), caller);
        _setResultDepths(result, _latency(opCost), _maxDepths(_cacheDepths(op1, caller), _cacheDepths(op2, caller)));
    }

    /**
     * @notice Adjusts the current HCU for the transaction.
     */
    function _adjustAndCheckFheTransactionLimitThreeOps(
        uint256 opCost,
        address caller,
        bytes32 op1,
        bytes32 op2,
        bytes32 op3,
        bytes32 result
    ) internal virtual override {
        _updateAndVerifyHCUTransactionLimit(_work(opCost), caller);
        _setResultDepths(
            result,
            _latency(opCost),
            _maxDepths(_cacheDepths(op1, caller), _maxDepths(_cacheDepths(op2, caller), _cacheDepths(op3, caller)))
        );
    }

    /**
     * @notice Adjusts the current HCU for an operation on an array of handles.
     */
    function _adjustAndCheckFheTransactionLimitArray(
        uint256 opCost,
        address caller,
        bytes32[] calldata values,
        bytes32 result
    ) internal virtual override {
        _updateAndVerifyHCUTransactionLimit(_work(opCost), caller);

        uint256 inputDepths = 0;
        for (uint256 i = 0; i < values.length; i++) {
            inputDepths = _maxDepths(inputDepths, _cacheDepths(values[i], caller));
        }

        _setResultDepths(result, _latency(opCost), inputDepths);
    }

    /**
     * @notice Adjusts the current HCU for an operation on a handle and an array of handles.
     */
    function _adjustAndCheckFheTransactionLimitValueAndArray(
        uint256 opCost,
        address caller,
        bytes32 value,
        bytes32[] calldata values,
        bytes32 result
    ) internal virtual override {
        _updateAndVerifyHCUTransactionLimit(_work(opCost), caller);

        uint256 inputDepths = _cacheDepths(value, caller);
        for (uint256 i = 0; i < values.length; i++) {
            inputDepths = _maxDepths(inputDepths, _cacheDepths(values[i], caller));
        }

        _setResultDepths(result, _latency(opCost), inputDepths);
    }

    /// @dev Records `result`'s depths: the operation's latency added to both depths of its deepest input, checked
    ///      against both limits.
    function _setResultDepths(bytes32 result, uint256 opLatency, uint256 inputDepths) private {
        uint256 depths = (opLatency | (opLatency << 128)) + inputDepths;
        HCULimitStorage storage $ = _getHCULimitStorage();
        if (uint128(depths) > $.maxHCUDepthPerTx) revert HCUTransactionDepthLimitExceeded();
        uint256 maxDepth = $.maxCrossTxDepth;
        uint256 carried = depths >> 128;
        if (maxDepth != 0 && carried > maxDepth) {
            revert HCUCrossTxDepthLimitExceeded(_blocksToDecay(carried - maxDepth, $.crossTxDepthDecayPerBlock));
        }
        _storeDepths(result, depths);
    }

    /**
     * @dev Returns both depths of `handle`. A handle first seen in this transaction has no depth within it;
     *      its carried depth is the one stored with `holder`'s permission, or 0 without one: verified inputs,
     *      bridged handles and permissions granted while disabled start new chains.
     */
    function _cacheDepths(bytes32 handle, address holder) private returns (uint256 depths) {
        bool known;
        (depths, known) = _loadDepths(handle);
        if (known) return depths;
        if (_getHCULimitStorage().maxCrossTxDepth == 0) return 0;
        (uint48 storedDepth, uint48 storedBlock) = ACL(aclAdd).getStoredDepth(handle, holder);
        depths = _decayed(storedDepth, storedBlock, _getHCULimitStorage().crossTxDepthDecayPerBlock) << 128;
        _storeDepths(handle, depths);
    }

    /// @dev Returns `handle`'s depths recorded in this transaction, and whether any were recorded.
    function _loadDepths(bytes32 handle) internal view returns (uint256 depths, bool known) {
        // Slot zero holds the transaction's HCU.
        if (handle == bytes32(0)) revert InvalidZeroHandle();
        uint256 word;
        assembly {
            word := tload(handle)
        }
        return (word & ~DEPTH_KNOWN, word & DEPTH_KNOWN != 0);
    }

    /// @dev Records `handle`'s depths for the rest of this transaction.
    function _storeDepths(bytes32 handle, uint256 depths) internal {
        // Slot zero holds the transaction's HCU.
        if (handle == bytes32(0)) revert InvalidZeroHandle();
        uint256 word = depths | DEPTH_KNOWN;
        assembly {
            tstore(handle, word)
        }
    }

    /// @dev Returns `value` minus `decayPerBlock` for each block since `sinceBlock`, floored at 0.
    function _decayed(uint48 value, uint48 sinceBlock, uint256 decayPerBlock) private view returns (uint256) {
        uint256 decay = (block.number - sinceBlock) * decayPerBlock;
        return decay >= value ? 0 : value - decay;
    }

    /// @dev Returns the blocks of decay needed to absorb `excess`, rounded up.
    function _blocksToDecay(uint256 excess, uint256 decayPerBlock) private pure returns (uint256) {
        return (excess + decayPerBlock - 1) / decayPerBlock;
    }

    /**
     * @notice Updates and verifies the HCU transaction limit.
     * @param opWork The operation's work, charged to the block meter and the transaction.
     * @param caller Original dapp caller address for allowlist and block-level checks.
     */
    function _updateAndVerifyHCUTransactionLimit(uint256 opWork, address caller) internal virtual {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.isAllowlistActivated && !$.isAllowlisted[caller]) {
            revert CallerNotAllowlisted(caller);
        }
        _updateAndVerifyHCUBlockLimit(opWork, caller);

        uint256 transactionHCU = opWork + _getHCUForTransaction();
        if (transactionHCU > uint256($.maxHCUPerTx)) {
            revert HCUTransactionLimitExceeded();
        }
        _setHCUForTransaction(transactionHCU);
    }

    /**
     * @notice Updates and enforces the public block HCU cap for one operation.
     * @dev No-op if caller is whitelisted.
     * @param opWork Work of the current operation.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function _updateAndVerifyHCUBlockLimit(uint256 opWork, address caller) internal virtual {
        HCULimitStorage storage $ = _getHCULimitStorage();

        if ($.blockHCUWhitelist[caller]) {
            return;
        }

        uint48 currentBlock = uint48(block.number);
        uint48 storedHCU = $.usedBlockHCU;
        if ($.lastSeenBlockNumber != currentBlock) {
            storedHCU = 0;
        }

        uint256 nextHCU = uint256(storedHCU) + opWork;
        if (nextHCU > uint256($.globalHCUCapPerBlock)) {
            revert HCUBlockLimitExceeded();
        }
        $.usedBlockHCU = uint48(nextHCU);
        $.lastSeenBlockNumber = currentBlock;
    }

    /**
     * @notice Gets the total HCU for the transaction.
     * @return transactionHCU The HCU for the transaction.
     * @dev This function uses inline assembly to store the HCU in a specific storage location.
     */
    function _getHCUForTransaction() internal view virtual returns (uint256 transactionHCU) {
        assembly {
            transactionHCU := tload(0)
        }
    }

    /**
     * @notice Updates the current HCU consumption for the transaction and stores it in the transient storage.
     * @param transactionHCU The total HCU for the transaction.
     * @dev This function uses inline assembly to store the HCU in a specific transient storage slot.
     */
    function _setHCUForTransaction(uint256 transactionHCU) internal virtual {
        assembly {
            tstore(0, transactionHCU) // to avoid collisions with handles (see _storeDepths)
        }
    }

    /**
     * @notice Sets the global HCU cap per block.
     * @param hcuPerBlock New cap value.
     * @dev Enforces hcuPerBlock >= maxHCUPerTx.
     */
    function _setHCUPerBlock(uint48 hcuPerBlock) private {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if (hcuPerBlock < $.maxHCUPerTx) revert HCUPerBlockBelowMaxPerTx();
        $.globalHCUCapPerBlock = hcuPerBlock;
        emit HCUPerBlockSet(hcuPerBlock);
    }

    /**
     * @notice Sets the per-transaction HCU depth limit.
     * @param maxHCUDepthPerTx New depth limit.
     * @dev Enforces a nonzero maxCrossTxDepth >= maxHCUDepthPerTx. Not compared with maxHCUPerTx, which counts
     *      work while depth counts latency.
     */
    function _setMaxHCUDepthPerTx(uint48 maxHCUDepthPerTx) private {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.maxCrossTxDepth != 0 && $.maxCrossTxDepth < maxHCUDepthPerTx) revert CrossTxDepthBelowMaxDepthPerTx();
        $.maxHCUDepthPerTx = maxHCUDepthPerTx;
        emit MaxHCUDepthPerTxSet(maxHCUDepthPerTx);
    }

    /**
     * @notice Sets the per-transaction HCU limit.
     * @param maxHCUPerTx New transaction limit.
     * @dev Enforces hcuPerBlock >= maxHCUPerTx.
     */
    function _setMaxHCUPerTx(uint48 maxHCUPerTx) private {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.globalHCUCapPerBlock < maxHCUPerTx) revert HCUPerBlockBelowMaxPerTx();
        $.maxHCUPerTx = maxHCUPerTx;
        emit MaxHCUPerTxSet(maxHCUPerTx);
    }

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}

    /// @dev Takes the maximum of each depth separately: the deepest branch in this transaction may carry less.
    function _maxDepths(uint256 a, uint256 b) private pure returns (uint256) {
        uint256 txDepth = uint128(a) >= uint128(b) ? uint128(a) : uint128(b);
        uint256 carried = a >> 128 >= b >> 128 ? a >> 128 : b >> 128;
        return txDepth | (carried << 128);
    }

    /**
     * @notice Getter function for the FHEVMExecutor contract address.
     * @return fhevmExecutorAddress Address of the FHEVMExecutor.
     */
    function getFHEVMExecutorAddress() public view virtual returns (address) {
        return FHEVM_EXECUTOR_ADDRESS;
    }

    /**
     * @notice Getter for the name and version of the contract.
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
     * @notice Returns the global block HCU cap.
     */
    function getGlobalHCUCapPerBlock() public view virtual returns (uint48) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        return $.globalHCUCapPerBlock;
    }

    /**
     * @notice Returns the cross-transaction depth limit and its decay per block; (0, 0) when disabled.
     */
    function getCrossTxDepthLimit() external view returns (uint48 maxDepth, uint48 decayPerBlock) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        return ($.maxCrossTxDepth, $.crossTxDepthDecayPerBlock);
    }

    /**
     * @notice Returns the per-transaction HCU depth limit.
     */
    function getMaxHCUDepthPerTx() public view virtual returns (uint48) {
        return _getHCULimitStorage().maxHCUDepthPerTx;
    }

    /**
     * @notice Returns the per-transaction HCU limit.
     */
    function getMaxHCUPerTx() public view virtual returns (uint48) {
        return _getHCULimitStorage().maxHCUPerTx;
    }

    /**
     * @notice Returns whether a caller bypasses the global block HCU cap.
     * @dev This exemption grants no computation admission and leaves transaction and depth limits in force.
     * @param account Caller address.
     */
    function isBlockHCUWhitelisted(address account) public view virtual returns (bool) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        return $.blockHCUWhitelist[account];
    }

    /// @notice Returns whether HCU-charged computation requires allowlist membership.
    function isAllowlistActivated() public view virtual returns (bool) {
        return _getHCULimitStorage().isAllowlistActivated;
    }

    /// @notice Returns whether a dapp is on the computation allowlist, independently of activation.
    /// @dev Membership grants no exemption from the block cap, transaction limit, or depth limit.
    function isAllowlisted(address account) public view virtual returns (bool) {
        return _getHCULimitStorage().isAllowlisted[account];
    }

    /// @notice Returns whether the account's allowlister deadline is strictly in the future.
    function isAllowlister(address account) public view virtual returns (bool) {
        return _getHCULimitStorage().allowlisters[account] > block.timestamp;
    }

    /// @notice Returns the stored Unix deadline in seconds, including expired grants; zero means no grant.
    function allowlisterDeadline(address account) public view virtual returns (uint256) {
        return _getHCULimitStorage().allowlisters[account];
    }

    /**
     * @notice Returns the effective public block HCU meter for the current block.
     * @dev If storage still contains a previous block meter, returns `(block.number, 0)`.
     */
    function getBlockMeter() external view returns (uint48 blockNumber, uint48 usedHCU) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint48 currentBlock = uint48(block.number);
        if ($.lastSeenBlockNumber != currentBlock) {
            return (currentBlock, 0);
        }
        return (currentBlock, $.usedBlockHCU);
    }
}
