// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "./Common.sol";
import "./Impl.sol";
library FHEOps {
    function add(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.add(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function sub(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.sub(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function mul(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.mul(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function lte(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function add(euint8 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.add(euint8.unwrap(a), euint16.unwrap(b)));
    }

    function sub(euint8 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.sub(euint8.unwrap(a), euint16.unwrap(b)));
    }

    function mul(euint8 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.mul(euint8.unwrap(a), euint16.unwrap(b)));
    }

    function lte(euint8 a, euint16 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint8.unwrap(a), euint16.unwrap(b)));
    }

    function add(euint8 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.add(euint8.unwrap(a), euint32.unwrap(b)));
    }

    function sub(euint8 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.sub(euint8.unwrap(a), euint32.unwrap(b)));
    }

    function mul(euint8 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.mul(euint8.unwrap(a), euint32.unwrap(b)));
    }

    function lte(euint8 a, euint32 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint8.unwrap(a), euint32.unwrap(b)));
    }

    function add(euint16 a, euint8 b) internal view returns (euint16) {
        return euint16.wrap(Impl.add(euint16.unwrap(a), euint8.unwrap(b)));
    }

    function sub(euint16 a, euint8 b) internal view returns (euint16) {
        return euint16.wrap(Impl.sub(euint16.unwrap(a), euint8.unwrap(b)));
    }

    function mul(euint16 a, euint8 b) internal view returns (euint16) {
        return euint16.wrap(Impl.mul(euint16.unwrap(a), euint8.unwrap(b)));
    }

    function lte(euint16 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint16.unwrap(a), euint8.unwrap(b)));
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

    function lte(euint16 a, euint16 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function add(euint16 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.add(euint16.unwrap(a), euint32.unwrap(b)));
    }

    function sub(euint16 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.sub(euint16.unwrap(a), euint32.unwrap(b)));
    }

    function mul(euint16 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.mul(euint16.unwrap(a), euint32.unwrap(b)));
    }

    function lte(euint16 a, euint32 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint16.unwrap(a), euint32.unwrap(b)));
    }

    function add(euint32 a, euint8 b) internal view returns (euint32) {
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint8.unwrap(b)));
    }

    function sub(euint32 a, euint8 b) internal view returns (euint32) {
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint8.unwrap(b)));
    }

    function mul(euint32 a, euint8 b) internal view returns (euint32) {
        return euint32.wrap(Impl.mul(euint32.unwrap(a), euint8.unwrap(b)));
    }

    function lte(euint32 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint32.unwrap(a), euint8.unwrap(b)));
    }

    function add(euint32 a, euint16 b) internal view returns (euint32) {
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint16.unwrap(b)));
    }

    function sub(euint32 a, euint16 b) internal view returns (euint32) {
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint16.unwrap(b)));
    }

    function mul(euint32 a, euint16 b) internal view returns (euint32) {
        return euint32.wrap(Impl.mul(euint32.unwrap(a), euint16.unwrap(b)));
    }

    function lte(euint32 a, euint16 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint32.unwrap(a), euint16.unwrap(b)));
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

    function lte(euint32 a, euint32 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function to_euint8(euint16 v) internal view returns (euint8) {
        return euint8.wrap(Impl.cast(euint16.unwrap(v), Common.euint16_t));
    }

    function to_euint8(euint32 v) internal view returns (euint8) {
        return euint8.wrap(Impl.cast(euint32.unwrap(v), Common.euint32_t));
    }

    function to_euint16(euint8 v) internal view returns (euint16) {
        return euint16.wrap(Impl.cast(euint8.unwrap(v), Common.euint8_t));
    }

    function to_euint16(euint32 v) internal view returns (euint16) {
        return euint16.wrap(Impl.cast(euint32.unwrap(v), Common.euint32_t));
    }

    function to_euint32(euint8 v) internal view returns (euint32) {
        return euint32.wrap(Impl.cast(euint8.unwrap(v), Common.euint8_t));
    }

    function to_euint32(euint16 v) internal view returns (euint32) {
        return euint32.wrap(Impl.cast(euint16.unwrap(v), Common.euint16_t));
    }
}