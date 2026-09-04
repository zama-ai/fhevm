// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/FHE.sol";
import {CoprocessorSetup} from "../CoprocessorSetup.sol";

contract EncryptedSetter {
    euint64 public encryptedResult64;
    euint32 public encryptedValue32;
    euint64 public encryptedValue64;

    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    }

    function computeResult64(externalEuint64 inputHandle, bytes memory inputProof) external {
        euint64 encryptedInput64 = FHE.fromExternal(inputHandle, inputProof);
        encryptedResult64 = FHE.add(encryptedInput64, 42); // simulate some computation
        FHE.allowThis(encryptedResult64);
        FHE.allow(encryptedResult64, msg.sender);
    }

    function setEncryptedValue32(
        externalEuint32 inputHandle,
        bytes memory inputProof,
        bool allowSender,
        bool allowContract
    ) external {
        encryptedValue32 = FHE.fromExternal(inputHandle, inputProof);
        if (allowSender) {
            FHE.allow(encryptedValue32, msg.sender);
        }
        if (allowContract) {
            FHE.allowThis(encryptedValue32);
        }
    }

    function setEncryptedValue64(
        externalEuint64 inputHandle,
        bytes memory inputProof,
        bool allowSender,
        bool allowContract
    ) external {
        encryptedValue64 = FHE.fromExternal(inputHandle, inputProof);
        if (allowSender) {
            FHE.allow(encryptedValue64, msg.sender);
        }
        if (allowContract) {
            FHE.allowThis(encryptedValue64);
        }
    }
}
