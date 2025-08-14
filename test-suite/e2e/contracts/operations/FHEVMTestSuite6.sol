// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

contract FHEVMTestSuite6 is E2ECoprocessorConfig {
    ebool public resEbool;
    euint8 public resEuint8;
    euint16 public resEuint16;
    euint32 public resEuint32;
    euint64 public resEuint64;
    euint128 public resEuint128;
    euint256 public resEuint256;

    function ne_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function max_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = FHE.max(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function and_euint32_euint256(externalEuint32 a, externalEuint256 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function le_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function shr_euint256_uint8(externalEuint256 a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint256 result = FHE.shr(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function eq_euint256_euint128(externalEuint256 a, externalEuint128 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function ne_euint64_euint256(externalEuint64 a, externalEuint256 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function xor_euint8_euint128(externalEuint8 a, externalEuint128 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function and_euint8_euint256(externalEuint8 a, externalEuint256 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function or_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function or_euint256_euint32(externalEuint256 a, externalEuint32 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function xor_uint256_euint256(uint256 a, externalEuint256 b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function eq_euint8_euint128(externalEuint8 a, externalEuint128 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function shr_euint16_euint8(externalEuint16 a, externalEuint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint16 result = FHE.shr(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function le_euint8_euint16(externalEuint8 a, externalEuint16 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function max_euint64_euint32(externalEuint64 a, externalEuint32 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.max(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function eq_euint64_euint16(externalEuint64 a, externalEuint16 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function and_euint16_euint64(externalEuint16 a, externalEuint64 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function not_euint256(externalEuint256 a, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint256 result = FHE.not(aProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function lt_euint8_euint128(externalEuint8 a, externalEuint128 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function or_euint32_euint256(externalEuint32 a, externalEuint256 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function rotr_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.rotr(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function ge_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function le_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function rotr_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.rotr(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function add_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = FHE.add(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function min_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function lt_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function and_euint256_euint32(externalEuint256 a, externalEuint32 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function or_euint64_euint32(externalEuint64 a, externalEuint32 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function add_euint16_euint128(externalEuint16 a, externalEuint128 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.add(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function neg_euint8(externalEuint8 a, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint8 result = FHE.neg(aProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function ge_euint8_euint32(externalEuint8 a, externalEuint32 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function ne_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint128 bProc = b;
        ebool result = FHE.ne(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function eq_euint16_euint128(externalEuint16 a, externalEuint128 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function rotl_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = FHE.rotl(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint8 = result;
    }
    function gt_euint16_euint128(externalEuint16 a, externalEuint128 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function max_euint16_euint128(externalEuint16 a, externalEuint128 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.max(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function le_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        uint32 bProc = b;
        ebool result = FHE.le(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function mul_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = FHE.mul(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function add_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.add(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function xor_euint256_uint256(externalEuint256 a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint256 bProc = b;
        euint256 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function xor_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function gt_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function not_euint64(externalEuint64 a, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 result = FHE.not(aProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function xor_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function rotl_euint16_uint8(externalEuint16 a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint16 result = FHE.rotl(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function ne_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        ebool result = FHE.ne(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function sub_euint32_euint16(externalEuint32 a, externalEuint16 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.sub(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function and_euint16_euint256(externalEuint16 a, externalEuint256 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function ge_euint32_euint8(externalEuint32 a, externalEuint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function lt_euint32_euint32(externalEuint32 a, externalEuint32 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function sub_euint8_euint32(externalEuint8 a, externalEuint32 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.sub(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function min_euint64_euint32(externalEuint64 a, externalEuint32 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function xor_euint256_euint64(externalEuint256 a, externalEuint64 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function ne_euint64_euint16(externalEuint64 a, externalEuint16 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function and_euint256_euint64(externalEuint256 a, externalEuint64 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.and(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function min_euint32_euint32(externalEuint32 a, externalEuint32 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function rotr_euint128_uint8(externalEuint128 a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint128 result = FHE.rotr(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function eq_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function not_euint32(externalEuint32 a, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint32 result = FHE.not(aProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function le_euint16_euint64(externalEuint16 a, externalEuint64 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function add_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.add(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function eq_euint16_euint64(externalEuint16 a, externalEuint64 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function shl_euint32_euint8(externalEuint32 a, externalEuint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.shl(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function min_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function le_euint32_euint64(externalEuint32 a, externalEuint64 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function shr_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.shr(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function shl_euint16_euint8(externalEuint16 a, externalEuint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint16 result = FHE.shl(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function gt_euint32_euint128(externalEuint32 a, externalEuint128 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function xor_euint32_euint256(externalEuint32 a, externalEuint256 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function eq_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function shr_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.shr(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function ne_euint8_euint32(externalEuint8 a, externalEuint32 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function min_euint32_euint128(externalEuint32 a, externalEuint128 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function min_euint16_euint64(externalEuint16 a, externalEuint64 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.min(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function max_euint16_euint64(externalEuint16 a, externalEuint64 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.max(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function rotr_euint256_uint8(externalEuint256 a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint256 result = FHE.rotr(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint256 = result;
    }
    function mul_euint32_euint8(externalEuint32 a, externalEuint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.mul(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function or_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint64 = result;
    }
    function ge_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        uint64 bProc = b;
        ebool result = FHE.ge(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function add_euint32_euint8(externalEuint32 a, externalEuint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.add(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }
    function lt_euint16_euint128(externalEuint16 a, externalEuint128 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function or_euint32_euint128(externalEuint32 a, externalEuint128 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.or(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function ge_euint64_euint16(externalEuint64 a, externalEuint16 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function xor_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint16 result = FHE.xor(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint16 = result;
    }
    function mul_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.mul(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function add_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = FHE.add(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
    function ge_euint16_euint128(externalEuint16 a, externalEuint128 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }
    function not_euint128(externalEuint128 a, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint128 result = FHE.not(aProc);
        FHE.makePubliclyDecryptable(result);
        resEuint128 = result;
    }
}
