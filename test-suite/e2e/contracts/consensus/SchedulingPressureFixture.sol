// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

contract SchedulingPressureFixture is E2ECoprocessorConfig {
    euint64[] public outputs;
    euint64 public dependent;

    function heavy(externalEuint64 input, bytes calldata proof) external {
        require(outputs.length == 0, "already seeded");
        euint64 value = FHE.fromExternal(input, proof);
        for (uint8 i = 0; i < 16; ++i) {
            value = FHE.add(value, value);
            outputs.push(value);
            expose(value);
        }
    }

    function child() external {
        require(outputs.length == 16, "missing producer");
        dependent = FHE.add(outputs[15], uint64(1));
        expose(dependent);
    }

    function expose(euint64 value) private {
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }
}
