// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../../lib/FHEVMConfig.sol";

contract TFHEManualTestSuite {
    ebool public resb;
    euint4 public res4;
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
        TFHE.setFHEVM(FHEVMConfig.defaultConfig()); // Set up the FHEVM configuration for this contract
    }

    function eqEbool(bool a, bool b) external {
        ebool input1 = TFHE.asEbool(a);
        ebool input2 = TFHE.asEbool(b);
        ebool result = TFHE.eq(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function eqEboolScalarL(bool a, bool b) external {
        ebool input2 = TFHE.asEbool(b);
        ebool result = TFHE.eq(a, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function eqEboolScalarR(bool a, bool b) external {
        ebool input1 = TFHE.asEbool(a);
        ebool result = TFHE.eq(input1, b);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEbool(bool a, bool b) external {
        ebool input1 = TFHE.asEbool(a);
        ebool input2 = TFHE.asEbool(b);
        ebool result = TFHE.ne(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEboolScalarL(bool a, bool b) external {
        ebool input2 = TFHE.asEbool(b);
        ebool result = TFHE.ne(a, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEboolScalarR(bool a, bool b) external {
        ebool input1 = TFHE.asEbool(a);
        ebool result = TFHE.ne(input1, b);
        TFHE.allowThis(result);
        resb = result;
    }

    function eqEbytes256(einput inp1, bytes calldata inputProof1, einput inp2, bytes calldata inputProof2) external {
        ebytes256 input1 = TFHE.asEbytes256(inp1, inputProof1);
        ebytes256 input2 = TFHE.asEbytes256(inp2, inputProof2);
        ebool result = TFHE.eq(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEbytes256(einput inp1, bytes calldata inputProof1, einput inp2, bytes calldata inputProof2) external {
        ebytes256 input1 = TFHE.asEbytes256(inp1, inputProof1);
        ebytes256 input2 = TFHE.asEbytes256(inp2, inputProof2);
        ebool result = TFHE.ne(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function eqEbytes64(bytes memory a, bytes memory b) external {
        ebytes64 input1 = TFHE.asEbytes64(TFHE.padToBytes64(a));
        ebytes64 input2 = TFHE.asEbytes64(TFHE.padToBytes64(b));
        ebool result = TFHE.eq(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function eqEbytes64ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = TFHE.padToBytes64(a);
        ebytes64 input2 = TFHE.asEbytes64(TFHE.padToBytes64(b));
        ebool result = TFHE.eq(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function eqEbytes64ScalarR(bytes memory a, bytes memory b) external {
        ebytes64 input1 = TFHE.asEbytes64(TFHE.padToBytes64(a));
        bytes memory input2 = TFHE.padToBytes64(b);
        ebool result = TFHE.eq(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEbytes64(bytes memory a, bytes memory b) external {
        ebytes64 input1 = TFHE.asEbytes64(TFHE.padToBytes64(a));
        ebytes64 input2 = TFHE.asEbytes64(TFHE.padToBytes64(b));
        ebool result = TFHE.ne(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEbytes64ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = TFHE.padToBytes64(a);
        ebytes64 input2 = TFHE.asEbytes64(TFHE.padToBytes64(b));
        ebool result = TFHE.ne(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEbytes64ScalarR(bytes memory a, bytes memory b) external {
        ebytes64 input1 = TFHE.asEbytes64(TFHE.padToBytes64(a));
        bytes memory input2 = TFHE.padToBytes64(b);
        ebool result = TFHE.ne(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function eqEbytes128(bytes memory a, bytes memory b) external {
        ebytes128 input1 = TFHE.asEbytes128(TFHE.padToBytes128(a));
        ebytes128 input2 = TFHE.asEbytes128(TFHE.padToBytes128(b));
        ebool result = TFHE.eq(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function eqEbytes128ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = TFHE.padToBytes128(a);
        ebytes128 input2 = TFHE.asEbytes128(TFHE.padToBytes128(b));
        ebool result = TFHE.eq(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function eqEbytes128ScalarR(bytes memory a, bytes memory b) external {
        ebytes128 input1 = TFHE.asEbytes128(TFHE.padToBytes128(a));
        bytes memory input2 = TFHE.padToBytes128(b);
        ebool result = TFHE.eq(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEbytes128(bytes memory a, bytes memory b) external {
        ebytes128 input1 = TFHE.asEbytes128(TFHE.padToBytes128(a));
        ebytes128 input2 = TFHE.asEbytes128(TFHE.padToBytes128(b));
        ebool result = TFHE.ne(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEbytes128ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = TFHE.padToBytes128(a);
        ebytes128 input2 = TFHE.asEbytes128(TFHE.padToBytes128(b));
        ebool result = TFHE.ne(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEbytes128ScalarR(bytes memory a, bytes memory b) external {
        ebytes128 input1 = TFHE.asEbytes128(TFHE.padToBytes128(a));
        bytes memory input2 = TFHE.padToBytes128(b);
        ebool result = TFHE.ne(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function eqEbytes256ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = TFHE.padToBytes256(a);
        ebytes256 input2 = TFHE.asEbytes256(TFHE.padToBytes256(b));
        ebool result = TFHE.eq(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function eqEbytes256ScalarR(bytes memory a, bytes memory b) external {
        ebytes256 input1 = TFHE.asEbytes256(TFHE.padToBytes256(a));
        bytes memory input2 = TFHE.padToBytes256(b);
        ebool result = TFHE.eq(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEbytes256ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = TFHE.padToBytes256(a);
        ebytes256 input2 = TFHE.asEbytes256(TFHE.padToBytes256(b));
        ebool result = TFHE.ne(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function neEbytes256ScalarR(bytes memory a, bytes memory b) external {
        ebytes256 input1 = TFHE.asEbytes256(TFHE.padToBytes256(a));
        bytes memory input2 = TFHE.padToBytes256(b);
        ebool result = TFHE.ne(input1, input2);
        TFHE.allowThis(result);
        resb = result;
    }

    function test_select_ebool(bool control, bool ifTrue, bool ifFalse) public {
        ebool controlProc = TFHE.asEbool(control);
        ebool ifTrueProc = TFHE.asEbool(ifTrue);
        ebool ifFalseProc = TFHE.asEbool(ifFalse);
        ebool result = TFHE.select(controlProc, ifTrueProc, ifFalseProc);
        TFHE.allowThis(result);
        resb = result;
    }

    function test_select_ebytes64(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = TFHE.asEbool(control);
        ebytes64 ifTrueProc = TFHE.asEbytes64(TFHE.padToBytes64(ifTrue));
        ebytes64 ifFalseProc = TFHE.asEbytes64(TFHE.padToBytes64(ifFalse));
        ebytes64 result = TFHE.select(controlProc, ifTrueProc, ifFalseProc);
        TFHE.allowThis(result);
        resB64 = result;
    }

    function test_select_ebytes128(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = TFHE.asEbool(control);
        ebytes128 ifTrueProc = TFHE.asEbytes128(TFHE.padToBytes128(ifTrue));
        ebytes128 ifFalseProc = TFHE.asEbytes128(TFHE.padToBytes128(ifFalse));
        ebytes128 result = TFHE.select(controlProc, ifTrueProc, ifFalseProc);
        TFHE.allowThis(result);
        resB128 = result;
    }

    function test_select_ebytes256(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = TFHE.asEbool(control);
        ebytes256 ifTrueProc = TFHE.asEbytes256(TFHE.padToBytes256(ifTrue));
        ebytes256 ifFalseProc = TFHE.asEbytes256(TFHE.padToBytes256(ifFalse));
        ebytes256 result = TFHE.select(controlProc, ifTrueProc, ifFalseProc);
        TFHE.allowThis(result);
        resB256 = result;
    }

    function test_select(einput control, einput ifTrue, einput ifFalse, bytes calldata inputProof) public {
        ebool controlProc = TFHE.asEbool(control, inputProof);
        euint32 ifTrueProc = TFHE.asEuint32(ifTrue, inputProof);
        euint32 ifFalseProc = TFHE.asEuint32(ifFalse, inputProof);
        euint32 result = TFHE.select(controlProc, ifTrueProc, ifFalseProc);
        TFHE.allowThis(result);
        res32 = result;
    }

    function test_select_eaddress(einput control, einput ifTrue, einput ifFalse, bytes calldata inputProof) public {
        ebool controlProc = TFHE.asEbool(control, inputProof);
        eaddress ifTrueProc = TFHE.asEaddress(ifTrue, inputProof);
        eaddress ifFalseProc = TFHE.asEaddress(ifFalse, inputProof);
        eaddress result = TFHE.select(controlProc, ifTrueProc, ifFalseProc);
        TFHE.allowThis(result);
        resAdd = result;
    }

    function test_eq_eaddress_eaddress(einput a, einput b, bytes calldata inputProof) public {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        eaddress bProc = TFHE.asEaddress(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }

    function test_ne_eaddress_eaddress(einput a, einput b, bytes calldata inputProof) public {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        eaddress bProc = TFHE.asEaddress(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }

    function test_eq_eaddress_address(einput a, address b, bytes calldata inputProof) public {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        ebool result = TFHE.eq(aProc, b);
        TFHE.allowThis(result);
        resb = result;
    }

    function test_eq_address_eaddress(einput a, address b, bytes calldata inputProof) public {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        ebool result = TFHE.eq(b, aProc);
        TFHE.allowThis(result);
        resb = result;
    }

    function test_ne_eaddress_address(einput a, address b, bytes calldata inputProof) public {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        ebool result = TFHE.ne(aProc, b);
        TFHE.allowThis(result);
        resb = result;
    }

    function test_ne_address_eaddress(einput a, address b, bytes calldata inputProof) public {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        ebool result = TFHE.ne(b, aProc);
        TFHE.allowThis(result);
        resb = result;
    }

    function test_ebool_to_euint4_cast(bool input) public {
        res4 = TFHE.asEuint4(TFHE.asEbool(input));
    }

    function test_ebool_to_euint8_cast(bool input) public {
        res8 = TFHE.asEuint8(TFHE.asEbool(input));
    }

    function test_ebool_to_euint16_cast(bool input) public {
        res16 = TFHE.asEuint16(TFHE.asEbool(input));
    }

    function test_ebool_to_euint32_cast(bool input) public {
        res32 = TFHE.asEuint32(TFHE.asEbool(input));
    }

    function test_ebool_to_euint64_cast(bool input) public {
        res64 = TFHE.asEuint64(TFHE.asEbool(input));
    }

    function test_ebool_to_euint128_cast(bool input) public {
        res128 = TFHE.asEuint128(TFHE.asEbool(input));
    }

    function test_ebool_to_euint256_cast(bool input) public {
        res256 = TFHE.asEuint256(TFHE.asEbool(input));
    }

    function test_euint4_to_euint256_cast(uint8 input) public {
        res256 = TFHE.asEuint256(TFHE.asEuint4(input));
    }

    function test_euint128_to_euint8_cast(uint128 input) public {
        res8 = TFHE.asEuint8(TFHE.asEuint128(input));
    }

    function test_ebool_not(bool input) public {
        resb = TFHE.not(TFHE.asEbool(input));
    }

    function test_ebool_and(bool a, bool b) public {
        resb = TFHE.and(TFHE.asEbool(a), TFHE.asEbool(b));
    }

    function test_ebool_and_scalarL(bool a, bool b) public {
        resb = TFHE.and(a, TFHE.asEbool(b));
    }

    function test_ebool_and_scalarR(bool a, bool b) public {
        resb = TFHE.and(TFHE.asEbool(a), b);
    }

    function test_ebool_or(bool a, bool b) public {
        resb = TFHE.or(TFHE.asEbool(a), TFHE.asEbool(b));
    }

    function test_ebool_or_scalarL(bool a, bool b) public {
        resb = TFHE.or(a, TFHE.asEbool(b));
    }

    function test_ebool_or_scalarR(bool a, bool b) public {
        resb = TFHE.or(TFHE.asEbool(a), b);
    }

    function test_ebool_xor(bool a, bool b) public {
        resb = TFHE.xor(TFHE.asEbool(a), TFHE.asEbool(b));
    }

    function test_ebool_xor_scalarL(bool a, bool b) public {
        resb = TFHE.xor(a, TFHE.asEbool(b));
    }

    function test_ebool_xor_scalarR(bool a, bool b) public {
        resb = TFHE.xor(TFHE.asEbool(a), b);
    }
}
