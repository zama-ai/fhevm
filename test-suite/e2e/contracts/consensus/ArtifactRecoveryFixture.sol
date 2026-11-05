// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @notice Cross-transaction graph for full-environment ingestion recovery.
/// Uses the deployed executor and ACL; the runner faults RPC delivery only.
contract ArtifactRecoveryFixture is E2ECoprocessorConfig {
    euint64 public root;
    euint64 public left;
    euint64 public right;
    euint64 public terminal;
    euint64 public canary;

    function seed(uint64 value) external {
        root = FHE.asEuint64(value);
        _expose(root);
    }

    function children() external {
        left = FHE.add(root, uint64(5));
        right = FHE.add(root, uint64(9));
        _expose(left);
        _expose(right);
    }

    function grandchild() external {
        terminal = FHE.add(left, right);
        _expose(terminal);
    }

    function independent(uint64 value) external {
        canary = FHE.asEuint64(value);
        _expose(canary);
    }

    function _expose(euint64 value) private {
        FHE.allowThis(value);
        FHE.makePubliclyDecryptable(value);
    }
}
