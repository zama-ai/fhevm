// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";

contract Rand {
    euint8 public value8;
    euint16 public value16;
    euint32 public value32;
    euint64 public value64;
    euint64 public value64Bounded;

    function generate8() public {
        value8 = TFHE.randEuint8();
        TFHE.allow(value8, address(this));
    }

    function generate8UpperBound(uint8 upperBound) public {
        value8 = TFHE.randEuint8(upperBound);
        TFHE.allow(value8, address(this));
    }

    function generate16() public {
        value16 = TFHE.randEuint16();
        TFHE.allow(value16, address(this));
    }

    function generate16UpperBound(uint16 upperBound) public {
        value16 = TFHE.randEuint16(upperBound);
        TFHE.allow(value16, address(this));
    }

    function generate32() public {
        value32 = TFHE.randEuint32();
        TFHE.allow(value32, address(this));
    }

    function generate32UpperBound(uint32 upperBound) public {
        value32 = TFHE.randEuint32(upperBound);
        TFHE.allow(value32, address(this));
    }

    function generate64() public {
        value64 = TFHE.randEuint64();
        TFHE.allow(value64, address(this));
    }

    function generate64UpperBound(uint32 upperBound) public {
        value64 = TFHE.randEuint64(upperBound);
        TFHE.allow(value64, address(this));
    }

    function generate64Reverting() public {
        try this.failingCall() {} catch {}
        value64Bounded = TFHE.randEuint64(1024);
        TFHE.allow(value64Bounded, address(this));
    }

    function failingCall() public {
        value64 = TFHE.randEuint64();
        TFHE.allow(value64, address(this));
        revert();
    }
}
