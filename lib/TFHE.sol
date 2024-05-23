// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

type ebool is uint256;
type euint4 is uint256;
type euint8 is uint256;
type euint16 is uint256;
type euint32 is uint256;
type euint64 is uint256;
type eaddress is uint256;

library Common {
    // Values used to communicate types to the runtime.
    uint8 internal constant ebool_t = 0;
    uint8 internal constant euint4_t = 1;
    uint8 internal constant euint8_t = 2;
    uint8 internal constant euint16_t = 3;
    uint8 internal constant euint32_t = 4;
    uint8 internal constant euint64_t = 5;
    uint8 internal constant euint128_t = 6;
    uint8 internal constant euint160_t = 7;
}

import "./Impl.sol";

library TFHE {
    euint4 constant NIL4 = euint4.wrap(0);
    euint8 constant NIL8 = euint8.wrap(0);
    euint16 constant NIL16 = euint16.wrap(0);
    euint32 constant NIL32 = euint32.wrap(0);
    euint64 constant NIL64 = euint64.wrap(0);

    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(ebool v) internal pure returns (bool) {
        return ebool.unwrap(v) != 0;
    }

    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(euint4 v) internal pure returns (bool) {
        return euint4.unwrap(v) != 0;
    }

    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(euint8 v) internal pure returns (bool) {
        return euint8.unwrap(v) != 0;
    }

    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(euint16 v) internal pure returns (bool) {
        return euint16.unwrap(v) != 0;
    }

    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(euint32 v) internal pure returns (bool) {
        return euint32.unwrap(v) != 0;
    }

    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(euint64 v) internal pure returns (bool) {
        return euint64.unwrap(v) != 0;
    }

    // Evaluate add(a, b) and return the result.
    function add(euint4 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.add(euint4.unwrap(a), euint4.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint4 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.sub(euint4.unwrap(a), euint4.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint4 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.mul(euint4.unwrap(a), euint4.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint4 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.and(euint4.unwrap(a), euint4.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint4 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.or(euint4.unwrap(a), euint4.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint4 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.xor(euint4.unwrap(a), euint4.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint4 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.eq(euint4.unwrap(a), euint4.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint4 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ne(euint4.unwrap(a), euint4.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint4 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ge(euint4.unwrap(a), euint4.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint4 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.gt(euint4.unwrap(a), euint4.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint4 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.le(euint4.unwrap(a), euint4.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint4 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.lt(euint4.unwrap(a), euint4.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint4 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.min(euint4.unwrap(a), euint4.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint4 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.max(euint4.unwrap(a), euint4.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint4 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.add(euint8.unwrap(asEuint8(a)), euint8.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint4 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.sub(euint8.unwrap(asEuint8(a)), euint8.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint4 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.mul(euint8.unwrap(asEuint8(a)), euint8.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint4 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.and(euint8.unwrap(asEuint8(a)), euint8.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint4 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.or(euint8.unwrap(asEuint8(a)), euint8.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint4 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.xor(euint8.unwrap(asEuint8(a)), euint8.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint4 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint8.unwrap(asEuint8(a)), euint8.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint4 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint8.unwrap(asEuint8(a)), euint8.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint4 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint8.unwrap(asEuint8(a)), euint8.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint4 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint8.unwrap(asEuint8(a)), euint8.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint4 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint8.unwrap(asEuint8(a)), euint8.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint4 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint8.unwrap(asEuint8(a)), euint8.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint4 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.min(euint8.unwrap(asEuint8(a)), euint8.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint4 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.max(euint8.unwrap(asEuint8(a)), euint8.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint4 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint4 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint4 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint4 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.and(euint16.unwrap(asEuint16(a)), euint16.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint4 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.or(euint16.unwrap(asEuint16(a)), euint16.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint4 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.xor(euint16.unwrap(asEuint16(a)), euint16.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint4 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint4 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint4 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint4 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint4 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint4 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint4 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint4 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint4 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint4 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint4 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint4 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(asEuint32(a)), euint32.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint4 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(asEuint32(a)), euint32.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint4 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(asEuint32(a)), euint32.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint4 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint4 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint4 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint4 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint4 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint4 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint4 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint4 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint4 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint4 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint4 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint4 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint4 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint4 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint4 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint4 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint4 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint4 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint4 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint4 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint4 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint4 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint4 a, uint8 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return euint4.wrap(Impl.add(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate add(a, b) and return the result.
    function add(uint8 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.add(euint4.unwrap(b), uint256(a), true));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint4 a, uint8 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return euint4.wrap(Impl.sub(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(uint8 a, euint4 b) internal pure returns (euint4) {
        euint4 aEnc = asEuint4(a);
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.sub(euint4.unwrap(aEnc), euint4.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint4 a, uint8 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return euint4.wrap(Impl.mul(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(uint8 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.mul(euint4.unwrap(b), uint256(a), true));
    }

    // Evaluate div(a, b) and return the result.
    function div(euint4 a, uint8 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return euint4.wrap(Impl.div(euint4.unwrap(a), uint256(b)));
    }

    // Evaluate rem(a, b) and return the result.
    function rem(euint4 a, uint8 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return euint4.wrap(Impl.rem(euint4.unwrap(a), uint256(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint4 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return ebool.wrap(Impl.eq(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(uint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.eq(euint4.unwrap(b), uint256(a), true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint4 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return ebool.wrap(Impl.ne(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(uint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ne(euint4.unwrap(b), uint256(a), true));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint4 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return ebool.wrap(Impl.ge(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(uint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.le(euint4.unwrap(b), uint256(a), true));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint4 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return ebool.wrap(Impl.gt(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(uint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.lt(euint4.unwrap(b), uint256(a), true));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint4 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return ebool.wrap(Impl.le(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate le(a, b) and return the result.
    function le(uint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ge(euint4.unwrap(b), uint256(a), true));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint4 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return ebool.wrap(Impl.lt(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(uint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.gt(euint4.unwrap(b), uint256(a), true));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint4 a, uint8 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return euint4.wrap(Impl.min(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate min(a, b) and return the result.
    function min(uint8 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.min(euint4.unwrap(b), uint256(a), true));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint4 a, uint8 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return euint4.wrap(Impl.max(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate max(a, b) and return the result.
    function max(uint8 a, euint4 b) internal pure returns (euint4) {
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint4.wrap(Impl.max(euint4.unwrap(b), uint256(a), true));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint8 a, euint4 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint8.wrap(Impl.add(euint8.unwrap(a), euint8.unwrap(asEuint8(b)), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint8 a, euint4 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint8.wrap(Impl.sub(euint8.unwrap(a), euint8.unwrap(asEuint8(b)), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint8 a, euint4 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint8.wrap(Impl.mul(euint8.unwrap(a), euint8.unwrap(asEuint8(b)), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint8 a, euint4 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint8.wrap(Impl.and(euint8.unwrap(a), euint8.unwrap(asEuint8(b))));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint8 a, euint4 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint8.wrap(Impl.or(euint8.unwrap(a), euint8.unwrap(asEuint8(b))));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint8 a, euint4 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint8.wrap(Impl.xor(euint8.unwrap(a), euint8.unwrap(asEuint8(b))));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.eq(euint8.unwrap(a), euint8.unwrap(asEuint8(b)), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ne(euint8.unwrap(a), euint8.unwrap(asEuint8(b)), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ge(euint8.unwrap(a), euint8.unwrap(asEuint8(b)), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.gt(euint8.unwrap(a), euint8.unwrap(asEuint8(b)), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.le(euint8.unwrap(a), euint8.unwrap(asEuint8(b)), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint8 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.lt(euint8.unwrap(a), euint8.unwrap(asEuint8(b)), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint8 a, euint4 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint8.wrap(Impl.min(euint8.unwrap(a), euint8.unwrap(asEuint8(b)), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint8 a, euint4 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint8.wrap(Impl.max(euint8.unwrap(a), euint8.unwrap(asEuint8(b)), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.add(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.sub(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.mul(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.and(euint8.unwrap(a), euint8.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.or(euint8.unwrap(a), euint8.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.xor(euint8.unwrap(a), euint8.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.min(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.max(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint8 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint8 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint8 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint8 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.and(euint16.unwrap(asEuint16(a)), euint16.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint8 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.or(euint16.unwrap(asEuint16(a)), euint16.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint8 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.xor(euint16.unwrap(asEuint16(a)), euint16.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint8 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint8 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint8 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint8 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint8 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint8 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint8 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint8 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint8 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint8 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint8 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint8 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(asEuint32(a)), euint32.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint8 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(asEuint32(a)), euint32.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint8 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(asEuint32(a)), euint32.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint8 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint8 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint8 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint8 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint8 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint8 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint8 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint8 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint8 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint8 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint8 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint8 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint8 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint8 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint8 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint8 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint8 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint8 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint8 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint8 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint8 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint8 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint8 a, uint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.add(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate add(a, b) and return the result.
    function add(uint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.add(euint8.unwrap(b), uint256(a), true));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint8 a, uint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.sub(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(uint8 a, euint8 b) internal pure returns (euint8) {
        euint8 aEnc = asEuint8(a);
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.sub(euint8.unwrap(aEnc), euint8.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint8 a, uint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.mul(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(uint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.mul(euint8.unwrap(b), uint256(a), true));
    }

    // Evaluate div(a, b) and return the result.
    function div(euint8 a, uint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.div(euint8.unwrap(a), uint256(b)));
    }

    // Evaluate rem(a, b) and return the result.
    function rem(euint8 a, uint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.rem(euint8.unwrap(a), uint256(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint8 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(uint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint8.unwrap(b), uint256(a), true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint8 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(uint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint8.unwrap(b), uint256(a), true));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint8 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(uint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint8.unwrap(b), uint256(a), true));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint8 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(uint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint8.unwrap(b), uint256(a), true));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint8 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate le(a, b) and return the result.
    function le(uint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint8.unwrap(b), uint256(a), true));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint8 a, uint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(uint8 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint8.unwrap(b), uint256(a), true));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint8 a, uint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.min(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate min(a, b) and return the result.
    function min(uint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.min(euint8.unwrap(b), uint256(a), true));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint8 a, uint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.max(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate max(a, b) and return the result.
    function max(uint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.max(euint8.unwrap(b), uint256(a), true));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint16 a, euint4 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint16 a, euint4 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint16 a, euint4 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint16 a, euint4 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint16.wrap(Impl.and(euint16.unwrap(a), euint16.unwrap(asEuint16(b))));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint16 a, euint4 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint16.wrap(Impl.or(euint16.unwrap(a), euint16.unwrap(asEuint16(b))));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint16 a, euint4 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint16.wrap(Impl.xor(euint16.unwrap(a), euint16.unwrap(asEuint16(b))));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint16 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint16 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint16 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint16 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint16 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint16 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint16 a, euint4 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint16 a, euint4 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.and(euint16.unwrap(a), euint16.unwrap(asEuint16(b))));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.or(euint16.unwrap(a), euint16.unwrap(asEuint16(b))));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.xor(euint16.unwrap(a), euint16.unwrap(asEuint16(b))));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint16 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint16 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint16 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint16 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint16 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint16 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.and(euint16.unwrap(a), euint16.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.or(euint16.unwrap(a), euint16.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.xor(euint16.unwrap(a), euint16.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint16 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint16 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint16 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint16 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(asEuint32(a)), euint32.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint16 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(asEuint32(a)), euint32.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint16 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(asEuint32(a)), euint32.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint16 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint16 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint16 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint16 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint16 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint16 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint16 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint16 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint16 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint16 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint16 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint16 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint16 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint16 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint16 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint16 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint16 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint16 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint16 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint16 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint16 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint16 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint16 a, uint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate add(a, b) and return the result.
    function add(uint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(b), uint256(a), true));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint16 a, uint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(uint16 a, euint16 b) internal pure returns (euint16) {
        euint16 aEnc = asEuint16(a);
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(aEnc), euint16.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint16 a, uint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(uint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(b), uint256(a), true));
    }

    // Evaluate div(a, b) and return the result.
    function div(euint16 a, uint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.div(euint16.unwrap(a), uint256(b)));
    }

    // Evaluate rem(a, b) and return the result.
    function rem(euint16 a, uint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.rem(euint16.unwrap(a), uint256(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint16 a, uint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(uint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(b), uint256(a), true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint16 a, uint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(uint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(b), uint256(a), true));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint16 a, uint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(uint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(b), uint256(a), true));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint16 a, uint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(uint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(b), uint256(a), true));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint16 a, uint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate le(a, b) and return the result.
    function le(uint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(b), uint256(a), true));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint16 a, uint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(uint16 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(b), uint256(a), true));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint16 a, uint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate min(a, b) and return the result.
    function min(uint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(b), uint256(a), true));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint16 a, uint16 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate max(a, b) and return the result.
    function max(uint16 a, euint16 b) internal pure returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(b), uint256(a), true));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint32 a, euint4 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint32 a, euint4 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint32 a, euint4 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint32 a, euint4 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(a), euint32.unwrap(asEuint32(b))));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint32 a, euint4 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(a), euint32.unwrap(asEuint32(b))));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint32 a, euint4 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(a), euint32.unwrap(asEuint32(b))));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint32 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint32 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint32 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint32 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint32 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint32 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint32 a, euint4 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint32 a, euint4 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(a), euint32.unwrap(asEuint32(b))));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(a), euint32.unwrap(asEuint32(b))));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(a), euint32.unwrap(asEuint32(b))));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint32 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint32 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint32 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint32 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint32 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint32 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint32 a, euint16 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint32 a, euint16 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint32 a, euint16 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint32 a, euint16 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(a), euint32.unwrap(asEuint32(b))));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint32 a, euint16 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(a), euint32.unwrap(asEuint32(b))));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint32 a, euint16 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(a), euint32.unwrap(asEuint32(b))));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint32 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint32 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint32 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint32 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint32 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint32 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint32 a, euint16 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint32 a, euint16 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(a), euint32.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(a), euint32.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(a), euint32.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint32 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint32 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint32 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint32 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint32 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint32 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(asEuint64(a)), euint64.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint32 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint32 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint32 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint32 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint32 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint32 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint32 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint32 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint32 a, uint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate add(a, b) and return the result.
    function add(uint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(b), uint256(a), true));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint32 a, uint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(uint32 a, euint32 b) internal pure returns (euint32) {
        euint32 aEnc = asEuint32(a);
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(aEnc), euint32.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint32 a, uint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(uint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(b), uint256(a), true));
    }

    // Evaluate div(a, b) and return the result.
    function div(euint32 a, uint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.div(euint32.unwrap(a), uint256(b)));
    }

    // Evaluate rem(a, b) and return the result.
    function rem(euint32 a, uint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.rem(euint32.unwrap(a), uint256(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint32 a, uint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(uint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(b), uint256(a), true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint32 a, uint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(uint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(b), uint256(a), true));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint32 a, uint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(uint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(b), uint256(a), true));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint32 a, uint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(uint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(b), uint256(a), true));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint32 a, uint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate le(a, b) and return the result.
    function le(uint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(b), uint256(a), true));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint32 a, uint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(uint32 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(b), uint256(a), true));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint32 a, uint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate min(a, b) and return the result.
    function min(uint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(b), uint256(a), true));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint32 a, uint32 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate max(a, b) and return the result.
    function max(uint32 a, euint32 b) internal pure returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(b), uint256(a), true));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint64 a, euint4 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint64 a, euint4 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint64 a, euint4 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint64 a, euint4 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint64 a, euint4 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint64 a, euint4 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint64 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint64 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint64 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint64 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint64 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint64 a, euint4 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint64 a, euint4 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint64 a, euint4 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint4(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint64 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint64 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint64 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint64 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint64 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint64 a, euint8 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint64 a, euint16 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint64 a, euint16 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint64 a, euint16 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint64 a, euint16 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint64 a, euint16 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint64 a, euint16 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint64 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint64 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint64 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint64 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint64 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint64 a, euint16 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint64 a, euint16 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint64 a, euint16 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint64 a, euint32 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint64 a, euint32 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint64 a, euint32 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint64 a, euint32 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint64 a, euint32 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint64 a, euint32 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(a), euint64.unwrap(asEuint64(b))));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint64 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint64 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint64 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint64 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint64 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint64 a, euint32 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint64 a, euint32 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint64 a, euint32 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    // Evaluate and(a, b) and return the result.
    function and(euint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(a), euint64.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(euint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(a), euint64.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(euint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(a), euint64.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    // Evaluate add(a, b) and return the result.
    function add(euint64 a, uint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate add(a, b) and return the result.
    function add(uint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(b), uint256(a), true));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(euint64 a, uint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate sub(a, b) and return the result.
    function sub(uint64 a, euint64 b) internal pure returns (euint64) {
        euint64 aEnc = asEuint64(a);
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(aEnc), euint64.unwrap(b), false));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(euint64 a, uint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate mul(a, b) and return the result.
    function mul(uint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(b), uint256(a), true));
    }

    // Evaluate div(a, b) and return the result.
    function div(euint64 a, uint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.div(euint64.unwrap(a), uint256(b)));
    }

    // Evaluate rem(a, b) and return the result.
    function rem(euint64 a, uint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.rem(euint64.unwrap(a), uint256(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(euint64 a, uint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(uint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(b), uint256(a), true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(euint64 a, uint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(uint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(b), uint256(a), true));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(euint64 a, uint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate ge(a, b) and return the result.
    function ge(uint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(b), uint256(a), true));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(euint64 a, uint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate gt(a, b) and return the result.
    function gt(uint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(b), uint256(a), true));
    }

    // Evaluate le(a, b) and return the result.
    function le(euint64 a, uint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate le(a, b) and return the result.
    function le(uint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(b), uint256(a), true));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(euint64 a, uint64 b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate lt(a, b) and return the result.
    function lt(uint64 a, euint64 b) internal pure returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(b), uint256(a), true));
    }

    // Evaluate min(a, b) and return the result.
    function min(euint64 a, uint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate min(a, b) and return the result.
    function min(uint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(b), uint256(a), true));
    }

    // Evaluate max(a, b) and return the result.
    function max(euint64 a, uint64 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate max(a, b) and return the result.
    function max(uint64 a, euint64 b) internal pure returns (euint64) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(b), uint256(a), true));
    }

    // Evaluate shl(a, b) and return the result.
    function shl(euint4 a, uint8 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return euint4.wrap(Impl.shl(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate shr(a, b) and return the result.
    function shr(euint4 a, uint8 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return euint4.wrap(Impl.shr(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate rotl(a, b) and return the result.
    function rotl(euint4 a, uint8 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return euint4.wrap(Impl.rotl(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate rotr(a, b) and return the result.
    function rotr(euint4 a, uint8 b) internal pure returns (euint4) {
        if (!isInitialized(a)) {
            a = asEuint4(0);
        }
        return euint4.wrap(Impl.rotr(euint4.unwrap(a), uint256(b), true));
    }

    // Evaluate shl(a, b) and return the result.
    function shl(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.shl(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate shl(a, b) and return the result.
    function shl(euint8 a, uint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.shl(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate shr(a, b) and return the result.
    function shr(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.shr(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate shr(a, b) and return the result.
    function shr(euint8 a, uint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.shr(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate rotl(a, b) and return the result.
    function rotl(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.rotl(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate rotl(a, b) and return the result.
    function rotl(euint8 a, uint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.rotl(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate rotr(a, b) and return the result.
    function rotr(euint8 a, euint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.rotr(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    // Evaluate rotr(a, b) and return the result.
    function rotr(euint8 a, uint8 b) internal pure returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.rotr(euint8.unwrap(a), uint256(b), true));
    }

    // Evaluate shl(a, b) and return the result.
    function shl(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.shl(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate shl(a, b) and return the result.
    function shl(euint16 a, uint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.shl(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate shr(a, b) and return the result.
    function shr(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.shr(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate shr(a, b) and return the result.
    function shr(euint16 a, uint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.shr(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate rotl(a, b) and return the result.
    function rotl(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.rotl(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate rotl(a, b) and return the result.
    function rotl(euint16 a, uint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.rotl(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate rotr(a, b) and return the result.
    function rotr(euint16 a, euint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.rotr(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    // Evaluate rotr(a, b) and return the result.
    function rotr(euint16 a, uint8 b) internal pure returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.rotr(euint16.unwrap(a), uint256(b), true));
    }

    // Evaluate shl(a, b) and return the result.
    function shl(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.shl(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate shl(a, b) and return the result.
    function shl(euint32 a, uint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.shl(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate shr(a, b) and return the result.
    function shr(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.shr(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate shr(a, b) and return the result.
    function shr(euint32 a, uint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.shr(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate rotl(a, b) and return the result.
    function rotl(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.rotl(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate rotl(a, b) and return the result.
    function rotl(euint32 a, uint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.rotl(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate rotr(a, b) and return the result.
    function rotr(euint32 a, euint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.rotr(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    // Evaluate rotr(a, b) and return the result.
    function rotr(euint32 a, uint8 b) internal pure returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.rotr(euint32.unwrap(a), uint256(b), true));
    }

    // Evaluate shl(a, b) and return the result.
    function shl(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.shl(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate shl(a, b) and return the result.
    function shl(euint64 a, uint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.shl(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate shr(a, b) and return the result.
    function shr(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.shr(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate shr(a, b) and return the result.
    function shr(euint64 a, uint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.shr(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate rotl(a, b) and return the result.
    function rotl(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.rotl(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate rotl(a, b) and return the result.
    function rotl(euint64 a, uint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.rotl(euint64.unwrap(a), uint256(b), true));
    }

    // Evaluate rotr(a, b) and return the result.
    function rotr(euint64 a, euint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.rotr(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    // Evaluate rotr(a, b) and return the result.
    function rotr(euint64 a, uint8 b) internal pure returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.rotr(euint64.unwrap(a), uint256(b), true));
    }

    // If 'control''s value is 'true', the result has the same value as 'a'.
    // If 'control''s value is 'false', the result has the same value as 'b'.
    function cmux(ebool control, euint4 a, euint4 b) internal pure returns (euint4) {
        return euint4.wrap(Impl.select(ebool.unwrap(control), euint4.unwrap(a), euint4.unwrap(b)));
    }

    function select(ebool control, euint4 a, euint4 b) internal pure returns (euint4) {
        return euint4.wrap(Impl.select(ebool.unwrap(control), euint4.unwrap(a), euint4.unwrap(b)));
    }

    // If 'control''s value is 'true', the result has the same value as 'a'.
    // If 'control''s value is 'false', the result has the same value as 'b'.
    function cmux(ebool control, euint8 a, euint8 b) internal pure returns (euint8) {
        return euint8.wrap(Impl.select(ebool.unwrap(control), euint8.unwrap(a), euint8.unwrap(b)));
    }

    function select(ebool control, euint8 a, euint8 b) internal pure returns (euint8) {
        return euint8.wrap(Impl.select(ebool.unwrap(control), euint8.unwrap(a), euint8.unwrap(b)));
    }

    // If 'control''s value is 'true', the result has the same value as 'a'.
    // If 'control''s value is 'false', the result has the same value as 'b'.
    function cmux(ebool control, euint16 a, euint16 b) internal pure returns (euint16) {
        return euint16.wrap(Impl.select(ebool.unwrap(control), euint16.unwrap(a), euint16.unwrap(b)));
    }

    function select(ebool control, euint16 a, euint16 b) internal pure returns (euint16) {
        return euint16.wrap(Impl.select(ebool.unwrap(control), euint16.unwrap(a), euint16.unwrap(b)));
    }

    // If 'control''s value is 'true', the result has the same value as 'a'.
    // If 'control''s value is 'false', the result has the same value as 'b'.
    function cmux(ebool control, euint32 a, euint32 b) internal pure returns (euint32) {
        return euint32.wrap(Impl.select(ebool.unwrap(control), euint32.unwrap(a), euint32.unwrap(b)));
    }

    function select(ebool control, euint32 a, euint32 b) internal pure returns (euint32) {
        return euint32.wrap(Impl.select(ebool.unwrap(control), euint32.unwrap(a), euint32.unwrap(b)));
    }

    // If 'control''s value is 'true', the result has the same value as 'a'.
    // If 'control''s value is 'false', the result has the same value as 'b'.
    function cmux(ebool control, euint64 a, euint64 b) internal pure returns (euint64) {
        return euint64.wrap(Impl.select(ebool.unwrap(control), euint64.unwrap(a), euint64.unwrap(b)));
    }

    function select(ebool control, euint64 a, euint64 b) internal pure returns (euint64) {
        return euint64.wrap(Impl.select(ebool.unwrap(control), euint64.unwrap(a), euint64.unwrap(b)));
    }

    function eq(euint4[] memory a, euint4[] memory b) internal pure returns (ebool) {
        require(larray.length != rarray.length, "Both arrays are not of the same size.");
        uint256[] memory larray;
        uint256[] memory rarray;
        for (uint i = 0; i < a.length; i++) {
            larray[i] = euint4.unwrap(a[i]);
        }
        for (uint i = 0; i < b.length; i++) {
            rarray[i] = euint4.unwrap(a[i]);
        }
        return ebool.wrap(Impl.eq(larray, rarray));
    }

    function eq(euint8[] memory a, euint8[] memory b) internal pure returns (ebool) {
        require(larray.length != rarray.length, "Both arrays are not of the same size.");
        uint256[] memory larray;
        uint256[] memory rarray;
        for (uint i = 0; i < a.length; i++) {
            larray[i] = euint8.unwrap(a[i]);
        }
        for (uint i = 0; i < b.length; i++) {
            rarray[i] = euint8.unwrap(a[i]);
        }
        return ebool.wrap(Impl.eq(larray, rarray));
    }

    function eq(euint16[] memory a, euint16[] memory b) internal pure returns (ebool) {
        require(larray.length != rarray.length, "Both arrays are not of the same size.");
        uint256[] memory larray;
        uint256[] memory rarray;
        for (uint i = 0; i < a.length; i++) {
            larray[i] = euint16.unwrap(a[i]);
        }
        for (uint i = 0; i < b.length; i++) {
            rarray[i] = euint16.unwrap(a[i]);
        }
        return ebool.wrap(Impl.eq(larray, rarray));
    }

    function eq(euint32[] memory a, euint32[] memory b) internal pure returns (ebool) {
        require(larray.length != rarray.length, "Both arrays are not of the same size.");
        uint256[] memory larray;
        uint256[] memory rarray;
        for (uint i = 0; i < a.length; i++) {
            larray[i] = euint32.unwrap(a[i]);
        }
        for (uint i = 0; i < b.length; i++) {
            rarray[i] = euint32.unwrap(a[i]);
        }
        return ebool.wrap(Impl.eq(larray, rarray));
    }

    function eq(euint64[] memory a, euint64[] memory b) internal pure returns (ebool) {
        require(larray.length != rarray.length, "Both arrays are not of the same size.");
        uint256[] memory larray;
        uint256[] memory rarray;
        for (uint i = 0; i < a.length; i++) {
            larray[i] = euint64.unwrap(a[i]);
        }
        for (uint i = 0; i < b.length; i++) {
            rarray[i] = euint64.unwrap(a[i]);
        }
        return ebool.wrap(Impl.eq(larray, rarray));
    }

    // Cast an encrypted integer from euint8 to euint4.
    function asEuint4(euint8 value) internal pure returns (euint4) {
        return euint4.wrap(Impl.cast(euint8.unwrap(value), Common.euint4_t));
    }

    // Cast an encrypted integer from euint16 to euint4.
    function asEuint4(euint16 value) internal pure returns (euint4) {
        return euint4.wrap(Impl.cast(euint16.unwrap(value), Common.euint4_t));
    }

    // Cast an encrypted integer from euint32 to euint4.
    function asEuint4(euint32 value) internal pure returns (euint4) {
        return euint4.wrap(Impl.cast(euint32.unwrap(value), Common.euint4_t));
    }

    // Cast an encrypted integer from euint64 to euint4.
    function asEuint4(euint64 value) internal pure returns (euint4) {
        return euint4.wrap(Impl.cast(euint64.unwrap(value), Common.euint4_t));
    }

    // Cast an encrypted integer from euint4 to ebool.
    function asEbool(euint4 value) internal pure returns (ebool) {
        return ne(value, 0);
    }

    // Converts an 'ebool' to an 'euint4'.
    function asEuint4(ebool b) internal pure returns (euint4) {
        return euint4.wrap(Impl.cast(ebool.unwrap(b), Common.euint4_t));
    }

    // Cast an encrypted integer from euint4 to euint8.
    function asEuint8(euint4 value) internal pure returns (euint8) {
        return euint8.wrap(Impl.cast(euint4.unwrap(value), Common.euint8_t));
    }

    // Cast an encrypted integer from euint16 to euint8.
    function asEuint8(euint16 value) internal pure returns (euint8) {
        return euint8.wrap(Impl.cast(euint16.unwrap(value), Common.euint8_t));
    }

    // Cast an encrypted integer from euint32 to euint8.
    function asEuint8(euint32 value) internal pure returns (euint8) {
        return euint8.wrap(Impl.cast(euint32.unwrap(value), Common.euint8_t));
    }

    // Cast an encrypted integer from euint64 to euint8.
    function asEuint8(euint64 value) internal pure returns (euint8) {
        return euint8.wrap(Impl.cast(euint64.unwrap(value), Common.euint8_t));
    }

    // Cast an encrypted integer from euint8 to ebool.
    function asEbool(euint8 value) internal pure returns (ebool) {
        return ne(value, 0);
    }

    // Convert a serialized 'ciphertext' to an encrypted euint8 integer.
    function asEbool(bytes memory ciphertext) internal pure returns (ebool) {
        return ebool.wrap(Impl.verify(ciphertext, Common.ebool_t));
    }

    // Convert a plaintext value to an encrypted euint8 integer.
    function asEbool(uint256 value) internal pure returns (ebool) {
        return ebool.wrap(Impl.trivialEncrypt(value, Common.ebool_t));
    }

    // Convert a plaintext boolean to an encrypted boolean.
    function asEbool(bool value) internal pure returns (ebool) {
        if (value) {
            return asEbool(1);
        } else {
            return asEbool(0);
        }
    }

    // Converts an 'ebool' to an 'euint8'.
    function asEuint8(ebool value) internal pure returns (euint8) {
        return euint8.wrap(Impl.cast(ebool.unwrap(value), Common.euint8_t));
    }

    // Evaluate and(a, b) and return the result.
    function and(ebool a, ebool b) internal pure returns (ebool) {
        return ebool.wrap(Impl.and(ebool.unwrap(a), ebool.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(ebool a, ebool b) internal pure returns (ebool) {
        return ebool.wrap(Impl.or(ebool.unwrap(a), ebool.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(ebool a, ebool b) internal pure returns (ebool) {
        return ebool.wrap(Impl.xor(ebool.unwrap(a), ebool.unwrap(b)));
    }

    function not(ebool a) internal pure returns (ebool) {
        return ebool.wrap(Impl.not(ebool.unwrap(a)));
    }

    // Cast an encrypted integer from euint4 to euint16.
    function asEuint16(euint4 value) internal pure returns (euint16) {
        return euint16.wrap(Impl.cast(euint4.unwrap(value), Common.euint16_t));
    }

    // Cast an encrypted integer from euint8 to euint16.
    function asEuint16(euint8 value) internal pure returns (euint16) {
        return euint16.wrap(Impl.cast(euint8.unwrap(value), Common.euint16_t));
    }

    // Cast an encrypted integer from euint32 to euint16.
    function asEuint16(euint32 value) internal pure returns (euint16) {
        return euint16.wrap(Impl.cast(euint32.unwrap(value), Common.euint16_t));
    }

    // Cast an encrypted integer from euint64 to euint16.
    function asEuint16(euint64 value) internal pure returns (euint16) {
        return euint16.wrap(Impl.cast(euint64.unwrap(value), Common.euint16_t));
    }

    // Cast an encrypted integer from euint16 to ebool.
    function asEbool(euint16 value) internal pure returns (ebool) {
        return ne(value, 0);
    }

    // Converts an 'ebool' to an 'euint16'.
    function asEuint16(ebool b) internal pure returns (euint16) {
        return euint16.wrap(Impl.cast(ebool.unwrap(b), Common.euint16_t));
    }

    // Cast an encrypted integer from euint4 to euint32.
    function asEuint32(euint4 value) internal pure returns (euint32) {
        return euint32.wrap(Impl.cast(euint4.unwrap(value), Common.euint32_t));
    }

    // Cast an encrypted integer from euint8 to euint32.
    function asEuint32(euint8 value) internal pure returns (euint32) {
        return euint32.wrap(Impl.cast(euint8.unwrap(value), Common.euint32_t));
    }

    // Cast an encrypted integer from euint16 to euint32.
    function asEuint32(euint16 value) internal pure returns (euint32) {
        return euint32.wrap(Impl.cast(euint16.unwrap(value), Common.euint32_t));
    }

    // Cast an encrypted integer from euint64 to euint32.
    function asEuint32(euint64 value) internal pure returns (euint32) {
        return euint32.wrap(Impl.cast(euint64.unwrap(value), Common.euint32_t));
    }

    // Cast an encrypted integer from euint32 to ebool.
    function asEbool(euint32 value) internal pure returns (ebool) {
        return ne(value, 0);
    }

    // Converts an 'ebool' to an 'euint32'.
    function asEuint32(ebool b) internal pure returns (euint32) {
        return euint32.wrap(Impl.cast(ebool.unwrap(b), Common.euint32_t));
    }

    // Cast an encrypted integer from euint4 to euint64.
    function asEuint64(euint4 value) internal pure returns (euint64) {
        return euint64.wrap(Impl.cast(euint4.unwrap(value), Common.euint64_t));
    }

    // Cast an encrypted integer from euint8 to euint64.
    function asEuint64(euint8 value) internal pure returns (euint64) {
        return euint64.wrap(Impl.cast(euint8.unwrap(value), Common.euint64_t));
    }

    // Cast an encrypted integer from euint16 to euint64.
    function asEuint64(euint16 value) internal pure returns (euint64) {
        return euint64.wrap(Impl.cast(euint16.unwrap(value), Common.euint64_t));
    }

    // Cast an encrypted integer from euint32 to euint64.
    function asEuint64(euint32 value) internal pure returns (euint64) {
        return euint64.wrap(Impl.cast(euint32.unwrap(value), Common.euint64_t));
    }

    // Cast an encrypted integer from euint64 to ebool.
    function asEbool(euint64 value) internal pure returns (ebool) {
        return ne(value, 0);
    }

    // Converts an 'ebool' to an 'euint64'.
    function asEuint64(ebool b) internal pure returns (euint64) {
        return euint64.wrap(Impl.cast(ebool.unwrap(b), Common.euint64_t));
    }

    function neg(euint4 value) internal pure returns (euint4) {
        return euint4.wrap(Impl.neg(euint4.unwrap(value)));
    }

    function not(euint4 value) internal pure returns (euint4) {
        return euint4.wrap(Impl.not(euint4.unwrap(value)));
    }

    function neg(euint8 value) internal pure returns (euint8) {
        return euint8.wrap(Impl.neg(euint8.unwrap(value)));
    }

    function not(euint8 value) internal pure returns (euint8) {
        return euint8.wrap(Impl.not(euint8.unwrap(value)));
    }

    function neg(euint16 value) internal pure returns (euint16) {
        return euint16.wrap(Impl.neg(euint16.unwrap(value)));
    }

    function not(euint16 value) internal pure returns (euint16) {
        return euint16.wrap(Impl.not(euint16.unwrap(value)));
    }

    function neg(euint32 value) internal pure returns (euint32) {
        return euint32.wrap(Impl.neg(euint32.unwrap(value)));
    }

    function not(euint32 value) internal pure returns (euint32) {
        return euint32.wrap(Impl.not(euint32.unwrap(value)));
    }

    function neg(euint64 value) internal pure returns (euint64) {
        return euint64.wrap(Impl.neg(euint64.unwrap(value)));
    }

    function not(euint64 value) internal pure returns (euint64) {
        return euint64.wrap(Impl.not(euint64.unwrap(value)));
    }

    // Convert a serialized 'ciphertext' to an encrypted euint4 integer.
    function asEuint4(bytes memory ciphertext) internal pure returns (euint4) {
        return euint4.wrap(Impl.verify(ciphertext, Common.euint4_t));
    }

    // Convert a plaintext value to an encrypted euint4 integer.
    function asEuint4(uint256 value) internal pure returns (euint4) {
        return euint4.wrap(Impl.trivialEncrypt(value, Common.euint4_t));
    }

    // Decrypts the encrypted 'value'.
    function decrypt(euint4 value) internal view returns (uint8) {
        return uint8(Impl.decrypt(euint4.unwrap(value)));
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint4 ciphertext.
    function reencrypt(euint4 value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint4.unwrap(value), publicKey);
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // If 'value' is not initialized, the returned value will contain the 'defaultValue' constant.
    // Return a serialized euint4 ciphertext.
    function reencrypt(
        euint4 value,
        bytes32 publicKey,
        uint8 defaultValue
    ) internal view returns (bytes memory reencrypted) {
        if (euint4.unwrap(value) != 0) {
            return Impl.reencrypt(euint4.unwrap(value), publicKey);
        } else {
            return Impl.reencrypt(euint4.unwrap(asEuint4(defaultValue)), publicKey);
        }
    }

    // Convert a serialized 'ciphertext' to an encrypted euint8 integer.
    function asEuint8(bytes memory ciphertext) internal pure returns (euint8) {
        return euint8.wrap(Impl.verify(ciphertext, Common.euint8_t));
    }

    // Convert a plaintext value to an encrypted euint8 integer.
    function asEuint8(uint256 value) internal pure returns (euint8) {
        return euint8.wrap(Impl.trivialEncrypt(value, Common.euint8_t));
    }

    // Decrypts the encrypted 'value'.
    function decrypt(euint8 value) internal view returns (uint8) {
        return uint8(Impl.decrypt(euint8.unwrap(value)));
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint8 ciphertext.
    function reencrypt(euint8 value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint8.unwrap(value), publicKey);
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // If 'value' is not initialized, the returned value will contain the 'defaultValue' constant.
    // Return a serialized euint8 ciphertext.
    function reencrypt(
        euint8 value,
        bytes32 publicKey,
        uint8 defaultValue
    ) internal view returns (bytes memory reencrypted) {
        if (euint8.unwrap(value) != 0) {
            return Impl.reencrypt(euint8.unwrap(value), publicKey);
        } else {
            return Impl.reencrypt(euint8.unwrap(asEuint8(defaultValue)), publicKey);
        }
    }

    // Convert a serialized 'ciphertext' to an encrypted euint16 integer.
    function asEuint16(bytes memory ciphertext) internal pure returns (euint16) {
        return euint16.wrap(Impl.verify(ciphertext, Common.euint16_t));
    }

    // Convert a plaintext value to an encrypted euint16 integer.
    function asEuint16(uint256 value) internal pure returns (euint16) {
        return euint16.wrap(Impl.trivialEncrypt(value, Common.euint16_t));
    }

    // Decrypts the encrypted 'value'.
    function decrypt(euint16 value) internal view returns (uint16) {
        return uint16(Impl.decrypt(euint16.unwrap(value)));
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint16 ciphertext.
    function reencrypt(euint16 value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint16.unwrap(value), publicKey);
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // If 'value' is not initialized, the returned value will contain the 'defaultValue' constant.
    // Return a serialized euint16 ciphertext.
    function reencrypt(
        euint16 value,
        bytes32 publicKey,
        uint16 defaultValue
    ) internal view returns (bytes memory reencrypted) {
        if (euint16.unwrap(value) != 0) {
            return Impl.reencrypt(euint16.unwrap(value), publicKey);
        } else {
            return Impl.reencrypt(euint16.unwrap(asEuint16(defaultValue)), publicKey);
        }
    }

    // Convert a serialized 'ciphertext' to an encrypted euint32 integer.
    function asEuint32(bytes memory ciphertext) internal pure returns (euint32) {
        return euint32.wrap(Impl.verify(ciphertext, Common.euint32_t));
    }

    // Convert a plaintext value to an encrypted euint32 integer.
    function asEuint32(uint256 value) internal pure returns (euint32) {
        return euint32.wrap(Impl.trivialEncrypt(value, Common.euint32_t));
    }

    // Decrypts the encrypted 'value'.
    function decrypt(euint32 value) internal view returns (uint32) {
        return uint32(Impl.decrypt(euint32.unwrap(value)));
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint32 ciphertext.
    function reencrypt(euint32 value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint32.unwrap(value), publicKey);
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // If 'value' is not initialized, the returned value will contain the 'defaultValue' constant.
    // Return a serialized euint32 ciphertext.
    function reencrypt(
        euint32 value,
        bytes32 publicKey,
        uint32 defaultValue
    ) internal view returns (bytes memory reencrypted) {
        if (euint32.unwrap(value) != 0) {
            return Impl.reencrypt(euint32.unwrap(value), publicKey);
        } else {
            return Impl.reencrypt(euint32.unwrap(asEuint32(defaultValue)), publicKey);
        }
    }

    // Convert a serialized 'ciphertext' to an encrypted euint64 integer.
    function asEuint64(bytes memory ciphertext) internal pure returns (euint64) {
        return euint64.wrap(Impl.verify(ciphertext, Common.euint64_t));
    }

    // Convert a plaintext value to an encrypted euint64 integer.
    function asEuint64(uint256 value) internal pure returns (euint64) {
        return euint64.wrap(Impl.trivialEncrypt(value, Common.euint64_t));
    }

    // Decrypts the encrypted 'value'.
    function decrypt(euint64 value) internal view returns (uint64) {
        return uint64(Impl.decrypt(euint64.unwrap(value)));
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint64 ciphertext.
    function reencrypt(euint64 value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint64.unwrap(value), publicKey);
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // If 'value' is not initialized, the returned value will contain the 'defaultValue' constant.
    // Return a serialized euint64 ciphertext.
    function reencrypt(
        euint64 value,
        bytes32 publicKey,
        uint64 defaultValue
    ) internal view returns (bytes memory reencrypted) {
        if (euint64.unwrap(value) != 0) {
            return Impl.reencrypt(euint64.unwrap(value), publicKey);
        } else {
            return Impl.reencrypt(euint64.unwrap(asEuint64(defaultValue)), publicKey);
        }
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint8 value.
    function reencrypt(ebool value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(ebool.unwrap(value), publicKey);
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint8 value.
    // If 'value' is not initialized, the returned value will contain the 'defaultValue' constant.
    function reencrypt(
        ebool value,
        bytes32 publicKey,
        bool defaultValue
    ) internal view returns (bytes memory reencrypted) {
        if (ebool.unwrap(value) != 0) {
            return Impl.reencrypt(ebool.unwrap(value), publicKey);
        } else {
            return Impl.reencrypt(ebool.unwrap(asEbool(defaultValue)), publicKey);
        }
    }

    // Returns the network public FHE key.
    function fhePubKey() internal view returns (bytes memory) {
        return Impl.fhePubKey();
    }

    // Generates a random encrypted 8-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint8() internal view returns (euint8) {
        return euint8.wrap(Impl.rand(Common.euint8_t));
    }

    // Generates a random encrypted 8-bit unsigned integer in the [0, upperBound) range.
    // The upperBound must be a power of 2.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint8(uint8 upperBound) internal view returns (euint8) {
        return euint8.wrap(Impl.randBounded(upperBound, Common.euint8_t));
    }

    // Generates a random encrypted 16-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint16() internal view returns (euint16) {
        return euint16.wrap(Impl.rand(Common.euint16_t));
    }

    // Generates a random encrypted 16-bit unsigned integer in the [0, upperBound) range.
    // The upperBound must be a power of 2.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint16(uint16 upperBound) internal view returns (euint16) {
        return euint16.wrap(Impl.randBounded(upperBound, Common.euint16_t));
    }

    // Generates a random encrypted 32-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint32() internal view returns (euint32) {
        return euint32.wrap(Impl.rand(Common.euint32_t));
    }

    // Generates a random encrypted 64-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint64() internal view returns (euint64) {
        return euint64.wrap(Impl.rand(Common.euint64_t));
    }

    // Generates a random encrypted 32-bit unsigned integer in the [0, upperBound) range.
    // The upperBound must be a power of 2.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint32(uint32 upperBound) internal view returns (euint32) {
        return euint32.wrap(Impl.randBounded(upperBound, Common.euint32_t));
    }

    function randEuint64(uint64 upperBound) internal view returns (euint64) {
        return euint64.wrap(Impl.randBounded(upperBound, Common.euint64_t));
    }

    // Decrypts the encrypted 'value'.
    function decrypt(eaddress value) internal view returns (address) {
        return address(uint160(Impl.decrypt(eaddress.unwrap(value))));
    }

    // Reencrypt  the encrypted 'value'.
    function reencrypt(eaddress value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(eaddress.unwrap(value), publicKey);
    }

    // From bytes to eaddress
    function asEaddress(bytes memory ciphertext) internal pure returns (eaddress) {
        return eaddress.wrap(Impl.verify(ciphertext, Common.euint160_t));
    }

    // Convert a plaintext value to an encrypted asEaddress.
    function asEaddress(address value) internal pure returns (eaddress) {
        return eaddress.wrap(Impl.trivialEncrypt(uint160(value), Common.euint160_t));
    }

    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(eaddress v) internal pure returns (bool) {
        return eaddress.unwrap(v) != 0;
    }

    // Evaluate eq(a, b) and return the result.
    function eq(eaddress a, eaddress b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), eaddress.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(eaddress a, eaddress b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), eaddress.unwrap(b), false));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(eaddress a, address b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), bProc, true));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(address b, eaddress a) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), bProc, true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(eaddress a, address b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), bProc, true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(address b, eaddress a) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), bProc, true));
    }

    function select(ebool control, eaddress a, eaddress b) internal pure returns (eaddress) {
        return eaddress.wrap(Impl.select(ebool.unwrap(control), eaddress.unwrap(a), eaddress.unwrap(b)));
    }

    // Decrypts the encrypted 'value'.
    function decrypt(ebool value) internal view returns (bool) {
        return (Impl.decrypt(ebool.unwrap(value)) != 0);
    }
}

using {tfheBinaryOperatorAdd4 as +} for euint4 global;

function tfheBinaryOperatorAdd4(euint4 lhs, euint4 rhs) pure returns (euint4) {
    return TFHE.add(lhs, rhs);
}

using {tfheBinaryOperatorSub4 as -} for euint4 global;

function tfheBinaryOperatorSub4(euint4 lhs, euint4 rhs) pure returns (euint4) {
    return TFHE.sub(lhs, rhs);
}

using {tfheBinaryOperatorMul4 as *} for euint4 global;

function tfheBinaryOperatorMul4(euint4 lhs, euint4 rhs) pure returns (euint4) {
    return TFHE.mul(lhs, rhs);
}

using {tfheBinaryOperatorAnd4 as &} for euint4 global;

function tfheBinaryOperatorAnd4(euint4 lhs, euint4 rhs) pure returns (euint4) {
    return TFHE.and(lhs, rhs);
}

using {tfheBinaryOperatorOr4 as |} for euint4 global;

function tfheBinaryOperatorOr4(euint4 lhs, euint4 rhs) pure returns (euint4) {
    return TFHE.or(lhs, rhs);
}

using {tfheBinaryOperatorXor4 as ^} for euint4 global;

function tfheBinaryOperatorXor4(euint4 lhs, euint4 rhs) pure returns (euint4) {
    return TFHE.xor(lhs, rhs);
}

using {tfheUnaryOperatorNeg4 as -} for euint4 global;

function tfheUnaryOperatorNeg4(euint4 input) pure returns (euint4) {
    return TFHE.neg(input);
}

using {tfheUnaryOperatorNot4 as ~} for euint4 global;

function tfheUnaryOperatorNot4(euint4 input) pure returns (euint4) {
    return TFHE.not(input);
}

using {tfheBinaryOperatorAdd8 as +} for euint8 global;

function tfheBinaryOperatorAdd8(euint8 lhs, euint8 rhs) pure returns (euint8) {
    return TFHE.add(lhs, rhs);
}

using {tfheBinaryOperatorSub8 as -} for euint8 global;

function tfheBinaryOperatorSub8(euint8 lhs, euint8 rhs) pure returns (euint8) {
    return TFHE.sub(lhs, rhs);
}

using {tfheBinaryOperatorMul8 as *} for euint8 global;

function tfheBinaryOperatorMul8(euint8 lhs, euint8 rhs) pure returns (euint8) {
    return TFHE.mul(lhs, rhs);
}

using {tfheBinaryOperatorAnd8 as &} for euint8 global;

function tfheBinaryOperatorAnd8(euint8 lhs, euint8 rhs) pure returns (euint8) {
    return TFHE.and(lhs, rhs);
}

using {tfheBinaryOperatorOr8 as |} for euint8 global;

function tfheBinaryOperatorOr8(euint8 lhs, euint8 rhs) pure returns (euint8) {
    return TFHE.or(lhs, rhs);
}

using {tfheBinaryOperatorXor8 as ^} for euint8 global;

function tfheBinaryOperatorXor8(euint8 lhs, euint8 rhs) pure returns (euint8) {
    return TFHE.xor(lhs, rhs);
}

using {tfheUnaryOperatorNeg8 as -} for euint8 global;

function tfheUnaryOperatorNeg8(euint8 input) pure returns (euint8) {
    return TFHE.neg(input);
}

using {tfheUnaryOperatorNot8 as ~} for euint8 global;

function tfheUnaryOperatorNot8(euint8 input) pure returns (euint8) {
    return TFHE.not(input);
}

using {tfheBinaryOperatorAdd16 as +} for euint16 global;

function tfheBinaryOperatorAdd16(euint16 lhs, euint16 rhs) pure returns (euint16) {
    return TFHE.add(lhs, rhs);
}

using {tfheBinaryOperatorSub16 as -} for euint16 global;

function tfheBinaryOperatorSub16(euint16 lhs, euint16 rhs) pure returns (euint16) {
    return TFHE.sub(lhs, rhs);
}

using {tfheBinaryOperatorMul16 as *} for euint16 global;

function tfheBinaryOperatorMul16(euint16 lhs, euint16 rhs) pure returns (euint16) {
    return TFHE.mul(lhs, rhs);
}

using {tfheBinaryOperatorAnd16 as &} for euint16 global;

function tfheBinaryOperatorAnd16(euint16 lhs, euint16 rhs) pure returns (euint16) {
    return TFHE.and(lhs, rhs);
}

using {tfheBinaryOperatorOr16 as |} for euint16 global;

function tfheBinaryOperatorOr16(euint16 lhs, euint16 rhs) pure returns (euint16) {
    return TFHE.or(lhs, rhs);
}

using {tfheBinaryOperatorXor16 as ^} for euint16 global;

function tfheBinaryOperatorXor16(euint16 lhs, euint16 rhs) pure returns (euint16) {
    return TFHE.xor(lhs, rhs);
}

using {tfheUnaryOperatorNeg16 as -} for euint16 global;

function tfheUnaryOperatorNeg16(euint16 input) pure returns (euint16) {
    return TFHE.neg(input);
}

using {tfheUnaryOperatorNot16 as ~} for euint16 global;

function tfheUnaryOperatorNot16(euint16 input) pure returns (euint16) {
    return TFHE.not(input);
}

using {tfheBinaryOperatorAdd32 as +} for euint32 global;

function tfheBinaryOperatorAdd32(euint32 lhs, euint32 rhs) pure returns (euint32) {
    return TFHE.add(lhs, rhs);
}

using {tfheBinaryOperatorSub32 as -} for euint32 global;

function tfheBinaryOperatorSub32(euint32 lhs, euint32 rhs) pure returns (euint32) {
    return TFHE.sub(lhs, rhs);
}

using {tfheBinaryOperatorMul32 as *} for euint32 global;

function tfheBinaryOperatorMul32(euint32 lhs, euint32 rhs) pure returns (euint32) {
    return TFHE.mul(lhs, rhs);
}

using {tfheBinaryOperatorAnd32 as &} for euint32 global;

function tfheBinaryOperatorAnd32(euint32 lhs, euint32 rhs) pure returns (euint32) {
    return TFHE.and(lhs, rhs);
}

using {tfheBinaryOperatorOr32 as |} for euint32 global;

function tfheBinaryOperatorOr32(euint32 lhs, euint32 rhs) pure returns (euint32) {
    return TFHE.or(lhs, rhs);
}

using {tfheBinaryOperatorXor32 as ^} for euint32 global;

function tfheBinaryOperatorXor32(euint32 lhs, euint32 rhs) pure returns (euint32) {
    return TFHE.xor(lhs, rhs);
}

using {tfheUnaryOperatorNeg32 as -} for euint32 global;

function tfheUnaryOperatorNeg32(euint32 input) pure returns (euint32) {
    return TFHE.neg(input);
}

using {tfheUnaryOperatorNot32 as ~} for euint32 global;

function tfheUnaryOperatorNot32(euint32 input) pure returns (euint32) {
    return TFHE.not(input);
}

using {tfheBinaryOperatorAdd64 as +} for euint64 global;

function tfheBinaryOperatorAdd64(euint64 lhs, euint64 rhs) pure returns (euint64) {
    return TFHE.add(lhs, rhs);
}

using {tfheBinaryOperatorSub64 as -} for euint64 global;

function tfheBinaryOperatorSub64(euint64 lhs, euint64 rhs) pure returns (euint64) {
    return TFHE.sub(lhs, rhs);
}

using {tfheBinaryOperatorMul64 as *} for euint64 global;

function tfheBinaryOperatorMul64(euint64 lhs, euint64 rhs) pure returns (euint64) {
    return TFHE.mul(lhs, rhs);
}

using {tfheBinaryOperatorAnd64 as &} for euint64 global;

function tfheBinaryOperatorAnd64(euint64 lhs, euint64 rhs) pure returns (euint64) {
    return TFHE.and(lhs, rhs);
}

using {tfheBinaryOperatorOr64 as |} for euint64 global;

function tfheBinaryOperatorOr64(euint64 lhs, euint64 rhs) pure returns (euint64) {
    return TFHE.or(lhs, rhs);
}

using {tfheBinaryOperatorXor64 as ^} for euint64 global;

function tfheBinaryOperatorXor64(euint64 lhs, euint64 rhs) pure returns (euint64) {
    return TFHE.xor(lhs, rhs);
}

using {tfheUnaryOperatorNeg64 as -} for euint64 global;

function tfheUnaryOperatorNeg64(euint64 input) pure returns (euint64) {
    return TFHE.neg(input);
}

using {tfheUnaryOperatorNot64 as ~} for euint64 global;

function tfheUnaryOperatorNot64(euint64 input) pure returns (euint64) {
    return TFHE.not(input);
}
