// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CoprocessorConfig, Impl} from "../lib/Impl.sol";
import {ZamaEthereumConfig} from "../config/ZamaConfig.sol";

/// @notice A simple contract for only testing solidity compilation
contract TestEthereumCoprocessorConfig is ZamaEthereumConfig {
    function getCoprocessorConfig() public pure returns (CoprocessorConfig memory) {
        return Impl.getCoprocessorConfig();
    }
}
