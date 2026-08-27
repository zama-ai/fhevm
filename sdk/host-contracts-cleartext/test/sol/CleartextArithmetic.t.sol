// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FhevmStackHarness} from "./FhevmStackHarness.sol";
import {FHEVMExecutor} from "../../src/contracts/FHEVMExecutor.sol";
import {FheType} from "../../src/contracts/shared/FheType.sol";
import {cleartextArithmeticAdd, fhevmExecutorAdd} from "../../src/addresses/FHEVMHostAddresses.sol";

/**
 * The cleartext op table, checked against native Solidity arithmetic.
 *
 * `CleartextArithmetic` is a hand-written reimplementation of FHE semantics, and it is the shared oracle
 * for every consumer of this package (the Hardhat plugin, the Foundry test library, the templates). That
 * makes a bug here uniquely dangerous: it is CONSISTENT. If `fheShr` or 8-bit wrapping were wrong, every
 * downstream test would agree on the same wrong answer and still pass. Nothing else in the stack has that
 * property, which is why this file exists.
 *
 * Expectations are derived independently — from the type's bit width and plain Solidity operators — rather
 * than by re-deriving the contract's own helper formulas. Where the contract masks (`& mask`), we take a
 * modulus; where it computes `~x + 1`, we compute `2^bw - x`. Agreement then means something.
 *
 * TYPE SETS ARE NOT UNIFORM, and the executor enforces that before the arithmetic ever runs:
 *   - add / sub / mul / div / rem / ge / gt / le / lt / min / max : Uint8..Uint128  (NOT Uint256)
 *   - bitAnd / bitOr / bitXor / not                               : Bool + Uint8..Uint256
 *   - shl / shr / rotl / rotr / neg                               : Uint8..Uint256
 *   - eq / ne                                                     : Bool + Uint8..Uint256
 *   - div / rem                                                   : scalar divisor ONLY
 */
contract CleartextArithmeticTest is FhevmStackHarness {
    bytes1 private constant NON_SCALAR = 0x00;
    bytes1 private constant SCALAR = 0x01;

    /// @dev Types accepted by the arithmetic and ordering ops. Uint256 is rejected by the executor.
    function _arithTypes() private pure returns (FheType[4] memory) {
        return [FheType.Uint8, FheType.Uint32, FheType.Uint64, FheType.Uint128];
    }

    /// @dev Types accepted by the bitwise/shift ops. Includes Uint256, which exercises the
    ///      `bitWidth >= 256` branches in `clamp`, `sub` and `bitNot`.
    function _wideTypes() private pure returns (FheType[5] memory) {
        return [FheType.Uint8, FheType.Uint32, FheType.Uint64, FheType.Uint128, FheType.Uint256];
    }

    /*//////////////////////////////////////////////////////////////
                              THE ORACLE
    //////////////////////////////////////////////////////////////*/

    function _bw(FheType t) private pure returns (uint256) {
        if (t == FheType.Bool) return 1;
        if (t == FheType.Uint8) return 8;
        if (t == FheType.Uint32) return 32;
        if (t == FheType.Uint64) return 64;
        if (t == FheType.Uint128) return 128;
        return 256;
    }

    /// @dev Reduce into the type, by modulus rather than by mask.
    function _wrap(uint256 x, uint256 bw) private pure returns (uint256) {
        if (bw >= 256) return x;
        return x % (uint256(1) << bw);
    }

    function _enc(uint256 v, FheType t) private returns (bytes32) {
        return executor.trivialEncrypt(v, t);
    }

    function _val(bytes32 handle) private view returns (uint256) {
        return executor.plaintexts(handle);
    }

    /*//////////////////////////////////////////////////////////////
                              ARITHMETIC
    //////////////////////////////////////////////////////////////*/

    function testFuzz_fheAdd(uint256 a, uint256 b) public {
        FheType[4] memory ts = _arithTypes();
        for (uint256 i; i < ts.length; ++i) {
            uint256 bw = _bw(ts[i]);
            (uint256 x, uint256 y) = (_wrap(a, bw), _wrap(b, bw));

            uint256 expected;
            unchecked {
                expected = _wrap(x + y, bw);
            }
            assertEq(_val(executor.fheAdd(_enc(x, ts[i]), _enc(y, ts[i]), NON_SCALAR)), expected, "fheAdd");
        }
    }

    function testFuzz_fheSub(uint256 a, uint256 b) public {
        FheType[4] memory ts = _arithTypes();
        for (uint256 i; i < ts.length; ++i) {
            uint256 bw = _bw(ts[i]);
            (uint256 x, uint256 y) = (_wrap(a, bw), _wrap(b, bw));

            // Borrow explicitly rather than relying on two's-complement wraparound.
            uint256 expected = (x + (uint256(1) << bw) - y) % (uint256(1) << bw);
            assertEq(_val(executor.fheSub(_enc(x, ts[i]), _enc(y, ts[i]), NON_SCALAR)), expected, "fheSub");
        }
    }

    function testFuzz_fheMul(uint256 a, uint256 b) public {
        FheType[4] memory ts = _arithTypes();
        for (uint256 i; i < ts.length; ++i) {
            uint256 bw = _bw(ts[i]);
            (uint256 x, uint256 y) = (_wrap(a, bw), _wrap(b, bw));

            // x, y < 2^128 so the product always fits in 256 bits; no truncation before the wrap.
            assertEq(_val(executor.fheMul(_enc(x, ts[i]), _enc(y, ts[i]), NON_SCALAR)), _wrap(x * y, bw), "fheMul");
        }
    }

    /// Division and remainder accept a SCALAR divisor only — the rhs is a raw value, not a handle.
    function testFuzz_fheDivRem(uint256 a, uint256 b) public {
        FheType[4] memory ts = _arithTypes();
        for (uint256 i; i < ts.length; ++i) {
            uint256 bw = _bw(ts[i]);
            uint256 x = _wrap(a, bw);
            uint256 y = _wrap(b, bw);
            if (y == 0) y = 1; // zero divisor is rejected by the executor; tested separately

            assertEq(_val(executor.fheDiv(_enc(x, ts[i]), bytes32(y), SCALAR)), x / y, "fheDiv");
            assertEq(_val(executor.fheRem(_enc(x, ts[i]), bytes32(y), SCALAR)), x % y, "fheRem");
        }
    }

    /*//////////////////////////////////////////////////////////////
                               BITWISE
    //////////////////////////////////////////////////////////////*/

    function testFuzz_fheBitwise(uint256 a, uint256 b) public {
        FheType[5] memory ts = _wideTypes();
        for (uint256 i; i < ts.length; ++i) {
            uint256 bw = _bw(ts[i]);
            (uint256 x, uint256 y) = (_wrap(a, bw), _wrap(b, bw));
            bytes32 hx = _enc(x, ts[i]);
            bytes32 hy = _enc(y, ts[i]);

            assertEq(_val(executor.fheBitAnd(hx, hy, NON_SCALAR)), x & y, "fheBitAnd");
            assertEq(_val(executor.fheBitOr(hx, hy, NON_SCALAR)), x | y, "fheBitOr");
            assertEq(_val(executor.fheBitXor(hx, hy, NON_SCALAR)), x ^ y, "fheBitXor");
        }
    }

    /// Shift amounts are reduced modulo the bit width, so a shift of `bw` or more WRAPS — it does not
    /// zero the value out, which is the intuitive-but-wrong expectation.
    function testFuzz_fheShifts(uint256 a, uint256 shift) public {
        FheType[5] memory ts = _wideTypes();
        for (uint256 i; i < ts.length; ++i) {
            uint256 bw = _bw(ts[i]);
            uint256 x = _wrap(a, bw);
            uint256 s = _wrap(shift, bw);
            uint256 k = s % bw;

            bytes32 hx = _enc(x, ts[i]);
            bytes32 hs = _enc(s, ts[i]);

            assertEq(_val(executor.fheShl(hx, hs, NON_SCALAR)), _wrap(x << k, bw), "fheShl");
            assertEq(_val(executor.fheShr(hx, hs, NON_SCALAR)), x >> k, "fheShr");

            uint256 rotl = k == 0 ? x : _wrap((x << k) | (x >> (bw - k)), bw);
            uint256 rotr = k == 0 ? x : _wrap((x >> k) | (x << (bw - k)), bw);
            assertEq(_val(executor.fheRotl(hx, hs, NON_SCALAR)), rotl, "fheRotl");
            assertEq(_val(executor.fheRotr(hx, hs, NON_SCALAR)), rotr, "fheRotr");
        }
    }

    /*//////////////////////////////////////////////////////////////
                          COMPARISON / SELECT
    //////////////////////////////////////////////////////////////*/

    /// Equality is defined on the wide set; ordering is not.
    function testFuzz_fheEquality(uint256 a, uint256 b) public {
        FheType[5] memory ts = _wideTypes();
        for (uint256 i; i < ts.length; ++i) {
            uint256 bw = _bw(ts[i]);
            (uint256 x, uint256 y) = (_wrap(a, bw), _wrap(b, bw));
            bytes32 hx = _enc(x, ts[i]);
            bytes32 hy = _enc(y, ts[i]);

            assertEq(_val(executor.fheEq(hx, hy, NON_SCALAR)), x == y ? 1 : 0, "fheEq");
            assertEq(_val(executor.fheNe(hx, hy, NON_SCALAR)), x != y ? 1 : 0, "fheNe");
        }
    }

    function testFuzz_fheOrdering(uint256 a, uint256 b) public {
        FheType[4] memory ts = _arithTypes();
        for (uint256 i; i < ts.length; ++i) {
            uint256 bw = _bw(ts[i]);
            (uint256 x, uint256 y) = (_wrap(a, bw), _wrap(b, bw));
            bytes32 hx = _enc(x, ts[i]);
            bytes32 hy = _enc(y, ts[i]);

            assertEq(_val(executor.fheGe(hx, hy, NON_SCALAR)), x >= y ? 1 : 0, "fheGe");
            assertEq(_val(executor.fheGt(hx, hy, NON_SCALAR)), x > y ? 1 : 0, "fheGt");
            assertEq(_val(executor.fheLe(hx, hy, NON_SCALAR)), x <= y ? 1 : 0, "fheLe");
            assertEq(_val(executor.fheLt(hx, hy, NON_SCALAR)), x < y ? 1 : 0, "fheLt");
            assertEq(_val(executor.fheMin(hx, hy, NON_SCALAR)), x < y ? x : y, "fheMin");
            assertEq(_val(executor.fheMax(hx, hy, NON_SCALAR)), x > y ? x : y, "fheMax");
        }
    }

    function testFuzz_fheIfThenElse(bool control, uint64 ifTrue, uint64 ifFalse) public {
        bytes32 r = executor.fheIfThenElse(
            _enc(control ? 1 : 0, FheType.Bool), _enc(ifTrue, FheType.Uint64), _enc(ifFalse, FheType.Uint64)
        );
        assertEq(_val(r), control ? ifTrue : ifFalse, "fheIfThenElse");
    }

    /*//////////////////////////////////////////////////////////////
                                UNARY
    //////////////////////////////////////////////////////////////*/

    function testFuzz_fheNeg(uint256 a) public {
        FheType[5] memory ts = _wideTypes();
        for (uint256 i; i < ts.length; ++i) {
            uint256 bw = _bw(ts[i]);
            uint256 x = _wrap(a, bw);

            // Two's complement stated as arithmetic: -x == 2^bw - x  (mod 2^bw).
            uint256 expected;
            if (bw >= 256) {
                unchecked {
                    expected = 0 - x;
                }
            } else {
                expected = ((uint256(1) << bw) - x) % (uint256(1) << bw);
            }
            assertEq(_val(executor.fheNeg(_enc(x, ts[i]))), expected, "fheNeg");
        }
    }

    function testFuzz_fheNot(uint256 a) public {
        FheType[5] memory ts = _wideTypes();
        for (uint256 i; i < ts.length; ++i) {
            uint256 bw = _bw(ts[i]);
            uint256 x = _wrap(a, bw);

            // Complement as XOR with all-ones, rather than `~x & mask`.
            uint256 allOnes = bw >= 256 ? type(uint256).max : (uint256(1) << bw) - 1;
            assertEq(_val(executor.fheNot(_enc(x, ts[i]))), x ^ allOnes, "fheNot");
        }
    }

    /*//////////////////////////////////////////////////////////////
                                 BOOL
    //////////////////////////////////////////////////////////////*/

    function testFuzz_boolOps(bool a, bool b) public {
        bytes32 hx = _enc(a ? 1 : 0, FheType.Bool);
        bytes32 hy = _enc(b ? 1 : 0, FheType.Bool);

        assertEq(_val(executor.fheBitAnd(hx, hy, NON_SCALAR)), (a && b) ? 1 : 0, "bool and");
        assertEq(_val(executor.fheBitOr(hx, hy, NON_SCALAR)), (a || b) ? 1 : 0, "bool or");
        assertEq(_val(executor.fheBitXor(hx, hy, NON_SCALAR)), (a != b) ? 1 : 0, "bool xor");
        assertEq(_val(executor.fheEq(hx, hy, NON_SCALAR)), (a == b) ? 1 : 0, "bool eq");
        assertEq(_val(executor.fheNot(hx)), a ? 0 : 1, "bool not");
    }

    /// Bool normalizes on the LOW BYTE only, matching `trivial_encrypt_be_bytes`. So 256 is false.
    function test_boolNormalizationUsesLowByteOnly() public {
        assertEq(_val(_enc(1, FheType.Bool)), 1);
        assertEq(_val(_enc(0, FheType.Bool)), 0);
        assertEq(_val(_enc(256, FheType.Bool)), 0, "low byte zero -> false");
        assertEq(_val(_enc(257, FheType.Bool)), 1, "low byte one -> true");
    }

    /*//////////////////////////////////////////////////////////////
                          SCALAR OPERAND PATH
    //////////////////////////////////////////////////////////////*/

    /// With `scalarByte == 0x01` the rhs is a RAW value, not a handle, and is truncated to the type.
    function testFuzz_scalarOperandIsTruncatedToType(uint64 a, uint256 rawScalar) public {
        uint256 bw = _bw(FheType.Uint64);

        uint256 expected;
        unchecked {
            expected = _wrap(uint256(a) + _wrap(rawScalar, bw), bw);
        }
        assertEq(
            _val(executor.fheAdd(_enc(a, FheType.Uint64), bytes32(rawScalar), SCALAR)), expected, "scalar fheAdd"
        );
    }

    /*//////////////////////////////////////////////////////////////
                             EDGE CASES
    //////////////////////////////////////////////////////////////*/

    function test_uint8Wraps() public {
        assertEq(_val(executor.fheAdd(_enc(255, FheType.Uint8), _enc(1, FheType.Uint8), NON_SCALAR)), 0, "255+1");
        assertEq(_val(executor.fheSub(_enc(0, FheType.Uint8), _enc(1, FheType.Uint8), NON_SCALAR)), 255, "0-1");
        assertEq(_val(executor.fheMul(_enc(16, FheType.Uint8), _enc(16, FheType.Uint8), NON_SCALAR)), 0, "16*16");
    }

    function test_negOfZeroIsZero() public {
        assertEq(_val(executor.fheNeg(_enc(0, FheType.Uint8))), 0);
    }

    /// A shift of exactly the width is a no-op (k = bw % bw = 0), not a zeroing.
    function test_shiftByWidthWrapsToNoOp() public {
        assertEq(_val(executor.fheShl(_enc(1, FheType.Uint8), _enc(8, FheType.Uint8), NON_SCALAR)), 1, "shl by 8");
        assertEq(_val(executor.fheShr(_enc(128, FheType.Uint8), _enc(8, FheType.Uint8), NON_SCALAR)), 128, "shr by 8");
    }

    /// The executor rejects a zero divisor up front, before the arithmetic layer is reached.
    function test_divByZeroReverts() public {
        bytes32 x = _enc(1, FheType.Uint64);

        vm.expectRevert(FHEVMExecutor.DivisionByZero.selector);
        executor.fheDiv(x, bytes32(0), SCALAR);
    }

    function test_remByZeroReverts() public {
        bytes32 x = _enc(1, FheType.Uint64);

        vm.expectRevert(FHEVMExecutor.DivisionByZero.selector);
        executor.fheRem(x, bytes32(0), SCALAR);
    }

    /// Uint256 has no arithmetic — the executor rejects it before CleartextArithmetic is consulted.
    function test_uint256RejectsArithmetic() public {
        bytes32 x = _enc(1, FheType.Uint256);
        bytes32 y = _enc(2, FheType.Uint256);

        vm.expectRevert(FHEVMExecutor.UnsupportedType.selector);
        executor.fheAdd(x, y, NON_SCALAR);
    }

    /// Division requires a scalar divisor; an encrypted one is rejected.
    function test_divRejectsEncryptedDivisor() public {
        bytes32 x = _enc(10, FheType.Uint64);
        bytes32 y = _enc(2, FheType.Uint64);

        vm.expectRevert(FHEVMExecutor.IsNotScalar.selector);
        executor.fheDiv(x, y, NON_SCALAR);
    }

    /*//////////////////////////////////////////////////////////////
                            DB WRITE GATING
    //////////////////////////////////////////////////////////////*/

    /// Only CleartextArithmetic may persist plaintexts. If anyone else could write, a contract under test
    /// could forge its own cleartext and every downstream assertion would be meaningless.
    function test_onlyArithmeticCanWriteToTheDb() public {
        assertTrue(db.isWriter(cleartextArithmeticAdd), "arithmetic must be a writer");
        assertFalse(db.isWriter(fhevmExecutorAdd), "executor must NOT be a writer");
        assertFalse(db.isWriter(address(this)), "the test must NOT be a writer");

        vm.expectRevert();
        db.set(bytes32(uint256(1)), 42);
    }
}
