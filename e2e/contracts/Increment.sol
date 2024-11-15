// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import { E2EFHEVMConfig } from "./E2EFHEVMConfig.sol";
import "fhevm/lib/TFHE.sol";

contract Increment is E2EFHEVMConfig {
    euint8 public counter;

    constructor() {

        counter = TFHE.asEuint8(0);
        TFHE.allow(counter, address(this));
        TFHE.allow(counter, msg.sender);
    }

    function increment() public {
        counter = TFHE.add(counter, 1);
        TFHE.allow(counter, address(this));
        TFHE.allow(counter, msg.sender);
    }
}
