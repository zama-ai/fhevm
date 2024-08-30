// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../../payment/Payment.sol";

contract TFHEManualTestSuite {
    ebool public resb;
    euint4 public res4;
    euint8 public res8;
    euint16 public res16;
    euint32 public res32;
    euint64 public res64;
    eaddress public resAdd;

    constructor() payable {
        Payment.depositForThis(msg.value);
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

    function test_ebool_not(bool input) public {
        resb = TFHE.not(TFHE.asEbool(input));
    }

    function test_ebool_and(bool a, bool b) public {
        resb = TFHE.and(TFHE.asEbool(a), TFHE.asEbool(b));
    }

    function test_ebool_or(bool a, bool b) public {
        resb = TFHE.or(TFHE.asEbool(a), TFHE.asEbool(b));
    }

    function test_ebool_xor(bool a, bool b) public {
        resb = TFHE.xor(TFHE.asEbool(a), TFHE.asEbool(b));
    }
}
