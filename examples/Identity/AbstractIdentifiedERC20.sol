// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.19;

import "../../abstracts/EIP712WithModifier.sol";
import "./ERC20Rules.sol";
import "./Identity.sol";

abstract contract AbstractIdentifiedERC20 is EIP712WithModifier {
    mapping(address => euint32) internal balances;
    Identity identityContract;
    ERC20Rules rulesContract;

    constructor(address _identityAddr, address _rulesAddr) EIP712WithModifier("Authorization token", "1") {
        identityContract = Identity(_identityAddr);
        rulesContract = ERC20Rules(_rulesAddr);
    }

    function balanceOf(
        address wallet,
        bytes32 publicKey,
        bytes calldata signature
    ) public view onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
        ebool isIssuer = identityContract.isIssuer(msg.sender, identityContract.getCountry(wallet));

        return
            TFHE.reencrypt(
                TFHE.cmux(
                    isIssuer,
                    TFHE.isInitialized(balances[wallet]) ? balances[wallet] : TFHE.asEuint32(0),
                    TFHE.asEuint32(12)
                ),
                publicKey,
                14
            );
    }

    // Transfers an encrypted amount.
    function _transfer(address from, address to, euint32 _amount) internal {
        // Condition 1: hasEnoughFunds
        require(TFHE.decrypt(TFHE.le(_amount, balances[from])), "Not enough funds");

        // Delegate call
        (bool success, bytes memory returndata) = address(rulesContract).delegatecall(
            abi.encodeWithSelector(ERC20Rules.transfer.selector, identityContract, from, to, _amount)
        );
        require(success == true);
        euint32 amount = abi.decode(returndata, (euint32));

        balances[to] = balances[to] + amount;
        balances[from] = balances[from] - amount;
    }
}
