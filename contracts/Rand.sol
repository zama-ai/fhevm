// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "../lib/TFHE.sol";

contract Rand {
    euint8 value8;
    euint16 value16;
    euint32 value32;

    function generate8() public {
        value8 = TFHE.randEuint8();
    }

    function generate16() public {
        value16 = TFHE.randEuint16();
    }

    function generate32() public {
        value32 = TFHE.randEuint32();
    }

    function get8() public view returns (uint8) {
        return TFHE.decrypt(value8);
    }

    function get16() public view returns (uint16) {
        return TFHE.decrypt(value16);
    }

    function get32() public view returns (uint32) {
        return TFHE.decrypt(value32);
    }

    // Must fail.
    function generate8InView() public view {
        TFHE.randEuint8();
    }

    // Must fail.
    function generate16InView() public view {
        TFHE.randEuint16();
    }

    // Must fail.
    function generate32InView() public view {
        TFHE.randEuint32();
    }
}
