// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

contract FHEVMTestSuite7 is E2ECoprocessorConfig {
    ebool public resEbool;
    euint8 public resEuint8;
    euint16 public resEuint16;
    euint32 public resEuint32;
    euint64 public resEuint64;
    euint128 public resEuint128;
    euint256 public resEuint256;

    function or_euint16_euint256(externalEuint16 a, externalEuint256 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function min_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function le_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function xor_euint128_euint256(externalEuint128 a, externalEuint256 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function xor_euint16_euint256(externalEuint16 a, externalEuint256 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function mul_euint16_euint128(externalEuint16 a, externalEuint128 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.mul(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function min_euint16_euint128(externalEuint16 a, externalEuint128 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function sub_euint32_euint8(externalEuint32 a, externalEuint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.sub(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function eq_euint256_euint256(externalEuint256 a, externalEuint256 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function gt_euint16_euint64(externalEuint16 a, externalEuint64 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function le_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint16 bProc = b;
        ebool result = FHE.le(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function sub_euint16_euint128(externalEuint16 a, externalEuint128 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.sub(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function ne_euint32_euint8(externalEuint32 a, externalEuint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function eq_euint32_euint8(externalEuint32 a, externalEuint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function xor_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function lt_euint16_euint64(externalEuint16 a, externalEuint64 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function le_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function min_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint16 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function lt_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint16 bProc = b;
        ebool result = FHE.lt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function xor_euint32_euint128(externalEuint32 a, externalEuint128 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function add_euint64_euint16(externalEuint64 a, externalEuint16 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.add(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function mul_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.mul(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function rotl_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.rotl(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function add_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.add(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function and_euint16_euint128(externalEuint16 a, externalEuint128 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function max_euint32_euint128(externalEuint32 a, externalEuint128 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.max(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function gt_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function and_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function max_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.max(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function xor_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function sub_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.sub(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function div_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = FHE.div(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function and_euint8_euint16(externalEuint8 a, externalEuint16 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint16 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function max_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = FHE.max(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function and_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function ne_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function neg_euint128(externalEuint128 a, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint128 result = FHE.neg(aProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function max_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.max(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function ge_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function and_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function rotl_euint128_uint8(externalEuint128 a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint128 result = FHE.rotl(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function min_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint8 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function add_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint16 result = FHE.add(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function ne_euint256_euint256(externalEuint256 a, externalEuint256 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function ge_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function add_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = FHE.add(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function gt_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint128 bProc = b;
        ebool result = FHE.gt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function lt_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function max_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.max(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function eq_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        uint64 bProc = b;
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function eq_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint16 bProc = b;
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function or_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function or_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint16 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function sub_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.sub(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function neg_euint32(externalEuint32 a, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint32 result = FHE.neg(aProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function mul_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = FHE.mul(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function sub_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.sub(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function rotl_euint256_uint8(externalEuint256 a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint256 result = FHE.rotl(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function eq_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function sub_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = FHE.sub(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function or_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function neg_euint16(externalEuint16 a, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint16 result = FHE.neg(aProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function or_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint8 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function div_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = FHE.div(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function eq_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function min_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function xor_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function ge_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint16 bProc = b;
        ebool result = FHE.ge(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function max_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.max(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function shl_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.shl(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function gt_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function xor_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function add_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint8 result = FHE.add(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function min_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function mul_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = FHE.mul(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function or_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function ge_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function not_euint8(externalEuint8 a, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint8 result = FHE.not(aProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function le_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        ebool result = FHE.le(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function lt_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        ebool result = FHE.lt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function ge_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function ne_euint128_euint256(externalEuint128 a, externalEuint256 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function neg_euint256(externalEuint256 a, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint256 result = FHE.neg(aProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function shl_euint16_uint8(externalEuint16 a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint16 result = FHE.shl(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function le_euint16_euint128(externalEuint16 a, externalEuint128 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
}
