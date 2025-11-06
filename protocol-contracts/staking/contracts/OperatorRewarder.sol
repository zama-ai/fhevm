// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
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
 * @notice Distributes protocol staking rewards to operator stakers, with optional owner fee.
 * @dev A rewarder contract that works in tandem with `OperatorStaking` and `ProtocolStaking` contracts.
 * This contract receives rewards directly from `ProtocolStaking` and distributes them to `OperatorStaking` staker.
 * The owner of this contract can opt to take a fee on the rewards.
 */
contract OperatorRewarder is Ownable {
    using SafeERC20 for IERC20;
    using Math for uint256;

    IERC20 private immutable _token;
    ProtocolStaking private immutable _protocolStaking;
    OperatorStaking private immutable _operatorStaking;
    uint16 private _ownerFeeBasisPoints;
    bool private _shutdown;
    uint256 private _lastClaimTotalAssetsPlusPaidRewards;
    uint256 private _totalRewardsPaid;
    int256 private _totalVirtualRewardsPaid;
    mapping(address => int256) private _rewardsPaid;

    /// @notice Emitted when the contract is shut down.
    event Shutdown();

    /// @notice Emitted when the owner fee is updated.
    event OwnerFeeUpdated(uint16 oldFee, uint16 newFee);

    /// @notice Error for unauthorized caller (not OperatorStaking).
    error CallerNotOperatorStaking(address caller);

    /// @notice Error for attempting to shutdown when already shutdown.
    error AlreadyShutdown();

    /// @notice Error for invalid basis points value.
    error InvalidBasisPoints(uint16 basisPoints);

    modifier onlyOperatorStaking() {
        require(msg.sender == address(operatorStaking()), CallerNotOperatorStaking(msg.sender));
        _;
    }

    constructor(address owner, ProtocolStaking protocolStaking_, OperatorStaking operatorStaking_) Ownable(owner) {
        _token = IERC20(protocolStaking_.stakingToken());
        _protocolStaking = protocolStaking_;
        _operatorStaking = operatorStaking_;
    }

    /**
     * @notice Claims rewards for a staker.
     * @param account The staker's address.
     */
    function claimRewards(address account) public virtual {
        uint256 earned_ = earned(account);
        if (earned_ > 0) {
            _rewardsPaid[account] += SafeCast.toInt256(earned_);
            _totalRewardsPaid += earned_;
            _doTransferOut(account, earned_);
        }
    }

    /**
     * @notice Claims owner fee.
     */
    function claimOwnerFee() public virtual onlyOwner {
        uint256 totalAssetsPlusPaidRewards = _totalAssetsPlusPaidRewards();
        uint256 unpaidOwnerFee_ = _unpaidOwnerFee(totalAssetsPlusPaidRewards);
        _lastClaimTotalAssetsPlusPaidRewards = totalAssetsPlusPaidRewards - unpaidOwnerFee_;
        if (unpaidOwnerFee_ > 0) {
            _doTransferOut(owner(), unpaidOwnerFee_);
        }
    }

    /**
     * @notice Sets the owner fee in basis points (1/100th of a percent).
     * @param basisPoints Fee in basis points (max 10000).
     */
    function setOwnerFee(uint16 basisPoints) public virtual onlyOwner {
        require(basisPoints <= 10000, InvalidBasisPoints(basisPoints));

        claimOwnerFee();
        emit OwnerFeeUpdated(_ownerFeeBasisPoints, basisPoints);
        _ownerFeeBasisPoints = basisPoints;
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
     * @notice Returns the staking token address.
     * @return The IERC20 staking token.
     */
    function token() public view returns (IERC20) {
        return _token;
    }

    /**
     * @notice Returns the ProtocolStaking contract address.
     * @return The ProtocolStaking contract.
     */
    function protocolStaking() public view returns (ProtocolStaking) {
        return _protocolStaking;
    }

    /**
     * @notice Returns the OperatorStaking contract address.
     * @return The OperatorStaking contract.
     */
    function operatorStaking() public view returns (OperatorStaking) {
        return _operatorStaking;
    }

    /**
     * @notice Returns true if contract is shutdown.
     * @return True if shutdown, false otherwise.
     */
    function isShutdown() public view returns (bool) {
        return _shutdown;
    }

    /**
     * @notice Returns the owner fee in basis points.
     * @return Fee in basis points.
     */
    function ownerFeeBasisPoints() public view returns (uint16) {
        return _ownerFeeBasisPoints;
    }

    /**
     * @notice Returns unpaid reward for a staker.
     * @param account The staker's address.
     * @return Amount of unpaid reward.
     */
    function earned(address account) public view virtual returns (uint256) {
        uint256 stakedBalance = operatorStaking().balanceOf(account);
        int256 allocation = SafeCast.toInt256(
            stakedBalance > 0 ? _allocation(stakedBalance, operatorStaking().totalSupply()) : 0
        );
        return SafeCast.toUint256(SignedMath.max(0, allocation - _rewardsPaid[account]));
    }

    /**
     * @notice Returns unpaid owner fee.
     * @return Amount of unpaid owner fee.
     */
    function unpaidOwnerFee() public view virtual returns (uint256) {
        return _unpaidOwnerFee(_totalAssetsPlusPaidRewards());
    }

    function _doTransferOut(address to, uint256 amount) internal {
        IERC20 token_ = token();
        if (amount > token_.balanceOf(address(this))) {
            protocolStaking().claimRewards(address(_operatorStaking));
        }
        token_.safeTransfer(to, amount);
    }

    function _totalAssetsPlusPaidRewards() internal view returns (uint256) {
        return
            token().balanceOf(address(this)) +
            (isShutdown() ? 0 : protocolStaking().earned(address(operatorStaking()))) +
            _totalRewardsPaid;
    }

    function _historicalReward() internal view returns (uint256) {
        uint256 totalAssetsPlusPaidRewards = _totalAssetsPlusPaidRewards();
        return totalAssetsPlusPaidRewards - _unpaidOwnerFee(totalAssetsPlusPaidRewards);
    }

    function _unpaidOwnerFee(uint256 totalAssetsPlusPaidRewards) internal view returns (uint256) {
        uint256 totalAssetsPlusPaidRewardsDelta = totalAssetsPlusPaidRewards - _lastClaimTotalAssetsPlusPaidRewards;
        return (totalAssetsPlusPaidRewardsDelta * ownerFeeBasisPoints()) / 10_000;
    }

    /// @dev Compute total allocation based on number of shares and total shares. Must take paid rewards into account after.
    function _allocation(uint256 share, uint256 total) private view returns (uint256) {
        return
            SafeCast.toUint256(SafeCast.toInt256(_historicalReward()) + _totalVirtualRewardsPaid).mulDiv(share, total);
    }
}
