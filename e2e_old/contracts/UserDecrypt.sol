// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import { E2EFHEVMConfig } from "./E2EFHEVMConfigSepolia.sol";
import "fhevm/lib/TFHE.sol";

contract UserDecrypt is E2EFHEVMConfig {
    ebool public resultBool;
    euint4 public result4;
    euint8 public result8;
    euint16 public result16;
    euint32 public result32;
    euint64 public result64;
    euint128 public result128;
    euint256 public result256;
    ebytes64 public resultEbytes64;
    ebytes128 public resultEbytes128;
    ebytes256 public resultEbytes256;

    constructor() {
        resultBool = TFHE.asEbool(true);
        TFHE.allowThis(resultBool);
        TFHE.allow(resultBool, msg.sender);

        result4 = TFHE.asEuint4(2);
        TFHE.allowThis(result4);
        TFHE.allow(result4, msg.sender);

        result8 = TFHE.asEuint8(4);
        TFHE.allowThis(result8);
        TFHE.allow(result8, msg.sender);

        result16 = TFHE.asEuint16(8);
        TFHE.allowThis(result16);
        TFHE.allow(result16, msg.sender);

        result32 = TFHE.asEuint32(16);
        TFHE.allowThis(result32);
        TFHE.allow(result32, msg.sender);

        result64 = TFHE.asEuint64(32);
        TFHE.allowThis(result64);
        TFHE.allow(result64, msg.sender);

        result128 = TFHE.asEuint128(64);
        TFHE.allowThis(result128);
        TFHE.allow(result128, msg.sender);

        result256 = TFHE.asEuint256(128);
        TFHE.allowThis(result256);
        TFHE.allow(result256, msg.sender);

        resultEbytes64 = TFHE.asEbytes64(TFHE.padToBytes64("0x100"));
        TFHE.allowThis(resultEbytes64);
        TFHE.allow(resultEbytes64, msg.sender);

        resultEbytes128 = TFHE.asEbytes128(TFHE.padToBytes128("0x200"));
        TFHE.allowThis(resultEbytes128);
        TFHE.allow(resultEbytes128, msg.sender);

        resultEbytes256 = TFHE.asEbytes256(TFHE.padToBytes256("0x300"));
        TFHE.allowThis(resultEbytes256);
        TFHE.allow(resultEbytes256, msg.sender);
    }
}
