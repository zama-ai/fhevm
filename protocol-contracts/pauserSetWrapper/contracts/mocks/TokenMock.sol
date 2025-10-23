// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.22;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { Pausable } from "@openzeppelin/contracts/utils/Pausable.sol";
import { AccessControl } from "@openzeppelin/contracts/access/AccessControl.sol";

contract TokenMock is ERC20, AccessControl, Pausable {
    bytes32 public constant MINTING_PAUSER_ROLE = keccak256("MINTING_PAUSER_ROLE");

    constructor() ERC20("Mock", "MOCK") {
        _mint(msg.sender, 11_000_000_000 * 1e18);
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function mint(address to, uint256 amount) public onlyRole(DEFAULT_ADMIN_ROLE) whenNotPaused {
        _mint(to, amount);
    }

    function pauseMinting() external onlyRole(MINTING_PAUSER_ROLE) {
        _pause();
    }

    function unpauseMinting() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}
