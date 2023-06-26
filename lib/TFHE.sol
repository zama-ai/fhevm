// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "./Common.sol";
import "./Impl.sol";

library TFHE {
    function add(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.add(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function sub(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.sub(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function mul(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.mul(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function and(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.and(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function or(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.or(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function xor(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.xor(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function eq(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.eq(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function ge(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.ge(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function gt(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.gt(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function le(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.le(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function lt(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lt(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function add(euint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.add(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function add(euint8 a, uint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.add(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function add(uint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.add(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function sub(euint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.sub(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function sub(euint8 a, uint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.sub(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function sub(uint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.sub(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function mul(euint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.mul(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function mul(euint8 a, uint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.mul(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function mul(uint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.mul(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function and(euint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.and(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function and(euint8 a, uint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.and(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function and(uint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.and(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function or(euint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.or(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function or(euint8 a, uint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.or(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function or(uint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.or(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function xor(euint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.xor(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function xor(euint8 a, uint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.xor(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function xor(uint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.xor(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function eq(euint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.eq(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function eq(euint8 a, uint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.eq(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function eq(uint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.eq(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function ge(euint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.ge(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function ge(euint8 a, uint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.ge(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function ge(uint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.ge(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function gt(euint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.gt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function gt(euint8 a, uint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.gt(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function gt(uint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.gt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function le(euint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.le(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function le(euint8 a, uint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.le(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function le(uint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.le(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function lt(euint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.lt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function lt(euint8 a, uint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.lt(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function lt(uint8 a, euint16 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.lt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b))
            );
    }

    function add(euint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function add(euint8 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function add(uint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function sub(euint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function sub(euint8 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function sub(uint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function mul(euint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function mul(euint8 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function mul(uint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function and(euint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function and(euint8 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function and(uint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function or(euint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function or(euint8 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function or(uint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function xor(euint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function xor(euint8 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function xor(uint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function eq(euint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function eq(euint8 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function eq(uint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function ge(euint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function ge(euint8 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function ge(uint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function gt(euint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function gt(euint8 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function gt(uint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function le(euint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function le(euint8 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function le(uint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function lt(euint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function lt(euint8 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function lt(uint8 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function add(euint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.add(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function add(euint16 a, uint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.add(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function add(uint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.add(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function sub(euint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.sub(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function sub(euint16 a, uint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.sub(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function sub(uint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.sub(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function mul(euint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.mul(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function mul(euint16 a, uint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.mul(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function mul(uint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.mul(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function and(euint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.and(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function and(euint16 a, uint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.and(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function and(uint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.and(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function or(euint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.or(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function or(euint16 a, uint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.or(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function or(uint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.or(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function xor(euint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.xor(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function xor(euint16 a, uint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.xor(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function xor(uint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.xor(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function eq(euint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.eq(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function eq(euint16 a, uint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.eq(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function eq(uint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.eq(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function ge(euint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.ge(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function ge(euint16 a, uint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.ge(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function ge(uint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.ge(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function gt(euint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.gt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function gt(euint16 a, uint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.gt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function gt(uint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.gt(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function le(euint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.le(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function le(euint16 a, uint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.le(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function le(uint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.le(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function lt(euint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.lt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function lt(euint16 a, uint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.lt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)))
            );
    }

    function lt(uint16 a, euint8 b) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.lt(
                    euint16.unwrap(asEuint16(a)),
                    euint16.unwrap(asEuint16(b))
                )
            );
    }

    function add(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.add(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function sub(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.sub(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function mul(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.mul(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function and(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.and(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function or(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.or(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function xor(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.xor(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function eq(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.eq(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function ge(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.ge(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function gt(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.gt(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function le(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.le(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function lt(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.lt(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function add(euint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function add(euint16 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function add(uint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function sub(euint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function sub(euint16 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function sub(uint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function mul(euint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function mul(euint16 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function mul(uint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function and(euint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function and(euint16 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function and(uint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function or(euint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function or(euint16 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function or(uint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function xor(euint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function xor(euint16 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function xor(uint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function eq(euint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function eq(euint16 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function eq(uint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function ge(euint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function ge(euint16 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function ge(uint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function gt(euint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function gt(euint16 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function gt(uint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function le(euint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function le(euint16 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function le(uint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function lt(euint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function lt(euint16 a, uint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function lt(uint16 a, euint32 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b))
            );
    }

    function add(euint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function add(euint32 a, uint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function add(uint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function sub(euint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function sub(euint32 a, uint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function sub(uint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function mul(euint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function mul(euint32 a, uint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function mul(uint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function and(euint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function and(euint32 a, uint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function and(uint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function or(euint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function or(euint32 a, uint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function or(uint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function xor(euint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function xor(euint32 a, uint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function xor(uint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function eq(euint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function eq(euint32 a, uint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function eq(uint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function ge(euint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function ge(euint32 a, uint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function ge(uint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function gt(euint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function gt(euint32 a, uint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function gt(uint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function le(euint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function le(euint32 a, uint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function le(uint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function lt(euint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function lt(euint32 a, uint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function lt(uint32 a, euint8 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function add(euint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function add(euint32 a, uint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function add(uint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.add(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function sub(euint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function sub(euint32 a, uint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function sub(uint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.sub(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function mul(euint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function mul(euint32 a, uint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function mul(uint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.mul(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function and(euint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function and(euint32 a, uint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function and(uint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.and(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function or(euint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function or(euint32 a, uint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function or(uint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.or(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function xor(euint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function xor(euint32 a, uint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function xor(uint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.xor(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function eq(euint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function eq(euint32 a, uint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function eq(uint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.eq(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function ge(euint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function ge(euint32 a, uint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function ge(uint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.ge(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function gt(euint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function gt(euint32 a, uint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function gt(uint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.gt(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function le(euint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function le(euint32 a, uint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function le(uint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.le(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function lt(euint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function lt(euint32 a, uint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)))
            );
    }

    function lt(uint32 a, euint16 b) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.lt(
                    euint32.unwrap(asEuint32(a)),
                    euint32.unwrap(asEuint32(b))
                )
            );
    }

    function add(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function sub(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function mul(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.mul(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function and(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.and(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function or(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.or(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function xor(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.xor(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function eq(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.eq(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function ge(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.ge(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function gt(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.gt(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function le(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.le(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function lt(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.lt(euint32.unwrap(a), euint32.unwrap(b)));
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

    function requireCt(euint8 ciphertext) internal view {
        Impl.requireCt(euint8.unwrap(ciphertext));
    }

    function optimisticRequireCt(euint8 ciphertext) internal view {
        Impl.optimisticRequireCt(euint32.unwrap(asEuint32(ciphertext)));
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

    function requireCt(euint16 ciphertext) internal view {
        Impl.requireCt(euint16.unwrap(ciphertext));
    }

    function optimisticRequireCt(euint16 ciphertext) internal view {
        Impl.optimisticRequireCt(euint32.unwrap(asEuint32(ciphertext)));
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

    function requireCt(euint32 ciphertext) internal view {
        Impl.requireCt(euint32.unwrap(ciphertext));
    }

    function optimisticRequireCt(euint32 ciphertext) internal view {
        Impl.optimisticRequireCt(euint32.unwrap(ciphertext));
    }

    // Returns the network public FHE key.
    function fhePubKey() internal view returns (bytes memory) {
        return Impl.fhePubKey();
    }
}
