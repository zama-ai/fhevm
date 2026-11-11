// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {EngineCoprocessorConfig} from "./EngineCoprocessorConfig.sol";

/// @notice Two encrypted types in one input: `and_euint64_euint32` copied from
/// test-suite/e2e/contracts/operations/FHEVMTestSuite4.sol, with the coprocessor config as a
/// constructor argument.
contract TestMultiInput is EngineCoprocessorConfig {
    euint64 public resEuint64;

    constructor(CoprocessorConfig memory config) EngineCoprocessorConfig(config) {}

    function and_euint64_euint32(externalEuint64 a, externalEuint32 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
}
