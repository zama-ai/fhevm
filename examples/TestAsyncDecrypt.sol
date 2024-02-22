// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "../lib/TFHE.sol";
import "../oracle/OracleCaller.sol";

contract TestAsyncDecrypt is OracleCaller {
    euint32 x;
    uint32 public y;

    constructor() {
        x = TFHE.asEuint32(32);
    }

    function request(uint32 input1, uint32 input2) public {
        euint32[] memory cts = new euint32[](1);
        cts[0] = x;
        uint256 requestID = Oracle.requestDecryption(cts, this.callback.selector, 0, block.timestamp + 100);
        addParamsUint(requestID, input1);
        addParamsUint(requestID, input2);
    }

    // Transfers an encrypted amount from the message sender address to the `to` address.
    function callback(uint256 requestID, uint32 decryptedInput) public onlyOracle returns (uint32) {
        uint256[] memory params = getParamsUint(requestID);
        unchecked {
            uint32 result = uint32(params[0]) + uint32(params[1]) + decryptedInput;
            y = result;
            return result;
        }
    }
}
