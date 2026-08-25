// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Not wired into the build, not compiled, not tested.

import {console} from "forge-std/Script.sol";
import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";

/**
 * @title FhevmDeployCreates
 * @notice Stage 1 of the deploy: every CREATE2, every one gated on its own §8 predicate.
 *
 * Idempotent by construction. Re-running is the resume path, and resume needs no local state — the
 * predicate is `getCode(predicted) != ""`, a chain query (§2). The manifest is read for the expected
 * addresses, not for progress.
 *
 * ---------------------------------------------------------------------------------------------
 * Abort and re-run
 * ---------------------------------------------------------------------------------------------
 *
 * Interrupt this at any point — Ctrl-C, a dead RPC, a create that reverted — and the fix is to run
 * exactly the same command again. Whatever landed is skipped; whatever did not is retried AT THE
 * SAME ADDRESS. There is nothing to clean up and no journal that can disagree with the chain.
 *
 * That is the property §2 is about, and it is the one thing the nonce path cannot offer: there, a
 * transaction that reverts still consumes its nonce, so `CREATE(deployer, n)` becomes permanently
 * unfillable while every later address stays correct — a failure that stays silent until something
 * calls the missing contract.
 *
 * The one unsafe moment is re-running while the previous attempt's transactions are STILL IN THE
 * MEMPOOL. `forge script` simulates against a fork at the head, which does not see them, so the
 * predicates report "not deployed" for creates that are about to land; the re-sent creates then
 * revert on chain, because the factory reverts when CREATE2 returns zero. Gas and a nonce are lost;
 * the addresses are not. `deploy-testnet.sh` refuses to start when the deployer's pending nonce is
 * ahead of its latest, which is exactly that condition.
 *
 * NOTE this stage deliberately does not use `forge script --resume`. Resume replays the transaction
 * list from the PREVIOUS simulation, which was computed against older chain state; re-running from
 * scratch re-derives it from what is actually on chain now. The two answers differ precisely in the
 * cases that matter.
 *
 * ---------------------------------------------------------------------------------------------
 * Why the gate is in Solidity and not in the shell (§8)
 * ---------------------------------------------------------------------------------------------
 *
 * `forge script` simulates the WHOLE run before broadcasting anything. An ungated CREATE2 at an
 * address that is already occupied fails in simulation, and the run dies before a single transaction
 * exists. A shell-side "is it deployed?" check cannot prevent that, because by the time the shell
 * knows, forge has already decided what to simulate. So the branch has to be inside the script:
 *
 *     if (predicted.code.length == 0) { factoryCreate2(salt, initCode); }
 *
 * ---------------------------------------------------------------------------------------------
 * Ordering (§6)
 * ---------------------------------------------------------------------------------------------
 *
 * The work list and its order are `FhevmCreate2Base._allCreates`, which is also what FhevmStatus
 * reports on — see there for the two hard edges and why nonce ordering satisfies them for free.
 *
 * ---------------------------------------------------------------------------------------------
 * Frontrunning (§4)
 * ---------------------------------------------------------------------------------------------
 *
 * Anyone may call the factory with our salt and initcode, and it does not matter, because no
 * constructor here captures anything from the caller. Note the criterion is stronger than it looks:
 * the factory forwards to CREATE2, so `msg.sender` during EVERY constructor — including the
 * ERC1967Proxy constructor's `initialize` delegatecall — is the FACTORY, not us. Any contract
 * expecting `owner = msg.sender` would be broken in the honest run too. If someone frontruns a
 * create, the predicate simply reports it done and we skip it.
 */
contract FhevmDeployCreates is FhevmCreate2Base {
    uint256 private _created;
    uint256 private _skipped;

    function run() external {
        _loadConfig();
        string memory manifest = _loadManifest();

        // Identity, not authorisation. The whole address set is a function of the deployer (§5.2), so
        // broadcasting from a different account produces creates that land at addresses nothing was
        // compiled for. Requires `--sender` alongside `--account`.
        require(msg.sender == cfg.deployer, "FhevmDeployCreates: broadcast sender is not FHEVM_DEPLOYER");

        _banner("creates");

        // The work list, and its order, live in the base — FhevmStatus reports on the same table.
        Create[] memory creates = _allCreates(manifest);

        vm.startBroadcast();
        for (uint256 i = 0; i < creates.length; i++) {
            _create2(manifest, creates[i].role, creates[i].initCode);
        }
        vm.stopBroadcast();

        console.log("");
        console.log("  created", _created);
        console.log("  already present", _skipped);
        console.log("  next: FhevmRegisterPausers (A/A'), FhevmOfferACLOwnership (B),");
        console.log("        FhevmAcceptACLOwnership (C),");
        console.log("        FhevmMaterializeStack (D), FhevmOfferACLOwnerToAdmin (E)");
    }

    /**
     * @dev One gated CREATE2, through the factory. Never a plain CREATE — the deployer's nonce plays
     *      no part in any address on this path, which is the whole point of it (§2).
     *
     *      The address is recomputed from THIS build's initcode and checked against the manifest
     *      before anything is sent. §8: a mismatch here is fatal and is NOT an attack. Different
     *      initcode yields a different address — that is what CREATE2 is — so a mismatch can only
     *      mean the sealed hash is wrong (build drift) or the contract at 0x4e59… is not the
     *      canonical factory. Check the build and the factory preflight, not the mempool.
     *
     *      Returns the address either way, so callers can use it as a constructor argument whether
     *      it was just created or was already there.
     */
    function _create2(string memory manifest, string memory role, bytes memory initCode) private returns (address) {
        address sealed_ = _readManifestAddress(manifest, role);
        address predicted = _predictCreate2Address(role, initCode);

        require(
            predicted == sealed_,
            string.concat("FhevmDeployCreates: build drift - ", role, " does not match the sealed address")
        );

        if (_deployed(predicted)) {
            _skipped++;
            _logRole(role, predicted, true);
            return predicted;
        }

        _factoryCreate2(_salt(role), initCode);

        // Verify by reading code back, never by parsing the factory's return data (§3).
        //
        // This observes SIMULATED state — forge has not broadcast anything yet — so it proves the
        // initcode constructs and lands where predicted, not that the transaction was mined. What
        // proves the latter is the next run's predicate, which is the same check against real state.
        // That is not a weakness of this line; it is why the whole stage is re-runnable.
        require(_deployed(predicted), string.concat("FhevmDeployCreates: no code at ", role, " after create"));

        _created++;
        _logRole(role, predicted, false);
        return predicted;
    }
}
