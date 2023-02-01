// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "../lib/Ciphertext.sol";
import "../lib/Common.sol";
import "../lib/FHEOps.sol";

// Shows the CMUX operation in Solidity.
contract CMUX {
    FHEUInt internal result;

    // Set result = (ifTrue - ifFalse) * control + ifFalse
    function cmux(bytes calldata controlBytes, bytes calldata ifTrueBytes, bytes calldata ifFalseBytes) public {
        FHEUInt control = Ciphertext.verify(controlBytes);
        FHEUInt ifTrue = Ciphertext.verify(ifTrueBytes);
        FHEUInt ifFalse = Ciphertext.verify(ifFalseBytes);
        result = FHEOps.cmux(control, ifTrue, ifFalse);
    }

    function getResult() public view returns (bytes memory) {
        return Ciphertext.reencrypt(result);
    }
}
