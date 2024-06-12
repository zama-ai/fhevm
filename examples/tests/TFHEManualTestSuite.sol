// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.20;

import "../../abstracts/Reencrypt.sol";
import "../../lib/TFHE.sol";

contract TFHEManualTestSuite is Reencrypt {
    function test_eq_array_4(
        bytes calldata a,
        bytes calldata b,
        bytes calldata c,
        bytes calldata d
    ) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a);
        euint4 bProc = TFHE.asEuint4(b);
        euint4 cProc = TFHE.asEuint4(c);
        euint4 dProc = TFHE.asEuint4(d);
        euint4[] memory arrA = new euint4[](2);
        arrA[0] = aProc;
        arrA[1] = bProc;
        euint4[] memory arrB = new euint4[](2);
        arrB[0] = cProc;
        arrB[1] = dProc;
        ebool result = TFHE.eq(arrA, arrB);
        return TFHE.decrypt(result);
    }

    function test_eq_array_8(
        bytes calldata a,
        bytes calldata b,
        bytes calldata c,
        bytes calldata d
    ) public returns (bool) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 cProc = TFHE.asEuint8(c);
        euint8 dProc = TFHE.asEuint8(d);
        euint8[] memory arrA = new euint8[](2);
        arrA[0] = aProc;
        arrA[1] = bProc;
        euint8[] memory arrB = new euint8[](2);
        arrB[0] = cProc;
        arrB[1] = dProc;
        ebool result = TFHE.eq(arrA, arrB);
        return TFHE.decrypt(result);
    }

    function test_eq_array_16(
        bytes calldata a,
        bytes calldata b,
        bytes calldata c,
        bytes calldata d
    ) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint16 cProc = TFHE.asEuint16(c);
        euint16 dProc = TFHE.asEuint16(d);
        euint16[] memory arrA = new euint16[](2);
        arrA[0] = aProc;
        arrA[1] = bProc;
        euint16[] memory arrB = new euint16[](2);
        arrB[0] = cProc;
        arrB[1] = dProc;
        ebool result = TFHE.eq(arrA, arrB);
        return TFHE.decrypt(result);
    }

    function test_eq_array_32(
        bytes calldata a,
        bytes calldata b,
        bytes calldata c,
        bytes calldata d
    ) public returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 cProc = TFHE.asEuint32(c);
        euint32 dProc = TFHE.asEuint32(d);
        euint32[] memory arrA = new euint32[](2);
        arrA[0] = aProc;
        arrA[1] = bProc;
        euint32[] memory arrB = new euint32[](2);
        arrB[0] = cProc;
        arrB[1] = dProc;
        ebool result = TFHE.eq(arrA, arrB);
        return TFHE.decrypt(result);
    }

    function test_eq_array_64(
        bytes calldata a,
        bytes calldata b,
        bytes calldata c,
        bytes calldata d
    ) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 cProc = TFHE.asEuint64(c);
        euint64 dProc = TFHE.asEuint64(d);
        euint64[] memory arrA = new euint64[](2);
        arrA[0] = aProc;
        arrA[1] = bProc;
        euint64[] memory arrB = new euint64[](2);
        arrB[0] = cProc;
        arrB[1] = dProc;
        ebool result = TFHE.eq(arrA, arrB);
        return TFHE.decrypt(result);
    }

    function test_select(
        bytes calldata control,
        bytes calldata ifTrue,
        bytes calldata ifFalse
    ) public returns (uint32) {
        ebool controlProc = TFHE.asEbool(control);
        euint32 ifTrueProc = TFHE.asEuint32(ifTrue);
        euint32 ifFalseProc = TFHE.asEuint32(ifFalse);
        return TFHE.decrypt(TFHE.select(controlProc, ifTrueProc, ifFalseProc));
    }

    function test_select_eaddress(
        bytes calldata control,
        bytes calldata ifTrue,
        bytes calldata ifFalse
    ) public returns (address) {
        ebool controlProc = TFHE.asEbool(control);
        eaddress ifTrueProc = TFHE.asEaddress(ifTrue);
        eaddress ifFalseProc = TFHE.asEaddress(ifFalse);
        return TFHE.decrypt(TFHE.select(controlProc, ifTrueProc, ifFalseProc));
    }

    function test_eq_eaddress_eaddress(bytes calldata a, bytes calldata b) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        eaddress bProc = TFHE.asEaddress(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_ne_eaddress_eaddress(bytes calldata a, bytes calldata b) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        eaddress bProc = TFHE.asEaddress(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_eq_eaddress_address(bytes calldata a, address b) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        address bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_eq_address_eaddress(address b, bytes calldata a) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        address bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_ne_eaddress_address(bytes calldata a, address b) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        address bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_ne_address_eaddress(address b, bytes calldata a) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a);
        address bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_eaddress_decrypt(bytes calldata addr) public returns (address) {
        eaddress addProc = TFHE.asEaddress(addr);
        return TFHE.decrypt(addProc);
    }

    function test_reencrypt_eaddress(
        bytes calldata addr,
        bytes32 publicKey,
        bytes calldata signature
    ) public virtual onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
        eaddress addProc = TFHE.asEaddress(addr);
        return TFHE.reencrypt(addProc, publicKey);
    }

    function test_ebool_to_euint4_cast(bool input) public returns (uint16) {
        return TFHE.decrypt(TFHE.asEuint4(TFHE.asEbool(input)));
    }

    function test_ebool_to_euint8_cast(bool input) public returns (uint16) {
        return TFHE.decrypt(TFHE.asEuint8(TFHE.asEbool(input)));
    }

    function test_ebool_to_euint16_cast(bool input) public returns (uint16) {
        return TFHE.decrypt(TFHE.asEuint16(TFHE.asEbool(input)));
    }

    function test_ebool_to_euint32_cast(bool input) public returns (uint32) {
        return TFHE.decrypt(TFHE.asEuint32(TFHE.asEbool(input)));
    }

    function test_ebool_to_euint64_cast(bool input) public returns (uint64) {
        return TFHE.decrypt(TFHE.asEuint64(TFHE.asEbool(input)));
    }

    function test_ebool_not(bool input) public returns (bool) {
        return TFHE.decrypt(TFHE.not(TFHE.asEbool(input)));
    }

    function test_ebool_and(bool a, bool b) public returns (bool) {
        return TFHE.decrypt(TFHE.and(TFHE.asEbool(a), TFHE.asEbool(b)));
    }

    function test_ebool_or(bool a, bool b) public returns (bool) {
        return TFHE.decrypt(TFHE.or(TFHE.asEbool(a), TFHE.asEbool(b)));
    }

    function test_ebool_xor(bool a, bool b) public returns (bool) {
        return TFHE.decrypt(TFHE.xor(TFHE.asEbool(a), TFHE.asEbool(b)));
    }
}
