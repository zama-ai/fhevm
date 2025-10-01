// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { IPauserSet } from "../interfaces/IPauserSet.sol";
import { GatewayOwnable } from "../shared/GatewayOwnable.sol";
import { gatewayConfigAddress } from "../../addresses/GatewayAddresses.sol";

/**
 * @title PauserSet smart contract
 * @dev See {IPauserSet}
 */
contract PauserSet is IPauserSet, GatewayOwnable {
    /// @dev The following constants are used for versioning the contract. They are made private
    /// @dev in order to force derived contracts to consider a different version. Note that
    /// @dev they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "PauserSet";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    mapping(address account => bool isPauser) pausers;

    /// @dev See {IPauserSet-addPauser}.
    function addPauser(address account) external onlyGatewayOwner {
        if (account == address(0)) revert InvalidNullPauser();
        if (pausers[account]) revert AccountAlreadyPauser(account);
        pausers[account] = true;
        emit AddPauser(account);
    }

    /// @dev See {IPauserSet-removePauser}.
    function removePauser(address account) external onlyGatewayOwner {
        if (account == address(0)) revert InvalidNullPauser();
        if (!pausers[account]) revert AccountNotPauser(account);
        pausers[account] = false;
        emit RemovePauser(account);
    }

    /// @dev See {IPauserSet-swapPauser}.
    function swapPauser(address oldAccount, address newAccount) external onlyGatewayOwner {
        if (oldAccount == address(0) || newAccount == address(0)) revert InvalidNullPauser();
        if (!pausers[oldAccount]) revert AccountNotPauser(oldAccount);
        if (pausers[newAccount]) revert AccountAlreadyPauser(newAccount);
        pausers[oldAccount] = false;
        pausers[newAccount] = true;
        emit SwapPauser(oldAccount, newAccount);
    }

    /// @dev See {IPauserSet-isPauser}.
    function isPauser(address account) external view returns (bool) {
        return pausers[account];
    }

    /// @dev See {IPauserSet-getVersion}.
    function getVersion() external pure returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    " v",
                    Strings.toString(MAJOR_VERSION),
                    ".",
                    Strings.toString(MINOR_VERSION),
                    ".",
                    Strings.toString(PATCH_VERSION)
                )
            );
    }
}
