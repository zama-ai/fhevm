// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ACL} from "../../contracts/ACL.sol";
import {FHEVMExecutor} from "../../contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "../../contracts/KMSVerifier.sol";
import {InputVerifier} from "../../contracts/InputVerifier.sol";
import {HCULimit} from "../../contracts/HCULimit.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {EmptyUUPSProxyACL} from "../../contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {aclAdd, fhevmExecutorAdd, hcuLimitAdd, inputVerifierAdd, kmsVerifierAdd} from "../../addresses/FHEVMHostAddresses.sol";

/**
 * @dev Thin wrapper so `deployCodeTo` can load locally compiled bytecode for the OZ proxy.
 * Foundry only exposes artifacts that live inside this repo, hence the re-exposed constructor.
 */
contract DeployableERC1967Proxy is ERC1967Proxy {
    constructor(address implementation, bytes memory data) ERC1967Proxy(implementation, data) {}
}

/**
 * @dev Helper used in tests to mirror the production deployment flow for host contracts.
 * It deploys the empty ACL implementation behind an ERC-1967 proxy at the canonical address,
 * then upgrades that proxy to the full ACL logic.
 */
abstract contract HostContractsDeployer is Test {
    function _deployACL(address owner) internal returns (ACL aclProxy, address aclImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxyACL());

        deployCodeTo(
            "test/utils/HostContractsDeployer.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxyACL.initialize, (owner))),
            aclAdd
        );
        vm.label(aclAdd, "ACL Proxy");

        aclImplementation = address(new ACL());
        vm.label(aclImplementation, "ACL Implementation");

        vm.prank(owner);
        EmptyUUPSProxyACL(aclAdd).upgradeToAndCall(
            aclImplementation,
            abi.encodeCall(ACL.initializeFromEmptyProxy, ())
        );

        aclProxy = ACL(aclAdd);
    }

    function _deployFHEVMExecutor(address owner)
        internal
        returns (FHEVMExecutor fhevmExecutorProxy, address fhevmExecutorImplementation)
    {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "test/utils/HostContractsDeployer.sol:DeployableERC1967Proxy",
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
            "test/utils/HostContractsDeployer.sol:DeployableERC1967Proxy",
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
            "test/utils/HostContractsDeployer.sol:DeployableERC1967Proxy",
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
            "test/utils/HostContractsDeployer.sol:DeployableERC1967Proxy",
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
}
