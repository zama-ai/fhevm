// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/FHE.sol";
import {CoprocessorSetup} from "../CoprocessorSetup.sol";

/**
 * @title  FHEVMCollectionSuite
 * @notice Correctness test suite for FHE.isIn and FHE.sum.
 */
contract FHEVMCollectionSuite {
    ebool public lastBool;
    euint8 public lastUint8;
    euint32 public lastUint32;
    euint64 public lastUint64;

    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    }

    // -------------------------------------------------------------------------
    // isIn — euint8
    // -------------------------------------------------------------------------

    function isIn_euint8_found(externalEuint8 encNeedle, bytes calldata inputProof, uint8[] calldata set) public {
        euint8 needle = FHE.fromExternal(encNeedle, inputProof);
        lastBool = FHE.isIn(needle, set);
        FHE.allowThis(lastBool);
    }

    // -------------------------------------------------------------------------
    // isIn — euint32
    // -------------------------------------------------------------------------

    function isIn_euint32_found(externalEuint32 encNeedle, bytes calldata inputProof, uint32[] calldata set) public {
        euint32 needle = FHE.fromExternal(encNeedle, inputProof);
        lastBool = FHE.isIn(needle, set);
        FHE.allowThis(lastBool);
    }

    // -------------------------------------------------------------------------
    // isIn — euint64
    // -------------------------------------------------------------------------

    function isIn_euint64_found(externalEuint64 encNeedle, bytes calldata inputProof, uint64[] calldata set) public {
        euint64 needle = FHE.fromExternal(encNeedle, inputProof);
        lastBool = FHE.isIn(needle, set);
        FHE.allowThis(lastBool);
    }

    // -------------------------------------------------------------------------
    // sum — euint8
    // -------------------------------------------------------------------------

    function sum_euint8(externalEuint8[] calldata encVals, bytes calldata inputProof) public {
        euint8[] memory values = new euint8[](encVals.length);
        for (uint256 i = 0; i < encVals.length; i++) {
            values[i] = FHE.fromExternal(encVals[i], inputProof);
        }
        lastUint8 = FHE.sum(values);
        FHE.allowThis(lastUint8);
    }

    // -------------------------------------------------------------------------
    // sum — euint32
    // -------------------------------------------------------------------------

    function sum_euint32(externalEuint32[] calldata encVals, bytes calldata inputProof) public {
        euint32[] memory values = new euint32[](encVals.length);
        for (uint256 i = 0; i < encVals.length; i++) {
            values[i] = FHE.fromExternal(encVals[i], inputProof);
        }
        lastUint32 = FHE.sum(values);
        FHE.allowThis(lastUint32);
    }

    // -------------------------------------------------------------------------
    // sum — euint64
    // -------------------------------------------------------------------------

    function sum_euint64(externalEuint64[] calldata encVals, bytes calldata inputProof) public {
        euint64[] memory values = new euint64[](encVals.length);
        for (uint256 i = 0; i < encVals.length; i++) {
            values[i] = FHE.fromExternal(encVals[i], inputProof);
        }
        lastUint64 = FHE.sum(values);
        FHE.allowThis(lastUint64);
    }

    // -------------------------------------------------------------------------
    // Verify input array is not modified by sum (uses trivial encryption to test
    // internal Solidity memory semantics — the encryption itself is not under test)
    // -------------------------------------------------------------------------

    function sum_euint32_checkNoMutation(uint32[] calldata vals) public returns (bool notMutated) {
        require(vals.length >= 2, "need at least 2 elements");
        euint32[] memory enc = new euint32[](vals.length);
        for (uint256 i = 0; i < vals.length; i++) {
            enc[i] = FHE.asEuint32(vals[i]);
        }
        bytes32 handle0Before = euint32.unwrap(enc[0]);
        bytes32 handle1Before = euint32.unwrap(enc[1]);
        FHE.sum(enc);
        bytes32 handle0After = euint32.unwrap(enc[0]);
        bytes32 handle1After = euint32.unwrap(enc[1]);
        notMutated = (handle0Before == handle0After) && (handle1Before == handle1After);
    }
}
