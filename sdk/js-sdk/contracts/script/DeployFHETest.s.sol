// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {console} from "forge-std-1.11.0/Script.sol";
import {FHETest} from "../src/FHETest.sol";
import {LocalConfig} from "../src/LocalConfig.sol";
import {FHETestScript} from "./FHETestScript.s.sol";

//
// Run in simulation mode:
// -----------------------
// forge script script/DeployFHETest.s.sol --rpc-url http://localhost:8545
//
// # Sepolia
// CHAIN=testnet && export CHAIN && source ../test/.env && export MNEMONIC && forge script script/DeployFHETest.s.sol --rpc-url https://ethereum-sepolia-rpc.publicnode.com
//
// # Devnet
// CHAIN=devnet && export CHAIN && source ../test/.env && export MNEMONIC && forge script script/DeployFHETest.s.sol --rpc-url https://ethereum-sepolia-rpc.publicnode.com
//
// Deploy:
// -------
// forge script script/DeployFHETest.s.sol --rpc-url http://localhost:8545 --broadcast
//
contract DeployFHETest is FHETestScript {
    function run() external {
        FHETest fheTest;
        // Skip deploy if already deployed, verify config
        if (expectedFheTestAddr.code.length > 0) {
            console.log("FHETest already deployed at:", expectedFheTestAddr);

            fheTest = FHETest(expectedFheTestAddr);
            verifyCoprocessorConfig(fheTest);

            console.log("CoprocessorConfig verified OK");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);

        fheTest = new FHETest();
        console.log("FHETest deployed at:", address(fheTest));

        fheTest.setCoprocessorConfig(coprocessorConfig);

        vm.stopBroadcast();
    }
}
