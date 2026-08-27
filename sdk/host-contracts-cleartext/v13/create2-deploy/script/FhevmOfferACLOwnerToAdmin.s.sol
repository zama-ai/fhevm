// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Compiles; never run.

import {console} from "forge-std/Script.sol";
import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";
import {IACLOwner} from "./Interfaces.sol";

/**
 * @title FhevmOfferACLOwnerToAdmin
 * @notice Step E — `ACLOwner.transferOwnership(admin)`. The last transaction the deployer
 *         sends, and the one that gives up root over the stack.
 *
 * This step is REQUIRED, not optional, and it is missing from `FhevmDeployScript` today — that
 * script constructs `new ACLOwner(deployer, aclAdd)` and never transfers it. `ACLOwner.execute` is an
 * unrestricted call made AS `ACL.owner()`, so whoever owns the ACLOwner can make any
 * `onlyACLOwner`-gated call on any host contract. The deployer must hold that during the run —
 * steps C and D are `onlyOwner` — which is exactly why a handover is structurally required rather
 * than good hygiene.
 *
 * ---------------------------------------------------------------------------------------------
 * This only OFFERS, and the run is not complete when it returns
 * ---------------------------------------------------------------------------------------------
 *
 * ACLOwner is `Ownable2Step`, so this writes `pendingOwner` and nothing else. The deployer is still
 * root afterwards. Ownership moves when the admin sends `acceptOwnership()` FROM ITS OWN KEY — a
 * transaction no script here can produce, and the reason FhevmVerify is a separate stage the
 * orchestrator waits for rather than a tail of this one.
 *
 * A dangling `pendingOwner` is a latent takeover, so `ACLOwner.pendingOwner() == 0` is a
 * terminal condition alongside `ACLOwner.owner() == admin`. Both are checked by FhevmVerify.
 *
 * `--admin` is mandatory with no default, and `_loadConfig` additionally refuses `admin == deployer`
 * — which would make this step a no-op that leaves the deployer as root forever while every check
 * downstream reports success.
 *
 * ---------------------------------------------------------------------------------------------
 * OPEN: E has no precondition on D
 * ---------------------------------------------------------------------------------------------
 *
 * The precondition table gives E one precondition, `ACLOwner.owner() == deployer`, and says nothing about
 * whether the stack was ever materialized. So this script can legitimately offer the ACLOwner to the
 * admin with every proxy still empty.
 *
 * That is not unsafe: step D is `onlyOwner` on the ACLOwner, so an admin who accepts early can
 * simply run D themselves. But an orchestrator that invokes stages out of order can hand over an
 * unmaterialized stack, and the hand-over is the point after which fixing anything costs a multisig
 * round-trip instead of a command. FhevmVerify catches it either way.
 *
 * This draft follows the precondition table and does NOT gate on D — it warns loudly instead. Promoting that warning to
 * a `require` is a design decision, not a draft one: it would be the second check in this path that is
 * not in the precondition table, and unlike C's pauser gate it forbids an ordering that is arguably legitimate (offer
 * early so the admin's multisig can schedule its acceptance while D runs).
 */
contract FhevmOfferACLOwnerToAdmin is FhevmCreate2Base {
    function run() external {
        _loadConfig();
        string memory manifest = _loadManifest();

        require(msg.sender == cfg.deployer, "FhevmOfferACLOwnerToAdmin: broadcast sender is not FHEVM_DEPLOYER");

        // reorg depth. Guards D: this is the last transaction the deployer sends, so a reorg that unwinds
        // the materialization after the admin has accepted leaves the fix on the far side of a
        // multisig. It is also what makes _warnIfNotMaterialized's slot reads worth trusting.
        _requireMinBlock();

        address aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);
        require(_deployed(aclOwner), "FhevmOfferACLOwnerToAdmin: run creates first");

        _banner("step E - offer the ACLOwner to the admin");

        // PREDICATE — offered or already accepted. Both are "done": re-offering after the admin has
        // accepted would need the deployer to still own the ACLOwner, which it no longer does.
        address owner_ = IACLOwner(aclOwner).owner();
        if (owner_ == cfg.admin || IACLOwner(aclOwner).pendingOwner() == cfg.admin) {
            console.log("  E  transferOwnership(admin) - already offered or accepted");
            return;
        }

        // PRECONDITION. Fatal: either this is not the stack the manifest describes, or the ACLOwner
        // was already handed to someone who is not our admin.
        require(
            owner_ == cfg.deployer,
            "FhevmOfferACLOwnerToAdmin: E precondition - deployer does not own the ACLOwner"
        );

        _warnIfNotMaterialized(manifest);

        vm.startBroadcast();
        IACLOwner(aclOwner).transferOwnership(cfg.admin);
        vm.stopBroadcast();

        console.log("  E  ACLOwner ownership OFFERED to", cfg.admin);
        console.log("");
        console.log("  The run is NOT complete. The deployer is still root over this stack until the");
        console.log("  admin sends acceptOwnership() from its own key:");
        console.log("");
        console.log("    cast send <ACL_OWNER> 'acceptOwnership()' --account <admin>");
        console.log("");
        console.log("  next: that transaction, then FhevmVerify");
    }

    /// @dev See the OPEN note in the header. A warning, deliberately, not a `require`.
    function _warnIfNotMaterialized(string memory manifest) private view {
        string[] memory proxyRoles = _allProxyRoles();
        for (uint256 i = 0; i < proxyRoles.length; i++) {
            if (_implementationOf(_readManifestAddress(manifest, proxyRoles[i])) == address(0)) {
                console.log("  WARNING: offering the ACLOwner while the stack is not materialized.");
                console.log("           Step D has not run. The admin will have to run it after accepting,");
                console.log("           and until then this stack answers no host call.");
                return;
            }
        }
    }
}
