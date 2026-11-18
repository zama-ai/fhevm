// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// One-shot QA workload: fresh independent outputs from a real encrypted input.
contract SnsLoad is E2ECoprocessorConfig {
    euint64 private input;

    event Output(bytes32 handle, uint64 offset);

    function setInput(externalEuint64 handle, bytes calldata proof) external {
        input = FHE.fromExternal(handle, proof);
        FHE.allowThis(input);
    }

    function produce(uint64 firstOffset, uint64 count) external {
        require(count > 0 && count <= 20);
        for (uint64 i = 0; i < count; i++) {
            uint64 offset = firstOffset + i;
            euint64 output = FHE.add(input, offset);
            FHE.allowThis(output);
            FHE.allow(output, msg.sender);
            FHE.makePubliclyDecryptable(output);
            emit Output(euint64.unwrap(output), offset);
        }
    }
}
