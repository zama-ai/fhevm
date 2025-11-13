// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CoprocessorConfig, Impl} from "../lib/Impl.sol";
import {ZamaEthereumConfig} from "../config/ZamaConfig.sol";

/// @notice A simple contract for testing CoprocessorConfig in constructor
contract TestEthereumCoprocessorConfig is ZamaEthereumConfig {
    function getCoprocessorConfig() public pure returns (CoprocessorConfig memory) {
        return Impl.getCoprocessorConfig();
    }
}
