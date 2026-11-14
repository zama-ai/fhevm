// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {aclAdd, hcuConfiguratorAdd} from "../addresses/FHEVMHostAddresses.sol";
import {ACL} from "./ACL.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {HCUConfigurator} from "./HCUConfigurator.sol";
import {HCULimitOperators} from "./HCULimitOperators.sol";
import {ALLOWLIST_QUOTA, BLOCK_WHITELIST_QUOTA, DEFAULT_QUOTA, NO_CAP} from "./shared/Constants.sol";

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

    /// @notice Returned when the contract's quota has a maximum of 0. Retrying fails until the contract is moved
    ///         or the owner resizes the quota.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    error HCUQuotaStopped(address account, uint16 quotaId);

    /// @notice Returned when the contract's quota is paused. Retrying fails until the owner unpauses it.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    error HCUQuotaPaused(address account, uint16 quotaId);

    /// @notice Returned when a contract grants a handle deeper than its quota's stored-depth limit, with allow or
    ///         allowTransient. Retrying succeeds once the handle has decayed for `retryAfterBlocks`.
    /// @param account Contract that grants the handle.
    /// @param quotaId Quota the contract computes in.
    error HCUStoredDepthExceeded(address account, uint16 quotaId, uint256 retryAfterBlocks);

    /// @notice Returned when a setting is written by an address other than HCUConfigurator.
    error CallerMustBeHCUConfigurator();

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

    /// @dev Limit, meter, pause flag and stored-depth limit of one quota, in one slot. All contracts assigned to the
    ///      quota share it. Usage decays by decayPerBlock every block, as carried depth does, and an operation fits
    ///      while usage stays at most maxHCU. maxStoredDepth caps the carried depth its contracts can grant; 0 leaves
    ///      only maxCrossTxDepth.
    struct Quota {
        uint48 maxHCU;
        uint48 decayPerBlock;
        uint48 usedHCU;
        uint48 lastBlock;
        bool paused;
        uint48 maxStoredDepth;
    }

    /// @custom:storage-location erc7201:fhevm.storage.HCULimit
    /// @dev The first five uint48 fields pack into a single 256-bit slot (5 × 48 = 240 bits). The
    ///      cross-transaction fields and the quotas are appended after the allowlist to keep the existing layout.
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
        /// @notice Contracts the allowlist admitted. No setter since quotas replaced it; an entry is deleted when its
        ///         contract moves to the default quota.
        mapping(address account => bool allowed) isAllowlisted;
        /// @notice Unused since quota admins replaced allowlisters.
        mapping(address account => uint256 deadline) allowlisters;
        /// @notice Whether the allowlist restricted FHE computation when quotas replaced it. No setter; while true and
        ///         the default quota is stopped, allowlisted contracts nobody assigned keep computing (see _quotaOf).
        bool isAllowlistActivated;
        /// @notice Maximum depth carried across transactions; 0 disables the check.
        uint48 maxCrossTxDepth;
        /// @notice Carried depth the coprocessors are assumed to drain per block.
        uint48 crossTxDepthDecayPerBlock;
        /// @notice Limit and meter of each quota.
        mapping(uint16 quotaId => Quota) quotas;
        /// @notice Quota of each contract; the default quota when never assigned.
        mapping(address account => uint16 quotaId) contractQuota;
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
     * @param defaultQuotaHCUPerBlock Maximum and decay per block of the default quota; type(uint48).max leaves it
     *        unmetered.
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
        _setTxLimits(maxHCUPerTx, maxHCUDepthPerTx);
    }

    /**
     * @notice Re-initializes the contract from V4 (the 0.14.x releases) or V5 (the computation allowlist).
     * @dev Quotas replace the global block cap, its whitelist and the allowlist. With the allowlist active, the default
     *      quota is stopped, so unknown contracts stay refused, and allowlisted contracts share the global cap in the
     *      allowlist quota, or are unmetered if also block-whitelisted. Otherwise the default quota gets the global cap,
     *      and the upgrade proposal assigns any block-whitelisted contract to a quota that is not metered. The cap
     *      becomes maximum and decay per block, which allows the same HCU per block. The cross-transaction depth limit
     *      starts disabled.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV6() public virtual reinitializer(REINITIALIZER_VERSION) {
        // HCUConfigurator is the only writer of these settings: refuse the upgrade until it runs.
        HCUConfigurator(hcuConfiguratorAdd).getVersion();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint48 globalCap = $.globalHCUCapPerBlock;
        if ($.isAllowlistActivated) {
            _setQuota(DEFAULT_QUOTA, 0, 0);
            _setQuota(ALLOWLIST_QUOTA, globalCap, globalCap);
            _setQuota(BLOCK_WHITELIST_QUOTA, NO_CAP, NO_CAP);
        } else {
            _setQuota(DEFAULT_QUOTA, globalCap, globalCap);
        }
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

    function setQuotaMaxStoredDepth(uint16 quotaId, uint48 maxStoredDepth) external onlyHCUConfigurator {
        _getHCULimitStorage().quotas[quotaId].maxStoredDepth = maxStoredDepth;
    }

    function setTxLimits(uint48 maxHCUPerTx, uint48 maxHCUDepthPerTx) external onlyHCUConfigurator {
        _setTxLimits(maxHCUPerTx, maxHCUDepthPerTx);
    }

    function setCrossTxDepthLimit(uint48 maxDepth, uint48 decayPerBlock) external onlyHCUConfigurator {
        HCULimitStorage storage $ = _getHCULimitStorage();
        $.maxCrossTxDepth = maxDepth;
        $.crossTxDepthDecayPerBlock = decayPerBlock;
    }

    /**
     * @notice Caches `handle`'s carried depth for the rest of this transaction and returns it, when `holder` grants the
     *         handle. If this transaction has not seen the handle yet, the depth comes from `holder`'s permission.
     *         Reverts with HCUStoredDepthExceeded above the stored-depth limit of `holder`'s quota. Returns 0 when
     *         disabled.
     */
    function cacheDepth(bytes32 handle, address holder) external returns (uint48 depth) {
        if (msg.sender != aclAdd) revert CallerMustBeACLContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.maxCrossTxDepth == 0) return 0;
        depth = uint48(_cacheDepths(handle, holder) >> 128);
        uint16 quotaId = _quotaOf(holder);
        uint48 maxStoredDepth = $.quotas[quotaId].maxStoredDepth;
        if (maxStoredDepth != 0 && depth > maxStoredDepth) {
            revert HCUStoredDepthExceeded(
                holder,
                quotaId,
                _blocksToDecay(depth - maxStoredDepth, $.crossTxDepthDecayPerBlock)
            );
        }
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
    ///      Carried depth and quota usage both decay this way.
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
     * @param opWork The operation's work, charged to the quota and the transaction.
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
     * @notice Charges one operation to the quota of the contract that runs it.
     * @param opWork Work of the current operation.
     * @param caller Contract that calls FHEVMExecutor.
     */
    function _updateAndVerifyHCUQuota(uint256 opWork, address caller) internal virtual {
        uint16 quotaId = _quotaOf(caller);
        Quota storage quota = _getHCULimitStorage().quotas[quotaId];
        if (quota.paused) revert HCUQuotaPaused(caller, quotaId);
        uint48 maxHCU = quota.maxHCU;
        if (maxHCU == 0) revert HCUQuotaStopped(caller, quotaId);
        if (maxHCU == NO_CAP) return;
        uint48 decayPerBlock = quota.decayPerBlock;
        uint256 used = _decayed(quota.usedHCU, quota.lastBlock, decayPerBlock) + opWork;
        if (used > maxHCU) {
            uint256 wait = opWork > maxHCU ? type(uint256).max : _blocksToDecay(used - maxHCU, decayPerBlock);
            revert HCUQuotaExceeded(caller, quotaId, wait);
        }
        quota.usedHCU = uint48(used);
        quota.lastBlock = uint48(block.number);
    }

    /// @dev Returns the quota a contract computes in: the one it is assigned to, else, while the default quota is
    ///      stopped and the allowlist was active, the quota that replaced the allowlist for it. Reopening the default
    ///      quota suspends this routing, so an open default quota costs no extra read; stopping it again resumes it.
    function _quotaOf(address account) private view returns (uint16) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint16 quotaId = $.contractQuota[account];
        if (quotaId != DEFAULT_QUOTA || $.quotas[DEFAULT_QUOTA].maxHCU != 0) return quotaId;
        if (!$.isAllowlistActivated || !$.isAllowlisted[account]) return DEFAULT_QUOTA;
        return $.blockHCUWhitelist[account] ? BLOCK_WHITELIST_QUOTA : ALLOWLIST_QUOTA;
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

    function _setQuota(uint16 quotaId, uint48 maxHCU, uint48 decayPerBlock) private {
        Quota storage quota = _getHCULimitStorage().quotas[quotaId];
        quota.maxHCU = maxHCU;
        quota.decayPerBlock = decayPerBlock;
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
     * @notice Returns the address of HCUConfigurator, the only writer of this contract's settings.
     */
    function getHCUConfiguratorAddress() external pure returns (address) {
        return hcuConfiguratorAdd;
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
     * @notice Returns the carried depth above which a quota's contracts cannot grant a handle; 0 when it has none.
     */
    function getQuotaMaxStoredDepth(uint16 quotaId) public view virtual returns (uint48) {
        return _getHCULimitStorage().quotas[quotaId].maxStoredDepth;
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
}
