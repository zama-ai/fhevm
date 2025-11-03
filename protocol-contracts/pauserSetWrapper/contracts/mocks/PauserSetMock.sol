// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "@openzeppelin/contracts/access/Ownable.sol";
import { IPauserSet } from "@fhevm/host-contracts/contracts/interfaces/IPauserSet.sol";

/**
 * @title PauserSetMock
 * @notice This contract is a mock of the PauserSet contract from host-contracts. It is used for
 * testing purposes and shouldn't be used in testnet or mainnet.
 */
contract PauserSetMock is Ownable, IPauserSet {
    /**
     * @notice Mapping of pausers.
     */
    mapping(address account => bool isPauser) pausers;

    constructor() Ownable(msg.sender) {}

    /**
     * @notice Adds a new pauser. Restricted to the owner.
     * @param account The address of the new pauser.
     */
    function addPauser(address account) external onlyOwner {
        if (account == address(0)) revert InvalidNullPauser();
        if (pausers[account]) revert AccountAlreadyPauser(account);
        pausers[account] = true;
        emit AddPauser(account);
    }

    /**
     * @notice Removes a pauser. Restricted to the owner.
     * @param account The address of the pauser to remove.
     */
    function removePauser(address account) external onlyOwner {
        if (account == address(0)) revert InvalidNullPauser();
        if (!pausers[account]) revert AccountNotPauser(account);
        pausers[account] = false;
        emit RemovePauser(account);
    }

    /**
     * @notice Swaps a pauser. Restricted to the owner.
     * @param oldAccount The address of the old pauser.
     * @param newAccount The address of the new pauser.
     */
    function swapPauser(address oldAccount, address newAccount) external onlyOwner {
        if (oldAccount == address(0) || newAccount == address(0)) revert InvalidNullPauser();
        if (!pausers[oldAccount]) revert AccountNotPauser(oldAccount);
        if (pausers[newAccount]) revert AccountAlreadyPauser(newAccount);
        pausers[oldAccount] = false;
        pausers[newAccount] = true;
        emit SwapPauser(oldAccount, newAccount);
    }

    /**
     * @notice Checks if an address is a pauser.
     * @param account The address to check.
     * @return True if the address is a pauser, false otherwise.
     */
    function isPauser(address account) external view returns (bool) {
        return pausers[account];
    }

    /**
     * @notice Returns the version of the contract.
     * @return The version of the contract.
     */
    function getVersion() external pure returns (string memory) {
        return "0.0.1";
    }
}
