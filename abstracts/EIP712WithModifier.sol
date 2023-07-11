// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";

abstract contract EIP712WithModifier is EIP712 {
    constructor(
        string memory name,
        string memory version
    ) EIP712(name, version) {}

    modifier onlySignedPublicKey(bytes32 publicKey, bytes memory signature) {
        bytes32 digest = _hashTypedDataV4(
            keccak256(
                abi.encode(keccak256("Reencrypt(bytes32 publicKey)"), publicKey)
            )
        );
        address signer = ECDSA.recover(digest, signature);
        require(
            signer == msg.sender,
            "EIP712 signer and transaction signer do not match"
        );
        _;
    }
}
