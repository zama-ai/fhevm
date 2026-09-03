// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";

/**
 * @title     TestERC20Mintable
 * @notice    This contract is an ERC20 token that is mintable by the owner.
 */
contract TestERC20Mintable is ERC20, Ownable2Step {
    /// @dev override number of decimals
    uint8 private immutable _DECIMALS;

    constructor(string memory name_, string memory symbol_, uint8 decimals_, address owner_)
        ERC20(name_, symbol_)
        Ownable(owner_)
    {
        _DECIMALS = decimals_;
    }

    /**
     * @notice Returns the number of decimals.
     */
    function decimals() public view override returns (uint8) {
        return _DECIMALS;
    }

    /**
     * @notice              Mint tokens.
     * @param amount        Amount of tokens to mint.
     */
    function mint(uint256 amount) public onlyOwner {
        _mint(msg.sender, amount);
    }
}
