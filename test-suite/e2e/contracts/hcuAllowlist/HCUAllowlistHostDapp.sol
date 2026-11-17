// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import {ZamaMultiChainConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {HCUAllowlistDapp} from "./HCUAllowlistDapp.sol";

/// @notice HCUAllowlistDapp with the Zama config of the chain it is deployed on.
/// @dev HCUAllowlistDapp inherits E2ECoprocessorConfig, whose constructor writes the local addresses of the E2E suite.
/// Solidity runs the base constructors in the order of the base list, and each one overwrites the config.
/// Keep ZamaMultiChainConfig last in the list, so that the config of the current chain is the final one.
contract HCUAllowlistHostDapp is HCUAllowlistDapp, ZamaMultiChainConfig {}
