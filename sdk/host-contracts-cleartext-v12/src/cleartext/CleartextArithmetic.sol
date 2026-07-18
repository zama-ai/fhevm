// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {FheType} from "../contracts/shared/FheType.sol";
import {FHEVMExecutor} from "../contracts/FHEVMExecutor.sol";
import {FheTypeBitWidth} from "./FheTypeBitWidth.sol";
import {ICleartextArithmetic} from "./ICleartextArithmetic.sol";
import {ICleartextDB} from "./ICleartextDB.sol";
import {cleartextDbAdd} from "../addresses/FHEVMHostAddresses.sol";
import {UUPSUpgradeableEmptyProxy} from "../contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "../contracts/shared/ACLOwnable.sol";

/**
 * @title CleartextArithmetic
 * @notice Cleartext computation + persistence layer for the execution mocks. Deployed as a
 *         standalone upgradeable contract (rather than an inlined library) so its bytecode does not
 *         count against `CleartextFHEVMExecutor`'s EIP-170 size limit. It reads operand cleartexts
 *         from `CleartextDB`, computes results mirroring FHE bit-width semantics, and writes them
 *         back — the executor never touches the DB.
 * @dev The math is `pure` (no local storage); the `record*` entry points perform external DB
 *      reads/writes. This contract is the sole writer registered in `CleartextDB`.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract CleartextArithmetic is ICleartextArithmetic, UUPSUpgradeableEmptyProxy, ACLOwnable {
    error UnsupportedBinaryOp(FHEVMExecutor.Operators op);
    error UnsupportedUnaryOp(FHEVMExecutor.Operators op);
    error UnsupportedTernaryOp(FHEVMExecutor.Operators op);

    /// @dev Name of the contract, used in `getVersion`.
    string private constant CONTRACT_NAME = "CleartextArithmetic";

    /// @dev Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @dev Minor version of the contract.
    uint256 private constant MINOR_VERSION = 3;

    /// @dev Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number used in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and any future `reinitializeVX` method.
     */
    uint64 private constant REINITIALIZER_VERSION = 2;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract from an empty proxy. No state to set (stateless/pure math).
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {}

    /// @notice Getter for the name and version of the contract.
    /// @return string Name and the version of the contract.
    function getVersion() external pure virtual returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    " v",
                    Strings.toString(MAJOR_VERSION),
                    ".",
                    Strings.toString(MINOR_VERSION),
                    ".",
                    Strings.toString(PATCH_VERSION)
                )
            );
    }

    /// @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}

    /// @inheritdoc ICleartextArithmetic
    function plaintexts(bytes32 handle) external view override returns (uint256) {
        return ICleartextDB(cleartextDbAdd).get(handle);
    }

    // -----------------------------------------------------------------------
    // record* entry points (see ICleartextArithmetic) — compute + persist
    // -----------------------------------------------------------------------

    /// @inheritdoc ICleartextArithmetic
    function recordCast(bytes32 result, bytes32 ct, FheType toType) external override {
        ICleartextDB db = ICleartextDB(cleartextDbAdd);
        db.set(result, _fheCast(db.get(ct), toType));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordTrivialEncrypt(bytes32 result, uint256 pt, FheType toType) external override {
        ICleartextDB(cleartextDbAdd).set(result, _normalizePlaintextToType(pt, toType));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordVerifyInput(bytes32 result, bytes32 inputHandle, bytes memory inputProof, FheType inputType)
        external
        override
    {
        (bool foundCleartext, uint256 cleartext) = _tryReadCleartextFromProof(inputHandle, inputProof);
        if (foundCleartext) {
            ICleartextDB(cleartextDbAdd).set(result, _normalizePlaintextToType(cleartext, inputType));
        }
    }

    /// @inheritdoc ICleartextArithmetic
    function recordRand(bytes32 result, FheType randType, bytes16 seed) external override {
        uint256 randomValue = uint256(keccak256(abi.encodePacked(seed, "randValue")));
        ICleartextDB(cleartextDbAdd).set(result, clamp(randomValue, FheTypeBitWidth.bitWidthForType(randType)));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordRandBounded(bytes32 result, uint256 upperBound, bytes16 seed) external override {
        uint256 randomValue = uint256(keccak256(abi.encodePacked(seed, "randBoundedValue")));
        ICleartextDB(cleartextDbAdd).set(result, randomValue % upperBound);
    }

    /// @inheritdoc ICleartextArithmetic
    function recordBinaryOp(
        FHEVMExecutor.Operators op,
        bytes32 result,
        bytes32 lhs,
        bytes32 rhs,
        bytes1 scalarByte,
        FheType fheType
    ) external override {
        ICleartextDB db = ICleartextDB(cleartextDbAdd);
        uint256 lhsValue = db.get(lhs);
        uint256 rhsValue = (scalarByte == 0x01) ? uint256(rhs) : db.get(rhs);
        db.set(result, _computeBinaryOp(op, lhsValue, rhsValue, fheType, scalarByte));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordUnaryOp(FHEVMExecutor.Operators op, bytes32 result, bytes32 ct, FheType fheType) external override {
        ICleartextDB db = ICleartextDB(cleartextDbAdd);
        db.set(result, _computeUnaryOp(op, db.get(ct), fheType));
    }

    /// @inheritdoc ICleartextArithmetic
    function recordTernaryOp(FHEVMExecutor.Operators op, bytes32 result, bytes32 lhs, bytes32 middle, bytes32 rhs)
        external
        override
    {
        if (op != FHEVMExecutor.Operators.fheIfThenElse) revert UnsupportedTernaryOp(op);
        ICleartextDB db = ICleartextDB(cleartextDbAdd);
        uint256 control = db.get(lhs);
        require(control == 0 || control == 1, "Unexpected FheIfThenElse control value");
        db.set(result, (control == 1) ? db.get(middle) : db.get(rhs));
    }

    // -----------------------------------------------------------------------
    // Internal computation (pure; the contract's own bytecode, not the ABI)
    // -----------------------------------------------------------------------

    function _computeBinaryOp(FHEVMExecutor.Operators op, uint256 lhsRaw, uint256 rhsRaw, FheType fheType, bytes1 scalarByte)
        internal
        pure
        returns (uint256)
    {
        (uint256 a, uint256 b, uint256 bw) = _resolveBinaryOperands(lhsRaw, rhsRaw, fheType, scalarByte);

        if (op == FHEVMExecutor.Operators.fheAdd) return add(a, b, bw);
        if (op == FHEVMExecutor.Operators.fheSub) return sub(a, b, bw);
        if (op == FHEVMExecutor.Operators.fheMul) return mul(a, b, bw);
        if (op == FHEVMExecutor.Operators.fheDiv) return a / b;
        if (op == FHEVMExecutor.Operators.fheRem) return a % b;
        if (op == FHEVMExecutor.Operators.fheBitAnd) return bitAnd(a, b, bw);
        if (op == FHEVMExecutor.Operators.fheBitOr) return bitOr(a, b, bw);
        if (op == FHEVMExecutor.Operators.fheBitXor) return bitXor(a, b, bw);
        if (op == FHEVMExecutor.Operators.fheShl) return shl(a, b, bw);
        if (op == FHEVMExecutor.Operators.fheShr) return shr(a, b, bw);
        if (op == FHEVMExecutor.Operators.fheRotl) return rotl(a, b, bw);
        if (op == FHEVMExecutor.Operators.fheRotr) return rotr(a, b, bw);
        if (op == FHEVMExecutor.Operators.fheEq) return (a == b) ? 1 : 0;
        if (op == FHEVMExecutor.Operators.fheNe) return (a != b) ? 1 : 0;
        if (op == FHEVMExecutor.Operators.fheGe) return (a >= b) ? 1 : 0;
        if (op == FHEVMExecutor.Operators.fheGt) return (a > b) ? 1 : 0;
        if (op == FHEVMExecutor.Operators.fheLe) return (a <= b) ? 1 : 0;
        if (op == FHEVMExecutor.Operators.fheLt) return (a < b) ? 1 : 0;
        if (op == FHEVMExecutor.Operators.fheMin) return (a < b) ? a : b;
        if (op == FHEVMExecutor.Operators.fheMax) return (a > b) ? a : b;

        revert UnsupportedBinaryOp(op);
    }

    function _computeUnaryOp(FHEVMExecutor.Operators op, uint256 valueRaw, FheType fheType)
        internal
        pure
        returns (uint256)
    {
        uint256 bw = FheTypeBitWidth.bitWidthForType(fheType);
        if (op == FHEVMExecutor.Operators.fheNeg) return neg(valueRaw, bw);
        if (op == FHEVMExecutor.Operators.fheNot) return bitNot(valueRaw, bw);

        revert UnsupportedUnaryOp(op);
    }

    /// @dev Bool matches `trivial_encrypt_be_bytes`: only the least-significant byte matters.
    function _normalizePlaintextToType(uint256 value, FheType fheType) internal pure returns (uint256) {
        if (fheType == FheType.Bool) {
            // forge-lint: disable-next-line(unsafe-typecast)
            return uint8(value) > 0 ? 1 : 0;
        }
        return clamp(value, FheTypeBitWidth.bitWidthForType(fheType));
    }

    /// @dev While the host contracts disable casting to Bool (prefer using FheNe instead), the
    ///      internals should not be opinionated about it and mirror the coprocessor's behavior.
    function _fheCast(uint256 valueRaw, FheType toType) internal pure returns (uint256) {
        if (toType == FheType.Bool) {
            return valueRaw > 0 ? 1 : 0;
        }
        return clamp(valueRaw, FheTypeBitWidth.bitWidthForType(toType));
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
        return clamp(value, FheTypeBitWidth.bitWidthForType(fheType));
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
        bw = FheTypeBitWidth.bitWidthForType(fheType);
        a = lhsRaw;
        b = (scalarByte == 0x01) ? normalizeScalarToType(rhsRaw, fheType) : rhsRaw;
    }

    function _tryReadCleartextFromProof(bytes32 inputHandle, bytes memory inputProof)
        internal
        pure
        returns (bool foundCleartext, uint256 cleartext)
    {
        if (inputProof.length < 2) {
            return (false, 0);
        }

        uint8 numHandles = uint8(inputProof[0]);
        uint8 numSigners = uint8(inputProof[1]);
        uint256 cleartextStart = 2 + uint256(numHandles) * 32 + uint256(numSigners) * 65;

        if (inputProof.length < cleartextStart + 32) {
            return (false, 0);
        }

        for (uint8 i = 0; i < numHandles; i++) {
            uint256 handleOffset = 2 + uint256(i) * 32;
            bytes32 handleInProof;
            assembly {
                handleInProof := mload(add(add(inputProof, 32), handleOffset))
            }

            if (handleInProof != inputHandle) {
                continue;
            }

            uint256 cleartextOffset = cleartextStart + uint256(i) * 32;
            if (inputProof.length < cleartextOffset + 32) {
                return (false, 0);
            }

            assembly {
                cleartext := mload(add(add(inputProof, 32), cleartextOffset))
            }
            return (true, cleartext);
        }

        return (false, 0);
    }
}
