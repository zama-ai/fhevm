// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {ERC1967Utils} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";

import {HostContractsDeployer} from "./HostContractsDeployer.sol";
import {aclAdd, fhevmExecutorAdd, kmsVerifierAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {ACL} from "../../contracts/ACL.sol";
import {FHEVMExecutor} from "../../contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "../../contracts/KMSVerifier.sol";
import {ERC1967Utils} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";

contract HostContractsDeployerTest is HostContractsDeployer {
    address private constant OWNER = address(0xBEEF);

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
        address verifyingContractSource = address(0x1234);
        uint64 chainIDSource = 31337;
        address[] memory initialSigners = new address[](2);
        initialSigners[0] = address(0x1111);
        initialSigners[1] = address(0x2222);
        uint256 initialThreshold = 1;

        (KMSVerifier kmsVerifierProxy, address kmsVerifierImplementation) = _deployKMSVerifier(
            OWNER,
            verifyingContractSource,
            chainIDSource,
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

    /**
     * @dev Mirrors how clients read the current implementation in production by peeking at the ERC-1967 slot.
     * Using the library constant avoids hard-coding the slot value here.
     */
    function _readImplementationSlot(address proxy) private view returns (address) {
        return address(uint160(uint256(vm.load(proxy, ERC1967Utils.IMPLEMENTATION_SLOT))));
    }
}
