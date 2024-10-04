// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "../gateway/GatewayCaller.sol";

// Contract for testing asynchronous decryption using the Gateway
contract TestAsyncDecrypt is GatewayCaller {
    // Encrypted state variables
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

    // Decrypted state variables
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
    bytes public yBytes256;

    // Tracks the latest decryption request ID
    uint256 public latestRequestID;

    // Constructor to initialize the contract and set up encrypted values
    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
        Gateway.setGateway(Gateway.defaultGatewayAddress());
        
        // Initialize encrypted variables with sample values
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
        xAddress = TFHE.asEaddress(0x8ba1f109551bD432803012645Ac136ddd64DBA72);
        TFHE.allowThis(xAddress);
        xAddress2 = TFHE.asEaddress(0xf48b8840387ba3809DAE990c930F3b4766A86ca3);
        TFHE.allowThis(xAddress2);
    }

    // Function to request decryption of a boolean value with an infinite loop in the callback
    function requestBoolInfinite() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        Gateway.requestDecryption(cts, this.callbackBoolInfinite.selector, 0, block.timestamp + 100, false);
    }

    // Callback function for the infinite loop decryption request (WARNING: This function will never complete)
    function callbackBoolInfinite(uint256 /*requestID*/, bool decryptedInput) public onlyGateway returns (bool) {
        uint256 i = 0;
        while (true) {
            i++;
        }
        yBool = decryptedInput;
        return yBool;
    }

    // Function to request decryption with an excessive delay (should revert)
    function requestBoolAboveDelay() public {
        // This should revert due to the excessive delay
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        Gateway.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 2 days, false);
    }

    // Request decryption of a boolean value
    function requestBool() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        // Request decryption with a 100-second deadline and non-trustless mode
        Gateway.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 100, false);
    }

    // Request decryption of a boolean value in trustless mode
    function requestBoolTrustless() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        // Request decryption with a 100-second deadline and trustless mode (true)
        uint256 requestID = Gateway.requestDecryption(
            cts,
            this.callbackBoolTrustless.selector,
            0,
            block.timestamp + 100,
            true
        );
        latestRequestID = requestID;
        // Save the requested handles for later verification
        saveRequestedHandles(requestID, cts);
    }

    // Attempt to request decryption of a fake boolean value (should revert)
    function requestFakeBool() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000000);
        // This should revert because the previous ebool is not honestly obtained
        Gateway.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 100, false);
    }

    // Callback function for non-trustless boolean decryption
    function callbackBool(uint256, bool decryptedInput) public onlyGateway returns (bool) {
        yBool = decryptedInput;
        return yBool;
    }

    // Callback function for trustless boolean decryption
    function callbackBoolTrustless(
        uint256 requestID,
        bool decryptedInput,
        bytes[] memory signatures
    ) public onlyGateway returns (bool) {
        // Verify that the requestID matches the latest request
        require(latestRequestID == requestID, "wrong requestID passed by Gateway");
        // Load the previously saved handles for verification
        uint256[] memory requestedHandles = loadRequestedHandles(latestRequestID);
        // Verify the signatures provided by the KMS (Key Management Service)
        bool isKMSVerified = Gateway.verifySignatures(requestedHandles, signatures);
        require(isKMSVerified, "KMS did not verify this decryption result");
        // If verification passes, store the decrypted value
        yBool = decryptedInput;
        return yBool;
    }
    // Function to request decryption of a 4-bit unsigned integer
    function requestUint4() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint4);
        Gateway.requestDecryption(cts, this.callbackUint4.selector, 0, block.timestamp + 100, false);
    }

    // Function to attempt requesting decryption of a fake 4-bit unsigned integer (should revert)
    function requestFakeUint4() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000100);
        Gateway.requestDecryption(cts, this.callbackUint4.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    // Callback function for 4-bit unsigned integer decryption
    function callbackUint4(uint256, uint8 decryptedInput) public onlyGateway returns (uint8) {
        yUint4 = decryptedInput;
        return decryptedInput;
    }

    // Function to request decryption of an 8-bit unsigned integer
    function requestUint8() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint8);
        Gateway.requestDecryption(cts, this.callbackUint8.selector, 0, block.timestamp + 100, false);
    }

    // Function to attempt requesting decryption of a fake 8-bit unsigned integer (should revert)
    function requestFakeUint8() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000200);
        Gateway.requestDecryption(cts, this.callbackUint8.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    // Callback function for 8-bit unsigned integer decryption
    function callbackUint8(uint256, uint8 decryptedInput) public onlyGateway returns (uint8) {
        yUint8 = decryptedInput;
        return decryptedInput;
    }

    // Function to request decryption of a 16-bit unsigned integer
    function requestUint16() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint16);
        Gateway.requestDecryption(cts, this.callbackUint16.selector, 0, block.timestamp + 100, false);
    }

    // Function to attempt requesting decryption of a fake 16-bit unsigned integer (should revert)
    function requestFakeUint16() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000300);
        Gateway.requestDecryption(cts, this.callbackUint16.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    // Callback function for 16-bit unsigned integer decryption
    function callbackUint16(uint256, uint16 decryptedInput) public onlyGateway returns (uint16) {
        yUint16 = decryptedInput;
        return decryptedInput;
    }

    // Function to request decryption of a 32-bit unsigned integer with additional inputs
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

    // Function to attempt requesting decryption of a fake 32-bit unsigned integer (should revert)
    function requestFakeUint32() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000400);
        Gateway.requestDecryption(cts, this.callbackUint32.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    // Callback function for 32-bit unsigned integer decryption
    function callbackUint32(uint256 requestID, uint32 decryptedInput) public onlyGateway returns (uint32) {
        uint256[] memory params = getParamsUint256(requestID);
        unchecked {
            uint32 result = uint32(params[0]) + uint32(params[1]) + decryptedInput;
            yUint32 = result;
            return result;
        }
    }

    // Function to request decryption of a 64-bit unsigned integer
    function requestUint64() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint64);
        Gateway.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100, false);
    }

    // Function to attempt requesting decryption of a fake 64-bit unsigned integer (should revert)
    function requestFakeUint64() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000500);
        Gateway.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    // Function to request decryption of a non-trivial 64-bit unsigned integer
    function requestUint64NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        euint64 inputNonTrivial = TFHE.asEuint64(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputNonTrivial);
        Gateway.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100, false);
    }

    // Callback function for 64-bit unsigned integer decryption
    function callbackUint64(uint256, uint64 decryptedInput) public onlyGateway returns (uint64) {
        yUint64 = decryptedInput;
        return decryptedInput;
    }

    // Function to request decryption of a non-trivial 256-bit encrypted bytes
    function requestEbytes256NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        ebytes256 inputNonTrivial = TFHE.asEbytes256(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputNonTrivial);
        Gateway.requestDecryption(cts, this.callbackBytes256.selector, 0, block.timestamp + 100, false);
    }

    // Callback function for 256-bit encrypted bytes decryption
    function callbackBytes256(uint256, bytes calldata decryptedInput) public onlyGateway returns (bytes memory) {
        yBytes256 = decryptedInput;
        return decryptedInput;
    }

    // Function to request decryption of an encrypted address
    function requestAddress() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xAddress);
        Gateway.requestDecryption(cts, this.callbackAddress.selector, 0, block.timestamp + 100, false);
    }

    // Function to request decryption of multiple encrypted addresses
    function requestSeveralAddresses() public {
        uint256[] memory cts = new uint256[](2);
        cts[0] = Gateway.toUint256(xAddress);
        cts[1] = Gateway.toUint256(xAddress2);
        Gateway.requestDecryption(cts, this.callbackAddresses.selector, 0, block.timestamp + 100, false);
    }

    // Callback function for multiple address decryption
    function callbackAddresses(
        uint256 /*requestID*/,
        address decryptedInput1,
        address decryptedInput2
    ) public onlyGateway returns (address) {
        yAddress = decryptedInput1;
        yAddress2 = decryptedInput2;
        return decryptedInput1;
    }

    // Function to attempt requesting decryption of a fake address (should revert)
    function requestFakeAddress() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000700);
        Gateway.requestDecryption(cts, this.callbackAddress.selector, 0, block.timestamp + 100, false); // this should revert because previous handle is not honestly obtained
    }

    // Callback function for address decryption
    function callbackAddress(uint256, address decryptedInput) public onlyGateway returns (address) {
        yAddress = decryptedInput;
        return decryptedInput;
    }

    // Function to request decryption of mixed data types
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

    // Callback function for mixed data type decryption
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

    // Function to request decryption of mixed data types including 256-bit encrypted bytes
    function requestMixedBytes256(einput inputHandle, bytes calldata inputProof) public {
        ebytes256 xBytes256 = TFHE.asEbytes256(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](3);
        cts[0] = Gateway.toUint256(xBool);
        cts[1] = Gateway.toUint256(xAddress);
        cts[2] = Gateway.toUint256(xBytes256);
        Gateway.requestDecryption(cts, this.callbackMixedBytes256.selector, 0, block.timestamp + 100, false);
    }

    // Callback function for mixed data type decryption including 256-bit encrypted bytes
    function callbackMixedBytes256(
        uint256,
        bool decBool,
        address decAddress,
        bytes memory bytesRes
    ) public onlyGateway {
        yBool = decBool;
        yAddress = decAddress;
        yBytes256 = bytesRes;
    }

    // Function to request trustless decryption of non-trivial 256-bit encrypted bytes
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

    // Callback function for trustless decryption of 256-bit encrypted bytes
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

    // Function to request trustless decryption of mixed data types including 256-bit encrypted bytes
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

    // Callback function for trustless decryption of mixed data types including 256-bit encrypted bytes
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