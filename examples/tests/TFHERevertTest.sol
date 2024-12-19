// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../FHEVMConfig.sol";

contract TFHERevertTest {
    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
    }

    function padToBytes64(bytes memory input) public pure {
        TFHE.padToBytes64(input);
    }

    function padToBytes128(bytes memory input) public pure {
        TFHE.padToBytes128(input);
    }

    function padToBytes256(bytes memory input) public pure {
        TFHE.padToBytes256(input);
    }
}
