// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.22;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { ERC20Burnable } from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import { ERC20Permit } from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import { ERC1363 } from "@openzeppelin/contracts/token/ERC20/extensions/ERC1363.sol";
import { Pausable } from "@openzeppelin/contracts/utils/Pausable.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IERC721 } from "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import { IERC1155 } from "@openzeppelin/contracts/token/ERC1155/IERC1155.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { AccessControl } from "@openzeppelin/contracts/access/AccessControl.sol";

contract ZamaERC20 is ERC20, ERC20Permit, ERC1363, ERC20Burnable, AccessControl, Pausable {
    using SafeERC20 for IERC20;

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant MINTING_PAUSER_ROLE = keccak256("MINTING_PAUSER_ROLE");

    event EtherRecovered(address indexed recipient, uint256 amount);
    event ERC20Recovered(address indexed token, address indexed recipient, uint256 amount);
    event ERC721Recovered(address indexed token, uint256 tokenId, address indexed recipient);
    event ERC1155Recovered(address indexed token, uint256 tokenId, address indexed recipient, uint256 amount);

    error FailedToSendEther();

    constructor(
        string memory name,
        string memory symbol,
        address initialSupplyReceiver,
        address initialAdmin
    ) ERC20(name, symbol) ERC20Permit(name) {
        _mint(initialSupplyReceiver, 11_000_000_000 * 1e18);
        _grantRole(DEFAULT_ADMIN_ROLE, initialAdmin);
    }

    /**
     * @notice Only a minter could mint new tokens.
     * @param to Receiver of the newly minted tokens.
     * @param amount Number of tokens to mint.
     */
    function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE) whenNotPaused {
        _mint(to, amount);
    }

    /**
     * @dev Triggers minting paused state.
     * Only a MINTING_PAUSER address can pause minting.
     * The contract must not be paused for minting.
     */
    function pauseMinting() external onlyRole(MINTING_PAUSER_ROLE) {
        _pause();
    }

    /**
     * @dev Returns to normal state.
     * Only DEFAULT_ADMIN_ROLE can unpause minting.
     * The contract must be paused for minting.
     */
    function unpauseMinting() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }

    /**
     * @dev Allows the sender to recover Ether held by the contract.
     * @param amount Amount of recovered ETH.
     * @param recipient Receiver of the recovered ETH.
     * Emits an EtherRecovered event upon success.
     */
    function recoverEther(uint256 amount, address recipient) external onlyRole(DEFAULT_ADMIN_ROLE) {
        (bool success, ) = recipient.call{ value: amount }("");
        if (!success) {
            revert FailedToSendEther();
        }
        emit EtherRecovered(recipient, amount);
    }

    /**
     * @dev Allows the sender to recover ERC20 tokens held by the contract.
     * @param token The address of the ERC20 token to recover.
     * @param amount The amount of the ERC20 token to recover.
     * @param recipient Receiver of the recovered tokens.
     * Emits an ERC20Recovered event upon success.
     */
    function recoverERC20(address token, uint256 amount, address recipient) external onlyRole(DEFAULT_ADMIN_ROLE) {
        IERC20(token).safeTransfer(recipient, amount);
        emit ERC20Recovered(token, recipient, amount);
    }

    /**
     * @dev Allows the sender to recover ERC721 tokens held by the contract.
     * @param token The address of the ERC721 token to recover.
     * @param tokenId The token ID of the ERC721 token to recover.
     * @param recipient Receiver of the recovered ERC721 token.
     * Emits an ERC721Recovered event upon success.
     */
    function recoverERC721(address token, uint256 tokenId, address recipient) external onlyRole(DEFAULT_ADMIN_ROLE) {
        IERC721(token).safeTransferFrom(address(this), recipient, tokenId);
        emit ERC721Recovered(token, tokenId, recipient);
    }

    /**
     * @dev Allows the sender to recover ERC1155 tokens held by the contract.
     * @param token The address of the ERC1155 token to recover.
     * @param tokenId The token ID of the ERC1155 token to recover.
     * @param recipient Receiver of the recovered ERC1155 token.
     * Emits an ERC1155Recovered event upon success.
     */
    function recoverERC1155(address token, uint256 tokenId, address recipient) external onlyRole(DEFAULT_ADMIN_ROLE) {
        uint256 amount = IERC1155(token).balanceOf(address(this), tokenId);
        IERC1155(token).safeTransferFrom({ from: address(this), to: recipient, id: tokenId, value: amount, data: "" });
        emit ERC1155Recovered(token, tokenId, recipient, amount);
    }

    /**
     * @dev See {IERC165-supportsInterface}.
     */
    function supportsInterface(bytes4 interfaceId) public view override(ERC1363, AccessControl) returns (bool) {
        return super.supportsInterface(interfaceId);
    }
}
