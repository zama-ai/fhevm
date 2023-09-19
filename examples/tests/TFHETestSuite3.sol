// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity >=0.8.13 <0.8.20;

import "../../lib/TFHE.sol";

contract TFHETestSuite3 {
    function sub_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function div_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.div(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }
}
