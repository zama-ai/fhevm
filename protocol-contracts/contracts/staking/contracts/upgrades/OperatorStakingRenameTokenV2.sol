// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {OperatorStaking} from "../OperatorStaking.sol";

/**
 * @title OperatorStakingRenameTokenV2
 * @notice Upgrade contract to fix swapped name and symbol in OperatorStaking deployment.
 * @dev The original deployment had name and symbol swapped.
 */
contract OperatorStakingRenameTokenV2 is OperatorStaking {
    /**
     * @notice Re-initializes the contract's name and symbol.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2(string memory name, string memory symbol) public virtual reinitializer(2) {
        __ERC20_init(name, symbol);
    }
}
