// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

contract RetainedMaterialFixture is E2ECoprocessorConfig {
    euint64 public retainedInput;
    euint64 public retainedOutput;
    euint64 public freshInput;
    euint64 public fromInput;
    euint64 public fromOutput;
    euint64 public retiredProbe;

    function seed(externalEuint64 input, bytes calldata proof) external {
        require(!FHE.isInitialized(retainedInput), "already seeded");
        retainedInput = FHE.fromExternal(input, proof);
        retainedOutput = FHE.add(retainedInput, uint64(42));
        authorize(retainedInput);
        authorize(retainedOutput);
    }

    function consume(externalEuint64 input, bytes calldata proof) external {
        require(FHE.isInitialized(retainedInput), "not seeded");
        freshInput = FHE.fromExternal(input, proof);
        fromInput = FHE.add(retainedInput, freshInput);
        fromOutput = FHE.add(retainedOutput, freshInput);
        authorize(freshInput);
        authorize(fromInput);
        authorize(fromOutput);
    }

    function probeRetired(uint64 increment) external {
        require(FHE.isInitialized(retainedOutput), "not seeded");
        retiredProbe = FHE.add(retainedOutput, increment);
        authorize(retiredProbe);
    }

    function authorize(euint64 value) private {
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }
}
