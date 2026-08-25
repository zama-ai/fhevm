// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Not wired into the build, not compiled, not tested.

import {console} from "forge-std/Script.sol";
import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";
import {
    IOwnable2Step,
    IPauserSet,
    IACLOwner,
    IWiredACL,
    IWiredFHEVMExecutor,
    IWiredHCULimit,
    IWiredCleartextArithmetic,
    IWiredCleartextDB,
    IWiredInputVerifier,
    IWiredKMSVerifier
} from "./Interfaces.sol";

/**
 * @title FhevmVerify
 * @notice The terminal conditions from plan §7. Reverts non-zero if any is unmet; the run is not
 *         "complete" until this passes.
 *
 * Read-only, no broadcast, no key. Runs against the chain and the MANIFEST — never against the
 * constants the deploy scripts used. That separation is the point: the deploy's own `require`s
 * compare what it just did against the same values it did it with, so they cannot catch a stack
 * built from a stale seal. Same reason VerifyFhevmDeploy.s.sol is separate on the nonce path.
 *
 * Run it twice, per §11 R2 — once at FHEVM_CONFIRMATIONS depth right after the deploy, and once at
 * greater depth at the end. Sepolia reorgs.
 */
contract FhevmVerify is FhevmCreate2Base {
    /**
     * Mirrored from pkg/ts/cleartext-config.ts. Deliberately literal rather than imported from
     * LocalHostBootstrap: the point of these checks is to be an independent statement of what the
     * signers should be, so taking them from the generated file would defeat it. If cleartext-config
     * changes, this has to change with it — and a drift between the two is exactly the failure these
     * lines exist to surface.
     *
     * The trailing index is supplied per-signer; foundry appends "/<index>" to the path.
     */
    string internal constant FHEVM_MNEMONIC = "test test test test test test test future home engine virtual motion";
    string internal constant COPROCESSOR_PATH = "m/44'/60'/0'/2";
    string internal constant KMS_PATH = "m/44'/60'/0'/3";
    uint256 internal constant COPROCESSOR_COUNT = 4;
    uint256 internal constant KMS_NODE_COUNT = 4;

    uint256 private _failures;

    function run() external {
        _loadConfig();
        string memory manifest = _loadManifest();

        _banner("verify");

        address acl = _readManifestAddress(manifest, R_ACL);
        address pauserSet = _readManifestAddress(manifest, R_PAUSER_SET);
        address aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);

        _checkFactory();
        _checkCode(manifest);
        bool materialized = _checkMaterialized(manifest);
        if (materialized) {
            _checkWiring(manifest);
            _checkSigners(manifest);
        } else {
            console.log("  ---- baked-in address checks skipped: the stack is not materialized (step D)");
        }

        _checkOwnership(acl, aclOwner);
        _checkPausers(pauserSet, aclOwner);

        console.log("");
        if (_failures != 0) {
            console.log("  FAILURES:", _failures);
            revert("FhevmVerify: stack does not match the manifest");
        }
        console.log("  OK - every terminal condition met");
        _mainnetReplayNotice();
    }

    // ---------------------------------------------------------------------------------------

    /// @dev §3's preflight, repeated here so `verify` standing alone is a complete statement about
    ///      the chain. The shell gates on this BEFORE deploying; this is the after-the-fact record.
    ///      A different contract squatting 0x4e59… on some testnet is the one realistic way a
    ///      "fatal mismatch" actually fires.
    function _checkFactory() private {
        _expect(_deployed(CREATE2_FACTORY), "factory has code at 0x4e59...");
    }

    function _checkCode(string memory manifest) private {
        string[] memory proxyRoles = _allProxyRoles();
        for (uint256 i = 0; i < proxyRoles.length; i++) {
            _expect(_deployed(_readManifestAddress(manifest, proxyRoles[i])), string.concat("code at ", proxyRoles[i]));
            _expect(
                _deployed(_readManifestAddress(manifest, _implRole(proxyRoles[i]))),
                string.concat("code at IMPL_", proxyRoles[i])
            );
        }
        _expect(_deployed(_readManifestAddress(manifest, R_PAUSER_SET)), "code at PAUSER_SET_ADDRESS");
        _expect(_deployed(_readManifestAddress(manifest, R_ACL_OWNER)), "code at ACL_OWNER");
        _expect(_deployed(_readManifestAddress(manifest, R_IMPL_EMPTY_ACL)), "code at IMPL_EMPTY_UUPS_PROXY_ACL");
        _expect(_deployed(_readManifestAddress(manifest, R_IMPL_EMPTY_SHARED)), "code at IMPL_EMPTY_UUPS_PROXY");
    }

    /// @dev The proxies' code is identical before and after step D, so this reads the ERC-1967 slot.
    function _checkMaterialized(string memory manifest) private returns (bool allMaterialized) {
        string[] memory proxyRoles = _allProxyRoles();
        allMaterialized = true;
        for (uint256 i = 0; i < proxyRoles.length; i++) {
            address live = _implementationOf(_readManifestAddress(manifest, proxyRoles[i]));
            address sealedImpl = _readManifestAddress(manifest, _implRole(proxyRoles[i]));
            bool ok = live == sealedImpl;
            if (!ok) allMaterialized = false;
            _expect(ok, string.concat(proxyRoles[i], " points at its sealed implementation"));
        }
    }

    /**
     * @dev Every baked-in address the host contracts expose, checked against the manifest.
     *
     *      Distinct in kind from everything else here. The other checks ask the CHAIN questions — is
     *      there code at this address, who owns this. These ask the BYTECODE what it was compiled
     *      with, and that is the only way to catch a stack assembled from a stale build, a
     *      FOUNDRY_REMAPPINGS that silently did not apply, or placeholder markers that survived into
     *      the implementations. Pass 3 scans for the same class of failure at build time; this is the
     *      same question asked of what actually got deployed.
     *
     *      Skipped unless every proxy are materialized: before step D they point at the empty
     *      implementations, which have none of these functions, so every call would revert and take
     *      the whole script with it rather than reporting a failure.
     */
    function _checkWiring(string memory manifest) private {
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
     * @dev The coprocessor and KMS signer sets, DERIVED FROM THE MNEMONIC rather than read off a
     *      generated file.
     *
     *      LocalHostBootstrap holds the same addresses, and step D seeded the chain from it — which
     *      is precisely why comparing against it here would be weak. It is a generated mirror; if it
     *      were regenerated wrongly, or generated from a different mnemonic, the chain and the mirror
     *      would agree with each other and both be wrong. Deriving from FHEVM_MNEMONIC at the paths
     *      in cleartext-config.ts checks the chain against the ACTUAL source, independently of
     *      whatever the build happened to bake in — which is §10's "chosen, not inherited" applied to
     *      the one part of the config that only exists in storage.
     *
     *      Why it matters (§12, §11 R1): these keys are what make the stack SDK-compatible. The
     *      js-sdk cleartext relayer derives its own keys from this mnemonic at these paths and looks
     *      a signer up by the address the chain reports. Seed a different set and everything else in
     *      this file still passes — the stack deploys, verifies against itself, and fails only when
     *      the relayer arrives. It is also why this stack is testnet-only: the mnemonic is published,
     *      so on mainnet these are keys everyone has.
     */
    function _checkSigners(string memory manifest) private {
        _checkSignerSet(
            IWiredInputVerifier(_readManifestAddress(manifest, R_INPUT_VERIFIER)).getCoprocessorSigners(),
            _derive(COPROCESSOR_PATH, COPROCESSOR_COUNT),
            "InputVerifier.getCoprocessorSigners()"
        );
        _expect(
            IWiredInputVerifier(_readManifestAddress(manifest, R_INPUT_VERIFIER)).getThreshold() == COPROCESSOR_COUNT,
            "InputVerifier.getThreshold() == coprocessor count"
        );

        // This generation keeps the KMS signer set and threshold on KMSVerifier itself; there is no
        // ProtocolConfig to read, and no per-node metadata stored on chain at all.
        IWiredKMSVerifier kv = IWiredKMSVerifier(_readManifestAddress(manifest, R_KMS_VERIFIER));
        _checkSignerSet(kv.getKmsSigners(), _derive(KMS_PATH, KMS_NODE_COUNT), "KMSVerifier.getKmsSigners()");
        _expect(kv.getThreshold() == KMS_NODE_COUNT, "KMSVerifier threshold");
    }

    /// @dev `count` consecutive addresses from FHEVM_MNEMONIC at `path`, starting at index 0.
    function _derive(string memory path, uint256 count) private pure returns (address[] memory out) {
        out = new address[](count);
        for (uint32 i = 0; i < count; i++) {
            out[i] = vm.addr(vm.deriveKey(FHEVM_MNEMONIC, path, i));
        }
    }

    /// @dev One signer array, element by element, with the mismatching entry printed.
    function _checkSignerSet(address[] memory got, address[] memory want, string memory what) private {
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

    /// @dev _expect with both addresses printed on failure — "wrong address" is useless without them.
    function _expectAddr(address got, address want, string memory what) private {
        _expect(got == want, what);
        if (got != want) {
            console.log("         got ", got);
            console.log("         want", want);
        }
    }

    /**
     * @dev §7's list, in full. The two `pendingOwner() == 0` checks are not tidiness: a dangling
     *      pending owner on either contract is a latent takeover — anyone holding that key can
     *      accept at any future moment — and it blocks completion.
     *
     *      `ACLOwner.owner() == admin` (not `pendingOwner == admin`) is what makes the admin's own
     *      `acceptOwnership()` transaction a gate rather than a suggestion. Until the admin has
     *      actually sent it, the deployer key is still root over the stack.
     */
    function _checkOwnership(address acl, address aclOwner) private {
        _expect(IOwnable2Step(acl).owner() == aclOwner, "ACL.owner() == ACLOwner");
        _expect(IOwnable2Step(acl).pendingOwner() == address(0), "ACL.pendingOwner() == 0");
        _expect(IACLOwner(aclOwner).owner() == cfg.admin, "ACLOwner.owner() == admin (admin accepted)");
        _expect(IACLOwner(aclOwner).pendingOwner() == address(0), "ACLOwner.pendingOwner() == 0");
        _expect(IACLOwner(aclOwner).acl() == acl, "ACLOwner.acl() == ACL");
    }

    function _checkPausers(address pauserSet, address aclOwner) private {
        _expect(IPauserSet(pauserSet).isPauser(aclOwner), "PauserSet.isPauser(ACLOwner)");
        if (cfg.pauser0 != address(0)) {
            _expect(IPauserSet(pauserSet).isPauser(cfg.pauser0), "PauserSet.isPauser(operator)");
        }
    }

    function _expect(bool ok, string memory what) private {
        if (!ok) {
            _failures++;
        }
        console.log(string.concat(ok ? "  ok   " : "  FAIL ", what));
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
    function _mainnetReplayNotice() private view {
        console.log("");
        console.log("  NOTE: these addresses prove INITCODE, never chain or operator.");
        console.log("  The same set is replayable by anyone on any chain, mainnet included.");
        console.log("  Consumers MUST check chain id, not just address. chainId =", cfg.chainId);
    }
}
