// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "../gateway/GatewayCaller.sol";

contract TestAsyncDecrypt is GatewayCaller {
    ebool xBool;
    euint4 xUint4;
    euint8 xUint8;
    euint16 xUint16;
    euint32 xUint32;
    euint64 xUint64;
    euint64 xUint64_2;
    euint64 xUint64_3;
    euint128 xUint128;
    eaddress xAddress;
    eaddress xAddress2;
    euint256 xUint256;

    bool public yBool;
    uint8 public yUint4;
    uint8 public yUint8;
    uint16 public yUint16;
    uint32 public yUint32;
    uint64 public yUint64;
    uint64 public yUint64_2;
    uint64 public yUint64_3;
    uint128 public yUint128;
    address public yAddress;
    address public yAddress2;
    uint256 public yUint256;
    bytes public yBytes64;
    bytes public yBytes128;
    bytes public yBytes256;

    uint256 public latestRequestID;

    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
        Gateway.setGateway(Gateway.defaultGatewayAddress());
        xBool = TFHE.asEbool(true);
        TFHE.allowThis(xBool);
        xUint4 = TFHE.asEuint4(4);
        TFHE.allowThis(xUint4);
        xUint8 = TFHE.asEuint8(42);
        TFHE.allowThis(xUint8);
        xUint16 = TFHE.asEuint16(16);
        TFHE.allowThis(xUint16);
        xUint32 = TFHE.asEuint32(32);
        TFHE.allowThis(xUint32);
        xUint64 = TFHE.asEuint64(18446744073709551600);
        TFHE.allowThis(xUint64);
        xUint64_2 = TFHE.asEuint64(76575465786);
        TFHE.allowThis(xUint64_2);
        xUint64_3 = TFHE.asEuint64(6400);
        TFHE.allowThis(xUint64_3);
        xUint128 = TFHE.asEuint128(1267650600228229401496703205443);
        TFHE.allowThis(xUint128);
        xUint256 = TFHE.asEuint256(27606985387162255149739023449108101809804435888681546220650096895197251);
        TFHE.allowThis(xUint256);
        xAddress = TFHE.asEaddress(0x8ba1f109551bD432803012645Ac136ddd64DBA72);
        TFHE.allowThis(xAddress);
        xAddress2 = TFHE.asEaddress(0xf48b8840387ba3809DAE990c930F3b4766A86ca3);
        TFHE.allowThis(xAddress2);
    }

    function requestBoolInfinite() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        Gateway.requestDecryption(cts, this.callbackBoolInfinite.selector, 0, block.timestamp + 100, false);
    }

    function callbackBoolInfinite(uint256 /*requestID*/, bool decryptedInput) public onlyGateway returns (bool) {
        uint256 i = 0;
        while (true) {
            i++;
        }
        yBool = decryptedInput;
        return yBool;
    }

    function requestBoolAboveDelay() public {
        // should revert
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        Gateway.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 2 days, false);
    }

    function requestBool() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        Gateway.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 100, false);
    }

    function requestBoolTrustless() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        uint256 requestID = Gateway.requestDecryption(
            cts,
            this.callbackBoolTrustless.selector,
            0,
            block.timestamp + 100,
            true
        );
        latestRequestID = requestID;
        saveRequestedHandles(requestID, cts);
    }

    function requestFakeBool() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000000);
        Gateway.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 100, false); // this should revert because previous ebool is not honestly obtained
    }

    function callbackBool(uint256, bool decryptedInput) public onlyGateway returns (bool) {
        yBool = decryptedInput;
        return yBool;
    }

    function callbackBoolTrustless(
        uint256 requestID,
        bool decryptedInput,
        bytes[] memory signatures
    ) public onlyGateway returns (bool) {
        require(latestRequestID == requestID, "wrong requestID passed by Gateway");
        uint256[] memory requestedHandles = loadRequestedHandles(latestRequestID);
        bool isKMSVerified = Gateway.verifySignatures(requestedHandles, signatures);
        require(isKMSVerified, "KMS did not verify this decryption result");
        yBool = decryptedInput;
        return yBool;
    }

    function requestUint4() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint4);
        Gateway.requestDecryption(cts, this.callbackUint4.selector, 0, block.timestamp + 100, false);
    }

    function requestFakeUint4() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000100);
        Gateway.requestDecryption(cts, this.callbackUint4.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    function callbackUint4(uint256, uint8 decryptedInput) public onlyGateway returns (uint8) {
        yUint4 = decryptedInput;
        return decryptedInput;
    }

    function requestUint8() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint8);
        Gateway.requestDecryption(cts, this.callbackUint8.selector, 0, block.timestamp + 100, false);
    }

    function requestFakeUint8() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000200);
        Gateway.requestDecryption(cts, this.callbackUint8.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    function callbackUint8(uint256, uint8 decryptedInput) public onlyGateway returns (uint8) {
        yUint8 = decryptedInput;
        return decryptedInput;
    }

    function requestUint16() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint16);
        Gateway.requestDecryption(cts, this.callbackUint16.selector, 0, block.timestamp + 100, false);
    }

    function requestFakeUint16() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000300);
        Gateway.requestDecryption(cts, this.callbackUint16.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    function callbackUint16(uint256, uint16 decryptedInput) public onlyGateway returns (uint16) {
        yUint16 = decryptedInput;
        return decryptedInput;
    }

    function requestUint32(uint32 input1, uint32 input2) public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint32);
        uint256 requestID = Gateway.requestDecryption(
            cts,
            this.callbackUint32.selector,
            0,
            block.timestamp + 100,
            false
        );
        addParamsUint256(requestID, input1);
        addParamsUint256(requestID, input2);
    }

    function requestFakeUint32() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000400);
        Gateway.requestDecryption(cts, this.callbackUint32.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    function callbackUint32(uint256 requestID, uint32 decryptedInput) public onlyGateway returns (uint32) {
        uint256[] memory params = getParamsUint256(requestID);
        unchecked {
            uint32 result = uint32(params[0]) + uint32(params[1]) + decryptedInput;
            yUint32 = result;
            return result;
        }
    }

    function requestUint64() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint64);
        Gateway.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100, false);
    }

    function requestFakeUint64() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000500);
        Gateway.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    function requestUint64NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        euint64 inputNonTrivial = TFHE.asEuint64(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputNonTrivial);
        Gateway.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100, false);
    }

    function callbackUint64(uint256, uint64 decryptedInput) public onlyGateway returns (uint64) {
        yUint64 = decryptedInput;
        return decryptedInput;
    }

    function requestUint128() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint128);
        Gateway.requestDecryption(cts, this.callbackUint128.selector, 0, block.timestamp + 100, false);
    }

    function requestUint128NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        euint128 inputNonTrivial = TFHE.asEuint128(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputNonTrivial);
        Gateway.requestDecryption(cts, this.callbackUint128.selector, 0, block.timestamp + 100, false);
    }

    function callbackUint128(uint256, uint128 decryptedInput) public onlyGateway returns (uint128) {
        yUint128 = decryptedInput;
        return decryptedInput;
    }

    function requestUint256() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint256);
        Gateway.requestDecryption(cts, this.callbackUint256.selector, 0, block.timestamp + 100, false);
    }

    function requestUint256NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        euint256 inputNonTrivial = TFHE.asEuint256(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputNonTrivial);
        Gateway.requestDecryption(cts, this.callbackUint256.selector, 0, block.timestamp + 100, false);
    }

    function callbackUint256(uint256, uint256 decryptedInput) public onlyGateway returns (uint256) {
        yUint256 = decryptedInput;
        return decryptedInput;
    }

    function requestEbytes64NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        ebytes64 inputNonTrivial = TFHE.asEbytes64(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputNonTrivial);
        Gateway.requestDecryption(cts, this.callbackBytes64.selector, 0, block.timestamp + 100, false);
    }

    function requestEbytes64Trivial(bytes calldata value) public {
        ebytes64 inputTrivial = TFHE.asEbytes64(TFHE.padToBytes64(value));
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputTrivial);
        Gateway.requestDecryption(cts, this.callbackBytes64.selector, 0, block.timestamp + 100, false);
    }

    function callbackBytes64(uint256, bytes calldata decryptedInput) public onlyGateway returns (bytes memory) {
        yBytes64 = decryptedInput;
        return decryptedInput;
    }

    function requestEbytes128NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        ebytes128 inputNonTrivial = TFHE.asEbytes128(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputNonTrivial);
        Gateway.requestDecryption(cts, this.callbackBytes128.selector, 0, block.timestamp + 100, false);
    }

    function requestEbytes128Trivial(bytes calldata value) public {
        ebytes128 inputTrivial = TFHE.asEbytes128(TFHE.padToBytes128(value));
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputTrivial);
        Gateway.requestDecryption(cts, this.callbackBytes128.selector, 0, block.timestamp + 100, false);
    }

    function callbackBytes128(uint256, bytes calldata decryptedInput) public onlyGateway returns (bytes memory) {
        yBytes128 = decryptedInput;
        return decryptedInput;
    }

    function requestEbytes256Trivial(bytes calldata value) public {
        ebytes256 inputTrivial = TFHE.asEbytes256(TFHE.padToBytes256(value));
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputTrivial);
        Gateway.requestDecryption(cts, this.callbackBytes256.selector, 0, block.timestamp + 100, false);
    }

    function requestEbytes256NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        ebytes256 inputNonTrivial = TFHE.asEbytes256(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputNonTrivial);
        Gateway.requestDecryption(cts, this.callbackBytes256.selector, 0, block.timestamp + 100, false);
    }

    function callbackBytes256(uint256, bytes calldata decryptedInput) public onlyGateway returns (bytes memory) {
        yBytes256 = decryptedInput;
        return decryptedInput;
    }

    function requestAddress() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xAddress);
        Gateway.requestDecryption(cts, this.callbackAddress.selector, 0, block.timestamp + 100, false);
    }

    function requestSeveralAddresses() public {
        uint256[] memory cts = new uint256[](2);
        cts[0] = Gateway.toUint256(xAddress);
        cts[1] = Gateway.toUint256(xAddress2);
        Gateway.requestDecryption(cts, this.callbackAddresses.selector, 0, block.timestamp + 100, false);
    }

    function callbackAddresses(
        uint256 /*requestID*/,
        address decryptedInput1,
        address decryptedInput2
    ) public onlyGateway returns (address) {
        yAddress = decryptedInput1;
        yAddress2 = decryptedInput2;
        return decryptedInput1;
    }

    function requestFakeAddress() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000700);
        Gateway.requestDecryption(cts, this.callbackAddress.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    function callbackAddress(uint256, address decryptedInput) public onlyGateway returns (address) {
        yAddress = decryptedInput;
        return decryptedInput;
    }

    function requestMixed(uint32 input1, uint32 input2) public {
        uint256[] memory cts = new uint256[](10);
        cts[0] = Gateway.toUint256(xBool);
        cts[1] = Gateway.toUint256(xBool);
        cts[2] = Gateway.toUint256(xUint4);
        cts[3] = Gateway.toUint256(xUint8);
        cts[4] = Gateway.toUint256(xUint16);
        cts[5] = Gateway.toUint256(xUint32);
        cts[6] = Gateway.toUint256(xUint64);
        cts[7] = Gateway.toUint256(xUint64);
        cts[8] = Gateway.toUint256(xUint64);
        cts[9] = Gateway.toUint256(xAddress);
        uint256 requestID = Gateway.requestDecryption(
            cts,
            this.callbackMixed.selector,
            0,
            block.timestamp + 100,
            false
        );
        addParamsUint256(requestID, input1);
        addParamsUint256(requestID, input2);
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
    ) public onlyGateway returns (uint8) {
        yBool = decBool_1;
        require(decBool_1 == decBool_2, "Wrong decryption");
        yUint4 = decUint4;
        yUint8 = decUint8;
        yUint16 = decUint16;
        uint256[] memory params = getParamsUint256(requestID);
        unchecked {
            uint32 result = uint32(params[0]) + uint32(params[1]) + decUint32;
            yUint32 = result;
        }
        yUint64 = decUint64_1;
        require(decUint64_1 == decUint64_2 && decUint64_2 == decUint64_3, "Wrong decryption");
        yAddress = decAddress;
        return yUint4;
    }

    function requestMixedBytes256(einput inputHandle, bytes calldata inputProof) public {
        ebytes256 xBytes256 = TFHE.asEbytes256(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](4);
        cts[0] = Gateway.toUint256(xBool);
        cts[1] = Gateway.toUint256(xAddress);
        cts[2] = Gateway.toUint256(xBytes256);
        ebytes64 input64Bytes = TFHE.asEbytes64(TFHE.padToBytes64(hex"aaff42"));
        cts[3] = Gateway.toUint256(input64Bytes);
        Gateway.requestDecryption(cts, this.callbackMixedBytes256.selector, 0, block.timestamp + 100, false);
    }

    function callbackMixedBytes256(
        uint256,
        bool decBool,
        address decAddress,
        bytes memory bytesRes,
        bytes memory bytesRes2
    ) public onlyGateway {
        yBool = decBool;
        yAddress = decAddress;
        yBytes256 = bytesRes;
        yBytes64 = bytesRes2;
    }

    function requestEbytes256NonTrivialTrustless(einput inputHandle, bytes calldata inputProof) public {
        ebytes256 inputNonTrivial = TFHE.asEbytes256(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputNonTrivial);
        uint256 requestID = Gateway.requestDecryption(
            cts,
            this.callbackBytes256Trustless.selector,
            0,
            block.timestamp + 100,
            true
        );
        latestRequestID = requestID;
        saveRequestedHandles(requestID, cts);
    }

    function callbackBytes256Trustless(
        uint256 requestID,
        bytes calldata decryptedInput,
        bytes[] memory signatures
    ) public onlyGateway returns (bytes memory) {
        require(latestRequestID == requestID, "wrong requestID passed by Gateway");
        uint256[] memory requestedHandles = loadRequestedHandles(latestRequestID);
        bool isKMSVerified = Gateway.verifySignatures(requestedHandles, signatures);
        require(isKMSVerified, "KMS did not verify this decryption result");
        yBytes256 = decryptedInput;
        return decryptedInput;
    }

    function requestMixedBytes256Trustless(einput inputHandle, bytes calldata inputProof) public {
        ebytes256 xBytes256 = TFHE.asEbytes256(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](3);
        cts[0] = Gateway.toUint256(xBool);
        cts[1] = Gateway.toUint256(xBytes256);
        cts[2] = Gateway.toUint256(xAddress);
        Gateway.requestDecryption(cts, this.callbackMixedBytes256Trustless.selector, 0, block.timestamp + 100, true);
        uint256 requestID = Gateway.requestDecryption(
            cts,
            this.callbackMixedBytes256Trustless.selector,
            0,
            block.timestamp + 100,
            true
        );
        latestRequestID = requestID;
        saveRequestedHandles(requestID, cts);
    }

    function callbackMixedBytes256Trustless(
        uint256 requestID,
        bool decBool,
        bytes memory bytesRes,
        address decAddress,
        bytes[] memory signatures
    ) public onlyGateway {
        require(latestRequestID == requestID, "wrong requestID passed by Gateway");
        uint256[] memory requestedHandles = loadRequestedHandles(latestRequestID);
        bool isKMSVerified = Gateway.verifySignatures(requestedHandles, signatures);
        require(isKMSVerified, "KMS did not verify this decryption result");
        yBool = decBool;
        yAddress = decAddress;
        yBytes256 = bytesRes;
    }
}
