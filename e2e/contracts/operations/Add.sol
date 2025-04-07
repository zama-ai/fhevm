// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import { E2EFHEVMConfig } from "../E2EFHEVMConfigSepolia.sol";
import "fhevm/lib/TFHE.sol";

contract Add is E2EFHEVMConfig {
    euint4 public result4;
    euint8 public result8;
    euint16 public result16;
    euint32 public result32;
    euint64 public result64;
    euint128 public result128;
    euint256 public result256;

    function add4() public {
        result4 = TFHE.add(TFHE.asEuint4(1), TFHE.asEuint4(2));
        TFHE.allow(result4, address(this));
        TFHE.allow(result4, msg.sender);
    }

    function add4Scalar() public {
        result4 = TFHE.add(TFHE.asEuint4(1), 2);
        TFHE.allow(result4, address(this));
        TFHE.allow(result4, msg.sender);
    }

    function add8() public {
        result8 = TFHE.add(TFHE.asEuint8(1), TFHE.asEuint8(2));
        TFHE.allow(result8, address(this));
        TFHE.allow(result8, msg.sender);
    }

    function add8Scalar() public {
        result8 = TFHE.add(TFHE.asEuint8(1), 2);
        TFHE.allow(result8, address(this));
        TFHE.allow(result8, msg.sender);
    }

    function add16() public {
        result16 = TFHE.add(TFHE.asEuint16(1), TFHE.asEuint16(2));
        TFHE.allow(result16, address(this));
        TFHE.allow(result16, msg.sender);
    }

    function add16Scalar() public {
        result16 = TFHE.add(TFHE.asEuint16(1), 2);
        TFHE.allow(result16, address(this));
        TFHE.allow(result16, msg.sender);
    }

    function add32() public {
        result32 = TFHE.add(TFHE.asEuint32(1), TFHE.asEuint32(2));
        TFHE.allow(result32, address(this));
        TFHE.allow(result32, msg.sender);
    }

    function add32Scalar() public {
        result32 = TFHE.add(TFHE.asEuint32(1), 2);
        TFHE.allow(result32, address(this));
        TFHE.allow(result32, msg.sender);
    }

    function add64() public {
        result64 = TFHE.add(TFHE.asEuint64(1), TFHE.asEuint64(2));
        TFHE.allow(result64, address(this));
        TFHE.allow(result64, msg.sender);
    }

    function add64Scalar() public {
        result64 = TFHE.add(TFHE.asEuint64(1), 2);
        TFHE.allow(result64, address(this));
        TFHE.allow(result64, msg.sender);
    }

    function add128() public {
        result128 = TFHE.add(TFHE.asEuint128(1), TFHE.asEuint128(2));
        TFHE.allow(result128, address(this));
        TFHE.allow(result128, msg.sender);
    }

    function add128Scalar() public {
        result128 = TFHE.add(TFHE.asEuint128(1), 2);
        TFHE.allow(result128, address(this));
        TFHE.allow(result128, msg.sender);
    }

    function add256() public {
        result256 = TFHE.add(TFHE.asEuint256(1), TFHE.asEuint256(2));
        TFHE.allow(result256, address(this));
        TFHE.allow(result256, msg.sender);
    }

    function add256Scalar() public {
        result256 = TFHE.add(TFHE.asEuint256(1), 2);
        TFHE.allow(result256, address(this));
        TFHE.allow(result256, msg.sender);
    }
}
