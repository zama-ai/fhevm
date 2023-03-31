// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

type euint8 is uint256;
type euint16 is uint256;
type euint32 is uint256;
type euint64 is uint256;
type euint128 is uint256;
type euint256 is uint256;

library Common {
    // Values used to communicate types at runtime to the cast() precompile.
    uint8 internal constant euint8_t = 0;
    uint8 internal constant euint16_t = 1;
    uint8 internal constant euint32_t = 2;
    uint8 internal constant euint64_t = 3;
    uint8 internal constant euint128_t = 4;
    uint8 internal constant euint256_t = 5;
}

library FHEOps {
    function add(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.add(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function sub(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.sub(euint8.unwrap(a), euint8.unwrap(b)));
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

    function lte(euint8 a, euint16 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint8.unwrap(a), euint16.unwrap(b)));
    }

    function add(euint8 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.add(euint8.unwrap(a), euint32.unwrap(b)));
    }

    function sub(euint8 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.sub(euint8.unwrap(a), euint32.unwrap(b)));
    }

    function lte(euint8 a, euint32 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint8.unwrap(a), euint32.unwrap(b)));
    }

    function add(euint8 a, euint64 b) internal view returns (euint64) {
        return euint64.wrap(Impl.add(euint8.unwrap(a), euint64.unwrap(b)));
    }

    function sub(euint8 a, euint64 b) internal view returns (euint64) {
        return euint64.wrap(Impl.sub(euint8.unwrap(a), euint64.unwrap(b)));
    }

    function lte(euint8 a, euint64 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint8.unwrap(a), euint64.unwrap(b)));
    }

    function add(euint8 a, euint128 b) internal view returns (euint128) {
        return euint128.wrap(Impl.add(euint8.unwrap(a), euint128.unwrap(b)));
    }

    function sub(euint8 a, euint128 b) internal view returns (euint128) {
        return euint128.wrap(Impl.sub(euint8.unwrap(a), euint128.unwrap(b)));
    }

    function lte(euint8 a, euint128 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint8.unwrap(a), euint128.unwrap(b)));
    }

    function add(euint8 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.add(euint8.unwrap(a), euint256.unwrap(b)));
    }

    function sub(euint8 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.sub(euint8.unwrap(a), euint256.unwrap(b)));
    }

    function lte(euint8 a, euint256 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint8.unwrap(a), euint256.unwrap(b)));
    }

    function add(euint16 a, euint8 b) internal view returns (euint16) {
        return euint16.wrap(Impl.add(euint16.unwrap(a), euint8.unwrap(b)));
    }

    function sub(euint16 a, euint8 b) internal view returns (euint16) {
        return euint16.wrap(Impl.sub(euint16.unwrap(a), euint8.unwrap(b)));
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

    function lte(euint16 a, euint16 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function add(euint16 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.add(euint16.unwrap(a), euint32.unwrap(b)));
    }

    function sub(euint16 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.sub(euint16.unwrap(a), euint32.unwrap(b)));
    }

    function lte(euint16 a, euint32 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint16.unwrap(a), euint32.unwrap(b)));
    }

    function add(euint16 a, euint64 b) internal view returns (euint64) {
        return euint64.wrap(Impl.add(euint16.unwrap(a), euint64.unwrap(b)));
    }

    function sub(euint16 a, euint64 b) internal view returns (euint64) {
        return euint64.wrap(Impl.sub(euint16.unwrap(a), euint64.unwrap(b)));
    }

    function lte(euint16 a, euint64 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint16.unwrap(a), euint64.unwrap(b)));
    }

    function add(euint16 a, euint128 b) internal view returns (euint128) {
        return euint128.wrap(Impl.add(euint16.unwrap(a), euint128.unwrap(b)));
    }

    function sub(euint16 a, euint128 b) internal view returns (euint128) {
        return euint128.wrap(Impl.sub(euint16.unwrap(a), euint128.unwrap(b)));
    }

    function lte(euint16 a, euint128 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint16.unwrap(a), euint128.unwrap(b)));
    }

    function add(euint16 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.add(euint16.unwrap(a), euint256.unwrap(b)));
    }

    function sub(euint16 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.sub(euint16.unwrap(a), euint256.unwrap(b)));
    }

    function lte(euint16 a, euint256 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint16.unwrap(a), euint256.unwrap(b)));
    }

    function add(euint32 a, euint8 b) internal view returns (euint32) {
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint8.unwrap(b)));
    }

    function sub(euint32 a, euint8 b) internal view returns (euint32) {
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint8.unwrap(b)));
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

    function lte(euint32 a, euint16 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint32.unwrap(a), euint16.unwrap(b)));
    }

    function add(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function sub(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function lte(euint32 a, euint32 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function add(euint32 a, euint64 b) internal view returns (euint64) {
        return euint64.wrap(Impl.add(euint32.unwrap(a), euint64.unwrap(b)));
    }

    function sub(euint32 a, euint64 b) internal view returns (euint64) {
        return euint64.wrap(Impl.sub(euint32.unwrap(a), euint64.unwrap(b)));
    }

    function lte(euint32 a, euint64 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint32.unwrap(a), euint64.unwrap(b)));
    }

    function add(euint32 a, euint128 b) internal view returns (euint128) {
        return euint128.wrap(Impl.add(euint32.unwrap(a), euint128.unwrap(b)));
    }

    function sub(euint32 a, euint128 b) internal view returns (euint128) {
        return euint128.wrap(Impl.sub(euint32.unwrap(a), euint128.unwrap(b)));
    }

    function lte(euint32 a, euint128 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint32.unwrap(a), euint128.unwrap(b)));
    }

    function add(euint32 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.add(euint32.unwrap(a), euint256.unwrap(b)));
    }

    function sub(euint32 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.sub(euint32.unwrap(a), euint256.unwrap(b)));
    }

    function lte(euint32 a, euint256 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint32.unwrap(a), euint256.unwrap(b)));
    }

    function add(euint64 a, euint8 b) internal view returns (euint64) {
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint8.unwrap(b)));
    }

    function sub(euint64 a, euint8 b) internal view returns (euint64) {
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint8.unwrap(b)));
    }

    function lte(euint64 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint64.unwrap(a), euint8.unwrap(b)));
    }

    function add(euint64 a, euint16 b) internal view returns (euint64) {
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint16.unwrap(b)));
    }

    function sub(euint64 a, euint16 b) internal view returns (euint64) {
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint16.unwrap(b)));
    }

    function lte(euint64 a, euint16 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint64.unwrap(a), euint16.unwrap(b)));
    }

    function add(euint64 a, euint32 b) internal view returns (euint64) {
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint32.unwrap(b)));
    }

    function sub(euint64 a, euint32 b) internal view returns (euint64) {
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint32.unwrap(b)));
    }

    function lte(euint64 a, euint32 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint64.unwrap(a), euint32.unwrap(b)));
    }

    function add(euint64 a, euint64 b) internal view returns (euint64) {
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint64.unwrap(b)));
    }

    function sub(euint64 a, euint64 b) internal view returns (euint64) {
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint64.unwrap(b)));
    }

    function lte(euint64 a, euint64 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint64.unwrap(a), euint64.unwrap(b)));
    }

    function add(euint64 a, euint128 b) internal view returns (euint128) {
        return euint128.wrap(Impl.add(euint64.unwrap(a), euint128.unwrap(b)));
    }

    function sub(euint64 a, euint128 b) internal view returns (euint128) {
        return euint128.wrap(Impl.sub(euint64.unwrap(a), euint128.unwrap(b)));
    }

    function lte(euint64 a, euint128 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint64.unwrap(a), euint128.unwrap(b)));
    }

    function add(euint64 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.add(euint64.unwrap(a), euint256.unwrap(b)));
    }

    function sub(euint64 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.sub(euint64.unwrap(a), euint256.unwrap(b)));
    }

    function lte(euint64 a, euint256 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint64.unwrap(a), euint256.unwrap(b)));
    }

    function add(euint128 a, euint8 b) internal view returns (euint128) {
        return euint128.wrap(Impl.add(euint128.unwrap(a), euint8.unwrap(b)));
    }

    function sub(euint128 a, euint8 b) internal view returns (euint128) {
        return euint128.wrap(Impl.sub(euint128.unwrap(a), euint8.unwrap(b)));
    }

    function lte(euint128 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint128.unwrap(a), euint8.unwrap(b)));
    }

    function add(euint128 a, euint16 b) internal view returns (euint128) {
        return euint128.wrap(Impl.add(euint128.unwrap(a), euint16.unwrap(b)));
    }

    function sub(euint128 a, euint16 b) internal view returns (euint128) {
        return euint128.wrap(Impl.sub(euint128.unwrap(a), euint16.unwrap(b)));
    }

    function lte(euint128 a, euint16 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint128.unwrap(a), euint16.unwrap(b)));
    }

    function add(euint128 a, euint32 b) internal view returns (euint128) {
        return euint128.wrap(Impl.add(euint128.unwrap(a), euint32.unwrap(b)));
    }

    function sub(euint128 a, euint32 b) internal view returns (euint128) {
        return euint128.wrap(Impl.sub(euint128.unwrap(a), euint32.unwrap(b)));
    }

    function lte(euint128 a, euint32 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint128.unwrap(a), euint32.unwrap(b)));
    }

    function add(euint128 a, euint64 b) internal view returns (euint128) {
        return euint128.wrap(Impl.add(euint128.unwrap(a), euint64.unwrap(b)));
    }

    function sub(euint128 a, euint64 b) internal view returns (euint128) {
        return euint128.wrap(Impl.sub(euint128.unwrap(a), euint64.unwrap(b)));
    }

    function lte(euint128 a, euint64 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint128.unwrap(a), euint64.unwrap(b)));
    }

    function add(euint128 a, euint128 b) internal view returns (euint128) {
        return euint128.wrap(Impl.add(euint128.unwrap(a), euint128.unwrap(b)));
    }

    function sub(euint128 a, euint128 b) internal view returns (euint128) {
        return euint128.wrap(Impl.sub(euint128.unwrap(a), euint128.unwrap(b)));
    }

    function lte(euint128 a, euint128 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint128.unwrap(a), euint128.unwrap(b)));
    }

    function add(euint128 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.add(euint128.unwrap(a), euint256.unwrap(b)));
    }

    function sub(euint128 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.sub(euint128.unwrap(a), euint256.unwrap(b)));
    }

    function lte(euint128 a, euint256 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint128.unwrap(a), euint256.unwrap(b)));
    }

    function add(euint256 a, euint8 b) internal view returns (euint256) {
        return euint256.wrap(Impl.add(euint256.unwrap(a), euint8.unwrap(b)));
    }

    function sub(euint256 a, euint8 b) internal view returns (euint256) {
        return euint256.wrap(Impl.sub(euint256.unwrap(a), euint8.unwrap(b)));
    }

    function lte(euint256 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint256.unwrap(a), euint8.unwrap(b)));
    }

    function add(euint256 a, euint16 b) internal view returns (euint256) {
        return euint256.wrap(Impl.add(euint256.unwrap(a), euint16.unwrap(b)));
    }

    function sub(euint256 a, euint16 b) internal view returns (euint256) {
        return euint256.wrap(Impl.sub(euint256.unwrap(a), euint16.unwrap(b)));
    }

    function lte(euint256 a, euint16 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint256.unwrap(a), euint16.unwrap(b)));
    }

    function add(euint256 a, euint32 b) internal view returns (euint256) {
        return euint256.wrap(Impl.add(euint256.unwrap(a), euint32.unwrap(b)));
    }

    function sub(euint256 a, euint32 b) internal view returns (euint256) {
        return euint256.wrap(Impl.sub(euint256.unwrap(a), euint32.unwrap(b)));
    }

    function lte(euint256 a, euint32 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint256.unwrap(a), euint32.unwrap(b)));
    }

    function add(euint256 a, euint64 b) internal view returns (euint256) {
        return euint256.wrap(Impl.add(euint256.unwrap(a), euint64.unwrap(b)));
    }

    function sub(euint256 a, euint64 b) internal view returns (euint256) {
        return euint256.wrap(Impl.sub(euint256.unwrap(a), euint64.unwrap(b)));
    }

    function lte(euint256 a, euint64 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint256.unwrap(a), euint64.unwrap(b)));
    }

    function add(euint256 a, euint128 b) internal view returns (euint256) {
        return euint256.wrap(Impl.add(euint256.unwrap(a), euint128.unwrap(b)));
    }

    function sub(euint256 a, euint128 b) internal view returns (euint256) {
        return euint256.wrap(Impl.sub(euint256.unwrap(a), euint128.unwrap(b)));
    }

    function lte(euint256 a, euint128 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint256.unwrap(a), euint128.unwrap(b)));
    }

    function add(euint256 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.add(euint256.unwrap(a), euint256.unwrap(b)));
    }

    function sub(euint256 a, euint256 b) internal view returns (euint256) {
        return euint256.wrap(Impl.sub(euint256.unwrap(a), euint256.unwrap(b)));
    }

    function lte(euint256 a, euint256 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint256.unwrap(a), euint256.unwrap(b)));
    }

    function to_euint8(euint16 v) internal view returns (euint8) {
        return euint8.wrap(Impl.cast(euint16.unwrap(v), Common.euint16_t));
    }

    function to_euint8(euint32 v) internal view returns (euint8) {
        return euint8.wrap(Impl.cast(euint32.unwrap(v), Common.euint32_t));
    }

    function to_euint8(euint64 v) internal view returns (euint8) {
        return euint8.wrap(Impl.cast(euint64.unwrap(v), Common.euint64_t));
    }

    function to_euint8(euint128 v) internal view returns (euint8) {
        return euint8.wrap(Impl.cast(euint128.unwrap(v), Common.euint128_t));
    }

    function to_euint8(euint256 v) internal view returns (euint8) {
        return euint8.wrap(Impl.cast(euint256.unwrap(v), Common.euint256_t));
    }

    function to_euint16(euint8 v) internal view returns (euint16) {
        return euint16.wrap(Impl.cast(euint8.unwrap(v), Common.euint8_t));
    }

    function to_euint16(euint32 v) internal view returns (euint16) {
        return euint16.wrap(Impl.cast(euint32.unwrap(v), Common.euint32_t));
    }

    function to_euint16(euint64 v) internal view returns (euint16) {
        return euint16.wrap(Impl.cast(euint64.unwrap(v), Common.euint64_t));
    }

    function to_euint16(euint128 v) internal view returns (euint16) {
        return euint16.wrap(Impl.cast(euint128.unwrap(v), Common.euint128_t));
    }

    function to_euint16(euint256 v) internal view returns (euint16) {
        return euint16.wrap(Impl.cast(euint256.unwrap(v), Common.euint256_t));
    }

    function to_euint32(euint8 v) internal view returns (euint32) {
        return euint32.wrap(Impl.cast(euint8.unwrap(v), Common.euint8_t));
    }

    function to_euint32(euint16 v) internal view returns (euint32) {
        return euint32.wrap(Impl.cast(euint16.unwrap(v), Common.euint16_t));
    }

    function to_euint32(euint64 v) internal view returns (euint32) {
        return euint32.wrap(Impl.cast(euint64.unwrap(v), Common.euint64_t));
    }

    function to_euint32(euint128 v) internal view returns (euint32) {
        return euint32.wrap(Impl.cast(euint128.unwrap(v), Common.euint128_t));
    }

    function to_euint32(euint256 v) internal view returns (euint32) {
        return euint32.wrap(Impl.cast(euint256.unwrap(v), Common.euint256_t));
    }

    function to_euint64(euint8 v) internal view returns (euint64) {
        return euint64.wrap(Impl.cast(euint8.unwrap(v), Common.euint8_t));
    }

    function to_euint64(euint16 v) internal view returns (euint64) {
        return euint64.wrap(Impl.cast(euint16.unwrap(v), Common.euint16_t));
    }

    function to_euint64(euint32 v) internal view returns (euint64) {
        return euint64.wrap(Impl.cast(euint32.unwrap(v), Common.euint32_t));
    }

    function to_euint64(euint128 v) internal view returns (euint64) {
        return euint64.wrap(Impl.cast(euint128.unwrap(v), Common.euint128_t));
    }

    function to_euint64(euint256 v) internal view returns (euint64) {
        return euint64.wrap(Impl.cast(euint256.unwrap(v), Common.euint256_t));
    }

    function to_euint128(euint8 v) internal view returns (euint128) {
        return euint128.wrap(Impl.cast(euint8.unwrap(v), Common.euint8_t));
    }

    function to_euint128(euint16 v) internal view returns (euint128) {
        return euint128.wrap(Impl.cast(euint16.unwrap(v), Common.euint16_t));
    }

    function to_euint128(euint32 v) internal view returns (euint128) {
        return euint128.wrap(Impl.cast(euint32.unwrap(v), Common.euint32_t));
    }

    function to_euint128(euint64 v) internal view returns (euint128) {
        return euint128.wrap(Impl.cast(euint64.unwrap(v), Common.euint64_t));
    }

    function to_euint128(euint256 v) internal view returns (euint128) {
        return euint128.wrap(Impl.cast(euint256.unwrap(v), Common.euint256_t));
    }

    function to_euint256(euint8 v) internal view returns (euint256) {
        return euint256.wrap(Impl.cast(euint8.unwrap(v), Common.euint8_t));
    }

    function to_euint256(euint16 v) internal view returns (euint256) {
        return euint256.wrap(Impl.cast(euint16.unwrap(v), Common.euint16_t));
    }

    function to_euint256(euint32 v) internal view returns (euint256) {
        return euint256.wrap(Impl.cast(euint32.unwrap(v), Common.euint32_t));
    }

    function to_euint256(euint64 v) internal view returns (euint256) {
        return euint256.wrap(Impl.cast(euint64.unwrap(v), Common.euint64_t));
    }

    function to_euint256(euint128 v) internal view returns (euint256) {
        return euint256.wrap(Impl.cast(euint128.unwrap(v), Common.euint128_t));
    }
}

library Impl {
    uint256 constant MaxCiphertextBytesLen = 32 + 65544;

    function add(uint256 a, uint256 b) internal view returns (uint256 result) {
        if (a == 0) {
            return b;
        } else if (b == 0) {
            return a;
        }
        bytes32[2] memory input;
        input[0] = bytes32(a);
        input[1] = bytes32(b);
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the add precompile.
        uint256 precompile = Precompiles.Add;
        assembly {
            if iszero(
                staticcall(
                    gas(),
                    precompile,
                    input,
                    inputLen,
                    output,
                    outputLen
                )
            ) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function sub(uint256 a, uint256 b) internal view returns (uint256 result) {
        if (a == 0) {
            return b;
        } else if (b == 0) {
            return a;
        }
        bytes32[2] memory input;
        input[0] = bytes32(a);
        input[1] = bytes32(b);
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the add precompile.
        uint256 precompile = Precompiles.Subtract;
        assembly {
            if iszero(
                staticcall(
                    gas(),
                    precompile,
                    input,
                    inputLen,
                    output,
                    outputLen
                )
            ) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    // Evaluate `lhs <= rhs` on the given ciphertexts and, if successful, return the resulting ciphertext.
    // If successful, the resulting ciphertext is automatically verified.
    function lte(
        uint256 lhs,
        uint256 rhs
    ) internal view returns (uint256 result) {
        bytes32[2] memory input;
        input[0] = bytes32(lhs);
        input[1] = bytes32(rhs);
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the lte precompile.
        uint256 precompile = Precompiles.LessThanOrEqual;
        assembly {
            if iszero(
                staticcall(
                    gas(),
                    precompile,
                    input,
                    inputLen,
                    output,
                    outputLen
                )
            ) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    //    function safeAdd(uint256 a, uint256 b) internal view returns (uint256) {
    //        TODO: Call addSafe() precompile.
    //        return 0;
    //    }

    function cast(
        uint256 ciphertext,
        uint8 toType
    ) internal view returns (uint256) {
        uint256 inputLen = 33;

        bytes memory input = new bytes(inputLen);

        assembly {
            mstore(add(input, 32), ciphertext)
        }

        // Pass in the desired return type
        input[inputLen - 1] = bytes1(toType);

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the cast precompile.
        uint256 precompile = Precompiles.Cast;
        assembly {
            if iszero(
                staticcall(
                    gas(),
                    precompile,
                    add(input, 32), // jump over the 32-bit `size` field of the `bytes` data structure to read actual bytes
                    inputLen,
                    output,
                    outputLen
                )
            ) {
                revert(0, 0)
            }
        }
        return 0;
    }

    function reencrypt(
        uint256 ciphertext
    ) internal view returns (bytes memory reencrypted) {
        bytes32[1] memory input;
        input[0] = bytes32(ciphertext);
        uint256 inputLen = 32;

        reencrypted = new bytes(MaxCiphertextBytesLen);

        // Call the reencrypt precompile.
        uint256 precompile = Precompiles.Reencrypt;
        assembly {
            if iszero(
                staticcall(
                    gas(),
                    precompile,
                    input,
                    inputLen,
                    reencrypted,
                    MaxCiphertextBytesLen
                )
            ) {
                revert(0, 0)
            }
        }
    }

    function verify(
        bytes memory _ciphertextWithProof,
        uint8 _toType
    ) internal view returns (uint256) {
        // TODO depending the TFHE-rs implementation of the type system.
        return 0;
    }

    function delegate(uint256 ciphertext) internal view {
        bytes32[1] memory input;
        input[0] = bytes32(ciphertext);
        uint256 inputLen = 32;

        // Call the delegate precompile
        uint256 precompile = Precompiles.Delegate;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, 0, 0)) {
                revert(0, 0)
            }
        }
    }

    function requireCt(uint256 ciphertext) internal view {
        bytes32[1] memory input;
        input[0] = bytes32(ciphertext);
        uint256 inputLen = 32;

        // Call the require precompile.
        uint256 precompile = Precompiles.Require;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, 0, 0)) {
                revert(0, 0)
            }
        }
    }
}

library Precompiles {
    uint256 public constant Add = 65;
    uint256 public constant Verify = 66;
    uint256 public constant Reencrypt = 67;
    uint256 public constant Delegate = 68;
    uint256 public constant Require = 69;
    uint256 public constant LessThanOrEqual = 70;
    uint256 public constant Subtract = 71;
    uint256 public constant Multiply = 72;
    uint256 public constant LessThan = 73;
    uint256 public constant Random = 74;
    uint256 public constant optimisticRequire = 75;
    uint256 public constant Cast = 76;
}

library Ciphertext {
    function as_euint8(
        bytes memory ciphertextWithProof
    ) internal view returns (euint8) {
        return euint8.wrap(Impl.verify(ciphertextWithProof, Common.euint8_t));
    }

    function reencrypt(
        euint8 ciphertext
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint8.unwrap(ciphertext));
    }

    function delegate(euint8 ciphertext) internal view {
        Impl.delegate(euint8.unwrap(ciphertext));
    }

    function as_euint16(
        bytes memory ciphertextWithProof
    ) internal view returns (euint16) {
        return euint16.wrap(Impl.verify(ciphertextWithProof, Common.euint16_t));
    }

    function reencrypt(
        euint16 ciphertext
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint16.unwrap(ciphertext));
    }

    function delegate(euint16 ciphertext) internal view {
        Impl.delegate(euint16.unwrap(ciphertext));
    }

    function as_euint32(
        bytes memory ciphertextWithProof
    ) internal view returns (euint32) {
        return euint32.wrap(Impl.verify(ciphertextWithProof, Common.euint32_t));
    }

    function reencrypt(
        euint32 ciphertext
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint32.unwrap(ciphertext));
    }

    function delegate(euint32 ciphertext) internal view {
        Impl.delegate(euint32.unwrap(ciphertext));
    }

    function as_euint64(
        bytes memory ciphertextWithProof
    ) internal view returns (euint64) {
        return euint64.wrap(Impl.verify(ciphertextWithProof, Common.euint64_t));
    }

    function reencrypt(
        euint64 ciphertext
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint64.unwrap(ciphertext));
    }

    function delegate(euint64 ciphertext) internal view {
        Impl.delegate(euint64.unwrap(ciphertext));
    }

    function as_euint128(
        bytes memory ciphertextWithProof
    ) internal view returns (euint128) {
        return
            euint128.wrap(Impl.verify(ciphertextWithProof, Common.euint128_t));
    }

    function reencrypt(
        euint128 ciphertext
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint128.unwrap(ciphertext));
    }

    function delegate(euint128 ciphertext) internal view {
        Impl.delegate(euint128.unwrap(ciphertext));
    }

    function as_euint256(
        bytes memory ciphertextWithProof
    ) internal view returns (euint256) {
        return
            euint256.wrap(Impl.verify(ciphertextWithProof, Common.euint256_t));
    }

    function reencrypt(
        euint256 ciphertext
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint256.unwrap(ciphertext));
    }

    function delegate(euint256 ciphertext) internal view {
        Impl.delegate(euint256.unwrap(ciphertext));
    }

    function requireCt(euint8 ciphertext) internal view {
        {
            Impl.requireCt(euint8.unwrap(ciphertext));
        }
    }
}
