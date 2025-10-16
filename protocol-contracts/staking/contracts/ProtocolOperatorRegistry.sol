// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @dev This contract creates a registry for operator to indicate which account holds their staked tokens.
contract ProtocolOperatorRegistry {
    /// @custom:storage-location erc7201:zama.storage.ProtocolOperatorRegistry
    struct ProtocolOperatorRegistryStorage {
        mapping(address => address) _operatorToStakedTokens;
        mapping(address => address) _stakedTokensToOperator;
    }

    // keccak256(abi.encode(uint256(keccak256("zama.storage.ProtocolOperatorRegistry")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PROTOCOL_OPERATOR_REGISTRY_STORAGE_LOCATION =
        0xf4991778404f39da1b7149b42e8195e0a86139aeb8fe7585bc5520f58085de00;

    event StakedTokensAccountSet(
        address indexed operator,
        address indexed previousStakedTokensAccount,
        address indexed newStakedTokensAccount
    );

    error StakingAccountNotOwnedByCaller();
    error StakingAccountAlreadyRegistered();

    /**
     * @dev Sets the staked tokens account for an operator `msg.sender`. Operators may unset their
     * staked tokens account by calling this function with `address(0)`.
     *
     * Requirements:
     *
     * - `msg.sender` must be the {Ownable-owner} of `account`.
     * - `account` must not already be claimed by another operator.
     */
    function setStakedTokensAccount(address account) public virtual {
        ProtocolOperatorRegistryStorage storage $ = _getProtocolOperatorRegistryStorage();
        if (account != address(0)) {
            require(Ownable(account).owner() == msg.sender, StakingAccountNotOwnedByCaller());
            require(operator(account) == address(0), StakingAccountAlreadyRegistered());

            $._stakedTokensToOperator[account] = msg.sender;
        }

        address currentStakedTokensAccount = stakedTokens(msg.sender);
        if (currentStakedTokensAccount != address(0)) {
            $._stakedTokensToOperator[currentStakedTokensAccount] = address(0);
        }
        $._operatorToStakedTokens[msg.sender] = account;

        emit StakedTokensAccountSet(msg.sender, currentStakedTokensAccount, account);
    }

    /// @dev Staked tokens account associated with a given operator account.
    function stakedTokens(address account) public view returns (address) {
        return _getProtocolOperatorRegistryStorage()._operatorToStakedTokens[account];
    }

    /// @dev Gets operator account associated with a given staked tokens account.
    function operator(address account) public view returns (address) {
        return _getProtocolOperatorRegistryStorage()._stakedTokensToOperator[account];
    }

    function _getProtocolOperatorRegistryStorage() private pure returns (ProtocolOperatorRegistryStorage storage $) {
        assembly {
            $.slot := PROTOCOL_OPERATOR_REGISTRY_STORAGE_LOCATION
        }
    }
}
