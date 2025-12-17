// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.27;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract ERC20NoDecimals is ERC20 {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {
        _mint(msg.sender, 1000000 * 10**18);
    }

    // Override decimals to revert, simulating a token without decimals function
    function decimals() public pure override returns (uint8) {
        revert("decimals not implemented");
    }
}

contract ERC20InvalidDecimals is ERC20 {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {
        _mint(msg.sender, 1000000 * 10**18);
    }

    // Override decimals to return invalid data (not 32 bytes)
    function decimals() public pure override returns (uint8) {
        assembly {
            // Return 64 bytes instead of 32
            mstore(0x00, 0x12)
            mstore(0x20, 0x34)
            return(0x00, 0x40)
        }
    }
}