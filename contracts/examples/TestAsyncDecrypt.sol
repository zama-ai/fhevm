// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/HTTPZ.sol";
import "../decryptionOracleLib/DecryptionOracleCaller.sol";
import "../addresses/DecryptionOracleAddress.sol";

/// @notice Contract for testing asynchronous decryption using the Gateway
contract TestAsyncDecrypt is DecryptionOracleCaller {
    /// @dev Encrypted state variables
    ebool xBool;
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

    /// @dev Decrypted state variables
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

    /// @dev Tracks the latest decryption request ID
    uint256 public latestRequestID;

    /// @notice Constructor to initialize the contract and set up encrypted values
    constructor() {
        HTTPZ.setCoprocessor(HTTPZConfig.defaultConfig());
        setDecryptionOracle(DECRYPTION_ORACLE_ADDRESS);

        /// @dev Initialize encrypted variables with sample values
        xBool = HTTPZ.asEbool(true);
        HTTPZ.allowThis(xBool);

        xUint8 = HTTPZ.asEuint8(42);
        HTTPZ.allowThis(xUint8);
        xUint16 = HTTPZ.asEuint16(16);
        HTTPZ.allowThis(xUint16);
        xUint32 = HTTPZ.asEuint32(32);
        HTTPZ.allowThis(xUint32);
        xUint64 = HTTPZ.asEuint64(18446744073709551600);
        HTTPZ.allowThis(xUint64);
        xUint64_2 = HTTPZ.asEuint64(76575465786);
        HTTPZ.allowThis(xUint64_2);
        xUint64_3 = HTTPZ.asEuint64(6400);
        HTTPZ.allowThis(xUint64_3);
        xUint128 = HTTPZ.asEuint128(1267650600228229401496703205443);
        HTTPZ.allowThis(xUint128);
        xUint256 = HTTPZ.asEuint256(27606985387162255149739023449108101809804435888681546220650096895197251);
        HTTPZ.allowThis(xUint256);
        xAddress = HTTPZ.asEaddress(0x8ba1f109551bD432803012645Ac136ddd64DBA72);
        HTTPZ.allowThis(xAddress);
        xAddress2 = HTTPZ.asEaddress(0xf48b8840387ba3809DAE990c930F3b4766A86ca3);
        HTTPZ.allowThis(xAddress2);
    }

    /// @notice Function to request decryption of a boolean value with an infinite loop in the callback
    function requestBoolInfinite() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(xBool);
        requestDecryption(cts, this.callbackBoolInfinite.selector);
    }

    /// @notice Callback function for the infinite loop decryption request (WARNING: This function will never complete)
    function callbackBoolInfinite(
        uint256 requestID,
        bool decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (bool) {
        uint256 i = 0;
        while (true) {
            i++;
        }
        yBool = decryptedInput;
        return yBool;
    }

    /// @notice Request decryption of a boolean value
    function requestBool() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(xBool);
        requestDecryption(cts, this.callbackBool.selector);
    }

    /// @notice Attempt to request decryption of a fake boolean value (should revert)
    function requestFakeBool() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000000;
        /// @dev This should revert because the previous ebool is not honestly obtained
        requestDecryption(cts, this.callbackBool.selector);
    }

    /// @notice Callback function for boolean decryption
    function callbackBool(
        uint256 requestID,
        bool decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (bool) {
        yBool = decryptedInput;
        return yBool;
    }

    /// @notice Callback function for 4-bit unsigned integer decryption
    /// @param decryptedInput The decrypted 4-bit unsigned integer
    /// @return The decrypted value
    function callbackUint4(
        uint256 requestID,
        uint8 decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (uint8) {
        yUint4 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of an 8-bit unsigned integer
    function requestUint8() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(xUint8);
        requestDecryption(cts, this.callbackUint8.selector);
    }

    /// @notice Attempt to request decryption of a fake 8-bit unsigned integer (should revert)
    function requestFakeUint8() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000200;
        /// @dev This should revert because the previous handle is not honestly obtained
        requestDecryption(cts, this.callbackUint8.selector);
    }

    /// @notice Callback function for 8-bit unsigned integer decryption
    /// @param decryptedInput The decrypted 8-bit unsigned integer
    /// @return The decrypted value
    function callbackUint8(
        uint256 requestID,
        uint8 decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (uint8) {
        yUint8 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of a 16-bit unsigned integer
    function requestUint16() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(xUint16);
        requestDecryption(cts, this.callbackUint16.selector);
    }

    /// @notice Attempt to request decryption of a fake 16-bit unsigned integer (should revert)
    function requestFakeUint16() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000300;
        /// @dev This should revert because the previous handle is not honestly obtained
        requestDecryption(cts, this.callbackUint16.selector);
    }

    /// @notice Callback function for 16-bit unsigned integer decryption
    /// @param decryptedInput The decrypted 16-bit unsigned integer
    /// @return The decrypted value
    function callbackUint16(
        uint256 requestID,
        uint16 decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (uint16) {
        yUint16 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of a 32-bit unsigned integer with additional inputs
    /// @param input1 First additional input
    /// @param input2 Second additional input
    function requestUint32(uint32 input1, uint32 input2) public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(xUint32);
        uint256 requestID = requestDecryption(cts, this.callbackUint32.selector);
        addParamsUint256(requestID, input1);
        addParamsUint256(requestID, input2);
    }

    /// @notice Attempt to request decryption of a fake 32-bit unsigned integer (should revert)
    function requestFakeUint32() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000400;
        /// @dev This should revert because the previous handle is not honestly obtained
        requestDecryption(cts, this.callbackUint32.selector);
    }

    /// @notice Callback function for 32-bit unsigned integer decryption
    /// @param requestID The ID of the decryption request
    /// @param decryptedInput The decrypted 32-bit unsigned integer
    /// @return The result of the computation
    function callbackUint32(
        uint256 requestID,
        uint32 decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (uint32) {
        uint256[] memory params = getParamsUint256(requestID);
        unchecked {
            uint32 result = uint32(uint256(params[0])) + uint32(uint256(params[1])) + decryptedInput;
            yUint32 = result;
            return result;
        }
    }

    /// @notice Request decryption of a 64-bit unsigned integer
    function requestUint64() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(xUint64);
        requestDecryption(cts, this.callbackUint64.selector);
    }

    /// @notice Attempt to request decryption of a fake 64-bit unsigned integer (should revert)
    function requestFakeUint64() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000500;
        /// @dev This should revert because the previous handle is not honestly obtained
        requestDecryption(cts, this.callbackUint64.selector);
    }

    /// @notice Request decryption of a non-trivial 64-bit unsigned integer
    /// @param inputHandle The input handle for the encrypted value
    /// @param inputProof The input proof for the encrypted value
    function requestUint64NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        euint64 inputNonTrivial = HTTPZ.asEuint64(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(inputNonTrivial);
        requestDecryption(cts, this.callbackUint64.selector);
    }

    /// @notice Callback function for 64-bit unsigned integer decryption
    /// @param decryptedInput The decrypted 64-bit unsigned integer
    /// @return The decrypted value
    function callbackUint64(
        uint256 requestID,
        uint64 decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (uint64) {
        yUint64 = decryptedInput;
        return decryptedInput;
    }

    function requestUint128() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(xUint128);
        requestDecryption(cts, this.callbackUint128.selector);
    }

    function requestUint128NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        euint128 inputNonTrivial = HTTPZ.asEuint128(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(inputNonTrivial);
        requestDecryption(cts, this.callbackUint128.selector);
    }

    function callbackUint128(
        uint256 requestID,
        uint128 decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (uint128) {
        yUint128 = decryptedInput;
        return decryptedInput;
    }

    function requestUint256() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(xUint256);
        requestDecryption(cts, this.callbackUint256.selector);
    }

    function requestUint256NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        euint256 inputNonTrivial = HTTPZ.asEuint256(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(inputNonTrivial);
        requestDecryption(cts, this.callbackUint256.selector);
    }

    function callbackUint256(
        uint256 requestID,
        uint256 decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (uint256) {
        yUint256 = decryptedInput;
        return decryptedInput;
    }

    function requestEbytes64NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        ebytes64 inputNonTrivial = HTTPZ.asEbytes64(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(inputNonTrivial);
        requestDecryption(cts, this.callbackBytes64.selector);
    }

    function requestEbytes64Trivial(bytes calldata value) public {
        ebytes64 inputTrivial = HTTPZ.asEbytes64(HTTPZ.padToBytes64(value));
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(inputTrivial);
        requestDecryption(cts, this.callbackBytes64.selector);
    }

    function callbackBytes64(
        uint256 requestID,
        bytes calldata decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (bytes memory) {
        yBytes64 = decryptedInput;
        return decryptedInput;
    }

    function requestEbytes128NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        ebytes128 inputNonTrivial = HTTPZ.asEbytes128(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(inputNonTrivial);
        requestDecryption(cts, this.callbackBytes128.selector);
    }

    function requestEbytes128Trivial(bytes calldata value) public {
        ebytes128 inputTrivial = HTTPZ.asEbytes128(HTTPZ.padToBytes128(value));
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(inputTrivial);
        requestDecryption(cts, this.callbackBytes128.selector);
    }

    function callbackBytes128(
        uint256 requestID,
        bytes calldata decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (bytes memory) {
        yBytes128 = decryptedInput;
        return decryptedInput;
    }

    function requestEbytes256Trivial(bytes calldata value) public {
        ebytes256 inputTrivial = HTTPZ.asEbytes256(HTTPZ.padToBytes256(value));
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(inputTrivial);
        requestDecryption(cts, this.callbackBytes256.selector);
    }

    function requestEbytes256NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        ebytes256 inputNonTrivial = HTTPZ.asEbytes256(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(inputNonTrivial);
        requestDecryption(cts, this.callbackBytes256.selector);
    }

    /// @notice Callback function for 256-bit encrypted bytes decryption
    /// @param decryptedInput The decrypted 256-bit bytes
    /// @return The decrypted value
    function callbackBytes256(
        uint256 requestID,
        bytes calldata decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (bytes memory) {
        yBytes256 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of an encrypted address
    function requestAddress() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = toBytes32(xAddress);
        requestDecryption(cts, this.callbackAddress.selector);
    }

    /// @notice Request decryption of multiple encrypted addresses
    function requestSeveralAddresses() public {
        bytes32[] memory cts = new bytes32[](2);
        cts[0] = toBytes32(xAddress);
        cts[1] = toBytes32(xAddress2);
        requestDecryption(cts, this.callbackAddresses.selector);
    }

    /// @notice Callback function for multiple address decryption
    /// @param decryptedInput1 The first decrypted address
    /// @param decryptedInput2 The second decrypted address
    /// @return The first decrypted address
    function callbackAddresses(
        uint256 requestID,
        address decryptedInput1,
        address decryptedInput2,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (address) {
        yAddress = decryptedInput1;
        yAddress2 = decryptedInput2;
        return decryptedInput1;
    }

    /// @notice Attempt to request decryption of a fake address (should revert)
    function requestFakeAddress() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000700;
        /// @dev This should revert because the previous handle is not honestly obtained
        requestDecryption(cts, this.callbackAddress.selector);
    }

    /// @notice Callback function for address decryption
    /// @param decryptedInput The decrypted address
    /// @return The decrypted address
    function callbackAddress(
        uint256 requestID,
        address decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) returns (address) {
        yAddress = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of mixed data types including 256-bit encrypted bytes
    /// @dev Demonstrates how to include encrypted bytes256 in a mixed decryption request
    /// @param inputHandle The encrypted input handle for the bytes256
    /// @param inputProof The proof for the encrypted bytes256
    function requestMixedBytes256(einput inputHandle, bytes calldata inputProof) public {
        ebytes256 xBytes256 = HTTPZ.asEbytes256(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](4);
        cts[0] = toBytes32(xBool);
        cts[1] = toBytes32(xAddress);
        cts[2] = toBytes32(xBytes256);
        ebytes64 input64Bytes = HTTPZ.asEbytes64(HTTPZ.padToBytes64(hex"aaff42"));
        cts[3] = toBytes32(input64Bytes);
        requestDecryption(cts, this.callbackMixedBytes256.selector);
    }

    /// @notice Callback function for mixed data type decryption including 256-bit encrypted bytes
    /// @dev Processes and stores the decrypted values
    /// @param decBool Decrypted boolean
    /// @param decAddress Decrypted address
    /// @param bytesRes Decrypted 256-bit bytes
    function callbackMixedBytes256(
        uint256 requestID,
        bool decBool,
        address decAddress,
        bytes memory bytesRes,
        bytes memory bytesRes2,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) {
        yBool = decBool;
        yAddress = decAddress;
        yBytes256 = bytesRes;
        yBytes64 = bytesRes2;
    }
}
