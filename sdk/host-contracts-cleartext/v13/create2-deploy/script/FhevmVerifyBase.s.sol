// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {console} from "forge-std/Script.sol";
import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";
import {
    IWiredACL,
    IWiredFHEVMExecutor,
    IWiredHCULimit,
    IWiredCleartextArithmetic,
    IWiredCleartextDB
} from "./Interfaces.sol";

/**
 * @title  FhevmVerifyBase
 * @notice Everything the deploy's `verify` and the upgrade's `verify` do identically.
 *
 * Extracted BEFORE the second consumer was written rather than after, which is the only time the split
 * is cheap: with one caller there is nothing to reconcile, so the boundary can be drawn where it belongs
 * instead of where two copies happen to have diverged.
 *
 * The line is drawn at **what a check is** versus **which checks to run**:
 *
 *   here                          the reporting mechanism (pass/fail, the failure counter, the summary
 *                                 and its revert), the comparison primitives that print both sides on
 *                                 failure, the comparison primitives that print both sides on failure
 *                                 (including whole signer arrays), and the two gates that are the same
 *                                 question for any deployment — is the canonical factory there, and does
 *                                 every role hold code
 *   the concrete verify           which roles, which values, which invariants, and in what order
 *
 * `FhevmVerify` and `FhevmVerifyUpgrade` therefore share no code with each other at all — only with
 * this. That is deliberate: they answer different questions. A deploy asserts a stack came into
 * existence correctly; an upgrade asserts an existing one changed in exactly the intended ways and in no
 * others. Letting either reach into the other's helpers would blur that, and the blurring is what makes
 * a check drift into "whatever the other one happened to need".
 *
 * @dev Every helper is `internal` rather than `private`: `private` in a base contract is invisible to
 *      the contract extending it, which is what made these live in the deploy's verify to begin with.
 */
abstract contract FhevmVerifyBase is FhevmCreate2Base {
    /**
     * @dev Failures are COUNTED, not thrown on.
     *
     *      One run should report everything wrong with a stack, not stop at the first thing. An operator
     *      fixing a testnet deploy wants the whole list; reverting on the first failure turns that into
     *      one round trip per problem. `_summary` is what finally reverts.
     */
    uint256 internal _failures;

    // ---------------------------------------------------------------------------------------
    // Reporting
    // ---------------------------------------------------------------------------------------

    function _expect(bool ok, string memory what) internal {
        if (!ok) {
            _failures++;
        }
        console.log(string.concat(ok ? "  ok   " : "  FAIL ", what));
    }

    /// @dev `_expect` with both addresses printed on failure — "wrong address" is useless without them.
    function _expectAddr(address got, address want, string memory what) internal {
        _expect(got == want, what);
        if (got != want) {
            console.log("         got ", got);
            console.log("         want", want);
        }
    }

    /// @dev Same, for a scalar.
    function _expectUint(uint256 got, uint256 want, string memory what) internal {
        _expect(got == want, what);
        if (got != want) {
            console.log("         got ", got);
            console.log("         want", want);
        }
    }

    /// @dev Same, for a string. Version strings are the main use, and a diff of one character matters.
    function _expectStr(string memory got, string memory want, string memory what) internal {
        bool ok = keccak256(bytes(got)) == keccak256(bytes(want));
        _expect(ok, what);
        if (!ok) {
            console.log("         got ", got);
            console.log("         want", want);
        }
    }

    /// @dev One signer array, element by element, with the mismatching entry printed.
    function _expectSignerSet(address[] memory got, address[] memory want, string memory what) internal {
        if (got.length != want.length) {
            _expect(false, string.concat(what, " - wrong length"));
            console.log("         got ", got.length);
            console.log("         want", want.length);
            return;
        }
        for (uint256 i = 0; i < want.length; i++) {
            if (got[i] != want[i]) {
                _expect(false, string.concat(what, " - mismatch"));
                console.log("         index", i);
                console.log("         got  ", got[i]);
                console.log("         want ", want[i]);
                return;
            }
        }
        _expect(true, string.concat(what, " (", vm.toString(want.length), " signers)"));
    }

    /**
     * @dev The final tally, and the only place this reverts.
     * @param what Named in the revert string, so a CI log says which verify failed without context.
     */
    function _summary(string memory what) internal view {
        console.log("");
        if (_failures == 0) {
            console.log(string.concat("  OK - ", what));
            return;
        }
        console.log("  failures:", _failures);
        revert(string.concat("FhevmVerify: ", what, " did not hold"));
    }

    // ---------------------------------------------------------------------------------------
    // Checks that are the same question for any deployment
    //
    // Deliberately short. Signer DERIVATION is not here: the deploy asks whether the stack registered the
    // addresses a published mnemonic produces, while the upgrade asks whether the set it already had came
    // through unchanged. Deriving during an upgrade would assert that a testnet stack was deployed with our
    // defaults, which is not something an upgrade gets to require.
    // ---------------------------------------------------------------------------------------

    /**
     * @dev The canonical factory has code.
     *
     *      Presence only. The §3 gate proper — that its RUNTIME code hashes to the known value — is the
     *      coordinator's, in `common.ts`, and runs in preflight before any script does. Repeating it here
     *      would need the expected hash as a second copy of a constant, which is how the two drift.
     */
    function _expectFactoryPresent() internal {
        _expect(_deployed(CREATE2_FACTORY), "factory has code at 0x4e59...");
    }

    /**
     * @dev Every role in `roles` holds code at the address the manifest records for it.
     *
     *      Returns rather than reverting so the caller decides: on a deploy an absent address means
     *      "not deployed yet", which is a legitimate state to report; on an upgrade it means the
     *      supplied stack is not there, which is fatal much earlier.
     */
    function _expectCodeAt(string memory manifest, string[] memory roles) internal returns (uint256 missing) {
        for (uint256 i = 0; i < roles.length; i++) {
            address a = _readManifestAddress(manifest, roles[i]);
            bool ok = _deployed(a);
            if (!ok) {
                missing++;
            }
            _expect(ok, string.concat("code at ", roles[i]));
        }
    }

    /**
     * @dev Which implementation each named proxy actually points at, against the manifest's seal.
     *
     *      The proxies' CODE is identical before and after a materialize, so presence proves nothing here
     *      and this reads the ERC-1967 slot instead.
     *
     * @param roles Proxy roles to check; the manifest's `IMPL_<role>` is the expected target. The caller
     *              chooses the list, and the two flows choose differently: a deploy checks every proxy,
     *              an upgrade only the ones its op list re-points.
     * @return mismatched How many point somewhere else. Returned rather than reverted on, because what it
     *                    MEANS differs: on a deploy, "not yet materialized" is a legitimate state to
     *                    report; on an upgrade it is the upgrade having not taken effect.
     */
    function _expectImplementations(
        string memory manifest,
        string[] memory roles
    ) internal returns (uint256 mismatched) {
        for (uint256 i = 0; i < roles.length; i++) {
            address live = _implementationOf(_readManifestAddress(manifest, roles[i]));
            address sealedImpl = _readManifestAddress(manifest, _implRole(roles[i]));
            bool ok = live == sealedImpl;
            if (!ok) {
                mismatched++;
            }
            _expect(ok, string.concat(roles[i], " points at its sealed implementation"));
        }
    }

    /**
     * @dev Every baked-in address the host contracts expose, checked against the manifest.
     *
     *      Distinct in kind from every other check here. The others ask the CHAIN questions — is there
     *      code at this address, which implementation does this proxy point at. These ask the BYTECODE
     *      what it was COMPILED WITH, and that is the only way to catch a stack assembled from a stale
     *      build, a FOUNDRY_REMAPPINGS that silently did not apply, or placeholder markers that survived
     *      into the implementations.
     *
     *      Common to both flows because it is the same assertion about the same v13 code: the address set
     *      compiled into the implementations is the one actually deployed. Only WHEN it is answerable
     *      differs — after a deploy's step D, and after an upgrade's materialize — and that is the
     *      caller's call, not this function's. Before either, the proxies point at empty implementations
     *      that have none of these functions, so every call reverts and takes the whole script down
     *      rather than reporting a failure.
     */
    function _expectWiring(string memory manifest) internal {
        address acl = _readManifestAddress(manifest, R_ACL);
        address executor = _readManifestAddress(manifest, R_FHEVM_EXECUTOR);
        address arithmetic = _readManifestAddress(manifest, R_CLEARTEXT_ARITHMETIC);

        _expectAddr(IWiredACL(acl).getFHEVMExecutorAddress(), executor, "ACL.getFHEVMExecutorAddress()");
        _expectAddr(
            IWiredACL(acl).getPauserSetAddress(),
            _readManifestAddress(manifest, R_PAUSER_SET),
            "ACL.getPauserSetAddress()"
        );

        _expectAddr(IWiredFHEVMExecutor(executor).getACLAddress(), acl, "FHEVMExecutor.getACLAddress()");
        _expectAddr(
            IWiredFHEVMExecutor(executor).getHCULimitAddress(),
            _readManifestAddress(manifest, R_HCU_LIMIT),
            "FHEVMExecutor.getHCULimitAddress()"
        );
        _expectAddr(
            IWiredFHEVMExecutor(executor).getInputVerifierAddress(),
            _readManifestAddress(manifest, R_INPUT_VERIFIER),
            "FHEVMExecutor.getInputVerifierAddress()"
        );
        _expectAddr(
            IWiredFHEVMExecutor(executor).getCleartextArithmeticAddress(),
            arithmetic,
            "CleartextFHEVMExecutor.getCleartextArithmeticAddress()"
        );

        _expectAddr(
            IWiredHCULimit(_readManifestAddress(manifest, R_HCU_LIMIT)).getFHEVMExecutorAddress(),
            executor,
            "HCULimit.getFHEVMExecutorAddress()"
        );
        _expectAddr(
            IWiredCleartextArithmetic(arithmetic).getCleartextDBAddress(),
            _readManifestAddress(manifest, R_CLEARTEXT_DB),
            "CleartextArithmetic.getCleartextDBAddress()"
        );
        _expectAddr(
            IWiredCleartextDB(_readManifestAddress(manifest, R_CLEARTEXT_DB)).getACLAddress(),
            acl,
            "CleartextDB.getACLAddress()"
        );
    }

    /**
     * @dev §11 R1, printed on every successful verify rather than filed in a document nobody reads.
     *
     *      The factory exists on mainnet, the manifest is public in git before the first transaction,
     *      and the signer keys derive from a published mnemonic. So anyone can deploy a bit-identical
     *      cleartext stack at these EXACT addresses on mainnet, with keys everyone has. A dApp that
     *      identifies this stack by address alone, pointed at the wrong chain, would function — with
     *      attacker-known keys. Our chain-id allow-list binds our tooling and nobody else's.
     */
    function _mainnetReplayNotice() internal view {
        console.log("");
        console.log("  NOTE: these addresses prove INITCODE, never chain or operator.");
        console.log("  The same set is replayable by anyone on any chain, mainnet included.");
        console.log("  Consumers MUST check chain id, not just address. chainId =", cfg.chainId);
    }
}
