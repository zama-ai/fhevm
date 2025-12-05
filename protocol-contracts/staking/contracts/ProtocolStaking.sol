// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {AccessControlDefaultAdminRulesUpgradeable} from "@openzeppelin/contracts-upgradeable/access/extensions/AccessControlDefaultAdminRulesUpgradeable.sol";
import {ERC20VotesUpgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20VotesUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SignedMath} from "@openzeppelin/contracts/utils/math/SignedMath.sol";
import {Checkpoints} from "@openzeppelin/contracts/utils/structs/Checkpoints.sol";
import {Time} from "@openzeppelin/contracts/utils/types/Time.sol";

interface IERC20Mintable is IERC20 {
    function mint(address to, uint256 amount) external;
}

/**
 * @dev Staking contract that distributes newly minted tokens to eligible accounts at a configurable flow rate.
 *
 * NOTE: This staking contract does not support non-standard ERC-20 tokens such as fee-on-transfer or rebasing tokens.
 * @custom:security-contact security@zama.ai
 */
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
        uint48 _unstakeCooldownPeriod;
        mapping(address account => Checkpoints.Trace208) _unstakeRequests;
        mapping(address account => uint256) _released;
        // Reward - issuance curve
        uint256 _lastUpdateTimestamp;
        uint256 _lastUpdateReward;
        uint256 _rewardRate;
        // Reward - recipient
        mapping(address staker => address) _rewardsRecipient;
        // Reward - payment tracking
        mapping(address staker => int256) _paid;
        int256 _totalVirtualPaid;
    }

    // keccak256(abi.encode(uint256(keccak256("zama.storage.ProtocolStaking")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PROTOCOL_STAKING_STORAGE_LOCATION =
        0x6867237db38693700f305f18dff1dbf600e282237f7d452b4c792e6b019c6b00;
    bytes32 private constant UPGRADER_ROLE = keccak256("UPGRADER_ROLE");
    bytes32 private constant MANAGER_ROLE = keccak256("MANAGER_ROLE");
    bytes32 private constant ELIGIBLE_ACCOUNT_ROLE = keccak256("ELIGIBLE_ACCOUNT_ROLE");

    /// @dev Emitted when tokens are staked by an account.
    event TokensStaked(address indexed account, uint256 amount);
    /// @dev Emitted when tokens are unstaked by an account.
    event TokensUnstaked(address indexed account, uint256 amount, uint48 releaseTime);
    /// @dev Emitted when tokens are released to a recipient after the unstaking cooldown period.
    event TokensReleased(address indexed recipient, uint256 amount);
    /// @dev Emitted when rewards of an account are claimed.
    event RewardsClaimed(address indexed account, address indexed recipient, uint256 amount);
    /// @dev Emitted when the reward rate is updated.
    event RewardRateSet(uint256 rewardRate);
    /// @dev Emitted when the unstake cooldown is updated.
    event UnstakeCooldownPeriodSet(uint256 unstakeCooldownPeriod);
    /// @dev Emitted when the reward recipient of an account is updated.
    event RewardsRecipientSet(address indexed account, address indexed recipient);

    /// @dev The account cannot be made eligible.
    error InvalidEligibleAccount(address account);
    /// @dev The tokens cannot be transferred.
    error TransferDisabled();
    /// @dev The unstake cooldown period is invalid.
    error InvalidUnstakeCooldownPeriod();

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @dev Initializes this upgradeable protocol staking contract.
    function initialize(
        string memory name,
        string memory symbol,
        string memory version,
        address stakingToken_,
        address governor,
        address upgrader,
        address manager,
        uint48 initialUnstakeCooldownPeriod,
        uint256 initialRewardRate
    ) public initializer {
        __AccessControlDefaultAdminRules_init(0, governor);
        _grantRole(UPGRADER_ROLE, upgrader);
        _grantRole(MANAGER_ROLE, manager);
        _setRoleAdmin(ELIGIBLE_ACCOUNT_ROLE, MANAGER_ROLE);
        __ERC20_init(name, symbol);
        __EIP712_init(name, version);
        _getProtocolStakingStorage()._stakingToken = stakingToken_;
        _setUnstakeCooldownPeriod(initialUnstakeCooldownPeriod);
        _setRewardRate(initialRewardRate);
    }

    /**
     * @dev Stake `amount` tokens from `msg.sender`.
     * @param amount The amount of tokens to stake.
     */
    function stake(uint256 amount) public {
        _mint(msg.sender, amount);
        IERC20(stakingToken()).safeTransferFrom(msg.sender, address(this), amount);

        emit TokensStaked(msg.sender, amount);
    }

    /**
     * @dev Unstake `amount` tokens from `msg.sender`'s staked balance to `msg.sender`.
     *
     * NOTE: Unstaked tokens are released by calling {release} after {unstakeCooldownPeriod}.
     * WARNING: Unstake release times are strictly increasing per account even if the cooldown period
     * is reduced. For a given account to fully realize the reduction in cooldown period, they may need
     * to wait up to `OLD_COOLDOWN_PERIOD - NEW_COOLDOWN_PERIOD` seconds after the cooldown period is updated.
     *
     * @param amount The amount of tokens to unstake.
     * @return releaseTime The timestamp when the unstaked tokens can be released.
     */
    function unstake(uint256 amount) public returns (uint48) {
        _burn(msg.sender, amount);

        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        (, uint256 lastReleaseTime, uint256 totalRequestedToWithdraw) = $
            ._unstakeRequests[msg.sender]
            .latestCheckpoint();
        uint48 releaseTime = SafeCast.toUint48(Math.max(Time.timestamp() + $._unstakeCooldownPeriod, lastReleaseTime));
        $._unstakeRequests[msg.sender].push(releaseTime, uint208(totalRequestedToWithdraw + amount));

        emit TokensUnstaked(msg.sender, amount, releaseTime);
        return releaseTime;
    }

    /**
     * @dev Releases tokens requested for unstaking after the cooldown period to `account`.
     * @param account The account to release tokens to.
     */
    function release(address account) public virtual {
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        uint256 totalAmountCooledDown = $._unstakeRequests[account].upperLookup(Time.timestamp());
        uint256 amountToRelease = totalAmountCooledDown - $._released[account];
        if (amountToRelease > 0) {
            $._released[account] = totalAmountCooledDown;
            IERC20(stakingToken()).safeTransfer(account, amountToRelease);
            emit TokensReleased(account, amountToRelease);
        }
    }

    /**
     * @dev Claim staking rewards for `account`. Can be called by anyone.
     * @param account The account to claim rewards for.
     */
    function claimRewards(address account) public {
        uint256 rewards = earned(account);
        if (rewards > 0) {
            _getProtocolStakingStorage()._paid[account] += SafeCast.toInt256(rewards);
            address recipient = rewardsRecipient(account);
            IERC20Mintable(stakingToken()).mint(recipient, rewards);
            emit RewardsClaimed(account, recipient, rewards);
        }
    }

    /**
     * @dev Sets the reward rate in tokens per second. Only callable by `MANAGER_ROLE` role.
     * @param rewardRate_ The new reward rate in tokens per second.
     */
    function setRewardRate(uint256 rewardRate_) public onlyRole(MANAGER_ROLE) {
        _setRewardRate(rewardRate_);
    }

    /**
     * @dev Adds the eligible account role to `account`. Only accounts with the eligible account
     * role earn rewards for staked tokens. Only callable by the role admin for
     * `ELIGIBLE_ACCOUNT_ROLE`. By default this is `MANAGER_ROLE`.
     * @param account The account to grant the `ELIGIBLE_ACCOUNT_ROLE` role to.
     */
    function addEligibleAccount(address account) public {
        grantRole(ELIGIBLE_ACCOUNT_ROLE, account);
    }

    /**
     * @dev Removes the eligible account role from `account`. `account` stops to earn rewards
     * but maintains all existing rewards. Only callable by the role admin for
     * `ELIGIBLE_ACCOUNT_ROLE`. By default this is `MANAGER_ROLE`.
     * @param account The account to revoke the `ELIGIBLE_ACCOUNT_ROLE` role from.
     */
    function removeEligibleAccount(address account) public {
        revokeRole(ELIGIBLE_ACCOUNT_ROLE, account);
    }

    /**
     * @dev Sets the {unstake} cooldown period in seconds to `unstakeCooldownPeriod`. Only callable
     * by `MANAGER_ROLE` role. See {unstake} for important notes regarding the cooldown period.
     * @param unstakeCooldownPeriod_ The new unstake cooldown period.
     */
    function setUnstakeCooldownPeriod(uint48 unstakeCooldownPeriod_) public onlyRole(MANAGER_ROLE) {
        _setUnstakeCooldownPeriod(unstakeCooldownPeriod_);
    }

    /**
     * @dev Sets the reward recipient for `msg.sender` to `recipient`. All future rewards for
     * `msg.sender` will be sent to `recipient`.
     * @param recipient The recipient that will receive rewards on behalf of `msg.sender` for all future  {claimRewards} calls.
     * A value of `address(0)` indicates that rewards should be sent to `msg.sender`.
     */
    function setRewardsRecipient(address recipient) public {
        _getProtocolStakingStorage()._rewardsRecipient[msg.sender] = recipient;

        emit RewardsRecipientSet(msg.sender, recipient);
    }

    /**
     * @dev Gets the amount of rewards earned by `account` at the current `block.timestamp`.
     * @param account The account to check rewards for.
     * @return The earned amount.
     */
    function earned(address account) public view returns (uint256) {
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        uint256 stakedWeight = isEligibleAccount(account) ? weight(balanceOf(account)) : 0;
        // if stakedWeight == 0, there is a risk of totalStakedWeight == 0. To avoid div by 0 just return 0
        uint256 allocation = stakedWeight > 0 ? _allocation(stakedWeight, $._totalEligibleStakedWeight) : 0;
        // Accounting rounding may have a marginal impact on earned rewards (dust).
        return SafeCast.toUint256(SignedMath.max(0, SafeCast.toInt256(allocation) - $._paid[account]));
    }

    /// @dev Returns the staking token which is used for staking and rewards.
    function stakingToken() public view returns (address) {
        return _getProtocolStakingStorage()._stakingToken;
    }

    /**
     * @dev Gets the staking weight for a given amount of tokens.
     * @param amount The amount being weighted.
     * @return The staking weight.
     */
    function weight(uint256 amount) public pure returns (uint256) {
        return Math.sqrt(amount);
    }

    /// @dev Returns the current total staked weight.
    function totalStakedWeight() public view returns (uint256) {
        return _getProtocolStakingStorage()._totalEligibleStakedWeight;
    }

    /// @dev Returns the current unstake cooldown period in seconds.
    function unstakeCooldownPeriod() public view returns (uint256) {
        return _getProtocolStakingStorage()._unstakeCooldownPeriod;
    }

    /**
     * @dev Gets the amount of tokens that have been unstaked but not released yet
     * for a given account `account`.
     * @param account The account having tokens cooling down.
     * @return The releasable amount of tokens after the cooldown period.
     */
    function awaitingRelease(address account) public view virtual returns (uint256) {
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        return $._unstakeRequests[account].latest() - $._released[account];
    }

    /**
     * @dev Gets the current protocol reward rate in tokens distributed per second.
     * @return The reward rate.
     */
    function rewardRate() public view returns (uint256) {
        return _getProtocolStakingStorage()._rewardRate;
    }

    /**
     * @dev Gets the recipient for rewards earned by `account`.
     * @param account The account that earned rewards.
     * @return The rewards recipient.
     */
    function rewardsRecipient(address account) public view returns (address) {
        address storedRewardsRecipient = _getProtocolStakingStorage()._rewardsRecipient[account];
        return storedRewardsRecipient == address(0) ? account : storedRewardsRecipient;
    }

    /**
     * @dev Indicates if the given account `account` has the eligible account role.
     * @param account The account being checked for eligibility.
     * @return True if eligible.
     */
    function isEligibleAccount(address account) public view returns (bool) {
        return hasRole(ELIGIBLE_ACCOUNT_ROLE, account);
    }

    function _grantRole(bytes32 role, address account) internal override returns (bool) {
        bool success = super._grantRole(role, account);
        if (role == ELIGIBLE_ACCOUNT_ROLE && success) {
            require(account != address(0), InvalidEligibleAccount(account));
            _updateRewards(account, 0, weight(balanceOf(account)));
        }
        return success;
    }

    function _revokeRole(bytes32 role, address account) internal override returns (bool) {
        bool success = super._revokeRole(role, account);
        if (role == ELIGIBLE_ACCOUNT_ROLE && success) {
            _updateRewards(account, weight(balanceOf(account)), 0);
        }
        return success;
    }

    function _setUnstakeCooldownPeriod(uint48 unstakeCooldownPeriod_) internal {
        require(unstakeCooldownPeriod_ != 0 && unstakeCooldownPeriod_ <= 365 days, InvalidUnstakeCooldownPeriod());
        _getProtocolStakingStorage()._unstakeCooldownPeriod = unstakeCooldownPeriod_;

        emit UnstakeCooldownPeriodSet(unstakeCooldownPeriod_);
    }

    /**
     * @dev Sets the reward rate in tokens per second.
     * @param rewardRate_ The new reward rate in tokens per second.
     */
    function _setRewardRate(uint256 rewardRate_) internal {
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        $._lastUpdateReward = _historicalReward();
        $._lastUpdateTimestamp = Time.timestamp();
        $._rewardRate = rewardRate_;

        emit RewardRateSet(rewardRate_);
    }

    function _updateRewards(address user, uint256 weightBefore, uint256 weightAfter) internal {
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        uint256 oldTotalWeight = $._totalEligibleStakedWeight;
        $._totalEligibleStakedWeight = oldTotalWeight - weightBefore + weightAfter;

        if (oldTotalWeight == 0) {
            $._lastUpdateReward = 0;
            $._totalVirtualPaid = 0;
            $._lastUpdateTimestamp = Time.timestamp();
        } else if (weightBefore != weightAfter) {
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

    function _update(address from, address to, uint256 value) internal override {
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

    function _authorizeUpgrade(address newImplementation) internal override onlyRole(UPGRADER_ROLE) {}

    function _historicalReward() internal view returns (uint256) {
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
