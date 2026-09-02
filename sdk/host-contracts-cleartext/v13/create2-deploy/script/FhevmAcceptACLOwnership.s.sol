// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Compiles; never run.

import {console} from "forge-std/Script.sol";
import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";
import {IOwnable2Step, IPauserSet, IACLOwner} from "./Interfaces.sol";

/**
 * @title FhevmAcceptACLOwnership
 * @notice Step C — `ACLOwner.acceptACLOwnership()`, which forwards to `ACL.acceptOwnership()`.
 *
 * THIS is where ownership of the stack actually moves, not step B. One transaction, and afterwards
 * every `onlyACLOwner` gate in the system — `_authorizeUpgrade` on every proxy,
 * `PauserSet.addPauser`, `PauserSet.removePauser` — answers "the ACLOwner" instead of "the deployer".
 *
 * It is sent BY the deployer but AS the ACLOwner: `acceptACLOwnership` is `onlyOwner` on the
 * ACLOwner, which the deployer owns until step E, and the inner `ACL.acceptOwnership()` therefore has
 * the ACLOwner as `msg.sender` — which is what `ACL.pendingOwner()` was set to at B.
 *
 * ---------------------------------------------------------------------------------------------
 * The point of no return
 * ---------------------------------------------------------------------------------------------
 *
 * Every step that needs `ACL.owner() == deployer` must already have happened. In this deployment
 * that is exactly A and A' (`PauserSet.addPauser`), and both are gated below.
 *
 * "Point of no return" overstates it slightly, and the overstatement matters when deciding how hard
 * to fail: `ACLOwner.execute(pauserSet, addPauser(...))` keeps every one of those calls reachable
 * forever — by the deployer before E, by the admin after. So the gates here do not protect
 * against an unrecoverable state. They protect against a SILENT one, which is worse in practice:
 * skip the pausers, run everything to completion, and the stack looks finished, works for every
 * normal operation, and has no reachable emergency stop until someone reads a `verify` failure.
 * Failing here costs one command. Noticing after E costs a multisig round-trip.
 *
 * ---------------------------------------------------------------------------------------------
 * Preconditions
 * ---------------------------------------------------------------------------------------------
 *
 *   ACL.pendingOwner() == aclOwner    step B ran. Straight out of the precondition table — and note nothing but B can
 *                                     make it true, so this ordering never needed a gate invented
 *                                     for it.
 *   PauserSet.isPauser(aclOwner)      step A ran. The one check in this path that is NOT in the precondition table; see
 *                                     above for what it buys. A' is deliberately not required — it
 *                                     is optional by design, and requiring it would turn a
 *                                     configuration choice into a blocker.
 *   ACLOwner.owner() == deployer      the deployer may still drive the ACLOwner, i.e. step E has not
 *                                     been accepted by the admin. Reachable only by running the
 *                                     stages out of order, and worth naming rather than letting
 *                                     `OwnableUnauthorizedAccount` surface from inside simulation.
 */
contract FhevmAcceptACLOwnership is FhevmCreate2Base {
    function run() external {
        _loadConfig();
        string memory manifest = _loadManifest();

        require(msg.sender == cfg.deployer, "FhevmAcceptACLOwnership: broadcast sender is not FHEVM_DEPLOYER");

        // reorg depth, and the step where it matters most. Both of this step's ordering preconditions —
        // `pendingOwner == aclOwner` from B, `isPauser(aclOwner)` from A — are reads about
        // transactions sent by earlier stages. A reorg that unwinds A while this reads it as done
        // produces a stack that passes every later check and has no reachable emergency stop.
        _requireMinBlock();

        address acl = _readManifestAddress(manifest, R_ACL);
        address pauserSet = _readManifestAddress(manifest, R_PAUSER_SET);
        address aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);
        require(
            _deployed(acl) && _deployed(pauserSet) && _deployed(aclOwner),
            "FhevmAcceptACLOwnership: run creates first"
        );

        _banner("step C - accept ACL ownership");

        vm.startBroadcast();

        // PREDICATE — the resume case, and the only one. Unlike B there is no second form to check:
        // ACL.acceptOwnership() clears pendingOwner as it sets owner, so "accepted" is a single state.
        if (IOwnable2Step(acl).owner() == aclOwner) {
            console.log("  C  acceptACLOwnership - already done");
        } else {
            require(
                IOwnable2Step(acl).pendingOwner() == aclOwner,
                "FhevmAcceptACLOwnership: C precondition - ACL.pendingOwner() is not the ACLOwner"
                " (run FhevmOfferACLOwnership, step B)"
            );
            require(
                IPauserSet(pauserSet).isPauser(aclOwner),
                "FhevmAcceptACLOwnership: C precondition - ACLOwner is not a registered pauser"
                " (run FhevmRegisterPausers, steps A/A')"
            );
            require(
                IACLOwner(aclOwner).owner() == cfg.deployer,
                "FhevmAcceptACLOwnership: C precondition - deployer no longer owns the ACLOwner"
                " (step E already accepted? the admin must send this)"
            );

            IACLOwner(aclOwner).acceptACLOwnership();
            console.log("  C  ACL ownership accepted by", aclOwner);
        }

        vm.stopBroadcast();

        console.log("");
        console.log("  ACL.owner() is now the ACLOwner. addPauser is no longer the deployer's to call");
        console.log("  directly - use ACLOwner.execute(pauserSet, ...) from here on.");
        console.log("  next: FhevmMaterializeStack (step D)");
    }
}
