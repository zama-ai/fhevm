// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "../payment/Payment.sol";

contract Reencrypt {
    ebool public xBool;
    euint4 public xUint4;
    euint8 public xUint8;
    euint16 public xUint16;
    euint32 public xUint32;
    euint64 public xUint64;
    eaddress public xAddress;
    ebytes256 public yBytes256;

    constructor() payable {
        Payment.depositForThis(msg.value);

        xBool = TFHE.asEbool(true);
        TFHE.allowThis(xBool);
        TFHE.allow(xBool, msg.sender);

        xUint4 = TFHE.asEuint4(4);
        TFHE.allowThis(xUint4);
        TFHE.allow(xUint4, msg.sender);

        xUint8 = TFHE.asEuint8(42);
        TFHE.allowThis(xUint8);
        TFHE.allow(xUint8, msg.sender);

        xUint16 = TFHE.asEuint16(16);
        TFHE.allowThis(xUint16);
        TFHE.allow(xUint16, msg.sender);

        xUint32 = TFHE.asEuint32(32);
        TFHE.allowThis(xUint32);
        TFHE.allow(xUint32, msg.sender);

        xUint64 = TFHE.asEuint64(18446744073709551600);
        TFHE.allowThis(xUint64);
        TFHE.allow(xUint64, msg.sender);

        xAddress = TFHE.asEaddress(0x8ba1f109551bD432803012645Ac136ddd64DBA72);
        TFHE.allowThis(xAddress);
        TFHE.allow(xAddress, msg.sender);
    }

    function setEBytes256(einput inputHandleEBytes256, bytes memory inputProofEBytes256) external {
        yBytes256 = TFHE.asEbytes256(inputHandleEBytes256, inputProofEBytes256);
        TFHE.allowThis(yBytes256);
        TFHE.allow(yBytes256, msg.sender);
    }
}
