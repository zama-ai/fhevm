// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Compiles; never run.

import {console} from "forge-std/Script.sol";
import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";
import {IOwnable2Step} from "./Interfaces.sol";

/**
 * @title FhevmOfferACLOwnership
 * @notice Plan §6 step B — `ACL.transferOwnership(ACLOwner)`.
 *
 * OFFER, not transfer, and the name says so on purpose. ACL is `Ownable2Step`: this call only writes
 * `pendingOwner`. Ownership actually moves at step C, when the ACLOwner accepts. Everything gated on
 * `ACL.owner()` — `_authorizeUpgrade` on all nine proxies, `PauserSet.addPauser` — still answers
 * "the deployer" the instant after this script succeeds.
 *
 * (FhevmDeployScript's comment on the nonce path claims the deployer loses `addPauser` after the
 * transfer. It does not. Harmless there, because it happens to do A before B — but the false model is
 * what makes this step look like the point of no return when C is.)
 *
 * ---------------------------------------------------------------------------------------------
 * Why B and C exist at all
 * ---------------------------------------------------------------------------------------------
 *
 * §5.1: it is tempting to initialize the ACL proxy with the `ACLOwner` address directly and delete
 * both steps. That is a genuine cycle — `aclAdd` ← ACL initcode ← ACLOwner address ← ACLOwner
 * initcode ← `aclAdd` — and it is the one place the dependency graph does close on itself. The
 * two-step handover is structural, not ceremony.
 *
 * ---------------------------------------------------------------------------------------------
 * Ordering
 * ---------------------------------------------------------------------------------------------
 *
 * B must precede C, and unlike the A/A' case that needed no gate added: step C's §8 precondition is
 * already `ACL.pendingOwner() == aclOwner`, which nothing but this script can make true. Running the
 * stages out of order fails there, on a check the plan called for anyway.
 *
 * B has no ordering relationship with A or A' in either direction, and this script deliberately does
 * NOT check that FhevmRegisterPausers has run. `PauserSet.addPauser` is `onlyACLOwner`, which reads
 * `Ownable2StepUpgradeable(aclAdd).owner()` live (ACLOwnable.sol) — and `Ownable2Step.transferOwnership`
 * does not touch `owner()`, only `pendingOwner`. So B leaves `addPauser` exactly as callable as it
 * found it. All three steps want `ACL.owner() == deployer`, all three still have it until C, and C
 * checks for both of them.
 *
 * Adding a pauser check here would be worse than redundant: it would invent a dependency that does
 * not exist, and block a legitimate order — offer B now, settle the operator pauser address later
 * (A' is optional, and its address may not be known yet), then C.
 */
contract FhevmOfferACLOwnership is FhevmCreate2Base {
    function run() external {
        _loadConfig();
        string memory manifest = _loadManifest();

        require(msg.sender == cfg.deployer, "FhevmOfferACLOwnership: broadcast sender is not FHEVM_DEPLOYER");

        // §11 R2. Guards the creates: the predicate below reads `ACL.owner()` / `pendingOwner()`,
        // and a reorg that unwinds the ACL proxy's creation would make both answer from a block
        // about to be orphaned.
        _requireMinBlock();

        address acl = _readManifestAddress(manifest, R_ACL);
        address aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);
        require(_deployed(acl) && _deployed(aclOwner), "FhevmOfferACLOwnership: run creates first");

        _banner("step B - offer ACL ownership");

        vm.startBroadcast();

        // PREDICATE. Two ways this is already done: the offer stands (`pendingOwner`), or it stands
        // and was accepted (`owner`) — a resume that comes back after C. Re-offering in the second
        // case would hand the deployer a live `pendingOwner` on a contract it no longer owns, which
        // §7 then refuses to call complete.
        if (IOwnable2Step(acl).owner() == aclOwner || IOwnable2Step(acl).pendingOwner() == aclOwner) {
            console.log("  B  transferOwnership(ACLOwner) - already done");
        } else {
            // PRECONDITION. Fatal, not a retry: someone else owns the ACL, so this stack is not the
            // one this manifest describes.
            require(
                IOwnable2Step(acl).owner() == cfg.deployer,
                "FhevmOfferACLOwnership: B precondition - ACL.owner() is not the deployer"
            );

            IOwnable2Step(acl).transferOwnership(aclOwner);
            console.log("  B  ACL.transferOwnership(ACLOwner)", aclOwner);
        }

        vm.stopBroadcast();

        console.log("");
        console.log("  ACL.owner() is still the deployer - ownership moves at C, not here.");
        console.log("  next: FhevmAcceptACLOwnership (step C)");
    }
}
