// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/FHE.sol";
import "../../lib/FHEVMConfig.sol";

contract FHEVMManualTestSuite {
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
        FHE.setCoprocessor(FHEVMConfig.defaultConfig()); // Set up the FHEVM configuration for this contract
    }

    function eqEbool(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.eq(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function eqEboolScalarL(bool a, bool b) external {
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.eq(a, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function eqEboolScalarR(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool result = FHE.eq(input1, b);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEbool(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.ne(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEboolScalarL(bool a, bool b) external {
        ebool input2 = FHE.asEbool(b);
        ebool result = FHE.ne(a, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEboolScalarR(bool a, bool b) external {
        ebool input1 = FHE.asEbool(a);
        ebool result = FHE.ne(input1, b);
        FHE.allowThis(result);
        resEbool = result;
    }

    function eqEbytes256(
        externalEbytes256 inp1,
        bytes calldata inputProof1,
        externalEbytes256 inp2,
        bytes calldata inputProof2
    ) external {
        ebytes256 input1 = FHE.fromExternal(inp1, inputProof1);
        ebytes256 input2 = FHE.fromExternal(inp2, inputProof2);
        ebool result = FHE.eq(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEbytes256(
        externalEbytes256 inp1,
        bytes calldata inputProof1,
        externalEbytes256 inp2,
        bytes calldata inputProof2
    ) external {
        ebytes256 input1 = FHE.fromExternal(inp1, inputProof1);
        ebytes256 input2 = FHE.fromExternal(inp2, inputProof2);
        ebool result = FHE.ne(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function eqEbytes64(bytes memory a, bytes memory b) external {
        ebytes64 input1 = FHE.asEbytes64(FHE.padToBytes64(a));
        ebytes64 input2 = FHE.asEbytes64(FHE.padToBytes64(b));
        ebool result = FHE.eq(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function eqEbytes64ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = FHE.padToBytes64(a);
        ebytes64 input2 = FHE.asEbytes64(FHE.padToBytes64(b));
        ebool result = FHE.eq(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function eqEbytes64ScalarR(bytes memory a, bytes memory b) external {
        ebytes64 input1 = FHE.asEbytes64(FHE.padToBytes64(a));
        bytes memory input2 = FHE.padToBytes64(b);
        ebool result = FHE.eq(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEbytes64(bytes memory a, bytes memory b) external {
        ebytes64 input1 = FHE.asEbytes64(FHE.padToBytes64(a));
        ebytes64 input2 = FHE.asEbytes64(FHE.padToBytes64(b));
        ebool result = FHE.ne(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEbytes64ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = FHE.padToBytes64(a);
        ebytes64 input2 = FHE.asEbytes64(FHE.padToBytes64(b));
        ebool result = FHE.ne(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEbytes64ScalarR(bytes memory a, bytes memory b) external {
        ebytes64 input1 = FHE.asEbytes64(FHE.padToBytes64(a));
        bytes memory input2 = FHE.padToBytes64(b);
        ebool result = FHE.ne(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function eqEbytes128(bytes memory a, bytes memory b) external {
        ebytes128 input1 = FHE.asEbytes128(FHE.padToBytes128(a));
        ebytes128 input2 = FHE.asEbytes128(FHE.padToBytes128(b));
        ebool result = FHE.eq(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function eqEbytes128ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = FHE.padToBytes128(a);
        ebytes128 input2 = FHE.asEbytes128(FHE.padToBytes128(b));
        ebool result = FHE.eq(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function eqEbytes128ScalarR(bytes memory a, bytes memory b) external {
        ebytes128 input1 = FHE.asEbytes128(FHE.padToBytes128(a));
        bytes memory input2 = FHE.padToBytes128(b);
        ebool result = FHE.eq(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEbytes128(bytes memory a, bytes memory b) external {
        ebytes128 input1 = FHE.asEbytes128(FHE.padToBytes128(a));
        ebytes128 input2 = FHE.asEbytes128(FHE.padToBytes128(b));
        ebool result = FHE.ne(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEbytes128ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = FHE.padToBytes128(a);
        ebytes128 input2 = FHE.asEbytes128(FHE.padToBytes128(b));
        ebool result = FHE.ne(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEbytes128ScalarR(bytes memory a, bytes memory b) external {
        ebytes128 input1 = FHE.asEbytes128(FHE.padToBytes128(a));
        bytes memory input2 = FHE.padToBytes128(b);
        ebool result = FHE.ne(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function eqEbytes256ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = FHE.padToBytes256(a);
        ebytes256 input2 = FHE.asEbytes256(FHE.padToBytes256(b));
        ebool result = FHE.eq(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function eqEbytes256ScalarR(bytes memory a, bytes memory b) external {
        ebytes256 input1 = FHE.asEbytes256(FHE.padToBytes256(a));
        bytes memory input2 = FHE.padToBytes256(b);
        ebool result = FHE.eq(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEbytes256ScalarL(bytes memory a, bytes memory b) external {
        bytes memory input1 = FHE.padToBytes256(a);
        ebytes256 input2 = FHE.asEbytes256(FHE.padToBytes256(b));
        ebool result = FHE.ne(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function neEbytes256ScalarR(bytes memory a, bytes memory b) external {
        ebytes256 input1 = FHE.asEbytes256(FHE.padToBytes256(a));
        bytes memory input2 = FHE.padToBytes256(b);
        ebool result = FHE.ne(input1, input2);
        FHE.allowThis(result);
        resEbool = result;
    }

    function test_select_ebool(bool control, bool ifTrue, bool ifFalse) public {
        ebool controlProc = FHE.asEbool(control);
        ebool ifTrueProc = FHE.asEbool(ifTrue);
        ebool ifFalseProc = FHE.asEbool(ifFalse);
        ebool result = FHE.select(controlProc, ifTrueProc, ifFalseProc);
        FHE.allowThis(result);
        resEbool = result;
    }

    function test_select_ebytes64(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = FHE.asEbool(control);
        ebytes64 ifTrueProc = FHE.asEbytes64(FHE.padToBytes64(ifTrue));
        ebytes64 ifFalseProc = FHE.asEbytes64(FHE.padToBytes64(ifFalse));
        ebytes64 result = FHE.select(controlProc, ifTrueProc, ifFalseProc);
        FHE.allowThis(result);
        resEbytes64 = result;
    }

    function test_select_ebytes128(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = FHE.asEbool(control);
        ebytes128 ifTrueProc = FHE.asEbytes128(FHE.padToBytes128(ifTrue));
        ebytes128 ifFalseProc = FHE.asEbytes128(FHE.padToBytes128(ifFalse));
        ebytes128 result = FHE.select(controlProc, ifTrueProc, ifFalseProc);
        FHE.allowThis(result);
        resEbytes128 = result;
    }

    function test_select_ebytes256(bool control, bytes memory ifTrue, bytes memory ifFalse) public {
        ebool controlProc = FHE.asEbool(control);
        ebytes256 ifTrueProc = FHE.asEbytes256(FHE.padToBytes256(ifTrue));
        ebytes256 ifFalseProc = FHE.asEbytes256(FHE.padToBytes256(ifFalse));
        ebytes256 result = FHE.select(controlProc, ifTrueProc, ifFalseProc);
        FHE.allowThis(result);
        resEbytes256 = result;
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
        resAdd = result;
    }

    function test_eq_eaddress_eaddress(externalEaddress a, externalEaddress b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        eaddress bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }

    function test_ne_eaddress_eaddress(externalEaddress a, externalEaddress b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        eaddress bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }

    function test_eq_eaddress_address(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.eq(aProc, b);
        FHE.allowThis(result);
        resEbool = result;
    }

    function test_eq_address_eaddress(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.eq(b, aProc);
        FHE.allowThis(result);
        resEbool = result;
    }

    function test_ne_eaddress_address(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.ne(aProc, b);
        FHE.allowThis(result);
        resEbool = result;
    }

    function test_ne_address_eaddress(externalEaddress a, address b, bytes calldata inputProof) public {
        eaddress aProc = FHE.fromExternal(a, inputProof);
        ebool result = FHE.ne(b, aProc);
        FHE.allowThis(result);
        resEbool = result;
    }

    function test_ebool_to_euint8_cast(bool input) public {
        resEuint8 = FHE.asEuint8(FHE.asEbool(input));
    }

    function test_ebool_to_euint16_cast(bool input) public {
        resEuint16 = FHE.asEuint16(FHE.asEbool(input));
    }

    function test_ebool_to_euint32_cast(bool input) public {
        resEuint32 = FHE.asEuint32(FHE.asEbool(input));
    }

    function test_ebool_to_euint64_cast(bool input) public {
        resEuint64 = FHE.asEuint64(FHE.asEbool(input));
    }

    function test_ebool_to_euint128_cast(bool input) public {
        resEuint128 = FHE.asEuint128(FHE.asEbool(input));
    }

    function test_ebool_to_euint256_cast(bool input) public {
        resEuint256 = FHE.asEuint256(FHE.asEbool(input));
    }

    function test_euint128_to_euint8_cast(uint128 input) public {
        resEuint8 = FHE.asEuint8(FHE.asEuint128(input));
    }

    function test_ebool_not(bool input) public {
        resEbool = FHE.not(FHE.asEbool(input));
    }

    function test_ebool_and(bool a, bool b) public {
        resEbool = FHE.and(FHE.asEbool(a), FHE.asEbool(b));
    }

    function test_ebool_and_scalarL(bool a, bool b) public {
        resEbool = FHE.and(a, FHE.asEbool(b));
    }

    function test_ebool_and_scalarR(bool a, bool b) public {
        resEbool = FHE.and(FHE.asEbool(a), b);
    }

    function test_ebool_or(bool a, bool b) public {
        resEbool = FHE.or(FHE.asEbool(a), FHE.asEbool(b));
    }

    function test_ebool_or_scalarL(bool a, bool b) public {
        resEbool = FHE.or(a, FHE.asEbool(b));
    }

    function test_ebool_or_scalarR(bool a, bool b) public {
        resEbool = FHE.or(FHE.asEbool(a), b);
    }

    function test_ebool_xor(bool a, bool b) public {
        resEbool = FHE.xor(FHE.asEbool(a), FHE.asEbool(b));
    }

    function test_ebool_xor_scalarL(bool a, bool b) public {
        resEbool = FHE.xor(a, FHE.asEbool(b));
    }

    function test_ebool_xor_scalarR(bool a, bool b) public {
        resEbool = FHE.xor(FHE.asEbool(a), b);
    }
}
