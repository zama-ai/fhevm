// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SignedMath} from "@openzeppelin/contracts/utils/math/SignedMath.sol";
import {OperatorStaking} from "./OperatorStaking.sol";
import {ProtocolStaking} from "./ProtocolStaking.sol";

/**
 * @title OperatorRewarder
 * @custom:security-contact security@zama.ai
 * @notice Distributes protocol staking rewards to operator delegators, with optional fee.
 * @dev A rewarder contract that works in tandem with `OperatorStaking` and `ProtocolStaking` contracts.
 * This contract receives rewards directly from `ProtocolStaking` and distributes them to `OperatorStaking` staker.
 * The owner of this contract can opt to take a fee on the rewards.
 */
contract OperatorRewarder {
    using SafeERC20 for IERC20;
    using Math for uint256;

    IERC20 private immutable _token;
    ProtocolStaking private immutable _protocolStaking;
    OperatorStaking private immutable _operatorStaking;
    address private _beneficiary;
    uint16 private _maxFeeBasisPoints;
    uint16 private _feeBasisPoints;
    bool private _shutdown;
    bool private _started;
    uint256 private _lastClaimTotalAssetsPlusPaidRewards;
    uint256 private _totalRewardsPaid;
    int256 private _totalVirtualRewardsPaid;
    mapping(address receiver => int256 rewardsPaid) private _rewardsPaid;
    mapping(address receiver => address claimer) private _authorizedClaimers;

    /**
     * @notice Emitted when the beneficiary is transferred.
     * @param oldBeneficiary The previous beneficiary address.
     * @param newBeneficiary The new beneficiary address.
     */
    event BeneficiaryTransferred(address indexed oldBeneficiary, address indexed newBeneficiary);

    /// @notice Emitted when the contract is shut down.
    event Shutdown();

    /**
     * @notice Emitted when the maximum fee is updated.
     * @param oldFee The previous maximum fee in basis points.
     * @param newFee The new maximum fee in basis points.
     */
    event MaxFeeUpdated(uint16 oldFee, uint16 newFee);

    /**
     * @notice Emitted when the fee is updated.
     * @param oldFee The previous fee in basis points.
     * @param newFee The new fee in basis points.
     */
    event FeeUpdated(uint16 oldFee, uint16 newFee);

    /**
     * @notice Emitted when an address is authorized to claim rewards on behalf of the receiver address.
     * @param receiver The address that will receive the rewards.
     * @param claimer The address authorized to claim rewards on behalf of the receiver.
     */
    event ClaimerAuthorized(address indexed receiver, address indexed claimer);

    /// @notice Emitted when rewards are claimed for an account.
    event RewardsClaimed(address indexed receiver, uint256 amount);

    /// @notice Emitted when fees are claimed by the beneficiary.
    event FeeClaimed(address indexed beneficiary, uint256 amount);

    /// @notice Error for invalid claimer address.
    error InvalidClaimer(address claimer);

    /// @notice Emitted when the claimer for the receiver address is already set.
    error ClaimerAlreadySet(address receiver, address claimer);

    /// @notice Emitted when an address is not authorized to claim rewards on behalf of the receiver address.
    error ClaimerNotAuthorized(address receiver, address claimer);

    /// @notice Thrown when the caller is not the ProtocolStaking's owner.
    error CallerNotProtocolStakingOwner(address caller);

    /// @notice Error for unauthorized caller (not OperatorStaking).
    error CallerNotOperatorStaking(address caller);

    /// @notice Error for unauthorized caller (not beneficiary).
    error CallerNotBeneficiary(address caller);

    /// @notice Error for invalid beneficiary address.
    error InvalidBeneficiary(address beneficiary);

    /// @notice Error for beneficiary already set to the same address.
    error BeneficiaryAlreadySet(address beneficiary);

    /// @notice Error for attempting to shutdown when already shutdown.
    error AlreadyShutdown();

    /// @notice Error for invalid basis points value.
    error InvalidBasisPoints(uint16 basisPoints);

    /// @notice Error for fee already set to the same value.
    error FeeAlreadySet(uint16 feeBasisPoints);

    /// @notice Error for basis points value greater than the maximum allowed.
    error MaxBasisPointsExceeded(uint16 basisPoints, uint16 maxBasisPoints);

    /// @notice Error for max fee already set to the same value.
    error MaxFeeAlreadySet(uint16 maxFeeBasisPoints);

    /// @notice Error for attempting to start an already started rewarder.
    error AlreadyStarted();

    /// @notice Error for attempting to use a rewarder that has not been started.
    error NotStarted();

    modifier onlyOperatorStaking() {
        require(msg.sender == address(operatorStaking()), CallerNotOperatorStaking(msg.sender));
        _;
    }

    modifier onlyOwner() {
        require(msg.sender == owner(), CallerNotProtocolStakingOwner(msg.sender));
        _;
    }

    modifier onlyBeneficiary() {
        require(msg.sender == _beneficiary, CallerNotBeneficiary(msg.sender));
        _;
    }

    modifier onlyClaimer(address receiver) {
        require(claimer(receiver) == msg.sender, ClaimerNotAuthorized(receiver, msg.sender));
        _;
    }

    modifier whenStarted() {
        require(_started, NotStarted());
        _;
    }

    /**
     * @notice Constructs the OperatorRewarder contract.
     * @dev The rewarder is not usable until `start()` is called by the OperatorStaking contract.
     * @param beneficiary_ The address that can set and claim fees.
     * @param protocolStaking_ The ProtocolStaking contract address.
     * @param operatorStaking_ The OperatorStaking contract address.
     * @param initialMaxFeeBasisPoints_ The initial max fee basis points.
     * @param initialFeeBasisPoints_ The initial fee basis points.
     */
    constructor(
        address beneficiary_,
        ProtocolStaking protocolStaking_,
        OperatorStaking operatorStaking_,
        uint16 initialMaxFeeBasisPoints_,
        uint16 initialFeeBasisPoints_
    ) {
        _transferBeneficiary(beneficiary_);
        _token = IERC20(protocolStaking_.stakingToken());
        _protocolStaking = protocolStaking_;
        _operatorStaking = operatorStaking_;

        // Set fee values directly without calling `setFee()` or `setMaxFee()`, which would snapshot
        // `_lastClaimTotalAssetsPlusPaidRewards` via `claimFee()` with potentially stale pending rewards.
        require(initialMaxFeeBasisPoints_ <= 10000, InvalidBasisPoints(initialMaxFeeBasisPoints_));
        require(
            initialFeeBasisPoints_ <= initialMaxFeeBasisPoints_,
            MaxBasisPointsExceeded(initialFeeBasisPoints_, initialMaxFeeBasisPoints_)
        );
        _maxFeeBasisPoints = initialMaxFeeBasisPoints_;
        _feeBasisPoints = initialFeeBasisPoints_;
    }

    /**
     * @notice Starts the rewarder, enabling its functionality.
     * @dev Must be called by OperatorStaking after the old rewarder (if any) has been shut down.
     * This ensures the rewarder starts operating only after any pending rewards from the old
     * rewarder have been claimed during shutdown, avoiding stale state.
     * The _lastClaimTotalAssetsPlusPaidRewards is left at 0, meaning any donations sent before
     * starting will be subject to fees.
     */
    function start() public virtual onlyOperatorStaking {
        require(!_started, AlreadyStarted());
        _started = true;
    }

    /**
     * @notice Transfers the beneficiary address. Only callable by the owner.
     * @param newBeneficiary The new beneficiary address.
     */
    function transferBeneficiary(address newBeneficiary) public virtual onlyOwner {
        _transferBeneficiary(newBeneficiary);
    }

    /**
     * @notice Claims rewards for a delegator. The caller must be authorized to claim rewards on
     * behalf of the delegator. By default, the caller is authorized to claim rewards on behalf of
     * themselves.
     * @param receiver The delegator's address that will receive the rewards.
     */
    function claimRewards(address receiver) public virtual whenStarted onlyClaimer(receiver) {
        uint256 earned_ = earned(receiver);
        if (earned_ > 0) {
            _rewardsPaid[receiver] += SafeCast.toInt256(earned_);
            _totalRewardsPaid += earned_;
            _doTransferOut(receiver, earned_);
            emit RewardsClaimed(receiver, earned_);
        }
    }

    /**
     * @notice Claims unpaid fees. Only callable by the beneficiary.
     */
    function claimFee() public virtual whenStarted onlyBeneficiary {
        _claimFee();
    }

    /**
     * @notice Sets the maximum fee in basis points (1/100th of a percent) that the beneficiary
     * can set.
     * If the new max fee is lower than the current fee:
     * - the fee is set to the new max fee
     * - the unpaid fees are claimed and transferred to the beneficiary
     * @param basisPoints Maximum fee in basis points (max 10000).
     */
    function setMaxFee(uint16 basisPoints) public virtual whenStarted onlyOwner {
        require(basisPoints != maxFeeBasisPoints(), MaxFeeAlreadySet(maxFeeBasisPoints()));

        _setMaxFee(basisPoints);
    }

    /**
     * @notice Sets the fee in basis points (1/100th of a percent). Only callable by the beneficiary.
     * Unpaid fees are claimed and transferred to the beneficiary.
     * @param basisPoints Fee in basis points (cannot be greater than the maximum fee).
     */
    function setFee(uint16 basisPoints) public virtual whenStarted onlyBeneficiary {
        require(basisPoints != feeBasisPoints(), FeeAlreadySet(feeBasisPoints()));

        _setFee(basisPoints);
    }

    /**
     * @notice Sets an address to be authorized to claim rewards on behalf of the caller. The caller
     * will be the address that will receive the rewards.
     * @param claimer_ The address to be authorized to claim rewards.
     */
    function setClaimer(address claimer_) public virtual {
        require(claimer_ != address(0), InvalidClaimer(address(0)));
        require(claimer(msg.sender) != claimer_, ClaimerAlreadySet(msg.sender, claimer_));

        _authorizedClaimers[msg.sender] = claimer_;
        emit ClaimerAuthorized(msg.sender, claimer_);
    }

    /**
     * @notice Shuts down the rewarder contract.
     * @dev Practically, this means the contract no longer considers unclaimed
     * earnings from the `ProtocolStaking` contract as an asset.
     */
    function shutdown() public virtual onlyOperatorStaking {
        require(!_shutdown, AlreadyShutdown());
        _shutdown = true;
        _protocolStaking.claimRewards(address(operatorStaking()));
        emit Shutdown();
    }

    /**
     * @notice Handles transfer of OperatorStaking shares, updating virtual rewards.
     * @param from Sender address.
     * @param to Recipient address.
     * @param shares Number of shares transferred.
     */
    function transferHook(address from, address to, uint256 shares) public virtual onlyOperatorStaking {
        uint256 oldTotalSupply = operatorStaking().totalSupply();
        if (oldTotalSupply == 0) return;

        int256 virtualAmount = SafeCast.toInt256(_allocation(shares, oldTotalSupply));

        if (from != address(0)) {
            _rewardsPaid[from] -= virtualAmount;
        } else {
            _totalVirtualRewardsPaid += virtualAmount;
        }

        if (to != address(0)) {
            _rewardsPaid[to] += virtualAmount;
        } else {
            _totalVirtualRewardsPaid -= virtualAmount;
        }
    }

    /**
     * @notice Returns the owner address, the ProtocolStaking owner address, which can set the
     * beneficiary and max fee.
     * @return TheProtocolStaking owner address.
     */
    function owner() public view virtual returns (address) {
        return protocolStaking().owner();
    }

    /**
     * @notice Returns the beneficiary address, the address that can set and claim fees.
     * @return The beneficiary address.
     */
    function beneficiary() public view virtual returns (address) {
        return _beneficiary;
    }

    /**
     * @notice Returns the authorized claimer for a receiver address. If no claimer is set (null
     * address), the receiver address is considered its own claimer.
     * @param receiver The receiver address.
     * @return The claimer address.
     */
    function claimer(address receiver) public view virtual returns (address) {
        address authorizedClaimer = _authorizedClaimers[receiver];
        return authorizedClaimer == address(0) ? receiver : authorizedClaimer;
    }

    /**
     * @notice Returns the staking token address.
     * @return The IERC20 staking token.
     */
    function token() public view virtual returns (IERC20) {
        return _token;
    }

    /**
     * @notice Returns the ProtocolStaking contract address.
     * @return The ProtocolStaking contract.
     */
    function protocolStaking() public view virtual returns (ProtocolStaking) {
        return _protocolStaking;
    }

    /**
     * @notice Returns the OperatorStaking contract address.
     * @return The OperatorStaking contract.
     */
    function operatorStaking() public view virtual returns (OperatorStaking) {
        return _operatorStaking;
    }

    /**
     * @notice Returns true if the contract is started.
     * @return True if started, false otherwise.
     */
    function isStarted() public view virtual returns (bool) {
        return _started;
    }

    /**
     * @notice Returns true if contract is shutdown.
     * @return True if shutdown, false otherwise.
     */
    function isShutdown() public view virtual returns (bool) {
        return _shutdown;
    }

    /**
     * @notice Returns the maximum fee in basis points that the beneficiary can set.
     * @return Fee in basis points.
     */
    function maxFeeBasisPoints() public view virtual returns (uint16) {
        return _maxFeeBasisPoints;
    }

    /**
     * @notice Returns the fee in basis points.
     * @return Fee in basis points.
     */
    function feeBasisPoints() public view virtual returns (uint16) {
        return _feeBasisPoints;
    }

    /**
     * @notice Returns unpaid reward for a delegator.
     * @param account The delegator's address.
     * @return Amount of unpaid reward.
     */
    function earned(address account) public view virtual returns (uint256) {
        uint256 delegatedBalance = operatorStaking().balanceOf(account);
        int256 allocation = SafeCast.toInt256(
            delegatedBalance > 0 ? _allocation(delegatedBalance, operatorStaking().totalSupply()) : 0
        );
        return SafeCast.toUint256(SignedMath.max(0, allocation - _rewardsPaid[account]));
    }

    /**
     * @notice Returns the total historical rewards distributed.
     * @dev This amount is computed as the sum of:
     * - the total rewards accumulated in the contract (see `_totalAssetsPlusPaidRewards()`) from
     * the start of the contract
     * - minus the fees not yet claimed by the beneficiary (see `_unpaidFee()`)
     * @return The total historical rewards amount.
     */
    function historicalReward() public view virtual returns (uint256) {
        uint256 totalAssetsPlusPaidRewards = _totalAssetsPlusPaidRewards();
        return totalAssetsPlusPaidRewards - _unpaidFee(totalAssetsPlusPaidRewards);
    }

    /**
     * @notice Returns unpaid fee (not yet claimed by the beneficiary).
     * @return Amount of unpaid fee.
     */
    function unpaidFee() public view virtual returns (uint256) {
        return _unpaidFee(_totalAssetsPlusPaidRewards());
    }

    /**
     * @notice Transfers the specified amount of tokens to the specified address.
     * @dev If the amount to transfer is greater than the balance of the rewarder, it will first
     * claim rewards from the ProtocolStaking contract.
     * @param to The address to transfer the tokens to.
     * @param amount The amount of tokens to transfer.
     */
    function _doTransferOut(address to, uint256 amount) internal virtual {
        IERC20 token_ = token();
        if (amount > token_.balanceOf(address(this))) {
            protocolStaking().claimRewards(address(_operatorStaking));
        }
        token_.safeTransfer(to, amount);
    }

    /**
     * @notice Transfers the beneficiary address.
     * @param newBeneficiary The new beneficiary address.
     * @dev Transferring the beneficiary address does not trigger a claim of unclaimed fees for the
     * old beneficiary on purpose. This is to avoid losing unclaimed fees in case a beneficiary loses
     * access to their private key. It is acceptable as the owner (who can set the beneficiary) is
     * expected to be a governance DAO.
     */
    function _transferBeneficiary(address newBeneficiary) internal virtual {
        require(newBeneficiary != address(0), InvalidBeneficiary(address(0)));
        require(newBeneficiary != _beneficiary, BeneficiaryAlreadySet(newBeneficiary));

        address oldBeneficiary = _beneficiary;
        _beneficiary = newBeneficiary;
        emit BeneficiaryTransferred(oldBeneficiary, newBeneficiary);
    }

    /**
     * @notice Claims fee. Fees are transferred to the beneficiary address.
     */
    function _claimFee() internal virtual {
        uint256 totalAssetsPlusPaidRewards = _totalAssetsPlusPaidRewards();
        uint256 unpaidFee_ = _unpaidFee(totalAssetsPlusPaidRewards);

        // Update the last claim value used to define the next unpaid fee (see `_unpaidFee()`).
        // This amount is exactly the same as `historicalReward()`, but we need to get the unpaid
        // fee separately in order to send the fee to the beneficiary below.
        _lastClaimTotalAssetsPlusPaidRewards = totalAssetsPlusPaidRewards - unpaidFee_;

        if (unpaidFee_ > 0) {
            _doTransferOut(beneficiary(), unpaidFee_);
            emit FeeClaimed(beneficiary(), unpaidFee_);
        }
    }

    /**
     * @notice Sets the maximum fee in basis points (1/100th of a percent) that the beneficiary
     * can set.
     * If the new max fee is lower than the current fee:
     * - the fee is set to the new max fee
     * - the unpaid fees are claimed and transferred to the beneficiary
     * @param basisPoints Maximum fee in basis points (max 10000).
     */
    function _setMaxFee(uint16 basisPoints) internal virtual {
        require(basisPoints <= 10000, InvalidBasisPoints(basisPoints));

        if (basisPoints < feeBasisPoints()) {
            _setFee(basisPoints);
        }

        emit MaxFeeUpdated(maxFeeBasisPoints(), basisPoints);
        _maxFeeBasisPoints = basisPoints;
    }

    /**
     * @notice Sets the fee in basis points (1/100th of a percent).
     * Unpaid fees are claimed and transferred to the beneficiary.
     * @param basisPoints Fee in basis points (cannot be greater than the maximum fee).
     */
    function _setFee(uint16 basisPoints) internal virtual {
        // The following statement also makes sure the basis points is not greater than 10000, as
        // the max fee basis points also follows this constraint.
        require(basisPoints <= maxFeeBasisPoints(), MaxBasisPointsExceeded(basisPoints, maxFeeBasisPoints()));

        _claimFee();
        emit FeeUpdated(feeBasisPoints(), basisPoints);
        _feeBasisPoints = basisPoints;
    }

    /**
     * @notice Returns the total assets plus earned rewards plus paid rewards.
     * @dev This amount is computed as the sum of:
     * - the balance of the rewarder contract (includes: total claimed but unpaid rewards + total donation + unpaid fees)
     * - the earned rewards by the operator staking contract (total earned but unpaid rewards)
     * - the total rewards paid to the delegators (total paid rewards)
     * @return Total assets plus earned rewards plus paid rewards.
     */
    function _totalAssetsPlusPaidRewards() internal view virtual returns (uint256) {
        return
            token().balanceOf(address(this)) +
            (isShutdown() ? 0 : protocolStaking().earned(address(operatorStaking()))) +
            _totalRewardsPaid;
    }

    /**
     * @notice Returns the unpaid fee.
     * @dev This amount is computed as a percentage (defined in basis points) of the amount of rewards
     * accumulated since the last time fees were claimed. This works because:
     * - claiming fees snapshots the historical rewards amount in the `_lastClaimTotalAssetsPlusPaidRewards`
     * value, which monitors all rewards accumulated since the start of the contract by excluding fees.
     * - the total rewards amount (`_totalAssetsPlusPaidRewards()`) is computed such as it is always
     * increasing, except when fees are claimed (where fees are transferred to the beneficiary).
     * This makes sure that the difference between both above values does not take claimed fees
     * into account when computing the next unpaid fees.
     * @param totalAssetsPlusPaidRewards The total assets plus earned rewards plus paid rewards.
     * @return Amount of unpaid fee.
     */
    function _unpaidFee(uint256 totalAssetsPlusPaidRewards) internal view virtual returns (uint256) {
        uint256 totalAssetsPlusPaidRewardsDelta = totalAssetsPlusPaidRewards - _lastClaimTotalAssetsPlusPaidRewards;
        return (totalAssetsPlusPaidRewardsDelta * feeBasisPoints()) / 10_000;
    }

    /**
     * @notice Compute total allocation based on number of shares and total shares.
     * @param share The number of shares.
     * @param total The total number of shares.
     * @return The total allocation.
     * The allocation corresponds to the rewards (both real and virtual) a user would receive if
     * the current weight distribution, with their updated stake, was constant since the deployment
     * of the protocol.
     * Total reward amount is computed as the sum of:
     * - historical rewards: total of all rewards generated up to that point
     * - paid virtual rewards: a pool of "virtual" rewards that account for changes in the weight distribution
     *  Note: the `mulDiv` rounds down: floor(totalRewards * share / total)
     */
    function _allocation(uint256 share, uint256 total) internal view virtual returns (uint256) {
        return
            SafeCast.toUint256(SafeCast.toInt256(historicalReward()) + _totalVirtualRewardsPaid).mulDiv(share, total);
    }
}
