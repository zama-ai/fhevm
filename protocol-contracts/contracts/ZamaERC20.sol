// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC1363.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

// @dev WARNING: This is for testing purposes only
contract ZamaERC20 is ERC20, ERC20Permit, ERC1363, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    constructor(string memory _name, string memory _symbol, address initialSupplyReceiver, address initialAdmin) ERC20(_name, _symbol) ERC20Permit(_name) {
        _mint(initialSupplyReceiver, 1_000_000_000 * 1e18);
        _grantRole(DEFAULT_ADMIN_ROLE, initialAdmin);
    }

    /**
     * @notice Only a minter could mint new tokens.
     * @param _to Receiver of the newly minted tokens.
    * @param _amount Number of tokens to mint.
     */
    function mint(address _to, uint256 _amount) public onlyRole(MINTER_ROLE) {
        _mint(_to, _amount);
    }

    /**
     * @notice Anyone could burn tokens he owns.
     * @param amount Number of tokens to burn from own balance.
     */
    function burn(uint256 amount) public {
        _burn(msg.sender, amount);
    }

    /**
     * @dev See {IERC165-supportsInterface}.
     */
    function supportsInterface(bytes4 interfaceId) public view override(ERC1363, AccessControl) returns (bool) {
        return super.supportsInterface(interfaceId);
    }
}
