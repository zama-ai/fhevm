// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "../lib/TFHE.sol";
import "../oracle/OracleCaller.sol";

contract TestAsyncDecrypt is OracleCaller {
    ebool xBool;
    euint4 xUint4;
    euint8 xUint8;
    euint16 xUint16;
    euint32 xUint32;
    euint64 xUint64;

    bool public yBool;
    uint8 public yUint4;
    uint8 public yUint8;
    uint16 public yUint16;
    uint32 public yUint32;
    uint64 public yUint64;

    constructor() {
        xBool = TFHE.asEbool(true);
        xUint4 = TFHE.asEuint4(4);
        xUint8 = TFHE.asEuint8(42);
        xUint16 = TFHE.asEuint16(16);
        xUint32 = TFHE.asEuint32(32);
        xUint64 = TFHE.asEuint64(64);
    }

    function requestBool() public {
        ebool[] memory cts = new ebool[](1);
        cts[0] = xBool;
        Oracle.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 100);
    }

    function callbackBool(uint256 /*requestID*/, bool decryptedInput) public onlyOracle returns (bool) {
        yBool = decryptedInput;
        return yBool;
    }

    function requestUint4() public {
        euint4[] memory cts = new euint4[](1);
        cts[0] = xUint4;
        Oracle.requestDecryption(cts, this.callbackUint4.selector, 0, block.timestamp + 100);
    }

    function callbackUint4(uint256 /*requestID*/, uint8 decryptedInput) public onlyOracle returns (uint8) {
        yUint4 = decryptedInput;
        return decryptedInput;
    }

    function requestUint8() public {
        euint8[] memory cts = new euint8[](1);
        cts[0] = xUint8;
        Oracle.requestDecryption(cts, this.callbackUint8.selector, 0, block.timestamp + 100);
    }

    function callbackUint8(uint256 /*requestID*/, uint8 decryptedInput) public onlyOracle returns (uint8) {
        yUint8 = decryptedInput;
        return decryptedInput;
    }

    function requestUint16() public {
        euint16[] memory cts = new euint16[](1);
        cts[0] = xUint16;
        Oracle.requestDecryption(cts, this.callbackUint16.selector, 0, block.timestamp + 100);
    }

    function callbackUint16(uint256 /*requestID*/, uint16 decryptedInput) public onlyOracle returns (uint16) {
        yUint16 = decryptedInput;
        return decryptedInput;
    }

    function requestUint32(uint32 input1, uint32 input2) public {
        euint32[] memory cts = new euint32[](1);
        cts[0] = xUint32;
        uint256 requestID = Oracle.requestDecryption(cts, this.callbackUint32.selector, 0, block.timestamp + 100);
        addParamsUint(requestID, input1);
        addParamsUint(requestID, input2);
    }

    function callbackUint32(uint256 requestID, uint32 decryptedInput) public onlyOracle returns (uint32) {
        uint256[] memory params = getParamsUint(requestID);
        unchecked {
            uint32 result = uint32(params[0]) + uint32(params[1]) + decryptedInput;
            yUint32 = result;
            return result;
        }
    }

    function requestUint64() public {
        euint64[] memory cts = new euint64[](1);
        cts[0] = xUint64;
        Oracle.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100);
    }

    function callbackUint64(uint256 /*requestID*/, uint64 decryptedInput) public onlyOracle returns (uint64) {
        yUint64 = decryptedInput;
        return decryptedInput;
    }
}
