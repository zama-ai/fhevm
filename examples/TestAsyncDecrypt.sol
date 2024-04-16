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
    euint64 xUint64_2;
    euint64 xUint64_3;
    eaddress xAddress;
    eaddress xAddress2;

    bool public yBool;
    uint8 public yUint4;
    uint8 public yUint8;
    uint16 public yUint16;
    uint32 public yUint32;
    uint64 public yUint64;
    uint64 public yUint64_2;
    uint64 public yUint64_3;
    address public yAddress;
    address public yAddress2;

    constructor() {
        xBool = TFHE.asEbool(true);
        xUint4 = TFHE.asEuint4(4);
        xUint8 = TFHE.asEuint8(42);
        xUint16 = TFHE.asEuint16(16);
        xUint32 = TFHE.asEuint32(32);
        xUint64 = TFHE.asEuint64(18446744073709551600);
        xUint64_2 = TFHE.asEuint64(76575465786);
        xUint64_3 = TFHE.asEuint64(6400);
        xAddress = TFHE.asEaddress(0x8ba1f109551bD432803012645Ac136ddd64DBA72);
        xAddress2 = TFHE.asEaddress(0xf48b8840387ba3809DAE990c930F3b4766A86ca3);
    }

    function requestBoolInfinite() public {
        ebool[] memory cts = new ebool[](1);
        cts[0] = xBool;
        Oracle.requestDecryption(cts, this.callbackBoolInfinite.selector, 0, block.timestamp + 100);
    }

    function callbackBoolInfinite(uint256 /*requestID*/, bool decryptedInput) public onlyOracle returns (bool) {
        uint256 i = 0;
        while (1 == 1) {
            i++;
        }
        yBool = decryptedInput;
        return yBool;
    }

    function requestBoolInfinite() public {
        ebool[] memory cts = new ebool[](1);
        cts[0] = xBool;
        Oracle.requestDecryption(cts, this.callbackBoolInfinite.selector, 0, block.timestamp + 100);
    }

    function callbackBoolInfinite(uint256 /*requestID*/, bool decryptedInput) public onlyOracle returns (bool) {
        uint256 i = 0;
        while (1 == 1) {
            i++;
        }
        yBool = decryptedInput;
        return yBool;
    }

    function requestBoolAboveDelay() public {
        // should revert
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Oracle.toCiphertext(xBool);
        Oracle.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 2 days);
    }

    function requestBool() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Oracle.toCiphertext(xBool);
        Oracle.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 100);
    }

    function requestFakeBool() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Ciphertext(42, CiphertextType.EBOOL);
        Oracle.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 100); // this should revert because previous ebool is not honestly obtained
    }

    function callbackBool(uint256, bool decryptedInput) public onlyOracle returns (bool) {
        yBool = decryptedInput;
        return yBool;
    }

    function requestUint4() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Oracle.toCiphertext(xUint4);
        Oracle.requestDecryption(cts, this.callbackUint4.selector, 0, block.timestamp + 100);
    }

    function requestFakeUint4() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Ciphertext(42, CiphertextType.EUINT4);
        Oracle.requestDecryption(cts, this.callbackUint4.selector, 0, block.timestamp + 100); // this should revert because previous handle is not honestly obtained
    }

    function callbackUint4(uint256, uint8 decryptedInput) public onlyOracle returns (uint8) {
        yUint4 = decryptedInput;
        return decryptedInput;
    }

    function requestUint8() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Oracle.toCiphertext(xUint8);
        Oracle.requestDecryption(cts, this.callbackUint8.selector, 0, block.timestamp + 100);
    }

    function requestFakeUint8() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Ciphertext(42, CiphertextType.EUINT8);
        Oracle.requestDecryption(cts, this.callbackUint8.selector, 0, block.timestamp + 100); // this should revert because previous handle is not honestly obtained
    }

    function callbackUint8(uint256, uint8 decryptedInput) public onlyOracle returns (uint8) {
        yUint8 = decryptedInput;
        return decryptedInput;
    }

    function requestUint16() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Oracle.toCiphertext(xUint16);
        Oracle.requestDecryption(cts, this.callbackUint16.selector, 0, block.timestamp + 100);
    }

    function requestFakeUint16() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Ciphertext(42, CiphertextType.EUINT16);
        Oracle.requestDecryption(cts, this.callbackUint16.selector, 0, block.timestamp + 100); // this should revert because previous handle is not honestly obtained
    }

    function callbackUint16(uint256, uint16 decryptedInput) public onlyOracle returns (uint16) {
        yUint16 = decryptedInput;
        return decryptedInput;
    }

    function requestUint32(uint32 input1, uint32 input2) public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Oracle.toCiphertext(xUint32);
        uint256 requestID = Oracle.requestDecryption(cts, this.callbackUint32.selector, 0, block.timestamp + 100);
        addParamsUint(requestID, input1);
        addParamsUint(requestID, input2);
    }

    function requestFakeUint32() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Ciphertext(42, CiphertextType.EUINT32);
        Oracle.requestDecryption(cts, this.callbackUint32.selector, 0, block.timestamp + 100); // this should revert because previous handle is not honestly obtained
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
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Oracle.toCiphertext(xUint64);
        Oracle.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100);
    }

    function requestFakeUint64() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Ciphertext(42, CiphertextType.EUINT64);
        Oracle.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100); // this should revert because previous handle is not honestly obtained
    }

    function callbackUint64(uint256, uint64 decryptedInput) public onlyOracle returns (uint64) {
        yUint64 = decryptedInput;
        return decryptedInput;
    }

    function requestAddress() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Oracle.toCiphertext(xAddress);
        Oracle.requestDecryption(cts, this.callbackAddress.selector, 0, block.timestamp + 100);
    }

    function requestSeveralAddresses() public {
        eaddress[] memory cts = new eaddress[](2);
        cts[0] = xAddress;
        cts[1] = xAddress2;
        Oracle.requestDecryption(cts, this.callbackAddresses.selector, 0, block.timestamp + 100);
    }

    function requestFakeAddress() public {
        Ciphertext[] memory cts = new Ciphertext[](1);
        cts[0] = Ciphertext(42, CiphertextType.EADDRESS);
        Oracle.requestDecryption(cts, this.callbackAddress.selector, 0, block.timestamp + 100); // this should revert because previous handle is not honestly obtained
    }

    function callbackAddress(uint256, address decryptedInput) public onlyOracle returns (address) {
        yAddress = decryptedInput;
        return decryptedInput;
    }

    function requestMixed(uint32 input1, uint32 input2) public {
        Ciphertext[] memory cts = new Ciphertext[](10);
        cts[0] = Oracle.toCiphertext(xBool);
        cts[1] = Oracle.toCiphertext(xBool);
        cts[2] = Oracle.toCiphertext(xUint4);
        cts[3] = Oracle.toCiphertext(xUint8);
        cts[4] = Oracle.toCiphertext(xUint16);
        cts[5] = Oracle.toCiphertext(xUint32);
        cts[6] = Oracle.toCiphertext(xUint64);
        cts[7] = Oracle.toCiphertext(xUint64);
        cts[8] = Oracle.toCiphertext(xUint64);
        cts[9] = Oracle.toCiphertext(xAddress);
        uint256 requestID = Oracle.requestDecryption(cts, this.callbackMixed.selector, 0, block.timestamp + 100);
        addParamsUint(requestID, input1);
        addParamsUint(requestID, input2);
    }

    function callbackMixed(
        uint256 requestID,
        bool decBool_1,
        bool decBool_2,
        uint8 decUint4,
        uint8 decUint8,
        uint16 decUint16,
        uint32 decUint32,
        uint64 decUint64_1,
        uint64 decUint64_2,
        uint64 decUint64_3,
        address decAddress
    ) public onlyOracle returns (uint8) {
        yBool = decBool_1;
        require(decBool_1 == decBool_2, "Wrong decryption");
        yUint4 = decUint4;
        yUint8 = decUint8;
        yUint16 = decUint16;
        uint256[] memory params = getParamsUint(requestID);
        unchecked {
            uint32 result = uint32(params[0]) + uint32(params[1]) + decUint32;
            yUint32 = result;
        }
        yUint64 = decUint64_1;
        require(decUint64_1 == decUint64_2 && decUint64_2 == decUint64_3, "Wrong decryption");
        yAddress = decAddress;
        return yUint4;
    }

    function requestMixed(uint32 input1, uint32 input2) public {
        Ciphertext[] memory cts = new Ciphertext[](10);
        cts[0] = Oracle.toCiphertext(xBool);
        cts[1] = Oracle.toCiphertext(xBool);
        cts[2] = Oracle.toCiphertext(xUint4);
        cts[3] = Oracle.toCiphertext(xUint8);
        cts[4] = Oracle.toCiphertext(xUint16);
        cts[5] = Oracle.toCiphertext(xUint32);
        cts[6] = Oracle.toCiphertext(xUint64);
        cts[7] = Oracle.toCiphertext(xUint64);
        cts[8] = Oracle.toCiphertext(xUint64);
        cts[9] = Oracle.toCiphertext(xAddress);
        uint256 requestID = Oracle.requestDecryption(cts, this.callbackMixed.selector, 0, block.timestamp + 100);
        addParamsUint(requestID, input1);
        addParamsUint(requestID, input2);
    }

    function callbackMixed(
        uint256 requestID,
        bool decBool_1,
        bool decBool_2,
        uint8 decUint4,
        uint8 decUint8,
        uint16 decUint16,
        uint32 decUint32,
        uint64 decUint64_1,
        uint64 decUint64_2,
        uint64 decUint64_3,
        address decAddress
    ) public onlyOracle returns (uint8) {
        yBool = decBool_1;
        require(decBool_1 == decBool_2, "Wrong decryption");
        yUint4 = decUint4;
        yUint8 = decUint8;
        yUint16 = decUint16;
        uint256[] memory params = getParamsUint(requestID);
        unchecked {
            uint32 result = uint32(params[0]) + uint32(params[1]) + decUint32;
            yUint32 = result;
        }
        yUint64 = decUint64_1;
        require(decUint64_1 == decUint64_2 && decUint64_2 == decUint64_3, "Wrong decryption");
        yAddress = decAddress;
        return yUint4;
    }

    function callbackAddresses(
        uint256 /*requestID*/,
        address decryptedInput1,
        address decryptedInput2
    ) public onlyOracle returns (address) {
        yAddress = decryptedInput1;
        yAddress2 = decryptedInput2;
        return decryptedInput1;
    }

    function requestSeveralUint64WithDuplicates() public {
        euint64[] memory cts = new euint64[](4);
        cts[0] = xUint64;
        cts[1] = xUint64_2;
        cts[2] = xUint64_3;
        cts[3] = xUint64_2;
        Oracle.requestDecryption(cts, this.callbackSeveralUint64WithDuplicates.selector, 0, block.timestamp + 100);
    }

    function callbackSeveralUint64WithDuplicates(
        uint256 /*requestID*/,
        uint64 decryptedInput,
        uint64 /*decryptedInput_2*/,
        uint64 decryptedInput_3,
        uint64 decryptedInput_4
    ) public onlyOracle returns (uint64) {
        yUint64 = decryptedInput;
        yUint64_2 = decryptedInput_4;
        yUint64_3 = decryptedInput_3;
        return decryptedInput;
    }
}
