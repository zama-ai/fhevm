// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

contract SlowLaneContention is E2ECoprocessorConfig {
    euint64 public lastResult;

    function runAddChain(externalEuint64 seed, bytes calldata inputProof, uint8 steps) external {
        euint64 value = FHE.fromExternal(seed, inputProof);

        for (uint8 i = 0; i < steps; i++) {
            value = FHE.add(value, value);
            FHE.allowThis(value);
        }

        lastResult = value;
        FHE.allowThis(lastResult);
        FHE.allow(lastResult, msg.sender);
        FHE.makePubliclyDecryptable(lastResult);
    }
}
