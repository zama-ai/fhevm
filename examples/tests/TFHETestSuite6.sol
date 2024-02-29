// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.20;

import "../../lib/TFHE.sol";

contract TFHETestSuite6 {
    function unary_op_neg_euint8(bytes calldata a) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 result = -aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_not_euint8(bytes calldata a) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 result = ~aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_neg_euint16(bytes calldata a) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 result = -aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_not_euint16(bytes calldata a) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 result = ~aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_neg_euint32(bytes calldata a) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 result = -aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_not_euint32(bytes calldata a) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 result = ~aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_neg_euint64(bytes calldata a) public view returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 result = -aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_not_euint64(bytes calldata a) public view returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 result = ~aProc;
        return TFHE.decrypt(result);
    }
}
