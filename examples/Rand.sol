// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";

contract Rand {
    ebool public valueb;
    euint4 public value4;
    euint8 public value8;
    euint16 public value16;
    euint32 public value32;
    euint64 public value64;
    euint64 public value64Bounded;
    euint128 public value128;
    euint256 public value256;
    ebytes64 public value512;
    ebytes128 public value1024;
    ebytes256 public value2048;

    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
    }

    function generateBool() public {
        valueb = TFHE.randEbool();
        TFHE.allowThis(valueb);
    }

    function generate4() public {
        value4 = TFHE.randEuint4();
        TFHE.allowThis(value4);
    }

    function generate4UpperBound(uint8 upperBound) public {
        value4 = TFHE.randEuint4(upperBound);
        TFHE.allowThis(value4);
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

    function generate64UpperBound(uint64 upperBound) public {
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

    function generate128() public {
        value128 = TFHE.randEuint128();
        TFHE.allowThis(value128);
    }

    function generate128UpperBound(uint128 upperBound) public {
        value128 = TFHE.randEuint128(upperBound);
        TFHE.allowThis(value128);
    }

    function generate256() public {
        value256 = TFHE.randEuint256();
        TFHE.allowThis(value256);
    }

    function generate256UpperBound(uint256 upperBound) public {
        value256 = TFHE.randEuint256(upperBound);
        TFHE.allowThis(value256);
    }

    function generate512() public {
        value512 = TFHE.randEbytes64();
        TFHE.allowThis(value512);
    }

    function generate1024() public {
        value1024 = TFHE.randEbytes128();
        TFHE.allowThis(value1024);
    }

    function generate2048() public {
        value2048 = TFHE.randEbytes256();
        TFHE.allowThis(value2048);
    }
}
