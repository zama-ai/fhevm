// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

contract FHEVMManualTestSuite is E2ECoprocessorConfig {
    ebool public resEbool;
    euint8 public resEuint8;
    euint16 public resEuint16;
    euint32 public resEuint32;
    euint64 public resEuint64;
    euint128 public resEuint128;
    euint256 public resEuint256;
    eaddress public resAdd;

    function eqEbool(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.eq(input1, input2);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function eqEboolScalarL(bool a, bool b) external {
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.eq(a, input2);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function eqEboolScalarR(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool result = FHE.eq(input1, b);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function neEbool(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.ne(input1, input2);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function neEboolScalarL(bool a, bool b) external {
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.ne(a, input2);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function neEboolScalarR(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool result = FHE.ne(input1, b);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function test_select_ebool(bool control, bool ifTrue, bool ifFalse) public {
        ebool controlProc = FHE.asEbool(control);
        ebool ifTrueProc = FHE.asEbool(ifTrue);
        ebool ifFalseProc = FHE.asEbool(ifFalse);
        ebool result = FHE.select(controlProc, ifTrueProc, ifFalseProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function test_select(
        externalEbool control,
        externalEuint32 ifTrue,
        externalEuint32 ifFalse,
        bytes calldata inputProof
    ) public {
        ebool controlProc = FHE.fromExternal(control, inputProof);
        euint32 ifTrueProc = FHE.fromExternal(ifTrue, inputProof);
        euint32 ifFalseProc = FHE.fromExternal(ifFalse, inputProof);
        euint32 result = FHE.select(controlProc, ifTrueProc, ifFalseProc);
        FHE.makePubliclyDecryptable(result);
        resEuint32 = result;
    }

    function test_select_eaddress(
        externalEbool control,
        externalEaddress ifTrue,
        externalEaddress ifFalse,
        bytes calldata inputProof
    ) public {
        ebool controlProc = FHE.fromExternal(control, inputProof);
        eaddress ifTrueProc = FHE.fromExternal(ifTrue, inputProof);
        eaddress ifFalseProc = FHE.fromExternal(ifFalse, inputProof);
        eaddress result = FHE.select(controlProc, ifTrueProc, ifFalseProc);
        FHE.makePubliclyDecryptable(result);
        resAdd = result;
    }

    function test_eq_eaddress_eaddress(externalEaddress a, externalEaddress b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        eaddress bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function test_ne_eaddress_eaddress(externalEaddress a, externalEaddress b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        eaddress bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function test_eq_eaddress_address(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.eq(aProc, b);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function test_eq_address_eaddress(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.eq(b, aProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function test_ne_eaddress_address(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.ne(aProc, b);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function test_ne_address_eaddress(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.ne(b, aProc);
        FHE.makePubliclyDecryptable(result);
        resEbool = result;
    }

    function test_ebool_to_euint8_cast(bool input) public {
        resEuint8 = FHE.asEuint8(FHE.asEbool(input));
        FHE.makePubliclyDecryptable(resEuint8);
    }

    function test_ebool_to_euint16_cast(bool input) public {
        resEuint16 = FHE.asEuint16(FHE.asEbool(input));
        FHE.makePubliclyDecryptable(resEuint16);
    }

    function test_ebool_to_euint32_cast(bool input) public {
        resEuint32 = FHE.asEuint32(FHE.asEbool(input));
        FHE.makePubliclyDecryptable(resEuint32);
    }

    function test_ebool_to_euint64_cast(bool input) public {
        resEuint64 = FHE.asEuint64(FHE.asEbool(input));
        FHE.makePubliclyDecryptable(resEuint64);
    }

    function test_ebool_to_euint128_cast(bool input) public {
        resEuint128 = FHE.asEuint128(FHE.asEbool(input));
        FHE.makePubliclyDecryptable(resEuint128);
    }

    function test_ebool_to_euint256_cast(bool input) public {
        resEuint256 = FHE.asEuint256(FHE.asEbool(input));
        FHE.makePubliclyDecryptable(resEuint256);
    }

    function test_euint128_to_euint8_cast(uint128 input) public {
        resEuint8 = FHE.asEuint8(FHE.asEuint128(input));
        FHE.makePubliclyDecryptable(resEuint8);
    }

    function test_ebool_not(bool input) public {
        resEbool = FHE.not(FHE.asEbool(input));
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_ebool_and(bool a, bool b) public {
        resEbool = FHE.and(FHE.asEbool(a), FHE.asEbool(b));
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_ebool_and_scalarL(bool a, bool b) public {
        resEbool = FHE.and(a, FHE.asEbool(b));
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_ebool_and_scalarR(bool a, bool b) public {
        resEbool = FHE.and(FHE.asEbool(a), b);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_ebool_or(bool a, bool b) public {
        resEbool = FHE.or(FHE.asEbool(a), FHE.asEbool(b));
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_ebool_or_scalarL(bool a, bool b) public {
        resEbool = FHE.or(a, FHE.asEbool(b));
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_ebool_or_scalarR(bool a, bool b) public {
        resEbool = FHE.or(FHE.asEbool(a), b);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_ebool_xor(bool a, bool b) public {
        resEbool = FHE.xor(FHE.asEbool(a), FHE.asEbool(b));
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_ebool_xor_scalarL(bool a, bool b) public {
        resEbool = FHE.xor(a, FHE.asEbool(b));
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_ebool_xor_scalarR(bool a, bool b) public {
        resEbool = FHE.xor(FHE.asEbool(a), b);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_shr_euint64_uint8(externalEuint64 a, uint8 b, bytes calldata inputProof) external {
        resEuint64 = FHE.shr(FHE.fromExternal(a, inputProof), b);
        FHE.makePubliclyDecryptable(resEuint64);
    }

    function test_shr_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) external {
        resEuint64 = FHE.shr(FHE.fromExternal(a, inputProof), FHE.fromExternal(b, inputProof));
        FHE.makePubliclyDecryptable(resEuint64);
    }

    function test_shl_euint64_uint8(externalEuint64 a, uint8 b, bytes calldata inputProof) external {
        resEuint64 = FHE.shl(FHE.fromExternal(a, inputProof), b);
        FHE.makePubliclyDecryptable(resEuint64);
    }

    function test_shl_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) external {
        resEuint64 = FHE.shl(FHE.fromExternal(a, inputProof), FHE.fromExternal(b, inputProof));
        FHE.makePubliclyDecryptable(resEuint64);
    }

    function test_rotl_euint64_uint8(externalEuint64 a, uint8 b, bytes calldata inputProof) external {
        resEuint64 = FHE.rotl(FHE.fromExternal(a, inputProof), b);
        FHE.makePubliclyDecryptable(resEuint64);
    }

    function test_rotl_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) external {
        resEuint64 = FHE.rotl(FHE.fromExternal(a, inputProof), FHE.fromExternal(b, inputProof));
        FHE.makePubliclyDecryptable(resEuint64);
    }

    function test_rotr_euint64_uint8(externalEuint64 a, uint8 b, bytes calldata inputProof) external {
        resEuint64 = FHE.rotr(FHE.fromExternal(a, inputProof), b);
        FHE.makePubliclyDecryptable(resEuint64);
    }

    function test_rotr_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) external {
        resEuint64 = FHE.rotr(FHE.fromExternal(a, inputProof), FHE.fromExternal(b, inputProof));
        FHE.makePubliclyDecryptable(resEuint64);
    }

    function test_sum_euint8(
        externalEuint8 a,
        externalEuint8 b,
        externalEuint8 c,
        bytes calldata inputProof
    ) external {
        euint8[] memory values = new euint8[](3);
        values[0] = FHE.fromExternal(a, inputProof);
        values[1] = FHE.fromExternal(b, inputProof);
        values[2] = FHE.fromExternal(c, inputProof);
        resEuint8 = FHE.sum(values);
        FHE.makePubliclyDecryptable(resEuint8);
    }

    function test_sum_euint16(externalEuint16 a, externalEuint16 b, bytes calldata inputProof) external {
        euint16[] memory values = new euint16[](2);
        values[0] = FHE.fromExternal(a, inputProof);
        values[1] = FHE.fromExternal(b, inputProof);
        resEuint16 = FHE.sum(values);
        FHE.makePubliclyDecryptable(resEuint16);
    }

    function test_sum_euint32(externalEuint32 a, externalEuint32 b, bytes calldata inputProof) external {
        euint32[] memory values = new euint32[](2);
        values[0] = FHE.fromExternal(a, inputProof);
        values[1] = FHE.fromExternal(b, inputProof);
        resEuint32 = FHE.sum(values);
        FHE.makePubliclyDecryptable(resEuint32);
    }

    function test_sum_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) external {
        euint64[] memory values = new euint64[](2);
        values[0] = FHE.fromExternal(a, inputProof);
        values[1] = FHE.fromExternal(b, inputProof);
        resEuint64 = FHE.sum(values);
        FHE.makePubliclyDecryptable(resEuint64);
    }

    function test_sum_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) external {
        euint128[] memory values = new euint128[](2);
        values[0] = FHE.fromExternal(a, inputProof);
        values[1] = FHE.fromExternal(b, inputProof);
        resEuint128 = FHE.sum(values);
        FHE.makePubliclyDecryptable(resEuint128);
    }

    function test_sum_euint8_duplicate(externalEuint8 a, bytes calldata inputProof) external {
        euint8 v = FHE.fromExternal(a, inputProof);
        euint8[] memory values = new euint8[](2);
        values[0] = v;
        values[1] = v;
        resEuint8 = FHE.sum(values);
        FHE.makePubliclyDecryptable(resEuint8);
    }

    function test_sum_euint8_uninitialized() external {
        euint8 uninit_;
        euint8[] memory values = new euint8[](2);
        values[0] = FHE.asEuint8(5);
        values[1] = uninit_;
        resEuint8 = FHE.sum(values);
        FHE.makePubliclyDecryptable(resEuint8);
    }

    function test_sum_euint8_empty() external {
        euint8[] memory values = new euint8[](0);
        resEuint8 = FHE.sum(values);
        FHE.makePubliclyDecryptable(resEuint8);
    }

    function test_sum_euint8_single(externalEuint8 a, bytes calldata inputProof) external {
        euint8[] memory values = new euint8[](1);
        values[0] = FHE.fromExternal(a, inputProof);
        resEuint8 = FHE.sum(values);
        FHE.makePubliclyDecryptable(resEuint8);
    }

    function test_sum_euint8_max_array() external {
        euint8[] memory values = new euint8[](100);
        for (uint256 i = 0; i < 100; i++) {
            values[i] = FHE.asEuint8(1);
        }
        resEuint8 = FHE.sum(values);
        FHE.makePubliclyDecryptable(resEuint8);
    }

    function test_isIn_euint8_found(externalEuint8 a, bytes calldata inputProof) external {
        euint8 value = FHE.fromExternal(a, inputProof);
        euint8[] memory set = new euint8[](3);
        set[0] = FHE.asEuint8(10);
        set[1] = FHE.asEuint8(20);
        set[2] = FHE.asEuint8(30);
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint8_not_found(externalEuint8 a, bytes calldata inputProof) external {
        euint8 value = FHE.fromExternal(a, inputProof);
        euint8[] memory set = new euint8[](3);
        set[0] = FHE.asEuint8(10);
        set[1] = FHE.asEuint8(20);
        set[2] = FHE.asEuint8(30);
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint16(externalEuint16 a, bytes calldata inputProof) external {
        euint16 value = FHE.fromExternal(a, inputProof);
        euint16[] memory set = new euint16[](2);
        set[0] = FHE.asEuint16(1000);
        set[1] = FHE.asEuint16(2000);
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint32(externalEuint32 a, bytes calldata inputProof) external {
        euint32 value = FHE.fromExternal(a, inputProof);
        euint32[] memory set = new euint32[](2);
        set[0] = FHE.asEuint32(100000);
        set[1] = FHE.asEuint32(200000);
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint64(externalEuint64 a, bytes calldata inputProof) external {
        euint64 value = FHE.fromExternal(a, inputProof);
        euint64[] memory set = new euint64[](2);
        set[0] = FHE.asEuint64(1000000000);
        set[1] = FHE.asEuint64(2000000000);
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint128(externalEuint128 a, bytes calldata inputProof) external {
        euint128 value = FHE.fromExternal(a, inputProof);
        euint128[] memory set = new euint128[](2);
        set[0] = FHE.asEuint128(10000000000000000000);
        set[1] = FHE.asEuint128(20000000000000000000);
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint8_uninitialized() external {
        euint8 uninit_;
        euint8[] memory set = new euint8[](2);
        set[0] = FHE.asEuint8(0);
        set[1] = FHE.asEuint8(1);
        resEbool = FHE.isIn(uninit_, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint8_single_element(externalEuint8 a, bytes calldata inputProof) external {
        euint8 value = FHE.fromExternal(a, inputProof);
        euint8[] memory set = new euint8[](1);
        set[0] = FHE.asEuint8(42);
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint8_max_array() external {
        euint8 value = FHE.asEuint8(50);
        euint8[] memory set = new euint8[](100);
        for (uint256 i = 0; i < 100; i++) {
            set[i] = FHE.asEuint8(uint8(i));
        }
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint8_empty_set() external {
        euint8 value = FHE.asEuint8(42);
        euint8[] memory set = new euint8[](0);
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint8_zero_initialized_set() external {
        euint8 value = FHE.asEuint8(0);
        euint8[] memory set = new euint8[](3);
        set[0] = FHE.asEuint8(0);
        set[1] = FHE.asEuint8(0);
        set[2] = FHE.asEuint8(0);
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint8_max_value_found() external {
        euint8 value = FHE.asEuint8(255);
        euint8[] memory set = new euint8[](3);
        set[0] = FHE.asEuint8(0);
        set[1] = FHE.asEuint8(128);
        set[2] = FHE.asEuint8(255);
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }

    function test_isIn_euint8_single_element_not_found() external {
        euint8 value = FHE.asEuint8(99);
        euint8[] memory set = new euint8[](1);
        set[0] = FHE.asEuint8(42);
        resEbool = FHE.isIn(value, set);
        FHE.makePubliclyDecryptable(resEbool);
    }
}
