// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "../abstracts/EIP712WithModifier.sol";
import "../lib/TFHE.sol";

// Shows the CMUX operation in Solidity.
contract CMUX is EIP712WithModifier {
    euint8 internal result;

    constructor() EIP712WithModifier("Authorization token", "1") {}

    // Set result = if control { ifTrue } else { ifFalse }
    function cmux(
        bytes calldata controlBytes,
        bytes calldata ifTrueBytes,
        bytes calldata ifFalseBytes
    ) public {
        euint8 control = TFHE.asEuint8(controlBytes);
        euint8 ifTrue = TFHE.asEuint8(ifTrueBytes);
        euint8 ifFalse = TFHE.asEuint8(ifFalseBytes);
        result = TFHE.cmux(control, ifTrue, ifFalse);
    }

    function getResult(
        bytes32 publicKey,
        bytes calldata signature
    )
        public
        view
        onlySignedPublicKey(publicKey, signature)
        returns (bytes memory)
    {
        return TFHE.reencrypt(result, publicKey);
    }
}
