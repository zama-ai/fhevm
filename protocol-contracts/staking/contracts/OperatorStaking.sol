// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ERC1363} from "@openzeppelin/contracts/token/ERC20/extensions/ERC1363.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC4626, IERC4626} from "@openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SignedMath} from "@openzeppelin/contracts/utils/math/SignedMath.sol";
import {ReentrancyGuardTransient} from "@openzeppelin/contracts/utils/ReentrancyGuardTransient.sol";
import {Checkpoints} from "@openzeppelin/contracts/utils/structs/Checkpoints.sol";
import {Time} from "@openzeppelin/contracts/utils/types/Time.sol";
import {OperatorRewarder} from "./OperatorRewarder.sol";
import {ProtocolStaking} from "./ProtocolStaking.sol";

/**
 * @title OperatorStaking
 * @custom:security-contact security@zama.ai
 * @notice Allows users to delegate assets to an operator staker and receive shares, with support for reward distribution.
 * @dev Integrates with ProtocolStaking and OperatorRewarder contracts. Inspired by ERC7540 but not fully compliant.
 * Also inherits ERC1363 to ease of users with potential OperatorStaking contract migrations.
 *
 * NOTE: This contract supports slashing on the `ProtocolStaking` level, meaning that the overall stake of this contract
 * may decrease due to slashing. These losses are symmetrically passed to delegators on the `OperatorStaking` level.
 * Slashing must first decrease the `ProtocolStaking` balance of this contract before affecting pending withdrawals.
 */
contract OperatorStaking is ERC1363, Ownable, ReentrancyGuardTransient {
    using Math for uint256;
    using Checkpoints for Checkpoints.Trace208;

    ProtocolStaking private immutable _protocolStaking;
    IERC20 private immutable _asset;
    address private _rewarder;
    uint256 private _totalSharesInRedemption;
    mapping(address => uint256) private _sharesReleased;
    mapping(address => Checkpoints.Trace208) private _redeemRequests;
    mapping(address => mapping(address => bool)) private _operator;

    /// @dev Emitted when an operator is set or unset for a controller.
    event OperatorSet(address indexed controller, address indexed operator, bool approved);

    /// @dev Emitted when a redeem request is made.
    event RedeemRequest(
        address indexed controller,
        address indexed owner,
        uint256 indexed requestId,
        address sender,
        uint256 shares,
        uint48 releaseTime
    );

    /// @dev Emitted when the rewarder contract is set.
    event RewarderSet(address oldRewarder, address newRewarder);

    /// @dev Throw when the rewarder address is not valid during {setRewarder}.
    error InvalidRewarder(address rewarder);

    /// @dev Thrown when the sender does not have authorization to perform an action.
    error Unauthorized();

    /// @dev Thrown when the controller address is not valid (e.g., zero address).
    error InvalidController();

    /**
     * @notice Initializes the OperatorStaking contract.
     * @param name The name of the ERC20 token.
     * @param symbol The symbol of the ERC20 token.
     * @param protocolStaking_ The ProtocolStaking contract address.
     * @param owner The owner address.
     * @param beneficiary The address that can set and claim fees.
     * @param initialMaxFeeBasisPoints_ The initial maximum fee basis points for the OperatorRewarder contract.
     * @param initialFeeBasisPoints_ The initial fee basis points for the OperatorRewarder contract.
     */
    constructor(
        string memory name,
        string memory symbol,
        ProtocolStaking protocolStaking_,
        address owner,
        address beneficiary,
        uint16 initialMaxFeeBasisPoints_,
        uint16 initialFeeBasisPoints_
    ) ERC20(name, symbol) Ownable(owner) {
        _asset = IERC20(protocolStaking_.stakingToken());
        _protocolStaking = protocolStaking_;

        IERC20(asset()).approve(address(protocolStaking_), type(uint256).max);

        address rewarder_ = address(
            new OperatorRewarder(
                owner,
                beneficiary,
                protocolStaking_,
                this,
                initialMaxFeeBasisPoints_,
                initialFeeBasisPoints_
            )
        );
        protocolStaking_.setRewardsRecipient(rewarder_);
        _rewarder = rewarder_;

        emit RewarderSet(address(0), rewarder_);
    }

    /**
     * @notice Deposit assets and receive shares.
     * @param assets Amount of assets to deposit.
     * @param receiver Address to receive the minted shares.
     * @return shares Amount of shares minted.
     */
    function deposit(uint256 assets, address receiver) public virtual returns (uint256) {
        uint256 maxAssets = maxDeposit(receiver);
        require(assets <= maxAssets, ERC4626.ERC4626ExceededMaxDeposit(receiver, assets, maxAssets));

        uint256 shares = previewDeposit(assets);
        _deposit(msg.sender, receiver, assets, shares);

        return shares;
    }

    /**
     * @notice Request to redeem shares for assets, subject to cooldown.
     * @param shares Amount of shares to redeem.
     * @param controller The controller address for the request.
     * @param owner The owner of the shares.
     */
    function requestRedeem(uint208 shares, address controller, address owner) public virtual {
        if (shares == 0) return;
        require(controller != address(0), InvalidController());
        if (msg.sender != owner) {
            _spendAllowance(owner, msg.sender, shares);
        }
        _burn(owner, shares);

        uint256 newTotalSharesInRedemption = totalSharesInRedemption() + shares;
        _totalSharesInRedemption = newTotalSharesInRedemption;

        ProtocolStaking protocolStaking_ = protocolStaking();
        int256 assetsToWithdraw = SafeCast.toInt256(previewRedeem(newTotalSharesInRedemption)) -
            SafeCast.toInt256(
                IERC20(asset()).balanceOf(address(this)) + protocolStaking_.awaitingRelease(address(this))
            );

        (, uint48 lastReleaseTime, uint208 controllerSharesRedeemed) = _redeemRequests[controller].latestCheckpoint();
        uint48 releaseTime = protocolStaking_.unstake(SafeCast.toUint256(SignedMath.max(assetsToWithdraw, 0)));
        assert(releaseTime >= lastReleaseTime); // should never happen
        _redeemRequests[controller].push(releaseTime, controllerSharesRedeemed + shares);

        emit RedeemRequest(controller, owner, 0, msg.sender, shares, releaseTime);
    }

    /**
     * @notice Redeem shares for assets after cooldown.
     * @param shares Amount of shares to redeem (use max uint256 for all claimable).
     * @param receiver Address to receive the assets.
     * @param controller The controller address for the redeem.
     * @return assets Amount of assets received.
     */
    function redeem(
        uint256 shares,
        address receiver,
        address controller
    ) public virtual nonReentrant returns (uint256) {
        require(msg.sender == controller || isOperator(controller, msg.sender), Unauthorized());

        uint256 maxShares = maxRedeem(controller);
        if (shares == type(uint256).max) {
            shares = maxShares;
        } else if (shares > maxShares) {
            revert ERC4626.ERC4626ExceededMaxRedeem(controller, shares, maxShares);
        }

        uint256 assets = previewRedeem(shares);

        if (assets > 0) {
            _totalSharesInRedemption -= shares;
            _sharesReleased[controller] += shares;
            _doTransferOut(receiver, assets);

            emit IERC4626.Withdraw(msg.sender, receiver, controller, assets, shares);
        }

        return assets;
    }

    /**
     * @dev Stake excess tokens held by this contract. Excess tokens held by this contract after
     * accounting for all in-flight redemptions are restaked into the `ProtocolStaking` contract.
     *
     * NOTE: Excess tokens will be in the `OperatorStaking` contract the operator is slashed
     * during a redemption flow or if donations are made to it. Anyone can call this function to
     * restake those tokens.
     */
    function stakeExcess() public virtual {
        ProtocolStaking protocolStaking_ = protocolStaking();
        protocolStaking_.release(address(this));
        uint256 amountToRestake = IERC20(asset()).balanceOf(address(this)) - previewRedeem(totalSharesInRedemption());
        protocolStaking_.stake(amountToRestake);
    }

    /**
     * @dev Set a new rewarder contract.
     * @param newRewarder The new rewarder contract address. This contract must not be the same as the current
     * and must have code.
     */
    function setRewarder(address newRewarder) public virtual onlyOwner {
        address oldRewarder = rewarder();
        require(newRewarder != oldRewarder && newRewarder.code.length > 0, InvalidRewarder(newRewarder));
        OperatorRewarder(oldRewarder).shutdown();
        _rewarder = newRewarder;
        protocolStaking().setRewardsRecipient(newRewarder);

        emit RewarderSet(oldRewarder, newRewarder);
    }

    /**
     * @notice Set or unset an operator for the caller.
     * @param operator The address to set as operator.
     * @param approved True to approve, false to revoke.
     */
    function setOperator(address operator, bool approved) public virtual {
        _operator[msg.sender][operator] = approved;

        emit OperatorSet(msg.sender, operator, approved);
    }

    /**
     * @notice Returns the address of the staking asset.
     * @return The asset address.
     */
    function asset() public view virtual returns (address) {
        return address(_asset);
    }

    /**
     * @notice Returns the ProtocolStaking contract address.
     * @return The ProtocolStaking contract address.
     */
    function protocolStaking() public view virtual returns (ProtocolStaking) {
        return _protocolStaking;
    }

    /**
     * @notice Returns the rewarder contract address.
     * @return The rewarder contract address.
     */
    function rewarder() public view virtual returns (address) {
        return _rewarder;
    }

    /**
     * @notice Returns the total assets managed by the contract.
     * @return The total assets.
     */
    function totalAssets() public view virtual returns (uint256) {
        ProtocolStaking protocolStaking_ = protocolStaking();
        return
            IERC20(asset()).balanceOf(address(this)) +
            protocolStaking_.balanceOf(address(this)) +
            protocolStaking_.awaitingRelease(address(this));
    }

    /**
     * @notice Returns the number of shares pending for redeem for a controller.
     * @param controller The controller address.
     * @return Amount of shares pending redeem.
     */
    function pendingRedeemRequest(uint256, address controller) public view virtual returns (uint256) {
        return _redeemRequests[controller].latest() - _redeemRequests[controller].upperLookup(Time.timestamp());
    }

    /**
     * @notice Returns the number of claimable shares for redeem for a controller.
     * @param controller The controller address.
     * @return Amount of claimable shares.
     */
    function claimableRedeemRequest(uint256, address controller) public view virtual returns (uint256) {
        return _redeemRequests[controller].upperLookup(Time.timestamp()) - _sharesReleased[controller];
    }

    /**
     * @notice Returns the total shares in redemption.
     * @return The total shares in redemption.
     */
    function totalSharesInRedemption() public view virtual returns (uint256) {
        return _totalSharesInRedemption;
    }

    /**
     * @notice Returns the maximum deposit allowed for an address.
     * @return The maximum deposit amount.
     */
    function maxDeposit(address) public view virtual returns (uint256) {
        return type(uint256).max;
    }

    /**
     * @notice Returns the maximum redeemable shares for an owner.
     * @param owner The owner address.
     * @return The maximum redeemable shares.
     */
    function maxRedeem(address owner) public view virtual returns (uint256) {
        return claimableRedeemRequest(0, owner);
    }

    /**
     * @notice Returns the number of shares that would be minted for a given deposit.
     * @param assets Amount of assets to deposit.
     * @return Amount of shares that would be minted.
     */
    function previewDeposit(uint256 assets) public view virtual returns (uint256) {
        return _convertToShares(assets, Math.Rounding.Floor);
    }

    /**
     * @notice Returns the amount of assets that would be received for redeeming shares.
     * @param shares Amount of shares to redeem.
     * @return Amount of assets that would be received.
     */
    function previewRedeem(uint256 shares) public view virtual returns (uint256) {
        return _convertToAssets(shares, Math.Rounding.Floor);
    }

    /**
     * @notice Returns true if the operator is approved for the controller.
     * @param controller The controller address.
     * @param operator The operator address.
     * @return True if operator is approved, false otherwise.
     */
    function isOperator(address controller, address operator) public view virtual returns (bool) {
        return _operator[controller][operator];
    }

    function _doTransferOut(address to, uint256 amount) internal {
        IERC20 asset_ = IERC20(asset());
        if (amount > asset_.balanceOf(address(this))) {
            protocolStaking().release(address(this));
        }
        SafeERC20.safeTransfer(asset_, to, amount);
    }

    /**
     * @dev Updates shares while notifying the rewarder that shares were transferred.
     */
    function _update(address from, address to, uint256 amount) internal virtual override {
        OperatorRewarder(rewarder()).transferHook(from, to, amount);
        super._update(from, to, amount);
    }

    function _deposit(address caller, address receiver, uint256 assets, uint256 shares) internal virtual {
        // If asset() is ERC-777, `transferFrom` can trigger a reentrancy BEFORE the transfer happens through the
        // `tokensToSend` hook. On the other hand, the `tokenReceived` hook, that is triggered after the transfer,
        // calls the vault, which is assumed not malicious.
        //
        // Conclusion: we need to do the transfer before we mint so that any reentrancy would happen before the
        // assets are transferred and before the shares are minted, which is a valid state.
        // slither-disable-next-line reentrancy-no-eth
        SafeERC20.safeTransferFrom(IERC20(asset()), caller, address(this), assets);
        _mint(receiver, shares);
        protocolStaking().stake(assets);

        emit IERC4626.Deposit(caller, receiver, assets, shares);
    }

    function _convertToShares(uint256 assets, Math.Rounding rounding) internal view virtual returns (uint256) {
        // Shares in redemption have not yet received assets, so we need to account for them in the conversion.
        return
            assets.mulDiv(
                (totalSupply() + totalSharesInRedemption()) + 10 ** _decimalsOffset(),
                totalAssets() + 1,
                rounding
            );
    }

    function _convertToAssets(uint256 shares, Math.Rounding rounding) internal view virtual returns (uint256) {
        // Shares in redemption have not yet received assets, so we need to account for them in the conversion.
        return
            shares.mulDiv(
                totalAssets() + 1,
                (totalSupply() + totalSharesInRedemption()) + 10 ** _decimalsOffset(),
                rounding
            );
    }

    function _decimalsOffset() internal view virtual returns (uint8) {
        return 0;
    }
}
