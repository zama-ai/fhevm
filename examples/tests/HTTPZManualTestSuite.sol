// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/HTTPZ.sol";
import "../FHEVMConfig.sol";

contract HTTPZManualTestSuite {
    ebool public resb;
    euint8 public res8;
    euint16 public res16;
    euint32 public res32;
    euint64 public res64;
    euint128 public res128;
    euint256 public res256;
    eaddress public resAdd;
    ebytes64 public resB64;
    ebytes128 public resB128;
    ebytes256 public resB256;

    constructor() {
        HTTPZ.setCoprocessor(FHEVMConfig.defaultConfig());
    }

    function eqEbool(bool a, bool b) external {
        ebool input1 = HTTPZ.asEbool(a);
        ebool input2 = HTTPZ.asEbool(b);
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function eqEboolScalarL(bool a, bool b) external {
        ebool input2 = HTTPZ.asEbool(b);
        ebool result = HTTPZ.eq(a, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function eqEboolScalarR(bool a, bool b) external {
        ebool input1 = HTTPZ.asEbool(a);
        ebool result = HTTPZ.eq(input1, b);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEbool(bool a, bool b) external {
        ebool input1 = HTTPZ.asEbool(a);
        ebool input2 = HTTPZ.asEbool(b);
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEboolScalarL(bool a, bool b) external {
        ebool input2 = HTTPZ.asEbool(b);
        ebool result = HTTPZ.ne(a, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEboolScalarR(bool a, bool b) external {
        ebool input1 = HTTPZ.asEbool(a);
        ebool result = HTTPZ.ne(input1, b);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function eqEbytes256(einput inp1, bytes calldata inputProof1, einput inp2, bytes calldata inputProof2) external {
        ebytes256 input1 = HTTPZ.asEbytes256(inp1, inputProof1);
        ebytes256 input2 = HTTPZ.asEbytes256(inp2, inputProof2);
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEbytes256(einput inp1, bytes calldata inputProof1, einput inp2, bytes calldata inputProof2) external {
        ebytes256 input1 = HTTPZ.asEbytes256(inp1, inputProof1);
        ebytes256 input2 = HTTPZ.asEbytes256(inp2, inputProof2);
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function eqEbytes64(bytes memory a, bytes memory b) external {
        ebytes64 input1 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(a));
        ebytes64 input2 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(b));
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function eqEbytes64ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes64(a);
        ebytes64 input2 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(b));
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function eqEbytes64ScalarR(bytes memory a, bytes memory b) external {
        ebytes64 input1 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(a));
        bytes memory input2 = HTTPZ.padToBytes64(b);
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEbytes64(bytes memory a, bytes memory b) external {
        ebytes64 input1 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(a));
        ebytes64 input2 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(b));
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEbytes64ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes64(a);
        ebytes64 input2 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(b));
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEbytes64ScalarR(bytes memory a, bytes memory b) external {
        ebytes64 input1 = HTTPZ.asEbytes64(HTTPZ.padToBytes64(a));
        bytes memory input2 = HTTPZ.padToBytes64(b);
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function eqEbytes128(bytes memory a, bytes memory b) external {
        ebytes128 input1 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(a));
        ebytes128 input2 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(b));
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function eqEbytes128ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes128(a);
        ebytes128 input2 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(b));
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function eqEbytes128ScalarR(bytes memory a, bytes memory b) external {
        ebytes128 input1 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(a));
        bytes memory input2 = HTTPZ.padToBytes128(b);
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEbytes128(bytes memory a, bytes memory b) external {
        ebytes128 input1 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(a));
        ebytes128 input2 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(b));
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEbytes128ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes128(a);
        ebytes128 input2 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(b));
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEbytes128ScalarR(bytes memory a, bytes memory b) external {
        ebytes128 input1 = HTTPZ.asEbytes128(HTTPZ.padToBytes128(a));
        bytes memory input2 = HTTPZ.padToBytes128(b);
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function eqEbytes256ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes256(a);
        ebytes256 input2 = HTTPZ.asEbytes256(HTTPZ.padToBytes256(b));
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function eqEbytes256ScalarR(bytes memory a, bytes memory b) external {
        ebytes256 input1 = HTTPZ.asEbytes256(HTTPZ.padToBytes256(a));
        bytes memory input2 = HTTPZ.padToBytes256(b);
        ebool result = HTTPZ.eq(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEbytes256ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = HTTPZ.padToBytes256(a);
        ebytes256 input2 = HTTPZ.asEbytes256(HTTPZ.padToBytes256(b));
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function neEbytes256ScalarR(bytes memory a, bytes memory b) external {
        ebytes256 input1 = HTTPZ.asEbytes256(HTTPZ.padToBytes256(a));
        bytes memory input2 = HTTPZ.padToBytes256(b);
        ebool result = HTTPZ.ne(input1, input2);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function test_select_ebool(bool control, bool ifTrue, bool ifFalse) public {
        ebool controlProc = HTTPZ.asEbool(control);
        ebool ifTrueProc = HTTPZ.asEbool(ifTrue);
        ebool ifFalseProc = HTTPZ.asEbool(ifFalse);
        ebool result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function test_select_ebytes64(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = HTTPZ.asEbool(control);
        ebytes64 ifTrueProc = HTTPZ.asEbytes64(HTTPZ.padToBytes64(ifTrue));
        ebytes64 ifFalseProc = HTTPZ.asEbytes64(HTTPZ.padToBytes64(ifFalse));
        ebytes64 result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        resB64 = result;
    }

    function test_select_ebytes128(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = HTTPZ.asEbool(control);
        ebytes128 ifTrueProc = HTTPZ.asEbytes128(HTTPZ.padToBytes128(ifTrue));
        ebytes128 ifFalseProc = HTTPZ.asEbytes128(HTTPZ.padToBytes128(ifFalse));
        ebytes128 result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        resB128 = result;
    }

    function test_select_ebytes256(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = HTTPZ.asEbool(control);
        ebytes256 ifTrueProc = HTTPZ.asEbytes256(HTTPZ.padToBytes256(ifTrue));
        ebytes256 ifFalseProc = HTTPZ.asEbytes256(HTTPZ.padToBytes256(ifFalse));
        ebytes256 result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        resB256 = result;
    }

    function test_select(einput control, einput ifTrue, einput ifFalse, bytes calldata inputProof) public {
        ebool controlProc = HTTPZ.asEbool(control, inputProof);
        euint32 ifTrueProc = HTTPZ.asEuint32(ifTrue, inputProof);
        euint32 ifFalseProc = HTTPZ.asEuint32(ifFalse, inputProof);
        euint32 result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        res32 = result;
    }

    function test_select_eaddress(einput control, einput ifTrue, einput ifFalse, bytes calldata inputProof) public {
        ebool controlProc = HTTPZ.asEbool(control, inputProof);
        eaddress ifTrueProc = HTTPZ.asEaddress(ifTrue, inputProof);
        eaddress ifFalseProc = HTTPZ.asEaddress(ifFalse, inputProof);
        eaddress result = HTTPZ.select(controlProc, ifTrueProc, ifFalseProc);
        HTTPZ.allowThis(result);
        resAdd = result;
    }

    function test_eq_eaddress_eaddress(einput a, einput b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.asEaddress(a, inputProof);
        eaddress bProc = HTTPZ.asEaddress(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function test_ne_eaddress_eaddress(einput a, einput b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.asEaddress(a, inputProof);
        eaddress bProc = HTTPZ.asEaddress(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function test_eq_eaddress_address(einput a, address b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.asEaddress(a, inputProof);
        ebool result = HTTPZ.eq(aProc, b);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function test_eq_address_eaddress(einput a, address b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.asEaddress(a, inputProof);
        ebool result = HTTPZ.eq(b, aProc);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function test_ne_eaddress_address(einput a, address b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.asEaddress(a, inputProof);
        ebool result = HTTPZ.ne(aProc, b);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function test_ne_address_eaddress(einput a, address b, bytes calldata inputProof) public {
        eaddress aProc = HTTPZ.asEaddress(a, inputProof);
        ebool result = HTTPZ.ne(b, aProc);
        HTTPZ.allowThis(result);
        resb = result;
    }

    function test_ebool_to_euint8_cast(bool input) public {
        res8 = HTTPZ.asEuint8(HTTPZ.asEbool(input));
    }

    function test_ebool_to_euint16_cast(bool input) public {
        res16 = HTTPZ.asEuint16(HTTPZ.asEbool(input));
    }

    function test_ebool_to_euint32_cast(bool input) public {
        res32 = HTTPZ.asEuint32(HTTPZ.asEbool(input));
    }

    function test_ebool_to_euint64_cast(bool input) public {
        res64 = HTTPZ.asEuint64(HTTPZ.asEbool(input));
    }

    function test_ebool_to_euint128_cast(bool input) public {
        res128 = HTTPZ.asEuint128(HTTPZ.asEbool(input));
    }

    function test_ebool_to_euint256_cast(bool input) public {
        res256 = HTTPZ.asEuint256(HTTPZ.asEbool(input));
    }

    function test_euint128_to_euint8_cast(uint128 input) public {
        res8 = HTTPZ.asEuint8(HTTPZ.asEuint128(input));
    }

    function test_ebool_not(bool input) public {
        resb = HTTPZ.not(HTTPZ.asEbool(input));
    }

    function test_ebool_and(bool a, bool b) public {
        resb = HTTPZ.and(HTTPZ.asEbool(a), HTTPZ.asEbool(b));
    }

    function test_ebool_and_scalarL(bool a, bool b) public {
        resb = HTTPZ.and(a, HTTPZ.asEbool(b));
    }

    function test_ebool_and_scalarR(bool a, bool b) public {
        resb = HTTPZ.and(HTTPZ.asEbool(a), b);
    }

    function test_ebool_or(bool a, bool b) public {
        resb = HTTPZ.or(HTTPZ.asEbool(a), HTTPZ.asEbool(b));
    }

    function test_ebool_or_scalarL(bool a, bool b) public {
        resb = HTTPZ.or(a, HTTPZ.asEbool(b));
    }

    function test_ebool_or_scalarR(bool a, bool b) public {
        resb = HTTPZ.or(HTTPZ.asEbool(a), b);
    }

    function test_ebool_xor(bool a, bool b) public {
        resb = HTTPZ.xor(HTTPZ.asEbool(a), HTTPZ.asEbool(b));
    }

    function test_ebool_xor_scalarL(bool a, bool b) public {
        resb = HTTPZ.xor(a, HTTPZ.asEbool(b));
    }

    function test_ebool_xor_scalarR(bool a, bool b) public {
        resb = HTTPZ.xor(HTTPZ.asEbool(a), b);
    }
}
