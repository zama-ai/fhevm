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
        uint256 _unstakeCooldownPeriod;
        mapping(address recipient => Checkpoints.Trace208) _unstakeRequests;
        mapping(address recipient => uint256) _released;
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
    event TokensUnstaked(address indexed account, address indexed recipient, uint256 amount);
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

    /// @dev Emitted when an account unstakes to the zero address.
    error InvalidUnstakeRecipient();
    /// @dev The account is already an eligible account.
    error EligibleAccountAlreadyExists(address account);
    /// @dev The account is not an eligible account.
    error EligibleAccountDoesNotExist(address account);
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
        uint256 initialUnstakeCooldownPeriod
    ) public initializer {
        __AccessControlDefaultAdminRules_init(0, governor);
        _grantRole(UPGRADER_ROLE, upgrader);
        _grantRole(MANAGER_ROLE, manager);
        __ERC20_init(name, symbol);
        __EIP712_init(name, version);
        _getProtocolStakingStorage()._stakingToken = stakingToken_;
        _setUnstakeCooldownPeriod(initialUnstakeCooldownPeriod);
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
     * @dev Unstake `amount` tokens from `msg.sender`'s staked balance to `recipient`.
     * @param recipient The recipient where unstaked tokens should be sent.
     * @param amount The amount of tokens to unstake.
     *
     * NOTE: Unstaked tokens are released by calling {release} after {unstakeCooldownPeriod}.
     */
    function unstake(address recipient, uint256 amount) public {
        require(recipient != address(0), InvalidUnstakeRecipient());
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
     * @param account The account to release tokens to.
     *
     * WARNING: If this contract is upgraded to add slashing, the ability to withdraw to a
     * different address should be reconsidered.
     */
    function release(address account) public {
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
        ProtocolStakingStorage storage $ = _getProtocolStakingStorage();
        $._lastUpdateReward = _historicalReward();
        $._lastUpdateTimestamp = Time.timestamp();
        $._rewardRate = rewardRate_;

        emit RewardRateSet(rewardRate_);
    }

    /**
     * @dev Adds the eligible account role to `account`. Only accounts with the eligible account
     * role earn rewards for staked tokens. Only callable by the `MANAGER_ROLE` role.
     * @param account The account to grant the `ELIGIBLE_ACCOUNT_ROLE` role to.
     */
    function addEligibleAccount(address account) public onlyRole(MANAGER_ROLE) {
        require(_grantRole(ELIGIBLE_ACCOUNT_ROLE, account), EligibleAccountAlreadyExists(account));
    }

    /**
     * @dev Removes the eligible account role from `account`. `account` stops to earn rewards
     * but maintains all existing rewards. Only callable by the `MANAGER_ROLE` role.
     * @param account The account to revoke the `ELIGIBLE_ACCOUNT_ROLE` role from.
     */
    function removeEligibleAccount(address account) public onlyRole(MANAGER_ROLE) {
        require(_revokeRole(ELIGIBLE_ACCOUNT_ROLE, account), EligibleAccountDoesNotExist(account));
    }

    /**
     * @dev Sets the {unstake} cooldown period in seconds to `unstakeCooldownPeriod`. Only callable
     * by `MANAGER_ROLE` role.
     * @param unstakeCooldownPeriod_ The new unstake cooldown period.
     */
    function setUnstakeCooldownPeriod(uint256 unstakeCooldownPeriod_) public onlyRole(MANAGER_ROLE) {
        _setUnstakeCooldownPeriod(unstakeCooldownPeriod_);
    }

    /**
     * @dev Sets the reward recipient for `msg.sender` to `recipient`. All future rewards for
     * `msg.sender` will be sent to `recipient`.
     * @param recipient The recipient that will receive rewards on behalf of `msg.sender` for all future  {claimRewards} calls.
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
    function awaitingRelease(address account) public view returns (uint256) {
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

    function _setUnstakeCooldownPeriod(uint256 unstakeCooldownPeriod_) internal {
        require(unstakeCooldownPeriod_ != 0, InvalidUnstakeCooldownPeriod());
        _getProtocolStakingStorage()._unstakeCooldownPeriod = unstakeCooldownPeriod_;

        emit UnstakeCooldownPeriodSet(unstakeCooldownPeriod_);
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
