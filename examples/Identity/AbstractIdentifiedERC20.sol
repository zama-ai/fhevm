// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.19;

import "../../abstracts/EIP712WithModifier.sol";
import "./ERC20Rules.sol";
import "./IdentityRegistry.sol";

abstract contract AbstractIdentifiedERC20 is EIP712WithModifier {
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

    function balanceOf(
        address wallet,
        bytes32 publicKey,
        bytes calldata signature
    ) public view onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
        uint registrarId = identityContract.registrars(msg.sender);
        require(registrarId > 0, "You're not a registrar");
        uint userRegistrarId = identityContract.getRegistrar(wallet);
        require(userRegistrarId == registrarId, "You're not managing this did");

        return TFHE.reencrypt(balances[wallet], publicKey, 0);
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
