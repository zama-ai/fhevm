// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {FhevmDeploy} from "../pkg/forge/src/FhevmDeploy.sol";
import {
    ACL_ADDRESS,
    CLEARTEXT_ARITHMETIC_ADDRESS,
    CLEARTEXT_DB_ADDRESS,
    DEPLOYER_ADDRESS,
    DEPLOYER_START_NONCE,
    FHEVM_EXECUTOR_ADDRESS,
    HCU_LIMIT_ADDRESS,
    INPUT_VERIFIER_ADDRESS,
    KMS_VERIFIER_ADDRESS,
    PAUSER_SET_ADDRESS
} from "../pkg/forge/src/FhevmDeploy.sol";
import {IACL} from "../pkg/forge/src/FhevmDeploy.sol";
import {ICleartextArithmetic} from "../pkg/forge/src/FhevmDeploy.sol";
import {ICleartextDB} from "../pkg/forge/src/FhevmDeploy.sol";
import {ICleartextFHEVMExecutor} from "../pkg/forge/src/FhevmDeploy.sol";
import {ICleartextInputVerifier} from "../pkg/forge/src/FhevmDeploy.sol";
import {ICleartextKMSVerifier} from "../pkg/forge/src/FhevmDeploy.sol";
import {IHCULimit} from "../pkg/forge/src/FhevmDeploy.sol";
import {IPauserSet} from "../pkg/forge/src/FhevmDeploy.sol";
import {LocalHostBootstrap} from "../pkg/forge/src/_internal/LocalHostBootstrap.sol";
import {LocalHostVersions} from "../pkg/forge/src/_internal/LocalHostVersions.sol";

/**
 * The Foundry half of the suite: `FhevmDeploy` is the one artifact a TS test cannot exercise, because it
 * only runs inside forge. What it proves that `test/templates.test.ts` cannot is that the generated
 * `pkg/forge/` files actually stand up a working stack — the TS tests check those files are internally
 * consistent, not that they function.
 */
contract FhevmDeployTest is Test, FhevmDeploy {
    function setUp() public {
        deployFhevm();
    }

    /// The address set is the product; a stack anywhere else is useless to a ZamaConfig consumer.
    function test_everyContractIsAtItsCanonicalAddress() public view {
        address[9] memory expected = [
            ACL_ADDRESS,
            FHEVM_EXECUTOR_ADDRESS,
            KMS_VERIFIER_ADDRESS,
            INPUT_VERIFIER_ADDRESS,
            HCU_LIMIT_ADDRESS,
            CLEARTEXT_ARITHMETIC_ADDRESS,
            CLEARTEXT_DB_ADDRESS,
            PAUSER_SET_ADDRESS,
            fhevmACLOwner()
        ];
        for (uint256 i = 0; i < expected.length; i++) {
            assertGt(expected[i].code.length, 0, "no code at a canonical address");
        }
    }

    /**
     * `getVersion()` is the only external evidence that the *real* implementation sits behind each proxy
     * rather than the empty one it was created over — phase 5 either ran or it did not.
     */
    function test_proxiesResolveToTheRealImplementations() public view {
        assertEq(IACL(ACL_ADDRESS).getVersion(), LocalHostVersions.ACL);
        assertEq(ICleartextFHEVMExecutor(FHEVM_EXECUTOR_ADDRESS).getVersion(), LocalHostVersions.FHEVM_EXECUTOR);
        assertEq(ICleartextKMSVerifier(KMS_VERIFIER_ADDRESS).getVersion(), LocalHostVersions.KMS_VERIFIER);
        assertEq(ICleartextInputVerifier(INPUT_VERIFIER_ADDRESS).getVersion(), LocalHostVersions.INPUT_VERIFIER);
        assertEq(IHCULimit(HCU_LIMIT_ADDRESS).getVersion(), LocalHostVersions.HCU_LIMIT);
        assertEq(ICleartextArithmetic(CLEARTEXT_ARITHMETIC_ADDRESS).getVersion(), LocalHostVersions.CLEARTEXT_ARITHMETIC);
        assertEq(IPauserSet(PAUSER_SET_ADDRESS).getVersion(), LocalHostVersions.PAUSER_SET);
    }

    /**
     * The addresses compiled into the bytecode must match the addresses the stack was deployed at. They
     * are baked in, so a mismatch is unfixable at runtime and invisible without reading them back.
     */
    function test_bakedInWiringMatchesTheDeployedStack() public view {
        assertEq(IACL(ACL_ADDRESS).getFHEVMExecutorAddress(), FHEVM_EXECUTOR_ADDRESS);
        assertEq(IACL(ACL_ADDRESS).getPauserSetAddress(), PAUSER_SET_ADDRESS);
        assertEq(ICleartextFHEVMExecutor(FHEVM_EXECUTOR_ADDRESS).getACLAddress(), ACL_ADDRESS);
        assertEq(ICleartextFHEVMExecutor(FHEVM_EXECUTOR_ADDRESS).getHCULimitAddress(), HCU_LIMIT_ADDRESS);
        assertEq(ICleartextFHEVMExecutor(FHEVM_EXECUTOR_ADDRESS).getInputVerifierAddress(), INPUT_VERIFIER_ADDRESS);
        assertEq(IHCULimit(HCU_LIMIT_ADDRESS).getFHEVMExecutorAddress(), FHEVM_EXECUTOR_ADDRESS);
    }

    /// Phase 3: ACLOwner holds ACL ownership and is a registered pauser, in that order.
    function test_aclOwnerHoldsOwnershipAndCanPause() public {
        assertEq(IACL(ACL_ADDRESS).owner(), fhevmACLOwner(), "ACLOwner must own ACL");
        assertTrue(IPauserSet(PAUSER_SET_ADDRESS).isPauser(fhevmACLOwner()), "ACLOwner must be a pauser");

        assertFalse(IACL(ACL_ADDRESS).paused());
        vm.prank(DEPLOYER_ADDRESS);
        IACLOwnerPause(fhevmACLOwner()).pause();
        assertTrue(IACL(ACL_ADDRESS).paused(), "pausing through ACLOwner must reach ACL");
    }

    /// Phase 5 pointed CleartextDB at the arithmetic contract.
    function test_initializersRanWithTheirArguments() public view {
        assertTrue(ICleartextDB(CLEARTEXT_DB_ADDRESS).isWriter(CLEARTEXT_ARITHMETIC_ADDRESS), "DB writer");
        assertFalse(ICleartextDB(CLEARTEXT_DB_ADDRESS).isWriter(FHEVM_EXECUTOR_ADDRESS), "executor is not a writer");
    }

    /**
     * The signer sets are the contract between this deploy and the js-sdk cleartext relayer: the relayer
     * derives its keys from FHEVM_MNEMONIC at fixed HD paths and looks them up by the address the chain
     * reports. Register anything else and it holds no key for the signer it is asked to be, so decrypt
     * fails at signing time rather than at deploy time.
     */
    function test_registeredSignersAreTheOnesTheSdkCanSignFor() public view {
        address[] memory expectedCoprocessors = LocalHostBootstrap.coprocessorSigners();
        address[] memory onChainCoprocessors = ICleartextInputVerifier(INPUT_VERIFIER_ADDRESS).getCoprocessorSigners();
        assertEq(onChainCoprocessors.length, expectedCoprocessors.length, "coprocessor signer count");
        for (uint256 i = 0; i < expectedCoprocessors.length; i++) {
            assertEq(onChainCoprocessors[i], expectedCoprocessors[i], "coprocessor signer");
        }
        assertEq(
            ICleartextInputVerifier(INPUT_VERIFIER_ADDRESS).getThreshold(),
            LocalHostBootstrap.COPROCESSOR_THRESHOLD,
            "coprocessor threshold"
        );

        address[] memory expectedKms = LocalHostBootstrap.kmsSigners();
        address[] memory onChainKms = ICleartextKMSVerifier(KMS_VERIFIER_ADDRESS).getKmsSigners();
        assertEq(onChainKms.length, expectedKms.length, "kms signer count");
        for (uint256 i = 0; i < expectedKms.length; i++) {
            assertEq(onChainKms[i], expectedKms[i], "kms signer");
        }
    }

    /**
     * The EIP-712 domain each proof is signed against. Bound at initialization and unreadable from the
     * initializer arguments afterwards, so `eip712Domain()` is the only way to confirm the deploy used the
     * values the SDK builds its typed data from — a mismatch makes every signature verify against the
     * wrong domain and fail for reasons that look nothing like a config error.
     */
    function test_eip712DomainsMatchTheBootstrapSources() public view {
        (,,, uint256 kmsChainId, address kmsVerifyingContract,,) =
            ICleartextKMSVerifier(KMS_VERIFIER_ADDRESS).eip712Domain();
        assertEq(kmsChainId, LocalHostBootstrap.GATEWAY_CHAIN_ID, "kms verifier chain id");
        assertEq(kmsVerifyingContract, LocalHostBootstrap.DECRYPTION_ADDRESS, "kms verifier verifyingContract");

        (,,, uint256 inputChainId, address inputVerifyingContract,,) =
            ICleartextInputVerifier(INPUT_VERIFIER_ADDRESS).eip712Domain();
        assertEq(inputChainId, LocalHostBootstrap.GATEWAY_CHAIN_ID, "input verifier chain id");
        assertEq(
            inputVerifyingContract, LocalHostBootstrap.INPUT_VERIFICATION_ADDRESS, "input verifier verifyingContract"
        );
    }

    /// No KMS *context* test in this generation: the KMSVerifier stores only the signer set and one
    /// threshold, so there is no on-chain node metadata (tx-sender/ip/storage) to check. 0.13 introduces
    /// ProtocolConfig and records all of it, which is why the v12->v13 migration has to be told the node
    /// details rather than reading them off the running stack.


    /// One KMS threshold, on the verifier itself. 0.13 splits it into four on ProtocolConfig.
    function test_kmsThresholdMatchesTheNodeCount() public view {
        assertEq(
            ICleartextKMSVerifier(KMS_VERIFIER_ADDRESS).getThreshold(),
            LocalHostBootstrap.KMS_NODE_COUNT,
            "kms threshold"
        );
    }

    /// The HCU limits. There is no override path: the bootstrap values are the only ones.
    function test_defaultHcuLimitsAreTheBootstrapValues() public view {
        IHCULimit limit = IHCULimit(HCU_LIMIT_ADDRESS);
        assertEq(limit.getGlobalHCUCapPerBlock(), LocalHostBootstrap.HCU_CAP_PER_BLOCK, "cap per block");
        assertEq(limit.getMaxHCUDepthPerTx(), LocalHostBootstrap.MAX_HCU_DEPTH_PER_TX, "max depth per tx");
        assertEq(limit.getMaxHCUPerTx(), LocalHostBootstrap.MAX_HCU_PER_TX, "max HCU per tx");
    }

    /// Callable from several `setUp()` bodies without redeploying.
    function test_deployIsIdempotent() public {
        address ownerBefore = fhevmACLOwner();
        deployFhevm();
        assertEq(fhevmACLOwner(), ownerBefore, "a second call must not redeploy");
    }
}

/**
 * There is deliberately no "configured" variant. Every bootstrap argument is `private`, so this contract
 * produces exactly one stack — see the note in FhevmDeploy about why a configurable one would be a
 * liability rather than a feature.
 */

/// The determinism guard: every address derives from the deployer's nonce, so a dirty one must abort.
contract FhevmDeployGuardTest is Test, FhevmDeploy {
    function test_refusesToDeployFromADirtyNonce() public {
        vm.setNonce(DEPLOYER_ADDRESS, uint64(DEPLOYER_START_NONCE + 3));
        vm.expectRevert(
            bytes("FhevmDeploy: deployer nonce must be DEPLOYER_START_NONCE; every address derives from it")
        );
        this.callDeployFhevm();
    }

    /// External so `vm.expectRevert` sees a call boundary.
    function callDeployFhevm() external {
        deployFhevm();
    }
}

/// `ACLOwner.pause()` is not on the generated interface (it is inherited); declare the one selector used.
interface IACLOwnerPause {
    function pause() external;
}
