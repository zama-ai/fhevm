// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "../lib/Ciphertext.sol";
import "../lib/Common.sol";
import "../lib/FHEOps.sol";

// Shows the rand precompile in Solidity.
contract Rand {
    FHEUInt internal value;

    function rand() public {
        value = FHEOps.rand();
    }

    function getValue() public view returns (bytes memory) {
        return Ciphertext.reencrypt(value);
    }
}
