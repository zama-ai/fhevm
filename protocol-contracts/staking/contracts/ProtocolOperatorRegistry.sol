// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev This contract creates a registry for operator to indicate which account holds their staking account.
 * @custom:security-contact security@zama.ai
 */
contract ProtocolOperatorRegistry {
    /// @custom:storage-location erc7201:zama.storage.ProtocolOperatorRegistry
    struct ProtocolOperatorRegistryStorage {
        mapping(address operator => address) _stakingAccounts;
        mapping(address stakingAccount => address) _operators;
    }

    // keccak256(abi.encode(uint256(keccak256("zama.storage.ProtocolOperatorRegistry")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PROTOCOL_OPERATOR_REGISTRY_STORAGE_LOCATION =
        0xf4991778404f39da1b7149b42e8195e0a86139aeb8fe7585bc5520f58085de00;

    /// @dev Emitted when the staking account is set for a given `operator`.
    event StakingAccountSet(
        address indexed operator,
        address indexed previousStakingAccount,
        address indexed newStakingAccount
    );

    /// @dev The caller is not the owner of the staking account.
    error StakingAccountNotOwnedByCaller();

    /**
     * @dev Sets the staking account for an operator `msg.sender`. Operators may unset their
     * staking account by calling this function with `address(0)`.
     * @param account The staking account being set by the owner.
     *
     * Requirements:
     *
     * - `msg.sender` must be the {Ownable-owner} of `account`.
     */
    function setStakingAccount(address account) public {
        ProtocolOperatorRegistryStorage storage $ = _getProtocolOperatorRegistryStorage();
        if (account != address(0)) {
            require(Ownable(account).owner() == msg.sender, StakingAccountNotOwnedByCaller());
            address oldOwner = operator(account);
            if (oldOwner != address(0)) {
                _setStakingAccount(oldOwner, account, address(0)); // unset staking account of old owner
            }
            $._operators[account] = msg.sender;
        }

        address currentStakingAccount = stakingAccount(msg.sender);
        if (currentStakingAccount != address(0)) {
            $._operators[currentStakingAccount] = address(0);
        }
        _setStakingAccount(msg.sender, currentStakingAccount, account);
    }

    /**
     * @dev Gets the staking account associated with an operator account.
     * @param account The operator account.
     * @return The staking account.
     */
    function stakingAccount(address account) public view returns (address) {
        return _getProtocolOperatorRegistryStorage()._stakingAccounts[account];
    }

    /**
     * @dev Gets operator account associated with a given staking account.
     * @param account The staking account.
     * @return The operator.
     */
    function operator(address account) public view returns (address) {
        return _getProtocolOperatorRegistryStorage()._operators[account];
    }

    /// @dev Sets the staking account of an operator.
    function _setStakingAccount(address operator_, address oldStakingAccount, address newStakingAccount) private {
        _getProtocolOperatorRegistryStorage()._stakingAccounts[operator_] = newStakingAccount;
        emit StakingAccountSet(operator_, oldStakingAccount, newStakingAccount);
    }

    function _getProtocolOperatorRegistryStorage() private pure returns (ProtocolOperatorRegistryStorage storage $) {
        assembly {
            $.slot := PROTOCOL_OPERATOR_REGISTRY_STORAGE_LOCATION
        }
    }
}
