// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @notice The operations an `FHEVMExecutor` can perform, declared apart from the contract that owns
 *         them.
 *
 * @dev WHY A SECOND DECLARATION. `Operators` belongs to `contracts/FHEVMExecutor.sol`, which is
 *      VENDORED — synced from upstream, never edited here (see `check:vendored-origin`). Importing it
 *      for the enum alone drags the whole executor, and every one of its imports, into anything that
 *      only wants to NAME an operation. `CleartextArithmeticBase` wants exactly that and nothing else,
 *      and being free of `contracts/` is what makes it portable.
 *
 * @dev IT IS DELIBERATELY NOT CALLED `FHEVMExecutor.Operators`. Re-declaring under the original
 *      spelling would produce a second type indistinguishable from the first at a glance — and they
 *      are NOT interchangeable: two enum declarations are two types, so a value crosses between them
 *      only through an explicit `uint8`. `CleartextFHEVMExecutor._op` is that crossing, and the
 *      different name is what makes it visible rather than a puzzle.
 *
 * @dev THE ORDER IS THE ABI. These are positions, not labels: `recordBinaryOp` carries a `uint8` on the
 *      wire and `_computeBinaryOp` dispatches on equality. An upstream release inserting or reordering
 *      a member would not fail to compile — it would map `fheAdd` onto `fheSub` and quietly compute
 *      wrong answers. `FhevmOperators.t.sol` is what stops that; do not touch this list without it.
 */
enum Operators {
    fheAdd,
    fheSub,
    fheMul,
    fheDiv,
    fheRem,
    fheBitAnd,
    fheBitOr,
    fheBitXor,
    fheShl,
    fheShr,
    fheRotl,
    fheRotr,
    fheEq,
    fheNe,
    fheGe,
    fheGt,
    fheLe,
    fheLt,
    fheMin,
    fheMax,
    fheNeg,
    fheNot,
    verifyInput,
    cast,
    trivialEncrypt,
    fheIfThenElse,
    fheRand,
    fheRandBounded,
    fheSum,
    fheIsIn,
    fheMulDiv
}
