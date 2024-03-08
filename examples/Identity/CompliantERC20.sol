// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "../../lib/TFHE.sol";
import "../../abstracts/Reencrypt.sol";
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

    function balanceOf(
        address wallet,
        bytes32 publicKey,
        bytes calldata signature
    ) public view override onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
        if (wallet == msg.sender) {
            return TFHE.reencrypt(balances[msg.sender], publicKey, 0);
        }

        uint64 userCountry = rulesContract.whitelistedWallets(msg.sender);
        require(userCountry > 0, "You're not registered as a country wallet");

        euint64 walletCountry = identityContract.getIdentifier(wallet, "country");
        ebool sameCountry = TFHE.eq(walletCountry, userCountry);
        euint64 balance = TFHE.isInitialized(balances[wallet]) ? balances[wallet] : TFHE.asEuint64(0);
        balance = TFHE.select(sameCountry, balance, TFHE.asEuint64(0));

        return TFHE.reencrypt(balance, publicKey, 0);
    }

    // Transfers an encrypted amount.
    function _transfer(address from, address to, euint64 _amount, ebool isTransferable) internal override {
        // Condition 1: hasEnoughFunds and hasEnoughAllowance (classical ERC20)
        euint64 amount = TFHE.select(isTransferable, _amount, TFHE.asEuint64(0));

        amount = rulesContract.transfer(from, to, amount);

        balances[to] = balances[to] + amount;
        balances[from] = balances[from] - amount;
        emit Transfer(from, to);
    }
}
