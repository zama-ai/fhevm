// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// Four mixing chains with pinned, repeatedly reused operands and independent work.
contract ManifestHealingStressFixture is E2ECoprocessorConfig {
    euint64[4] public roots;
    euint64[4] public heads;
    euint64[4] public mixed;
    euint64 public independent;

    function seed(uint64 base) external {
        for (uint64 i; i < 4; i++) {
            roots[i] = FHE.asEuint64(base + i);
            heads[i] = roots[i];
            FHE.allowThis(roots[i]);
            FHE.makePubliclyDecryptable(roots[i]);
        }
    }

    function advance(uint64 value) external {
        euint64[4] memory previous = heads;
        for (uint64 i; i < 4; i++) {
            mixed[i] = FHE.add(previous[i], previous[(i + 1) % 4]);
            FHE.allowThis(mixed[i]);
            heads[i] = FHE.add(mixed[i], roots[i]);
            FHE.allowThis(heads[i]);
            FHE.makePubliclyDecryptable(heads[i]);
        }
        independent = FHE.asEuint64(value);
        FHE.allowThis(independent);
        FHE.makePubliclyDecryptable(independent);
    }

    function pinRoots() external {
        for (uint64 i; i < 4; i++) roots[i] = heads[i];
    }
}
