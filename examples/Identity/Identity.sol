// SPDX-License-Identifier: BSD-3-Clause-Clear

// Owner = ONU
// Issuer par pays
// Did associé à un issuer

pragma solidity 0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

import "../../abstracts/EIP712WithModifier.sol";

import "../../lib/TFHE.sol";

contract Identity is EIP712WithModifier, Ownable {
    // A mapping from wallet to an identity.
    mapping(address => UserIdentity) internal identities;

    struct UserIdentity {
        euint8 country;
        ebool issuer;
        mapping(string => euint32) identifiers;
    }

    mapping(address => mapping(address => mapping(string => bool))) permissions; // users => contracts => identifiers[]

    event NewDid(address wallet);
    event RemoveDid(address wallet);

    constructor() Ownable() EIP712WithModifier("Authorization token", "1") {
        _transferOwnership(msg.sender);
    }

    function changeOwner(address currentOwner, address newOwner) public onlyOwner {
        require(TFHE.isInitialized(identities[newOwner].country), "Address already owns a wallet");
        UserIdentity storage ident = identities[currentOwner];
        UserIdentity storage newIdent = identities[newOwner];
        newIdent = ident;
        delete identities[currentOwner];
    }

    function setIssuer(address wallet, bytes calldata encryptedIssuer) public onlyOwner {
        require(TFHE.isInitialized(identities[wallet].country), "This wallet is already registered");
        ebool issuer = TFHE.asEbool(encryptedIssuer);
        identities[wallet].issuer = issuer;
    }

    // Add user
    function addDid(address wallet, bytes calldata encryptedCountry, bytes calldata encryptedIssuer) public {
        require(!TFHE.isInitialized(identities[wallet].country), "This wallet is already registered");
        euint8 country = TFHE.asEuint8(encryptedCountry);
        ebool issuer = TFHE.asEbool(encryptedIssuer);
        addDid(wallet, country, issuer);
    }

    function addDid(address wallet, euint8 country, ebool issuer) internal onlyIssuer(country) {
        UserIdentity storage newIdentity = identities[wallet];
        newIdentity.country = country;
        newIdentity.issuer = issuer;

        emit NewDid(wallet);
    }

    function removeDid(address wallet) public {
        require(TFHE.isInitialized(identities[wallet].country), "This wallet isn't registered");
        euint8 country = identities[wallet].country;
        removeDid(wallet, country);
    }

    function removeDid(address wallet, euint8 country) internal onlyIssuer(country) {
        delete identities[wallet];

        emit RemoveDid(wallet);
    }

    // Set user's identifiers
    function setIdentifier(address wallet, string calldata identifier, bytes calldata encryptedValue) public {
        euint32 value = TFHE.asEuint32(encryptedValue);
        setIdentifier(wallet, identifier, value);
    }

    function setIdentifier(
        address wallet,
        string calldata identifier,
        euint32 value
    ) internal onlyExistingWallet(wallet) {
        require(
            !Strings.equal(identifier, "issuer") && !Strings.equal(identifier, "country"),
            "Reserved identifier name"
        );
        euint8 country = identities[wallet].country;
        _setIdentifier(wallet, identifier, value, country);
    }

    function _setIdentifier(
        address wallet,
        string calldata identifier,
        euint32 value,
        euint8 country
    ) internal onlyIssuer(country) {
        identities[wallet].identifiers[identifier] = value;
    }

    // User handling permission permission
    function grantAccess(address allowed, string calldata identifier) public {
        permissions[msg.sender][allowed][identifier] = true;
    }

    function revokeAccess(address allowed, string calldata identifier) public {
        delete permissions[msg.sender][allowed][identifier];
    }

    // Get encrypted country

    function getCountry(address wallet) public view returns (euint8) {
        return _getCountry(wallet);
    }

    function _getCountry(
        address wallet
    ) internal view onlyExistingWallet(wallet) onlyAllowed(wallet, "country") returns (euint8) {
        return identities[wallet].country;
    }

    function isIssuer(
        address wallet,
        euint8 country
    ) public view onlyAllowed(wallet, "country") onlyAllowed(wallet, "issuer") returns (ebool) {
        return _isIssuer(wallet, country);
    }

    // Get encrypted identifiers
    function reencryptIdentifier(
        address wallet,
        string calldata identifier,
        bytes32 publicKey,
        bytes calldata signature
    ) public view onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
        euint32 ident = _getIdentifier(wallet, identifier);
        require(TFHE.isInitialized(ident), "This identifier is unknown");

        return TFHE.reencrypt(ident, publicKey, 0);
    }

    function getIdentifier(address wallet, string calldata identifier) public view returns (euint32) {
        return _getIdentifier(wallet, identifier);
    }

    function _getIdentifier(
        address wallet,
        string calldata identifier
    ) internal view onlyExistingWallet(wallet) onlyAllowed(wallet, identifier) returns (euint32) {
        return identities[wallet].identifiers[identifier];
    }

    function _isIssuer(address wallet, euint8 country) internal view onlyExistingWallet(wallet) returns (ebool) {
        if (ebool.unwrap(identities[wallet].issuer) == 0) return TFHE.asEbool(false);
        euint8 issuerCountry = identities[wallet].country;
        ebool matchingCountry = TFHE.eq(country, issuerCountry);
        ebool issuer = identities[wallet].issuer;
        return TFHE.and(matchingCountry, issuer);
    }

    // ACL
    modifier onlyExistingWallet(address wallet) {
        require(TFHE.isInitialized(identities[wallet].country), "This wallet isn't registered");
        _;
    }

    modifier onlyIssuer(euint8 country) {
        if (owner() != msg.sender) {
            require(TFHE.decrypt(_isIssuer(msg.sender, country)));
        }
        _;
    }

    modifier onlyAllowed(address wallet, string memory identifier) {
        require(
            owner() == msg.sender || permissions[wallet][msg.sender][identifier],
            "User didn't give you permission to access this identifier."
        );
        _;
    }
}
