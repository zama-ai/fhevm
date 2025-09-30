// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/FHE.sol";
import {CoprocessorSetup} from "./CoprocessorSetup.sol";

/// @notice First contract for testing asynchronous decryption via multiple contracts
contract TestAsyncDecryptA {
    euint64 xUint64;
    uint64 public yUint64;

    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
        xUint64 = FHE.asEuint64(424242);
        FHE.allowThis(xUint64);
    }

    function requestUint64() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint64);
        FHE.requestDecryption(cts, this.callbackUint64.selector);
    }

    function callbackUint64(uint256 requestID, bytes memory cleartexts, bytes memory decryptionProof) public {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        uint64 decryptedInput = abi.decode(cleartexts, (uint64));
        yUint64 = decryptedInput;
        revert();
    }
}

/// @notice Second contract for testing asynchronous decryption via multiple contracts
contract TestAsyncDecryptB {
    euint64 xUint64;
    uint64 public yUint64;

    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
        xUint64 = FHE.asEuint64(373737);
        FHE.allowThis(xUint64);
    }

    function requestUint64() public {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(xUint64);
        FHE.requestDecryption(cts, this.callbackUint64.selector);
    }

    function callbackUint64(uint256 requestID, bytes memory cleartexts, bytes memory decryptionProof) public {
        FHE.checkSignatures(requestID, cleartexts, decryptionProof);
        uint64 decryptedInput = abi.decode(cleartexts, (uint64));
        yUint64 = decryptedInput;
    }
}
