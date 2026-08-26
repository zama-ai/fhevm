// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Not wired into the build, not compiled, not tested.

import {console} from "forge-std/Script.sol";
import {FhevmVerifyBase} from "./FhevmVerifyBase.s.sol";
import {LocalHostVersions} from "../../pkg/forge/src/_internal/LocalHostVersions.sol";
import {
    IVersioned,
    IOwnable2Step,
    IPauserSet,
    IACLOwner,
    IWiredInputVerifier,
    IWiredProtocolConfig
} from "./Interfaces.sol";

/**
 * @title  FhevmVerifyUpgrade
 * @notice The terminal conditions of plans/CREATE2_TESTNET_UPGRADE_PLAN.md §7 — the Solidity half.
 *
 * Read-only, no broadcast, no key.
 *
 * A deploy's verify asks "did this stack come into existence correctly". This asks a strictly harder
 * question: **did an existing stack change in exactly the intended ways, and in no others.** The second
 * half is what makes it hard, and it is why this shares no code with `FhevmVerify` — only the reporting
 * primitives and the two checks that are the same question for any deployment, both in `FhevmVerifyBase`.
 *
 * Two examples of how differently the same-looking check reads here:
 *
 *   - `FhevmVerify` DERIVES the signer sets from the published mnemonic, because a fresh deploy is
 *     supposed to hold exactly the keys that mnemonic produces. An upgrade cannot require that — a
 *     testnet stack need not have been deployed with our defaults — so it compares against the set
 *     `compute` snapshotted off the live chain instead. Deriving here would turn a survival check into an
 *     assertion about how the stack was born.
 *   - `FhevmVerify` checks `ACLOwner.owner() == cfg.admin`, i.e. the admin accepted. Here the invariant is
 *     that ownership did not MOVE, so the comparison is against the snapshot, not the config. Comparing
 *     to config would pass an upgrade that had quietly re-pointed ownership at the configured admin.
 *
 * ## The manifest fields this requires (§4)
 *
 * `FhevmComputeUpgradeAddresses.s.sol` seals these; this reads them and nothing else:
 *
 *   .address.<role>          the 9 supplied live roles, the 2 new proxies, the fresh shared empty
 *                            implementation, and `IMPL_<role>` for each of the 7 upgraded proxies
 *   .preUpgrade.admin        `ACLOwner.owner()` as it was BEFORE
 *   .preUpgrade.kmsSigners   `KMSVerifier.getKmsSigners()` as it was before
 *   .preUpgrade.kmsThreshold `KMSVerifier.getThreshold()` as it was before
 *   .preUpgrade.coprocessorSigners / .preUpgrade.coprocessorThreshold
 *                            `InputVerifier`'s, which the upgrade does not touch at all
 *
 * A snapshot taken by `compute` rather than by this script is deliberate and is the whole basis of the
 * check: by the time `verify` runs, the pre-upgrade values are gone from the chain. Reading them "before"
 * from inside an after-the-fact script is not possible, so the seal is the only witness — and it is
 * written before `materialize`, by a stage that cannot know what `materialize` will do.
 *
 * ## What is NOT here, and why
 *
 * §7.1's survey — every zero-argument getter on the live stack, unchanged — lives in
 * `upgrade-testnet.ts`, together with the ownership/pauser LOG scans and the `--handle` value re-read.
 * That split is a capability constraint, not a preference: Solidity cannot enumerate an ABI, so a
 * Solidity survey would be a hand-maintained list of getters — exactly what the survey exists to avoid.
 * The log scans need `eth_getLogs` over the upgrade's block range, which is likewise a coordinator job.
 *
 * The consequence is that **this script passing is necessary and not sufficient.** It says the intended
 * changes happened; only the coordinator's passes say nothing else did.
 */
contract FhevmVerifyUpgrade is FhevmVerifyBase {
    function run() external {
        _loadConfig();
        string memory manifest = _loadManifest();

        _banner("verify upgrade");

        address acl = _readManifestAddress(manifest, R_ACL);
        address aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);
        address pauserSet = _readManifestAddress(manifest, R_PAUSER_SET);

        _expectFactoryPresent();
        _checkCode(manifest);

        // Unlike a deploy, "not materialized" is not a state to report and carry on from — the stack is
        // then still the previous generation, and every check below would be answering about v12. Say so
        // once and stop, rather than emitting a wall of failures that all have one cause.
        if (_expectImplementations(manifest, _upgradedProxyRoles()) != 0) {
            _summary("the upgrade has been materialized");
            return;
        }

        _checkVersions(manifest);
        _expectWiring(manifest);
        _checkSurvivedValues(manifest);
        _checkOwnershipUnchanged(manifest, acl, aclOwner);
        _checkPausersUnchanged(pauserSet, aclOwner);

        _summary("every terminal condition for the upgrade");
        console.log("  NOT sufficient on its own: the section 7.1 survey and the log scans run in the");
        console.log("  coordinator. This says the intended changes happened, not that nothing else did.");
        _mainnetReplayNotice();
    }

    // ---------------------------------------------------------------------------------------
    // Which proxies this flow is responsible for
    // ---------------------------------------------------------------------------------------

    /**
     * @dev The 7 proxies §6's op list re-points, in op order.
     *
     *      Not `_allProxyRoles()`: `INPUT_VERIFIER_ADDRESS` and `CLEARTEXT_DB_ADDRESS` are absent from the
     *      op list because their bytecode is byte-identical across the two generations (§5, confirmed by
     *      `npm run list:upgrade-ops`). They therefore have no `IMPL_` entry in this manifest, and asking
     *      for one would fail on a correct upgrade.
     *
     *      Their exclusion is checked POSITIVELY elsewhere rather than assumed: `_checkVersions` requires
     *      `InputVerifier` to still report v0.2.0, and `_checkSurvivedValues` requires its signer set
     *      intact. An upgrade that re-pointed them by mistake fails there.
     */
    function _upgradedProxyRoles() private pure returns (string[] memory r) {
        r = new string[](7);
        r[0] = R_PROTOCOL_CONFIG;
        r[1] = R_KMS_GENERATION;
        r[2] = R_ACL;
        r[3] = R_FHEVM_EXECUTOR;
        r[4] = R_HCU_LIMIT;
        r[5] = R_KMS_VERIFIER;
        r[6] = R_CLEARTEXT_ARITHMETIC;
        for (uint256 i = 0; i < r.length; i++) {
            // Same guard, same reason, as _sharedProxyRoles': the length is a literal because Solidity has
            // no array-literal length, so an unset entry is possible and would flow into `_implRole` and
            // then into a manifest lookup for `IMPL_`, which parses as a missing key rather than as this.
            require(bytes(r[i]).length != 0, "FhevmVerifyUpgrade: _upgradedProxyRoles has an unset entry");
        }
    }

    // ---------------------------------------------------------------------------------------
    // Checks
    // ---------------------------------------------------------------------------------------

    /**
     * @dev Code at everything this flow either created or was handed.
     *
     *      The supplied live addresses are included on purpose. On a deploy, no code at a role means "not
     *      deployed yet"; here it means the operator pointed `--acl` (or another flag) at something that
     *      is not a contract, which §3.1 should have caught before any transaction. Re-asserting it lets
     *      this script stand alone as a statement about the chain.
     */
    function _checkCode(string memory manifest) private {
        string[] memory upgraded = _upgradedProxyRoles();
        string[] memory roles = new string[](upgraded.length * 2 + 5);
        uint256 n;
        for (uint256 i = 0; i < upgraded.length; i++) {
            roles[n++] = upgraded[i];
            roles[n++] = _implRole(upgraded[i]);
        }
        roles[n++] = R_INPUT_VERIFIER;
        roles[n++] = R_CLEARTEXT_DB;
        roles[n++] = R_PAUSER_SET;
        roles[n++] = R_ACL_OWNER;
        roles[n++] = R_IMPL_EMPTY_SHARED;
        require(n == roles.length, "FhevmVerifyUpgrade: role list arity");
        _expectCodeAt(manifest, roles);
    }

    /**
     * @dev The versions each proxy now reports, against the generated `LocalHostVersions`.
     *
     *      Generated, never hand-written (§7): the constants are read out of the `MAJOR/MINOR/PATCH`
     *      declarations in the sources this package vendors, so bumping a contract cannot leave a stale
     *      expectation here. A literal string would be a second copy of a version number, and the whole
     *      class of bug this file exists to catch is two copies disagreeing.
     *
     *      The last two lines are the interesting ones. They assert what did NOT move: `InputVerifier`
     *      still reports v0.2.0 and `PauserSet` its own version. Both are deliberately absent from the op
     *      list, so a moved version there means something re-pointed a proxy nobody intended to touch —
     *      the failure mode that a list of only positive expectations cannot see.
     */
    function _checkVersions(string memory manifest) private {
        _expectVersion(manifest, R_ACL, LocalHostVersions.ACL);
        _expectVersion(manifest, R_FHEVM_EXECUTOR, LocalHostVersions.FHEVM_EXECUTOR);
        _expectVersion(manifest, R_HCU_LIMIT, LocalHostVersions.HCU_LIMIT);
        _expectVersion(manifest, R_KMS_VERIFIER, LocalHostVersions.KMS_VERIFIER);
        _expectVersion(manifest, R_CLEARTEXT_ARITHMETIC, LocalHostVersions.CLEARTEXT_ARITHMETIC);
        _expectVersion(manifest, R_PROTOCOL_CONFIG, LocalHostVersions.PROTOCOL_CONFIG);
        _expectVersion(manifest, R_KMS_GENERATION, LocalHostVersions.KMS_GENERATION);

        _expectVersion(manifest, R_INPUT_VERIFIER, LocalHostVersions.INPUT_VERIFIER);
        _expectVersion(manifest, R_PAUSER_SET, LocalHostVersions.PAUSER_SET);
    }

    function _expectVersion(string memory manifest, string memory role, string memory want) private {
        address a = _readManifestAddress(manifest, role);
        _expectStr(IVersioned(a).getVersion(), want, string.concat(role, ".getVersion()"));
    }

    /**
     * @dev The values that must come through the upgrade untouched, against `compute`'s snapshot.
     *
     *      The KMS signer set is the load-bearing one, and not for the reason it looks like. §3.1 cannot
     *      validate `KMS_VERIFIER_ADDRESS` — nothing else on chain corroborates it — so an operator who
     *      points it at ANOTHER deployment's verifier gets through validation. v13 reads its KMS signers
     *      from `ProtocolConfig`, seeded during `materialize` from the migration input, which means a
     *      wrong verifier does not fail loudly: it silently REPLACES the signer set during what is
     *      supposed to be a migration. This comparison is what turns that into a failure. The identical
     *      mistake was found in `test/ts/upgrade-e2e.test.ts` and is asserted there too.
     *
     *      Note where each side comes from: the set is read from `ProtocolConfig` (v13's home for it) and
     *      compared against what was snapshotted off `KMSVerifier` (v12's home). The migration moved it,
     *      so reading v13's copy and comparing to v13's copy would compare the chain against itself.
     *
     *      `InputVerifier`'s coprocessor set is here for the opposite reason: nothing was supposed to
     *      touch it at all, so this is the check that its absence from the op list actually held.
     */
    function _checkSurvivedValues(string memory manifest) private {
        IWiredProtocolConfig pc = IWiredProtocolConfig(_readManifestAddress(manifest, R_PROTOCOL_CONFIG));
        _expectSignerSet(
            pc.getKmsSigners(),
            vm.parseJsonAddressArray(manifest, ".preUpgrade.kmsSigners"),
            "ProtocolConfig.getKmsSigners() == the pre-upgrade KMS set"
        );

        uint256 kmsThreshold = vm.parseJsonUint(manifest, ".preUpgrade.kmsThreshold");
        _expectUint(pc.getPublicDecryptionThreshold(), kmsThreshold, "publicDecryption threshold survived");
        _expectUint(pc.getUserDecryptionThreshold(), kmsThreshold, "userDecryption threshold survived");
        _expectUint(pc.getKmsGenThreshold(), kmsThreshold, "kmsGen threshold survived");
        _expectUint(pc.getMpcThreshold(), kmsThreshold, "mpc threshold survived");

        IWiredInputVerifier iv = IWiredInputVerifier(_readManifestAddress(manifest, R_INPUT_VERIFIER));
        _expectSignerSet(
            iv.getCoprocessorSigners(),
            vm.parseJsonAddressArray(manifest, ".preUpgrade.coprocessorSigners"),
            "InputVerifier.getCoprocessorSigners() untouched"
        );
        _expectUint(
            iv.getThreshold(),
            vm.parseJsonUint(manifest, ".preUpgrade.coprocessorThreshold"),
            "InputVerifier.getThreshold() untouched"
        );
    }

    /**
     * @dev §1's first invariant: ownership NEVER changes.
     *
     *      `ACLOwner` is the single atomic upgrade root, so its owner is root over the whole stack. An
     *      upgrade that moved it would hand the stack to whoever holds the new key — and it would still
     *      pass every version and wiring check above, because the code is correct and only the authority
     *      moved.
     *
     *      The `pendingOwner() == 0` pair is not tidiness. A dangling pending owner on either contract is
     *      a latent takeover: whoever holds that key can call `acceptOwnership()` at any future moment.
     *      An upgrade has no business STARTING a transfer, so a non-zero value here is a failure even
     *      though `owner()` still reads correctly.
     *
     *      Value comparison only. That nobody EMITTED an ownership event across the upgrade's blocks is
     *      the coordinator's scan, and the two are not redundant: values show the endpoints match, logs
     *      show the path between them was straight.
     */
    function _checkOwnershipUnchanged(string memory manifest, address acl, address aclOwner) private {
        address admin = vm.parseJsonAddress(manifest, ".preUpgrade.admin");

        _expectAddr(IOwnable2Step(acl).owner(), aclOwner, "ACL.owner() is still the same ACLOwner");
        _expectAddr(IOwnable2Step(acl).pendingOwner(), address(0), "ACL.pendingOwner() == 0");
        _expectAddr(IACLOwner(aclOwner).owner(), admin, "ACLOwner.owner() is still the pre-upgrade admin");
        _expectAddr(IACLOwner(aclOwner).pendingOwner(), address(0), "ACLOwner.pendingOwner() == 0");
        _expectAddr(IACLOwner(aclOwner).acl(), acl, "ACLOwner.acl() is still the same ACL");
    }

    /**
     * @dev §1's second invariant: the pauser set is the same set.
     *
     *      What can be asserted here is bounded, and the bound is worth naming: `PauserSet` exposes
     *      `isPauser(address)` and no enumeration. So this can only confirm that accounts SOMEONE THOUGHT
     *      TO NAME are still pausers — it cannot show that nobody else was added. Proving the membership
     *      never moved at all needs the `AddPauser`/`RemovePauser`/`SwapPauser` log scan, which is the
     *      coordinator's. Anyone reading a green run here should know which half they got.
     *
     *      That the ACL still points at the SAME PauserSet is not repeated here — it is one of
     *      `_expectWiring`'s readings, compared against the manifest's supplied `PAUSER_SET_ADDRESS`. That
     *      is the right witness for "the same one": §3.1 validated the supplied value by cross-reading
     *      `ACL.getPauserSetAddress()` off the live stack BEFORE any transaction, so the manifest holds a
     *      pre-upgrade fact. Re-comparing it to a variable read from that same manifest key would be a
     *      tautology, which is what this comment replaced.
     */
    function _checkPausersUnchanged(address pauserSet, address aclOwner) private {
        _expect(IPauserSet(pauserSet).isPauser(aclOwner), "PauserSet.isPauser(ACLOwner) still true");
        // Only if the operator configured one. An upgrade config need not name a pauser at all, and a zero
        // here means "not asserted", not "not a pauser".
        if (cfg.pauser0 != address(0)) {
            _expect(IPauserSet(pauserSet).isPauser(cfg.pauser0), "PauserSet.isPauser(operator) still true");
        }
    }
}
