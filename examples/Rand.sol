// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "../payment/Payment.sol";

contract Rand {
    euint8 public value8;
    euint16 public value16;
    euint32 public value32;
    euint64 public value64;
    euint64 public value64Bounded;

    constructor() payable {
        Payment.depositForThis(msg.value);
    }

    function generate8() public {
        value8 = TFHE.randEuint8();
        TFHE.allowThis(value8);
    }

    function generate8UpperBound(uint8 upperBound) public {
        value8 = TFHE.randEuint8(upperBound);
        TFHE.allowThis(value8);
    }

    function generate16() public {
        value16 = TFHE.randEuint16();
        TFHE.allowThis(value16);
    }

    function generate16UpperBound(uint16 upperBound) public {
        value16 = TFHE.randEuint16(upperBound);
        TFHE.allowThis(value16);
    }

    function generate32() public {
        value32 = TFHE.randEuint32();
        TFHE.allowThis(value32);
    }

    function generate32UpperBound(uint32 upperBound) public {
        value32 = TFHE.randEuint32(upperBound);
        TFHE.allowThis(value32);
    }

    function generate64() public {
        value64 = TFHE.randEuint64();
        TFHE.allowThis(value64);
    }

    function generate64UpperBound(uint32 upperBound) public {
        value64 = TFHE.randEuint64(upperBound);
        TFHE.allowThis(value64);
    }

    function generate64Reverting() public {
        try this.failingCall() {} catch {}
        value64Bounded = TFHE.randEuint64(1024);
        TFHE.allowThis(value64Bounded);
    }

    function failingCall() public {
        value64 = TFHE.randEuint64();
        TFHE.allowThis(value64);
        revert();
    }
}
