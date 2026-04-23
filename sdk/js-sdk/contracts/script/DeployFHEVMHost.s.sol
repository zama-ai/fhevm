// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Script, console} from "forge-std/Script.sol";
import {Signer, FhevmAddresses, FhevmAddressesLib} from "./libraries/FhevmAddressesLib.sol";
import {INPUT_VERIFICATION_ADDRESS, DECRYPTION_ADDRESS, ANVIL_MNEMONIC} from "./libraries/Constants.sol";
import {EmptyUUPSProxyACL} from "@fhevm/host-contracts/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {CleartextACL} from "@fhevm/cleartext/CleartextACL.sol";
import {CleartextKMSVerifier} from "@fhevm/cleartext/CleartextKMSVerifier.sol";
import {ACL} from "@fhevm/host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "@fhevm/host-contracts/contracts/FHEVMExecutor.sol";
import {HCULimit} from "@fhevm/host-contracts/contracts/HCULimit.sol";
import {InputVerifier} from "@fhevm/host-contracts/contracts/InputVerifier.sol";
import {KMSVerifier} from "@fhevm/host-contracts/contracts/KMSVerifier.sol";
import {PauserSet} from "@fhevm/host-contracts/contracts/immutable/PauserSet.sol";
import {FHETest} from "../../src/FHETest.sol";
import {CleartextInputVerifier} from "@fhevm/cleartext/CleartextInputVerifier.sol";
import {CleartextHCULimit} from "@fhevm/cleartext/CleartextHCULimit.sol";
import {CleartextFHEVMExecutor} from "@fhevm/cleartext/CleartextFHEVMExecutor.sol";

import {
    aclAdd,
    fhevmExecutorAdd,
    kmsVerifierAdd,
    inputVerifierAdd,
    hcuLimitAdd,
    pauserSetAdd
} from "@fhevm/host-contracts/addresses/FHEVMHostAddresses.sol";

contract DeployFHEVMHost is Script {
    function run() external {
        (uint256 deployerKey, uint256 emptyUupsDeployerKey, address deployer,) = _resolveAndFundDeployers();

        FhevmAddresses memory a = FhevmAddressesLib.compute(vm, deployer, 0);

        console.log("Deployer:         ", deployer);
        console.log("Deployer nonce:   ", vm.getNonce(deployer));

        // Empty UUPS impls are deployed under a dedicated secondary key so the
        // main deployer's nonce timeline is preserved for the proxies. The
        // secondary key must be pre-funded with gas on a real chain.
        vm.startBroadcast(emptyUupsDeployerKey);
        address emptyUupsProxyACLAddress = FhevmAddressesLib.newEmptyUUPSProxyACL();
        vm.stopBroadcast();

        vm.startBroadcast(deployerKey);

        FhevmAddressesLib.deployPauserSetAt(a.pauserSet);
        require(vm.getNonce(deployer) == 1, "Expecting nonce=1");

        FhevmAddressesLib.deployACLAt(deployer, a.acl, emptyUupsProxyACLAddress);
        require(vm.getNonce(deployer) == 2, "Expecting nonce=2");

        address emptyUupsProxyAddress = FhevmAddressesLib.newEmptyUUPSProxy();
        require(vm.getNonce(deployer) == 3, "Expecting nonce=3");

        FhevmAddressesLib.deployFHEVMExecutorAt(a.fhevmExecutor, emptyUupsProxyAddress);
        require(vm.getNonce(deployer) == 4, "Expecting nonce=4");

        FhevmAddressesLib.deployKMSVerifierAt(a.kmsVerifier, emptyUupsProxyAddress);
        require(vm.getNonce(deployer) == 5, "Expecting nonce=5");

        FhevmAddressesLib.deployInputVerifierAt(a.inputVerifier, emptyUupsProxyAddress);
        require(vm.getNonce(deployer) == 6, "Expecting nonce=6");

        FhevmAddressesLib.deployHCULimitAt(a.hcuLimit, emptyUupsProxyAddress);
        require(vm.getNonce(deployer) == 7, "Expecting nonce=7");

        FhevmAddressesLib.setACLImplementation(a.acl, address(new CleartextACL()));
        FhevmAddressesLib.setFHEVMExecutorImplementation(a.fhevmExecutor, address(new CleartextFHEVMExecutor()));

        // The KMS/Input/HCU upgrades each need their own bundle of config
        // values. Wrap each in a scope block so those locals free up stack
        // slots as soon as the call returns.
        {
            uint64 chainIdGateway = FhevmAddressesLib.resolveGatewayChainIdFromEnv(vm);
            address[] memory kmsSigners = FhevmAddressesLib.resolveKmsSignersFromEnv(vm);
            uint256 kmsThreshold = FhevmAddressesLib.resolveKmsThresholdFromEnv(vm);
            FhevmAddressesLib.setKMSVerifierImplementation(
                a.kmsVerifier,
                address(new CleartextKMSVerifier()),
                kmsSigners,
                kmsThreshold,
                DECRYPTION_ADDRESS,
                chainIdGateway
            );

            address[] memory coprocessorSigners = FhevmAddressesLib.resolveCoprocessorSignersFromEnv(vm);
            uint256 coprocessorThreshold = FhevmAddressesLib.resolveCoprocessorThresholdFromEnv(vm);
            FhevmAddressesLib.setInputVerifierImplementation(
                a.inputVerifier,
                address(new CleartextInputVerifier()),
                coprocessorSigners,
                coprocessorThreshold,
                INPUT_VERIFICATION_ADDRESS,
                chainIdGateway
            );
        }

        FhevmAddressesLib.setHCULimitImplementation(
            a.hcuLimit,
            address(new CleartextHCULimit()),
            type(uint48).max, // hcuCapPerBlock
            5_000_000, // maxHCUDepthPerTx
            20_000_000 // maxHCUPerTx
        );
        {
            address[] memory pausers = FhevmAddressesLib.resolvePausersFromEnv(vm);
            FhevmAddressesLib.addPausers(a.pauserSet, pausers);
        }

        require(vm.getNonce(deployer) == 19, "Expecting nonce=19");
        address fheTestAdd = vm.computeCreateAddress(deployer, vm.getNonce(deployer));
        FhevmAddressesLib.deployFHETestAt(fheTestAdd, a.acl, a.fhevmExecutor, a.kmsVerifier);

        FhevmAddressesLib.deployFhevmCheats(vm, a, fheTestAdd);

        vm.stopBroadcast();

        // Signer memory fheTestUser = FhevmAddressesLib.resolveFHETestUserFromEnv(vm);
        // if (fheTestUser.addr != address(0)) {
        //     vm.startBroadcast(fheTestUser.privateKey);
        //     FHETest(fheTestAdd).initFheTest(true);
        //     vm.stopBroadcast();
        // }
        // cast rpc anvil_setBalance 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 0x56bc75e2d63100000 --rpc-url http://127.0.0.1:8545

        console.log("\n--- Deployment complete ---");
        console.log("Deployer:         ", deployer);
        console.log("aclAdd:           ", aclAdd);
        console.log("fhevmExecutorAdd: ", fhevmExecutorAdd);
        console.log("kmsVerifierAdd:   ", kmsVerifierAdd);
        console.log("inputVerifierAdd: ", inputVerifierAdd);
        console.log("hcuLimitAdd:      ", hcuLimitAdd);
        console.log("pauserSetAdd:     ", pauserSetAdd);
        console.log("fheTest:          ", fheTestAdd);

        // Post-deploy verification: read the threshold back from the
        // KMSVerifier proxy and check it matches the value we configured.
        {
            uint256 expectedKmsThreshold = FhevmAddressesLib.resolveKmsThresholdFromEnv(vm);
            uint256 actualKmsThreshold = KMSVerifier(a.kmsVerifier).getThreshold();
            require(actualKmsThreshold == expectedKmsThreshold, "KMSVerifier threshold mismatch after deploy");
            console.log("KMSVerifier threshold verified: ", actualKmsThreshold);
        }

        // Sanity-log each deployed contract's `getVersion()` string. Every
        // call DELEGATECALLs into the current impl (cleartext or real) and
        // returns the version baked into its bytecode.
        console.log("ACL version:            ", ACL(a.acl).getVersion());
        console.log("FHEVMExecutor version:  ", FHEVMExecutor(a.fhevmExecutor).getVersion());
        console.log("KMSVerifier version:    ", KMSVerifier(a.kmsVerifier).getVersion());
        console.log("InputVerifier version:  ", InputVerifier(a.inputVerifier).getVersion());
        console.log("HCULimit version:       ", HCULimit(a.hcuLimit).getVersion());
        console.log("PauserSet version:      ", PauserSet(a.pauserSet).getVersion());
        console.log("FHETest version:        ", FHETest(fheTestAdd).CONTRACT_NAME());
    }

    function _resolveAndFundDeployers()
        private
        returns (uint256 deployerKey, uint256 emptyUupsDeployerKey, address deployer, address emptyUupsDeployer)
    {
        uint256 anvilPk = vm.deriveKey(ANVIL_MNEMONIC, "m/44'/60'/0'/0/", 0);

        // Keep the signer structs scoped to this helper so `run()` only keeps
        // the values it needs for the deployment flow.
        Signer memory d = FhevmAddressesLib.resolveDeployerFromEnv(vm);
        Signer memory eu = FhevmAddressesLib.resolveEmptyUupsDeployerFromEnv(vm);
        Signer memory fheTestUser = FhevmAddressesLib.resolveFHETestUserFromEnv(vm);

        deployerKey = d.privateKey;
        emptyUupsDeployerKey = eu.privateKey;

        deployer = d.addr;
        emptyUupsDeployer = eu.addr;

        require(
            vm.getNonce(deployer) == 0,
            string.concat(
                "\n",
                "=====================================================================\n",
                "  FHEVM host contracts appear to be ALREADY DEPLOYED on this chain.\n",
                "=====================================================================\n",
                "\n"
            )
        );

        vm.startBroadcast(anvilPk);

        (bool ok1,) = payable(deployer).call{value: 1000 ether}("");
        require(ok1, "DeployFHEVMHost: transfer to deployer failed");

        (bool ok2,) = payable(emptyUupsDeployer).call{value: 1000 ether}("");
        require(ok2, "DeployFHEVMHost: transfer to emptyUupsDeployer failed");

        if (fheTestUser.addr != address(0)) {
            (bool ok3,) = payable(fheTestUser.addr).call{value: 1000 ether}("");
            require(ok3, "DeployFHEVMHost: transfer to fheTestUser failed");
        }

        vm.stopBroadcast();
    }
}
