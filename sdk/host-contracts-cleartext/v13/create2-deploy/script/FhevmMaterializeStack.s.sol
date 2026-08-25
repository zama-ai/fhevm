// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Compiles; never run.

import {console} from "forge-std/Script.sol";
import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";
import {MaterializeInitData} from "./MaterializeInitData.sol";
import {IOwnable2Step, IACLOwner} from "./Interfaces.sol";

/**
 * @title FhevmMaterializeStack
 * @notice Plan §6 step D — `ACLOwner.upgrade(ops)`. Nine empty proxies become the real stack in ONE
 *         transaction: each op points a proxy at its sealed implementation and runs the initializer
 *         in the same call.
 *
 * Atomic by construction, and that is the design, not a convenience. `ACLOwner.upgrade` loops over
 * the ops and reverts as a whole, so the stack is never left half-materialized. Nine separate
 * upgrades — which is what the nonce path's forge-fhevm ancestor did, before ACLOwner existed —
 * can fail midway and leave some proxies real and some still empty.
 *
 * ---------------------------------------------------------------------------------------------
 * The tri-state, and why the third state cannot be an else-branch (§8)
 * ---------------------------------------------------------------------------------------------
 *
 * The proxies' runtime CODE never changes when they are materialized — an ERC1967Proxy is the same
 * bytes before and after — so `getCode`, the predicate every other stage in this path uses, says
 * nothing at all here. What changes is the ERC-1967 implementation slot, and that is what gets read.
 *
 *   every slot == the sealed implementations         → done, skip
 *   every slot == their EMPTY proxy implementation   → run
 *   anything else (mixed, or a third implementation)     → FATAL, a human decides
 *
 * Note the run state is NOT "slot is zero". An ERC1967Proxy sets its implementation in the
 * constructor, and OpenZeppelin's _setImplementation reverts when that implementation has no code, so
 * a deployed proxy's slot is never zero. Before D, ACL points at EmptyUUPSProxyACL and the other
 * the rest share EmptyUUPSProxy — see _emptyImplRoleFor.
 *
 * Mixed is fatal rather than resumable because the atomicity that protects the happy path works
 * against a retry: re-running `upgrade` against a partially-materialized stack hits
 * `onlyFromEmptyProxy` / `reinitializer` on the proxies that already moved, and the ENTIRE batch
 * reverts — permanently, at every future attempt. No sequence of retries gets out of it.
 *
 * Reaching mixed state should be impossible, since one transaction carries all of them. It is checked
 * because "impossible" here means "impossible unless someone ran a different upgrade against these
 * proxies" — precisely the case where proceeding silently is worst.
 *
 * ---------------------------------------------------------------------------------------------
 * Preconditions
 * ---------------------------------------------------------------------------------------------
 *
 *   ACL.owner() == aclOwner        the only link in the chain this step has to re-check.
 *                                  `_authorizeUpgrade` on every proxy is `onlyACLOwner`, so the
 *                                  ACLOwner must already hold ACL — which is step C, which could not
 *                                  itself have run without A and B. One check covers all four.
 *   ACLOwner.owner() == deployer   `upgrade` is `onlyOwner`. True until the admin accepts at E.
 */
contract FhevmMaterializeStack is FhevmCreate2Base {
    function run() external {
        _loadConfig();
        string memory manifest = _loadManifest();

        require(msg.sender == cfg.deployer, "FhevmMaterializeStack: broadcast sender is not FHEVM_DEPLOYER");

        // §11 R2, and the step with the least recoverable failure. The tri-state below is decided by
        // one ERC-1967 slot read per proxy; if a reorg unwinds step C the `ACL.owner()` precondition is read
        // from a doomed block, and `upgrade` is the one call in this path that cannot be retried
        // against a stack it half-changed.
        _requireMinBlock();

        address acl = _readManifestAddress(manifest, R_ACL);
        address aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);
        require(_deployed(acl) && _deployed(aclOwner), "FhevmMaterializeStack: run creates first");

        _banner("step D - materialize the stack");

        (IACLOwner.Op[] memory ops, uint256 materialized) = _buildOps(manifest);

        if (materialized == _allProxyRoles().length) {
            console.log("  D  upgrade - already done (every slot match the seal)");
            return;
        }
        require(materialized == 0, "FhevmMaterializeStack: D - stack is partially materialized, this is not resumable");

        require(
            IOwnable2Step(acl).owner() == aclOwner,
            "FhevmMaterializeStack: D precondition - ACLOwner does not own ACL yet"
            " (run FhevmAcceptACLOwnership, step C)"
        );
        require(
            IACLOwner(aclOwner).owner() == cfg.deployer,
            "FhevmMaterializeStack: D precondition - deployer does not own the ACLOwner"
        );

        vm.startBroadcast();
        IACLOwner(aclOwner).upgrade(ops);
        vm.stopBroadcast();

        console.log("  D  every proxy materialized in one ACLOwner.upgrade");
        console.log("");
        console.log("  next: FhevmOfferACLOwnerToAdmin (step E)");
    }

    /**
     * @dev Assembles the ops and classifies the stack in the same pass, because both need the
     *      same slot reads and the classification decides whether the ops are used at all.
     *
     *      Note the ops are built even when the answer turns out to be "already done". That is
     *      deliberate: the loop is also where a proxy pointing at an UNSEALED implementation is
     *      caught, and that check has to run against all of them regardless of the verdict.
     */
    function _buildOps(string memory manifest) private view returns (IACLOwner.Op[] memory ops, uint256 materialized) {
        string[] memory proxyRoles = _allProxyRoles();
        ops = new IACLOwner.Op[](_allProxyRoles().length);

        address cleartextArithmeticAdd = _readManifestAddress(manifest, R_CLEARTEXT_ARITHMETIC);

        for (uint256 i = 0; i < 9; i++) {
            address proxy = _readManifestAddress(manifest, proxyRoles[i]);
            address sealedImpl = _readManifestAddress(manifest, _implRole(proxyRoles[i]));
            address live = _implementationOf(proxy);

            address emptyImpl = _readManifestAddress(manifest, _emptyImplRoleFor(i));

            if (live == sealedImpl) {
                materialized++;
            } else if (live != emptyImpl) {
                // Neither the sealed implementation nor the empty proxy this run created: something
                // else upgraded it. Not a retry, not a resume.
                revert(
                    string.concat(
                        "FhevmMaterializeStack: D - ",
                        proxyRoles[i],
                        " holds an implementation this manifest did not seal"
                    )
                );
            }

            // The creates stage made every implementation; if one is missing the batch would revert
            // inside `upgradeToAndCall` on a codeless address, with a far less obvious message.
            require(_deployed(sealedImpl), string.concat("FhevmMaterializeStack: D - no code at IMPL_", proxyRoles[i]));

            ops[i] = IACLOwner.Op({
                proxy: proxy,
                implementation: sealedImpl,
                initData: MaterializeInitData.initData(i, cleartextArithmeticAdd)
            });
        }
    }
}
