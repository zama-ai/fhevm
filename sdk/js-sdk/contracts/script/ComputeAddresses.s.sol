// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Script, console} from "forge-std/Script.sol";
import {Signer, FhevmAddresses, FhevmAddressesLib} from "./libraries/FhevmAddressesLib.sol";
import {FHEVM_HOST_ADDRESSES_FILE} from "./libraries/Constants.sol";

/**
 * @title ComputeAddresses
 * @notice Step 1 of the two-phase FHEVM host deployment.
 *
 * Computes the deterministic CREATE addresses for all 5 UUPS proxies and
 * PauserSet, then writes them into FHEVMHostAddresses.sol so that the
 * implementation contracts can be compiled with the correct baked-in
 * addresses before the actual deployment runs.
 *
 * Usage:
 *
 *   DEPLOYER_PRIVATE_KEY=<key> forge script script/ComputeAddresses.s.sol \
 *       --rpc-url <rpc>
 *
 * After running this script, execute `forge build` and then run
 * DeployFHEVMHost.s.sol with the same deployer key and nonce.
 */
contract ComputeAddresses is Script {
    function run() external {
        Signer memory d = FhevmAddressesLib.resolveDeployerFromEnv(vm);
        uint64 nonce = vm.getNonce(d.addr);

        FhevmAddresses memory a = FhevmAddressesLib.compute(vm, d.addr, nonce);

        console.log("Deployer:         ", d.addr);
        console.log("Starting nonce:   ", nonce);
        console.log("aclAdd:           ", a.acl);
        console.log("fhevmExecutorAdd: ", a.fhevmExecutor);
        console.log("kmsVerifierAdd:   ", a.kmsVerifier);
        console.log("inputVerifierAdd: ", a.inputVerifier);
        console.log("hcuLimitAdd:      ", a.hcuLimit);
        console.log("pauserSetAdd:     ", a.pauserSet);

        FhevmAddressesLib.writeAddressesFile(vm, FHEVM_HOST_ADDRESSES_FILE, a);

        console.log("\nAddresses file written to:");
        console.log(string.concat("  ", vm.projectRoot(), "/", FHEVM_HOST_ADDRESSES_FILE));
        console.log("Run `forge build` then DeployFHEVMHost.s.sol.");
    }
}
