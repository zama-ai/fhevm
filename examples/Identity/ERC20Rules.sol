// SPDX-License-Identifier: BSD-3-Clause-Clear

// Owner = ONU
// Issuer par pays
// Did associé à un issuer

pragma solidity 0.8.19;

import "./Identity.sol";

import "../../lib/TFHE.sol";

contract ERC20Rules {
    address immutable _this;

    constructor() {
        _this = address(this);
    }

    function transfer(
        Identity identityContract,
        address from,
        address to,
        euint32 _amount
    ) public view returns (euint32) {
        require(address(this) != _this, "isTransferable must be called with delegatecall");

        // Condition 1: 10k limit between two different countries
        euint8 fromCountry = identityContract.getCountry(from);
        euint8 toCountry = identityContract.getCountry(to);
        require(TFHE.isInitialized(fromCountry) && TFHE.isInitialized(toCountry), "You don't have access");
        ebool sameCountry = TFHE.eq(fromCountry, toCountry);
        ebool amountAbove10k = TFHE.gt(_amount, 10000);
        ebool countryCondition = TFHE.cmux(
            sameCountry,
            TFHE.asEbool(true),
            TFHE.cmux(amountAbove10k, TFHE.asEbool(false), TFHE.asEbool(true))
        );

        // Condition 2: Check that noone is blacklisted
        ebool fromBlacklisted = TFHE.asEbool(identityContract.getIdentifier(from, "blacklist"));
        ebool toBlacklisted = TFHE.asEbool(identityContract.getIdentifier(to, "blacklist"));
        ebool whitelisted = TFHE.not(TFHE.and(toBlacklisted, fromBlacklisted));

        euint32 amount = TFHE.cmux(
            countryCondition,
            TFHE.cmux(whitelisted, _amount, TFHE.asEuint32(0)),
            TFHE.asEuint32(0)
        );

        return amount;
    }
}
