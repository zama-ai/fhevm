// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.19;

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
}
