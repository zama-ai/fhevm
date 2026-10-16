// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {console} from "forge-std/Script.sol";
import {FhevmUpgradeChecks} from "./FhevmUpgradeChecks.s.sol";
import {IVersioned, IWiredInputVerifier, IWiredProtocolConfig} from "./Interfaces.sol";

/**
 * @title  FhevmVerifyUpgrade
 * @notice The terminal conditions of an upgrade — the Solidity half.
 *
 * Read-only, no broadcast, no key.
 *
 * A deploy's verify asks "did this stack come into existence correctly". This asks a strictly harder
 * question: **did an existing stack change in exactly the intended ways, and in no others.** The second
 * half is what makes it hard, and it is why this shares no code with `FhevmVerify` — only the reporting
 * primitives in `FhevmVerifyBase`, and the upgrade's own checks in `FhevmUpgradeChecks`, which the gate
 * before materialize runs too.
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
 * ## The manifest fields this requires
 *
 * `FhevmComputeUpgradeAddresses.s.sol` and the coordinator seal these; this reads them and nothing else:
 *
 *   .address.<role>                  the 9 supplied live roles, the 2 new proxies, the fresh shared empty
 *                                    implementation, and `IMPL_<role>` for each of the 7 upgraded proxies
 *   .preUpgrade.admin                `ACLOwner.owner()` as it was BEFORE
 *   .preUpgrade.implementation.<r>   the ERC-1967 slot of each live proxy as it was before
 *   .preUpgrade.kmsSigners / .kmsThreshold / .migration.existingContextId
 *                                    `KMSVerifier`'s, which ProtocolConfig must now hold
 *   .preUpgrade.coprocessorSigners / .coprocessorThreshold
 *                                    `InputVerifier`'s, which the upgrade does not touch at all
 *
 * A snapshot taken by `compute` rather than by this script is deliberate and is the whole basis of the
 * check: by the time `verify` runs, the pre-upgrade values are gone from the chain. Reading them "before"
 * from inside an after-the-fact script is not possible, so the seal is the only witness — and it is
 * written before `materialize`, by a stage that cannot know what `materialize` will do.
 *
 * ## Independence
 *
 * The coordinator runs this against a FRESH `--out`, so `_checkSealedBuild` and `_checkDeployedCode` are
 * an independent recompile: the current checkout must reproduce the sealed init-code hashes, and the
 * code on chain must be that build's output. An auditor with the source, a node and the manifest can
 * re-run exactly this and needs to trust nothing the deploying machine kept.
 *
 * ## What is NOT here, and why
 *
 * The survey — every zero-argument getter on the live stack, unchanged — lives in
 * `upgrade-testnet.ts`, together with the event scans and the `--handle` value re-read. That split is a
 * capability constraint, not a preference: Solidity cannot enumerate an ABI, so a Solidity survey would be
 * a hand-maintained list of getters — exactly what the survey exists to avoid. The event scans need
 * `eth_getLogs` over the upgrade's block range, which is likewise a coordinator job.
 *
 * The consequence is that **this script passing is necessary and not sufficient.** It says the intended
 * changes happened; only the coordinator's passes say nothing else did.
 */
contract FhevmVerifyUpgrade is FhevmUpgradeChecks {
    function run() external {
        _loadUpgradeConfig();
        string memory manifest = _loadManifest();

        _banner("verify upgrade");

        _expectFactoryPresent();

        console.log("--- the sealed build ---");
        _checkSealedBuild(manifest);
        _checkDeployedCode(manifest);
        _checkImplementationIdentity(manifest);
        _checkImplementationWiring(manifest);
        _checkLiveCode(manifest);

        // Unlike a deploy, "not materialized" is not a state to report and carry on from — the stack is
        // then still the previous generation, and every check below would be answering about v12. Say so
        // once and stop, rather than emitting a wall of failures that all have one cause.
        console.log("--- the seven slots ---");
        if (_expectImplementations(manifest, _upgradeProxyRoles()) != 0) {
            _summary("the upgrade has been materialized");
            return;
        }
        _checkUntouchedProxies(manifest);

        console.log("--- the stack, after ---");
        _checkVersions(manifest);
        _expectWiring(manifest);
        _checkSurvivedValues(manifest);
        _checkOwnershipUnchanged(manifest);
        _checkPausersUnchanged(manifest);

        _summary("every terminal condition for the upgrade");
        console.log("  NOT sufficient on its own: the survey and the event scans run in the");
        console.log("  coordinator. This says the intended changes happened, not that nothing else did.");
        _mainnetReplayNotice();
    }

    // ---------------------------------------------------------------------------------------
    // Checks
    // ---------------------------------------------------------------------------------------

    /**
     * @dev Code at everything this flow was HANDED rather than created.
     *
     *      On a deploy, no code at a role means "not deployed yet"; here it means the operator pointed
     *      `--acl` (or another flag) at something that is not a contract, which the supplied-address
     *      validation should have caught before any transaction. Re-asserting it lets this script stand
     *      alone as a statement about the chain. The ten creates are covered by `_checkSealedBuild`.
     */
    function _checkLiveCode(string memory manifest) private {
        string[] memory roles = new string[](7);
        roles[0] = R_ACL;
        roles[1] = R_FHEVM_EXECUTOR;
        roles[2] = R_HCU_LIMIT;
        roles[3] = R_KMS_VERIFIER;
        roles[4] = R_CLEARTEXT_ARITHMETIC;
        roles[5] = R_INPUT_VERIFIER;
        roles[6] = R_CLEARTEXT_DB;
        _expectCodeAt(manifest, roles);
        string[] memory singletons = new string[](2);
        singletons[0] = R_PAUSER_SET;
        singletons[1] = R_ACL_OWNER;
        _expectCodeAt(manifest, singletons);
    }

    /**
     * @dev The versions each proxy now reports, against the generated `LocalHostVersions`.
     *
     *      The last three lines are the interesting ones. They assert what did NOT move: `InputVerifier`,
     *      `CleartextDB` and `PauserSet` are deliberately absent from the op list, so a moved version there
     *      means something re-pointed a proxy nobody intended to touch — the failure mode that a list of
     *      only positive expectations cannot see.
     */
    function _checkVersions(string memory manifest) private {
        string[] memory upgraded = _upgradeProxyRoles();
        for (uint256 i; i < upgraded.length; i++) {
            _expectVersion(manifest, upgraded[i]);
        }
        string[] memory untouched = _untouchedProxyRoles();
        for (uint256 i; i < untouched.length; i++) {
            _expectVersion(manifest, untouched[i]);
        }
        _expectVersion(manifest, R_PAUSER_SET);
    }

    function _expectVersion(string memory manifest, string memory role) private {
        address a = _readManifestAddress(manifest, role);
        _expectStr(IVersioned(a).getVersion(), _versionFor(role), string.concat(role, ".getVersion()"));
    }

    /**
     * @dev The values that must come through the upgrade untouched, against `compute`'s snapshot.
     *
     *      The KMS signer set is the load-bearing one, and not for the reason it looks like. The
     *      supplied-address validation cannot validate `KMS_VERIFIER_ADDRESS` — nothing else on chain
     *      corroborates it — so an operator who points it at ANOTHER deployment's verifier gets through
     *      validation. v13 reads its KMS signers from `ProtocolConfig`, seeded during `materialize` from
     *      the migration input, which means a wrong verifier does not fail loudly: it silently REPLACES the
     *      signer set during what is supposed to be a migration. This comparison is what turns that into a
     *      failure. The identical mistake was found in `test/ts/upgrade-e2e.test.ts` and is asserted there too.
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
        _expectUint(
            pc.getCurrentKmsContextId(),
            vm.parseUint(vm.parseJsonString(manifest, ".preUpgrade.migration.existingContextId")),
            "ProtocolConfig.getCurrentKmsContextId() == the pre-upgrade context id"
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
}
