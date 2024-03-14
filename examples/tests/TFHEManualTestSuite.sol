// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.20;

import "../../lib/TFHE.sol";

contract TFHEManualTestSuite {
    function test_select(
        bytes calldata control,
        bytes calldata ifTrue,
        bytes calldata ifFalse
    ) public view returns (uint32) {
        ebool controlProc = TFHE.asEbool(control);
        euint32 ifTrueProc = TFHE.asEuint32(ifTrue);
        euint32 ifFalseProc = TFHE.asEuint32(ifFalse);
        return TFHE.decrypt(TFHE.select(controlProc, ifTrueProc, ifFalseProc));
    }

    function test_eq_eaddress_eaddress(bytes calldata a, bytes calldata b) public view returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        eaddress bProc = TFHE.asEaddress(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_ne_eaddress_eaddress(bytes calldata a, bytes calldata b) public view returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        eaddress bProc = TFHE.asEaddress(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_eaddress_address(bytes calldata a, address b) public view returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        address bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_address_eaddress(address b, bytes calldata a) public view returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        address bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_eaddress_address(bytes calldata a, address b) public view returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        address bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_address_eaddress(address b, bytes calldata a) public view returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        address bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_eaddress_decrypt(bytes calldata addr) public view returns (address) {
        eaddress addProc = TFHE.asEaddress(addr);
        return TFHE.decryptCustom(addProc);
    }

    function test_ebool_to_euint4_cast(bool input) public view returns (uint16) {
        return TFHE.decrypt(TFHE.asEuint4(TFHE.asEbool(input)));
    }

    function test_ebool_to_euint8_cast(bool input) public view returns (uint16) {
        return TFHE.decrypt(TFHE.asEuint8(TFHE.asEbool(input)));
    }

    function test_ebool_to_euint16_cast(bool input) public view returns (uint16) {
        return TFHE.decrypt(TFHE.asEuint16(TFHE.asEbool(input)));
    }

    function test_ebool_to_euint32_cast(bool input) public view returns (uint32) {
        return TFHE.decrypt(TFHE.asEuint32(TFHE.asEbool(input)));
    }

    function test_ebool_to_euint64_cast(bool input) public view returns (uint64) {
        return TFHE.decrypt(TFHE.asEuint64(TFHE.asEbool(input)));
    }

    function test_ebool_not(bool input) public view returns (bool) {
        return TFHE.decrypt(TFHE.not(TFHE.asEbool(input)));
    }

    function test_ebool_and(bool a, bool b) public view returns (bool) {
        return TFHE.decrypt(TFHE.and(TFHE.asEbool(a), TFHE.asEbool(b)));
    }

    function test_ebool_or(bool a, bool b) public view returns (bool) {
        return TFHE.decrypt(TFHE.or(TFHE.asEbool(a), TFHE.asEbool(b)));
    }

    function test_ebool_xor(bool a, bool b) public view returns (bool) {
        return TFHE.decrypt(TFHE.xor(TFHE.asEbool(a), TFHE.asEbool(b)));
    }
}
