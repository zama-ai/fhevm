// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import { E2EFHEVMConfig } from "../E2EFHEVMConfigSepolia.sol";
import "fhevm/lib/TFHE.sol";

contract Div is E2EFHEVMConfig {
    euint4 public result4;
    euint8 public result8;
    euint16 public result16;
    euint32 public result32;
    euint64 public result64;
    euint128 public result128;
    euint256 public result256;

    function div4Scalar() public {
        result4 = TFHE.div(TFHE.asEuint4(9), 4);
        TFHE.allow(result4, address(this));
        TFHE.allow(result4, msg.sender);
    }

    function div8Scalar() public {
        result8 = TFHE.div(TFHE.asEuint8(137), 11);
        TFHE.allow(result8, address(this));
        TFHE.allow(result8, msg.sender);
    }

    function div16Scalar() public {
        result16 = TFHE.div(TFHE.asEuint16(50585), 313);
        TFHE.allow(result16, address(this));
        TFHE.allow(result16, msg.sender);
    }

    function div32Scalar() public {
        result32 = TFHE.div(TFHE.asEuint32(3294967296), 342);
        TFHE.allow(result32, address(this));
        TFHE.allow(result32, msg.sender);
    }

    function div64Scalar() public {
        result64 = TFHE.div(TFHE.asEuint64(6494967296), 342);
        TFHE.allow(result64, address(this));
        TFHE.allow(result64, msg.sender);
    }

    function div128Scalar() public {
        result128 = TFHE.div(TFHE.asEuint128(6494967296), 342);
        TFHE.allow(result128, address(this));
        TFHE.allow(result128, msg.sender);
    }

    function div256Scalar() public {
        result256 = TFHE.div(TFHE.asEuint256(6494967296), 342);
        TFHE.allow(result256, address(this));
        TFHE.allow(result256, msg.sender);
    }
}
