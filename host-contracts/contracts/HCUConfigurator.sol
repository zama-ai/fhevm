// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {aclAdd, hcuLimitAdd} from "../addresses/FHEVMHostAddresses.sol";
import {ACL} from "./ACL.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {HCULimit} from "./HCULimit.sol";
import {BLOCK_WHITELIST_QUOTA, DEFAULT_QUOTA, MAX_BPS, MAX_TIER, NO_CAP, RESERVED_TIER, hcuScaled} from "./shared/Constants.sol";

/**
 * @title HCUConfigurator
 * @notice Changes the settings of HCULimit: who may change each setting, the checks on its value, and its event.
 * @dev HCULimit accepts its settings from this contract only and checks nothing, so the rules below hold only while
 *      this contract is correct. The quota admins live here, since no FHE operation reads them.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract HCUConfigurator is UUPSUpgradeableEmptyProxy, ACLOwnable {
    /// @notice Returned when the sender is neither the ACL owner nor a pauser.
    error NotPauser(address sender);

    /// @notice Returned when the sender is not the admin of the quota.
    error NotQuotaAdmin(uint16 quotaId, address sender);

    /// @notice Returned when a quota admin moves a contract into or out of a paused quota.
    error QuotaMembershipPaused(uint16 quotaId);

    /// @notice Returned when the contract is not in the quota it is moved out of.
    error ContractNotInQuota(address account, uint16 quotaId);

    /// @notice Returned when a quota admin adds a contract while the default quota's maximum is above 0: the
    ///         contract may then have more room in the default quota than in the admin's quota.
    error DefaultQuotaNotStopped();

    /// @notice Returned when:
    ///         - only one of a quota's maximum and decay is 0;
    ///         - a contract is assigned or granted to a quota whose maximum is 0, other than the default quota;
    ///         - an admin is set for the default quota.
    error InvalidQuota(uint16 quotaId);

    /// @notice Returned when only one of the capacity's maximum and decay is 0, or when the capacity's maximum at the
    ///         operator's minimum scale would be below maxHCUPerTx, unless the capacity is unmetered or stopped.
    error InvalidCapacity();

    /// @notice Returned when a capacity operator's minimum scale is 0 or above 10,000.
    error InvalidMinScale(uint16 minScaleBps);

    /// @notice Returned when `adjustCapacity` is not called by the capacity operator.
    error NotCapacityOperator(address sender);

    /// @notice Returned when the capacity operator's step is refused: the capacity is unmetered, the scale is below the
    ///         minimum or above 10,000, the step is too large, or it comes too soon, `retryAfterBlocks` before the next.
    error CapacityStepRefused(uint16 scaleBps, uint256 retryAfterBlocks);

    /// @notice Returned when a threshold is set on Reserved or above MAX_TIER, or would increase with the tier number,
    ///         or when a quota is moved to a tier whose threshold is 0.
    error InvalidTier(uint8 tier);

    /// @notice Returned when only one of the cross-transaction depth limit and its decay is zero.
    error InvalidCrossTxDepthLimit();

    /// @notice Returned if a nonzero cross-transaction depth limit is below maxHCUDepthPerTx.
    error CrossTxDepthBelowMaxDepthPerTx();

    /// @notice Emitted when the cross-transaction depth limit is updated.
    event CrossTxDepthLimitSet(uint48 maxDepth, uint48 decayPerBlock);

    /// @notice Emitted when a quota's maximum and decay are set.
    event QuotaSet(uint16 indexed quotaId, uint48 maxHCU, uint48 decayPerBlock, address indexed setBy);

    /// @notice Emitted when the owner names a quota's admin; address(0) removes it.
    event QuotaAdminSet(uint16 indexed quotaId, address indexed admin);

    /// @notice Emitted when a quota is paused or unpaused.
    event QuotaPauseSet(uint16 indexed quotaId, bool paused, address indexed setBy);

    /// @notice Emitted when the chain's capacity is set.
    event CapacitySet(uint48 maxHCU, uint48 decayPerBlock, address indexed setBy);

    /// @notice Emitted when a tier's load threshold is set.
    event TierSet(uint8 indexed tier, uint16 maxLoadBps, address indexed setBy);

    /// @notice Emitted when a quota is moved to a tier.
    event QuotaTierSet(uint16 indexed quotaId, uint8 indexed tier, address indexed setBy);

    /// @notice Emitted when a contract is assigned to a quota, by the owner or the quota's admin.
    event ContractQuotaAssigned(uint16 indexed quotaId, address indexed account, address indexed setBy);

    /// @notice Emitted when the per-transaction HCU depth limit is updated.
    /// @param maxHCUDepthPerTx New depth limit.
    event MaxHCUDepthPerTxSet(uint48 maxHCUDepthPerTx);

    /// @notice Emitted when the per-transaction HCU limit is updated.
    /// @param maxHCUPerTx New transaction limit.
    event MaxHCUPerTxSet(uint48 maxHCUPerTx);

    /// @notice Emitted when the capacity scale changes: a step by the capacity operator, or the owner's reset to 10,000.
    event CapacityScaleSet(uint16 scaleBps, address indexed setBy);

    /// @notice Emitted when the owner names, changes or removes the capacity operator.
    event CapacityOperatorSet(address indexed operator, uint16 minScaleBps, uint16 maxStepBps, uint32 stepBlocks);

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "HCUConfigurator";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 1;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 2;

    /// @notice The contract whose settings this contract changes.
    HCULimit private constant HCU_LIMIT = HCULimit(hcuLimitAdd);

    /// @dev The address that scales the capacity between the owner's minimum and 10,000 basis points, one bounded step
    ///      at a time, and when it last stepped; 0 lets it step at once.
    struct CapacityOperator {
        address account;
        uint16 minScaleBps;
        uint16 maxStepBps;
        uint32 stepBlocks;
        uint48 lastStepBlock;
    }

    /// @custom:storage-location erc7201:fhevm.storage.HCUConfigurator
    struct HCUConfiguratorStorage {
        /// @notice Address that moves contracts between the default quota and each quota; address(0) when none.
        mapping(uint16 quotaId => address admin) quotaAdmins;
        /// @notice The capacity operator; account is address(0) when none.
        CapacityOperator capacityOperator;
    }

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.HCUConfigurator")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant HCU_CONFIGURATOR_STORAGE_LOCATION =
        0xb138efae62c1e8fe7282cda9cd237607a81b190b364e4655cc3a97b4b08ddc00;

    function _getHCUConfiguratorStorage() internal pure returns (HCUConfiguratorStorage storage $) {
        assembly {
            $.slot := HCU_CONFIGURATOR_STORAGE_LOCATION
        }
    }

    /// @dev Pausers are the ACL's pausers (PauserSet).
    modifier onlyACLOwnerOrPauser() {
        if (msg.sender != ACL(aclAdd).owner() && !ACL(aclAdd).isPauser(msg.sender)) revert NotPauser(msg.sender);
        _;
    }

    /// @dev A paused quota's membership is frozen for its admin; the owner can still move contracts with assignQuota.
    modifier onlyQuotaAdmin(uint16 quotaId) {
        if (msg.sender != _getHCUConfiguratorStorage().quotaAdmins[quotaId]) revert NotQuotaAdmin(quotaId, msg.sender);
        if (HCU_LIMIT.isQuotaPaused(quotaId)) revert QuotaMembershipPaused(quotaId);
        _;
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract. It starts with no quota admin.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice Sets a quota's maximum usage and how much usage decays per block, like setCrossTxDepthLimit.
     *         A maximum of type(uint48).max leaves the quota unmetered; (0, 0) stops it.
     */
    function setQuota(uint16 quotaId, uint48 maxHCU, uint48 decayPerBlock) external onlyACLOwner {
        // Both 0 stops the quota. A maximum without decay would never free up, and a decay without maximum means nothing.
        if ((maxHCU == 0) != (decayPerBlock == 0)) revert InvalidQuota(quotaId);
        HCU_LIMIT.setQuota(quotaId, maxHCU, decayPerBlock);
        emit QuotaSet(quotaId, maxHCU, decayPerBlock, msg.sender);
    }

    /**
     * @notice Sets the HCU load the coprocessors absorb on this chain and how much they drain per block. Every
     *         operation charges this load, whatever its quota. A maximum of type(uint48).max leaves it unmetered and
     *         resets the capacity scale to 10,000; (0, 0) stops every operation.
     * @dev The operator cannot step while the capacity is unmetered, so metering resumes at the owner's capacity.
     */
    function setCapacity(uint48 maxHCU, uint48 decayPerBlock) external onlyACLOwner {
        // Same rule as a quota: both 0 stops every operation.
        if ((maxHCU == 0) != (decayPerBlock == 0)) revert InvalidCapacity();
        _checkReservedFitsATransaction(maxHCU, _minScaleBps(), HCU_LIMIT.getMaxHCUPerTx());
        HCU_LIMIT.setCapacity(maxHCU, decayPerBlock);
        emit CapacitySet(maxHCU, decayPerBlock, msg.sender);
        if (maxHCU == NO_CAP) {
            HCU_LIMIT.setCapacityScale(MAX_BPS);
            emit CapacityScaleSet(MAX_BPS, msg.sender);
        }
    }

    /**
     * @notice Sets the threshold of tier 1 to MAX_TIER, in basis points of the capacity's maximum: the load of this tier
     *         and every tier below it is refused above it. 0 stops the tier. Reserved (tier 0) is fixed at 10,000.
     *         Thresholds apply only while the capacity is metered.
     * @dev Thresholds never increase with the tier number: a threshold is at most the tier above's and at least the tier
     *      below's. So tiers are set from the top down, and stopped from the bottom up.
     */
    function setTier(uint8 tier, uint16 maxLoadBps) external onlyACLOwner {
        if (tier == RESERVED_TIER || tier > MAX_TIER) revert InvalidTier(tier);
        if (maxLoadBps > _maxLoadBps(tier - 1)) revert InvalidTier(tier);
        if (tier < MAX_TIER && maxLoadBps < _maxLoadBps(tier + 1)) revert InvalidTier(tier);
        HCU_LIMIT.setTier(tier, maxLoadBps);
        emit TierSet(tier, maxLoadBps, msg.sender);
    }

    /**
     * @notice Moves a quota to a tier. A tier whose threshold is 0 is refused, so a mistyped tier cannot stop a quota.
     */
    function setQuotaTier(uint16 quotaId, uint8 tier) external onlyACLOwner {
        if (_maxLoadBps(tier) == 0) revert InvalidTier(tier);
        HCU_LIMIT.setQuotaTier(quotaId, tier);
        emit QuotaTierSet(quotaId, tier, msg.sender);
    }

    /**
     * @notice Names the address that moves contracts between the default quota and this quota.
     *         address(0) removes the admin. The default quota and the quotas that replaced the allowlist take none.
     */
    function setQuotaAdmin(uint16 quotaId, address admin) external onlyACLOwner {
        if (quotaId <= BLOCK_WHITELIST_QUOTA) revert InvalidQuota(quotaId);
        _getHCUConfiguratorStorage().quotaAdmins[quotaId] = admin;
        emit QuotaAdminSet(quotaId, admin);
    }

    /**
     * @notice Moves a contract to the default quota or to a quota whose maximum is above 0, from any quota,
     *         including a paused one.
     */
    function assignQuota(uint16 quotaId, address account) external onlyACLOwner {
        if (quotaId != DEFAULT_QUOTA && _maxHCU(quotaId) == 0) revert InvalidQuota(quotaId);
        _assignQuota(quotaId, account);
    }

    /**
     * @notice Moves a contract from the default quota into the admin's quota.
     * @dev Only while the default quota is stopped: a contract then gains room by joining any quota, so an admin
     *      cannot starve a contract it does not own. An allowlisted contract nobody assigned computes in the allowlist
     *      quota, so it is not in the default quota and only the owner moves it.
     */
    function grantQuota(uint16 quotaId, address account) external onlyQuotaAdmin(quotaId) {
        if (_maxHCU(DEFAULT_QUOTA) != 0) revert DefaultQuotaNotStopped();
        if (_maxHCU(quotaId) == 0) revert InvalidQuota(quotaId);
        if (HCU_LIMIT.getContractQuota(account) != DEFAULT_QUOTA) revert ContractNotInQuota(account, DEFAULT_QUOTA);
        _assignQuota(quotaId, account);
    }

    /**
     * @notice Moves a contract from the admin's quota back to the default quota.
     * @dev Only quotas above the allowlist's take an admin, and a contract is in one of those only when assigned to it.
     */
    function revokeQuota(uint16 quotaId, address account) external onlyQuotaAdmin(quotaId) {
        // getContractQuota differs from the assigned quota only by routing to quotas 1 and 2, which take no admin.
        if (HCU_LIMIT.getContractQuota(account) != quotaId) revert ContractNotInQuota(account, quotaId);
        _assignQuota(DEFAULT_QUOTA, account);
    }

    /**
     * @notice Stops every FHE operation from the quota's contracts and stops its admin from moving contracts,
     *         keeping its maximum, decay and contracts. Only the owner unpauses.
     */
    function pauseQuota(uint16 quotaId) external onlyACLOwnerOrPauser {
        _setQuotaPaused(quotaId, true);
    }

    /**
     * @notice Lifts a quota's pause.
     */
    function unpauseQuota(uint16 quotaId) external onlyACLOwner {
        _setQuotaPaused(quotaId, false);
    }

    /**
     * @notice Sets the per-transaction HCU depth limit.
     * @param maxHCUDepthPerTx New depth limit.
     * @dev Enforces a nonzero maxCrossTxDepth >= maxHCUDepthPerTx. It is not compared with maxHCUPerTx, which counts
     *      work while depth counts latency.
     */
    function setMaxHCUDepthPerTx(uint48 maxHCUDepthPerTx) external onlyACLOwner {
        (uint48 maxCrossTxDepth, ) = HCU_LIMIT.getCrossTxDepthLimit();
        if (maxCrossTxDepth != 0 && maxCrossTxDepth < maxHCUDepthPerTx) revert CrossTxDepthBelowMaxDepthPerTx();
        HCU_LIMIT.setTxLimits(HCU_LIMIT.getMaxHCUPerTx(), maxHCUDepthPerTx);
        emit MaxHCUDepthPerTxSet(maxHCUDepthPerTx);
    }

    /**
     * @notice Sets the per-transaction HCU limit.
     * @param maxHCUPerTx New transaction limit.
     */
    function setMaxHCUPerTx(uint48 maxHCUPerTx) external onlyACLOwner {
        (uint48 capacityMax, , , ) = HCU_LIMIT.getCapacity();
        _checkReservedFitsATransaction(capacityMax, _minScaleBps(), maxHCUPerTx);
        HCU_LIMIT.setTxLimits(maxHCUPerTx, HCU_LIMIT.getMaxHCUDepthPerTx());
        emit MaxHCUPerTxSet(maxHCUPerTx);
    }

    /**
     * @notice Sets the cross-transaction depth limit and how much carried depth decays per block.
     * @dev (0, 0) disables it. Decay must be nonzero when enabled, or a chain that reaches the limit stays
     *      blocked forever. The limit must be at least maxHCUDepthPerTx, so a transaction whose inputs carry no
     *      depth is never refused. Permissions granted while disabled carry no depth.
     */
    function setCrossTxDepthLimit(uint48 maxDepth, uint48 decayPerBlock) external onlyACLOwner {
        if ((maxDepth == 0) != (decayPerBlock == 0)) revert InvalidCrossTxDepthLimit();
        if (maxDepth != 0) {
            if (maxDepth < HCU_LIMIT.getMaxHCUDepthPerTx()) revert CrossTxDepthBelowMaxDepthPerTx();
            // Every operation on a stored handle reads ACL.getStoredDepth: refuse to enable before ACL has it.
            ACL(aclAdd).getStoredDepth(bytes32(0), address(0));
        }
        HCU_LIMIT.setCrossTxDepthLimit(maxDepth, decayPerBlock);
        emit CrossTxDepthLimitSet(maxDepth, decayPerBlock);
    }

    /**
     * @notice Names the capacity operator, or removes it with address(0), which ignores the other arguments. Either way
     *         the scale returns to 10,000 and the operator may step at once.
     * @param minScaleBps Lowest scale the operator can set, above 0 and at most 10,000. The capacity's maximum at this
     *        scale must stay at least maxHCUPerTx, so that Reserved can always run a full transaction.
     * @param maxStepBps Largest change of the scale per call.
     * @param stepBlocks Blocks between two calls.
     */
    function setCapacityOperator(
        address operator,
        uint16 minScaleBps,
        uint16 maxStepBps,
        uint32 stepBlocks
    ) external onlyACLOwner {
        if (operator == address(0)) {
            delete _getHCUConfiguratorStorage().capacityOperator;
            (minScaleBps, maxStepBps, stepBlocks) = (0, 0, 0);
        } else {
            if (minScaleBps == 0 || minScaleBps > MAX_BPS) revert InvalidMinScale(minScaleBps);
            (uint48 capacityMax, , , ) = HCU_LIMIT.getCapacity();
            _checkReservedFitsATransaction(capacityMax, minScaleBps, HCU_LIMIT.getMaxHCUPerTx());
            _getHCUConfiguratorStorage().capacityOperator = CapacityOperator(
                operator,
                minScaleBps,
                maxStepBps,
                stepBlocks,
                0
            );
        }
        HCU_LIMIT.setCapacityScale(MAX_BPS);
        emit CapacityScaleSet(MAX_BPS, msg.sender);
        emit CapacityOperatorSet(operator, minScaleBps, maxStepBps, stepBlocks);
    }

    /**
     * @notice Sets the capacity scale, which multiplies the owner's capacity, both its maximum and its decay. Only while
     *         the capacity is metered; from the owner's minimum to 10,000 basis points, at most maxStepBps from the
     *         current scale, and stepBlocks after the last step.
     */
    function adjustCapacity(uint16 scaleBps) external {
        CapacityOperator storage capacityOperator = _getHCUConfiguratorStorage().capacityOperator;
        if (msg.sender != capacityOperator.account) revert NotCapacityOperator(msg.sender);
        (uint48 capacityMax, , uint16 currentScaleBps, ) = HCU_LIMIT.getCapacity();
        uint256 step = scaleBps > currentScaleBps ? scaleBps - currentScaleBps : currentScaleBps - scaleBps;
        if (
            capacityMax == NO_CAP ||
            scaleBps < capacityOperator.minScaleBps ||
            scaleBps > MAX_BPS ||
            step > capacityOperator.maxStepBps
        ) revert CapacityStepRefused(scaleBps, 0);
        uint256 lastStepBlock = capacityOperator.lastStepBlock;
        if (lastStepBlock != 0 && block.number < lastStepBlock + capacityOperator.stepBlocks) {
            revert CapacityStepRefused(scaleBps, lastStepBlock + capacityOperator.stepBlocks - block.number);
        }
        capacityOperator.lastStepBlock = uint48(block.number);
        HCU_LIMIT.setCapacityScale(scaleBps);
        emit CapacityScaleSet(scaleBps, msg.sender);
    }

    /**
     * @notice Returns the capacity operator and its bounds; address(0) when there is none.
     */
    function getCapacityOperator()
        external
        view
        returns (address operator, uint16 minScaleBps, uint16 maxStepBps, uint32 stepBlocks)
    {
        CapacityOperator storage capacityOperator = _getHCUConfiguratorStorage().capacityOperator;
        return (
            capacityOperator.account,
            capacityOperator.minScaleBps,
            capacityOperator.maxStepBps,
            capacityOperator.stepBlocks
        );
    }

    /**
     * @notice Returns a quota's admin; address(0) when it has none.
     */
    function getQuotaAdmin(uint16 quotaId) external view returns (address) {
        return _getHCUConfiguratorStorage().quotaAdmins[quotaId];
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

    function _maxHCU(uint16 quotaId) private view returns (uint48 maxHCU) {
        (maxHCU, , ) = HCU_LIMIT.getQuota(quotaId);
    }

    /// @dev The lowest scale the capacity can reach: the operator's minimum, or 10,000 without an operator.
    function _minScaleBps() private view returns (uint16) {
        CapacityOperator storage capacityOperator = _getHCUConfiguratorStorage().capacityOperator;
        return capacityOperator.account == address(0) ? MAX_BPS : capacityOperator.minScaleBps;
    }

    /// @dev Reserved can always run a full transaction: the capacity's maximum at the lowest scale is at least
    ///      maxHCUPerTx. An unmetered or stopped capacity is exempt, so the owner can still stop the chain.
    function _checkReservedFitsATransaction(uint48 capacityMax, uint16 minScaleBps, uint48 maxHCUPerTx) private pure {
        if (capacityMax == NO_CAP || capacityMax == 0) return;
        if (hcuScaled(capacityMax, minScaleBps) < maxHCUPerTx) revert InvalidCapacity();
    }

    function _maxLoadBps(uint8 tier) private view returns (uint16 maxLoadBps) {
        (maxLoadBps, ) = HCU_LIMIT.getTier(tier);
    }

    function _assignQuota(uint16 quotaId, address account) private {
        HCU_LIMIT.assignQuota(quotaId, account);
        emit ContractQuotaAssigned(quotaId, account, msg.sender);
    }

    function _setQuotaPaused(uint16 quotaId, bool paused) private {
        HCU_LIMIT.setQuotaPaused(quotaId, paused);
        emit QuotaPauseSet(quotaId, paused, msg.sender);
    }

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
