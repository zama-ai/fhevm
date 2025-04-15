// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "../decryptionOracleLib/DecryptionOracleCaller.sol";

contract TestInput {
    uint64 public yUint64;

    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
    }

    function requestUint64NonTrivial(einput inputHandle, bytes calldata inputProof) public {
        euint64 inputNonTrivial = TFHE.asEuint64(inputHandle, inputProof);
        TFHE.allowThis(inputNonTrivial);
    }
}