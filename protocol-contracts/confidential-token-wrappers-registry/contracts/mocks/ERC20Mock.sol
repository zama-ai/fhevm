// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract ERC20Mock is ERC20 {
    uint8 private immutable _decimals;

    error MintAmountExceedsMax(uint256 amount, uint256 maxAmount);

    uint256 public constant MAX_MINT_AMOUNT_TOKENS = 1_000_000;

    constructor(string memory name_, string memory symbol_, uint8 decimals_) ERC20(name_, symbol_) {
        _decimals = decimals_;
    }

    function mint(address to, uint256 amount) external {
        uint256 maxMintAmount = MAX_MINT_AMOUNT_TOKENS * 10 ** decimals();
        if (amount > maxMintAmount) {
            revert MintAmountExceedsMax(amount, maxMintAmount);
        }
        _mint(to, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return _decimals;
    }
}
