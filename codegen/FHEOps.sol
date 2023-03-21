// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

type FHEUInt8 is uint256;
type FHEUInt16 is uint256;
type FHEUInt32 is uint256;
type FHEUInt64 is uint256;
type FHEUInt128 is uint256;
type FHEUInt256 is uint256;

library Common {
// Values used to communicate types at runtime to the cast() precompile.
    uint8 internal constant typeUInt8 = 0;
    uint8 internal constant typeUInt16 = 1;
    uint8 internal constant typeUInt32 = 2;
    uint8 internal constant typeUInt64 = 3;
    uint8 internal constant typeUInt128 = 4;
    uint8 internal constant typeUInt256 = 5;
}

library FHEOps {
    function add(FHEUInt8 a, FHEUInt8 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.add(FHEUInt8.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function sub(FHEUInt8 a, FHEUInt8 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.sub(FHEUInt8.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function lte(FHEUInt8 a, FHEUInt8 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt8.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function add(FHEUInt8 a, FHEUInt16 b) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.add(FHEUInt8.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function sub(FHEUInt8 a, FHEUInt16 b) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.sub(FHEUInt8.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function lte(FHEUInt8 a, FHEUInt16 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt8.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function add(FHEUInt8 a, FHEUInt32 b) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.add(FHEUInt8.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function sub(FHEUInt8 a, FHEUInt32 b) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.sub(FHEUInt8.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function lte(FHEUInt8 a, FHEUInt32 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt8.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function add(FHEUInt8 a, FHEUInt64 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.add(FHEUInt8.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function sub(FHEUInt8 a, FHEUInt64 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.sub(FHEUInt8.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function lte(FHEUInt8 a, FHEUInt64 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt8.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function add(FHEUInt8 a, FHEUInt128 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.add(FHEUInt8.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function sub(FHEUInt8 a, FHEUInt128 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.sub(FHEUInt8.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function lte(FHEUInt8 a, FHEUInt128 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt8.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function add(FHEUInt8 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.add(FHEUInt8.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function sub(FHEUInt8 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.sub(FHEUInt8.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function lte(FHEUInt8 a, FHEUInt256 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt8.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function add(FHEUInt16 a, FHEUInt8 b) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.add(FHEUInt16.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function sub(FHEUInt16 a, FHEUInt8 b) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.sub(FHEUInt16.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function lte(FHEUInt16 a, FHEUInt8 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt16.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function add(FHEUInt16 a, FHEUInt16 b) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.add(FHEUInt16.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function sub(FHEUInt16 a, FHEUInt16 b) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.sub(FHEUInt16.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function lte(FHEUInt16 a, FHEUInt16 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt16.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function add(FHEUInt16 a, FHEUInt32 b) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.add(FHEUInt16.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function sub(FHEUInt16 a, FHEUInt32 b) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.sub(FHEUInt16.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function lte(FHEUInt16 a, FHEUInt32 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt16.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function add(FHEUInt16 a, FHEUInt64 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.add(FHEUInt16.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function sub(FHEUInt16 a, FHEUInt64 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.sub(FHEUInt16.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function lte(FHEUInt16 a, FHEUInt64 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt16.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function add(FHEUInt16 a, FHEUInt128 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.add(FHEUInt16.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function sub(FHEUInt16 a, FHEUInt128 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.sub(FHEUInt16.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function lte(FHEUInt16 a, FHEUInt128 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt16.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function add(FHEUInt16 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.add(FHEUInt16.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function sub(FHEUInt16 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.sub(FHEUInt16.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function lte(FHEUInt16 a, FHEUInt256 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt16.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function add(FHEUInt32 a, FHEUInt8 b) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.add(FHEUInt32.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function sub(FHEUInt32 a, FHEUInt8 b) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.sub(FHEUInt32.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function lte(FHEUInt32 a, FHEUInt8 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt32.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function add(FHEUInt32 a, FHEUInt16 b) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.add(FHEUInt32.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function sub(FHEUInt32 a, FHEUInt16 b) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.sub(FHEUInt32.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function lte(FHEUInt32 a, FHEUInt16 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt32.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function add(FHEUInt32 a, FHEUInt32 b) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.add(FHEUInt32.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function sub(FHEUInt32 a, FHEUInt32 b) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.sub(FHEUInt32.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function lte(FHEUInt32 a, FHEUInt32 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt32.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function add(FHEUInt32 a, FHEUInt64 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.add(FHEUInt32.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function sub(FHEUInt32 a, FHEUInt64 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.sub(FHEUInt32.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function lte(FHEUInt32 a, FHEUInt64 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt32.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function add(FHEUInt32 a, FHEUInt128 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.add(FHEUInt32.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function sub(FHEUInt32 a, FHEUInt128 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.sub(FHEUInt32.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function lte(FHEUInt32 a, FHEUInt128 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt32.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function add(FHEUInt32 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.add(FHEUInt32.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function sub(FHEUInt32 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.sub(FHEUInt32.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function lte(FHEUInt32 a, FHEUInt256 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt32.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function add(FHEUInt64 a, FHEUInt8 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.add(FHEUInt64.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function sub(FHEUInt64 a, FHEUInt8 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.sub(FHEUInt64.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function lte(FHEUInt64 a, FHEUInt8 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt64.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function add(FHEUInt64 a, FHEUInt16 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.add(FHEUInt64.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function sub(FHEUInt64 a, FHEUInt16 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.sub(FHEUInt64.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function lte(FHEUInt64 a, FHEUInt16 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt64.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function add(FHEUInt64 a, FHEUInt32 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.add(FHEUInt64.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function sub(FHEUInt64 a, FHEUInt32 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.sub(FHEUInt64.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function lte(FHEUInt64 a, FHEUInt32 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt64.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function add(FHEUInt64 a, FHEUInt64 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.add(FHEUInt64.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function sub(FHEUInt64 a, FHEUInt64 b) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.sub(FHEUInt64.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function lte(FHEUInt64 a, FHEUInt64 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt64.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function add(FHEUInt64 a, FHEUInt128 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.add(FHEUInt64.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function sub(FHEUInt64 a, FHEUInt128 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.sub(FHEUInt64.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function lte(FHEUInt64 a, FHEUInt128 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt64.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function add(FHEUInt64 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.add(FHEUInt64.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function sub(FHEUInt64 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.sub(FHEUInt64.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function lte(FHEUInt64 a, FHEUInt256 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt64.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function add(FHEUInt128 a, FHEUInt8 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.add(FHEUInt128.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function sub(FHEUInt128 a, FHEUInt8 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.sub(FHEUInt128.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function lte(FHEUInt128 a, FHEUInt8 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt128.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function add(FHEUInt128 a, FHEUInt16 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.add(FHEUInt128.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function sub(FHEUInt128 a, FHEUInt16 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.sub(FHEUInt128.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function lte(FHEUInt128 a, FHEUInt16 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt128.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function add(FHEUInt128 a, FHEUInt32 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.add(FHEUInt128.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function sub(FHEUInt128 a, FHEUInt32 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.sub(FHEUInt128.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function lte(FHEUInt128 a, FHEUInt32 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt128.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function add(FHEUInt128 a, FHEUInt64 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.add(FHEUInt128.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function sub(FHEUInt128 a, FHEUInt64 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.sub(FHEUInt128.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function lte(FHEUInt128 a, FHEUInt64 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt128.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function add(FHEUInt128 a, FHEUInt128 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.add(FHEUInt128.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function sub(FHEUInt128 a, FHEUInt128 b) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.sub(FHEUInt128.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function lte(FHEUInt128 a, FHEUInt128 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt128.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function add(FHEUInt128 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.add(FHEUInt128.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function sub(FHEUInt128 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.sub(FHEUInt128.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function lte(FHEUInt128 a, FHEUInt256 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt128.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function add(FHEUInt256 a, FHEUInt8 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.add(FHEUInt256.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function sub(FHEUInt256 a, FHEUInt8 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.sub(FHEUInt256.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function lte(FHEUInt256 a, FHEUInt8 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt256.unwrap(a), FHEUInt8.unwrap(b)));
    }

    function add(FHEUInt256 a, FHEUInt16 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.add(FHEUInt256.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function sub(FHEUInt256 a, FHEUInt16 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.sub(FHEUInt256.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function lte(FHEUInt256 a, FHEUInt16 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt256.unwrap(a), FHEUInt16.unwrap(b)));
    }

    function add(FHEUInt256 a, FHEUInt32 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.add(FHEUInt256.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function sub(FHEUInt256 a, FHEUInt32 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.sub(FHEUInt256.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function lte(FHEUInt256 a, FHEUInt32 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt256.unwrap(a), FHEUInt32.unwrap(b)));
    }

    function add(FHEUInt256 a, FHEUInt64 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.add(FHEUInt256.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function sub(FHEUInt256 a, FHEUInt64 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.sub(FHEUInt256.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function lte(FHEUInt256 a, FHEUInt64 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt256.unwrap(a), FHEUInt64.unwrap(b)));
    }

    function add(FHEUInt256 a, FHEUInt128 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.add(FHEUInt256.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function sub(FHEUInt256 a, FHEUInt128 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.sub(FHEUInt256.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function lte(FHEUInt256 a, FHEUInt128 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt256.unwrap(a), FHEUInt128.unwrap(b)));
    }

    function add(FHEUInt256 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.add(FHEUInt256.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function sub(FHEUInt256 a, FHEUInt256 b) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.sub(FHEUInt256.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function lte(FHEUInt256 a, FHEUInt256 b) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.lte(FHEUInt256.unwrap(a), FHEUInt256.unwrap(b)));
    }

    function toFHEUint8(FHEUInt16 v) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.cast(FHEUInt16.unwrap(v), Common.typeUInt16));
    }

    function toFHEUint8(FHEUInt32 v) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.cast(FHEUInt32.unwrap(v), Common.typeUInt32));
    }

    function toFHEUint8(FHEUInt64 v) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.cast(FHEUInt64.unwrap(v), Common.typeUInt64));
    }

    function toFHEUint8(FHEUInt128 v) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.cast(FHEUInt128.unwrap(v), Common.typeUInt128));
    }

    function toFHEUint8(FHEUInt256 v) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.cast(FHEUInt256.unwrap(v), Common.typeUInt256));
    }

    function toFHEUint16(FHEUInt8 v) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.cast(FHEUInt8.unwrap(v), Common.typeUInt8));
    }

    function toFHEUint16(FHEUInt32 v) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.cast(FHEUInt32.unwrap(v), Common.typeUInt32));
    }

    function toFHEUint16(FHEUInt64 v) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.cast(FHEUInt64.unwrap(v), Common.typeUInt64));
    }

    function toFHEUint16(FHEUInt128 v) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.cast(FHEUInt128.unwrap(v), Common.typeUInt128));
    }

    function toFHEUint16(FHEUInt256 v) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.cast(FHEUInt256.unwrap(v), Common.typeUInt256));
    }

    function toFHEUint32(FHEUInt8 v) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.cast(FHEUInt8.unwrap(v), Common.typeUInt8));
    }

    function toFHEUint32(FHEUInt16 v) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.cast(FHEUInt16.unwrap(v), Common.typeUInt16));
    }

    function toFHEUint32(FHEUInt64 v) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.cast(FHEUInt64.unwrap(v), Common.typeUInt64));
    }

    function toFHEUint32(FHEUInt128 v) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.cast(FHEUInt128.unwrap(v), Common.typeUInt128));
    }

    function toFHEUint32(FHEUInt256 v) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.cast(FHEUInt256.unwrap(v), Common.typeUInt256));
    }

    function toFHEUint64(FHEUInt8 v) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.cast(FHEUInt8.unwrap(v), Common.typeUInt8));
    }

    function toFHEUint64(FHEUInt16 v) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.cast(FHEUInt16.unwrap(v), Common.typeUInt16));
    }

    function toFHEUint64(FHEUInt32 v) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.cast(FHEUInt32.unwrap(v), Common.typeUInt32));
    }

    function toFHEUint64(FHEUInt128 v) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.cast(FHEUInt128.unwrap(v), Common.typeUInt128));
    }

    function toFHEUint64(FHEUInt256 v) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.cast(FHEUInt256.unwrap(v), Common.typeUInt256));
    }

    function toFHEUint128(FHEUInt8 v) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.cast(FHEUInt8.unwrap(v), Common.typeUInt8));
    }

    function toFHEUint128(FHEUInt16 v) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.cast(FHEUInt16.unwrap(v), Common.typeUInt16));
    }

    function toFHEUint128(FHEUInt32 v) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.cast(FHEUInt32.unwrap(v), Common.typeUInt32));
    }

    function toFHEUint128(FHEUInt64 v) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.cast(FHEUInt64.unwrap(v), Common.typeUInt64));
    }

    function toFHEUint128(FHEUInt256 v) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.cast(FHEUInt256.unwrap(v), Common.typeUInt256));
    }

    function toFHEUint256(FHEUInt8 v) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.cast(FHEUInt8.unwrap(v), Common.typeUInt8));
    }

    function toFHEUint256(FHEUInt16 v) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.cast(FHEUInt16.unwrap(v), Common.typeUInt16));
    }

    function toFHEUint256(FHEUInt32 v) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.cast(FHEUInt32.unwrap(v), Common.typeUInt32));
    }

    function toFHEUint256(FHEUInt64 v) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.cast(FHEUInt64.unwrap(v), Common.typeUInt64));
    }

    function toFHEUint256(FHEUInt128 v) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.cast(FHEUInt128.unwrap(v), Common.typeUInt128));
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
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
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
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    // Evaluate `lhs <= rhs` on the given ciphertexts and, if successful, return the resulting ciphertext.
    // If successful, the resulting ciphertext is automatically verified.
    function lte(uint256 lhs, uint256 rhs) internal view returns (uint256 result) {
        bytes32[2] memory input;
        input[0] = bytes32(lhs);
        input[1] = bytes32(rhs);
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the lte precompile.
        uint256 precompile = Precompiles.LessThanOrEqual;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }
    

//    function safeAdd(uint256 a, uint256 b) internal view returns (uint256) {
//        TODO: Call addSafe() precompile.
//        return 0;
//    }

    function cast(uint256 ciphertext, uint8 toType) internal view returns(uint256) {
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

    function reencrypt(uint256 ciphertext) internal view returns (bytes memory reencrypted) {
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
    function verify8(bytes memory ciphertextWithProof) internal view returns (FHEUInt8) {
        return FHEUInt8.wrap(Impl.verify(ciphertextWithProof, Common.typeUInt8));
    }

    function reencrypt(FHEUInt8 ciphertext) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(FHEUInt8.unwrap(ciphertext));
    }

    function delegate(FHEUInt8 ciphertext) internal view {
        Impl.delegate(FHEUInt8.unwrap(ciphertext));
    }

    function verify16(bytes memory ciphertextWithProof) internal view returns (FHEUInt16) {
        return FHEUInt16.wrap(Impl.verify(ciphertextWithProof, Common.typeUInt16));
    }

    function reencrypt(FHEUInt16 ciphertext) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(FHEUInt16.unwrap(ciphertext));
    }

    function delegate(FHEUInt16 ciphertext) internal view {
        Impl.delegate(FHEUInt16.unwrap(ciphertext));
    }

    function verify32(bytes memory ciphertextWithProof) internal view returns (FHEUInt32) {
        return FHEUInt32.wrap(Impl.verify(ciphertextWithProof, Common.typeUInt32));
    }

    function reencrypt(FHEUInt32 ciphertext) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(FHEUInt32.unwrap(ciphertext));
    }

    function delegate(FHEUInt32 ciphertext) internal view {
        Impl.delegate(FHEUInt32.unwrap(ciphertext));
    }

    function verify64(bytes memory ciphertextWithProof) internal view returns (FHEUInt64) {
        return FHEUInt64.wrap(Impl.verify(ciphertextWithProof, Common.typeUInt64));
    }

    function reencrypt(FHEUInt64 ciphertext) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(FHEUInt64.unwrap(ciphertext));
    }

    function delegate(FHEUInt64 ciphertext) internal view {
        Impl.delegate(FHEUInt64.unwrap(ciphertext));
    }

    function verify128(bytes memory ciphertextWithProof) internal view returns (FHEUInt128) {
        return FHEUInt128.wrap(Impl.verify(ciphertextWithProof, Common.typeUInt128));
    }

    function reencrypt(FHEUInt128 ciphertext) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(FHEUInt128.unwrap(ciphertext));
    }

    function delegate(FHEUInt128 ciphertext) internal view {
        Impl.delegate(FHEUInt128.unwrap(ciphertext));
    }

    function verify256(bytes memory ciphertextWithProof) internal view returns (FHEUInt256) {
        return FHEUInt256.wrap(Impl.verify(ciphertextWithProof, Common.typeUInt256));
    }

    function reencrypt(FHEUInt256 ciphertext) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(FHEUInt256.unwrap(ciphertext));
    }

    function delegate(FHEUInt256 ciphertext) internal view {
        Impl.delegate(FHEUInt256.unwrap(ciphertext));
    }

    function requireCt(FHEUInt8 ciphertext) internal view {{
        Impl.requireCt(FHEUInt8.unwrap(ciphertext));
    }}
}