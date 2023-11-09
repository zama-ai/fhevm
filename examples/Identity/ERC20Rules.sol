// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.19;

import "../../lib/TFHE.sol";
import "./IdentityRegistry.sol";

contract ERC20Rules {
    address immutable _this;

    string[] public identifiers;

    constructor() {
        _this = address(this);
        identifiers = ["country", "blacklist"];
    }

    function getIdentifiers() public view returns (string[] memory) {
        return identifiers;
    }

    function transfer(
        IdentityRegistry identityContract,
        address from,
        address to,
        euint32 amount
    ) public view returns (euint32) {
        require(address(this) != _this, "isTransferable must be called with delegatecall");

        // Condition 1: 10k limit between two different countries
        ebool transferLimitOK = checkLimitTransfer(identityContract, from, to, amount);

        ebool condition = transferLimitOK;

        // Condition 2: Check that noone is blacklisted
        ebool blacklistOK = checkBlacklist(identityContract, from, to);

        condition = TFHE.and(condition, blacklistOK);

        // Condition 3: Check country to country rules
        ebool c2cOK = checkCountryToCountry(identityContract, from, to);

        condition = TFHE.and(condition, c2cOK);

        return TFHE.cmux(condition, amount, TFHE.asEuint32(0));
    }

    function checkLimitTransfer(
        IdentityRegistry identityContract,
        address from,
        address to,
        euint32 amount
    ) internal view returns (ebool) {
        euint32 fromCountry = identityContract.getIdentifier(from, "country");
        euint32 toCountry = identityContract.getIdentifier(to, "country");
        require(TFHE.isInitialized(fromCountry) && TFHE.isInitialized(toCountry), "You don't have access");
        ebool sameCountry = TFHE.eq(fromCountry, toCountry);
        ebool amountBelow10k = TFHE.le(amount, 10000);

        return TFHE.or(sameCountry, amountBelow10k);
    }

    function checkBlacklist(IdentityRegistry identityContract, address from, address to) internal view returns (ebool) {
        ebool fromBlacklisted = TFHE.asEbool(identityContract.getIdentifier(from, "blacklist"));
        ebool toBlacklisted = TFHE.asEbool(identityContract.getIdentifier(to, "blacklist"));
        return TFHE.not(TFHE.and(toBlacklisted, fromBlacklisted));
    }

    function checkCountryToCountry(
        IdentityRegistry identityContract,
        address from,
        address to
    ) internal view returns (ebool) {
        // Disallow transfer from country 2 to country 1
        euint16[1] memory c2cRestrictions = [TFHE.shl(TFHE.asEuint16(2), 8) + TFHE.asEuint16(1)];

        euint32 fromCountry = identityContract.getIdentifier(from, "country");
        euint32 toCountry = identityContract.getIdentifier(to, "country");
        require(TFHE.isInitialized(fromCountry) && TFHE.isInitialized(toCountry), "You don't have access");
        euint16 countryToCountry = TFHE.shl(TFHE.asEuint16(fromCountry), 8) + TFHE.asEuint16(toCountry);
        ebool condition = TFHE.asEbool(true);

        // Check all countryToCountry restrictions
        for (uint i = 0; i < c2cRestrictions.length; i++) {
            condition = TFHE.and(condition, TFHE.ne(c2cRestrictions[i], countryToCountry));
        }

        return condition;
    }
}
