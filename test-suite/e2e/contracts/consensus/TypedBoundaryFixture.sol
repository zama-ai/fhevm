// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// Pairwise type/operation matrix: each graph is run with a locally produced
/// operand, then again with that same operand persisted by an earlier transaction.
contract TypedBoundaryFixture is E2ECoprocessorConfig {
    mapping(uint8 => bytes32[]) private outputs;
    function values(uint8 kind) external view returns(bytes32[] memory) { return outputs[kind]; }
    euint8 private seed8;
    function stage8(externalEuint8 input, bytes calldata proof) external {
        delete outputs[2];
        euint8 zero = FHE.asEuint8(0);
        _save(2, zero);
        seed8 = FHE.xor(FHE.fromExternal(input, proof), zero);
        _save(2, seed8);
        _graph8(seed8);
    }
    function consume8() external {
        delete outputs[2];
        _graph8(seed8);
    }
    function _graph8(euint8 value) private {
        euint8 wrap = FHE.add(value, 1);
        _save(2, wrap);
        euint8 one = FHE.asEuint8(1);
        _save(2, one);
        euint8 reverse = FHE.sub(one, value);
        _save(2, reverse);
        euint8 product = FHE.mul(value, 2);
        _save(2, product);
        euint8 quotient = FHE.div(value, 2);
        _save(2, quotient);
        euint8 remainder = FHE.rem(value, 2);
        _save(2, remainder);
        euint8 shift = FHE.shl(value, 1);
        _save(2, shift);
        euint8 rotate = FHE.rotl(value, 1);
        _save(2, rotate);
        euint8 bitwise = FHE.xor(value, 1);
        _save(2, bitwise);
        ebool predicate = FHE.eq(value, type(uint8).max);
        _save(2, predicate);
        euint8 selected = FHE.select(predicate, value, bitwise);
        _save(2, selected);
        _save(2, FHE.asEuint16(value));
    }
    euint16 private seed16;
    function stage16(externalEuint16 input, bytes calldata proof) external {
        delete outputs[3];
        euint16 zero = FHE.asEuint16(0);
        _save(3, zero);
        seed16 = FHE.xor(FHE.fromExternal(input, proof), zero);
        _save(3, seed16);
        _graph16(seed16);
    }
    function consume16() external {
        delete outputs[3];
        _graph16(seed16);
    }
    function _graph16(euint16 value) private {
        euint16 wrap = FHE.add(value, 1);
        _save(3, wrap);
        euint16 one = FHE.asEuint16(1);
        _save(3, one);
        euint16 reverse = FHE.sub(one, value);
        _save(3, reverse);
        euint16 product = FHE.mul(value, 2);
        _save(3, product);
        euint16 quotient = FHE.div(value, 2);
        _save(3, quotient);
        euint16 remainder = FHE.rem(value, 2);
        _save(3, remainder);
        euint16 shift = FHE.shl(value, 1);
        _save(3, shift);
        euint16 rotate = FHE.rotl(value, 1);
        _save(3, rotate);
        euint16 bitwise = FHE.xor(value, 1);
        _save(3, bitwise);
        ebool predicate = FHE.eq(value, type(uint16).max);
        _save(3, predicate);
        euint16 selected = FHE.select(predicate, value, bitwise);
        _save(3, selected);
        _save(3, FHE.asEuint8(value));
    }
    euint32 private seed32;
    function stage32(externalEuint32 input, bytes calldata proof) external {
        delete outputs[4];
        euint32 zero = FHE.asEuint32(0);
        _save(4, zero);
        seed32 = FHE.xor(FHE.fromExternal(input, proof), zero);
        _save(4, seed32);
        _graph32(seed32);
    }
    function consume32() external {
        delete outputs[4];
        _graph32(seed32);
    }
    function _graph32(euint32 value) private {
        euint32 wrap = FHE.add(value, 1);
        _save(4, wrap);
        euint32 one = FHE.asEuint32(1);
        _save(4, one);
        euint32 reverse = FHE.sub(one, value);
        _save(4, reverse);
        euint32 product = FHE.mul(value, 2);
        _save(4, product);
        euint32 quotient = FHE.div(value, 2);
        _save(4, quotient);
        euint32 remainder = FHE.rem(value, 2);
        _save(4, remainder);
        euint32 shift = FHE.shl(value, 1);
        _save(4, shift);
        euint32 rotate = FHE.rotl(value, 1);
        _save(4, rotate);
        euint32 bitwise = FHE.xor(value, 1);
        _save(4, bitwise);
        ebool predicate = FHE.eq(value, type(uint32).max);
        _save(4, predicate);
        euint32 selected = FHE.select(predicate, value, bitwise);
        _save(4, selected);
        _save(4, FHE.asEuint8(value));
    }
    euint64 private seed64;
    function stage64(externalEuint64 input, bytes calldata proof) external {
        delete outputs[5];
        euint64 zero = FHE.asEuint64(0);
        _save(5, zero);
        seed64 = FHE.xor(FHE.fromExternal(input, proof), zero);
        _save(5, seed64);
        _graph64(seed64);
    }
    function consume64() external {
        delete outputs[5];
        _graph64(seed64);
    }
    function _graph64(euint64 value) private {
        euint64 wrap = FHE.add(value, 1);
        _save(5, wrap);
        euint64 one = FHE.asEuint64(1);
        _save(5, one);
        euint64 reverse = FHE.sub(one, value);
        _save(5, reverse);
        euint64 product = FHE.mul(value, 2);
        _save(5, product);
        euint64 quotient = FHE.div(value, 2);
        _save(5, quotient);
        euint64 remainder = FHE.rem(value, 2);
        _save(5, remainder);
        euint64 shift = FHE.shl(value, 1);
        _save(5, shift);
        euint64 rotate = FHE.rotl(value, 1);
        _save(5, rotate);
        euint64 bitwise = FHE.xor(value, 1);
        _save(5, bitwise);
        ebool predicate = FHE.eq(value, type(uint64).max);
        _save(5, predicate);
        euint64 selected = FHE.select(predicate, value, bitwise);
        _save(5, selected);
        _save(5, FHE.asEuint8(value));
    }
    euint128 private seed128;
    function stage128(externalEuint128 input, bytes calldata proof) external {
        delete outputs[6];
        euint128 zero = FHE.asEuint128(0);
        _save(6, zero);
        seed128 = FHE.xor(FHE.fromExternal(input, proof), zero);
        _save(6, seed128);
        _graph128(seed128);
    }
    function consume128() external {
        delete outputs[6];
        _graph128(seed128);
    }
    function _graph128(euint128 value) private {
        euint128 wrap = FHE.add(value, 1);
        _save(6, wrap);
        euint128 one = FHE.asEuint128(1);
        _save(6, one);
        euint128 reverse = FHE.sub(one, value);
        _save(6, reverse);
        euint128 product = FHE.mul(value, 2);
        _save(6, product);
        euint128 quotient = FHE.div(value, 2);
        _save(6, quotient);
        euint128 remainder = FHE.rem(value, 2);
        _save(6, remainder);
        euint128 shift = FHE.shl(value, 1);
        _save(6, shift);
        euint128 rotate = FHE.rotl(value, 1);
        _save(6, rotate);
        euint128 bitwise = FHE.xor(value, 1);
        _save(6, bitwise);
        ebool predicate = FHE.eq(value, type(uint128).max);
        _save(6, predicate);
        euint128 selected = FHE.select(predicate, value, bitwise);
        _save(6, selected);
        _save(6, FHE.asEuint8(value));
    }
    euint256 private seed256;
    function stage256(externalEuint256 input, bytes calldata proof) external {
        delete outputs[8];
        euint256 zero = FHE.asEuint256(0);
        _save(8, zero);
        seed256 = FHE.xor(FHE.fromExternal(input, proof), zero);
        _save(8, seed256);
        _graph256(seed256);
    }
    function consume256() external {
        delete outputs[8];
        _graph256(seed256);
    }
    function _graph256(euint256 value) private {
        euint256 bitwise = FHE.xor(value, 1);
        _save(8, bitwise);
        ebool predicate = FHE.eq(value, type(uint256).max);
        _save(8, predicate);
        euint256 selected = FHE.select(predicate, value, bitwise);
        _save(8, selected);
        _save(8, FHE.asEuint8(value));
    }
    ebool private boolSeed;
    function stageBool(externalEbool input, bytes calldata proof) external {
        delete outputs[0];
        boolSeed = FHE.not(FHE.fromExternal(input, proof));
        _save(0, boolSeed);
        _boolGraph(boolSeed);
    }
    function consumeBool() external { delete outputs[0]; _boolGraph(boolSeed); }
    function _boolGraph(ebool value) private {
        ebool negated = FHE.not(value);
        _save(0, negated);
        _save(0, FHE.xor(value, negated));
        _save(0, FHE.and(value, negated));
    }
    function _save(uint8 kind, ebool value) private {
        outputs[kind].push(ebool.unwrap(value));
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }
    function _save(uint8 kind, euint8 value) private {
        outputs[kind].push(euint8.unwrap(value));
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }
    function _save(uint8 kind, euint16 value) private {
        outputs[kind].push(euint16.unwrap(value));
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }
    function _save(uint8 kind, euint32 value) private {
        outputs[kind].push(euint32.unwrap(value));
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }
    function _save(uint8 kind, euint64 value) private {
        outputs[kind].push(euint64.unwrap(value));
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }
    function _save(uint8 kind, euint128 value) private {
        outputs[kind].push(euint128.unwrap(value));
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }
    function _save(uint8 kind, euint256 value) private {
        outputs[kind].push(euint256.unwrap(value));
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }
}
