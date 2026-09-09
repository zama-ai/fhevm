// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {FhevmDeploy} from "../pkg/forge/src/FhevmDeploy.sol";
import {CLEARTEXT_DB_ADDRESS, FHEVM_EXECUTOR_ADDRESS} from "../pkg/forge/src/FhevmDeploy.sol";
import {ICleartextDB} from "../pkg/forge/src/FhevmDeploy.sol";
import {ICleartextFHEVMExecutor} from "../pkg/forge/src/FhevmDeploy.sol";
import {FheType} from "../pkg/src/contracts/shared/FheType.sol";

/**
 * The v13 nary operators, `fheSum` and `fheIsIn`, end to end: executor -> CleartextArithmetic ->
 * CleartextDB.
 *
 * The cases and their expected values mirror the canonical suite, so the cleartext stack computes what
 * the coprocessor is asserted to compute:
 *   contracts: test-suite/e2e/contracts/operations/FHEVMManualTestSuite.sol
 *   expected:  test-suite/e2e/test/fhevmOperations/manual.ts
 * Each test below names the upstream case it corresponds to. Two cases are ours rather than theirs and
 * are marked as such.
 *
 * ## Why Foundry rather than test/ts
 *
 * `_naryOp` requires `ACL.isAllowed(handle, msg.sender)` for every input, and `trivialEncrypt` grants
 * only a TRANSIENT allowance — cleared when the transaction ends. Composing handles therefore has to
 * happen inside one transaction, which is what a Solidity test body is and what a sequence of
 * viem/ethers calls is not.
 *
 * ## Two things about the fixtures
 *
 * `trivialEncrypt` stands in for `FHE.asEuintN`. Its handle derivation contains no counter, so the same
 * plaintext, type and block yield the same handle — which is how the duplicate-element cases below get
 * their duplicates, and why distinct values are used wherever distinct handles are needed.
 *
 * The upstream "uninitialized element" cases do not reach this layer as `bytes32(0)`: `FHE.sum` and
 * `FHE.isIn` replace an uninitialized handle with `asEuintN(0)` before calling the executor
 * (library-solidity/lib/FHE.sol). So the equivalent fixture here is an explicit encryption of 0, and
 * "uninitialized is 0" is a library guarantee rather than something this contract implements.
 */
contract CleartextNaryOpsTest is Test, FhevmDeploy {
    ICleartextFHEVMExecutor internal executor;
    ICleartextDB internal db;

    function setUp() public {
        deployFhevm();
        executor = ICleartextFHEVMExecutor(FHEVM_EXECUTOR_ADDRESS);
        db = ICleartextDB(CLEARTEXT_DB_ADDRESS);
    }

    function _encrypt(uint256[] memory plaintexts, FheType fheType) internal returns (bytes32[] memory handles) {
        handles = new bytes32[](plaintexts.length);
        for (uint256 i = 0; i < plaintexts.length; i++) {
            handles[i] = executor.trivialEncrypt(plaintexts[i], fheType);
        }
    }

    function _values2(uint256 a, uint256 b) internal pure returns (uint256[] memory out) {
        out = new uint256[](2);
        out[0] = a;
        out[1] = b;
    }

    function _values3(uint256 a, uint256 b, uint256 c) internal pure returns (uint256[] memory out) {
        out = new uint256[](3);
        out[0] = a;
        out[1] = b;
        out[2] = c;
    }

    // -----------------------------------------------------------------------
    // fheSum
    // -----------------------------------------------------------------------

    /// Upstream: test_sum_euint8 (three elements).
    function test_sum_euint8() public {
        bytes32[] memory values = _encrypt(_values3(10, 20, 12), FheType.Uint8);
        assertEq(db.get(executor.fheSum(values, FheType.Uint8)), 42);
    }

    /// Upstream: test_sum_euint16.
    function test_sum_euint16() public {
        bytes32[] memory values = _encrypt(_values2(30000, 2000), FheType.Uint16);
        assertEq(db.get(executor.fheSum(values, FheType.Uint16)), 32000);
    }

    /// Upstream: test_sum_euint32.
    function test_sum_euint32() public {
        bytes32[] memory values = _encrypt(_values2(4000000000, 200000000), FheType.Uint32);
        assertEq(db.get(executor.fheSum(values, FheType.Uint32)), 4200000000);
    }

    /// Upstream: test_sum_euint64.
    function test_sum_euint64() public {
        bytes32[] memory values = _encrypt(_values2(18000000000000000000, 400000000000000000), FheType.Uint64);
        assertEq(db.get(executor.fheSum(values, FheType.Uint64)), 18400000000000000000);
    }

    /// Upstream: test_sum_euint128.
    function test_sum_euint128() public {
        bytes32[] memory values = _encrypt(_values2(2 ** 120, 2 ** 119), FheType.Uint128);
        assertEq(db.get(executor.fheSum(values, FheType.Uint128)), 2 ** 120 + 2 ** 119);
    }

    /// Upstream: test_sum_euint8_duplicate — the same handle twice sums to value * 2.
    function test_sum_euint8_duplicate() public {
        bytes32 handle = executor.trivialEncrypt(21, FheType.Uint8);
        bytes32[] memory values = new bytes32[](2);
        values[0] = handle;
        values[1] = handle;
        assertEq(db.get(executor.fheSum(values, FheType.Uint8)), 42, "a duplicate element counts twice");
    }

    /// Upstream: test_sum_euint8_uninitialized — 5 + (uninitialized -> 0) == 5.
    function test_sum_euint8_uninitializedIsZero() public {
        bytes32[] memory values = _encrypt(_values2(5, 0), FheType.Uint8);
        assertEq(db.get(executor.fheSum(values, FheType.Uint8)), 5);
    }

    /// Upstream: test_sum_euint8_empty.
    function test_sum_euint8_empty() public {
        bytes32[] memory values = new bytes32[](0);
        assertEq(db.get(executor.fheSum(values, FheType.Uint8)), 0);
    }

    /// Upstream: test_sum_euint8_single.
    function test_sum_euint8_single() public {
        bytes32[] memory values = new bytes32[](1);
        values[0] = executor.trivialEncrypt(42, FheType.Uint8);
        assertEq(db.get(executor.fheSum(values, FheType.Uint8)), 42);
    }

    /// Upstream: test_sum_euint8_max_array — 100 elements, the narrow collection cap, each 1.
    function test_sum_euint8_maxArray() public {
        bytes32 one = executor.trivialEncrypt(1, FheType.Uint8);
        bytes32[] memory values = new bytes32[](100);
        for (uint256 i = 0; i < 100; i++) {
            values[i] = one;
        }
        assertEq(db.get(executor.fheSum(values, FheType.Uint8)), 100);
    }

    /**
     * OURS, not upstream: the fold wraps at the result type's bit-width. 200 + 100 + 0 = 300, and
     * 300 & 0xff = 44.
     *
     * This pins the clamping, not the folding — accumulate-then-clamp gives the same answer for every
     * width below 256, which was checked by making that substitution and watching this still pass.
     */
    function test_sum_euint8_wrapsAtTheResultTypeBitWidth() public {
        bytes32[] memory values = _encrypt(_values3(200, 100, 0), FheType.Uint8);
        assertEq(db.get(executor.fheSum(values, FheType.Uint8)), 44, "fheSum should wrap at 8 bits");
    }

    // -----------------------------------------------------------------------
    // fheIsIn
    // -----------------------------------------------------------------------

    /// Upstream: test_isIn_euint8_found.
    function test_isIn_euint8_found() public {
        bytes32[] memory set = _encrypt(_values3(7, 8, 9), FheType.Uint8);
        bytes32 needle = executor.trivialEncrypt(8, FheType.Uint8);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint8)), 1);
    }

    /// Upstream: test_isIn_euint8_not_found.
    function test_isIn_euint8_notFound() public {
        bytes32[] memory set = _encrypt(_values3(7, 8, 9), FheType.Uint8);
        bytes32 needle = executor.trivialEncrypt(11, FheType.Uint8);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint8)), 0);
    }

    /// Upstream: test_isIn_euint16.
    function test_isIn_euint16() public {
        bytes32[] memory set = _encrypt(_values2(1000, 60000), FheType.Uint16);
        bytes32 needle = executor.trivialEncrypt(60000, FheType.Uint16);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint16)), 1);
    }

    /// Upstream: test_isIn_euint32.
    function test_isIn_euint32() public {
        bytes32[] memory set = _encrypt(_values2(1, 4000000000), FheType.Uint32);
        bytes32 needle = executor.trivialEncrypt(4000000000, FheType.Uint32);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint32)), 1);
    }

    /// Upstream: test_isIn_euint64.
    function test_isIn_euint64() public {
        bytes32[] memory set = _encrypt(_values2(1, 18000000000000000000), FheType.Uint64);
        bytes32 needle = executor.trivialEncrypt(18000000000000000000, FheType.Uint64);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint64)), 1);
    }

    /// Upstream: test_isIn_euint128.
    function test_isIn_euint128() public {
        bytes32[] memory set = _encrypt(_values2(1, 2 ** 120), FheType.Uint128);
        bytes32 needle = executor.trivialEncrypt(2 ** 120, FheType.Uint128);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint128)), 1);
    }

    /// Upstream: test_isIn_euint8_uninitialized — an uninitialized needle is 0, and 0 is in {0, 1}.
    function test_isIn_euint8_uninitializedNeedleIsZero() public {
        bytes32[] memory set = _encrypt(_values2(0, 1), FheType.Uint8);
        bytes32 needle = executor.trivialEncrypt(0, FheType.Uint8);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint8)), 1);
    }

    /// Upstream: test_isIn_euint8_single_element.
    function test_isIn_euint8_singleElement() public {
        bytes32[] memory set = new bytes32[](1);
        set[0] = executor.trivialEncrypt(42, FheType.Uint8);
        bytes32 needle = executor.trivialEncrypt(42, FheType.Uint8);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint8)), 1);
    }

    /// Upstream: test_isIn_euint8_max_array — needle 50 within the set {0..99}.
    function test_isIn_euint8_maxArray() public {
        bytes32[] memory set = new bytes32[](100);
        for (uint256 i = 0; i < 100; i++) {
            set[i] = executor.trivialEncrypt(i, FheType.Uint8);
        }
        bytes32 needle = executor.trivialEncrypt(50, FheType.Uint8);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint8)), 1);
    }

    /// Upstream: test_isIn_euint8_empty_set.
    function test_isIn_euint8_emptySet() public {
        bytes32[] memory set = new bytes32[](0);
        bytes32 needle = executor.trivialEncrypt(42, FheType.Uint8);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint8)), 0);
    }

    /// Upstream: test_isIn_euint8_zero_initialized_set — enc(0) is found in {0, 0, 0}.
    function test_isIn_euint8_zeroInitializedSet() public {
        bytes32[] memory set = _encrypt(_values3(0, 0, 0), FheType.Uint8);
        bytes32 needle = executor.trivialEncrypt(0, FheType.Uint8);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint8)), 1);
    }

    /**
     * OURS, not upstream: a needle equal to the set's *sum* rather than to any element must not match.
     * Guards against an `isIn` that folds instead of comparing.
     */
    function test_isIn_euint8_doesNotMatchTheSum() public {
        bytes32[] memory set = _encrypt(_values3(1, 2, 3), FheType.Uint8);
        bytes32 needle = executor.trivialEncrypt(6, FheType.Uint8);
        assertEq(db.get(executor.fheIsIn(needle, set, FheType.Uint8)), 0);
    }
}
