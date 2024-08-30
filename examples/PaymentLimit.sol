// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "../payment/Payment.sol";

contract PaymentLimit {
    constructor() payable {
        Payment.depositForThis(msg.value);
    }

    function wayunderBlockFHEGasLimit() external {
        // should pass if only tx in block
        euint64 x = TFHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 3; i++) {
            result = TFHE.mul(result, x);
        }
    }

    function underBlockFHEGasLimit() external {
        // should pass if only tx in block
        euint64 x = TFHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 15; i++) {
            result = TFHE.mul(result, x);
        }
    }

    function aboveBlockFHEGasLimit() external {
        // should revert due to exceeding block fheGas limit
        euint64 x = TFHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 16; i++) {
            result = TFHE.mul(result, x);
        }
    }
}
