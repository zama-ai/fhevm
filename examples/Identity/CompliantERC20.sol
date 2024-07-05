// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../EncryptedERC20.sol";
import "./ERC20Rules.sol";
import "./IdentityRegistry.sol";

contract CompliantERC20 is EncryptedERC20 {
    IdentityRegistry identityContract;
    ERC20Rules rulesContract;

    constructor(
        address _identityAddr,
        address _rulesAddr,
        string memory _name,
        string memory _symbol
    ) EncryptedERC20(_name, _symbol) {
        identityContract = IdentityRegistry(_identityAddr);
        rulesContract = ERC20Rules(_rulesAddr);
    }

    function identifiers() public view returns (string[] memory) {
        return rulesContract.getIdentifiers();
    }

    function getIdentifier(address wallet, string calldata identifier) external view returns (euint64) {
        require(msg.sender == address(rulesContract), "Access restricted to the current ERC20Rules");
        return identityContract.getIdentifier(wallet, identifier);
    }

    // Transfers an encrypted amount.
    function _transfer(address from, address to, euint64 _amount, ebool isTransferable) internal override {
        // Condition 1: hasEnoughFunds and hasEnoughAllowance (classical ERC20)
        euint64 amount = TFHE.select(isTransferable, _amount, TFHE.asEuint64(0));

        amount = rulesContract.transfer(from, to, amount);

        balances[to] = TFHE.add(balances[to], amount);
        balances[from] = TFHE.sub(balances[from], amount);
        emit Transfer(from, to);
    }
}
