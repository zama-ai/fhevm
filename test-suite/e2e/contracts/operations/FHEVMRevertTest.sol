// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import { E2EFHEVMConfig } from "../E2EFHEVMConfigLocal.sol";

contract FHEVMRevertTest is E2EFHEVMConfig {

    function padToBytes64(bytes memory input) public pure {
        FHE.padToBytes64(input);
    }

    function padToBytes128(bytes memory input) public pure {
        FHE.padToBytes128(input);
    }

    function padToBytes256(bytes memory input) public pure {
        FHE.padToBytes256(input);
    }
}