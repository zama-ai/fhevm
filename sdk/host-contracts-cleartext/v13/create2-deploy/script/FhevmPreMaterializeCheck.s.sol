// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {console} from "forge-std/Script.sol";
import {FhevmUpgradeChecks} from "./FhevmUpgradeChecks.s.sol";
import {IACLOwner} from "./Interfaces.sol";

/**
 * @title  FhevmPreMaterializeCheck
 * @notice The gate between `creates` and `materialize`: everything that must be true BEFORE the one
 *         transaction that cannot be retried. Read-only, no broadcast, no key.
 *
 * `materialize` is the point of no return — `ACLOwner.upgrade` is atomic, its reinitializers are
 * `reinitializer(n)`-guarded, and a second run reverts. Everything the seal, the build and the chain can
 * be asked beforehand is asked here, and reported in full rather than stopped at the first failure, so an
 * operator sees the whole picture before deciding to send. `materialize` re-asserts the load-bearing
 * subset as `require`s of its own, so skipping this gate does not skip the checks — only the report.
 *
 * What this can see that the atomic call's simulation cannot:
 *
 *   - an implementation that IS a different contract than its role claims. Two contracts sharing a
 *     reinitializer selector would upgrade cleanly and fail only in use; `getVersion()` on the
 *     implementation itself catches the swap.
 *   - a build that no longer reproduces the seal, or code on chain that is not this build's.
 *   - a KMS signer rotation between `compute` and now, which would turn the sealed migration from a
 *     carry-over into a replacement.
 *   - a live proxy whose implementation moved since the seal — someone else upgraded under us.
 *
 * Run against a FRESH `--out` this is also the independent recompile: the current checkout must
 * reproduce the sealed init-code hashes byte for byte.
 */
contract FhevmPreMaterializeCheck is FhevmUpgradeChecks {
    function run() external {
        _loadUpgradeConfig();
        string memory manifest = _loadManifest();

        _banner("pre-materialize check");

        _expectFactoryPresent();

        console.log("--- the sealed build ---");
        _checkSealedBuild(manifest);
        _checkDeployedCode(manifest);

        console.log("--- the implementations, asked directly ---");
        _checkImplementationIdentity(manifest);
        _checkImplementationWiring(manifest);

        // A completed materialize is a legitimate state to meet here — a resumed `all` arrives at this gate
        // after the atomic call landed. The pre-state and migration checks are about the call that is
        // still to come, so they are answered by `verify` instead, and this says so rather than failing.
        if (_materialized(manifest)) {
            console.log("--- the live stack ---");
            console.log("  materialize already complete: every op target holds its sealed implementation");
            _summary("every pre-materialize condition (already materialized; run verify)");
            return;
        }

        console.log("--- the live stack, as sealed ---");
        _checkPreState(manifest);
        _checkOwnershipUnchanged(manifest);
        _expectAddr(
            IACLOwner(_readManifestAddress(manifest, R_ACL_OWNER)).owner(),
            cfg.admin,
            "--admin is the account that will send ACLOwner.upgrade"
        );
        _checkPausersUnchanged(manifest);

        console.log("--- the sealed migration ---");
        (bool fresh, string memory why) = _migrationMatchesLive();
        _expect(fresh, fresh ? "sealed KMS migration still matches the live KMSVerifier" : why);

        _summary("every pre-materialize condition");
        _printOps(manifest);
    }

    /// @dev All seven op targets already at their sealed implementation. Anything partial is left to
    ///      `_checkPreState`, which names each offending slot.
    function _materialized(string memory manifest) private view returns (bool) {
        string[] memory roles = _upgradeProxyRoles();
        for (uint256 i; i < roles.length; i++) {
            address live = _implementationOf(_readManifestAddress(manifest, roles[i]));
            if (live != _readManifestAddress(manifest, _implRole(roles[i]))) return false;
        }
        return true;
    }
}
