// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract ERC20Mock is ERC20 {
    error MintAmountExceedsMax(uint256 amount, uint256 maxAmount);

    uint256 public constant MAX_MINT_AMOUNT = 1_000_000 * 10 ** 18;

    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {}

    function mint(address to, uint256 amount) external {
        if (amount > MAX_MINT_AMOUNT) {
            revert MintAmountExceedsMax(amount, MAX_MINT_AMOUNT);
        }
        _mint(to, amount);
    }
}
