// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "@openzeppelin/contracts/access/Ownable.sol";
import {IPauserSet} from "@fhevm/host-contracts/contracts/interfaces/IPauserSet.sol";

/// @dev Just for testing, use the real PauserSet from Ethereum host-contracts on mainnet
contract PauserSetMock is Ownable, IPauserSet {
    mapping(address account => bool isPauser) pausers;

    constructor() Ownable(msg.sender) {}

    function addPauser(address account) external onlyOwner {
        if (account == address(0)) revert InvalidNullPauser();
        if (pausers[account]) revert AccountAlreadyPauser(account);
        pausers[account] = true;
        emit AddPauser(account);
    }

    function removePauser(address account) external onlyOwner {
        if (account == address(0)) revert InvalidNullPauser();
        if (!pausers[account]) revert AccountNotPauser(account);
        pausers[account] = false;
        emit RemovePauser(account);
    }

    function swapPauser(address oldAccount, address newAccount) external onlyOwner {
        if (oldAccount == address(0) || newAccount == address(0)) revert InvalidNullPauser();
        if (!pausers[oldAccount]) revert AccountNotPauser(oldAccount);
        if (pausers[newAccount]) revert AccountAlreadyPauser(newAccount);
        pausers[oldAccount] = false;
        pausers[newAccount] = true;
        emit SwapPauser(oldAccount, newAccount);
    }

    function isPauser(address account) external view returns (bool) {
        return pausers[account];
    }

    function getVersion() external pure returns (string memory){
        return "PauserSetMock";
    }
}
