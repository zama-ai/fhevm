// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType, LibFheType} from "./LibFheType.sol";
import {Operators} from "./FhevmOperatorsEnum.sol";
import {LibFhevmHandle} from "./LibFhevmHandle.sol";
import {ICleartextArithmetic} from "./interfaces/ICleartextArithmetic.sol";
import {IPlaintexts} from "./interfaces/IPlaintexts.sol";
import {ICleartextDB} from "./interfaces/ICleartextDB.sol";
import {LibCleartextPad} from "./LibCleartextPad.sol";
import {LibCleartextZkPok} from "./LibCleartextZkPok.sol";

/**
 * @title CleartextArithmeticBase
 * @notice The whole cleartext computation + persistence layer, WITHOUT the ACL and the proxy: a plain,
 *         immutable contract that reads operand cleartexts from `CleartextDB`, computes results
 *         mirroring FHE bit-width semantics, and writes them back.
 * @dev This is `CleartextArithmetic` minus two things, and only those two: `ACLOwnable` (who may
 *      upgrade) and `UUPSUpgradeableEmptyProxy` (that it can be upgraded at all). Everything a caller
 *      of `ICleartextArithmetic` actually uses lives here, so this contract is complete on its own —
 *      `new CleartextArithmeticBase()` and register it with `CleartextDB`, no proxy, no owner, no
 *      initializer.
 *
 *      Use it wherever the upgrade path is not wanted: a forge test, a local rig, a one-shot
 *      deployment. All an heir must supply is `getCleartextDBAddress()`, naming the store to compute
 *      into. `CleartextArithmetic` supplies the host's and adds the proxy machinery back; it overrides
 *      no behaviour beyond that.
 *
 *      Declaring no storage of its own, it cannot disturb the derived contract's storage layout.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
abstract contract CleartextArithmeticBase is ICleartextArithmetic {
    /// @notice Marks a cleartext (mock) implementation. Real host contracts have no such selector, so a
    ///         consumer can probe it to tell a cleartext stack from a production deployment.
    bool public constant IS_CLEARTEXT = true;

    error CleartextErrorUnsupportedBinaryOp(Operators op);
    error CleartextErrorUnsupportedUnaryOp(Operators op);
    error CleartextErrorUnsupportedTernaryOp(Operators op);
    error CleartextErrorUnsupportedNaryOp(Operators op);
    error CleartextErrorNotABoolean(uint256 value);
    error CleartextErrorPlaintextTooWide(uint256 value, FheType fheType);
    error CleartextErrorPadPackageWrongChain(uint64 chainId);
    error CleartextErrorPadPackageLabelMismatch(bytes32 label, bytes32 expectedLabel);

    /// @inheritdoc IPlaintexts
    /// @dev Guarded like every `record*` above: a handle minted on another chain names a slot in THIS
    ///      chain's DB, which is either empty or — worse — some unrelated handle's value. Reading it
    ///      would answer a question nobody asked, so it reverts instead.
    function plaintexts(bytes32 handle) external view override returns (uint256) {
        _checkHandleChainId(handle);
        return _operand(_db(), handle);
    }

    /**
     * @dev EVERY READ OF AN OPERAND GOES THROUGH HERE, and that is the only reason it exists. The store
     *      answers zero for a handle it never saw, which is indistinguishable from a handle worth zero --
     *      fine on a stack that minted everything it is asked about, and wrong on a FORK, where handles
     *      predate the cleartext layer entirely.
     *
     *      This contract keeps the plain behaviour. `CleartextForgeArithmetic` overrides it to answer for
     *      handles the store does not know, which is a test concern and stays out of the deployable
     *      contract: no chain ever runs the override, and the seam costs a virtual dispatch.
     */
    function _operand(ICleartextDB db, bytes32 handle) internal view virtual returns (uint256) {
        return db.get(handle);
    }

    /// @notice Whether the store holds a cleartext for `handle`.
    /// @dev    Unlike `plaintexts` this does NOT check the chain field: a handle from elsewhere is
    ///         simply absent, which is the answer a caller asking "do you know this one" wants.
    function hasPlaintext(bytes32 handle) external view virtual returns (bool) {
        return _db().has(handle);
    }

    /**
     * @notice Getter function for the CleartextDB contract address.
     * @dev UNIMPLEMENTED, and the ONLY place the DB address is decided: every `record*` below reaches
     *      the store through `_db()`, so this one function is the whole of "where does the state live".
     *      Leaving it abstract is what keeps this file free of `addresses/` — the semantics do not
     *      depend on any particular deployment, and a contract that does not know its own store has no
     *      business being deployable. `CleartextArithmetic` answers `cleartextDbAdd`.
     */
    function getCleartextDBAddress() public view virtual returns (address);

    /// @dev The store, resolved through the overridable getter above.
    function _db() internal view returns (ICleartextDB) {
        return ICleartextDB(getCleartextDBAddress());
    }

    /**
     * @dev Every handle a `record*` touches passes through here. A handle minted on another chain
     *      names a slot in THIS chain's store, which is either empty or — worse — some unrelated
     *      handle's value, so by default it is refused rather than read.
     *
     *      `virtual` because that reasoning is a property of the STORE, not of the arithmetic: a store
     *      keyed by chain has a distinct slot per chain and can hold foreign handles safely. Such a
     *      deployment overrides this; every other one inherits the refusal.
     */
    function _checkHandleChainId(bytes32 handle) internal view virtual {
        LibFhevmHandle.checkChainId(handle);
    }

    /**
     * @dev RFC-030's coprocessor step 1, before anything is unmasked: the input package must name the chain this runs
     *      on, and its label must be the hash of its witness. PaddedInputter enforces both on-chain, so on an honest
     *      chain this never fails; the coprocessor repeats them as defence in depth, so that a package submitted to
     *      one chain, or a forged one reusing its label, is never unmasked through a compromised chain. The hash
     *      check is the load-bearing one: the label selects the pad and only the witness ties it to a chain, so a
     *      foreign label under a local `chainId` passes the chain check alone. This contract stands in for the
     *      coprocessor, so it repeats both.
     *      Specified in https://github.com/zama-ai/tech-spec/blob/950587724a33f72066e864a8934ee0909144d147/rfcs/030-pad-based-input.md#L597
     *
     *      `virtual` for the same reason as `_checkHandleChainId`.
     */
    function _checkPadPackage(bytes memory inputPackage) internal view virtual {
        uint64 chainId = LibCleartextPad.packageChainId(inputPackage);
        if (chainId != block.chainid) revert CleartextErrorPadPackageWrongChain(chainId);
        (bytes32 label, bytes32 expectedLabel) = LibCleartextPad.packageLabels(inputPackage);
        if (label != expectedLabel) revert CleartextErrorPadPackageLabelMismatch(label, expectedLabel);
    }

    // -----------------------------------------------------------------------
    // record* entry points (see ICleartextArithmetic) — compute + persist
    //
    // `public`, not `external`, so a contract that inherits this one can drive them directly instead
    // of paying for an `this.record*(...)` call back into itself -- which is what the executor's own
    // cleartext subclass does, on the path that records what it just computed.
    // -----------------------------------------------------------------------

    /// @inheritdoc ICleartextArithmetic
    function recordCast(bytes32 result, bytes32 ct, FheType toType) public override {
        _checkHandleChainId(result);
        _checkHandleChainId(ct);
        ICleartextDB db = _db();
        db.set(result, _fheCast(_operand(db, ct), toType));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordTrivialEncrypt(bytes32 result, uint256 pt, FheType toType) public override {
        _checkHandleChainId(result);
        _db().set(result, _normalizePlaintextToType(pt, toType));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordVerifyInput(bytes32 result, bytes32 inputHandle, bytes memory inputProof, FheType inputType)
        public
        override
    {
        _checkHandleChainId(result);
        uint256 cleartext;
        // Byte 21 tells the two input flows apart: InputVerifier returns the input handle itself, whose
        // byte 21 is its index in the proof (never above 254), while PaddedInputter (RFC-030) returns a
        // fresh compute handle stamped 0xff like any computation.
        if (uint8(result[21]) == 0xff) {
            // `inputHandle` is then the external handle, a digest carrying no chain id, so it is not
            // checked. `inputProof` is the input package, already validated by PaddedInputter; its chain
            // and label are checked again, as the coprocessor does.
            _checkPadPackage(inputProof);
            cleartext = LibCleartextPad.unmaskPart(inputHandle, inputProof, inputType);
        } else {
            _checkHandleChainId(inputHandle);
            bool foundCleartext;
            (foundCleartext, cleartext) = LibCleartextZkPok.tryReadCleartext(inputHandle, inputProof);
            if (!foundCleartext) return;
        }
        _db().set(result, _normalizePlaintextToType(cleartext, inputType));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordRand(bytes32 result, FheType randType, bytes16 seed) public override {
        _checkHandleChainId(result);
        uint256 randomValue = randomUint256(randType, seed);
        _db().set(result, clamp(randomValue, LibFheType.bitWidthForType(randType)));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordRandBounded(bytes32 result, uint256 upperBound, bytes16 seed) public override {
        _checkHandleChainId(result);
        uint256 randomValue = randomBoundedUint256(upperBound, seed);
        _db().set(result, randomValue);
    }

    /// @inheritdoc ICleartextArithmetic
    function recordBinaryOp(Operators op, bytes32 result, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, FheType fheType)
        public
        override
    {
        _checkHandleChainId(result);
        _checkHandleChainId(lhs);
        // A scalar `rhs` is a plaintext, not a handle; only check it when it is one.
        if (scalarByte != 0x01) _checkHandleChainId(rhs);
        ICleartextDB db = _db();
        uint256 lhsValue = _operand(db, lhs);
        uint256 rhsValue = (scalarByte == 0x01) ? uint256(rhs) : _operand(db, rhs);
        db.set(result, _computeBinaryOp(op, lhsValue, rhsValue, fheType, scalarByte));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordMulDiv(
        bytes32 result,
        bytes32 factor1,
        bytes32 factor2,
        bytes32 divisor,
        bytes1 scalarByte,
        FheType fheType
    ) public override {
        _checkHandleChainId(result);
        _checkHandleChainId(factor1);
        // `0x03` marks factor2 as a scalar; only check it when it is a handle.
        if (scalarByte != 0x03) _checkHandleChainId(factor2);
        ICleartextDB db = _db();
        uint256 a = _operand(db, factor1);
        // `0x03` marks factor2 as a scalar (`0x01`: encrypted; the executor admits nothing else). Scalars are
        // read the way `fheDiv`'s are (`_resolveBinaryOperands`): reduced to the operand type.
        uint256 b = (scalarByte == 0x03) ? normalizeScalarToType(uint256(factor2), fheType) : _operand(db, factor2);
        uint256 d = normalizeScalarToType(uint256(divisor), fheType);
        // Two 64-bit factors fit in 128 bits, so the widened product cannot overflow; `d` is non-zero because
        // the executor's `DivisionByZero` check ran on the same reduced value.
        db.set(result, clamp((a * b) / d, LibFheType.bitWidthForType(fheType)));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordUnaryOp(Operators op, bytes32 result, bytes32 ct, FheType fheType) public override {
        _checkHandleChainId(result);
        _checkHandleChainId(ct);
        ICleartextDB db = _db();
        db.set(result, _computeUnaryOp(op, _operand(db, ct), fheType));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordTernaryOp(Operators op, bytes32 result, bytes32 lhs, bytes32 middle, bytes32 rhs) public override {
        _checkHandleChainId(result);
        _checkHandleChainId(lhs);
        _checkHandleChainId(middle);
        _checkHandleChainId(rhs);
        if (op != Operators.fheIfThenElse) revert CleartextErrorUnsupportedTernaryOp(op);
        ICleartextDB db = _db();
        uint256 control = _operand(db, lhs);
        require(control == 0 || control == 1, "Unexpected FheIfThenElse control value");
        db.set(result, (control == 1) ? _operand(db, middle) : _operand(db, rhs));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordNaryOp(Operators op, bytes32 result, bytes32 value, bytes32[] memory values, FheType fheType)
        public
        override
    {
        _checkHandleChainId(result);
        for (uint256 k = 0; k < values.length; k++) {
            _checkHandleChainId(values[k]);
        }
        // Same rule as the needle read below: `fheSum` passes bytes32(0), which is not a handle.
        if (op == Operators.fheIsIn) _checkHandleChainId(value);
        ICleartextDB db = _db();

        uint256[] memory operands = new uint256[](values.length);
        for (uint256 i = 0; i < values.length; i++) {
            operands[i] = _operand(db, values[i]);
        }

        // `fheSum` passes bytes32(0) as `value` (see ICleartextArithmetic), which is not a handle the
        // DB knows — so the needle is only read for the op that actually has one.
        uint256 needle = (op == Operators.fheIsIn) ? _operand(db, value) : 0;

        db.set(result, _computeNaryOp(op, needle, operands, fheType));
    }

    // -----------------------------------------------------------------------
    // Randomness (`view virtual` so a test double can make it deterministic)
    // -----------------------------------------------------------------------

    function randomUint256(FheType randType, bytes16 seed) internal view virtual returns (uint256) {
        return uint256(keccak256(abi.encodePacked(randType, seed, "randValue")));
    }

    function randomBoundedUint256(uint256 upperBound, bytes16 seed) internal view virtual returns (uint256) {
        return (uint256(keccak256(abi.encodePacked(upperBound, seed, "randBoundedValue"))) % upperBound);
    }

    // -----------------------------------------------------------------------
    // Operator semantics (pure; the contract's own bytecode, not the ABI)
    // -----------------------------------------------------------------------

    function _computeBinaryOp(Operators op, uint256 lhsRaw, uint256 rhsRaw, FheType fheType, bytes1 scalarByte)
        internal
        pure
        returns (uint256)
    {
        (uint256 a, uint256 b, uint256 bw) = _resolveBinaryOperands(lhsRaw, rhsRaw, fheType, scalarByte);

        if (op == Operators.fheAdd) return add(a, b, bw);
        if (op == Operators.fheSub) return sub(a, b, bw);
        if (op == Operators.fheMul) return mul(a, b, bw);
        if (op == Operators.fheDiv) return a / b;
        if (op == Operators.fheRem) return a % b;
        if (op == Operators.fheBitAnd) return bitAnd(a, b, bw);
        if (op == Operators.fheBitOr) return bitOr(a, b, bw);
        if (op == Operators.fheBitXor) return bitXor(a, b, bw);
        if (op == Operators.fheShl) return shl(a, b, bw);
        if (op == Operators.fheShr) return shr(a, b, bw);
        if (op == Operators.fheRotl) return rotl(a, b, bw);
        if (op == Operators.fheRotr) return rotr(a, b, bw);
        if (op == Operators.fheEq) return (a == b) ? 1 : 0;
        if (op == Operators.fheNe) return (a != b) ? 1 : 0;
        if (op == Operators.fheGe) return (a >= b) ? 1 : 0;
        if (op == Operators.fheGt) return (a > b) ? 1 : 0;
        if (op == Operators.fheLe) return (a <= b) ? 1 : 0;
        if (op == Operators.fheLt) return (a < b) ? 1 : 0;
        if (op == Operators.fheMin) return (a < b) ? a : b;
        if (op == Operators.fheMax) return (a > b) ? a : b;

        revert CleartextErrorUnsupportedBinaryOp(op);
    }

    function _computeNaryOp(Operators op, uint256 needle, uint256[] memory values, FheType fheType)
        internal
        pure
        returns (uint256)
    {
        if (op == Operators.fheSum) return sum(values, LibFheType.bitWidthForType(fheType));
        // fheIsIn's result is a Bool, so no bit-width clamping applies to the 0/1 it returns.
        if (op == Operators.fheIsIn) return isIn(needle, values);

        revert CleartextErrorUnsupportedNaryOp(op);
    }

    function _computeUnaryOp(Operators op, uint256 valueRaw, FheType fheType) internal pure returns (uint256) {
        uint256 bw = LibFheType.bitWidthForType(fheType);
        if (op == Operators.fheNeg) return neg(valueRaw, bw);
        if (op == Operators.fheNot) return bitNot(valueRaw, bw);

        revert CleartextErrorUnsupportedUnaryOp(op);
    }

    /// @dev Bool matches `trivial_encrypt_be_bytes`: only the least-significant byte matters.
    function _normalizePlaintextToType(uint256 value, FheType fheType) internal pure returns (uint256) {
        if (fheType == FheType.Bool) {
            // An `ebool` plaintext is 0 or 1 and nothing else. Both callers — `recordTrivialEncrypt`
            // and `recordVerifyInput` — receive a value that claims to BE a boolean, so anything wider
            // is a malformed input rather than a value to coerce. Refusing it is the whole point of a
            // mock: silently folding 2 to `true` would let a test pass on a payload the real stack
            // would never have produced.
            //
            // `normalizeScalarToType` is the deliberate exception: a scalar operand is not declared
            // boolean, and there the coprocessor's own `arr_non_zero` rule applies.
            if (value > 1) revert CleartextErrorNotABoolean(value);
            return value;
        }

        // Out of range for the declared type, so REFUSED rather than clamped.
        //
        // This is the one place that rejects rather than wraps, and the distinction is deliberate.
        // Arithmetic is modular because real TFHE arithmetic is modular: `euint8(200) + euint8(100)`
        // genuinely is 44 on the coprocessor, and a mock that reverted would fail programs the real
        // stack accepts. A PLAINTEXT is different — it is an input asserting "this value is a uint8",
        // so a value that is not one is a caller mistake, and clamping would hide it behind a number
        // the test never wrote.
        uint256 bitWidth = LibFheType.bitWidthForType(fheType);
        if (bitWidth < 256 && value >= (uint256(1) << bitWidth)) {
            revert CleartextErrorPlaintextTooWide(value, fheType);
        }
        return value;
    }

    /// @dev While the host contracts disable casting to Bool (prefer using FheNe instead), the
    ///      internals should not be opinionated about it and mirror the coprocessor's behavior.
    function _fheCast(uint256 valueRaw, FheType toType) internal pure returns (uint256) {
        if (toType == FheType.Bool) {
            return valueRaw > 0 ? 1 : 0;
        }
        return clamp(valueRaw, LibFheType.bitWidthForType(toType));
    }

    function clamp(uint256 value, uint256 bitWidth) internal pure returns (uint256) {
        if (bitWidth >= 256) {
            return value;
        }
        return value & ((uint256(1) << bitWidth) - 1);
    }

    /// @dev Bool matches `arr_non_zero`: any non-zero byte in the scalar makes it `true`.
    function normalizeScalarToType(uint256 value, FheType fheType) internal pure returns (uint256) {
        if (fheType == FheType.Bool) {
            return value == 0 ? 0 : 1;
        }
        return clamp(value, LibFheType.bitWidthForType(fheType));
    }

    function add(uint256 a, uint256 b, uint256 bitWidth) internal pure returns (uint256) {
        unchecked {
            return clamp(a + b, bitWidth);
        }
    }

    function sub(uint256 a, uint256 b, uint256 bitWidth) internal pure returns (uint256) {
        unchecked {
            if (bitWidth >= 256) {
                return a - b;
            }
            return clamp(a - b + (uint256(1) << bitWidth), bitWidth);
        }
    }

    function mul(uint256 a, uint256 b, uint256 bitWidth) internal pure returns (uint256) {
        unchecked {
            return clamp(a * b, bitWidth);
        }
    }

    function bitAnd(uint256 a, uint256 b, uint256 bitWidth) internal pure returns (uint256) {
        return clamp(a & b, bitWidth);
    }

    function bitOr(uint256 a, uint256 b, uint256 bitWidth) internal pure returns (uint256) {
        return clamp(a | b, bitWidth);
    }

    function bitXor(uint256 a, uint256 b, uint256 bitWidth) internal pure returns (uint256) {
        return clamp(a ^ b, bitWidth);
    }

    function shl(uint256 a, uint256 b, uint256 bitWidth) internal pure returns (uint256) {
        return clamp(a << (b % bitWidth), bitWidth);
    }

    function shr(uint256 a, uint256 b, uint256 bitWidth) internal pure returns (uint256) {
        return clamp(a >> (b % bitWidth), bitWidth);
    }

    function rotl(uint256 a, uint256 b, uint256 bitWidth) internal pure returns (uint256) {
        uint256 shift = b % bitWidth;
        if (shift == 0) {
            return a;
        }
        return clamp((a << shift) | (a >> (bitWidth - shift)), bitWidth);
    }

    function rotr(uint256 a, uint256 b, uint256 bitWidth) internal pure returns (uint256) {
        uint256 shift = b % bitWidth;
        if (shift == 0) {
            return a;
        }
        return clamp((a >> shift) | (a << (bitWidth - shift)), bitWidth);
    }

    /// @dev Folds through `add` rather than accumulating and clamping once. For any bit-width below 256
    ///      the two agree (the sum mod 2^bw is the same either way), so this is about not having a
    ///      second addition rule to keep in step with `fheAdd` — and about staying unchecked, which a
    ///      bare `+` would not be. An empty set sums to 0.
    function sum(uint256[] memory values, uint256 bitWidth) internal pure returns (uint256) {
        uint256 total = 0;
        for (uint256 i = 0; i < values.length; i++) {
            total = add(total, values[i], bitWidth);
        }
        return total;
    }

    /// @dev Membership as a Bool (1/0). Operands are already normalized to their type by the time they
    ///      reach the DB, so plain equality is the right comparison. An empty set contains nothing.
    function isIn(uint256 value, uint256[] memory values) internal pure returns (uint256) {
        for (uint256 i = 0; i < values.length; i++) {
            if (values[i] == value) {
                return 1;
            }
        }
        return 0;
    }

    function neg(uint256 value, uint256 bitWidth) internal pure returns (uint256) {
        unchecked {
            return clamp(~value + 1, bitWidth);
        }
    }

    function bitNot(uint256 value, uint256 bitWidth) internal pure returns (uint256) {
        if (bitWidth >= 256) {
            return ~value;
        }
        uint256 mask = (uint256(1) << bitWidth) - 1;
        return ~value & mask;
    }

    /// @dev Resolve binary operands for an FHE operation. Encrypted operands are assumed already
    ///      in-range; scalar operands are truncated to the type's bit-width.
    function _resolveBinaryOperands(uint256 lhsRaw, uint256 rhsRaw, FheType fheType, bytes1 scalarByte)
        internal
        pure
        returns (uint256 a, uint256 b, uint256 bw)
    {
        bw = LibFheType.bitWidthForType(fheType);
        a = lhsRaw;
        b = (scalarByte == 0x01) ? normalizeScalarToType(rhsRaw, fheType) : rhsRaw;
    }
}
