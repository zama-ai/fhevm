// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Script, console} from "forge-std/Script.sol";

import {ACL} from "../../src/contracts/ACL.sol";
import {FHEVMExecutor} from "../../src/contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "../../src/contracts/KMSVerifier.sol";
import {InputVerifier} from "../../src/contracts/InputVerifier.sol";
import {HCULimit} from "../../src/contracts/HCULimit.sol";
import {ProtocolConfig} from "../../src/contracts/ProtocolConfig.sol";
import {KMSGeneration} from "../../src/contracts/KMSGeneration.sol";
import {PauserSet} from "../../src/contracts/immutable/PauserSet.sol";
import {KmsNode} from "../../src/contracts/shared/Structs.sol";
import {CleartextArithmetic} from "../../src/cleartext/CleartextArithmetic.sol";
import {CleartextDB} from "../../src/cleartext/CleartextDB.sol";
import {ACLOwner} from "../../src/upgrade/ACLOwner.sol";
import {LocalHostBootstrap} from "../src/_internal/LocalHostBootstrap.sol";
import {LocalHostVersions} from "../src/_internal/LocalHostVersions.sol";

import {
    aclAdd,
    fhevmExecutorAdd,
    kmsVerifierAdd,
    inputVerifierAdd,
    hcuLimitAdd,
    protocolConfigAdd,
    kmsGenerationAdd,
    pauserSetAdd,
    cleartextArithmeticAdd,
    cleartextDbAdd
} from "../../src/addresses/FHEVMHostAddresses.sol";

/// @dev The EIP-712 domain view (ERC-5267), used to check what each verifier was initialized with.
interface IEIP712Domain {
    function eip712Domain()
        external
        view
        returns (
            bytes1 fields,
            string memory name,
            string memory version,
            uint256 chainId,
            address verifyingContract,
            bytes32 salt,
            uint256[] memory extensions
        );
}

/**
 * @title VerifyFhevmDeploy
 * @notice Standalone validation of a deployed cleartext FHEVM stack. Sends nothing.
 *
 * Run it against any RPC to check that a stack is present, correctly wired, and carries the default
 * bootstrap. It is a *separate* tool from the deploy on purpose: FhevmDeployScript's own `require`s can
 * only compare what it just did against the same constants it used, so they cannot catch a stack that was
 * built from a stale config, deployed by an older revision, or half-upgraded afterwards. This reads the
 * chain back and checks it against the package.
 *
 * Usage (from the package root):
 *
 *   FOUNDRY_REMAPPINGS=fhevm-config-0.13.0/=internal/.deploy-config/ \
 *     forge script pkg/forge/script/VerifyFhevmDeploy.s.sol --rpc-url <rpc>
 *
 * No private key, and no required environment variables: the addresses come from the
 * `fhevm-config-0.13.0/` addresses.sol this script was compiled against, which is what makes it a check
 * of a *specific* deploy. Point it at a stack built from a different config and every address will report
 * no code — that is the correct answer, not a bug.
 *
 * Optional environment variables, each pinning an identity that is not derivable from the config:
 *   ACL_OWNER_ADDRESS      — the expected ACLOwner contract (CREATE(deployer, 12) on a fresh deploy, but
 *                            an inherited address on an upgraded stack, so it cannot be computed here)
 *   UPGRADE_ADMIN_ADDRESS  — the expected `ACLOwner.owner()`, i.e. the account that can upgrade the stack
 *   PAUSER_ADDRESS_0       — an operator pauser expected to be registered, matching FhevmDeployScript
 *
 * Set none and ownership is only checked structurally, which cannot tell a correct owner from a wrong
 * one — see group 4. Each is reported as skipped so a passing run never overstates what it verified.
 *
 * Works against a stack deployed by either path, `FhevmDeployScript` or the TypeScript `deploy()` with no
 * config, since both apply the same bootstrap.
 *
 * What it checks, in five groups:
 *   1. code     — every address holds code, and each proxy resolves to a real implementation
 *   2. versions — getVersion() per contract, against the version constants in pkg/src
 *   3. wiring   — the addresses compiled into each contract are the ones actually deployed. This is the
 *                 group that catches a mis-addressed build, and it needs reading rather than assuming:
 *                 the initializers make no cross-contract calls, so a stack whose code points at empty
 *                 addresses deploys perfectly quietly and only fails in use.
 *   4. ownership— ACL is owned by an ACLOwner *contract* (not an EOA) whose `acl()` points back at it,
 *                 plus whatever the optional env vars pin
 *   5. bootstrap— signers, KMS nodes, thresholds, HCU limits and both EIP-712 domains, against
 *                 LocalHostBootstrap (the generated mirror of DEFAULT_BOOTSTRAP_CONFIG)
 *
 * Every check is reported, then the script reverts if any failed — so one run lists everything wrong
 * rather than stopping at the first problem. The one exception is group 1: if an address holds no code
 * the run stops there, because every later check would revert on the same absent contract.
 */
contract VerifyFhevmDeploy is Script {
    uint256 private _passed;
    uint256 private _failed;
    uint256 private _skipped;

    function run() external {
        console.log("Verifying cleartext FHEVM stack at the compiled-in addresses");
        console.log("");

        _checkCode();

        // Stop here if anything is missing. Every later group calls into these addresses, so on an empty
        // or wrongly-addressed chain they would all revert with Foundry's own "call to non-contract
        // address" long before the summary — which reads like a broken tool rather than an absent stack.
        if (_failed != 0) {
            console.log("");
            console.log("failed:", _failed);
            revert(
                "VerifyFhevmDeploy: no stack at the compiled-in addresses. Either nothing is deployed on"
                " this chain, or this build was compiled for a different deployer/start nonce - re-run"
                " ComputeAddresses and forge build against the deploy you mean to verify."
            );
        }

        _checkVersions();
        _checkWiring();
        _checkOwnership();
        _checkCoprocessors();
        _checkKmsContext();
        _checkThresholds();
        _checkHcuLimits();
        _checkEip712Domains();

        console.log("");
        console.log("passed: ", _passed);
        console.log("failed: ", _failed);
        console.log("skipped:", _skipped);
        require(_failed == 0, "VerifyFhevmDeploy: stack does not match this package");
        console.log("OK - stack matches this package and the default bootstrap");
    }

    // ------------------------------------------------------------------
    // 1. code
    // ------------------------------------------------------------------

    function _checkCode() private {
        console.log("[code]");
        _hasCode("ACL", aclAdd);
        _hasCode("FHEVMExecutor", fhevmExecutorAdd);
        _hasCode("KMSVerifier", kmsVerifierAdd);
        _hasCode("InputVerifier", inputVerifierAdd);
        _hasCode("HCULimit", hcuLimitAdd);
        _hasCode("ProtocolConfig", protocolConfigAdd);
        _hasCode("KMSGeneration", kmsGenerationAdd);
        _hasCode("CleartextArithmetic", cleartextArithmeticAdd);
        _hasCode("CleartextDB", cleartextDbAdd);
        _hasCode("PauserSet", pauserSetAdd);
    }

    // ------------------------------------------------------------------
    // 2. versions
    // ------------------------------------------------------------------

    /**
     * @dev The three cleartext subclasses inherit `getVersion` from the contract that declares it, so they
     *      report the base name — `CleartextFHEVMExecutor` identifies itself as `FHEVMExecutor`, which is
     *      why the constant below is keyed by the reporting name rather than the deployed type.
     *      CleartextDB declares no `getVersion` at all, hence its absence here — it is covered by the
     *      wiring group.
     *
     *      The expected strings come from `LocalHostVersions`, generated from the contracts' own
     *      CONTRACT_NAME + MAJOR/MINOR/PATCH constants, so a version bump cannot leave this check stale.
     */
    function _checkVersions() private {
        console.log("[versions]");
        _eqStr("ACL.getVersion", ACL(aclAdd).getVersion(), LocalHostVersions.ACL);
        _eqStr("FHEVMExecutor.getVersion", FHEVMExecutor(fhevmExecutorAdd).getVersion(), LocalHostVersions.FHEVM_EXECUTOR);
        _eqStr("KMSVerifier.getVersion", KMSVerifier(kmsVerifierAdd).getVersion(), LocalHostVersions.KMS_VERIFIER);
        _eqStr("InputVerifier.getVersion", InputVerifier(inputVerifierAdd).getVersion(), LocalHostVersions.INPUT_VERIFIER);
        _eqStr("HCULimit.getVersion", HCULimit(hcuLimitAdd).getVersion(), LocalHostVersions.HCU_LIMIT);
        _eqStr("ProtocolConfig.getVersion", ProtocolConfig(protocolConfigAdd).getVersion(), LocalHostVersions.PROTOCOL_CONFIG);
        _eqStr("KMSGeneration.getVersion", KMSGeneration(kmsGenerationAdd).getVersion(), LocalHostVersions.KMS_GENERATION);
        _eqStr(
            "CleartextArithmetic.getVersion",
            CleartextArithmetic(cleartextArithmeticAdd).getVersion(),
            LocalHostVersions.CLEARTEXT_ARITHMETIC
        );
        _eqStr("PauserSet.getVersion", PauserSet(pauserSetAdd).getVersion(), LocalHostVersions.PAUSER_SET);
    }

    // ------------------------------------------------------------------
    // 3. wiring — the compiled-in addresses are the deployed ones
    // ------------------------------------------------------------------

    function _checkWiring() private {
        console.log("[wiring]");
        _eqAddr("ACL.getFHEVMExecutorAddress", ACL(aclAdd).getFHEVMExecutorAddress(), fhevmExecutorAdd);
        _eqAddr("ACL.getPauserSetAddress", ACL(aclAdd).getPauserSetAddress(), pauserSetAdd);
        _eqAddr("FHEVMExecutor.getACLAddress", FHEVMExecutor(fhevmExecutorAdd).getACLAddress(), aclAdd);
        _eqAddr("FHEVMExecutor.getHCULimitAddress", FHEVMExecutor(fhevmExecutorAdd).getHCULimitAddress(), hcuLimitAdd);
        _eqAddr(
            "FHEVMExecutor.getInputVerifierAddress",
            FHEVMExecutor(fhevmExecutorAdd).getInputVerifierAddress(),
            inputVerifierAdd
        );
        _eqAddr("HCULimit.getFHEVMExecutorAddress", HCULimit(hcuLimitAdd).getFHEVMExecutorAddress(), fhevmExecutorAdd);
        // CleartextDB's initial writer is CleartextArithmetic — the cleartext half of the wiring.
        _isTrue(
            "CleartextDB.isWriter(CleartextArithmetic)", CleartextDB(cleartextDbAdd).isWriter(cleartextArithmeticAdd)
        );
    }

    // ------------------------------------------------------------------
    // 4. ownership
    // ------------------------------------------------------------------

    /**
     * @dev The stack must be owned by an `ACLOwner` contract rather than an EOA. That is not cosmetic:
     *      `updateV12ToV13` requires it, and an EOA-owned stack has no atomic upgrade path. Checked both
     *      ways — the owner has code, and its `acl()` points back at this ACL — so an unrelated contract
     *      holding ownership is not mistaken for a correct setup.
     *
     *      Those structural checks have a real limit, which is why the env pins exist: any `ACLOwner`
     *      deployed against this ACL satisfies them, including one an attacker deployed and had ownership
     *      transferred to. The address cannot be derived here — on a fresh deploy it is
     *      CREATE(deployer, 12), but it survives an upgrade, so on an upgraded stack it came from the
     *      previous generation at an unrelated nonce. Only the caller knows which one to expect.
     *
     *      `UPGRADE_ADMIN_ADDRESS` is the more valuable of the two: `ACLOwner.owner()` is the account that
     *      can upgrade every proxy in the stack, and nothing else here constrains it at all.
     */
    function _checkOwnership() private {
        console.log("[ownership]");
        address owner = ACL(aclAdd).owner();

        if (owner.code.length == 0) {
            _fail("ACL.owner is an ACLOwner contract", "owner has no code (EOA-owned stack)");
            return;
        }
        _pass(string.concat("ACL.owner is a contract at ", vm.toString(owner)));
        _eqAddr("ACLOwner.acl points back at ACL", ACLOwner(owner).acl(), aclAdd);
        // The deploy registers the ACLOwner as a pauser while it still can — addPauser is onlyACLOwner,
        // so after the ownership transfer only the ACLOwner itself could add one.
        _isTrue("PauserSet.isPauser(ACLOwner)", PauserSet(pauserSetAdd).isPauser(owner));

        _eqAddrIfPinned("ACL.owner", owner, "ACL_OWNER_ADDRESS");
        _eqAddrIfPinned("ACLOwner.owner (upgrade admin)", ACLOwner(owner).owner(), "UPGRADE_ADMIN_ADDRESS");

        address pauser = vm.envOr("PAUSER_ADDRESS_0", address(0));
        if (pauser == address(0)) {
            _skip("PauserSet.isPauser(PAUSER_ADDRESS_0)", "PAUSER_ADDRESS_0");
        } else {
            _isTrue(
                string.concat("PauserSet.isPauser(", vm.toString(pauser), ")"), PauserSet(pauserSetAdd).isPauser(pauser)
            );
        }
    }

    // ------------------------------------------------------------------
    // 5. bootstrap
    // ------------------------------------------------------------------

    function _checkCoprocessors() private {
        console.log("[bootstrap: coprocessors]");
        address[] memory want = LocalHostBootstrap.coprocessorSigners();
        address[] memory got = InputVerifier(inputVerifierAdd).getCoprocessorSigners();
        _eqAddrArray("InputVerifier.getCoprocessorSigners", got, want);
        _eqUint(
            "InputVerifier.getThreshold",
            InputVerifier(inputVerifierAdd).getThreshold(),
            LocalHostBootstrap.COPROCESSOR_THRESHOLD
        );
    }

    function _checkKmsContext() private {
        console.log("[bootstrap: KMS context]");
        _eqAddrArray(
            "ProtocolConfig.getKmsSigners",
            ProtocolConfig(protocolConfigAdd).getKmsSigners(),
            LocalHostBootstrap.kmsSigners()
        );

        uint256 contextId = ProtocolConfig(protocolConfigAdd).getCurrentKmsContextId();
        KmsNode[] memory nodes = ProtocolConfig(protocolConfigAdd).getKmsNodesForContext(contextId);
        if (nodes.length != LocalHostBootstrap.KMS_NODE_COUNT) {
            _fail("ProtocolConfig KMS node count", "wrong length");
            return;
        }
        _pass("ProtocolConfig KMS node count");

        address[] memory txSenders = LocalHostBootstrap.kmsTxSenders();
        string[] memory ips = LocalHostBootstrap.kmsIpAddresses();
        string[] memory urls = LocalHostBootstrap.kmsStorageUrls();
        for (uint256 i = 0; i < nodes.length; i++) {
            _eqAddr("KMS node txSender", nodes[i].txSenderAddress, txSenders[i]);
            _eqStr("KMS node ipAddress", nodes[i].ipAddress, ips[i]);
            _eqStr("KMS node storageUrl", nodes[i].storageUrl, urls[i]);
        }
    }

    /// @dev DEFAULT_KMS_THRESHOLDS sets every threshold to the node count.
    function _checkThresholds() private {
        console.log("[bootstrap: thresholds]");
        uint256 want = LocalHostBootstrap.KMS_NODE_COUNT;
        ProtocolConfig pc = ProtocolConfig(protocolConfigAdd);
        _eqUint("publicDecryption", pc.getPublicDecryptionThreshold(), want);
        _eqUint("userDecryption", pc.getUserDecryptionThreshold(), want);
        _eqUint("kmsGen", pc.getKmsGenThreshold(), want);
        _eqUint("mpc", pc.getMpcThreshold(), want);
    }

    function _checkHcuLimits() private {
        console.log("[bootstrap: HCU limits]");
        HCULimit h = HCULimit(hcuLimitAdd);
        _eqUint("globalHCUCapPerBlock", h.getGlobalHCUCapPerBlock(), LocalHostBootstrap.HCU_CAP_PER_BLOCK);
        _eqUint("maxHCUDepthPerTx", h.getMaxHCUDepthPerTx(), LocalHostBootstrap.MAX_HCU_DEPTH_PER_TX);
        _eqUint("maxHCUPerTx", h.getMaxHCUPerTx(), LocalHostBootstrap.MAX_HCU_PER_TX);
    }

    /**
     * @dev The EIP-712 domains are the whole point of the two verifying-contract constants: a proof is
     *      signed over this domain, so a wrong verifyingContract or chainId means every signature the
     *      relayer produces fails to verify, with nothing at deploy time to indicate it.
     */
    function _checkEip712Domains() private {
        console.log("[bootstrap: EIP-712 domains]");
        _checkDomain("KMSVerifier", kmsVerifierAdd, "Decryption", LocalHostBootstrap.DECRYPTION_ADDRESS);
        _checkDomain(
            "InputVerifier", inputVerifierAdd, "InputVerification", LocalHostBootstrap.INPUT_VERIFICATION_ADDRESS
        );
    }

    function _checkDomain(string memory label, address target, string memory wantName, address wantVerifying) private {
        (, string memory name,, uint256 chainId, address verifyingContract,,) = IEIP712Domain(target).eip712Domain();
        _eqStr(string.concat(label, " domain name"), name, wantName);
        _eqUint(string.concat(label, " domain chainId"), chainId, LocalHostBootstrap.GATEWAY_CHAIN_ID);
        _eqAddr(string.concat(label, " domain verifyingContract"), verifyingContract, wantVerifying);
    }

    // ------------------------------------------------------------------
    // assertions — every one reports, none reverts, so a run lists all problems
    // ------------------------------------------------------------------

    function _pass(string memory what) private {
        _passed++;
        console.log(string.concat("  ok   ", what));
    }

    function _fail(string memory what, string memory detail) private {
        _failed++;
        console.log(string.concat("  FAIL ", what, " -- ", detail));
    }

    /// @dev A check the caller did not ask for. Counted separately: a skipped check is not a passing one,
    ///      and a run that silently omitted it would overstate what it proved.
    function _skip(string memory what, string memory envVar) private {
        _skipped++;
        console.log(string.concat("  skip ", what, " -- set ", envVar, " to check it"));
    }

    /// @dev Compares against an env-supplied expectation, or reports the check as skipped if unset.
    function _eqAddrIfPinned(string memory what, address got, string memory envVar) private {
        address want = vm.envOr(envVar, address(0));
        if (want == address(0)) {
            _skip(string.concat(what, " == ", envVar), envVar);
            return;
        }
        _eqAddr(string.concat(what, " == ", envVar), got, want);
    }

    function _hasCode(string memory what, address a) private {
        if (a.code.length == 0) {
            _fail(string.concat(what, " has code"), string.concat("no code at ", vm.toString(a)));
        } else {
            _pass(string.concat(what, " has code at ", vm.toString(a)));
        }
    }

    function _eqAddr(string memory what, address got, address want) private {
        if (got == want) {
            _pass(what);
        } else {
            _fail(what, string.concat("got ", vm.toString(got), ", want ", vm.toString(want)));
        }
    }

    function _eqUint(string memory what, uint256 got, uint256 want) private {
        if (got == want) {
            _pass(what);
        } else {
            _fail(what, string.concat("got ", vm.toString(got), ", want ", vm.toString(want)));
        }
    }

    function _eqStr(string memory what, string memory got, string memory want) private {
        if (keccak256(bytes(got)) == keccak256(bytes(want))) {
            _pass(string.concat(what, " = ", got));
        } else {
            _fail(what, string.concat('got "', got, '", want "', want, '"'));
        }
    }

    function _isTrue(string memory what, bool got) private {
        if (got) {
            _pass(what);
        } else {
            _fail(what, "returned false");
        }
    }

    function _eqAddrArray(string memory what, address[] memory got, address[] memory want) private {
        if (got.length != want.length) {
            _fail(what, string.concat("got ", vm.toString(got.length), " entries, want ", vm.toString(want.length)));
            return;
        }
        for (uint256 i = 0; i < want.length; i++) {
            if (got[i] != want[i]) {
                _fail(
                    what,
                    string.concat(
                        "index ", vm.toString(i), ": got ", vm.toString(got[i]), ", want ", vm.toString(want[i])
                    )
                );
                return;
            }
        }
        _pass(string.concat(what, " (", vm.toString(want.length), " entries)"));
    }
}
