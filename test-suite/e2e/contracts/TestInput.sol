// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import { E2EFHEVMConfig } from "./E2EFHEVMConfigLocal.sol";

contract TestInput is E2EFHEVMConfig {
    uint64 public yUint64;

    function requestUint64NonTrivial(externalEuint64 inputHandle, bytes calldata inputProof) public {
        euint64 inputNonTrivial = FHE.fromExternal(inputHandle, inputProof);
        FHE.allowThis(inputNonTrivial);
    }
}