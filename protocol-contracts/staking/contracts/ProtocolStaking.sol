// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {AccessControlDefaultAdminRulesUpgradeable} from "@openzeppelin/contracts-upgradeable/access/extensions/AccessControlDefaultAdminRulesUpgradeable.sol";
import {ERC20VotesUpgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20VotesUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {Checkpoints} from "@openzeppelin/contracts/utils/structs/Checkpoints.sol";
import {Time} from "@openzeppelin/contracts/utils/types/Time.sol";

interface IERC20Mintable is IERC20 {
    function mint(address to, uint256 amount) external;
}

contract ProtocolStaking is AccessControlDefaultAdminRulesUpgradeable, ERC20VotesUpgradeable, UUPSUpgradeable {
    using Checkpoints for Checkpoints.Trace208;
    using SafeERC20 for IERC20;
    using Math for uint256;

    /// @custom:storage-location erc7201:zama.storage.ProtocolStaking
    struct ProtocolStakingStorage {
        // Stake - general
        address _stakingToken;
        uint256 _totalEligibleStakedWeight;
        // Stake - release
        uint256 _unstakeCooldownPeriod;
        mapping(address => Checkpoints.Trace208) _unstakeRequests;
        mapping(address => uint256) _released;
        // Reward - issuance curve
        uint256 _lastUpdateTimestamp;
        uint256 _lastUpdateReward;
        uint256 _rewardRate;
        // Reward - recipient
        mapping(address => address) _rewardsRecipient;
        // Reward - payment tracking
        mapping(address => int256) _paid;
        int256 _totalVirtualPaid;
    }

    // keccak256(abi.encode(uint256(keccak256("zama.storage.ProtocolStaking")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PROTOCOL_STAKING_STORAGE_LOCATION =
        0x6867237db38693700f305f18dff1dbf600e282237f7d452b4c792e6b019c6b00;
    bytes32 private constant ELIGIBLE_ACCOUNT_ROLE = keccak256("eligible-account-role");

    event TokensStaked(address indexed account, uint256 amount);
    event TokensUnstaked(address indexed account, address indexed recipient, uint256 amount);
    event RewardRateSet(uint256 rewardRate);
    event UnstakeCooldownPeriodSet(uint256 unstakeCooldownPeriod);
    event RewardsRecipientSet(address indexed account, address indexed recipient);

    error InvalidAmount();
    error EligibleAccountAlreadyExists(address account);
    error EligibleAccountDoesNotExist(address account);
    error TransferDisabled();
    error InvalidUnstakeCooldownPeriod();

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        string memory name,
        string memory symbol,
        string memory version,
        address stakingToken_,
        address governor,
        uint256 initialUnstakeCooldownPeriod
    ) public virtual initializer {
        __AccessControlDefaultAdminRules_init(0, governor);
        __ERC20_init(name, symbol);
        __EIP712_init(name, version);
        _getProtocolStakingStorage()._stakingToken = stakingToken_;
        _setUnstakeCooldownPeriod(initialUnstakeCooldownPeriod);
    }

    /// @dev Stake `amount` tokens from `msg.sender`.
    function stake(uint256 amount) public virtual {
        _mint(msg.sender, amount);
        IERC20(stakingToken()).safeTransferFrom(msg.sender, address(this), amount);

        emit TokensStaked(msg.sender, amount);
    }

    /**
     * @dev Unstake `amount` tokens from `msg.sender`'s staked balance to `recipient`.
     *
     * NOTE: Unstaked tokens will not be sent immediately if {unstakeCooldownPeriod} is non-zero.
     */
    function unstake(address recipient, uint256 amount) public virtual {
        _burn(msg.sender, amount);

        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        (, uint256 lastReleaseTime, uint256 totalRequestedToWithdraw) = $
            ._unstakeRequests[recipient]
            .latestCheckpoint();
        uint256 releaseTime = Time.timestamp() + $._unstakeCooldownPeriod;
        $._unstakeRequests[recipient].push(
            uint48(Math.max(releaseTime, lastReleaseTime)),
            uint208(totalRequestedToWithdraw + amount)
        );

        emit TokensUnstaked(msg.sender, recipient, amount);
    }

    /**
     * @dev Releases tokens requested for unstaking after the cooldown period to `account`.
     *
     * WARNING: If this contract is upgraded to add slashing, the ability to withdraw to a
     * different address should be reconsidered.
     */
    function release(address account) public virtual {
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        uint256 totalAmountCooledDown = $._unstakeRequests[account].upperLookup(Time.timestamp());
        uint256 amountToRelease = totalAmountCooledDown - $._released[account];
        if (amountToRelease > 0) {
            $._released[account] = totalAmountCooledDown;
            IERC20(stakingToken()).safeTransfer(account, amountToRelease);
        }
    }

    /// @dev Claim staking rewards for `account`. Can be called by anyone.
    function claimRewards(address account) public virtual {
        uint256 rewards = earned(account);
        if (rewards > 0) {
            _getProtocolStakingStorage()._paid[account] += SafeCast.toInt256(rewards);
            IERC20Mintable(stakingToken()).mint(rewardsRecipient(account), rewards);
        }
    }

    /// @dev Sets the reward rate in tokens per second. Only callable by {owner}.
    function setRewardRate(uint256 rewardRate) public virtual onlyRole(DEFAULT_ADMIN_ROLE) {
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        $._lastUpdateReward = _historicalReward();
        $._lastUpdateTimestamp = Time.timestamp();
        $._rewardRate = rewardRate;

        emit RewardRateSet(rewardRate);
    }

    /**
     * @dev Adds the eligible account role to `account`. Only accounts with the eligible account role earn rewards for staked tokens.
     * Only callable by the `ELIGIBLE_ACCOUNT_ROLE` role admin (by default {owner}).
     */
    function addEligibleAccount(address account) public virtual onlyRole(getRoleAdmin(ELIGIBLE_ACCOUNT_ROLE)) {
        require(_grantRole(ELIGIBLE_ACCOUNT_ROLE, account), EligibleAccountAlreadyExists(account));
    }

    /**
     * @dev Removes the eligible account role from `account`. `account` stops to earn rewards but maintains all existing rewards.
     * Only callable by the `ELIGIBLE_ACCOUNT_ROLE` role admin (by default {owner}).
     */
    function removeEligibleAccount(address account) public virtual onlyRole(getRoleAdmin(ELIGIBLE_ACCOUNT_ROLE)) {
        require(_revokeRole(ELIGIBLE_ACCOUNT_ROLE, account), EligibleAccountDoesNotExist(account));
    }

    /// @dev Sets the {unstake} cooldown period in seconds to `unstakeCooldownPeriod`. Only callable by {owner}.
    function setUnstakeCooldownPeriod(uint256 unstakeCooldownPeriod_) public virtual onlyRole(DEFAULT_ADMIN_ROLE) {
        _setUnstakeCooldownPeriod(unstakeCooldownPeriod_);
    }

    /// @dev Sets the reward recipient for `msg.sender` to `recipient`. All future rewards for `msg.sender` will be sent to `recipient`.
    function setRewardsRecipient(address recipient) public virtual {
        _getProtocolStakingStorage()._rewardsRecipient[msg.sender] = recipient;

        emit RewardsRecipientSet(msg.sender, recipient);
    }

    /// @dev Returns the amount of rewards earned by `account` at the current `block.timestamp`.
    function earned(address account) public view virtual returns (uint256) {
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        uint256 stakedWeight = isEligibleAccount(account) ? weight(balanceOf(account)) : 0;
        // if stakedWeight == 0, there is a risk of totalStakedWeight == 0. To avoid div by 0 just return 0
        uint256 allocation = stakedWeight > 0 ? _allocation(stakedWeight, $._totalEligibleStakedWeight) : 0;
        return SafeCast.toUint256(SafeCast.toInt256(allocation) - $._paid[account]);
    }

    /// @dev Returns the staking token which is used for staking and rewards.
    function stakingToken() public view virtual returns (address) {
        return _getProtocolStakingStorage()._stakingToken;
    }

    /// @dev Gets the staking weight for a given raw amount.
    function weight(uint256 amount) public view virtual returns (uint256) {
        return Math.sqrt(amount);
    }

    /// @dev Returns the current total staked weight.
    function totalStakedWeight() public view virtual returns (uint256) {
        return _getProtocolStakingStorage()._totalEligibleStakedWeight;
    }

    /// @dev Returns the current unstake cooldown period in seconds.
    function unstakeCooldownPeriod() public view virtual returns (uint256) {
        return _getProtocolStakingStorage()._unstakeCooldownPeriod;
    }

    /// @dev Returns the amount of tokens cooling down for the given account `account`.
    function tokensInCooldown(address account) public view virtual returns (uint256) {
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        return $._unstakeRequests[account].latest() - $._released[account];
    }

    /// @dev Returns the recipient for rewards earned by `account`.
    function rewardsRecipient(address account) public view virtual returns (address) {
        address storedRewardsRecipient = _getProtocolStakingStorage()._rewardsRecipient[account];
        return storedRewardsRecipient == address(0) ? account : storedRewardsRecipient;
    }

    /// @dev Indicates if the given account `account` has the eligible account role.
    function isEligibleAccount(address account) public view virtual returns (bool) {
        return hasRole(ELIGIBLE_ACCOUNT_ROLE, account);
    }

    function _grantRole(bytes32 role, address account) internal virtual override returns (bool) {
        bool success = super._grantRole(role, account);
        if (role == ELIGIBLE_ACCOUNT_ROLE && success) {
            _updateRewards(account, 0, weight(balanceOf(account)));
        }
        return success;
    }

    function _revokeRole(bytes32 role, address account) internal virtual override returns (bool) {
        bool success = super._revokeRole(role, account);
        if (role == ELIGIBLE_ACCOUNT_ROLE && success) {
            _updateRewards(account, weight(balanceOf(account)), 0);
        }
        return success;
    }

    function _setUnstakeCooldownPeriod(uint256 unstakeCooldownPeriod_) internal virtual {
        if (unstakeCooldownPeriod_ == 0) revert InvalidUnstakeCooldownPeriod();
        _getProtocolStakingStorage()._unstakeCooldownPeriod = unstakeCooldownPeriod_;

        emit UnstakeCooldownPeriodSet(unstakeCooldownPeriod_);
    }

    function _updateRewards(address user, uint256 weightBefore, uint256 weightAfter) internal {
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        uint256 oldTotalWeight = $._totalEligibleStakedWeight;
        $._totalEligibleStakedWeight = oldTotalWeight - weightBefore + weightAfter;

        if (weightBefore != weightAfter && oldTotalWeight > 0) {
            if (weightBefore > weightAfter) {
                int256 virtualAmount = SafeCast.toInt256(_allocation(weightBefore - weightAfter, oldTotalWeight));
                $._paid[user] -= virtualAmount;
                $._totalVirtualPaid -= virtualAmount;
            } else {
                int256 virtualAmount = SafeCast.toInt256(_allocation(weightAfter - weightBefore, oldTotalWeight));
                $._paid[user] += virtualAmount;
                $._totalVirtualPaid += virtualAmount;
            }
        }
    }

    function _update(address from, address to, uint256 value) internal virtual override {
        // Disable Transfers
        require(from == address(0) || to == address(0), TransferDisabled());
        if (isEligibleAccount(from)) {
            uint256 balanceBefore = balanceOf(from);
            uint256 balanceAfter = balanceBefore - value;
            _updateRewards(from, weight(balanceBefore), weight(balanceAfter));
        }
        if (isEligibleAccount(to)) {
            uint256 balanceBefore = balanceOf(to);
            uint256 balanceAfter = balanceBefore + value;
            _updateRewards(to, weight(balanceBefore), weight(balanceAfter));
        }
        super._update(from, to, value);
    }

    function _authorizeUpgrade(address newImplementation) internal virtual override onlyRole(DEFAULT_ADMIN_ROLE) {}

    function _historicalReward() internal view virtual returns (uint256) {
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        return $._lastUpdateReward + (Time.timestamp() - $._lastUpdateTimestamp) * $._rewardRate;
    }

    function _allocation(uint256 share, uint256 total) private view returns (uint256) {
        return
            SafeCast
                .toUint256(SafeCast.toInt256(_historicalReward()) + _getProtocolStakingStorage()._totalVirtualPaid)
                .mulDiv(share, total);
    }

    function _getProtocolStakingStorage() private pure returns (ProtocolStakingStorage storage $) {
        assembly {
            $.slot := PROTOCOL_STAKING_STORAGE_LOCATION
        }
    }
}
