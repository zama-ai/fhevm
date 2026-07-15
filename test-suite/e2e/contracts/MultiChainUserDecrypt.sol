// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "./E2ECoprocessorConfigLocal.sol";

/// @notice Minimal user-decryption fixture for multi-chain isolation tests.
contract MultiChainUserDecrypt is E2ECoprocessorConfig {
    euint64 public xUint64;

    constructor() {
        xUint64 = FHE.asEuint64(18446744073709551600);
        FHE.allowThis(xUint64);
        FHE.allow(xUint64, msg.sender);
    }
}
