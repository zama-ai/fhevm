// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

import "../../abstracts/EIP712WithModifier.sol";

import "../../lib/TFHE.sol";

contract IdentityRegistry is EIP712WithModifier, Ownable {
    // A mapping from wallet to registrarId
    mapping(address => uint) public registrars;

    // A mapping from wallet to an identity.
    mapping(address => Identity) internal identities;

    struct Identity {
        uint registrarId;
        mapping(string => euint32) identifiers;
    }

    mapping(address => mapping(address => mapping(string => bool))) permissions; // users => contracts => identifiers[]

    event NewRegistrar(address wallet, uint registrarId);
    event NewDid(address wallet);
    event RemoveDid(address wallet);

    constructor() Ownable() EIP712WithModifier("Authorization token", "1") {
        _transferOwnership(msg.sender);
    }

    function addRegistrar(address wallet, uint registrarId) public onlyOwner {
        require(registrarId > 0, "registrarId needs to be > 0");
        registrars[wallet] = registrarId;
        emit NewRegistrar(wallet, registrarId);
    }

    function removeRegistrar(address wallet) public onlyOwner {
        delete registrars[wallet];
    }

    // Add user
    function addDid(address wallet) public onlyRegistrar {
        require(identities[wallet].registrarId == 0, "This wallet is already registered");
        Identity storage newIdentity = identities[wallet];
        newIdentity.registrarId = registrars[msg.sender];

        emit NewDid(wallet);
    }

    function removeDid(address wallet) public onlyExistingWallet(wallet) onlyRegistrarOf(wallet) {
        require(identities[wallet].registrarId > 0, "This wallet isn't registered");
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
    ) internal onlyExistingWallet(wallet) onlyRegistrarOf(wallet) {
        identities[wallet].identifiers[identifier] = value;
    }

    // User handling permission permission
    function grantAccess(address allowed, string[] calldata identifiers) public {
        for (uint i = 0; i < identifiers.length; i++) {
            permissions[msg.sender][allowed][identifiers[i]] = true;
        }
    }

    function revokeAccess(address allowed, string[] calldata identifiers) public {
        for (uint i = 0; i < identifiers.length; i++) {
            delete permissions[msg.sender][allowed][identifiers[i]];
        }
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

    function getRegistrar(address wallet) public view returns (uint) {
        return identities[wallet].registrarId;
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

    // ACL
    modifier onlyExistingWallet(address wallet) {
        require(identities[wallet].registrarId > 0, "This wallet isn't registered");
        _;
    }

    modifier onlyRegistrar() {
        require(registrars[msg.sender] > 0, "You're not a registrar");
        _;
    }

    modifier onlyRegistrarOf(address wallet) {
        uint registrarId = registrars[msg.sender];
        require(identities[wallet].registrarId == registrarId, "You're not managing this identity");
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
