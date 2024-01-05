// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.19;

import "../abstracts/Reencrypt.sol";
import "../lib/TFHE.sol";

// Shows the CMUX operation in Solidity.
contract CMUX is Reencrypt {
    euint8 internal result;

    constructor() {}

    // Set result = if control { ifTrue } else { ifFalse }
    function cmux(bytes calldata controlBytes, bytes calldata ifTrueBytes, bytes calldata ifFalseBytes) public {
        ebool control = TFHE.asEbool(controlBytes);
        euint8 ifTrue = TFHE.asEuint8(ifTrueBytes);
        euint8 ifFalse = TFHE.asEuint8(ifFalseBytes);
        result = TFHE.cmux(control, ifTrue, ifFalse);
    }

    function getResult(
        bytes32 publicKey,
        bytes calldata signature
    ) public view onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
        return TFHE.reencrypt(result, publicKey);
    }
}
