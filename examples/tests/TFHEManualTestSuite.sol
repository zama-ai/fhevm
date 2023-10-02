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
}
