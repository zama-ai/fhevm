// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Script, console, console2} from "forge-std/Script.sol";
import {AssertLib} from "./libraries/AssertLib.sol";
import {DeployLib} from "./libraries/DeployLib.sol";
import {FhevmConfigLib} from "./libraries/FhevmConfigLib.sol";
import {FhevmAddresses} from "./libraries/structs/FhevmAddressesStruct.sol";

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

        FhevmAddresses memory a = DeployLib.preComputeAddressesAuto(vm, false);

        DeployLib.writeAddressesFile(vm, addressesFile, a);

        console.log("\nAddresses file written to:");
        console.log(string.concat("  ", vm.projectRoot(), "/", addressesFile));
    }
}

contract PrintFHEVMHostAddressesDotSol is Script {
    function run() external view {
        FhevmAddresses memory a = DeployLib.preComputeAddressesAuto(vm, false);

        console.log("");
        console.log("================================================================================");
        console.log("FHEVM Host Addresses");
        console.log("================================================================================");
        console.log("  acl                                       :", a.acl);
        console.log("  fhevmExecutor                             :", a.fhevmExecutor);
        console.log("  kmsVerifier                               :", a.kmsVerifier);
        console.log("  inputVerifier                             :", a.inputVerifier);
        console.log("  hcuLimit                                  :", a.hcuLimit);
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
        FhevmAddresses memory a = DeployLib.preComputeAddressesAuto(vm, false);

        DeployLib.verifyDeploy(a);
    }
}
