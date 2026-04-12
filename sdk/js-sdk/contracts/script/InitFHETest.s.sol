// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {console} from "forge-std-1.11.0/Script.sol";
import {FHETest} from "../src/FHETest.sol";
import {FheType} from "@fhevm/solidity/lib/FheType.sol";
import {FHETestScript} from "./FHETestScript.s.sol";

//
// Run in simulation mode:
// -----------------------
// forge script script/InitFHETest.s.sol --rpc-url http://localhost:8545
// FORCE=1 forge script script/InitFHETest.s.sol --rpc-url http://localhost:8545
//
// # Testnet
// CHAIN=testnet && export CHAIN && source ../test/.env && export MNEMONIC && forge script script/InitFHETest.s.sol --rpc-url https://ethereum-sepolia-rpc.publicnode.com
//
// # Devnet
// CHAIN=devnet && export CHAIN && source ../test/.env && export MNEMONIC && forge script script/InitFHETest.s.sol --rpc-url https://ethereum-sepolia-rpc.publicnode.com
//
// Broadcast:
// ----------
// forge script script/InitFHETest.s.sol --rpc-url http://localhost:8545 --broadcast
// FORCE=1 forge script script/InitFHETest.s.sol --rpc-url http://localhost:8545 --broadcast
//
contract InitFHETest is FHETestScript {
    function run() external {
        require(expectedFheTestAddr.code.length > 0, "FHETest not deployed, run DeployFHETest first");

        bool force = vm.envOr("FORCE", uint256(0)) != 0;

        FHETest fheTest = FHETest(expectedFheTestAddr);
        console.log("FHETest at:", expectedFheTestAddr);

        vm.startBroadcast(deployerPrivateKey);

        // Initialize encrypted values and make them publicly decryptable
        if (!fheTest.hasHandleOf(deployer, FheType.Bool) || force) {
            console.log("  Initializing Ebool...");
            fheTest.setClearEbool(true, true);
        }

        if (!fheTest.hasHandleOf(deployer, FheType.Uint8) || force) {
            console.log("  Initializing Euint8...");
            fheTest.setClearEuint8(type(uint8).max, true);
        }

        if (!fheTest.hasHandleOf(deployer, FheType.Uint16) || force) {
            console.log("  Initializing Euint16...");
            fheTest.setClearEuint16(type(uint16).max, true);
        }

        if (!fheTest.hasHandleOf(deployer, FheType.Uint32) || force) {
            console.log("  Initializing Euint32...");
            fheTest.setClearEuint32(type(uint32).max, true);
        }

        if (!fheTest.hasHandleOf(deployer, FheType.Uint64) || force) {
            console.log("  Initializing Euint64...");
            fheTest.setClearEuint64(type(uint64).max, true);
        }

        if (!fheTest.hasHandleOf(deployer, FheType.Uint128) || force) {
            console.log("  Initializing Euint128...");
            fheTest.setClearEuint128(type(uint128).max, true);
        }

        if (!fheTest.hasHandleOf(deployer, FheType.Uint256) || force) {
            console.log("  Initializing Euint256...");
            fheTest.setClearEuint256(type(uint256).max, true);
        }

        if (!fheTest.hasHandleOf(deployer, FheType.Uint160) || force) {
            console.log("  Initializing Address...");
            fheTest.setClearEaddress(address(type(uint160).max), true);
        }

        vm.stopBroadcast();

        console.log("Done.");
    }
}
