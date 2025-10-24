// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {ERC1967Utils} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";

import {HostContractsDeployerTestUtils} from "./HostContractsDeployerTestUtils.sol";
import {aclAdd, fhevmExecutorAdd, hcuLimitAdd, inputVerifierAdd, kmsVerifierAdd, pauserSetAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {ACL} from "../../contracts/ACL.sol";
import {FHEVMExecutor} from "../../contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "../../contracts/KMSVerifier.sol";
import {InputVerifier} from "../../contracts/InputVerifier.sol";
import {HCULimit} from "../../contracts/HCULimit.sol";
import {PauserSet} from "../../contracts/immutable/PauserSet.sol";
import {ERC1967Utils} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";

contract TestHostContractsDeployerTestUtils is HostContractsDeployerTestUtils {
    address private constant OWNER = address(0xBEEF);
    address private constant GATEWAY_SOURCE_CONTRACT = address(0x1234);
    uint64 private constant GATEWAY_CHAIN_ID = 31337;

    function test_DeployACL_DeploysProxyAndUpgradesImplementation() public {
        (ACL aclProxy, address aclImplementation) = _deployACL(OWNER);

        assertEq(address(aclProxy), aclAdd, "ACL proxy address mismatch");
        assertNotEq(aclImplementation, address(0), "Implementation not deployed");
        assertEq(aclProxy.owner(), OWNER, "Owner mismatch");
        assertEq(aclProxy.getVersion(), "ACL v0.2.0", "Version mismatch");
        assertEq(_readImplementationSlot(aclAdd), aclImplementation, "Implementation slot mismatch");
    }

    function test_DeployFHEVMExecutor_UsesProxyUpgradeFlow() public {
        // ACLOwnable gates upgrades by reading the owner at `aclAdd`, so seed ACL first.
        _deployACL(OWNER);
        (FHEVMExecutor fhevmExecutorProxy, address fhevmExecutorImplementation) = _deployFHEVMExecutor(OWNER);

        assertEq(address(fhevmExecutorProxy), fhevmExecutorAdd, "FHEVMExecutor proxy address mismatch");
        assertNotEq(fhevmExecutorImplementation, address(0), "Implementation not deployed");
        assertEq(fhevmExecutorProxy.getVersion(), "FHEVMExecutor v0.1.0", "Version mismatch");
        assertEq(
            _readImplementationSlot(fhevmExecutorAdd),
            fhevmExecutorImplementation,
            "Implementation slot mismatch"
        );
    }

    function test_DeployKMSVerifier_UsesProxyUpgradeFlow() public {
        // KMSVerifier inherits ACLOwnable as well, ensuring the ACL proxy is in place avoids upgrade reverts.
        _deployACL(OWNER);
        address[] memory initialSigners = new address[](2);
        initialSigners[0] = address(0x1111);
        initialSigners[1] = address(0x2222);
        uint256 initialThreshold = 1;

        (KMSVerifier kmsVerifierProxy, address kmsVerifierImplementation) = _deployKMSVerifier(
            OWNER,
            GATEWAY_SOURCE_CONTRACT,
            GATEWAY_CHAIN_ID,
            initialSigners,
            initialThreshold
        );

        assertEq(address(kmsVerifierProxy), kmsVerifierAdd, "KMSVerifier proxy address mismatch");
        assertNotEq(kmsVerifierImplementation, address(0), "Implementation not deployed");
        assertEq(kmsVerifierProxy.getVersion(), "KMSVerifier v0.1.0", "Version mismatch");
        assertEq(kmsVerifierProxy.getThreshold(), initialThreshold, "Threshold mismatch");
        address[] memory storedSigners = kmsVerifierProxy.getKmsSigners();
        assertEq(storedSigners.length, initialSigners.length, "Signers length mismatch");
        assertEq(storedSigners[0], initialSigners[0], "Signer[0] mismatch");
        assertEq(storedSigners[1], initialSigners[1], "Signer[1] mismatch");
        assertEq(
            _readImplementationSlot(kmsVerifierAdd),
            kmsVerifierImplementation,
            "Implementation slot mismatch"
        );
    }

    function test_DeployInputVerifier_UsesProxyUpgradeFlow() public {
        // InputVerifier is also ACLOwnable; seed ACL proxy to ensure upgrade authorization succeeds.
        _deployACL(OWNER);
        address[] memory initialSigners = new address[](3);
        initialSigners[0] = address(0xaaaa);
        initialSigners[1] = address(0xbbbb);
        initialSigners[2] = address(0xcccc);
        uint256 initialThreshold = 2;

        (InputVerifier inputVerifierProxy, address inputVerifierImplementation) = _deployInputVerifier(
            OWNER,
            GATEWAY_SOURCE_CONTRACT,
            GATEWAY_CHAIN_ID,
            initialSigners,
            initialThreshold
        );

        assertEq(address(inputVerifierProxy), inputVerifierAdd, "InputVerifier proxy address mismatch");
        assertNotEq(inputVerifierImplementation, address(0), "Implementation not deployed");
        assertEq(inputVerifierProxy.getVersion(), "InputVerifier v0.2.0", "Version mismatch");
        assertEq(inputVerifierProxy.getThreshold(), initialThreshold, "Threshold mismatch");
        address[] memory storedSigners = inputVerifierProxy.getCoprocessorSigners();
        assertEq(storedSigners.length, initialSigners.length, "Signers length mismatch");
        assertEq(storedSigners[0], initialSigners[0], "Signer[0] mismatch");
        assertEq(storedSigners[1], initialSigners[1], "Signer[1] mismatch");
        assertEq(storedSigners[2], initialSigners[2], "Signer[2] mismatch");
        assertEq(
            _readImplementationSlot(inputVerifierAdd),
            inputVerifierImplementation,
            "Implementation slot mismatch"
        );
    }

    function test_DeployHCULimit_UsesProxyUpgradeFlow() public {
        // HCULimit needs the ACL owner context for upgrade authorization.
        _deployACL(OWNER);
        (HCULimit hcuLimitProxy, address hcuLimitImplementation) = _deployHCULimit(OWNER);

        assertEq(address(hcuLimitProxy), hcuLimitAdd, "HCULimit proxy address mismatch");
        assertNotEq(hcuLimitImplementation, address(0), "Implementation not deployed");
        assertEq(hcuLimitProxy.getVersion(), "HCULimit v0.1.0", "Version mismatch");
        assertEq(_readImplementationSlot(hcuLimitAdd), hcuLimitImplementation, "Implementation slot mismatch");
    }

    function test_DeployPauserSet_UsesCanonicalAddress() public {
        // PauserSet relies on ACL ownership checks; ensure ACL proxy is in place.
        (ACL aclProxy,) = _deployACL(OWNER);
        PauserSet pauserSet = _deployPauserSet();

        assertEq(address(pauserSet), pauserSetAdd, "PauserSet address mismatch");
        assertEq(pauserSet.getVersion(), "PauserSet v0.1.0", "Version mismatch");

        address pauser = address(0xdead);
        vm.prank(aclProxy.owner());
        pauserSet.addPauser(pauser);
        assertTrue(pauserSet.isPauser(pauser), "Pauser not added");
    }

    function test_DeployFullHostStack_DeploysAndWiresAllContracts() public {
        address[] memory kmsSigners = new address[](1);
        kmsSigners[0] = address(0x7777);
        address[] memory inputSigners = new address[](1);
        inputSigners[0] = address(0x8888);
        address pauser = address(0xbeef);

        _deployFullHostStack(
            OWNER,
            pauser,
            GATEWAY_SOURCE_CONTRACT,
            GATEWAY_SOURCE_CONTRACT,
            GATEWAY_CHAIN_ID,
            kmsSigners,
            1,
            inputSigners,
            1
        );

        ACL aclProxy = ACL(aclAdd);
        FHEVMExecutor fheExecutor = FHEVMExecutor(fhevmExecutorAdd);

        assertEq(aclProxy.getPauserSetAddress(), pauserSetAdd, "ACL PauserSet wiring mismatch");
        assertEq(fheExecutor.getACLAddress(), aclAdd, "Executor ACL wiring mismatch");
        assertEq(fheExecutor.getHCULimitAddress(), hcuLimitAdd, "Executor HCULimit wiring mismatch");
        assertEq(KMSVerifier(kmsVerifierAdd).getThreshold(), 1, "KMSVerifier threshold mismatch");
        assertEq(InputVerifier(inputVerifierAdd).getThreshold(), 1, "InputVerifier threshold mismatch");
        assertTrue(PauserSet(pauserSetAdd).isPauser(pauser), "Pauser not registered");
        assertFalse(aclProxy.isAllowed(bytes32(uint256(1)), address(this)), "Unexpected ACL allow");

        vm.prank(pauser);
        aclProxy.pause();
        assertTrue(aclProxy.paused(), "ACL not paused by pauser");
        vm.prank(pauser);
        aclProxy.unpause();

        vm.expectRevert(abi.encodeWithSelector(ACL.SenderNotAllowed.selector, address(this)));
        aclProxy.allowTransient(bytes32(uint256(1)), address(this));

        vm.prank(fhevmExecutorAdd);
        aclProxy.allowTransient(bytes32(uint256(1)), address(this));
        assertTrue(aclProxy.allowedTransient(bytes32(uint256(1)), address(this)), "Transient allow failed");
    }

    function _readImplementationSlot(address proxy) private view returns (address) {
        return address(uint160(uint256(vm.load(proxy, ERC1967Utils.IMPLEMENTATION_SLOT))));
    }
}
