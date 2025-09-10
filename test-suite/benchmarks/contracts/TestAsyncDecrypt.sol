// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import { E2ECoprocessorConfig } from "./E2ECoprocessorConfig.sol";

/// @notice Contract for testing asynchronous decryption using the Gateway
contract TestAsyncDecrypt is E2ECoprocessorConfig {
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
        bytes memory cleartexts,
        bytes memory decryptionProof
    ) public returns (bool) {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        uint256 i = 0;
        while (true) {
            i++;
        }
        (bool decryptedInput) = abi.decode(cleartexts, (bool));
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
    function callbackBool(uint256 requestID, bytes memory cleartexts, bytes memory decryptionProof) public returns (bool) {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (bool decryptedInput) = abi.decode(cleartexts, (bool));
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
    /// @param cleartexts The decrypted 8-bit unsigned integer ABI encoded in bytes
    /// @return The decrypted value
    function callbackUint8(uint256 requestID, bytes memory cleartexts, bytes memory decryptionProof) public returns (uint8) {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (uint8 decryptedInput) = abi.decode(cleartexts, (uint8));
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
    /// @param requestID The ID of the decryption request
    /// @param cleartexts The decrypted 16-bit unsigned integer ABI encoded in bytes
    /// @param decryptionProof The decryption proof containing KMS signatures and extra data
    /// @return The decrypted value
    function callbackUint16(
        uint256 requestID,
        bytes memory cleartexts,
        bytes memory decryptionProof
    ) public returns (uint16) {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (uint16 decryptedInput) = abi.decode(cleartexts, (uint16));
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
    /// @param cleartexts The decrypted 32-bit unsigned integer ABI encoded in bytes
    /// @param decryptionProof The decryption proof containing KMS signatures and extra data
    /// @return The result of the computation
    function callbackUint32(
        uint256 requestID,
        bytes memory cleartexts,
        bytes memory decryptionProof
    ) public returns (uint32) {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        uint256[] memory params = getParamsUint256(requestID);
        (uint32 decryptedInput) = abi.decode(cleartexts, (uint32));
        unchecked {
            uint32 result = uint32(uint256(params[0])) + uint32(uint256(params[1])) + decryptedInput;
            yUint32 = result;
            return result;
        }
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
    /// @param requestID The ID of the decryption request
    /// @param cleartexts The decrypted 64-bit unsigned integer ABI encoded in bytes
    /// @param decryptionProof The decryption proof containing KMS signatures and extra data
    /// @return The decrypted value
    function callbackUint64(
        uint256 requestID,
        bytes memory cleartexts,
        bytes memory decryptionProof
    ) public returns (uint64) {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (uint64 decryptedInput) = abi.decode(cleartexts, (uint64));
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
        bytes memory cleartexts,
        bytes memory decryptionProof
    ) public returns (uint128) {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (uint128 decryptedInput) = abi.decode(cleartexts, (uint128));
        yUint128 = decryptedInput;
        return decryptedInput;
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
        bytes memory cleartexts,
        bytes memory decryptionProof
    ) public returns (uint256) {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (uint256 decryptedInput) = abi.decode(cleartexts, (uint256));
        yUint256 = decryptedInput;
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
    /// @param requestID The ID of the decryption request
    /// @param cleartexts The 2 decrypted addresses ABI encoded in bytes
    /// @param decryptionProof The decryption proof containing KMS signatures and extra data
    /// @return The first decrypted address
    function callbackAddresses(
        uint256 requestID,
        bytes memory cleartexts,
        bytes memory decryptionProof
    ) public returns (address) {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (address decryptedInput1, address decryptedInput2) = abi.decode(cleartexts, (address, address));
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
    /// @param requestID The ID of the decryption request
    /// @param cleartexts The decrypted address ABI encoded in bytes
    /// @param decryptionProof The decryption proof containing KMS signatures and extra data
    /// @return The decrypted address
    function callbackAddress(
        uint256 requestID,
        bytes memory cleartexts,
        bytes memory decryptionProof
    ) public returns (address) {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (address decryptedInput) = abi.decode(cleartexts, (address));
        yAddress = decryptedInput;
        return decryptedInput;
    }

    /// @notice Request decryption of mixed data types
    /// @dev Demonstrates how to do a mixed decryption request
    /// @param inputHandle The encrypted input handle for euint256
    /// @param inputProof The proof for the encrypted euint256
    function requestMixed(externalEuint256 inputHandle, bytes calldata inputProof) public {
        bytes32[] memory cts = new bytes32[](4);
        cts[0] = FHE.toBytes32(xBool);
        cts[1] = FHE.toBytes32(xAddress);
        cts[2] = FHE.toBytes32(xUint32);
        euint256 inputEuint256 = FHE.fromExternal(inputHandle, inputProof);
        cts[3] = FHE.toBytes32(inputEuint256);
        FHE.requestDecryption(cts, this.callbackMixed.selector);
    }

    /// @notice Callback function for mixed data type decryption including 256-bit encrypted bytes
    /// @dev Processes and stores the decrypted values
    /// @param requestID The ID of the decryption request
    /// @param cleartexts The decrypted values ABI encoded in bytes
    /// @param decryptionProof The decryption proof containing KMS signatures and extra data
    function callbackMixed(
        uint256 requestID,
        bytes memory cleartexts,
        bytes memory decryptionProof
    ) public {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (bool decBool, address decAddress, uint32 decEuint32, uint256 decEuint256) = abi.decode(cleartexts, (bool, address, uint32, uint256));
        yBool = decBool;
        yAddress = decAddress;
        yUint32 = decEuint32;
        yUint256 = decEuint256;
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

    /// @notice Request decryption of a 32-bit unsigned integer
    function requestUint32_2() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint32_2);
        FHE.requestDecryption(cts, this.callbackUint32_2.selector);
    }

    function callbackUint32_2(uint256 requestID, bytes memory cleartexts, bytes memory decryptionProof) public {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (uint32 decryptedInput) = abi.decode(cleartexts, (uint32));
        yUint32_2 = decryptedInput;
    }

    /// @notice Request decryption of a 32-bit unsigned integer
    function requestUint32_3() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint32_3);
        FHE.requestDecryption(cts, this.callbackUint32_3.selector);
    }

    function callbackUint32_3(uint256 requestID, bytes memory cleartexts, bytes memory decryptionProof) public {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (uint32 decryptedInput) = abi.decode(cleartexts, (uint32));
        yUint32_3 = decryptedInput;
    }

    function requestUint128_Many() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint128_2);
        FHE.requestDecryption(cts, this.callbackUint128_2.selector);
        bytes32[] memory cts_2 = new bytes32[](1);
        cts_2[0] = FHE.toBytes32(xUint128_3);
        FHE.requestDecryption(cts_2, this.callbackUint128_3.selector);
    }

    function callbackUint128_2(uint256 requestID, bytes memory cleartexts, bytes memory decryptionProof) public {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (uint128 decryptedInput) = abi.decode(cleartexts, (uint128));
        yUint128_2 = decryptedInput;
    }

    function callbackUint128_3(uint256 requestID, bytes memory cleartexts, bytes memory decryptionProof) public {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        (uint128 decryptedInput) = abi.decode(cleartexts, (uint128));
        yUint128_3 = decryptedInput;
    }
}
