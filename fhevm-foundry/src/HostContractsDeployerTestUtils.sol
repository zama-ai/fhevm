// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "@fhevm-host-contracts/contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "@fhevm-host-contracts/contracts/KMSVerifier.sol";
import {InputVerifier} from "@fhevm-host-contracts/contracts/InputVerifier.sol";
import {HCULimit} from "@fhevm-host-contracts/contracts/HCULimit.sol";
import {PauserSet} from "@fhevm-host-contracts/contracts/immutable/PauserSet.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {EmptyUUPSProxyACL} from "@fhevm-host-contracts/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {aclAdd, fhevmExecutorAdd, hcuLimitAdd, inputVerifierAdd, kmsVerifierAdd, pauserSetAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

/**
 * @dev Thin wrapper so `deployCodeTo` can load locally compiled bytecode for the OZ proxy.
 * Foundry only exposes artifacts that live inside this repo, hence the re-exposed constructor.
 */
contract DeployableERC1967Proxy is ERC1967Proxy {
    constructor(address implementation, bytes memory data) ERC1967Proxy(implementation, data) {}
}

/**
 * @dev Test harness that reconstructs the on-chain host stack inside Foundry.
 *
 * Host contracts (ACL, FHEVMExecutor, KMS/Input verifiers, HCULimit, PauserSet) are deployed on mainnet
 * behind deterministic UUPS proxies anchored at addresses defined in `FHEVMHostAddresses.sol`. Rather than
 * mocking behaviours piecemeal, this helper redeploys each proxy + implementation pair exactly how production
 * does:
 *  - write the appropriate empty proxy runtime to the canonical address using `deployCodeTo`;
 *  - perform the privileged upgrade calls with the expected initializer payloads;
 *  - label the proxy and implementation for nicer traces.
 *
 * Tests that inherit this contract can call the `_deploy*` helpers to stitch together a realistic environment
 * where cross-contract permission checks (ACLOwnable, slot reads, etc.) behave the same as on-chain.
 */
abstract contract HostContractsDeployerTestUtils is Test {
    function _deployACL(address owner) internal returns (ACL aclProxy, address aclImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxyACL());

        deployCodeTo(
            "src/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxyACL.initialize, (owner))),
            aclAdd
        );
        vm.label(aclAdd, "ACL Proxy");

        aclImplementation = address(new ACL());
        vm.label(aclImplementation, "ACL Implementation");

        vm.prank(owner);
        EmptyUUPSProxyACL(aclAdd).upgradeToAndCall(aclImplementation, abi.encodeCall(ACL.initializeFromEmptyProxy, ()));

        aclProxy = ACL(aclAdd);
    }

    function _deployFHEVMExecutor(
        address owner
    ) internal returns (FHEVMExecutor fhevmExecutorProxy, address fhevmExecutorImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "src/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            fhevmExecutorAdd
        );
        vm.label(fhevmExecutorAdd, "FHEVMExecutor Proxy");

        fhevmExecutorImplementation = address(new FHEVMExecutor());
        vm.label(fhevmExecutorImplementation, "FHEVMExecutor Implementation");

        vm.prank(owner);
        EmptyUUPSProxy(fhevmExecutorAdd).upgradeToAndCall(
            fhevmExecutorImplementation,
            abi.encodeCall(FHEVMExecutor.initializeFromEmptyProxy, ())
        );

        fhevmExecutorProxy = FHEVMExecutor(fhevmExecutorAdd);
    }

    function _deployKMSVerifier(
        address owner,
        address verifyingContractSource,
        uint64 chainIDSource,
        address[] memory initialSigners,
        uint256 initialThreshold
    ) internal returns (KMSVerifier kmsVerifierProxy, address kmsVerifierImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "src/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            kmsVerifierAdd
        );
        vm.label(kmsVerifierAdd, "KMSVerifier Proxy");

        kmsVerifierImplementation = address(new KMSVerifier());
        vm.label(kmsVerifierImplementation, "KMSVerifier Implementation");

        vm.prank(owner);
        EmptyUUPSProxy(kmsVerifierAdd).upgradeToAndCall(
            kmsVerifierImplementation,
            abi.encodeCall(
                KMSVerifier.initializeFromEmptyProxy,
                (verifyingContractSource, chainIDSource, initialSigners, initialThreshold)
            )
        );

        kmsVerifierProxy = KMSVerifier(kmsVerifierAdd);
    }

    function _deployInputVerifier(
        address owner,
        address verifyingContractSource,
        uint64 chainIDSource,
        address[] memory initialSigners,
        uint256 initialThreshold
    ) internal returns (InputVerifier inputVerifierProxy, address inputVerifierImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "src/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            inputVerifierAdd
        );
        vm.label(inputVerifierAdd, "InputVerifier Proxy");

        inputVerifierImplementation = address(new InputVerifier());
        vm.label(inputVerifierImplementation, "InputVerifier Implementation");

        vm.prank(owner);
        EmptyUUPSProxy(inputVerifierAdd).upgradeToAndCall(
            inputVerifierImplementation,
            abi.encodeCall(
                InputVerifier.initializeFromEmptyProxy,
                (verifyingContractSource, chainIDSource, initialSigners, initialThreshold)
            )
        );

        inputVerifierProxy = InputVerifier(inputVerifierAdd);
    }

    function _deployHCULimit(address owner) internal returns (HCULimit hcuLimitProxy, address hcuLimitImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
                "src/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            hcuLimitAdd
        );
        vm.label(hcuLimitAdd, "HCULimit Proxy");

        hcuLimitImplementation = address(new HCULimit());
        vm.label(hcuLimitImplementation, "HCULimit Implementation");

        vm.prank(owner);
        EmptyUUPSProxy(hcuLimitAdd).upgradeToAndCall(
            hcuLimitImplementation,
            abi.encodeCall(HCULimit.initializeFromEmptyProxy, ())
        );

        hcuLimitProxy = HCULimit(hcuLimitAdd);
    }

    function _deployFullHostStack(
        address owner,
        address pauser,
        address kmsVerifyingSource,
        address inputVerifyingSource,
        uint64 chainIDSource,
        address[] memory kmsSigners,
        uint256 kmsThreshold,
        address[] memory inputSigners,
        uint256 inputThreshold
    ) internal {
        (ACL aclProxy, ) = _deployACL(owner);
        PauserSet pauserSet = _deployPauserSet();
        (FHEVMExecutor fheExecutor, ) = _deployFHEVMExecutor(owner);
        _deployHCULimit(owner);
        _deployKMSVerifier(owner, kmsVerifyingSource, chainIDSource, kmsSigners, kmsThreshold);
        _deployInputVerifier(owner, inputVerifyingSource, chainIDSource, inputSigners, inputThreshold);

        vm.prank(owner);
        pauserSet.addPauser(pauser);

        require(fheExecutor.getACLAddress() == aclAdd, "executor ACL wiring");
        require(fheExecutor.getHCULimitAddress() == hcuLimitAdd, "executor HCU wiring");
        require(aclProxy.getPauserSetAddress() == pauserSetAdd, "ACL PauserSet wiring");
        require(KMSVerifier(kmsVerifierAdd).getThreshold() == kmsThreshold, "KMS threshold wiring");
        require(InputVerifier(inputVerifierAdd).getThreshold() == inputThreshold, "Input threshold wiring");
    }

    function _deployPauserSet() internal returns (PauserSet pauserSet) {
        vm.etch(pauserSetAdd, address(new PauserSet()).code);
        vm.label(pauserSetAdd, "PauserSet");
        pauserSet = PauserSet(pauserSetAdd);
    }
}
