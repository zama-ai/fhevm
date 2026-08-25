// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Compiles; never run.

import {console} from "forge-std/Script.sol";
import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";
import {IOwnable2Step, IACLOwner} from "./Interfaces.sol";

/**
 * @title FhevmAcceptOwnershipAsAdmin
 * @notice Step F — `ACLOwner.acceptOwnership()`, sent BY THE ADMIN. The transaction that actually
 *         ends the deployment.
 *
 * There is no step F in the plan. §6 stops at E and §7 describes this only as prose — "the admin
 * must send `acceptOwnership()`… the runner waits for and verifies it". That prose IS a step: it has
 * a predicate, a precondition, a sender, and a terminal condition that fails without it. Leaving it
 * as an instruction printed at the end of E made it the one part of the sequence with no script,
 * which is also the one part that decides whether the deployer is still root over the stack.
 *
 * ---------------------------------------------------------------------------------------------
 * The one script here NOT sent by the deployer
 * ---------------------------------------------------------------------------------------------
 *
 * Every other script requires `msg.sender == FHEVM_DEPLOYER`. This one requires
 * `msg.sender == FHEVM_ADMIN`, and that inversion is the entire point: `Ownable2Step` exists so that
 * ownership cannot be pushed onto an address that has not demonstrated it can transact. A handover
 * the deployer could complete alone would prove nothing about the admin's key.
 *
 * So this script is only usable when the admin is a key the operator can sign with. When the admin
 * is a multisig — which is the case §7 is really written for — nobody runs this: the multisig
 * executes `acceptOwnership()` through its own flow, and `deploy-testnet.sh --stage accept-admin`
 * degrades to polling until it lands. Both paths end in the same chain state, and FhevmVerify cannot
 * tell them apart, which is correct.
 *
 * ---------------------------------------------------------------------------------------------
 * After this
 * ---------------------------------------------------------------------------------------------
 *
 * `ACLOwner.pendingOwner()` returns to 0 — `Ownable2Step._transferOwnership` clears it as it sets
 * the owner — which is what makes §7's "no dangling pendingOwner" condition satisfiable at all. A
 * pending owner that never accepts is a standing offer anyone holding that key can take up later.
 */
contract FhevmAcceptOwnershipAsAdmin is FhevmCreate2Base {
    function run() external {
        _loadConfig();
        string memory manifest = _loadManifest();

        // NOT the deployer. See the header — this is the point of the two-step handover.
        require(msg.sender == cfg.admin, "FhevmAcceptOwnershipAsAdmin: broadcast sender is not FHEVM_ADMIN");

        address aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);
        require(_deployed(aclOwner), "FhevmAcceptOwnershipAsAdmin: run creates first");

        // §11 R2. Guards E: the offer this accepts must be buried before it is read, or a reorg can
        // unwind the `pendingOwner` write that the precondition below just confirmed.
        _requireMinBlock();

        _banner("step F - accept ownership of the ACLOwner");

        if (IACLOwner(aclOwner).owner() == cfg.admin) {
            console.log("  F  acceptOwnership - already done");
            return;
        }

        require(
            IACLOwner(aclOwner).pendingOwner() == cfg.admin,
            "FhevmAcceptOwnershipAsAdmin: F precondition - ACLOwner.pendingOwner() is not the admin"
            " (run FhevmOfferACLOwnerToAdmin, step E)"
        );

        vm.startBroadcast();
        IOwnable2Step(aclOwner).acceptOwnership();
        vm.stopBroadcast();

        console.log("  F  ACLOwner ownership accepted by", cfg.admin);
        console.log("");
        console.log("  The deployer key is no longer root over this stack.");
        console.log("  next: FhevmVerify (plan section 7)");
    }
}
