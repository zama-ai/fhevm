// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// Small cross-transaction graph: root -> child -> blocked, plus independent work.
contract ManifestLifecycleFixture is E2ECoprocessorConfig {
    euint64 public root;
    euint64 public child;
    euint64 public blocked;
    euint64 public independent;

    function seed(uint64 value) external {
        root = FHE.asEuint64(value);
        FHE.allowThis(root);
    }

    function derive() external {
        child = FHE.add(root, uint64(1));
        FHE.allowThis(child);
    }

    function consume(uint64 value) external {
        blocked = FHE.add(child, uint64(2));
        independent = FHE.asEuint64(value);
        FHE.allowThis(blocked);
        FHE.allowThis(independent);
    }
}
