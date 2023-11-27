// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.19;

import "../../lib/TFHE.sol";

contract TFHEManualTestSuite {
    function test_cmux(
        bytes calldata control,
        bytes calldata ifTrue,
        bytes calldata ifFalse
    ) public view returns (uint32) {
        ebool controlProc = TFHE.asEbool(control);
        euint32 ifTrueProc = TFHE.asEuint32(ifTrue);
        euint32 ifFalseProc = TFHE.asEuint32(ifFalse);
        return TFHE.decrypt(TFHE.cmux(controlProc, ifTrueProc, ifFalseProc));
    }

    function test_ebool_to_euint16_cast(bool input) public view returns (uint16) {
        return TFHE.decrypt(TFHE.asEuint16(TFHE.asEbool(input)));
    }

    function test_ebool_to_euint32_cast(bool input) public view returns (uint32) {
        return TFHE.decrypt(TFHE.asEuint32(TFHE.asEbool(input)));
    }

    function test_opt_req(bool input) public view {
        TFHE.optReq(TFHE.asEbool(input));
    }

    uint32 counter = 0;

    function test_opt_req_stateful(bool input) public {
        TFHE.optReq(TFHE.asEbool(input));
        counter += 1;
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
