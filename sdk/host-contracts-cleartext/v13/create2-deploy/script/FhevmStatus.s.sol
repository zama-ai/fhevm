// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Compiles; never run.

import {console} from "forge-std/Script.sol";
import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";
import {IOwnable2Step, IPauserSet, IACLOwner} from "./Interfaces.sol";

/**
 * @title FhevmStatus
 * @notice What is done, what is left, and why. Read-only: no broadcast, no key, no state written.
 *
 * Answers the question a resumable deploy makes unavoidable — "where did I get to?" — from the only
 * source that can be trusted for it. Resume on this path has no journal (§2), so a status board
 * assembled from local files would be a second opinion that can disagree with the chain. Everything
 * below is a chain read plus the sealed manifest.
 *
 * It never reverts on a bad stack. That is the point: it exists to be run when something is wrong,
 * so it classifies and reports rather than failing on the first problem the way the deploy scripts
 * deliberately do.
 *
 * ---------------------------------------------------------------------------------------------
 * The four verdicts a create can have
 * ---------------------------------------------------------------------------------------------
 *
 *   done       code at the sealed address. Nothing to do — including when someone else's
 *              transaction put it there (§4: frontrunning is harmless, because construction
 *              captures nothing from the caller, so their bytes are our bytes).
 *   todo       no code. The next `--stage creates` will send it.
 *   DRIFT      this build predicts a DIFFERENT address than the manifest sealed. Fatal, and not an
 *              attack — different initcode gives a different address, which is what CREATE2 is. It
 *              means the build moved under the seal. Check the build, not the mempool (§8).
 *   NO CODE    `vm.getCode` returned nothing: the artifact is not in out/. The build did not run,
 *              or ran with a different --out, or the contract was renamed.
 *
 * ---------------------------------------------------------------------------------------------
 * Steps A-F: done / ready / blocked
 * ---------------------------------------------------------------------------------------------
 *
 * "blocked" is reported with the specific unmet predicate, because on this path a step is never
 * blocked by "the previous stage did not run" in the abstract — it is blocked by a named piece of
 * chain state that some earlier step was supposed to write, and that is the thing worth printing.
 */
contract FhevmStatus is FhevmCreate2Base {
    uint256 private _done;
    uint256 private _todo;
    uint256 private _bad;

    function run() external {
        _loadConfig();
        string memory manifest = _loadManifest();

        _banner("status");

        _reportCreates(manifest);
        _reportSteps(manifest);
    }

    // =======================================================================================
    // The 22 creates
    // =======================================================================================

    function _reportCreates(string memory manifest) private {
        console.log("--- creates (22) ---");

        Create[] memory creates = _allCreates(manifest);
        for (uint256 i = 0; i < creates.length; i++) {
            _reportCreate(manifest, creates[i]);
        }

        console.log("");
        console.log("  done", _done);
        console.log("  todo", _todo);
        if (_bad != 0) {
            console.log("  PROBLEMS", _bad);
        }
        console.log("");
    }

    function _reportCreate(string memory manifest, Create memory c) private {
        address sealed_ = _readManifestAddress(manifest, c.role);

        if (c.initCode.length == 0) {
            _bad++;
            console.log(string.concat("  NO CODE  ", c.role, "  - artifact missing from out/, run forge build"));
            return;
        }

        address predicted = _predictCreate2Address(c.role, c.initCode);
        if (predicted != sealed_) {
            _bad++;
            console.log(string.concat("  DRIFT    ", c.role, "  - sealed"), sealed_);
            console.log("           this build predicts                            ", predicted);
            return;
        }

        // EIP-3860 (§11 R3). Not fatal to report, but a create that exceeds it fails on chain, and
        // finding that out here costs nothing.
        if (c.initCode.length > MAX_INITCODE_SIZE) {
            _bad++;
            console.log(string.concat("  TOO BIG  ", c.role, "  - initcode over the EIP-3860 limit:"), c.initCode.length);
            return;
        }

        if (_deployed(sealed_)) {
            _done++;
            console.log(string.concat("  done     ", c.role), sealed_);
        } else {
            _todo++;
            console.log(string.concat("  todo     ", c.role), sealed_);
        }
    }

    // =======================================================================================
    // Steps A-F
    // =======================================================================================

    function _reportSteps(string memory manifest) private {
        console.log("--- steps A-F ---");

        address acl = _readManifestAddress(manifest, R_ACL);
        address pauserSet = _readManifestAddress(manifest, R_PAUSER_SET);
        address aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);

        if (!_deployed(acl) || !_deployed(pauserSet) || !_deployed(aclOwner)) {
            console.log("  all blocked - the creates stage has not finished");
            console.log("");
            return;
        }

        address owner_ = IOwnable2Step(acl).owner();
        bool aOk = IPauserSet(pauserSet).isPauser(aclOwner);

        _reportA(pauserSet, aclOwner, owner_);
        _reportAPrime(pauserSet, owner_);
        _reportB(acl, aclOwner, owner_);
        _reportC(acl, aclOwner, owner_, aOk);
        _reportD(manifest, aclOwner, owner_);
        _reportE(aclOwner);
        _reportF(aclOwner);

        console.log("");
    }

    function _reportA(address pauserSet, address aclOwner, address aclCurrentOwner) private view {
        if (IPauserSet(pauserSet).isPauser(aclOwner)) {
            console.log("  A   addPauser(ACLOwner)     done");
        } else if (aclCurrentOwner == cfg.deployer) {
            console.log("  A   addPauser(ACLOwner)     ready");
        } else {
            // Not unrecoverable — §6.1 keeps it reachable via ACLOwner.execute forever — but it is
            // no longer this stage's to do, and step C will refuse until it is done.
            console.log("  A   addPauser(ACLOwner)     BLOCKED - ACL.owner() is no longer the deployer;");
            console.log("                                        use ACLOwner.execute(pauserSet, ...)");
        }
    }

    function _reportAPrime(address pauserSet, address aclCurrentOwner) private view {
        if (cfg.pauser0 == address(0)) {
            console.log("  A'  addPauser(operator)     not configured (FHEVM_PAUSER_0 unset) - optional");
        } else if (IPauserSet(pauserSet).isPauser(cfg.pauser0)) {
            console.log("  A'  addPauser(operator)     done");
        } else if (aclCurrentOwner == cfg.deployer) {
            console.log("  A'  addPauser(operator)     ready");
        } else {
            console.log("  A'  addPauser(operator)     BLOCKED - use ACLOwner.execute(pauserSet, ...)");
        }
    }

    function _reportB(address acl, address aclOwner, address aclCurrentOwner) private view {
        if (aclCurrentOwner == aclOwner || IOwnable2Step(acl).pendingOwner() == aclOwner) {
            console.log("  B   offer ACL ownership     done");
        } else if (aclCurrentOwner == cfg.deployer) {
            console.log("  B   offer ACL ownership     ready");
        } else {
            console.log("  B   offer ACL ownership     BLOCKED - ACL.owner() is not the deployer:", aclCurrentOwner);
        }
    }

    function _reportC(address acl, address aclOwner, address aclCurrentOwner, bool aOk) private view {
        if (aclCurrentOwner == aclOwner) {
            console.log("  C   accept ACL ownership    done");
            return;
        }
        if (IOwnable2Step(acl).pendingOwner() != aclOwner) {
            console.log("  C   accept ACL ownership    BLOCKED - ACL.pendingOwner() is not the ACLOwner (run B)");
        } else if (!aOk) {
            console.log("  C   accept ACL ownership    BLOCKED - ACLOwner is not a registered pauser (run A)");
        } else if (IACLOwner(aclOwner).owner() != cfg.deployer) {
            console.log("  C   accept ACL ownership    BLOCKED - the deployer no longer owns the ACLOwner");
        } else {
            console.log("  C   accept ACL ownership    ready");
        }
    }

    /**
     * @dev The tri-state (§8). The proxies' runtime code never changes when they are materialized,
     *      so this counts ERC-1967 implementation slots rather than calling `getCode`.
     *
     *      Mixed is the state worth having a status board for at all: `ACLOwner.upgrade` is atomic,
     *      so re-running it against a partly-materialized stack reverts permanently, and no sequence
     *      of retries gets out of it. This is where a human finds out.
     */
    function _reportD(string memory manifest, address aclOwner, address aclCurrentOwner) private {
        string[] memory proxyRoles = _allProxyRoles();
        uint256 live;
        uint256 foreign;

        for (uint256 i = 0; i < 9; i++) {
            address slot = _implementationOf(_readManifestAddress(manifest, proxyRoles[i]));

            // Before D the slot holds the EMPTY proxy implementation, not zero — an ERC1967Proxy sets
            // it in the constructor. Treating zero as "not yet materialized" would make the pending
            // state unreachable and report every un-upgraded proxy as foreign.
            if (slot == _readManifestAddress(manifest, _emptyImplRoleFor(i))) continue;

            if (slot == _readManifestAddress(manifest, _implRole(proxyRoles[i]))) {
                live++;
            } else {
                foreign++;
                console.log(string.concat("      ", proxyRoles[i], " holds an unsealed implementation"), slot);
            }
        }

        if (foreign != 0) {
            _bad++;
            console.log("  D   materialize             FATAL - proxies point at implementations this manifest");
            console.log("                                      did not seal. A human decides.");
        } else if (live == 9) {
            console.log("  D   materialize             done");
        } else if (live != 0) {
            _bad++;
            console.log("  D   materialize             FATAL - partially materialized, and upgrade is atomic:");
            console.log("                                      re-running reverts permanently. Slots filled:", live);
        } else if (aclCurrentOwner != aclOwner) {
            console.log("  D   materialize             BLOCKED - the ACLOwner does not own ACL yet (run C)");
        } else if (IACLOwner(aclOwner).owner() != cfg.deployer) {
            console.log("  D   materialize             BLOCKED - the deployer no longer owns the ACLOwner");
        } else {
            console.log("  D   materialize             ready");
        }
    }

    /// @dev E only OFFERS. Reported separately from F so that "offered" never reads as "handed over"
    ///      — between the two the deployer is still root, and that window is the whole reason
    ///      `Ownable2Step` is used here.
    function _reportE(address aclOwner) private view {
        address owner_ = IACLOwner(aclOwner).owner();

        if (owner_ == cfg.admin || IACLOwner(aclOwner).pendingOwner() == cfg.admin) {
            console.log("  E   offer to admin          done");
        } else if (owner_ == cfg.deployer) {
            console.log("  E   offer to admin          ready");
        } else {
            console.log("  E   offer to admin          BLOCKED - the ACLOwner is owned by neither the");
            console.log("                                        deployer nor the configured admin:", owner_);
        }
    }

    /// @dev The only step the deployer cannot send. Until it lands the deployer holds
    ///      `ACLOwner.execute` — an unrestricted call as `ACL.owner()`, i.e. root over the stack —
    ///      so "waiting" here is not a formality, it is the deployment still being unfinished.
    function _reportF(address aclOwner) private view {
        if (IACLOwner(aclOwner).owner() == cfg.admin) {
            console.log("  F   admin accepts           done - the deployer is no longer root");
        } else if (IACLOwner(aclOwner).pendingOwner() == cfg.admin) {
            console.log("  F   admin accepts           WAITING - the admin must send acceptOwnership()");
            console.log("                                        from its own key. Nobody else can.");
            console.log("                                        The deployer is STILL root until then.");
        } else {
            console.log("  F   admin accepts           BLOCKED - nothing has been offered to the admin (run E)");
        }
    }
}
