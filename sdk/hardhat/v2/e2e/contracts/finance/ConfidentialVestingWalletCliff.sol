// SPDX-License-Identifier: BSD-3-Clause
pragma solidity ^0.8.24;

import {euint64} from "@fhevm/solidity/lib/FHE.sol";
import {ConfidentialVestingWallet} from "./ConfidentialVestingWallet.sol";

/**
 * @title  ConfidentialVestingWalletCliff.
 * @notice This contract offers a simple vesting wallet with a cliff for ConfidentialERC20 tokens.
 *         This is based on the VestingWalletCliff.sol contract written by OpenZeppelin.
 *         see: openzeppelin/openzeppelin-contracts/blob/master/contracts/finance/VestingWalletCliff.sol
 * @dev    This implementation is a linear vesting curve with a cliff.
 *         To use with the native asset, it is necessary to wrap the native asset to a ConfidentialERC20-like token.
 */
abstract contract ConfidentialVestingWalletCliff is ConfidentialVestingWallet {
    /// @notice Returned if the cliff duration is greater than the vesting duration.
    error InvalidCliffDuration(uint128 cliffSeconds, uint128 durationSeconds);

    /// @notice Cliff timestamp.
    uint128 public immutable CLIFF;

    /**
     * @param beneficiary_      Beneficiary address.
     * @param startTimestamp_   Start timestamp.
     * @param duration_         Duration (in seconds).
     * @param cliffSeconds_     Cliff (in seconds).
     */
    constructor(address beneficiary_, uint128 startTimestamp_, uint128 duration_, uint128 cliffSeconds_)
        ConfidentialVestingWallet(beneficiary_, startTimestamp_, duration_)
    {
        if (cliffSeconds_ > duration_) {
            revert InvalidCliffDuration(cliffSeconds_, duration_);
        }

        CLIFF = startTimestamp_ + cliffSeconds_;
    }

    /**
     * @notice                  Return the vested amount based on a linear vesting schedule with a cliff.
     * @param totalAllocation   Total allocation that is vested.
     * @param timestamp         Current timestamp.
     * @return vestedAmount     Vested amount.
     */
    function _vestingSchedule(euint64 totalAllocation, uint128 timestamp) internal virtual override returns (euint64) {
        return timestamp < CLIFF ? _EUINT64_ZERO : super._vestingSchedule(totalAllocation, timestamp);
    }
}
