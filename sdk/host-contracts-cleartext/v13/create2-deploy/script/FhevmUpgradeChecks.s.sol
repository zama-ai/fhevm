// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {console} from "forge-std/Script.sol";
import {FhevmVerifyBase} from "./FhevmVerifyBase.s.sol";
import {FhevmUpgradeBase} from "./FhevmUpgradeBase.s.sol";
import {UpgradeInitData} from "./UpgradeInitData.sol";
import {IProtocolConfig} from "../../pkg/src/contracts/interfaces/IProtocolConfig.sol";
import {KmsNode} from "../../pkg/src/contracts/shared/Structs.sol";
import {
    IVersioned,
    IOwnable2Step,
    IPauserSet,
    IACLOwner,
    IWiredACL,
    IWiredFHEVMExecutor,
    IWiredHCULimit,
    IWiredCleartextArithmetic
} from "./Interfaces.sol";

/**
 * @title  FhevmUpgradeChecks
 * @notice The upgrade's non-reverting checks, shared by the gate BEFORE materialize and the verify AFTER it.
 *
 * Both scripts ask most of the same questions of the chain — is the sealed build the one deployed, are the
 * implementations the contracts they claim to be, is authority where it was — and only differ in which
 * side of the atomic call they stand on. One home for the questions keeps the two answers comparable.
 *
 * Every check below reads the CHAIN and the BUILD, and compares them to the SEAL. Nothing here re-derives
 * an expectation from the thing under test.
 */
abstract contract FhevmUpgradeChecks is FhevmVerifyBase, FhevmUpgradeBase {
    // ---------------------------------------------------------------------------------------
    // The build against the seal, and the chain against the build
    // ---------------------------------------------------------------------------------------

    /**
     * @dev Every sealed create re-derives from THIS build and holds code.
     *
     *      Run against a fresh `--out`, this is the independent recompile: if the current checkout does
     *      not reproduce the sealed init-code hashes, the seal came from a build that no longer exists.
     */
    function _checkSealedBuild(string memory manifest) internal {
        UpgradeCreate[] memory t = _upgradeCreateTable(manifest);
        for (uint256 i; i < t.length; i++) {
            address sealedAddress = _readManifestAddress(manifest, t[i].role);
            _expectAddr(
                _predictCreate2Address(t[i].role, t[i].initCode),
                sealedAddress,
                string.concat(t[i].role, " re-derives from this build")
            );
            _expect(_deployed(sealedAddress), string.concat("code at ", t[i].role));
        }
    }

    /**
     * @dev The runtime code at every sealed create is the artifact's deployed bytecode.
     *
     *      Presence and a matching address prove the INITCODE; this proves what the initcode LEFT. The two
     *      agree by construction for honest bytecode, but this one is what an auditor with a node and the
     *      source can re-run without trusting the seal at all.
     */
    function _checkDeployedCode(string memory manifest) internal {
        UpgradeCreate[] memory t = _upgradeCreateTable(manifest);
        for (uint256 i; i < t.length; i++) {
            address sealedAddress = _readManifestAddress(manifest, t[i].role);
            _expect(
                _matchesDeployedCode(sealedAddress, t[i].artifact),
                string.concat(t[i].role, " runtime code == ", t[i].artifact)
            );
        }
    }

    // ---------------------------------------------------------------------------------------
    // The implementations, asked directly
    // ---------------------------------------------------------------------------------------

    /**
     * @dev Each sealed implementation identifies itself as the contract its role expects.
     *
     *      `getVersion()` is pure and the wiring getters return compile-time constants, so both answer on
     *      the implementation address without a proxy in front. Two tables are index-aligned by hand —
     *      roles and artifacts — and a swap between two contracts that share a reinitializer selector
     *      would survive `ACLOwner.upgrade`'s simulation. This is the check that does not depend on the
     *      tables: it asks the code.
     */
    function _checkImplementationIdentity(string memory manifest) internal {
        string[] memory roles = _upgradeProxyRoles();
        for (uint256 i; i < roles.length; i++) {
            address impl = _readManifestAddress(manifest, _implRole(roles[i]));
            _expectStr(
                IVersioned(impl).getVersion(),
                _versionFor(roles[i]),
                string.concat(_implRole(roles[i]), ".getVersion()")
            );
        }
    }

    /// @dev The addresses BAKED into each implementation are the live stack's, read off the implementations.
    function _checkImplementationWiring(string memory manifest) internal {
        address acl = _readManifestAddress(manifest, _implRole(R_ACL));
        address executor = _readManifestAddress(manifest, _implRole(R_FHEVM_EXECUTOR));
        address hcu = _readManifestAddress(manifest, _implRole(R_HCU_LIMIT));
        address arithmetic = _readManifestAddress(manifest, _implRole(R_CLEARTEXT_ARITHMETIC));

        _expectAddr(IWiredACL(acl).getFHEVMExecutorAddress(), existingExecutor, "IMPL_ACL bakes FHEVMExecutor");
        _expectAddr(IWiredACL(acl).getPauserSetAddress(), existingPauserSet, "IMPL_ACL bakes PauserSet");
        _expectAddr(IWiredFHEVMExecutor(executor).getACLAddress(), existingAcl, "IMPL_FHEVM_EXECUTOR bakes ACL");
        _expectAddr(
            IWiredFHEVMExecutor(executor).getHCULimitAddress(), existingHcuLimit, "IMPL_FHEVM_EXECUTOR bakes HCULimit"
        );
        _expectAddr(
            IWiredFHEVMExecutor(executor).getInputVerifierAddress(),
            existingInputVerifier,
            "IMPL_FHEVM_EXECUTOR bakes InputVerifier"
        );
        _expectAddr(
            IWiredFHEVMExecutor(executor).getCleartextArithmeticAddress(),
            existingArithmetic,
            "IMPL_FHEVM_EXECUTOR bakes CleartextArithmetic"
        );
        _expectAddr(
            IWiredHCULimit(hcu).getFHEVMExecutorAddress(), existingExecutor, "IMPL_HCU_LIMIT bakes FHEVMExecutor"
        );
        _expectAddr(
            IWiredCleartextArithmetic(arithmetic).getCleartextDBAddress(),
            existingDb,
            "IMPL_CLEARTEXT_ARITHMETIC bakes CleartextDB"
        );
    }

    // ---------------------------------------------------------------------------------------
    // Slots
    // ---------------------------------------------------------------------------------------

    /// @dev The two proxies the op list must not touch still run the implementation they ran at seal time.
    function _checkUntouchedProxies(string memory manifest) internal {
        string[] memory roles = _untouchedProxyRoles();
        for (uint256 i; i < roles.length; i++) {
            _expectAddr(
                _implementationOf(_readManifestAddress(manifest, roles[i])),
                _sealedPreviousImplementation(manifest, roles[i]),
                string.concat(roles[i], " still points at its pre-upgrade implementation")
            );
        }
    }

    /// @dev Before materialize: every op target still holds the implementation the seal expects it to.
    function _checkPreState(string memory manifest) internal {
        string[] memory roles = _upgradeProxyRoles();
        for (uint256 i; i < roles.length; i++) {
            _expectAddr(
                _implementationOf(_readManifestAddress(manifest, roles[i])),
                _expectedPreviousImplementation(manifest, roles[i]),
                string.concat(roles[i], " holds its sealed pre-upgrade implementation")
            );
        }
        _checkUntouchedProxies(manifest);
    }

    // ---------------------------------------------------------------------------------------
    // Authority
    // ---------------------------------------------------------------------------------------

    /**
     * @dev Ownership is where the seal recorded it, and nothing is pending.
     *
     *      Against `.preUpgrade.admin`, not `cfg.admin`: the invariant is that authority did not MOVE.
     *      Comparing to config would pass an upgrade that had re-pointed ownership at the configured admin.
     *      A dangling `pendingOwner` on either contract is a latent takeover, so it fails even though
     *      `owner()` reads correctly.
     */
    function _checkOwnershipUnchanged(string memory manifest) internal {
        address acl = _readManifestAddress(manifest, R_ACL);
        address aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);
        address admin = vm.parseJsonAddress(manifest, ".preUpgrade.admin");

        _expectAddr(IOwnable2Step(acl).owner(), aclOwner, "ACL.owner() is still the same ACLOwner");
        _expectAddr(IOwnable2Step(acl).pendingOwner(), address(0), "ACL.pendingOwner() == 0");
        _expectAddr(IACLOwner(aclOwner).owner(), admin, "ACLOwner.owner() is still the pre-upgrade admin");
        _expectAddr(IACLOwner(aclOwner).pendingOwner(), address(0), "ACLOwner.pendingOwner() == 0");
        _expectAddr(IACLOwner(aclOwner).ACL_ADDRESS(), acl, "ACLOwner.ACL_ADDRESS() is still the same ACL");
    }

    /**
     * @dev Bounded on purpose: `PauserSet` has no enumeration, so this confirms named accounts are still
     *      pausers and cannot show nobody was added. The log scan in the coordinator covers the rest.
     */
    function _checkPausersUnchanged(string memory manifest) internal {
        address pauserSet = _readManifestAddress(manifest, R_PAUSER_SET);
        address aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);
        _expect(IPauserSet(pauserSet).isPauser(aclOwner), "PauserSet.isPauser(ACLOwner) still true");
        if (cfg.pauser0 != address(0)) {
            _expect(IPauserSet(pauserSet).isPauser(cfg.pauser0), "PauserSet.isPauser(operator) still true");
        }
    }

    // ---------------------------------------------------------------------------------------
    // The payload
    // ---------------------------------------------------------------------------------------

    /**
     * @dev The exact `ACLOwner.upgrade` payload, op by op, and its digest — for the human comparing what a
     *      wallet is about to sign with what this build says it should be.
     */
    function _printOps(string memory manifest) internal view {
        IACLOwner.Op[] memory ops = _ops(manifest);
        string[] memory roles = _upgradeProxyRoles();
        console.log("");
        console.log("  ACLOwner.upgrade ops, in order:");
        for (uint256 i; i < ops.length; i++) {
            console.log(string.concat("    [", vm.toString(i), "] ", roles[i]));
            console.log("        proxy         ", ops[i].proxy);
            console.log("        implementation", ops[i].implementation);
            console.log(string.concat("        calls          ", UpgradeInitData.initName(i)));
            console.log(string.concat("        initData      ", vm.toString(ops[i].initData)));
        }
        _printMigration();
        console.log("");
        console.log("  target  ", _readManifestAddress(manifest, R_ACL_OWNER));
        console.log(string.concat("  calldata keccak256  ", vm.toString(keccak256(_upgradeCalldata(manifest)))));
    }

    /**
     * @dev The only init parameters this upgrade carries, decoded: what `ProtocolConfig.initializeFromMigration`
     *      will be given. Sealed by `compute` off the live KMSVerifier (or from `--migration`), and printed here
     *      so the operator reads them as values, not as the hex above.
     */
    function _printMigration() internal view {
        KmsNode[] memory nodes = _migrationNodes();
        IProtocolConfig.KmsThresholds memory t = _migrationThresholds();
        console.log("");
        console.log("  init parameters (ProtocolConfig.initializeFromMigration):");
        console.log("    existingContextId ", _migrationContextId());
        for (uint256 i; i < nodes.length; i++) {
            console.log(string.concat("    kmsNode[", vm.toString(i), "]"));
            console.log("        signer   ", nodes[i].signerAddress);
            console.log("        txSender ", nodes[i].txSenderAddress);
            console.log(string.concat("        ip        ", nodes[i].ipAddress));
            console.log(string.concat("        storage   ", nodes[i].storageUrl));
        }
        console.log("    thresholds  publicDecryption", t.publicDecryption);
        console.log("                userDecryption  ", t.userDecryption);
        console.log("                kmsGen          ", t.kmsGen);
        console.log("                mpc             ", t.mpc);
        console.log("    the six other ops take no arguments (reinitializers / initializeFromEmptyProxy)");
    }
}
