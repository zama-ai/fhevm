// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/FHE.sol";
import {CoprocessorSetup} from "../lib/CoprocessorSetup.sol";

contract TestInput {
    euint64 _xUint64;

    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    }

    function setUint64(externalEuint64 inputHandle, bytes calldata inputProof) public {
        euint64 inputEuint64 = FHE.fromExternal(inputHandle, inputProof);
        FHE.allowThis(inputEuint64);
        FHE.allow(inputEuint64, msg.sender);
        _xUint64 = inputEuint64;
    }

    function getEuint64() public view returns (euint64) {
        return _xUint64;
    }
}
