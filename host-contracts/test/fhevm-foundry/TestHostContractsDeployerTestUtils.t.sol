// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ERC1967Utils} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";

import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {aclAdd, fhevmExecutorAdd, hcuLimitAdd, inputVerifierAdd, kmsVerifierAdd, pauserSetAdd, protocolConfigAdd, kmsGenerationAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "@fhevm-host-contracts/contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "@fhevm-host-contracts/contracts/KMSVerifier.sol";
import {InputVerifier} from "@fhevm-host-contracts/contracts/InputVerifier.sol";
import {HCULimit} from "@fhevm-host-contracts/contracts/HCULimit.sol";
import {PauserSet} from "@fhevm-host-contracts/contracts/immutable/PauserSet.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {KMSGeneration} from "@fhevm-host-contracts/contracts/KMSGeneration.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsNode, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {Vm} from "forge-std/Test.sol";
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
        assertEq(aclProxy.getVersion(), "ACL v0.5.0", "Version mismatch");
        assertEq(_readImplementationSlot(aclAdd), aclImplementation, "Implementation slot mismatch");
    }

    function test_DeployFHEVMExecutor_UsesProxyUpgradeFlow() public {
        // ACLOwnable gates upgrades by reading the owner at `aclAdd`, so seed ACL first.
        _deployACL(OWNER);
        (FHEVMExecutor fhevmExecutorProxy, address fhevmExecutorImplementation) = _deployFHEVMExecutor(OWNER);

        assertEq(address(fhevmExecutorProxy), fhevmExecutorAdd, "FHEVMExecutor proxy address mismatch");
        assertNotEq(fhevmExecutorImplementation, address(0), "Implementation not deployed");
        assertEq(fhevmExecutorProxy.getVersion(), "FHEVMExecutor v0.5.0", "Version mismatch");
        assertEq(
            _readImplementationSlot(fhevmExecutorAdd),
            fhevmExecutorImplementation,
            "Implementation slot mismatch"
        );
    }

    function test_DeployKMSVerifier_UsesProxyUpgradeFlow() public {
        // KMSVerifier inherits ACLOwnable as well, ensuring the ACL proxy is in place avoids upgrade reverts.
        _deployACL(OWNER);
        KmsNodeParams[] memory initialKmsNodeParams = _makeKmsNodeParams(2);
        _deployProtocolConfig(OWNER, initialKmsNodeParams, _defaultThresholds());

        (KMSVerifier kmsVerifierProxy, address kmsVerifierImplementation) = _deployKMSVerifier(
            OWNER,
            GATEWAY_SOURCE_CONTRACT,
            GATEWAY_CHAIN_ID
        );

        assertEq(address(kmsVerifierProxy), kmsVerifierAdd, "KMSVerifier proxy address mismatch");
        assertNotEq(kmsVerifierImplementation, address(0), "Implementation not deployed");
        assertEq(kmsVerifierProxy.getVersion(), "KMSVerifier v0.4.0", "Version mismatch");
        assertEq(kmsVerifierProxy.getThreshold(), _defaultThresholds().publicDecryption, "Threshold mismatch");
        address[] memory storedSigners = kmsVerifierProxy.getKmsSigners();
        assertEq(storedSigners.length, initialKmsNodeParams.length, "Signers length mismatch");
        assertEq(storedSigners[0], initialKmsNodeParams[0].signerAddress, "Signer[0] mismatch");
        assertEq(storedSigners[1], initialKmsNodeParams[1].signerAddress, "Signer[1] mismatch");
        assertEq(_readImplementationSlot(kmsVerifierAdd), kmsVerifierImplementation, "Implementation slot mismatch");
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
        assertEq(hcuLimitProxy.getVersion(), "HCULimit v0.4.0", "Version mismatch");
        assertEq(_readImplementationSlot(hcuLimitAdd), hcuLimitImplementation, "Implementation slot mismatch");
    }

    function test_DeployPauserSet_UsesCanonicalAddress() public {
        // PauserSet relies on ACL ownership checks; ensure ACL proxy is in place.
        (ACL aclProxy, ) = _deployACL(OWNER);
        PauserSet pauserSet = _deployPauserSet();

        assertEq(address(pauserSet), pauserSetAdd, "PauserSet address mismatch");
        assertEq(pauserSet.getVersion(), "PauserSet v0.1.0", "Version mismatch");

        address pauser = address(0xdead);
        vm.prank(aclProxy.owner());
        pauserSet.addPauser(pauser);
        assertTrue(pauserSet.isPauser(pauser), "Pauser not added");
    }

    function test_DeployFullHostStack_DeploysAndWiresAllContracts() public {
        KmsNodeParams[] memory initialKmsNodeParams = _makeKmsNodeParams(1);
        address[] memory inputSigners = new address[](1);
        inputSigners[0] = address(0x8888);
        address pauser = address(0xbeef);

        _deployFullHostStack(
            OWNER,
            pauser,
            GATEWAY_SOURCE_CONTRACT,
            GATEWAY_SOURCE_CONTRACT,
            GATEWAY_CHAIN_ID,
            initialKmsNodeParams,
            _defaultThresholds(),
            inputSigners,
            1
        );

        ACL aclProxy = ACL(aclAdd);
        FHEVMExecutor fheExecutor = FHEVMExecutor(fhevmExecutorAdd);

        assertEq(aclProxy.getPauserSetAddress(), pauserSetAdd, "ACL PauserSet wiring mismatch");
        assertEq(fheExecutor.getACLAddress(), aclAdd, "Executor ACL wiring mismatch");
        assertEq(fheExecutor.getHCULimitAddress(), hcuLimitAdd, "Executor HCULimit wiring mismatch");
        assertEq(
            KMSVerifier(kmsVerifierAdd).getThreshold(),
            _defaultThresholds().publicDecryption,
            "KMSVerifier threshold mismatch"
        );
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

    function test_DeployProtocolConfig_UsesProxyUpgradeFlow() public {
        _deployACL(OWNER);

        KmsNodeParams[] memory nodeParams = _makeKmsNodeParams(2);

        (ProtocolConfig pcProxy, address pcImplementation) = _deployProtocolConfig(
            OWNER,
            nodeParams,
            _defaultThresholds()
        );

        assertEq(address(pcProxy), protocolConfigAdd, "ProtocolConfig proxy address mismatch");
        assertNotEq(pcImplementation, address(0), "Implementation not deployed");
        assertEq(pcProxy.getVersion(), "ProtocolConfig v0.2.0", "Version mismatch");
        assertEq(pcProxy.getPublicDecryptionThreshold(), 1, "Public decryption threshold mismatch");
        assertEq(pcProxy.getUserDecryptionThreshold(), 1, "User decryption threshold mismatch");
        assertEq(pcProxy.getKmsGenThreshold(), 1, "KmsGen threshold mismatch");
        assertEq(pcProxy.getMpcThreshold(), 1, "Mpc threshold mismatch");
        assertEq(_readImplementationSlot(protocolConfigAdd), pcImplementation, "Implementation slot mismatch");
    }

    function test_DeployProtocolConfigMirror_UsesProxyUpgradeFlow() public {
        _deployACL(OWNER);

        KmsNodeParams[] memory nodeParams = _makeKmsNodeParams(2);
        uint256 canonicalContextId = KMS_CONTEXT_COUNTER_BASE + 7;
        uint256 canonicalEpochId = EPOCH_COUNTER_BASE + 7;
        IProtocolConfig.KmsThresholds memory thresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 2,
            kmsGen: 2,
            mpc: 1
        });

        (ProtocolConfig pcProxy, address pcImplementation) = _deployProtocolConfigMirror(
            OWNER,
            canonicalContextId,
            canonicalEpochId,
            nodeParams,
            thresholds
        );

        assertEq(address(pcProxy), protocolConfigAdd, "ProtocolConfig proxy address mismatch");
        assertNotEq(pcImplementation, address(0), "Implementation not deployed");
        assertEq(pcProxy.getVersion(), "ProtocolConfig v0.2.0", "Version mismatch");
        assertEq(pcProxy.getCurrentKmsContextId(), canonicalContextId, "Context ID mismatch");
        assertEq(pcProxy.getUserDecryptionThreshold(), 2, "User decryption threshold mismatch");
        (uint256 activeContextId, uint256 activeEpochId) = pcProxy.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, canonicalContextId, "Active context mismatch");
        assertEq(activeEpochId, canonicalEpochId, "Active epoch mismatch");
        assertEq(_readImplementationSlot(protocolConfigAdd), pcImplementation, "Implementation slot mismatch");
    }

    /// @dev the four MPC-metadata fields added to KmsNodeParams (partyId/mpcIdentity/caCert/
    ///      storagePrefix) are not stored on KmsNode and only survive through the MirrorKmsContextAndEpoch
    ///      event. The four stored fields survive into KmsNode storage. A transposition of any field
    ///      would fail this round-trip assertion.
    function test_MirrorKmsContextAndEpoch_PreservesAllNodeParamFields() public {
        _deployACL(OWNER);
        KmsNodeParams[] memory bootstrap = _makeKmsNodeParams(1);
        _deployProtocolConfig(OWNER, bootstrap, _defaultThresholds());
        ProtocolConfig pcProxy = ProtocolConfig(protocolConfigAdd);

        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();
        uint256 mirroredContextId = KMS_CONTEXT_COUNTER_BASE + 4;
        uint256 mirroredEpochId = EPOCH_COUNTER_BASE + 4;

        vm.recordLogs();
        vm.prank(OWNER);
        pcProxy.mirrorKmsContextAndEpoch(
            mirroredContextId,
            mirroredEpochId,
            nodes,
            thresholds,
            "kms-v3",
            new PcrValues[](0)
        );
        Vm.Log[] memory logs = vm.getRecordedLogs();

        // Decode MirrorKmsContextAndEpoch to recover the full (8-field) KmsNodeParams array.
        KmsNodeParams[] memory emitted;
        bool found;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == IProtocolConfig.MirrorKmsContextAndEpoch.selector) {
                (emitted, , , ) = abi.decode(
                    logs[i].data,
                    (KmsNodeParams[], IProtocolConfig.KmsThresholds, string, PcrValues[])
                );
                assertEq(uint256(logs[i].topics[2]), mirroredEpochId, "epoch id topic mismatch");
                found = true;
                break;
            }
        }
        assertTrue(found, "MirrorKmsContextAndEpoch not emitted");
        assertEq(emitted.length, nodes.length, "node count mismatch");
        for (uint256 i = 0; i < nodes.length; i++) {
            assertEq(emitted[i].txSenderAddress, nodes[i].txSenderAddress, "txSender mismatch");
            assertEq(emitted[i].signerAddress, nodes[i].signerAddress, "signer mismatch");
            assertEq(emitted[i].ipAddress, nodes[i].ipAddress, "ipAddress mismatch");
            assertEq(emitted[i].storageUrl, nodes[i].storageUrl, "storageUrl mismatch");
            assertEq(emitted[i].partyId, nodes[i].partyId, "partyId mismatch");
            assertEq(emitted[i].mpcIdentity, nodes[i].mpcIdentity, "mpcIdentity mismatch");
            assertEq(emitted[i].caCert, nodes[i].caCert, "caCert mismatch");
            assertEq(emitted[i].storagePrefix, nodes[i].storagePrefix, "storagePrefix mismatch");
        }

        // The four stored fields also survive into KmsNode storage.
        KmsNode[] memory stored = pcProxy.getKmsNodesForContext(mirroredContextId);
        assertEq(stored.length, nodes.length, "stored node count mismatch");
        for (uint256 i = 0; i < nodes.length; i++) {
            assertEq(stored[i].txSenderAddress, nodes[i].txSenderAddress, "stored txSender mismatch");
            assertEq(stored[i].signerAddress, nodes[i].signerAddress, "stored signer mismatch");
            assertEq(stored[i].ipAddress, nodes[i].ipAddress, "stored ipAddress mismatch");
            assertEq(stored[i].storageUrl, nodes[i].storageUrl, "stored storageUrl mismatch");
        }
    }

    function test_DeployKMSGeneration_UsesProxyUpgradeFlow() public {
        _deployACL(OWNER);

        // KMSGeneration reads from ProtocolConfig so we need it deployed
        KmsNodeParams[] memory nodeParams = _makeKmsNodeParams(1);
        _deployProtocolConfig(OWNER, nodeParams, _defaultThresholds());

        (KMSGeneration kgProxy, address kgImplementation) = _deployKMSGeneration(OWNER);

        assertEq(address(kgProxy), kmsGenerationAdd, "KMSGeneration proxy address mismatch");
        assertNotEq(kgImplementation, address(0), "Implementation not deployed");
        assertEq(kgProxy.getVersion(), "KMSGeneration v0.2.0", "Version mismatch");
        assertEq(_readImplementationSlot(kmsGenerationAdd), kgImplementation, "Implementation slot mismatch");
    }

    function _readImplementationSlot(address proxy) private view returns (address) {
        return address(uint160(uint256(vm.load(proxy, ERC1967Utils.IMPLEMENTATION_SLOT))));
    }
}
