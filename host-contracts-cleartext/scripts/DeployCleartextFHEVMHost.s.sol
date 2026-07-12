// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Script, console, console2} from "forge-std/Script.sol";
import {AssertLib} from "./libraries/AssertLib.sol";
import {DeployLib} from "./libraries/DeployLib.sol";
import {FhevmConfigLib} from "./libraries/FhevmConfigLib.sol";
import {FhevmAddresses} from "./libraries/structs/FhevmAddressesStruct.sol";
// import {
//     aclAdd,
//     fhevmExecutorAdd,
//     kmsVerifierAdd,
//     inputVerifierAdd,
//     hcuLimitAdd,
//     pauserSetAdd,
//     kmsGenerationAdd,
//     protocolConfigAdd
// } from "../../src/v0.13.0/host-contracts/addresses/FHEVMHostAddresses.sol";

contract Deploy is Script {
    function run() external {
        DeployLib.deployAuto(vm);
    }
}

contract PrintFhevmSigners is Script {
    function run() external view {
        console2.log("JSON_RESULT_START");
        console2.log(FhevmConfigLib.resolveDeployersAsJson(vm));
        console2.log("JSON_RESULT_END");
    }
}

contract WriteFHEVMHostAddressesDotSol is Script {
    function run() external {
        AssertLib.assertEnvExists(vm, "FHEVM_HOST_ADDRESSES_FILE");
        string memory addressesFile = vm.envString("FHEVM_HOST_ADDRESSES_FILE");

        FhevmAddresses memory a = DeployLib.preComputeAddresses(vm, false);

        DeployLib.writeAddressesFile(vm, addressesFile, a);

        console.log("\nAddresses file written to:");
        console.log(string.concat("  ", vm.projectRoot(), "/", addressesFile));
    }
}

contract PrintFHEVMHostAddressesDotSol is Script {
    function run() external view {
        FhevmAddresses memory a = DeployLib.preComputeAddresses(vm, false);

        console.log("");
        console.log("================================================================================");
        console.log("FHEVM Host Addresses");
        console.log("================================================================================");
        console.log("  acl                                       :", a.acl);
        console.log("  fhevmExecutor                             :", a.fhevmExecutor);
        console.log("  kmsVerifier                               :", a.kmsVerifier);
        console.log("  inputVerifier                             :", a.inputVerifier);
        console.log("  hcuLimit                                  :", a.hcuLimit);
        console.log("  protocolConfig                            :", a.protocolConfig);
        console.log("  kmsGeneration                             :", a.kmsGeneration);
        console.log("  pauserSet                                 :", a.pauserSet);
        console.log("  verifyingContractAddressInputVerification :", a.verifyingContractAddressInputVerification);
        console.log("  verifyingContractAddressDecryption        :", a.verifyingContractAddressDecryption);
        console.log("--------------------------------------------------------------------------------");
        console.log("Pausers (count):", a.pausers.length);
        for (uint256 i = 0; i < a.pausers.length; i++) {
            console.log(string.concat("  pauser[", vm.toString(i), "]"), a.pausers[i].addr);
        }
        console.log("================================================================================");
    }
}

contract Verify is Script {
    function run() external view {
        FhevmAddresses memory a = DeployLib.preComputeAddresses(vm, false);

        DeployLib.verifyDeploy(a);
    }
}

/*
// cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 1
address constant aclAdd = 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D;

// cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 3
address constant fhevmExecutorAdd = 0xe3a9105a3a932253A70F126eb1E3b589C643dD24;

// cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 4
address constant kmsVerifierAdd = 0x901F8942346f7AB3a01F6D7613119Bca447Bb030;

// cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 5
address constant inputVerifierAdd = 0x36772142b74871f255CbD7A3e89B401d3e45825f;

// cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 6
address constant hcuLimitAdd = 0x233ff88A48c172d29F675403e6A8e302b0F032D9;

// cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 7
address constant protocolConfigAdd = 0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc;

// cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 8
address constant kmsGenerationAdd = 0x216be43148dB537BeddBC268163deb1a802b5553;

// cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 9
address constant pauserSetAdd = 0xded0D2a71268DC12622BdD1b55d68a1CB5662327;
*/