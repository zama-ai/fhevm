// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {
    FHE,
    ebool,
    euint8,
    euint16,
    euint32,
    euint64,
    euint128,
    euint256,
    eaddress,
    externalEbool,
    externalEuint32,
    externalEaddress
} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

contract FHEVMManualTestSuiteWithAllowSender is ZamaEthereumConfig {
    ebool public resEbool;
    euint8 public resEuint8;
    euint16 public resEuint16;
    euint32 public resEuint32;
    euint64 public resEuint64;
    euint128 public resEuint128;
    euint256 public resEuint256;
    eaddress public resAdd;

    constructor() {}

    function eqEbool(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.eq(input1, input2);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function eqEboolScalarL(bool a, bool b) external {
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.eq(a, input2);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function eqEboolScalarR(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool result = FHE.eq(input1, b);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function neEbool(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.ne(input1, input2);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function neEboolScalarL(bool a, bool b) external {
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.ne(a, input2);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function neEboolScalarR(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool result = FHE.ne(input1, b);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function test_select_ebool(bool control, bool ifTrue, bool ifFalse) public {
        ebool controlProc = FHE.asEbool(control);
        ebool ifTrueProc = FHE.asEbool(ifTrue);
        ebool ifFalseProc = FHE.asEbool(ifFalse);
        ebool result = FHE.select(controlProc, ifTrueProc, ifFalseProc);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
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
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
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
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resAdd = result;
    }

    function test_eq_eaddress_eaddress(externalEaddress a, externalEaddress b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        eaddress bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function test_ne_eaddress_eaddress(externalEaddress a, externalEaddress b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        eaddress bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function test_eq_eaddress_address(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.eq(aProc, b);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function test_eq_address_eaddress(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.eq(b, aProc);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function test_ne_eaddress_address(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.ne(aProc, b);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function test_ne_address_eaddress(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.ne(b, aProc);
        FHE.allowThis(result);
        FHE.allow(result, msg.sender);
        resEbool = result;
    }

    function test_ebool_to_euint8_cast(bool input) public {
        resEuint8 = FHE.asEuint8(FHE.asEbool(input));
        FHE.allowThis(resEuint8);
        FHE.allow(resEuint8, msg.sender);
    }

    function test_ebool_to_euint16_cast(bool input) public {
        resEuint16 = FHE.asEuint16(FHE.asEbool(input));
        FHE.allowThis(resEuint16);
        FHE.allow(resEuint16, msg.sender);
    }

    function test_ebool_to_euint32_cast(bool input) public {
        resEuint32 = FHE.asEuint32(FHE.asEbool(input));
        FHE.allowThis(resEuint32);
        FHE.allow(resEuint32, msg.sender);
    }

    function test_ebool_to_euint64_cast(bool input) public {
        resEuint64 = FHE.asEuint64(FHE.asEbool(input));
        FHE.allowThis(resEuint64);
        FHE.allow(resEuint64, msg.sender);
    }

    function test_ebool_to_euint128_cast(bool input) public {
        resEuint128 = FHE.asEuint128(FHE.asEbool(input));
        FHE.allowThis(resEuint128);
        FHE.allow(resEuint128, msg.sender);
    }

    function test_ebool_to_euint256_cast(bool input) public {
        resEuint256 = FHE.asEuint256(FHE.asEbool(input));
        FHE.allowThis(resEuint256);
        FHE.allow(resEuint256, msg.sender);
    }

    function test_euint128_to_euint8_cast(uint128 input) public {
        resEuint8 = FHE.asEuint8(FHE.asEuint128(input));
        FHE.allowThis(resEuint8);
        FHE.allow(resEuint8, msg.sender);
    }

    function test_ebool_not(bool input) public {
        resEbool = FHE.not(FHE.asEbool(input));
        FHE.allowThis(resEbool);
        FHE.allow(resEbool, msg.sender);
    }

    function test_ebool_and(bool a, bool b) public {
        resEbool = FHE.and(FHE.asEbool(a), FHE.asEbool(b));
        FHE.allowThis(resEbool);
        FHE.allow(resEbool, msg.sender);
    }

    function test_ebool_and_scalarL(bool a, bool b) public {
        resEbool = FHE.and(a, FHE.asEbool(b));
        FHE.allowThis(resEbool);
        FHE.allow(resEbool, msg.sender);
    }

    function test_ebool_and_scalarR(bool a, bool b) public {
        resEbool = FHE.and(FHE.asEbool(a), b);
        FHE.allowThis(resEbool);
        FHE.allow(resEbool, msg.sender);
    }

    function test_ebool_or(bool a, bool b) public {
        resEbool = FHE.or(FHE.asEbool(a), FHE.asEbool(b));
        FHE.allowThis(resEbool);
        FHE.allow(resEbool, msg.sender);
    }

    function test_ebool_or_scalarL(bool a, bool b) public {
        resEbool = FHE.or(a, FHE.asEbool(b));
        FHE.allowThis(resEbool);
        FHE.allow(resEbool, msg.sender);
    }

    function test_ebool_or_scalarR(bool a, bool b) public {
        resEbool = FHE.or(FHE.asEbool(a), b);
        FHE.allowThis(resEbool);
        FHE.allow(resEbool, msg.sender);
    }

    function test_ebool_xor(bool a, bool b) public {
        resEbool = FHE.xor(FHE.asEbool(a), FHE.asEbool(b));
        FHE.allowThis(resEbool);
        FHE.allow(resEbool, msg.sender);
    }

    function test_ebool_xor_scalarL(bool a, bool b) public {
        resEbool = FHE.xor(a, FHE.asEbool(b));
        FHE.allowThis(resEbool);
        FHE.allow(resEbool, msg.sender);
    }

    function test_ebool_xor_scalarR(bool a, bool b) public {
        resEbool = FHE.xor(FHE.asEbool(a), b);
        FHE.allowThis(resEbool);
        FHE.allow(resEbool, msg.sender);
    }
}
