// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "../decryptionOracleLib/DecryptionOracleCaller.sol";
import "../addresses/DecryptionOracleAddress.sol";

contract TestInput is DecryptionOracleCaller {
    ebool xBool;
    euint8 xUint8;
    euint64 xUint64;
    eaddress xAddress;
    bool public yBool;
    uint8 public yUint8;
    uint64 public yUint64;
    address public yAddress;

    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
        setDecryptionOracle(DECRYPTION_ORACLE_ADDRESS);
    }

    function requestUint64NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        euint64 inputNonTrivial = TFHE.asEuint64(inputHandle, inputProof);
        uint256[] memory cts = new uint256[](1);
        cts[0] = toUint256(inputNonTrivial);
        requestDecryption(cts, this.callbackUint64.selector);
    }

    function callbackUint64(
        uint256 requestID,
        uint64 decryptedInput,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) {
        yUint64 = decryptedInput;
    }

    function requestMixedNonTrivial(
        einput inputHandleBool,
        einput inputHandleUint8,
        einput inputHandleAddress,
        bytes calldata inputProof
    ) public {
        ebool encBool = TFHE.asEbool(inputHandleBool, inputProof);
        euint8 encUint8 = TFHE.asEuint8(inputHandleUint8, inputProof);
        eaddress encAddress = TFHE.asEaddress(inputHandleAddress, inputProof);
        uint256[] memory cts = new uint256[](3);
        cts[0] = toUint256(encBool);
        cts[1] = toUint256(encUint8);
        cts[2] = toUint256(encAddress);
        requestDecryption(cts, this.callbackMixed.selector);
    }

    function callbackMixed(
        uint256 requestID,
        bool decryptedBool,
        uint8 decryptedUint8,
        address decryptedAddress,
        bytes[] memory signatures
    ) public checkSignatures(requestID, signatures) {
        yBool = decryptedBool;
        yUint8 = decryptedUint8;
        yAddress = decryptedAddress;
    }
}
