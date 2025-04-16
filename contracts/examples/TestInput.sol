// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/HTTPZ.sol";
import "../addresses/DecryptionOracleAddress.sol";
import "../lib/HTTPZConfig.sol";

contract TestInput {
    ebool xBool;
    euint8 xUint8;
    euint64 xUint64;
    eaddress xAddress;
    bool public yBool;
    uint8 public yUint8;
    uint64 public yUint64;
    address public yAddress;

    constructor() {
        HTTPZ.setCoprocessor(HTTPZConfig.defaultConfig());
        HTTPZ.setDecryptionOracle(DECRYPTION_ORACLE_ADDRESS);
    }

    function requestUint64NonTrivial(externalEuint64 inputHandle, bytes calldata inputProof) public {
        euint64 inputNonTrivial = HTTPZ.fromExternal(inputHandle, inputProof);
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = HTTPZ.toBytes32(inputNonTrivial);
        HTTPZ.requestDecryption(cts, this.callbackUint64.selector);
    }

    function callbackUint64(uint256 requestID, uint64 decryptedInput, bytes[] memory signatures) public {
        HTTPZ.checkSignatures(requestID, signatures);
        yUint64 = decryptedInput;
    }

    function requestMixedNonTrivial(
        externalEbool inputHandleBool,
        externalEuint8 inputHandleUint8,
        externalEaddress inputHandleAddress,
        bytes calldata inputProof
    ) public {
        ebool encBool = HTTPZ.fromExternal(inputHandleBool, inputProof);
        euint8 encUint8 = HTTPZ.fromExternal(inputHandleUint8, inputProof);
        eaddress encAddress = HTTPZ.fromExternal(inputHandleAddress, inputProof);
        bytes32[] memory cts = new bytes32[](3);
        cts[0] = HTTPZ.toBytes32(encBool);
        cts[1] = HTTPZ.toBytes32(encUint8);
        cts[2] = HTTPZ.toBytes32(encAddress);
        HTTPZ.requestDecryption(cts, this.callbackMixed.selector);
    }

    function callbackMixed(
        uint256 requestID,
        bool decryptedBool,
        uint8 decryptedUint8,
        address decryptedAddress,
        bytes[] memory signatures
    ) public {
        HTTPZ.checkSignatures(requestID, signatures);
        yBool = decryptedBool;
        yUint8 = decryptedUint8;
        yAddress = decryptedAddress;
    }
}
