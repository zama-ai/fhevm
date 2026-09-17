// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {ERC20, ERC1363} from "@openzeppelin/contracts/token/ERC20/extensions/ERC1363.sol";
import {ERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";

contract ERC20Mock is ERC1363, ERC20Permit, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    uint8 private immutable _decimals;

    /// @dev Fixed per-call {mint} cap for callers without {MINTER_ROLE}, in base units. Callers holding
    /// {MINTER_ROLE} bypass this cap entirely (mirrors the real ZAMA token's minter behaviour).
    uint256 public immutable publicMintCap;

    error MintAmountExceedsMax(uint256 amount, uint256 maxAmount);

    constructor(string memory name_, string memory symbol_, uint8 decimals_) ERC20(name_, symbol_) ERC20Permit(name_) {
        _decimals = decimals_;
        publicMintCap = 1_000_000 * 10 ** decimals_;
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function decimals() public view virtual override returns (uint8) {
        return _decimals;
    }

    /// @dev Mint `amount` tokens to `to`. Callers with {MINTER_ROLE} can mint any amount; all other
    /// callers are capped at {publicMintCap} per call.
    function mint(address to, uint256 amount) public virtual {
        if (!hasRole(MINTER_ROLE, msg.sender) && amount > publicMintCap) {
            revert MintAmountExceedsMax(amount, publicMintCap);
        }
        _mint(to, amount);
    }

    function supportsInterface(bytes4 interfaceId) public view virtual override(ERC1363, AccessControl) returns (bool) {
        return super.supportsInterface(interfaceId);
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
