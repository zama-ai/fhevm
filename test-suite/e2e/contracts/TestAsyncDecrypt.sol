// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2EFHEVMConfig} from "./E2EFHEVMConfigLocal.sol";

/// @notice Contract for testing asynchronous decryption using the Gateway
contract TestAsyncDecrypt is E2EFHEVMConfig {
    /// @dev Encrypted state variables
    ebool xBool;
    euint8 xUint8;
    euint16 xUint16;
    euint32 xUint32;
    euint32 xUint32_2;
    euint32 xUint32_3;
    euint64 xUint64;
    euint64 xUint64_2;
    euint64 xUint64_3;
    euint128 xUint128;
    euint128 xUint128_2;
    euint128 xUint128_3;
    eaddress xAddress;
    eaddress xAddress2;
    euint256 xUint256;

    /// @dev Decrypted state variables
    bool public yBool;
    uint8 public yUint8;
    uint16 public yUint16;
    uint32 public yUint32;
    uint32 public yUint32_2;
    uint32 public yUint32_3;
    uint64 public yUint64;
    uint64 public yUint64_2;
    uint64 public yUint64_3;
    uint128 public yUint128;
    uint128 public yUint128_2;
    uint128 public yUint128_3;
    address public yAddress;
    address public yAddress2;
    uint256 public yUint256;
    bytes public yBytes64;
    bytes public yBytes128;
    bytes public yBytes256;

    /// @dev private mapping that can be used to link a requestID to an array of uint256
    mapping(uint256 => uint256[]) private paramsUint256;

    /// @notice Constructor to initialize the contract and set up encrypted values
    constructor() {
        /// @dev Initialize encrypted variables with sample values
        xBool = FHE.asEbool(true);
        FHE.allowThis(xBool);
        xUint8 = FHE.asEuint8(42);
        FHE.allowThis(xUint8);
        xUint16 = FHE.asEuint16(16);
        FHE.allowThis(xUint16);
        xUint32 = FHE.asEuint32(32);
        FHE.allowThis(xUint32);
        xUint32_2 = FHE.asEuint32(1000);
        FHE.allowThis(xUint32_2);
        xUint32_3 = FHE.asEuint32(2000);
        FHE.allowThis(xUint32_3);
        xUint64 = FHE.asEuint64(18446744073709551600);
        FHE.allowThis(xUint64);
        xUint64_2 = FHE.asEuint64(76575465786);
        FHE.allowThis(xUint64_2);
        xUint64_3 = FHE.asEuint64(6400);
        FHE.allowThis(xUint64_3);
        xUint128 = FHE.asEuint128(1267650600228229401496703205443);
        FHE.allowThis(xUint128);
        xUint128_2 = FHE.asEuint128(10000);
        FHE.allowThis(xUint128_2);
        xUint128_3 = FHE.asEuint128(20000);
        FHE.allowThis(xUint128_3);
        xUint256 = FHE.asEuint256(27606985387162255149739023449108101809804435888681546220650096895197251);
        FHE.allowThis(xUint256);
        xAddress = FHE.asEaddress(0x8ba1f109551bD432803012645Ac136ddd64DBA72);
        FHE.allowThis(xAddress);
        xAddress2 = FHE.asEaddress(0xf48b8840387ba3809DAE990c930F3b4766A86ca3);
        FHE.allowThis(xAddress2);
    }

    /// @notice Function to request decryption of a boolean value with an infinite loop in the callback
    function requestBoolInfinite() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xBool);
        FHE.requestDecryption(cts, this.callbackBoolInfinite.selector);
    }

    /// @notice Callback function for the infinite loop decryption request (WARNING: This function will never complete)
    function callbackBoolInfinite(
        uint256 requestID,
        bool decryptedInput,
        bytes[] memory signatures
    ) public returns (bool) {
        FHE.checkSignatures(requestID, signatures);
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
        cts[0] = FHE.toBytes32(xBool);
        FHE.requestDecryption(cts, this.callbackBool.selector);
    }

    /// @notice Attempt to request decryption of a fake boolean value (should revert)
    function requestFakeBool() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000000;
        /// @dev This should revert because the previous ebool is not honestly obtained
        FHE.requestDecryption(cts, this.callbackBool.selector);
    }

    /// @notice Callback function for boolean decryption
    function callbackBool(uint256 requestID, bool decryptedInput, bytes[] memory signatures) public returns (bool) {
        FHE.checkSignatures(requestID, signatures);
        yBool = decryptedInput;
        return yBool;
    }

    /// @notice Request decryption of an 8-bit unsigned integer
    function requestUint8() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint8);
        FHE.requestDecryption(cts, this.callbackUint8.selector);
    }

    /// @notice Attempt to request decryption of a fake 8-bit unsigned integer (should revert)
    function requestFakeUint8() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000200;
        /// @dev This should revert because the previous handle is not honestly obtained
        FHE.requestDecryption(cts, this.callbackUint8.selector);
    }

    /// @notice Callback function for 8-bit unsigned integer decryption
    /// @param decryptedInput The decrypted 8-bit unsigned integer
    /// @return The decrypted value
    function callbackUint8(uint256 requestID, uint8 decryptedInput, bytes[] memory signatures) public returns (uint8) {
        FHE.checkSignatures(requestID, signatures);
        yUint8 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of a 16-bit unsigned integer
    function requestUint16() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint16);
        FHE.requestDecryption(cts, this.callbackUint16.selector);
    }

    /// @notice Attempt to request decryption of a fake 16-bit unsigned integer (should revert)
    function requestFakeUint16() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000300;
        /// @dev This should revert because the previous handle is not honestly obtained
        FHE.requestDecryption(cts, this.callbackUint16.selector);
    }

    /// @notice Callback function for 16-bit unsigned integer decryption
    /// @param decryptedInput The decrypted 16-bit unsigned integer
    /// @return The decrypted value
    function callbackUint16(
        uint256 requestID,
        uint16 decryptedInput,
        bytes[] memory signatures
    ) public returns (uint16) {
        FHE.checkSignatures(requestID, signatures);
        yUint16 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of a 32-bit unsigned integer with additional inputs
    /// @param input1 First additional input
    /// @param input2 Second additional input
    function requestUint32(uint32 input1, uint32 input2) public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint32);
        uint256 requestID = FHE.requestDecryption(cts, this.callbackUint32.selector);
        addParamsUint256(requestID, input1);
        addParamsUint256(requestID, input2);
    }

    /// @notice Attempt to request decryption of a fake 32-bit unsigned integer (should revert)
    function requestFakeUint32() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000400;
        /// @dev This should revert because the previous handle is not honestly obtained
        FHE.requestDecryption(cts, this.callbackUint32.selector);
    }

    /// @notice Callback function for 32-bit unsigned integer decryption
    /// @param requestID The ID of the decryption request
    /// @param decryptedInput The decrypted 32-bit unsigned integer
    /// @return The result of the computation
    function callbackUint32(
        uint256 requestID,
        uint32 decryptedInput,
        bytes[] memory signatures
    ) public returns (uint32) {
        FHE.checkSignatures(requestID, signatures);
        uint256[] memory params = getParamsUint256(requestID);
        unchecked {
            uint32 result = uint32(uint256(params[0])) + uint32(uint256(params[1])) + decryptedInput;
            yUint32 = result;
            return result;
        }
    }

    /// @notice Request decryption of a 32-bit unsigned integer
    function requestUint32_2() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint32_2);
        FHE.requestDecryption(cts, this.callbackUint32_2.selector);
    }

    function callbackUint32_2(uint256 requestID, uint32 decryptedInput, bytes[] memory signatures) public {
        FHE.checkSignatures(requestID, signatures);
        yUint32_2 = decryptedInput;
    }

    /// @notice Request decryption of a 32-bit unsigned integer
    function requestUint32_3() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint32_3);
        FHE.requestDecryption(cts, this.callbackUint32_3.selector);
    }

    function callbackUint32_3(uint256 requestID, uint32 decryptedInput, bytes[] memory signatures) public {
        FHE.checkSignatures(requestID, signatures);
        yUint32_3 = decryptedInput;
    }

    /// @notice Request decryption of a 64-bit unsigned integer
    function requestUint64() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint64);
        FHE.requestDecryption(cts, this.callbackUint64.selector);
    }

    /// @notice Attempt to request decryption of a fake 64-bit unsigned integer (should revert)
    function requestFakeUint64() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000500;
        /// @dev This should revert because the previous handle is not honestly obtained
        FHE.requestDecryption(cts, this.callbackUint64.selector);
    }

    /// @notice Request decryption of a non-trivial 64-bit unsigned integer
    /// @param inputHandle The input handle for the encrypted value
    /// @param inputProof The input proof for the encrypted value
    function requestUint64NonTrivial(externalEuint64 inputHandle, bytes calldata inputProof) public {
        euint64 inputNonTrivial = FHE.fromExternal(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(inputNonTrivial);
        FHE.requestDecryption(cts, this.callbackUint64.selector);
    }

    /// @notice Callback function for 64-bit unsigned integer decryption
    /// @param decryptedInput The decrypted 64-bit unsigned integer
    /// @return The decrypted value
    function callbackUint64(
        uint256 requestID,
        uint64 decryptedInput,
        bytes[] memory signatures
    ) public returns (uint64) {
        FHE.checkSignatures(requestID, signatures);
        yUint64 = decryptedInput;
        return decryptedInput;
    }

    function requestUint128() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint128);
        FHE.requestDecryption(cts, this.callbackUint128.selector);
    }

    function requestUint128NonTrivial(externalEuint128 inputHandle, bytes calldata inputProof) public {
        euint128 inputNonTrivial = FHE.fromExternal(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(inputNonTrivial);
        FHE.requestDecryption(cts, this.callbackUint128.selector);
    }

    function callbackUint128(
        uint256 requestID,
        uint128 decryptedInput,
        bytes[] memory signatures
    ) public returns (uint128) {
        FHE.checkSignatures(requestID, signatures);
        yUint128 = decryptedInput;
        return decryptedInput;
    }

    function requestUint128_Many() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint128_2);
        FHE.requestDecryption(cts, this.callbackUint128_2.selector);
        bytes32[] memory cts_2 = new bytes32[](1);
        cts_2[0] = FHE.toBytes32(xUint128_3);
        FHE.requestDecryption(cts_2, this.callbackUint128_3.selector);
    }

    function callbackUint128_2(uint256 requestID, uint128 decryptedInput, bytes[] memory signatures) public {
        FHE.checkSignatures(requestID, signatures);
        yUint128_2 = decryptedInput;
    }

    function callbackUint128_3(uint256 requestID, uint128 decryptedInput, bytes[] memory signatures) public {
        FHE.checkSignatures(requestID, signatures);
        yUint128_3 = decryptedInput;
    }

    function requestUint256() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint256);
        FHE.requestDecryption(cts, this.callbackUint256.selector);
    }

    function requestUint256NonTrivial(externalEuint256 inputHandle, bytes calldata inputProof) public {
        euint256 inputNonTrivial = FHE.fromExternal(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(inputNonTrivial);
        FHE.requestDecryption(cts, this.callbackUint256.selector);
    }

    function callbackUint256(
        uint256 requestID,
        uint256 decryptedInput,
        bytes[] memory signatures
    ) public returns (uint256) {
        FHE.checkSignatures(requestID, signatures);
        yUint256 = decryptedInput;
        return decryptedInput;
    }

    function requestEbytes64NonTrivial(externalEbytes64 inputHandle, bytes calldata inputProof) public {
        ebytes64 inputNonTrivial = FHE.fromExternal(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(inputNonTrivial);
        FHE.requestDecryption(cts, this.callbackBytes64.selector);
    }

    function requestEbytes64Trivial(bytes calldata value) public {
        ebytes64 inputTrivial = FHE.asEbytes64(FHE.padToBytes64(value));
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(inputTrivial);
        FHE.requestDecryption(cts, this.callbackBytes64.selector);
    }

    function callbackBytes64(
        uint256 requestID,
        bytes calldata decryptedInput,
        bytes[] memory signatures
    ) public returns (bytes memory) {
        FHE.checkSignatures(requestID, signatures);
        yBytes64 = decryptedInput;
        return decryptedInput;
    }

    function requestEbytes128NonTrivial(externalEbytes128 inputHandle, bytes calldata inputProof) public {
        ebytes128 inputNonTrivial = FHE.fromExternal(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(inputNonTrivial);
        FHE.requestDecryption(cts, this.callbackBytes128.selector);
    }

    function requestEbytes128Trivial(bytes calldata value) public {
        ebytes128 inputTrivial = FHE.asEbytes128(FHE.padToBytes128(value));
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(inputTrivial);
        FHE.requestDecryption(cts, this.callbackBytes128.selector);
    }

    function callbackBytes128(
        uint256 requestID,
        bytes calldata decryptedInput,
        bytes[] memory signatures
    ) public returns (bytes memory) {
        FHE.checkSignatures(requestID, signatures);
        yBytes128 = decryptedInput;
        return decryptedInput;
    }

    function requestEbytes256Trivial(bytes calldata value) public {
        ebytes256 inputTrivial = FHE.asEbytes256(FHE.padToBytes256(value));
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(inputTrivial);
        FHE.requestDecryption(cts, this.callbackBytes256.selector);
    }

    function requestEbytes256NonTrivial(externalEbytes256 inputHandle, bytes calldata inputProof) public {
        ebytes256 inputNonTrivial = FHE.fromExternal(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(inputNonTrivial);
        FHE.requestDecryption(cts, this.callbackBytes256.selector);
    }

    /// @notice Callback function for 256-bit encrypted bytes decryption
    /// @param decryptedInput The decrypted 256-bit bytes
    /// @return The decrypted value
    function callbackBytes256(
        uint256 requestID,
        bytes calldata decryptedInput,
        bytes[] memory signatures
    ) public returns (bytes memory) {
        FHE.checkSignatures(requestID, signatures);
        yBytes256 = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of an encrypted address
    function requestAddress() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xAddress);
        FHE.requestDecryption(cts, this.callbackAddress.selector);
    }

    /// @notice Request decryption of multiple encrypted addresses
    function requestSeveralAddresses() public {
        bytes32[] memory cts = new bytes32[](2);
        cts[0] = FHE.toBytes32(xAddress);
        cts[1] = FHE.toBytes32(xAddress2);
        FHE.requestDecryption(cts, this.callbackAddresses.selector);
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
    ) public returns (address) {
        FHE.checkSignatures(requestID, signatures);
        yAddress = decryptedInput1;
        yAddress2 = decryptedInput2;
        return decryptedInput1;
    }

    /// @notice Attempt to request decryption of a fake address (should revert)
    function requestFakeAddress() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = 0x4200000000000000000000000000000000000000000000000000000000000700;
        /// @dev This should revert because the previous handle is not honestly obtained
        FHE.requestDecryption(cts, this.callbackAddress.selector);
    }

    /// @notice Callback function for address decryption
    /// @param decryptedInput The decrypted address
    /// @return The decrypted address
    function callbackAddress(
        uint256 requestID,
        address decryptedInput,
        bytes[] memory signatures
    ) public returns (address) {
        FHE.checkSignatures(requestID, signatures);
        yAddress = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of mixed data types including 256-bit encrypted bytes
    /// @dev Demonstrates how to include encrypted bytes256 in a mixed decryption request
    /// @param inputHandle The encrypted input handle for the bytes256
    /// @param inputProof The proof for the encrypted bytes256
    function requestMixedBytes256(externalEbytes256 inputHandle, bytes calldata inputProof) public {
        ebytes256 xBytes256 = FHE.fromExternal(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](4);
        cts[0] = FHE.toBytes32(xBool);
        cts[1] = FHE.toBytes32(xAddress);
        cts[2] = FHE.toBytes32(xBytes256);
        ebytes64 input64Bytes = FHE.asEbytes64(FHE.padToBytes64(hex"aaff42"));
        cts[3] = FHE.toBytes32(input64Bytes);
        FHE.requestDecryption(cts, this.callbackMixedBytes256.selector);
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
    ) public {
        FHE.checkSignatures(requestID, signatures);
        yBool = decBool;
        yAddress = decAddress;
        yBytes256 = bytesRes;
        yBytes64 = bytesRes2;
    }

    /// @dev internal setter to link a decryption requestID to a uint256 value
    /// @dev if used multiple times with same requestID, it will map the requestID to the list of all added inputs
    function addParamsUint256(uint256 requestID, uint256 value) internal {
        paramsUint256[requestID].push(value);
    }

    /// @dev internal getter to recover all uint256 values linked to a specific requestID
    function getParamsUint256(uint256 requestID) internal view returns (uint256[] memory) {
        return paramsUint256[requestID];
    }
}
