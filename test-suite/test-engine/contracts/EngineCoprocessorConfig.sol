// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CoprocessorConfig, FHE} from "@fhevm/solidity/lib/FHE.sol";

/// @notice Base of every engine test contract: the protocol addresses are a constructor argument
/// (`this.engine.contracts.coprocessorConfig()`, from the profile), so one build serves every
/// environment. `FHE.setCoprocessor` stores them; the bytecode holds no address.
abstract contract EngineCoprocessorConfig {
    constructor(CoprocessorConfig memory config) {
        FHE.setCoprocessor(config);
    }
}
