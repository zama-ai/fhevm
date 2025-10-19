// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Checkpoints} from "@openzeppelin/contracts/utils/structs/Checkpoints.sol";
import {Time} from "@openzeppelin/contracts/utils/types/Time.sol";
import {ProtocolStaking} from "./../ProtocolStaking.sol";

contract ProtocolStakingSlashingMock is ProtocolStaking {
    using Checkpoints for Checkpoints.Trace208;
    using SafeERC20 for IERC20;

    bytes32 private constant PROTOCOL_STAKING_STORAGE_LOCATION =
        0x6867237db38693700f305f18dff1dbf600e282237f7d452b4c792e6b019c6b00;

    mapping(address => uint256) private _slashedAmount;

    function slash(address account, uint256 amount) public {
        _burn(account, amount);
    }

    function slashWithdrawal(address account, uint256 amount) public {
        _slashedAmount[account] += amount;
    }

    function awaitingRelease(address account) public view virtual override returns (uint256) {
        return tokensToReleaseAt(account, type(uint48).max);
    }

    function tokensToReleaseAt(address account, uint48 timestamp) public view virtual returns (uint256) {
        ProtocolStakingStorage storage $ = __getProtocolStakingStorage();
        return $._unstakeRequests[account].upperLookup(timestamp) - $._released[account] - _slashedAmount[account];
    }

    function release(address account) public virtual override {
        uint256 amountToRelease = tokensToReleaseAt(account, Time.timestamp());
        if (amountToRelease > 0) {
            __getProtocolStakingStorage()._released[account] += amountToRelease;
            IERC20(stakingToken()).safeTransfer(account, amountToRelease);
        }
    }

    function __getProtocolStakingStorage() private pure returns (ProtocolStaking.ProtocolStakingStorage storage $) {
        assembly {
            $.slot := PROTOCOL_STAKING_STORAGE_LOCATION
        }
    }
}
