// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "../gateway/GatewayCaller.sol";

/// @notice Contract for testing asynchronous decryption using the Gateway
contract TestAsyncDecrypt is GatewayCaller {
    /// @dev Encrypted state variables
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

    /// @dev Decrypted state variables
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

    /// @dev Tracks the latest decryption request ID
    uint256 public latestRequestID;

    /// @notice Constructor to initialize the contract and set up encrypted values
    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
        Gateway.setGateway(Gateway.defaultGatewayAddress());

        /// @dev Initialize encrypted variables with sample values
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

    /// @notice Function to request decryption of a boolean value with an infinite loop in the callback
    function requestBoolInfinite() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        Gateway.requestDecryption(cts, this.callbackBoolInfinite.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function for the infinite loop decryption request (WARNING: This function will never complete)
    function callbackBoolInfinite(uint256 /*requestID*/, bool decryptedInput) public onlyGateway returns (bool) {
        uint256 i = 0;
        while (true) {
            i++;
        }
        yBool = decryptedInput;
        return yBool;
    }

    /// @notice Function to request decryption with an excessive delay (should revert)
    function requestBoolAboveDelay() public {
        /// @dev This should revert due to the excessive delay
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        Gateway.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 2 days, false);
    }

    /// @notice Request decryption of a boolean value
    function requestBool() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        /// @dev Request decryption with a 100-second deadline and non-trustless mode
        Gateway.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Request decryption of a boolean value in trustless mode
    function requestBoolTrustless() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xBool);
        /// @dev Request decryption with a 100-second deadline and trustless mode (true)
        uint256 requestID = Gateway.requestDecryption(
            cts,
            this.callbackBoolTrustless.selector,
            0,
            block.timestamp + 100,
            true
        );
        latestRequestID = requestID;
        /// @dev Save the requested handles for later verification
        saveRequestedHandles(requestID, cts);
    }

    /// @notice Attempt to request decryption of a fake boolean value (should revert)
    function requestFakeBool() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000000);
        /// @dev This should revert because the previous ebool is not honestly obtained
        Gateway.requestDecryption(cts, this.callbackBool.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function for non-trustless boolean decryption
    function callbackBool(uint256, bool decryptedInput) public onlyGateway returns (bool) {
        yBool = decryptedInput;
        return yBool;
    }

    /// @notice Callback function for trustless boolean decryption
    function callbackBoolTrustless(
        uint256 requestID,
        bool decryptedInput,
        bytes[] memory signatures
    ) public onlyGateway returns (bool) {
        /// @dev Verify that the requestID matches the latest request
        require(latestRequestID == requestID, "wrong requestID passed by Gateway");
        /// @dev Load the previously saved handles for verification
        uint256[] memory requestedHandles = loadRequestedHandles(latestRequestID);
        /// @dev Verify the signatures provided by the KMS (Key Management Service)
        bool isKMSVerified = Gateway.verifySignatures(requestedHandles, signatures);
        require(isKMSVerified, "KMS did not verify this decryption result");
        /// @dev If verification passes, store the decrypted value
        yBool = decryptedInput;
        return yBool;
    }

    /// @notice Request decryption of a 4-bit unsigned integer
    function requestUint4() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint4);
        Gateway.requestDecryption(cts, this.callbackUint4.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Attempt to request decryption of a fake 4-bit unsigned integer (should revert)
    function requestFakeUint4() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000100);
        /// @dev This should revert because the previous handle is not honestly obtained
        Gateway.requestDecryption(cts, this.callbackUint4.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function for 4-bit unsigned integer decryption
    /// @param decryptedInput The decrypted 4-bit unsigned integer
    /// @return The decrypted value
    function callbackUint4(uint256, uint8 decryptedInput) public onlyGateway returns (uint8) {
        yUint4 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of an 8-bit unsigned integer
    function requestUint8() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint8);
        Gateway.requestDecryption(cts, this.callbackUint8.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Attempt to request decryption of a fake 8-bit unsigned integer (should revert)
    function requestFakeUint8() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000200);
        /// @dev This should revert because the previous handle is not honestly obtained
        Gateway.requestDecryption(cts, this.callbackUint8.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function for 8-bit unsigned integer decryption
    /// @param decryptedInput The decrypted 8-bit unsigned integer
    /// @return The decrypted value
    function callbackUint8(uint256, uint8 decryptedInput) public onlyGateway returns (uint8) {
        yUint8 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of a 16-bit unsigned integer
    function requestUint16() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint16);
        Gateway.requestDecryption(cts, this.callbackUint16.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Attempt to request decryption of a fake 16-bit unsigned integer (should revert)
    function requestFakeUint16() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000300);
        /// @dev This should revert because the previous handle is not honestly obtained
        Gateway.requestDecryption(cts, this.callbackUint16.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function for 16-bit unsigned integer decryption
    /// @param decryptedInput The decrypted 16-bit unsigned integer
    /// @return The decrypted value
    function callbackUint16(uint256, uint16 decryptedInput) public onlyGateway returns (uint16) {
        yUint16 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of a 32-bit unsigned integer with additional inputs
    /// @param input1 First additional input
    /// @param input2 Second additional input
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

    /// @notice Attempt to request decryption of a fake 32-bit unsigned integer (should revert)
    function requestFakeUint32() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000400);
        /// @dev This should revert because the previous handle is not honestly obtained
        Gateway.requestDecryption(cts, this.callbackUint32.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function for 32-bit unsigned integer decryption
    /// @param requestID The ID of the decryption request
    /// @param decryptedInput The decrypted 32-bit unsigned integer
    /// @return The result of the computation
    function callbackUint32(uint256 requestID, uint32 decryptedInput) public onlyGateway returns (uint32) {
        uint256[] memory params = getParamsUint256(requestID);
        unchecked {
            uint32 result = uint32(params[0]) + uint32(params[1]) + decryptedInput;
            yUint32 = result;
            return result;
        }
    }

    /// @notice Request decryption of a 64-bit unsigned integer
    function requestUint64() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xUint64);
        Gateway.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Attempt to request decryption of a fake 64-bit unsigned integer (should revert)
    function requestFakeUint64() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000500);
        /// @dev This should revert because the previous handle is not honestly obtained
        Gateway.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Request decryption of a non-trivial 64-bit unsigned integer
    /// @param inputHandle The input handle for the encrypted value
    /// @param inputProof The input proof for the encrypted value
    function requestUint64NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        euint64 inputNonTrivial = TFHE.asEuint64(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputNonTrivial);
        Gateway.requestDecryption(cts, this.callbackUint64.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function for 64-bit unsigned integer decryption
    /// @param decryptedInput The decrypted 64-bit unsigned integer
    /// @return The decrypted value
    function callbackUint64(uint256, uint64 decryptedInput) public onlyGateway returns (uint64) {
        yUint64 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of a non-trivial 256-bit encrypted bytes
    /// @param inputHandle The input handle for the encrypted value
    /// @param inputProof The input proof for the encrypted value
    function requestEbytes256NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        ebytes256 inputNonTrivial = TFHE.asEbytes256(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(inputNonTrivial);
        Gateway.requestDecryption(cts, this.callbackBytes256.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function for 256-bit encrypted bytes decryption
    /// @param decryptedInput The decrypted 256-bit bytes
    /// @return The decrypted value
    function callbackBytes256(uint256, bytes calldata decryptedInput) public onlyGateway returns (bytes memory) {
        yBytes256 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of an encrypted address
    function requestAddress() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(xAddress);
        Gateway.requestDecryption(cts, this.callbackAddress.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Request decryption of multiple encrypted addresses
    function requestSeveralAddresses() public {
        uint256[] memory cts = new uint256[](2);
        cts[0] = Gateway.toUint256(xAddress);
        cts[1] = Gateway.toUint256(xAddress2);
        Gateway.requestDecryption(cts, this.callbackAddresses.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function for multiple address decryption
    /// @param decryptedInput1 The first decrypted address
    /// @param decryptedInput2 The second decrypted address
    /// @return The first decrypted address
    function callbackAddresses(
        uint256 /*requestID*/,
        address decryptedInput1,
        address decryptedInput2
    ) public onlyGateway returns (address) {
        yAddress = decryptedInput1;
        yAddress2 = decryptedInput2;
        return decryptedInput1;
    }

    /// @notice Attempt to request decryption of a fake address (should revert)
    function requestFakeAddress() public {
        uint256[] memory cts = new uint256[](1);
        cts[0] = uint256(0x4200000000000000000000000000000000000000000000000000000000000700);
        /// @dev This should revert because the previous handle is not honestly obtained
        Gateway.requestDecryption(cts, this.callbackAddress.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function for address decryption
    /// @param decryptedInput The decrypted address
    /// @return The decrypted address
    function callbackAddress(uint256, address decryptedInput) public onlyGateway returns (address) {
        yAddress = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of multiple encrypted data types
    /// @dev This function demonstrates how to request decryption for various encrypted data types in a single call
    /// @param input1 First additional input parameter for the callback function
    /// @param input2 Second additional input parameter for the callback function
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

    /// @notice Callback function for mixed data type decryption
    /// @dev Processes the decrypted values and performs some basic checks
    /// @param requestID The ID of the decryption request
    /// @param decBool_1 First decrypted boolean
    /// @param decBool_2 Second decrypted boolean
    /// @param decUint4 Decrypted 4-bit unsigned integer
    /// @param decUint8 Decrypted 8-bit unsigned integer
    /// @param decUint16 Decrypted 16-bit unsigned integer
    /// @param decUint32 Decrypted 32-bit unsigned integer
    /// @param decUint64_1 First decrypted 64-bit unsigned integer
    /// @param decUint64_2 Second decrypted 64-bit unsigned integer
    /// @param decUint64_3 Third decrypted 64-bit unsigned integer
    /// @param decAddress Decrypted address
    /// @return The decrypted 4-bit unsigned integer
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

    /// @notice Request decryption of mixed data types including 256-bit encrypted bytes
    /// @dev Demonstrates how to include encrypted bytes256 in a mixed decryption request
    /// @param inputHandle The encrypted input handle for the bytes256
    /// @param inputProof The proof for the encrypted bytes256
    function requestMixedBytes256(einput inputHandle, bytes calldata inputProof) public {
        ebytes256 xBytes256 = TFHE.asEbytes256(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](3);
        cts[0] = Gateway.toUint256(xBool);
        cts[1] = Gateway.toUint256(xAddress);
        cts[2] = Gateway.toUint256(xBytes256);
        Gateway.requestDecryption(cts, this.callbackMixedBytes256.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function for mixed data type decryption including 256-bit encrypted bytes
    /// @dev Processes and stores the decrypted values
    /// @param decBool Decrypted boolean
    /// @param decAddress Decrypted address
    /// @param bytesRes Decrypted 256-bit bytes
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

    /// @notice Request trustless decryption of non-trivial 256-bit encrypted bytes
    /// @dev Demonstrates how to request trustless decryption for complex encrypted bytes256
    /// @param inputHandle The encrypted input handle for the bytes256
    /// @param inputProof The proof for the encrypted bytes256
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

    /// @notice Callback function for trustless decryption of 256-bit encrypted bytes
    /// @dev Verifies the decryption result using KMS signatures
    /// @param requestID The ID of the decryption request
    /// @param decryptedInput The decrypted bytes256 value
    /// @param signatures The signatures from the KMS for verification
    /// @return The decrypted bytes256 value
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

    /// @notice Request trustless decryption of mixed data types including 256-bit encrypted bytes
    /// @dev Demonstrates how to request trustless decryption for multiple data types
    /// @param inputHandle The encrypted input handle for the bytes256
    /// @param inputProof The proof for the encrypted bytes256
    function requestMixedBytes256Trustless(einput inputHandle, bytes calldata inputProof) public {
        ebytes256 xBytes256 = TFHE.asEbytes256(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](3);
        cts[0] = Gateway.toUint256(xBool);
        cts[1] = Gateway.toUint256(xBytes256);
        cts[2] = Gateway.toUint256(xAddress);
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

    /// @notice Callback function for trustless decryption of mixed data types including 256-bit encrypted bytes
    /// @dev Verifies and processes the decrypted values
    /// @param requestID The ID of the decryption request
    /// @param decBool Decrypted boolean
    /// @param bytesRes Decrypted 256-bit bytes
    /// @param decAddress Decrypted address
    /// @param signatures The signatures from the KMS for verification
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
