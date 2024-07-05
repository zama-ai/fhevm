// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "./IdentityRegistry.sol";

interface ICompliantERC20 {
    function getIdentifier(address wallet, string calldata identifier) external view returns (euint64);
}

contract ERC20Rules {
    string[] public identifiers;

    mapping(address => uint64) public whitelistedWallets;
    mapping(string => uint8) public countries;
    uint16[] public country2CountryRestrictions;

    constructor() {
        identifiers = ["country", "blacklist"];
        whitelistedWallets[address(0x133725C6461120439E85887C7639316CB27a2D9d)] = 1;
        whitelistedWallets[address(0x4CaCeF78615AFecEf7eF182CfbD243195Fc90a29)] = 2;

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

    function transfer(address from, address to, euint64 amount) public returns (euint64) {
        ICompliantERC20 erc20 = ICompliantERC20(msg.sender);
        // Condition 1: 10k limit between two different countries
        ebool transferLimitOK = checkLimitTransfer(erc20, from, to, amount);

        ebool condition = transferLimitOK;

        // Condition 2: Check that noone is blacklisted
        ebool blacklistOK = checkBlacklist(erc20, from, to);

        condition = TFHE.and(condition, blacklistOK);

        // Condition 3: Check country to country rules
        ebool c2cOK = checkCountryToCountry(erc20, from, to);

        condition = TFHE.and(condition, c2cOK);

        return TFHE.select(condition, amount, TFHE.asEuint64(0));
    }

    function checkLimitTransfer(
        ICompliantERC20 erc20,
        address from,
        address to,
        euint64 amount
    ) internal returns (ebool) {
        euint8 fromCountry = TFHE.asEuint8(erc20.getIdentifier(from, "country"));
        euint8 toCountry = TFHE.asEuint8(erc20.getIdentifier(to, "country"));
        require(TFHE.isInitialized(fromCountry) && TFHE.isInitialized(toCountry), "You don't have access");
        ebool sameCountry = TFHE.eq(fromCountry, toCountry);
        ebool amountBelow10k = TFHE.le(amount, 10000);

        return TFHE.or(sameCountry, amountBelow10k);
    }

    function checkBlacklist(ICompliantERC20 erc20, address from, address to) internal returns (ebool) {
        ebool fromBlacklisted = TFHE.asEbool(erc20.getIdentifier(from, "blacklist"));
        ebool toBlacklisted = TFHE.asEbool(erc20.getIdentifier(to, "blacklist"));
        return TFHE.not(TFHE.or(toBlacklisted, fromBlacklisted));
    }

    function checkCountryToCountry(ICompliantERC20 erc20, address from, address to) internal returns (ebool) {
        // Disallow transfer from country 2 to country 1
        uint16[] memory c2cRestrictions = getC2CRestrictions();

        euint64 fromCountry = erc20.getIdentifier(from, "country");
        euint64 toCountry = erc20.getIdentifier(to, "country");
        require(TFHE.isInitialized(fromCountry) && TFHE.isInitialized(toCountry), "You don't have access");
        euint16 countryToCountry = TFHE.add(TFHE.shl(TFHE.asEuint16(fromCountry), 8), TFHE.asEuint16(toCountry));
        ebool condition = TFHE.asEbool(true);

        // Check all countryToCountry restrictions
        for (uint i = 0; i < c2cRestrictions.length; i++) {
            condition = TFHE.and(condition, TFHE.ne(countryToCountry, c2cRestrictions[i]));
        }

        return condition;
    }
}
