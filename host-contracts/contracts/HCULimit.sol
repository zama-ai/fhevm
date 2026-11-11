// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {aclAdd, hcuConfiguratorAdd} from "../addresses/FHEVMHostAddresses.sol";
import {ACL} from "./ACL.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {HCULimitOperators} from "./HCULimitOperators.sol";
import {ALLOWLIST_QUOTA, BLOCK_WHITELIST_QUOTA, DEFAULT_QUOTA, MAX_BPS} from "./shared/Constants.sol";

/**
 * @title HCULimit
 * @notice This contract manages the total allowed complexity for FHE operations at the
 * transaction level, the HCU depth carried across transactions, and the HCU quotas per block.
 * @dev The contract is designed to be used with the FHEVMExecutor contract.
 */
/// @dev This contract was migrated from Ownable2StepUpgradeable to ACLOwnable.
/// Deployed proxies retain residual `_owner` and `_pendingOwner` values in the
/// Ownable2StepUpgradeable EIP-7201 storage namespace. These slots are unused
/// by ACLOwnable and have no effect on contract behavior.
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract HCULimit is UUPSUpgradeableEmptyProxy, ACLOwnable, HCULimitOperators {
    /// @notice Returned when an operation would take a quota's usage over its maximum.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    /// @param retryAfterBlocks Blocks of decay after which this operation fits, if nothing else spends the quota;
    ///        type(uint256).max if the operation alone exceeds the maximum. Later operations of the same
    ///        transaction can need more, so clients simulate again after waiting.
    error HCUQuotaExceeded(address account, uint16 quotaId, uint256 retryAfterBlocks);

    /// @notice Returned when the operation would take the chain's load over the threshold of the quota's tier, and the
    ///         load needs a longer wait than the quota.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    /// @param retryAfterBlocks Blocks of decay after which this operation fits, if no contract adds load meanwhile;
    ///        type(uint256).max if the operation alone exceeds the threshold. Other contracts keep adding load, so
    ///        the wait can be longer.
    error HCUChainLoadTooHigh(address account, uint16 quotaId, uint256 retryAfterBlocks);

    /// @notice Returned when the quota's tier admits no load: its threshold is 0, or the capacity is stopped. Retrying
    ///         fails until the owner moves the quota or changes the threshold or the capacity.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    error HCUTierStopped(address account, uint16 quotaId);

    /// @notice Returned when the contract's quota has a maximum of 0. Retrying fails until the contract is moved
    ///         or the owner resizes the quota.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    error HCUQuotaStopped(address account, uint16 quotaId);

    /// @notice Returned when the contract's quota is paused. Retrying fails until the owner unpauses it.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    error HCUQuotaPaused(address account, uint16 quotaId);

    /// @notice Returned if the transaction exceeds the maximum allowed homomorphic complexity units.
    error HCUTransactionLimitExceeded();

    /// @notice Returned if the transaction exceeds the maximum allowed depth of homomorphic complexity units.
    error HCUTransactionDepthLimitExceeded();

    /// @notice Returned if a handle is the zero handle.
    /// @dev Handle slot zero is reserved for the transaction HCU accumulator.
    error InvalidZeroHandle();

    /// @notice Returned when the caller is not the ACL contract.
    error CallerMustBeACLContract();

    /// @notice Returned when an operation's depth carried across transactions exceeds the limit.
    /// @param retryAfterBlocks Blocks of decay after which this operation fits, if nothing else extends its chain.
    ///        Later operations of the same transaction can need more, so clients simulate again after waiting.
    error HCUCrossTxDepthLimitExceeded(uint256 retryAfterBlocks);

    /// @notice Returned when the caller is not HCUConfigurator, the only writer of the settings.
    error CallerMustBeHCUConfigurator();

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "HCULimit";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 6;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @notice A quota or capacity with this maximum is not metered.
    uint48 private constant NO_CAP = type(uint48).max;

    /// @notice Tier of every quota never moved to another tier.
    uint8 private constant DEFAULT_TIER = 0;

    /// @dev Per-handle transient word: bit 255 marks the depths as known, bits 128-254 hold the
    ///      depth carried across transactions and bits 0-127 the depth within this transaction.
    uint256 private constant DEPTH_KNOWN = 1 << 255;

    /// @dev Limit, meter, pause flag and tier of one quota, in one slot. All contracts assigned to the quota share it.
    ///      Usage decays by decayPerBlock every block, as carried depth does, and an operation fits while usage stays at
    ///      most maxHCU.
    struct Quota {
        uint48 maxHCU;
        uint48 decayPerBlock;
        uint48 usedHCU;
        uint48 lastBlock;
        bool paused;
        uint8 tier;
    }

    /// @dev Limit and meter of all FHE work on this chain, in one slot. Every operation charges it, whatever its quota,
    ///      and the load decays as a quota's usage does.
    struct Capacity {
        uint48 maxHCU;
        uint48 decayPerBlock;
        uint48 usedHCU;
        uint48 lastBlock;
    }

    /// @custom:storage-location erc7201:fhevm.storage.HCULimit
    /// @dev The first five uint48 fields pack into a single 256-bit slot (5 × 48 = 240 bits). The
    ///      cross-transaction fields and the quotas are appended after the mapping to keep the existing layout.
    struct HCULimitStorage {
        /// @notice Unused since quotas replaced the global block cap; reinitializeV6 copies it into a quota.
        uint48 globalHCUCapPerBlock;
        /// @notice Unused since quotas replaced the global block cap.
        uint48 usedBlockHCU;
        /// @notice Unused since quotas replaced the global block cap.
        uint48 lastSeenBlockNumber;
        /// @notice Maximum sequential HCU depth per transaction.
        uint48 maxHCUDepthPerTx;
        /// @notice Maximum total HCU per transaction.
        uint48 maxHCUPerTx;
        /// @notice Contracts that skipped the global block cap. No setter since quotas replaced it; read only to route
        ///         allowlisted contracts nobody assigned (see _quotaOf).
        mapping(address account => bool isWhitelisted) blockHCUWhitelist;
        // The next three fields are the allowlist of the 0.14.x patch, in its order and types.
        /// @notice Contracts the allowlist admitted. No setter since quotas replaced it; an entry is deleted when its
        ///         contract moves to the default quota.
        mapping(address account => bool allowed) isAllowlisted;
        /// @notice Unused since quota admins replaced allowlisters (the patch stored each allowlister's deadline).
        mapping(address account => uint256 deadline) allowlisters;
        /// @notice Whether the allowlist restricted FHE computation when quotas replaced it. No setter; while true and
        ///         the default quota is stopped, allowlisted contracts nobody assigned keep computing (see _quotaOf).
        bool isAllowlistActivated;
        /// @notice Maximum depth carried across transactions; 0 disables the check.
        uint48 maxCrossTxDepth;
        /// @notice Carried depth the coprocessors are assumed to drain per block.
        uint48 crossTxDepthDecayPerBlock;
        /// @notice Cap and meter of each quota.
        mapping(uint16 quotaId => Quota) quotas;
        /// @notice Quota of each contract; the default quota when never assigned.
        mapping(address account => uint16 quotaId) contractQuota;
        /// @notice HCU load the coprocessors absorb on this chain, metered across all quotas.
        Capacity capacity;
        /// @notice Load, in basis points of the capacity's maximum, above which each tier's operations are refused.
        mapping(uint8 tier => uint16 maxLoadBps) tierMaxLoadBps;
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

    modifier onlyHCUConfigurator() {
        if (msg.sender != hcuConfiguratorAdd) revert CallerMustBeHCUConfigurator();
        _;
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract.
     * @param defaultQuotaHCUPerBlock Maximum and decay per block of the default quota; type(uint48).max leaves it unmetered.
     * @param maxHCUDepthPerTx Maximum sequential HCU depth per transaction.
     * @param maxHCUPerTx Maximum total HCU per transaction.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        uint48 defaultQuotaHCUPerBlock,
        uint48 maxHCUDepthPerTx,
        uint48 maxHCUPerTx
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        _setQuota(DEFAULT_QUOTA, defaultQuotaHCUPerBlock, defaultQuotaHCUPerBlock);
        _setCapacity(NO_CAP, NO_CAP);
        _setTier(DEFAULT_TIER, MAX_BPS);
        _setTxLimits(maxHCUPerTx, maxHCUDepthPerTx);
    }

    /**
     * @notice Re-initializes the contract from V5, which has the allowlist, or from V4.
     * @dev Quotas replace the global block cap, its whitelist and the allowlist. With the allowlist active, the default
     *      quota is stopped, so unknown contracts stay refused, and allowlisted contracts share the global cap in the
     *      allowlist quota, or are unmetered if also block-whitelisted. Otherwise the default quota gets the global cap,
     *      and the upgrade proposal assigns any block-whitelisted contract to a quota that is not metered. The cap becomes maximum and decay per block, which allows the
     *      same HCU per block. The capacity starts unmetered and the default tier admits up to full capacity, so tiers
     *      change nothing until the owner sets the capacity.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV6() public virtual reinitializer(REINITIALIZER_VERSION) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint48 globalCap = $.globalHCUCapPerBlock;
        if ($.isAllowlistActivated) {
            _setQuota(DEFAULT_QUOTA, 0, 0);
            _setQuota(ALLOWLIST_QUOTA, globalCap, globalCap);
            _setQuota(BLOCK_WHITELIST_QUOTA, NO_CAP, NO_CAP);
        } else {
            _setQuota(DEFAULT_QUOTA, globalCap, globalCap);
        }
        _setCapacity(NO_CAP, NO_CAP);
        _setTier(DEFAULT_TIER, MAX_BPS);
    }

    // Setters for HCUConfigurator, which checks each value and emits its event.

    function setQuota(uint16 quotaId, uint48 maxHCU, uint48 decayPerBlock) external onlyHCUConfigurator {
        _setQuota(quotaId, maxHCU, decayPerBlock);
    }

    /// @dev A contract sent to the default quota also leaves the allowlist, so that the allowlist cannot route it again.
    function assignQuota(uint16 quotaId, address account) external onlyHCUConfigurator {
        HCULimitStorage storage $ = _getHCULimitStorage();
        $.contractQuota[account] = quotaId;
        if (quotaId == DEFAULT_QUOTA) delete $.isAllowlisted[account];
    }

    function setQuotaPaused(uint16 quotaId, bool paused) external onlyHCUConfigurator {
        _getHCULimitStorage().quotas[quotaId].paused = paused;
    }

    function setQuotaTier(uint16 quotaId, uint8 tier) external onlyHCUConfigurator {
        _getHCULimitStorage().quotas[quotaId].tier = tier;
    }

    function setTier(uint8 tier, uint16 maxLoadBps) external onlyHCUConfigurator {
        _setTier(tier, maxLoadBps);
    }

    function setCapacity(uint48 maxHCU, uint48 decayPerBlock) external onlyHCUConfigurator {
        _setCapacity(maxHCU, decayPerBlock);
    }

    function setTxLimits(uint48 maxHCUPerTx, uint48 maxHCUDepthPerTx) external onlyHCUConfigurator {
        _setTxLimits(maxHCUPerTx, maxHCUDepthPerTx);
    }

    function setCrossTxDepthLimit(uint48 maxDepth, uint48 decayPerBlock) external onlyHCUConfigurator {
        HCULimitStorage storage $ = _getHCULimitStorage();
        $.maxCrossTxDepth = maxDepth;
        $.crossTxDepthDecayPerBlock = decayPerBlock;
    }

    function getCrossTxDepthLimit() external view returns (uint48 maxDepth, uint48 decayPerBlock) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        return ($.maxCrossTxDepth, $.crossTxDepthDecayPerBlock);
    }

    /**
     * @notice Caches `handle`'s carried depth for the rest of this transaction and returns it. If this
     *         transaction has not seen the handle yet, the depth comes from `holder`'s permission.
     *         Returns 0 when disabled.
     */
    function cacheDepth(bytes32 handle, address holder) external returns (uint48) {
        if (msg.sender != aclAdd) revert CallerMustBeACLContract();
        if (_getHCULimitStorage().maxCrossTxDepth == 0) return 0;
        return uint48(_handleDepths(handle, holder) >> 128);
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
        _setHCUForHandle(result, depths | DEPTH_KNOWN);
    }

    /// @dev Returns `value` minus `decayPerBlock` for each block since `sinceBlock`, floored at 0.
    ///      Carried depth and quota usage both decay this way.
    function _decayed(uint48 value, uint48 sinceBlock, uint256 decayPerBlock) private view returns (uint256) {
        uint256 decay = (block.number - sinceBlock) * decayPerBlock;
        return decay >= value ? 0 : value - decay;
    }

    /// @dev Returns the blocks of decay needed to absorb `excess`, rounded up.
    function _blocksToDecay(uint256 excess, uint256 decayPerBlock) private pure returns (uint256) {
        return (excess + decayPerBlock - 1) / decayPerBlock;
    }

    /// @dev Returns a meter's usage after decay plus `opHCU`, and the blocks of decay before that fits `limit`:
    ///      0 when it fits now, type(uint256).max when `opHCU` alone exceeds `limit`.
    function _meter(
        uint48 usedHCU,
        uint48 lastBlock,
        uint48 decayPerBlock,
        uint256 limit,
        uint256 opHCU
    ) private view returns (uint256 used, uint256 retryAfterBlocks) {
        used = _decayed(usedHCU, lastBlock, decayPerBlock) + opHCU;
        if (used > limit) {
            retryAfterBlocks = opHCU > limit ? type(uint256).max : _blocksToDecay(used - limit, decayPerBlock);
        }
    }

    /**
     * @dev Returns both depths of `handle`. A handle first seen in this transaction has no depth within it;
     *      its carried depth is the one stored with `holder`'s permission, or 0 without one: verified inputs,
     *      bridged handles and permissions granted while disabled start new chains.
     */
    function _handleDepths(bytes32 handle, address holder) private returns (uint256 depths) {
        // Handle slot zero is reserved for _getHCUForTransaction.
        if (handle == bytes32(0)) revert InvalidZeroHandle();
        assembly {
            depths := tload(handle)
        }
        if (depths & DEPTH_KNOWN != 0) return depths & ~DEPTH_KNOWN;
        if (_getHCULimitStorage().maxCrossTxDepth == 0) return 0;
        (uint48 storedDepth, uint48 storedBlock) = ACL(aclAdd).getStoredDepth(handle, holder);
        depths = _decayed(storedDepth, storedBlock, _getHCULimitStorage().crossTxDepthDecayPerBlock) << 128;
        _setHCUForHandle(handle, depths | DEPTH_KNOWN);
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
        _updateAndVerifyHCUTransactionLimit(opCost >> 32, caller);
        _setResultDepths(result, uint32(opCost), _handleDepths(op1, caller));
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
        _updateAndVerifyHCUTransactionLimit(opCost >> 32, caller);
        _setResultDepths(result, uint32(opCost), _maxDepths(_handleDepths(op1, caller), _handleDepths(op2, caller)));
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
        _updateAndVerifyHCUTransactionLimit(opCost >> 32, caller);
        _setResultDepths(
            result,
            uint32(opCost),
            _maxDepths(_handleDepths(op1, caller), _maxDepths(_handleDepths(op2, caller), _handleDepths(op3, caller)))
        );
    }

    /**
     * @notice Adjusts the current HCU for an operation without encrypted inputs.
     */
    function _adjustAndCheckFheTransactionLimitNoOps(
        uint256 opCost,
        address caller,
        bytes32 result
    ) internal virtual override {
        _updateAndVerifyHCUTransactionLimit(opCost >> 32, caller);
        _setResultDepths(result, uint32(opCost), 0);
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
        _updateAndVerifyHCUTransactionLimit(opCost >> 32, caller);

        uint256 inputDepths = 0;
        for (uint256 i = 0; i < values.length; i++) {
            inputDepths = _maxDepths(inputDepths, _handleDepths(values[i], caller));
        }

        _setResultDepths(result, uint32(opCost), inputDepths);
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
        _updateAndVerifyHCUTransactionLimit(opCost >> 32, caller);

        uint256 inputDepths = _handleDepths(value, caller);
        for (uint256 i = 0; i < values.length; i++) {
            inputDepths = _maxDepths(inputDepths, _handleDepths(values[i], caller));
        }

        _setResultDepths(result, uint32(opCost), inputDepths);
    }

    /**
     * @notice Updates and verifies the HCU transaction limit.
     * @param opWork The operation's work, charged to the quota, the capacity and the transaction.
     * @param caller Contract that runs the operation, charged to its quota.
     */
    function _updateAndVerifyHCUTransactionLimit(uint256 opWork, address caller) internal virtual {
        _updateAndVerifyHCUQuota(opWork, caller);

        uint256 transactionHCU = opWork + _getHCUForTransaction();
        if (transactionHCU > uint256(_getHCULimitStorage().maxHCUPerTx)) {
            revert HCUTransactionLimitExceeded();
        }
        _setHCUForTransaction(transactionHCU);
    }

    /**
     * @notice Charges one operation to the quota of the contract that runs it and to the chain's capacity. The
     *         operation must fit the quota's maximum and keep the chain's load within the threshold of the quota's tier.
     * @dev Checks both meters before writing either, so a refused operation spends neither.
     * @param opHCU HCU cost of the current operation.
     * @param caller Contract that calls FHEVMExecutor.
     */
    function _updateAndVerifyHCUQuota(uint256 opHCU, address caller) internal virtual {
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint16 quotaId = _quotaOf(caller);
        Quota storage quota = $.quotas[quotaId];
        if (quota.paused) revert HCUQuotaPaused(caller, quotaId);
        uint48 maxHCU = quota.maxHCU;
        if (maxHCU == 0) revert HCUQuotaStopped(caller, quotaId);
        uint256 used;
        uint256 quotaWait;
        if (maxHCU != NO_CAP) {
            (used, quotaWait) = _meter(quota.usedHCU, quota.lastBlock, quota.decayPerBlock, maxHCU, opHCU);
        }
        // An unmetered quota still charges the capacity, so the capacity sees all load.
        (uint256 load, uint256 loadWait) = _chainLoad(quota.tier, opHCU, caller, quotaId);
        // The meter with the longer wait names the error, so waiting that long clears both.
        if (loadWait > quotaWait) revert HCUChainLoadTooHigh(caller, quotaId, loadWait);
        if (quotaWait != 0) revert HCUQuotaExceeded(caller, quotaId, quotaWait);
        if (maxHCU != NO_CAP) {
            quota.usedHCU = uint48(used);
            quota.lastBlock = uint48(block.number);
        }
        Capacity storage capacity = $.capacity;
        if (capacity.maxHCU != NO_CAP) {
            capacity.usedHCU = uint48(load);
            capacity.lastBlock = uint48(block.number);
        }
    }

    /// @dev Returns the quota a contract computes in: the one it is assigned to, else, while the default quota is
    ///      stopped and the allowlist was active, the quota that replaced the allowlist for it.
    function _quotaOf(address account) private view returns (uint16) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint16 quotaId = $.contractQuota[account];
        if (quotaId != DEFAULT_QUOTA || $.quotas[DEFAULT_QUOTA].maxHCU != 0) return quotaId;
        if (!$.isAllowlistActivated || !$.isAllowlisted[account]) return DEFAULT_QUOTA;
        return $.blockHCUWhitelist[account] ? BLOCK_WHITELIST_QUOTA : ALLOWLIST_QUOTA;
    }

    /// @dev Returns the chain's load after decay plus `opHCU`, and the blocks of decay before it fits the threshold of
    ///      `tier`; (0, 0) while the capacity is unmetered.
    function _chainLoad(
        uint8 tier,
        uint256 opHCU,
        address caller,
        uint16 quotaId
    ) private view returns (uint256 load, uint256 retryAfterBlocks) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        Capacity storage capacity = $.capacity;
        uint48 maxLoad = capacity.maxHCU;
        if (maxLoad == NO_CAP) return (0, 0);
        uint256 threshold = (uint256(maxLoad) * $.tierMaxLoadBps[tier]) / MAX_BPS;
        if (threshold == 0) revert HCUTierStopped(caller, quotaId);
        return _meter(capacity.usedHCU, capacity.lastBlock, capacity.decayPerBlock, threshold, opHCU);
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
     * @notice Sets the HCU for a handle in the transient storage.
     * @param handle The handle for which to set the HCU.
     * @param handleHCU The HCU to set for the handle.
     * @dev This function uses inline assembly to store the HCU in a specific transient storage slot.
     */
    function _setHCUForHandle(bytes32 handle, uint256 handleHCU) internal virtual {
        // Handle slot zero is reserved for _setHCUForTransaction.
        if (handle == bytes32(0)) revert InvalidZeroHandle();
        assembly {
            tstore(handle, handleHCU)
        }
    }

    /**
     * @notice Updates the current HCU consumption for the transaction and stores it in the transient storage.
     * @param transactionHCU The total HCU for the transaction.
     * @dev This function uses inline assembly to store the HCU in a specific transient storage slot.
     */
    function _setHCUForTransaction(uint256 transactionHCU) internal virtual {
        assembly {
            tstore(0, transactionHCU) // to avoid collisions with handles (see _setHCUForHandle)
        }
    }

    function _setQuota(uint16 quotaId, uint48 maxHCU, uint48 decayPerBlock) private {
        Quota storage quota = _getHCULimitStorage().quotas[quotaId];
        quota.maxHCU = maxHCU;
        quota.decayPerBlock = decayPerBlock;
    }

    function _setCapacity(uint48 maxHCU, uint48 decayPerBlock) private {
        Capacity storage capacity = _getHCULimitStorage().capacity;
        capacity.maxHCU = maxHCU;
        capacity.decayPerBlock = decayPerBlock;
    }

    function _setTier(uint8 tier, uint16 maxLoadBps) private {
        _getHCULimitStorage().tierMaxLoadBps[tier] = maxLoadBps;
    }

    function _setTxLimits(uint48 maxHCUPerTx, uint48 maxHCUDepthPerTx) private {
        HCULimitStorage storage $ = _getHCULimitStorage();
        $.maxHCUPerTx = maxHCUPerTx;
        $.maxHCUDepthPerTx = maxHCUDepthPerTx;
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
     * @notice Returns HCUConfigurator's address, the only address that changes the settings.
     */
    function getHCUConfiguratorAddress() external pure returns (address) {
        return hcuConfiguratorAdd;
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
     * @notice Returns a quota's maximum usage, its decay per block and its current usage after decay.
     */
    function getQuota(
        uint16 quotaId
    ) public view virtual returns (uint48 maxHCU, uint48 decayPerBlock, uint48 usedHCU) {
        Quota storage quota = _getHCULimitStorage().quotas[quotaId];
        return (
            quota.maxHCU,
            quota.decayPerBlock,
            uint48(_decayed(quota.usedHCU, quota.lastBlock, quota.decayPerBlock))
        );
    }

    /**
     * @notice Returns the chain's capacity, its decay per block and its current load after decay.
     */
    function getCapacity() public view virtual returns (uint48 maxHCU, uint48 decayPerBlock, uint48 usedHCU) {
        Capacity storage capacity = _getHCULimitStorage().capacity;
        return (
            capacity.maxHCU,
            capacity.decayPerBlock,
            uint48(_decayed(capacity.usedHCU, capacity.lastBlock, capacity.decayPerBlock))
        );
    }

    /**
     * @notice Returns a tier's load threshold, in basis points of the capacity's maximum; 0 when stopped or never set.
     */
    function getTier(uint8 tier) public view virtual returns (uint16) {
        return _getHCULimitStorage().tierMaxLoadBps[tier];
    }

    /**
     * @notice Returns the tier of a quota.
     */
    function getQuotaTier(uint16 quotaId) public view virtual returns (uint8) {
        return _getHCULimitStorage().quotas[quotaId].tier;
    }

    /**
     * @notice Returns whether a quota is paused.
     */
    function isQuotaPaused(uint16 quotaId) public view virtual returns (bool) {
        return _getHCULimitStorage().quotas[quotaId].paused;
    }

    /**
     * @notice Returns the quota a contract computes in; 0 is the default quota. An allowlisted contract nobody assigned
     *         is in the quota that replaced the allowlist for it.
     */
    function getContractQuota(address account) public view virtual returns (uint16) {
        return _quotaOf(account);
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
}
