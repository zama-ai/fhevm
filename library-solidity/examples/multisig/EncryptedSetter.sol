// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/FHE.sol";
import {CoprocessorSetup} from "../CoprocessorSetup.sol";

contract EncryptedSetter {
    euint64 public encryptedResult;

    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    }

    function setEncryptedValue(externalEuint64 inputHandle, bytes memory inputProof) external {
        euint64 encryptedInput = FHE.fromExternal(inputHandle, inputProof);
        encryptedResult = FHE.add(encryptedInput, 42); // simulate some computation
        FHE.allowThis(encryptedResult);
        FHE.allow(encryptedResult, msg.sender);
    }
}
