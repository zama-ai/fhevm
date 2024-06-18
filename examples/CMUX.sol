// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.25;

import "../abstracts/Reencrypt.sol";
import "../lib/TFHE.sol";

// Shows the SELECT operation in Solidity.
contract SELECT is Reencrypt {
    euint8 internal result;

    constructor() {}

    // Set result = if control { ifTrue } else { ifFalse }
    function select(einput controlBytes, einput ifTrueBytes, einput ifFalseBytes, bytes calldata inputProof) public {
        ebool control = TFHE.asEbool(controlBytes, inputProof);
        euint8 ifTrue = TFHE.asEuint8(ifTrueBytes, inputProof);
        euint8 ifFalse = TFHE.asEuint8(ifFalseBytes, inputProof);
        result = TFHE.select(control, ifTrue, ifFalse);
    }

    // function getResult(
    //     bytes32 publicKey,
    //     bytes calldata signature
    // ) public view onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
    //     return TFHE.reencrypt(result, publicKey);
    // }
}
