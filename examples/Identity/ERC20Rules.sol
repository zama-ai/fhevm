// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.19;

import "../../lib/TFHE.sol";
import "./IdentityRegistry.sol";

contract ERC20Rules {
    address immutable _this;

    string[] public identifiers;

    mapping(address => uint32) public countryWallets;
    mapping(string => uint8) public countries;
    uint16[] public country2CountryRestrictions;

    constructor() {
        _this = address(this);
        identifiers = ["country", "blacklist"];
        countryWallets[address(0x133725C6461120439E85887C7639316CB27a2D9d)] = 1;
        countryWallets[address(0x4CaCeF78615AFecEf7eF182CfbD243195Fc90a29)] = 2;

        countries["fr"] = 1;
        countries["us"] = 2;

        country2CountryRestrictions = [createRestriction(countries["us"], countries["fr"])];
    }

    function createRestriction(uint16 from, uint16 to) internal pure returns (uint16) {
        return (from << 8) + to;
    }

    function getIdentifiers() public view returns (string[] memory) {
        return identifiers;
    }

    function getC2CRestrictions() public view returns (uint16[] memory) {
        return country2CountryRestrictions;
    }

    function transfer(
        IdentityRegistry identityContract,
        ERC20Rules rulesContract,
        address from,
        address to,
        euint32 amount
    ) public view returns (euint32) {
        require(address(this) != _this, "transfer must be called with delegatecall");

        // Condition 1: 10k limit between two different countries
        ebool transferLimitOK = checkLimitTransfer(identityContract, from, to, amount);

        ebool condition = transferLimitOK;

        // Condition 2: Check that noone is blacklisted
        ebool blacklistOK = checkBlacklist(identityContract, from, to);

        condition = TFHE.and(condition, blacklistOK);

        // Condition 3: Check country to country rules
        ebool c2cOK = checkCountryToCountry(identityContract, rulesContract, from, to);

        condition = TFHE.and(condition, c2cOK);

        return TFHE.cmux(condition, amount, TFHE.asEuint32(0));
    }

    function checkLimitTransfer(
        IdentityRegistry identityContract,
        address from,
        address to,
        euint32 amount
    ) internal view returns (ebool) {
        euint8 fromCountry = TFHE.asEuint8(identityContract.getIdentifier(from, "country"));
        euint8 toCountry = TFHE.asEuint8(identityContract.getIdentifier(to, "country"));
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
        ERC20Rules rulesContract,
        address from,
        address to
    ) internal view returns (ebool) {
        // Disallow transfer from country 2 to country 1
        uint16[] memory c2cRestrictions = rulesContract.getC2CRestrictions();

        euint32 fromCountry = identityContract.getIdentifier(from, "country");
        euint32 toCountry = identityContract.getIdentifier(to, "country");
        require(TFHE.isInitialized(fromCountry) && TFHE.isInitialized(toCountry), "You don't have access");
        euint16 countryToCountry = TFHE.shl(TFHE.asEuint16(fromCountry), 8) + TFHE.asEuint16(toCountry);
        ebool condition = TFHE.asEbool(true);

        // Check all countryToCountry restrictions
        for (uint i = 0; i < c2cRestrictions.length; i++) {
            condition = TFHE.and(condition, TFHE.ne(countryToCountry, c2cRestrictions[i]));
        }

        return condition;
    }
}
