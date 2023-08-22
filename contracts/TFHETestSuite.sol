// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "../lib/TFHE.sol";

contract TFHETestSuite {
    function addUint8(uint8 a, uint8 b) public view returns (uint8) {
        return TFHE.decrypt(TFHE.add(TFHE.asEuint8(a), TFHE.asEuint8(b)));
    }

    function mulUint8(uint8 a, uint8 b) public view returns (uint8) {
        return TFHE.decrypt(TFHE.mul(TFHE.asEuint8(a), TFHE.asEuint8(b)));
    }
}
