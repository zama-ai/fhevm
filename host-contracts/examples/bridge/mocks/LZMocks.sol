// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @dev Test-only re-export so Hardhat's compile:specific('examples') step picks up
 *      the LayerZero V2 endpoint mock from node_modules.
 */

// solhint-disable-next-line no-unused-import
import {EndpointV2Mock} from "@layerzerolabs/test-devtools-evm-foundry/contracts/mocks/EndpointV2Mock.sol";
