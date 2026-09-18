// SPDX-License-Identifier: BSD-3-Clause
pragma solidity ^0.8.24;

import {FHE, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {IConfidentialERC20} from "../token/ERC20/IConfidentialERC20.sol";

/**
 * @title  ConfidentialVestingWallet.
 * @notice This contract offers a simple vesting wallet for ConfidentialERC20 tokens.
 *         This is based on the VestingWallet.sol contract written by OpenZeppelin.
 *         see: openzeppelin/openzeppelin-contracts/blob/master/contracts/finance/VestingWallet.sol
 * @dev    Default implementation is a linear vesting curve.
 *         To use with the native asset, it is necessary to wrap the native asset to a ConfidentialERC20-like token.
 */
abstract contract ConfidentialVestingWallet {
    /// @notice      Emitted when tokens are released to the beneficiary address.
    /// @param token Address of the token being released.
    event ConfidentialERC20Released(address indexed token);

    /// @notice Beneficiary address.
    address public immutable BENEFICIARY;

    /// @notice Duration (in seconds).
    uint128 public immutable DURATION;

    /// @notice End timestamp.
    uint128 public immutable END_TIMESTAMP;

    /// @notice Start timestamp.
    uint128 public immutable START_TIMESTAMP;

    /// @notice Constant for zero using FHE.
    /// @dev    Since it is expensive to compute 0, it is stored instead.
    euint64 internal immutable _EUINT64_ZERO;

    /// @notice Total encrypted amount released (to the beneficiary).
    mapping(address token => euint64 amountReleased) internal _amountReleased;

    /**
     * @param beneficiary_      Beneficiary address.
     * @param startTimestamp_   Start timestamp.
     * @param duration_         Duration (in seconds).
     */
    constructor(address beneficiary_, uint128 startTimestamp_, uint128 duration_) {
        START_TIMESTAMP = startTimestamp_;
        DURATION = duration_;
        END_TIMESTAMP = startTimestamp_ + duration_;
        BENEFICIARY = beneficiary_;

        /// @dev Store this constant variable in the storage.
        _EUINT64_ZERO = FHE.asEuint64(0);

        FHE.allow(_EUINT64_ZERO, beneficiary_);
        FHE.allowThis(_EUINT64_ZERO);
    }

    /**
     * @notice  Release the tokens that have already vested.
     * @dev     Anyone can call this function but the beneficiary receives the tokens.
     */
    function release(address token) public virtual {
        euint64 amount = _releasable(token);
        euint64 amountReleased = FHE.add(_amountReleased[token], amount);
        _amountReleased[token] = amountReleased;

        FHE.allow(amountReleased, BENEFICIARY);
        FHE.allowThis(amountReleased);
        FHE.allowTransient(amount, token);
        IConfidentialERC20(token).transfer(BENEFICIARY, amount);

        emit ConfidentialERC20Released(token);
    }

    /**
     * @notice                  Return the encrypted amount of total tokens released.
     * @dev                     It is only reencryptable by the owner.
     * @return amountReleased   Total amount of tokens released.
     */
    function released(address token) public view virtual returns (euint64 amountReleased) {
        return _amountReleased[token];
    }

    /**
     * @notice                  Calculate the amount of tokens that can be released.
     * @return releasableAmount Releasable amount.
     */
    function _releasable(address token) internal virtual returns (euint64 releasableAmount) {
        return FHE.sub(_vestedAmount(token, uint128(block.timestamp)), released(token));
    }

    /**
     * @notice                  Calculate the amount of tokens that has already vested.
     * @param timestamp         Current timestamp.
     * @return vestedAmount     Vested amount.
     */
    function _vestedAmount(address token, uint128 timestamp) internal virtual returns (euint64 vestedAmount) {
        return _vestingSchedule(FHE.add(IConfidentialERC20(token).balanceOf(address(this)), released(token)), timestamp);
    }

    /**
     * @notice                  Return the vested amount based on a linear vesting schedule.
     * @dev                     It must be overriden for non-linear schedules.
     * @param totalAllocation   Total allocation that is vested.
     * @param timestamp         Current timestamp.
     * @return vestedAmount     Vested amount.
     */
    function _vestingSchedule(euint64 totalAllocation, uint128 timestamp)
        internal
        virtual
        returns (euint64 vestedAmount)
    {
        if (timestamp < START_TIMESTAMP) {
            return _EUINT64_ZERO;
        } else if (timestamp >= END_TIMESTAMP) {
            return totalAllocation;
        } else {
            /// @dev It casts to euint128 to prevent overflow with the multiplication.
            return
                FHE.asEuint64(FHE.div(FHE.mul(FHE.asEuint128(totalAllocation), timestamp - START_TIMESTAMP), DURATION));
        }
    }
}
