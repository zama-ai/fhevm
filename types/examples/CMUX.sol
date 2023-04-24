// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "../lib/Ciphertext.sol";
import "../lib/Common.sol";
import "../lib/FHEOps.sol";

// Shows the CMUX operation in Solidity.
contract CMUX {
    euint8 internal result;

    // Set result = (ifTrue - ifFalse) * control + ifFalse
    function cmux(
        bytes calldata controlBytes,
        bytes calldata ifTrueBytes,
        bytes calldata ifFalseBytes
    ) public {
        euint8 control = Ciphertext.asEuint8(controlBytes);
        euint8 ifTrue = Ciphertext.asEuint8(ifTrueBytes);
        euint8 ifFalse = Ciphertext.asEuint8(ifFalseBytes);
        result = FHEOps.cmux(control, ifTrue, ifFalse);
    }

    function getResult() public view returns (bytes memory) {
        return Ciphertext.reencrypt(result);
    }
}
