// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "./E2ECoprocessorConfigLocal.sol";

contract SnsBatchPublication is E2ECoprocessorConfig {
    bytes32[] private issued;

    function issue() external {
        for (uint256 i = 0; i < 16; i++) {
            euint8 value = FHE.asEuint8(uint8(issued.length));
            FHE.makePubliclyDecryptable(value);
            issued.push(euint8.unwrap(value));
        }
    }

    function handles() external view returns (bytes32[] memory) {
        return issued;
    }
}
