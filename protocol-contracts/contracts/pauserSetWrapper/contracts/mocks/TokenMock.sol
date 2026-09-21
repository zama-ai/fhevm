// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.22;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { Pausable } from "@openzeppelin/contracts/utils/Pausable.sol";
import { AccessControl } from "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title TokenMock
 * @notice This contract is a mock of a token that can be paused. It is used for testing purposes
 * and shouldn't be used in testnet or mainnet.
 */
contract TokenMock is ERC20, AccessControl, Pausable {
    /**
     * @notice The role that allows the pauser to pause the minting of the token.
     */
    bytes32 public constant MINTING_PAUSER_ROLE = keccak256("MINTING_PAUSER_ROLE");

    constructor() ERC20("Mock", "MOCK") {
        _mint(msg.sender, 11_000_000_000 * 1e18);
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    /**
     * @notice Mints a new amount of tokens to the given address. Restricted to the default admin.
     * @param to The address to mint the tokens to.
     * @param amount The amount of tokens to mint.
     */
    function mint(address to, uint256 amount) public onlyRole(DEFAULT_ADMIN_ROLE) whenNotPaused {
        _mint(to, amount);
    }

    /**
     * @notice Pauses the minting of the token. Restricted to the pauser role.
     */
    function pauseMinting() external onlyRole(MINTING_PAUSER_ROLE) {
        _pause();
    }

    /**
     * @notice Unpauses the minting of the token. Restricted to the admin role.
     */
    function unpauseMinting() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}
