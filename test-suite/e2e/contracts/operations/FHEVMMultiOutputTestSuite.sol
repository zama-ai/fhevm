// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// PROBE ONLY. Drives the coprocessor's multi-output path end to end using the
/// synthetic `FHE.reverse` operator. Not for production.
contract FHEVMMultiOutputTestSuite is E2ECoprocessorConfig {
    euint64[] internal results;

    function _encrypt(uint64[] calldata values) internal returns (euint64[] memory inputs) {
        inputs = new euint64[](values.length);
        for (uint256 i = 0; i < values.length; i++) {
            inputs[i] = FHE.asEuint64(values[i]);
        }
    }

    function _publish(euint64[] memory out) internal {
        for (uint256 i = 0; i < out.length; i++) {
            FHE.makePubliclyDecryptable(out[i]);
            results.push(out[i]);
        }
    }

    /// Reverses `values` and reveals every output.
    function reverseRevealAll(uint64[] calldata values) external {
        euint64[] memory out = FHE.reverse(_encrypt(values));
        delete results;
        _publish(out);
    }

    /// Reverses `values` but reveals only the outputs listed in `reveal`. The
    /// unrevealed siblings must stay inaccessible while the operation as a whole
    /// still succeeds — the "shuffle 52, reveal 5" shape.
    function reverseRevealSome(uint64[] calldata values, uint256[] calldata reveal) external {
        euint64[] memory out = FHE.reverse(_encrypt(values));
        delete results;
        for (uint256 i = 0; i < out.length; i++) {
            results.push(out[i]);
        }
        for (uint256 i = 0; i < reveal.length; i++) {
            FHE.makePubliclyDecryptable(out[reveal[i]]);
        }
    }

    /// Reverses `values`, then adds `addend` to one chosen output. Exercises a
    /// consumer depending on a specific sibling, and the HCU depth stamped on
    /// every output rather than only the first.
    function reverseThenAdd(uint64[] calldata values, uint256 index, uint64 addend) external {
        euint64[] memory out = FHE.reverse(_encrypt(values));
        euint64 sum = FHE.add(out[index], FHE.asEuint64(addend));
        FHE.makePubliclyDecryptable(sum);
        delete results;
        results.push(sum);
    }

    /// Two reverses in one transaction. Distinct inputs must mint distinct
    /// handles; identical inputs deliberately mint identical ones, since handles
    /// are content-addressed and no per-call nonce enters the preimage.
    function reverseTwoGroups(uint64[] calldata a, uint64[] calldata b) external {
        euint64[] memory first = FHE.reverse(_encrypt(a));
        euint64[] memory second = FHE.reverse(_encrypt(b));
        delete results;
        _publish(first);
        _publish(second);
    }

    function resultsLength() external view returns (uint256) {
        return results.length;
    }

    function resultAt(uint256 i) external view returns (euint64) {
        return results[i];
    }
}
