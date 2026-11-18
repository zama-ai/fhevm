// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// One-shot QA workload: every output depends on a fresh encrypted input.
contract SnsLoad is E2ECoprocessorConfig {
    event Output(bytes32 handle, uint64 offset);

    function produce(externalEuint64 handle, bytes calldata proof, uint64 count) external {
        require(count > 0 && count <= 20);
        euint64 input = FHE.fromExternal(handle, proof);
        for (uint64 i = 0; i < count; i++) {
            euint64 output = FHE.add(input, uint64(42 + i));
            FHE.allowThis(output);
            FHE.allow(output, msg.sender);
            FHE.makePubliclyDecryptable(output);
            emit Output(euint64.unwrap(output), 42 + i);
        }
    }
}
