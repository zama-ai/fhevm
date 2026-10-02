// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// Seven drift roots, two overlapping ct64 branches, and independent work.
contract ManifestHealingFixture is E2ECoprocessorConfig {
    euint64[7] public roots;
    euint64[7] public children;
    euint64[7] public consumers;
    euint64 public joined;
    euint64 public tail;
    euint64 public queuedJoin;
    euint64 public recovered;
    euint64 public independent;
    euint64 public reused;

    function seed(uint64 base) external {
        for (uint64 i; i < 7; i++) {
            roots[i] = FHE.asEuint64(base + i);
            FHE.allowThis(roots[i]);
        }
    }

    function derive() external {
        for (uint64 i; i < 7; i++) {
            children[i] = FHE.add(roots[i], uint64(10));
            FHE.allowThis(children[i]);
        }
    }

    function joinBranches() external {
        joined = FHE.add(children[0], children[1]);
        FHE.allowThis(joined);
    }

    function extendBranch() external {
        tail = FHE.add(joined, uint64(1));
        FHE.allowThis(tail);
    }

    function consume(uint64 value) external {
        for (uint64 i; i < 7; i++) {
            // Exercise inferred containment on the first two branches and direct
            // root usability for every other reason.
            consumers[i] = FHE.add(i < 2 ? children[i] : roots[i], uint64(100));
            FHE.allowThis(consumers[i]);
            FHE.makePubliclyDecryptable(consumers[i]);
        }
        queuedJoin = FHE.add(consumers[0], consumers[1]);
        FHE.allowThis(queuedJoin);
        recovered = FHE.add(queuedJoin, tail);
        FHE.allowThis(recovered);
        FHE.makePubliclyDecryptable(recovered);
        independent = FHE.asEuint64(value);
        FHE.allowThis(independent);
    }

    function reuse() external {
        reused = FHE.add(recovered, roots[1]);
        FHE.allowThis(reused);
        FHE.makePubliclyDecryptable(reused);
    }
}
