// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @dev Split out of FHEVMManualTestSuite to stay under the EIP-170 contract size limit.
contract FHEVMOperatorEdgeCaseTestSuite is E2ECoprocessorConfig {
    bytes32[] private _resBatch;

    /// @dev Shift results are ordered [shl, shr, rotl, rotr] for the scalar RHS, then the encrypted RHS.
    function resBatch() external view returns (bytes32[] memory) {
        return _resBatch;
    }

    function _push(euint8 v) private {
        FHE.makePubliclyDecryptable(v);
        _resBatch.push(euint8.unwrap(v));
    }

    function _push(euint16 v) private {
        FHE.makePubliclyDecryptable(v);
        _resBatch.push(euint16.unwrap(v));
    }

    function _push(euint32 v) private {
        FHE.makePubliclyDecryptable(v);
        _resBatch.push(euint32.unwrap(v));
    }

    function _push(euint64 v) private {
        FHE.makePubliclyDecryptable(v);
        _resBatch.push(euint64.unwrap(v));
    }

    function _push(euint128 v) private {
        FHE.makePubliclyDecryptable(v);
        _resBatch.push(euint128.unwrap(v));
    }

    function _push(euint256 v) private {
        FHE.makePubliclyDecryptable(v);
        _resBatch.push(euint256.unwrap(v));
    }

    function test_shifts_euint8(externalEuint8 a, uint8 s, externalEuint8 e, bytes calldata inputProof) external {
        delete _resBatch;
        euint8 x = FHE.fromExternal(a, inputProof);
        euint8 n = FHE.fromExternal(e, inputProof);
        _push(FHE.shl(x, s));
        _push(FHE.shr(x, s));
        _push(FHE.rotl(x, s));
        _push(FHE.rotr(x, s));
        _push(FHE.shl(x, n));
        _push(FHE.shr(x, n));
        _push(FHE.rotl(x, n));
        _push(FHE.rotr(x, n));
    }

    function test_shifts_euint16(externalEuint16 a, uint8 s, externalEuint8 e, bytes calldata inputProof) external {
        delete _resBatch;
        euint16 x = FHE.fromExternal(a, inputProof);
        euint8 n = FHE.fromExternal(e, inputProof);
        _push(FHE.shl(x, s));
        _push(FHE.shr(x, s));
        _push(FHE.rotl(x, s));
        _push(FHE.rotr(x, s));
        _push(FHE.shl(x, n));
        _push(FHE.shr(x, n));
        _push(FHE.rotl(x, n));
        _push(FHE.rotr(x, n));
    }

    function test_shifts_euint32(externalEuint32 a, uint8 s, externalEuint8 e, bytes calldata inputProof) external {
        delete _resBatch;
        euint32 x = FHE.fromExternal(a, inputProof);
        euint8 n = FHE.fromExternal(e, inputProof);
        _push(FHE.shl(x, s));
        _push(FHE.shr(x, s));
        _push(FHE.rotl(x, s));
        _push(FHE.rotr(x, s));
        _push(FHE.shl(x, n));
        _push(FHE.shr(x, n));
        _push(FHE.rotl(x, n));
        _push(FHE.rotr(x, n));
    }

    function test_shifts_euint64(externalEuint64 a, uint8 s, externalEuint8 e, bytes calldata inputProof) external {
        delete _resBatch;
        euint64 x = FHE.fromExternal(a, inputProof);
        euint8 n = FHE.fromExternal(e, inputProof);
        _push(FHE.shl(x, s));
        _push(FHE.shr(x, s));
        _push(FHE.rotl(x, s));
        _push(FHE.rotr(x, s));
        _push(FHE.shl(x, n));
        _push(FHE.shr(x, n));
        _push(FHE.rotl(x, n));
        _push(FHE.rotr(x, n));
    }

    function test_shifts_euint128(externalEuint128 a, uint8 s, externalEuint8 e, bytes calldata inputProof) external {
        delete _resBatch;
        euint128 x = FHE.fromExternal(a, inputProof);
        euint8 n = FHE.fromExternal(e, inputProof);
        _push(FHE.shl(x, s));
        _push(FHE.shr(x, s));
        _push(FHE.rotl(x, s));
        _push(FHE.rotr(x, s));
        _push(FHE.shl(x, n));
        _push(FHE.shr(x, n));
        _push(FHE.rotl(x, n));
        _push(FHE.rotr(x, n));
    }

    function test_shifts_euint256(externalEuint256 a, uint8 s, externalEuint8 e, bytes calldata inputProof) external {
        delete _resBatch;
        euint256 x = FHE.fromExternal(a, inputProof);
        euint8 n = FHE.fromExternal(e, inputProof);
        _push(FHE.shl(x, s));
        _push(FHE.shr(x, s));
        _push(FHE.rotl(x, s));
        _push(FHE.rotr(x, s));
        _push(FHE.shl(x, n));
        _push(FHE.shr(x, n));
        _push(FHE.rotl(x, n));
        _push(FHE.rotr(x, n));
    }

    function test_divrem_euint8(externalEuint8 a, uint8 d, bytes calldata inputProof) external {
        delete _resBatch;
        euint8 x = FHE.fromExternal(a, inputProof);
        _push(FHE.div(x, d));
        _push(FHE.rem(x, d));
    }

    function test_divrem_euint16(externalEuint16 a, uint16 d, bytes calldata inputProof) external {
        delete _resBatch;
        euint16 x = FHE.fromExternal(a, inputProof);
        _push(FHE.div(x, d));
        _push(FHE.rem(x, d));
    }

    function test_divrem_euint32(externalEuint32 a, uint32 d, bytes calldata inputProof) external {
        delete _resBatch;
        euint32 x = FHE.fromExternal(a, inputProof);
        _push(FHE.div(x, d));
        _push(FHE.rem(x, d));
    }

    function test_divrem_euint64(externalEuint64 a, uint64 d, bytes calldata inputProof) external {
        delete _resBatch;
        euint64 x = FHE.fromExternal(a, inputProof);
        _push(FHE.div(x, d));
        _push(FHE.rem(x, d));
    }

    function test_divrem_euint128(externalEuint128 a, uint128 d, bytes calldata inputProof) external {
        delete _resBatch;
        euint128 x = FHE.fromExternal(a, inputProof);
        _push(FHE.div(x, d));
        _push(FHE.rem(x, d));
    }

    function test_arith_euint8(externalEuint8 a, uint8 s, externalEuint8 e, bytes calldata inputProof) external {
        delete _resBatch;
        euint8 x = FHE.fromExternal(a, inputProof);
        euint8 y = FHE.fromExternal(e, inputProof);
        _push(FHE.add(x, s));
        _push(FHE.sub(x, s));
        _push(FHE.mul(x, s));
        _push(FHE.add(x, y));
        _push(FHE.sub(x, y));
        _push(FHE.mul(x, y));
        _push(FHE.neg(x));
        _push(FHE.not(x));
    }

    function test_arith_euint16(externalEuint16 a, uint16 s, externalEuint16 e, bytes calldata inputProof) external {
        delete _resBatch;
        euint16 x = FHE.fromExternal(a, inputProof);
        euint16 y = FHE.fromExternal(e, inputProof);
        _push(FHE.add(x, s));
        _push(FHE.sub(x, s));
        _push(FHE.mul(x, s));
        _push(FHE.add(x, y));
        _push(FHE.sub(x, y));
        _push(FHE.mul(x, y));
        _push(FHE.neg(x));
        _push(FHE.not(x));
    }

    function test_arith_euint32(externalEuint32 a, uint32 s, externalEuint32 e, bytes calldata inputProof) external {
        delete _resBatch;
        euint32 x = FHE.fromExternal(a, inputProof);
        euint32 y = FHE.fromExternal(e, inputProof);
        _push(FHE.add(x, s));
        _push(FHE.sub(x, s));
        _push(FHE.mul(x, s));
        _push(FHE.add(x, y));
        _push(FHE.sub(x, y));
        _push(FHE.mul(x, y));
        _push(FHE.neg(x));
        _push(FHE.not(x));
    }

    function test_arith_euint64(externalEuint64 a, uint64 s, externalEuint64 e, bytes calldata inputProof) external {
        delete _resBatch;
        euint64 x = FHE.fromExternal(a, inputProof);
        euint64 y = FHE.fromExternal(e, inputProof);
        _push(FHE.add(x, s));
        _push(FHE.sub(x, s));
        _push(FHE.mul(x, s));
        _push(FHE.add(x, y));
        _push(FHE.sub(x, y));
        _push(FHE.mul(x, y));
        _push(FHE.neg(x));
        _push(FHE.not(x));
    }

    function test_arith_euint128(
        externalEuint128 a,
        uint128 s,
        externalEuint128 e,
        bytes calldata inputProof
    ) external {
        delete _resBatch;
        euint128 x = FHE.fromExternal(a, inputProof);
        euint128 y = FHE.fromExternal(e, inputProof);
        _push(FHE.add(x, s));
        _push(FHE.sub(x, s));
        _push(FHE.mul(x, s));
        _push(FHE.add(x, y));
        _push(FHE.sub(x, y));
        _push(FHE.mul(x, y));
        _push(FHE.neg(x));
        _push(FHE.not(x));
    }

    /// @dev add/sub/mul stop at euint128; neg/not go up to euint256.
    function test_negnot_euint256(externalEuint256 a, bytes calldata inputProof) external {
        delete _resBatch;
        euint256 x = FHE.fromExternal(a, inputProof);
        _push(FHE.neg(x));
        _push(FHE.not(x));
    }

    /// @dev Narrowing casts, widest target first.
    function test_narrow_euint16(externalEuint16 a, bytes calldata inputProof) external {
        delete _resBatch;
        euint16 x = FHE.fromExternal(a, inputProof);
        _push(FHE.asEuint8(x));
    }

    function test_narrow_euint32(externalEuint32 a, bytes calldata inputProof) external {
        delete _resBatch;
        euint32 x = FHE.fromExternal(a, inputProof);
        _push(FHE.asEuint16(x));
        _push(FHE.asEuint8(x));
    }

    function test_narrow_euint64(externalEuint64 a, bytes calldata inputProof) external {
        delete _resBatch;
        euint64 x = FHE.fromExternal(a, inputProof);
        _push(FHE.asEuint32(x));
        _push(FHE.asEuint16(x));
        _push(FHE.asEuint8(x));
    }

    function test_narrow_euint128(externalEuint128 a, bytes calldata inputProof) external {
        delete _resBatch;
        euint128 x = FHE.fromExternal(a, inputProof);
        _push(FHE.asEuint64(x));
        _push(FHE.asEuint32(x));
        _push(FHE.asEuint16(x));
        _push(FHE.asEuint8(x));
    }

    function test_narrow_euint256(externalEuint256 a, bytes calldata inputProof) external {
        delete _resBatch;
        euint256 x = FHE.fromExternal(a, inputProof);
        _push(FHE.asEuint128(x));
        _push(FHE.asEuint64(x));
        _push(FHE.asEuint32(x));
        _push(FHE.asEuint16(x));
        _push(FHE.asEuint8(x));
    }
}
