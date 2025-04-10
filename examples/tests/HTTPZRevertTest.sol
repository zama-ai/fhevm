// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/HTTPZ.sol";
import "../FHEVMConfig.sol";

contract HTTPZRevertTest {
    constructor() {
        HTTPZ.setCoprocessor(FHEVMConfig.defaultConfig());
    }

    function padToBytes64(bytes memory input) public pure {
        HTTPZ.padToBytes64(input);
    }

    function padToBytes128(bytes memory input) public pure {
        HTTPZ.padToBytes128(input);
    }

    function padToBytes256(bytes memory input) public pure {
        HTTPZ.padToBytes256(input);
    }
}
