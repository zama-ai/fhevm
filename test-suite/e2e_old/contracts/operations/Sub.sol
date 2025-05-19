// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import { E2EFHEVMConfig } from "../E2EFHEVMConfigSepolia.sol";
import "fhevm/lib/TFHE.sol";

contract Sub is E2EFHEVMConfig {
    euint4 public result4;
    euint8 public result8;
    euint16 public result16;
    euint32 public result32;
    euint64 public result64;
    euint128 public result128;
    euint256 public result256;

    function sub4() public {
        result4 = TFHE.sub(TFHE.asEuint4(2), TFHE.asEuint4(1));
        TFHE.allow(result4, address(this));
        TFHE.allow(result4, msg.sender);
    }

    function sub4Scalar() public {
        result4 = TFHE.sub(TFHE.asEuint4(2), 1);
        TFHE.allow(result4, address(this));
        TFHE.allow(result4, msg.sender);
    }

    function sub8() public {
        result8 = TFHE.sub(TFHE.asEuint8(4), TFHE.asEuint8(3));
        TFHE.allow(result8, address(this));
        TFHE.allow(result8, msg.sender);
    }

    function sub8Scalar() public {
        result8 = TFHE.sub(TFHE.asEuint8(4), 3);
        TFHE.allow(result8, address(this));
        TFHE.allow(result8, msg.sender);
    }

    function sub16() public {
        result16 = TFHE.sub(TFHE.asEuint16(8), TFHE.asEuint16(7));
        TFHE.allow(result16, address(this));
        TFHE.allow(result16, msg.sender);
    }

    function sub16Scalar() public {
        result16 = TFHE.sub(TFHE.asEuint16(8), 7);
        TFHE.allow(result16, address(this));
        TFHE.allow(result16, msg.sender);
    }

    function sub32() public {
        result32 = TFHE.sub(TFHE.asEuint32(16), TFHE.asEuint32(15));
        TFHE.allow(result32, address(this));
        TFHE.allow(result32, msg.sender);
    }

    function sub32Scalar() public {
        result32 = TFHE.sub(TFHE.asEuint32(16), 15);
        TFHE.allow(result32, address(this));
        TFHE.allow(result32, msg.sender);
    }

    function sub64() public {
        result64 = TFHE.sub(TFHE.asEuint64(32), TFHE.asEuint64(31));
        TFHE.allow(result64, address(this));
        TFHE.allow(result64, msg.sender);
    }

    function sub64Scalar() public {
        result64 = TFHE.sub(TFHE.asEuint64(32), 31);
        TFHE.allow(result64, address(this));
        TFHE.allow(result64, msg.sender);
    }

    function sub128() public {
        result128 = TFHE.sub(TFHE.asEuint128(64), TFHE.asEuint128(63));
        TFHE.allow(result128, address(this));
        TFHE.allow(result128, msg.sender);
    }

    function sub128Scalar() public {
        result128 = TFHE.sub(TFHE.asEuint128(64), 63);
        TFHE.allow(result128, address(this));
        TFHE.allow(result128, msg.sender);
    }

    function sub256() public {
        result256 = TFHE.sub(TFHE.asEuint256(128), TFHE.asEuint256(127));
        TFHE.allow(result256, address(this));
        TFHE.allow(result256, msg.sender);
    }

    function sub256Scalar() public {
        result256 = TFHE.sub(TFHE.asEuint256(128), 127);
        TFHE.allow(result256, address(this));
        TFHE.allow(result256, msg.sender);
    }
}
