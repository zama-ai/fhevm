// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Compiles; never run.

import {console} from "forge-std/Script.sol";
import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";
import {IOwnable2Step, IPauserSet} from "./Interfaces.sol";

/**
 * @title FhevmRegisterPausers
 * @notice Steps A and A' — register the pausers in `PauserSet`, while the deployer still
 *         owns the ACL.
 *
 * ---------------------------------------------------------------------------------------------
 * Why this is its own script
 * ---------------------------------------------------------------------------------------------
 *
 * Because an independently invocable unit is what an orchestrator can sequence. The shell drives
 * these today; a TS driver will want to drive them tomorrow, and neither should
 * have to edit Solidity to change what runs when. A and A' are the natural seam: they are the only
 * calls that are not part of the ownership handover — they write to a contract with no proxy, no
 * upgrade path and no initializer — and the only two that remain reachable after the run is over
 * (via `ACLOwner.execute(pauserSet, addPauser(...))`, by the deployer before E and by the
 * admin after). Pauser policy can be revisited without touching the script that moves ownership.
 *
 * The split costs nothing in safety, because CONTROL FLOW NEVER GUARANTEED ORDER IN THE FIRST PLACE.
 * A single script is not a transaction: under --broadcast each step is its own, a run can die
 * between any two of them, and every stage has always been separately invocable. What makes order
 * safe is that each step carries a precondition checked against CHAIN STATE — which is where the
 * guarantee has to live whether these calls sit in one file or six.
 *
 * So FhevmAcceptACLOwnership gates step C on `PauserSet.isPauser(ACLOwner)`. That gate was always required;
 * having A and A' in the same file as C only made it easy not to notice. Read it as part of this
 * file.
 *
 * ---------------------------------------------------------------------------------------------
 * Why "before C" and not "before B"
 * ---------------------------------------------------------------------------------------------
 *
 * `PauserSet.addPauser` is `onlyACLOwner`, which resolves to `Ownable2StepUpgradeable(aclAdd).owner()`
 * — a live call, evaluated when `addPauser` runs. ACL is `Ownable2Step`, so `transferOwnership`
 * (step B) only OFFERS: `owner()` is still the deployer between B and C, and ownership actually moves
 * when the ACLOwner accepts. A and A' therefore need only precede **C**.
 *
 * (FhevmDeployScript's comment claims the deployer loses `addPauser` after the transfer. It does
 * not — the accept is what moves it. Harmless there because it does A before B anyway, but the false
 * model is exactly what would make someone order these two scripts wrongly.)
 *
 * ---------------------------------------------------------------------------------------------
 * Why the ACLOwner must be a pauser at all
 * ---------------------------------------------------------------------------------------------
 *
 * `ACLOwner.pause()` forwards to `ACL.pause()`, which is gated on the caller being a registered
 * pauser — and the ACLOwner is a contract, so it cannot register itself later without already having
 * the ability to make that call. Miss step A and the emergency stop is unreachable through the
 * standing admin. It is a terminal condition for that reason, not for tidiness.
 */
contract FhevmRegisterPausers is FhevmCreate2Base {
    address private acl;
    address private pauserSet;
    address private aclOwner;

    function run() external {
        _loadConfig();
        string memory manifest = _loadManifest();

        require(msg.sender == cfg.deployer, "FhevmRegisterPausers: broadcast sender is not FHEVM_DEPLOYER");

        // reorg depth. Here it guards the creates: `isPauser` and `ACL.owner()` below are read against
        // PauserSet and the ACL proxy, and a reorg that unwinds their creation would make both reads
        // answer from a block about to be orphaned.
        _requireMinBlock();

        acl = _readManifestAddress(manifest, R_ACL);
        pauserSet = _readManifestAddress(manifest, R_PAUSER_SET);
        aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);

        require(
            _deployed(acl) && _deployed(pauserSet) && _deployed(aclOwner),
            "FhevmRegisterPausers: run creates first"
        );

        _banner("pausers - steps A, A'");

        vm.startBroadcast();

        // A — the standing admin. Required.
        _addPauser(aclOwner, "A  addPauser(ACLOwner)");

        // A' — the operator pauser. Optional, and deliberately non-blocking: an unset
        // FHEVM_PAUSER_0 is a configuration choice, not an incomplete run.
        if (cfg.pauser0 == address(0)) {
            console.log("  A' skipped (FHEVM_PAUSER_0 unset)");
        } else {
            _addPauser(cfg.pauser0, "A' addPauser(operator)");
        }

        vm.stopBroadcast();

        console.log("");
        console.log("  next: FhevmOfferACLOwnership (step B)");
    }

    /**
     * @dev One gated registration.
     *
     *      PREDICATE `isPauser(who)` — already true is the normal resume case, and skipping is not
     *      cosmetic: `addPauser` reverts `AccountAlreadyPauser`, and it would do so in SIMULATION,
     *      which kills the whole run before any transaction exists. `forge script` simulates
     *      everything before it broadcasts anything, so this branch cannot live in the shell.
     *
     *      PRECONDITION `ACL.owner() == deployer` — fatal, not a retry. Either C has already run (in
     *      which case this must go through `ACLOwner.execute` instead) or the stack is in a state
     *      this run did not create.
     */
    function _addPauser(address who, string memory label) private {
        if (IPauserSet(pauserSet).isPauser(who)) {
            console.log(string.concat("  ", label, " - already done"), who);
            return;
        }

        require(
            IOwnable2Step(acl).owner() == cfg.deployer,
            string.concat(
                "FhevmRegisterPausers: ",
                label,
                " precondition - ACL.owner() is not the deployer (past step C? use ACLOwner.execute)"
            )
        );

        IPauserSet(pauserSet).addPauser(who);
        console.log(string.concat("  ", label), who);
    }
}
