// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Script, console, console2} from "forge-std/Script.sol";
import {DeployLib} from "./libraries/DeployLib.sol";
import {FhevmConfigLib} from "./libraries/FhevmConfigLib.sol";
import {Signer} from "./libraries/SignerLib.sol";
import {Signer} from "./libraries/SignerLib.sol";
import {FhevmAddresses} from "./libraries/structs/FhevmAddressesStruct.sol";
import {
    aclAdd,
    fhevmExecutorAdd,
    kmsVerifierAdd,
    inputVerifierAdd,
    hcuLimitAdd,
    pauserSetAdd
} from "../src/host-contracts/addresses/FHEVMHostAddresses.sol";

contract Deploy is Script {
    function run() external {
        DeployLib.deployAuto(vm);
    }
}

contract PrintFhevmSigners is Script {
    function run() external {
        console2.log("JSON_RESULT_START");
        console2.log(FhevmConfigLib.resolveDeployersAsJson(vm));
        console2.log("JSON_RESULT_END");
    }
}

string constant FHEVM_HOST_ADDRESSES_FILE = "src/host-contracts/addresses/FHEVMHostAddresses.sol";

contract WriteFHEVMHostAddressesDotSol is Script {
    function run() external {
        FhevmAddresses memory a = DeployLib.preComputeAddressesAuto(vm, false);

        DeployLib.writeAddressesFile(vm, FHEVM_HOST_ADDRESSES_FILE, a);

        console.log("\nAddresses file written to:");
        console.log(string.concat("  ", vm.projectRoot(), "/", FHEVM_HOST_ADDRESSES_FILE));
    }
}

contract Verify is Script {
    function run() external {
        FhevmAddresses memory a = DeployLib.preComputeAddressesAuto(vm, false);

        DeployLib.verifyDeploy(a);
    }
}
