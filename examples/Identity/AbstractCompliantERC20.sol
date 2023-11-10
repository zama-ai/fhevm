// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.19;

import "../../abstracts/EIP712WithModifier.sol";
import "./ERC20Rules.sol";
import "./IdentityRegistry.sol";

abstract contract AbstractCompliantERC20 is EIP712WithModifier {
    mapping(address => euint32) internal balances;
    IdentityRegistry identityContract;
    ERC20Rules rulesContract;

    constructor(address _identityAddr, address _rulesAddr) EIP712WithModifier("Authorization token", "1") {
        identityContract = IdentityRegistry(_identityAddr);
        rulesContract = ERC20Rules(_rulesAddr);
    }

    function identifiers() public view returns (string[] memory) {
        return rulesContract.getIdentifiers();
    }

    function balanceOfUser(
        address wallet,
        bytes32 publicKey,
        bytes calldata signature
    ) public view onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
        uint32 userCountry = rulesContract.countryWallets(msg.sender);
        require(userCountry > 0, "You're not registered as a country wallet");

        euint32 walletCountry = identityContract.getIdentifier(wallet, "country");
        ebool sameCountry = TFHE.eq(walletCountry, userCountry);
        euint32 balance = TFHE.isInitialized(balances[wallet]) ? balances[wallet] : TFHE.asEuint32(0);
        balance = TFHE.cmux(sameCountry, balance, TFHE.asEuint32(0));

        return TFHE.reencrypt(balance, publicKey, 0);
    }

    // Transfers an encrypted amount.
    function _transfer(address from, address to, euint32 _amount) internal {
        // Condition 1: hasEnoughFunds
        ebool enoughFund = TFHE.le(_amount, balances[from]);
        euint32 amount = TFHE.cmux(enoughFund, _amount, TFHE.asEuint32(0));

        // Delegate call
        (bool success, bytes memory returndata) = address(rulesContract).delegatecall(
            abi.encodeWithSelector(ERC20Rules.transfer.selector, identityContract, from, to, amount)
        );
        require(success == true);
        amount = abi.decode(returndata, (euint32));

        balances[to] = balances[to] + amount;
        balances[from] = balances[from] - amount;
    }
}
