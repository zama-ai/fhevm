// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @dev Test-only re-export so Hardhat's compile:specific('examples') step picks up
 *      the LayerZero V2 endpoint mock from node_modules. Without one of these imports
 *      living under a compiled source path, Hardhat does not produce artifacts for
 *      the mock and it cannot be deployed from TypeScript via getContractFactory.
 *
 *      Note: we deliberately do NOT import SimpleMessageLibMock because it transitively
 *      pulls in TestHelperOz5 which depends on forge-std (incompatible with Hardhat).
 *      The Hardhat tests in test/bridge/ never trigger an actual LayerZero send through
 *      the endpoint — they impersonate the endpoint to inject lzReceive/lzCompose
 *      directly, which does not require a registered send/receive library.
 */

// solhint-disable-next-line no-unused-import
import {EndpointV2Mock} from "@layerzerolabs/test-devtools-evm-foundry/contracts/mocks/EndpointV2Mock.sol";
