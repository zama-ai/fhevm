// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Not wired into the build, not compiled, not tested.

import {console} from "forge-std/Script.sol";
import {FhevmVerifyBase} from "./FhevmVerifyBase.s.sol";
import {FhevmCleartextConfig as C} from "./FhevmCleartextConfig.sol";
import {IOwnable2Step, IPauserSet, IACLOwner, IWiredInputVerifier, IWiredKMSVerifier} from "./Interfaces.sol";

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
contract FhevmVerify is FhevmVerifyBase {
    function run() external {
        _loadConfig();
        string memory manifest = _loadManifest();

        _banner("verify");

        address acl = _readManifestAddress(manifest, R_ACL);
        address pauserSet = _readManifestAddress(manifest, R_PAUSER_SET);
        address aclOwner = _readManifestAddress(manifest, R_ACL_OWNER);

        // §3's preflight, repeated so `verify` standing alone is a complete statement about the chain.
        // The coordinator gates on this BEFORE deploying; this is the after-the-fact record. A different
        // contract squatting 0x4e59... on some testnet is the one realistic way it actually fires.
        _expectFactoryPresent();
        _checkCode(manifest);
        bool materialized = _checkMaterialized(manifest);
        if (materialized) {
            _expectWiring(manifest);
            _checkSigners(manifest);
        } else {
            console.log("  ---- baked-in address checks skipped: the stack is not materialized (step D)");
        }

        _checkOwnership(acl, aclOwner);
        _checkPausers(pauserSet, aclOwner);

        _summary("every terminal condition for the deploy");
        _mainnetReplayNotice();
    }

    // ---------------------------------------------------------------------------------------

    /**
     * @dev Code at every address the deploy is responsible for: each proxy, each proxy's implementation,
     *      and the four singletons that are not proxies at all.
     */
    function _checkCode(string memory manifest) private {
        string[] memory proxyRoles = _allProxyRoles();
        string[] memory roles = new string[](proxyRoles.length * 2 + 4);
        uint256 n;
        for (uint256 i = 0; i < proxyRoles.length; i++) {
            roles[n++] = proxyRoles[i];
            roles[n++] = _implRole(proxyRoles[i]);
        }
        roles[n++] = R_PAUSER_SET;
        roles[n++] = R_ACL_OWNER;
        roles[n++] = R_IMPL_EMPTY_ACL;
        roles[n++] = R_IMPL_EMPTY_SHARED;
        require(n == roles.length, "FhevmVerify: role list arity");
        _expectCodeAt(manifest, roles);
    }

    /// @dev Every proxy, since a deploy materializes all of them at once (step D).
    function _checkMaterialized(string memory manifest) private returns (bool) {
        return _expectImplementations(manifest, _allProxyRoles()) == 0;
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
        _expectSignerSet(
            IWiredInputVerifier(_readManifestAddress(manifest, R_INPUT_VERIFIER)).getCoprocessorSigners(),
            _derive(C.CLEARTEXT_COPROCESSORS_MNEMONIC_PATH, C.CLEARTEXT_COPROCESSOR_COUNT),
            "InputVerifier.getCoprocessorSigners()"
        );
        _expect(
            IWiredInputVerifier(_readManifestAddress(manifest, R_INPUT_VERIFIER)).getThreshold() ==
                C.CLEARTEXT_COPROCESSOR_THRESHOLD,
            "InputVerifier.getThreshold() == coprocessor threshold"
        );

        // This generation keeps the KMS signer set and threshold on KMSVerifier itself; there is no
        // ProtocolConfig to read, and no per-node metadata stored on chain at all.
        IWiredKMSVerifier kv = IWiredKMSVerifier(_readManifestAddress(manifest, R_KMS_VERIFIER));
        _expectSignerSet(
            kv.getKmsSigners(),
            _derive(C.CLEARTEXT_KMS_NODES_MNEMONIC_PATH, C.CLEARTEXT_KMS_NODE_COUNT),
            "KMSVerifier.getKmsSigners()"
        );
        _expectUint(kv.getThreshold(), C.CLEARTEXT_KMS_NODE_COUNT, "KMSVerifier threshold");
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

    /**
     * @dev The signer pool at an HD path, derived rather than read from the chain.
     *
     *      Deriving is the point: comparing the chain against itself would pass whatever it held. These are
     *      the keys the js-sdk cleartext relayer will use, so the question is whether the stack registered
     *      the addresses those keys produce.
     */
    function _derive(string memory path, uint256 count) private pure returns (address[] memory out) {
        out = new address[](count);
        for (uint32 i = 0; i < count; i++) {
            out[i] = vm.addr(vm.deriveKey(C.FHEVM_MNEMONIC, path, i));
        }
    }

    function _checkPausers(address pauserSet, address aclOwner) private {
        _expect(IPauserSet(pauserSet).isPauser(aclOwner), "PauserSet.isPauser(ACLOwner)");
        if (cfg.pauser0 != address(0)) {
            _expect(IPauserSet(pauserSet).isPauser(cfg.pauser0), "PauserSet.isPauser(operator)");
        }
    }
}
