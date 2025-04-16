// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/HTTPZ.sol";
import "../../lib/HTTPZConfig.sol";

contract TFHEManualTestSuite {
    ebool public resEbool;
    euint8 public resEuint8;
    euint16 public resEuint16;
    euint32 public resEuint32;
    euint64 public resEuint64;
    euint128 public resEuint128;
    euint256 public resEuint256;
    eaddress public resAdd;
    ebytes64 public resEbytes64;
    ebytes128 public resEbytes128;
    ebytes256 public resEbytes256;

    constructor() {
        HTTPZ.setCoprocessor(HTTPZConfig.defaultConfig()); // Set up the FHEVM configuration for this contract
    }

    function eqEbool(bool a, bool b) external {
        ebool input1 = HTTPZ.asEbool(a);
        ebool input2 = HTTPZ.asEbool(b);
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function eqEboolScalarL(bool a, bool b) external {
        ebool input2 = HTTPZ.asEbool(b);
        ebool result = HTTPZ.eq(a, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function eqEboolScalarR(bool a, bool b) external {
        ebool input1 = HTTPZ.asEbool(a);
        ebool result = HTTPZ.eq(input1, b);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEbool(bool a, bool b) external {
        ebool input1 = HTTPZ.asEbool(a);
        ebool input2 = HTTPZ.asEbool(b);
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEboolScalarL(bool a, bool b) external {
        ebool input2 = HTTPZ.asEbool(b);
        ebool result = HTTPZ.ne(a, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEboolScalarR(bool a, bool b) external {
        ebool input1 = HTTPZ.asEbool(a);
        ebool result = HTTPZ.ne(input1, b);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function eqEbytes256(
        externalEbytes256 inp1,
        bytes calldata inputProof1,
        externalEbytes256 inp2,
        bytes calldata inputProof2
    ) external {
        ebytes256 input1 = HTTPZ.fromExternal(inp1, inputProof1);
        ebytes256 input2 = HTTPZ.fromExternal(inp2, inputProof2);
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEbytes256(
        externalEbytes256 inp1,
        bytes calldata inputProof1,
        externalEbytes256 inp2,
        bytes calldata inputProof2
    ) external {
        ebytes256 input1 = HTTPZ.fromExternal(inp1, inputProof1);
        ebytes256 input2 = HTTPZ.fromExternal(inp2, inputProof2);
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function eqEbytes64(bytes memory a, bytes memory b) external {
        ebytes64 input1 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(a));
        ebytes64 input2 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(b));
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function eqEbytes64ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes64(a);
        ebytes64 input2 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(b));
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function eqEbytes64ScalarR(bytes memory a, bytes memory b) external {
        ebytes64 input1 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(a));
        bytes memory input2 = HTTPZ.padToBytes64(b);
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEbytes64(bytes memory a, bytes memory b) external {
        ebytes64 input1 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(a));
        ebytes64 input2 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(b));
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEbytes64ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes64(a);
        ebytes64 input2 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(b));
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEbytes64ScalarR(bytes memory a, bytes memory b) external {
        ebytes64 input1 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(a));
        bytes memory input2 = HTTPZ.padToBytes64(b);
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function eqEbytes128(bytes memory a, bytes memory b) external {
        ebytes128 input1 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(a));
        ebytes128 input2 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(b));
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function eqEbytes128ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes128(a);
        ebytes128 input2 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(b));
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function eqEbytes128ScalarR(bytes memory a, bytes memory b) external {
        ebytes128 input1 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(a));
        bytes memory input2 = HTTPZ.padToBytes128(b);
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEbytes128(bytes memory a, bytes memory b) external {
        ebytes128 input1 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(a));
        ebytes128 input2 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(b));
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEbytes128ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes128(a);
        ebytes128 input2 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(b));
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEbytes128ScalarR(bytes memory a, bytes memory b) external {
        ebytes128 input1 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(a));
        bytes memory input2 = HTTPZ.padToBytes128(b);
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function eqEbytes256ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes256(a);
        ebytes256 input2 = HTTPZ.asEbytes256(HTTPZ.padToBytes256(b));
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function eqEbytes256ScalarR(bytes memory a, bytes memory b) external {
        ebytes256 input1 = HTTPZ.asEbytes256(HTTPZ.padToBytes256(a));
        bytes memory input2 = HTTPZ.padToBytes256(b);
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEbytes256ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes256(a);
        ebytes256 input2 = HTTPZ.asEbytes256(HTTPZ.padToBytes256(b));
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function neEbytes256ScalarR(bytes memory a, bytes memory b) external {
        ebytes256 input1 = HTTPZ.asEbytes256(HTTPZ.padToBytes256(a));
        bytes memory input2 = HTTPZ.padToBytes256(b);
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function test_select_ebool(bool control, bool ifTrue, bool ifFalse) public {
        ebool controlProc = HTTPZ.asEbool(control);
        ebool ifTrueProc = HTTPZ.asEbool(ifTrue);
        ebool ifFalseProc = HTTPZ.asEbool(ifFalse);
        ebool result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function test_select_ebytes64(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = HTTPZ.asEbool(control);
        ebytes64 ifTrueProc = HTTPZ.asEbytes64(HTTPZ.padToBytes64(ifTrue));
        ebytes64 ifFalseProc = HTTPZ.asEbytes64(HTTPZ.padToBytes64(ifFalse));
        ebytes64 result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        resEbytes64 = result;
    }

    function test_select_ebytes128(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = HTTPZ.asEbool(control);
        ebytes128 ifTrueProc = HTTPZ.asEbytes128(HTTPZ.padToBytes128(ifTrue));
        ebytes128 ifFalseProc = HTTPZ.asEbytes128(HTTPZ.padToBytes128(ifFalse));
        ebytes128 result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        resEbytes128 = result;
    }

    function test_select_ebytes256(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = HTTPZ.asEbool(control);
        ebytes256 ifTrueProc = HTTPZ.asEbytes256(HTTPZ.padToBytes256(ifTrue));
        ebytes256 ifFalseProc = HTTPZ.asEbytes256(HTTPZ.padToBytes256(ifFalse));
        ebytes256 result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        resEbytes256 = result;
    }

    function test_select(
        externalEbool control,
        externalEuint32 ifTrue,
        externalEuint32 ifFalse,
        bytes calldata inputProof
    ) public {
        ebool controlProc = HTTPZ.fromExternal(control, inputProof);
        euint32 ifTrueProc = HTTPZ.fromExternal(ifTrue, inputProof);
        euint32 ifFalseProc = HTTPZ.fromExternal(ifFalse, inputProof);
        euint32 result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }

    function test_select_eaddress(
        externalEbool control,
        externalEaddress ifTrue,
        externalEaddress ifFalse,
        bytes calldata inputProof
    ) public {
        ebool controlProc = HTTPZ.fromExternal(control, inputProof);
        eaddress ifTrueProc = HTTPZ.fromExternal(ifTrue, inputProof);
        eaddress ifFalseProc = HTTPZ.fromExternal(ifFalse, inputProof);
        eaddress result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        resAdd = result;
    }

    function test_eq_eaddress_eaddress(externalEaddress a, externalEaddress b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.fromExternal(a, inputProof);
        eaddress bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function test_ne_eaddress_eaddress(externalEaddress a, externalEaddress b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.fromExternal(a, inputProof);
        eaddress bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function test_eq_eaddress_address(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.fromExternal(a, inputProof);
        ebool result = HTTPZ.eq(aProc, b);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function test_eq_address_eaddress(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.fromExternal(a, inputProof);
        ebool result = HTTPZ.eq(b, aProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function test_ne_eaddress_address(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.fromExternal(a, inputProof);
        ebool result = HTTPZ.ne(aProc, b);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function test_ne_address_eaddress(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.fromExternal(a, inputProof);
        ebool result = HTTPZ.ne(b, aProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }

    function test_ebool_to_euint8_cast(bool input) public {
        resEuint8 = HTTPZ.asEuint8(HTTPZ.asEbool(input));
    }

    function test_ebool_to_euint16_cast(bool input) public {
        resEuint16 = HTTPZ.asEuint16(HTTPZ.asEbool(input));
    }

    function test_ebool_to_euint32_cast(bool input) public {
        resEuint32 = HTTPZ.asEuint32(HTTPZ.asEbool(input));
    }

    function test_ebool_to_euint64_cast(bool input) public {
        resEuint64 = HTTPZ.asEuint64(HTTPZ.asEbool(input));
    }

    function test_ebool_to_euint128_cast(bool input) public {
        resEuint128 = HTTPZ.asEuint128(HTTPZ.asEbool(input));
    }

    function test_ebool_to_euint256_cast(bool input) public {
        resEuint256 = HTTPZ.asEuint256(HTTPZ.asEbool(input));
    }

    function test_euint128_to_euint8_cast(uint128 input) public {
        resEuint8 = HTTPZ.asEuint8(HTTPZ.asEuint128(input));
    }

    function test_ebool_not(bool input) public {
        resEbool = HTTPZ.not(HTTPZ.asEbool(input));
    }

    function test_ebool_and(bool a, bool b) public {
        resEbool = HTTPZ.and(HTTPZ.asEbool(a), HTTPZ.asEbool(b));
    }

    function test_ebool_and_scalarL(bool a, bool b) public {
        resEbool = HTTPZ.and(a, HTTPZ.asEbool(b));
    }

    function test_ebool_and_scalarR(bool a, bool b) public {
        resEbool = HTTPZ.and(HTTPZ.asEbool(a), b);
    }

    function test_ebool_or(bool a, bool b) public {
        resEbool = HTTPZ.or(HTTPZ.asEbool(a), HTTPZ.asEbool(b));
    }

    function test_ebool_or_scalarL(bool a, bool b) public {
        resEbool = HTTPZ.or(a, HTTPZ.asEbool(b));
    }

    function test_ebool_or_scalarR(bool a, bool b) public {
        resEbool = HTTPZ.or(HTTPZ.asEbool(a), b);
    }

    function test_ebool_xor(bool a, bool b) public {
        resEbool = HTTPZ.xor(HTTPZ.asEbool(a), HTTPZ.asEbool(b));
    }

    function test_ebool_xor_scalarL(bool a, bool b) public {
        resEbool = HTTPZ.xor(a, HTTPZ.asEbool(b));
    }

    function test_ebool_xor_scalarR(bool a, bool b) public {
        resEbool = HTTPZ.xor(HTTPZ.asEbool(a), b);
    }
}
