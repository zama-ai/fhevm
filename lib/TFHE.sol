// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "./Common.sol";
import "./Impl.sol";

library TFHE {
    function isInitialized(euint8 v) internal pure returns (bool) {
        return euint8.unwrap(v) != 0;
    }

    function isInitialized(euint16 v) internal pure returns (bool) {
        return euint16.unwrap(v) != 0;
    }

    function isInitialized(euint32 v) internal pure returns (bool) {
        return euint32.unwrap(v) != 0;
    }

    function add(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.add(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function sub(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.sub(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function mul(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.mul(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function and(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.and(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function or(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.or(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function xor(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.xor(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function shl(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.shl(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function shr(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.shr(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function eq(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.eq(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function ne(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.ne(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function ge(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.ge(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function gt(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.gt(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function le(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.le(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function lt(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.lt(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function min(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.min(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function max(euint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.max(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    function add(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.add(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function sub(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.sub(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function mul(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.mul(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function and(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.and(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function or(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.or(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function xor(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.xor(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function shl(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.shl(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function shr(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.shr(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function eq(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.eq(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function ne(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.ne(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function ge(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.ge(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function gt(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.gt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function le(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.le(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function lt(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.lt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function min(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.min(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function max(euint8 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(
                Impl.max(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false)
            );
    }

    function add(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function sub(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function mul(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function and(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function or(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function xor(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function shl(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.shl(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function shr(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.shr(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function eq(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function ne(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.ne(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function ge(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function gt(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function le(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function lt(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function min(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.min(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function max(euint8 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.max(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function add(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.add(euint8.unwrap(a), uint256(b), true));
    }

    function add(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.add(euint8.unwrap(b), uint256(a), true));
    }

    function sub(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.sub(euint8.unwrap(a), uint256(b), true));
    }

    function sub(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.sub(euint8.unwrap(b), uint256(a), true));
    }

    function mul(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.mul(euint8.unwrap(a), uint256(b), true));
    }

    function mul(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.mul(euint8.unwrap(b), uint256(a), true));
    }

    function shl(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.shl(euint8.unwrap(a), uint256(b), true));
    }

    function shl(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.shl(euint8.unwrap(b), uint256(a), true));
    }

    function shr(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.shr(euint8.unwrap(a), uint256(b), true));
    }

    function shr(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.shr(euint8.unwrap(b), uint256(a), true));
    }

    function eq(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.eq(euint8.unwrap(a), uint256(b), true));
    }

    function eq(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.eq(euint8.unwrap(b), uint256(a), true));
    }

    function ne(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.ne(euint8.unwrap(a), uint256(b), true));
    }

    function ne(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.ne(euint8.unwrap(b), uint256(a), true));
    }

    function ge(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.ge(euint8.unwrap(a), uint256(b), true));
    }

    function ge(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.le(euint8.unwrap(b), uint256(a), true));
    }

    function gt(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.gt(euint8.unwrap(a), uint256(b), true));
    }

    function gt(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.lt(euint8.unwrap(b), uint256(a), true));
    }

    function le(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.le(euint8.unwrap(a), uint256(b), true));
    }

    function le(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.ge(euint8.unwrap(b), uint256(a), true));
    }

    function lt(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.lt(euint8.unwrap(a), uint256(b), true));
    }

    function lt(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.gt(euint8.unwrap(b), uint256(a), true));
    }

    function min(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.min(euint8.unwrap(a), uint256(b), true));
    }

    function min(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.min(euint8.unwrap(b), uint256(a), true));
    }

    function max(euint8 a, uint8 b) internal view returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.max(euint8.unwrap(a), uint256(b), true));
    }

    function max(uint8 a, euint8 b) internal view returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.max(euint8.unwrap(b), uint256(a), true));
    }

    function add(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.add(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function sub(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.sub(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function mul(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.mul(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function and(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.and(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function or(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.or(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function xor(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.xor(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function shl(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.shl(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function shr(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.shr(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function eq(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.eq(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function ne(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.ne(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function ge(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.ge(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function gt(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.gt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function le(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.le(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function lt(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.lt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function min(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.min(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function max(euint16 a, euint8 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint16.wrap(
                Impl.max(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false)
            );
    }

    function add(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.add(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function sub(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.sub(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function mul(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.mul(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function and(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.and(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function or(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.or(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function xor(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.xor(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function shl(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.shl(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function shr(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.shr(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function eq(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.eq(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function ne(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.ne(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function ge(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.ge(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function gt(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.gt(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function le(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.le(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function lt(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.lt(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function min(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.min(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function max(euint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint16.wrap(Impl.max(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    function add(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function sub(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function mul(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function and(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function or(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function xor(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function shl(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.shl(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function shr(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.shr(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function eq(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function ne(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.ne(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function ge(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function gt(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function le(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function lt(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function min(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.min(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function max(euint16 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(
                Impl.max(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false)
            );
    }

    function add(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(a), uint256(b), true));
    }

    function add(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(b), uint256(a), true));
    }

    function sub(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(a), uint256(b), true));
    }

    function sub(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(b), uint256(a), true));
    }

    function mul(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(a), uint256(b), true));
    }

    function mul(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(b), uint256(a), true));
    }

    function shl(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.shl(euint16.unwrap(a), uint256(b), true));
    }

    function shl(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.shl(euint16.unwrap(b), uint256(a), true));
    }

    function shr(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.shr(euint16.unwrap(a), uint256(b), true));
    }

    function shr(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.shr(euint16.unwrap(b), uint256(a), true));
    }

    function eq(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.eq(euint16.unwrap(a), uint256(b), true));
    }

    function eq(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.eq(euint16.unwrap(b), uint256(a), true));
    }

    function ne(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.ne(euint16.unwrap(a), uint256(b), true));
    }

    function ne(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.ne(euint16.unwrap(b), uint256(a), true));
    }

    function ge(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.ge(euint16.unwrap(a), uint256(b), true));
    }

    function ge(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.le(euint16.unwrap(b), uint256(a), true));
    }

    function gt(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.gt(euint16.unwrap(a), uint256(b), true));
    }

    function gt(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.lt(euint16.unwrap(b), uint256(a), true));
    }

    function le(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.le(euint16.unwrap(a), uint256(b), true));
    }

    function le(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.ge(euint16.unwrap(b), uint256(a), true));
    }

    function lt(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.lt(euint16.unwrap(a), uint256(b), true));
    }

    function lt(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.gt(euint16.unwrap(b), uint256(a), true));
    }

    function min(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(a), uint256(b), true));
    }

    function min(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(b), uint256(a), true));
    }

    function max(euint16 a, uint16 b) internal view returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(a), uint256(b), true));
    }

    function max(uint16 a, euint16 b) internal view returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(b), uint256(a), true));
    }

    function add(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function sub(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function mul(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function and(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function or(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function xor(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function shl(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.shl(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function shr(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.shr(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function eq(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function ne(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.ne(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function ge(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function gt(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function le(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function lt(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function min(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.min(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function max(euint32 a, euint8 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return
            euint32.wrap(
                Impl.max(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function add(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function sub(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function mul(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function and(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function or(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function xor(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function shl(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.shl(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function shr(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.shr(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function eq(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function ne(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.ne(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function ge(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function gt(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function le(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function lt(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function min(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.min(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function max(euint32 a, euint16 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return
            euint32.wrap(
                Impl.max(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false)
            );
    }

    function add(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.add(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function sub(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.sub(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function mul(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.mul(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function and(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function or(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function xor(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function shl(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.shl(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function shr(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.shr(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function eq(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.eq(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function ne(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.ne(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function ge(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.ge(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function gt(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.gt(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function le(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.le(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function lt(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.lt(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function min(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.min(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function max(euint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return
            euint32.wrap(Impl.max(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    function add(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(a), uint256(b), true));
    }

    function add(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(b), uint256(a), true));
    }

    function sub(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(a), uint256(b), true));
    }

    function sub(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(b), uint256(a), true));
    }

    function mul(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(a), uint256(b), true));
    }

    function mul(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(b), uint256(a), true));
    }

    function shl(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.shl(euint32.unwrap(a), uint256(b), true));
    }

    function shl(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.shl(euint32.unwrap(b), uint256(a), true));
    }

    function shr(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.shr(euint32.unwrap(a), uint256(b), true));
    }

    function shr(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.shr(euint32.unwrap(b), uint256(a), true));
    }

    function eq(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.eq(euint32.unwrap(a), uint256(b), true));
    }

    function eq(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.eq(euint32.unwrap(b), uint256(a), true));
    }

    function ne(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.ne(euint32.unwrap(a), uint256(b), true));
    }

    function ne(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.ne(euint32.unwrap(b), uint256(a), true));
    }

    function ge(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.ge(euint32.unwrap(a), uint256(b), true));
    }

    function ge(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.le(euint32.unwrap(b), uint256(a), true));
    }

    function gt(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.gt(euint32.unwrap(a), uint256(b), true));
    }

    function gt(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.lt(euint32.unwrap(b), uint256(a), true));
    }

    function le(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.le(euint32.unwrap(a), uint256(b), true));
    }

    function le(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.ge(euint32.unwrap(b), uint256(a), true));
    }

    function lt(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.lt(euint32.unwrap(a), uint256(b), true));
    }

    function lt(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.gt(euint32.unwrap(b), uint256(a), true));
    }

    function min(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(a), uint256(b), true));
    }

    function min(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(b), uint256(a), true));
    }

    function max(euint32 a, uint32 b) internal view returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(a), uint256(b), true));
    }

    function max(uint32 a, euint32 b) internal view returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(b), uint256(a), true));
    }

    function cmux(
        euint8 control,
        euint8 a,
        euint8 b
    ) internal view returns (euint8) {
        return
            euint8.wrap(
                Impl.cmux(
                    euint8.unwrap(control),
                    euint8.unwrap(a),
                    euint8.unwrap(b)
                )
            );
    }

    function cmux(
        euint16 control,
        euint16 a,
        euint16 b
    ) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.cmux(
                    euint16.unwrap(control),
                    euint16.unwrap(a),
                    euint16.unwrap(b)
                )
            );
    }

    function cmux(
        euint32 control,
        euint32 a,
        euint32 b
    ) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.cmux(
                    euint32.unwrap(control),
                    euint32.unwrap(a),
                    euint32.unwrap(b)
                )
            );
    }

    function asEuint8(euint16 ciphertext) internal view returns (euint8) {
        return
            euint8.wrap(Impl.cast(euint16.unwrap(ciphertext), Common.euint8_t));
    }

    function asEuint8(euint32 ciphertext) internal view returns (euint8) {
        return
            euint8.wrap(Impl.cast(euint32.unwrap(ciphertext), Common.euint8_t));
    }

    function asEuint16(euint8 ciphertext) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.cast(euint8.unwrap(ciphertext), Common.euint16_t)
            );
    }

    function asEuint16(euint32 ciphertext) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.cast(euint32.unwrap(ciphertext), Common.euint16_t)
            );
    }

    function asEuint32(euint8 ciphertext) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.cast(euint8.unwrap(ciphertext), Common.euint32_t)
            );
    }

    function asEuint32(euint16 ciphertext) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.cast(euint16.unwrap(ciphertext), Common.euint32_t)
            );
    }

    function asEuint8(bytes memory ciphertext) internal view returns (euint8) {
        return euint8.wrap(Impl.verify(ciphertext, Common.euint8_t));
    }

    function asEuint8(uint256 value) internal view returns (euint8) {
        return euint8.wrap(Impl.trivialEncrypt(value, Common.euint8_t));
    }

    function reencrypt(
        euint8 ciphertext,
        bytes32 publicKey
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint8.unwrap(ciphertext), publicKey);
    }

    function reencrypt(
        euint8 ciphertext,
        bytes32 publicKey,
        uint8 defaultValue
    ) internal view returns (bytes memory reencrypted) {
        if (euint8.unwrap(ciphertext) != 0) {
            return Impl.reencrypt(euint8.unwrap(ciphertext), publicKey);
        } else {
            return
                Impl.reencrypt(
                    euint8.unwrap(asEuint8(defaultValue)),
                    publicKey
                );
        }
    }

    function req(euint8 ciphertext) internal view {
        Impl.req(euint8.unwrap(ciphertext));
    }

    function optReq(euint8 ciphertext) internal view {
        Impl.optReq(euint32.unwrap(asEuint32(ciphertext)));
    }

    function asEuint16(
        bytes memory ciphertext
    ) internal view returns (euint16) {
        return euint16.wrap(Impl.verify(ciphertext, Common.euint16_t));
    }

    function asEuint16(uint256 value) internal view returns (euint16) {
        return euint16.wrap(Impl.trivialEncrypt(value, Common.euint16_t));
    }

    function reencrypt(
        euint16 ciphertext,
        bytes32 publicKey
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint16.unwrap(ciphertext), publicKey);
    }

    function reencrypt(
        euint16 ciphertext,
        bytes32 publicKey,
        uint16 defaultValue
    ) internal view returns (bytes memory reencrypted) {
        if (euint16.unwrap(ciphertext) != 0) {
            return Impl.reencrypt(euint16.unwrap(ciphertext), publicKey);
        } else {
            return
                Impl.reencrypt(
                    euint16.unwrap(asEuint16(defaultValue)),
                    publicKey
                );
        }
    }

    function req(euint16 ciphertext) internal view {
        Impl.req(euint16.unwrap(ciphertext));
    }

    function optReq(euint16 ciphertext) internal view {
        Impl.optReq(euint32.unwrap(asEuint32(ciphertext)));
    }

    function asEuint32(
        bytes memory ciphertext
    ) internal view returns (euint32) {
        return euint32.wrap(Impl.verify(ciphertext, Common.euint32_t));
    }

    function asEuint32(uint256 value) internal view returns (euint32) {
        return euint32.wrap(Impl.trivialEncrypt(value, Common.euint32_t));
    }

    function reencrypt(
        euint32 ciphertext,
        bytes32 publicKey
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint32.unwrap(ciphertext), publicKey);
    }

    function reencrypt(
        euint32 ciphertext,
        bytes32 publicKey,
        uint32 defaultValue
    ) internal view returns (bytes memory reencrypted) {
        if (euint32.unwrap(ciphertext) != 0) {
            return Impl.reencrypt(euint32.unwrap(ciphertext), publicKey);
        } else {
            return
                Impl.reencrypt(
                    euint32.unwrap(asEuint32(defaultValue)),
                    publicKey
                );
        }
    }

    function req(euint32 ciphertext) internal view {
        Impl.req(euint32.unwrap(ciphertext));
    }

    function optReq(euint32 ciphertext) internal view {
        Impl.optReq(euint32.unwrap(ciphertext));
    }

    // Returns the network public FHE key.
    function fhePubKey() internal view returns (bytes memory) {
        return Impl.fhePubKey();
    }
}
