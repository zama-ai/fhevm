// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC20, ERC1363} from "@openzeppelin/contracts/token/ERC20/extensions/ERC1363.sol";

contract ERC20Mock is ERC1363 {
    uint8 private immutable _decimals;

    constructor(string memory name_, string memory symbol_, uint8 decimals_) ERC20(name_, symbol_) {
        _decimals = decimals_;
    }

    function decimals() public view virtual override returns (uint8) {
        return _decimals;
    }

    function mint(address to, uint256 amount) public virtual {
        _mint(to, amount);
    }
}

contract ERC20RevertDecimalsMock is ERC20Mock {
    constructor() ERC20Mock("ERC20RevertDecimalsMock", "ERC20RevertDecimalsMock", 18) {}

    function decimals() public pure override returns (uint8) {
        revert("Decimals not available");
    }
}

contract ERC20ExcessDecimalsMock is ERC20Mock {
    constructor() ERC20Mock("ERC20ExcessDecimalsMock", "ERC20ExcessDecimalsMock", 18) {}

    function decimals() public pure override returns (uint8) {
        assembly {
            mstore(0, 300)
            return(0, 0x20)
        }
    }
}
