// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.20;

import "../../abstracts/Reencrypt.sol";
import "../../lib/TFHE.sol";

contract TFHEManualTestSuite is Reencrypt {
    function test_eq_array_4(einput a, einput b, einput c, einput d, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 cProc = TFHE.asEuint4(c, inputProof);
        euint4 dProc = TFHE.asEuint4(d, inputProof);
        euint4[] memory arrA = new euint4[](2);
        arrA[0] = aProc;
        arrA[1] = bProc;
        euint4[] memory arrB = new euint4[](2);
        arrB[0] = cProc;
        arrB[1] = dProc;
        ebool result = TFHE.eq(arrA, arrB);
        return TFHE.decrypt(result);
    }

    function test_eq_array_8(einput a, einput b, einput c, einput d, bytes calldata inputProof) public returns (bool) {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 cProc = TFHE.asEuint8(c, inputProof);
        euint8 dProc = TFHE.asEuint8(d, inputProof);
        euint8[] memory arrA = new euint8[](2);
        arrA[0] = aProc;
        arrA[1] = bProc;
        euint8[] memory arrB = new euint8[](2);
        arrB[0] = cProc;
        arrB[1] = dProc;
        ebool result = TFHE.eq(arrA, arrB);
        return TFHE.decrypt(result);
    }

    function test_eq_array_16(einput a, einput b, einput c, einput d, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 cProc = TFHE.asEuint16(c, inputProof);
        euint16 dProc = TFHE.asEuint16(d, inputProof);
        euint16[] memory arrA = new euint16[](2);
        arrA[0] = aProc;
        arrA[1] = bProc;
        euint16[] memory arrB = new euint16[](2);
        arrB[0] = cProc;
        arrB[1] = dProc;
        ebool result = TFHE.eq(arrA, arrB);
        return TFHE.decrypt(result);
    }

    function test_eq_array_32(einput a, einput b, einput c, einput d, bytes calldata inputProof) public returns (bool) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 cProc = TFHE.asEuint32(c, inputProof);
        euint32 dProc = TFHE.asEuint32(d, inputProof);
        euint32[] memory arrA = new euint32[](2);
        arrA[0] = aProc;
        arrA[1] = bProc;
        euint32[] memory arrB = new euint32[](2);
        arrB[0] = cProc;
        arrB[1] = dProc;
        ebool result = TFHE.eq(arrA, arrB);
        return TFHE.decrypt(result);
    }

    function test_eq_array_64(einput a, einput b, einput c, einput d, bytes calldata inputProof) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 cProc = TFHE.asEuint64(c, inputProof);
        euint64 dProc = TFHE.asEuint64(d, inputProof);
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
        einput control,
        einput ifTrue,
        einput ifFalse,
        bytes calldata inputProof
    ) public returns (uint32) {
        ebool controlProc = TFHE.asEbool(control, inputProof);
        euint32 ifTrueProc = TFHE.asEuint32(ifTrue, inputProof);
        euint32 ifFalseProc = TFHE.asEuint32(ifFalse, inputProof);
        return TFHE.decrypt(TFHE.select(controlProc, ifTrueProc, ifFalseProc));
    }

    function test_select_eaddress(
        einput control,
        einput ifTrue,
        einput ifFalse,
        bytes calldata inputProof
    ) public returns (address) {
        ebool controlProc = TFHE.asEbool(control, inputProof);
        eaddress ifTrueProc = TFHE.asEaddress(ifTrue, inputProof);
        eaddress ifFalseProc = TFHE.asEaddress(ifFalse, inputProof);
        return TFHE.decrypt(TFHE.select(controlProc, ifTrueProc, ifFalseProc));
    }

    function test_eq_eaddress_eaddress(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        eaddress bProc = TFHE.asEaddress(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_ne_eaddress_eaddress(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        eaddress bProc = TFHE.asEaddress(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_eq_eaddress_address(einput a, bytes calldata inputProof, address b) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        address bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_eq_address_eaddress(address b, einput a, bytes calldata inputProof) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        address bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_ne_eaddress_address(einput a, bytes calldata inputProof, address b) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        address bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_ne_address_eaddress(address b, einput a, bytes calldata inputProof) public returns (bool) {
        eaddress aProc = TFHE.asEaddress(a, inputProof);
        address bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function test_eaddress_decrypt(einput addr, bytes calldata inputProof) public returns (address) {
        eaddress addProc = TFHE.asEaddress(addr, inputProof);
        return TFHE.decrypt(addProc);
    }

    function test_reencrypt_eaddress(
        einput addr,
        bytes calldata inputProof,
        bytes32 publicKey,
        bytes calldata signature
    ) public virtual onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
        eaddress addProc = TFHE.asEaddress(addr, inputProof);
        return TFHE.reencrypt(addProc, publicKey);
    }

    function test_ebool_to_euint4_cast(bool inputProof) public returns (uint16) {
        return TFHE.decrypt(TFHE.asEuint4(TFHE.asEbool(inputProof)));
    }

    function test_ebool_to_euint8_cast(bool inputProof) public returns (uint16) {
        return TFHE.decrypt(TFHE.asEuint8(TFHE.asEbool(inputProof)));
    }

    function test_ebool_to_euint16_cast(bool inputProof) public returns (uint16) {
        return TFHE.decrypt(TFHE.asEuint16(TFHE.asEbool(inputProof)));
    }

    function test_ebool_to_euint32_cast(bool inputProof) public returns (uint32) {
        return TFHE.decrypt(TFHE.asEuint32(TFHE.asEbool(inputProof)));
    }

    function test_ebool_to_euint64_cast(bool inputProof) public returns (uint64) {
        return TFHE.decrypt(TFHE.asEuint64(TFHE.asEbool(inputProof)));
    }

    function test_ebool_not(bool inputProof) public returns (bool) {
        return TFHE.decrypt(TFHE.not(TFHE.asEbool(inputProof)));
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
