// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Not wired into the build, not compiled, not tested.

import {console} from "forge-std/Script.sol";
import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";

/**
 * @title FhevmComputeCreate2Addresses
 * @notice ONE pass of the three-build address pipeline (plan §5.3). Selected by `FHEVM_PASS`.
 *
 * NOT `pkg/forge/script/ComputeAddresses.s.sol`, which is the nonce path and is untouched by this
 * plan. Every address here is `vm.computeCreate2Address(salt, initCodeHash, factory)`; every address
 * there is `vm.computeCreateAddress(deployer, nonce)`. Both write the same `addresses.sol` for the
 * same consumers — they differ only in how the values are derived — so the file names are the only
 * thing telling a reader which set of rules applies, and "CREATE2" is in this one's name for that
 * reason. Confusing them is not a compile error; it is a stack deployed to the wrong addresses.
 *
 * The pipeline exists because `EmptyUUPSProxy` and `PauserSet` bake `aclAdd` as a compiled-in
 * immediate and this path forbids bytecode patching. Their init-code hashes — and therefore their
 * CREATE2 addresses — only exist after a build against a config that already holds the real
 * `aclAdd`. A single `forge script` cannot recompile mid-run, so the recompiles happen in the shell
 * and this script runs three times against three different out/ directories:
 *
 *   pass 1   in: any addresses.sol (the committed placeholders)
 *            hashes EmptyUUPSProxyACL, ERC1967Proxy — neither references a host address
 *            out: impl₁, aclAdd; writes addresses.sol = real ACL_ADDRESS + markers elsewhere
 *
 *   pass 2   in: that addresses.sol
 *            hashes EmptyUUPSProxy, PauserSet, ACLOwner — all three import aclAdd ONLY, so the
 *            marker siblings around it are harmless
 *            out: impl₃, the shared-impl proxies, pauserSetAdd, aclOwnerAdd; writes the complete
 *                 addresses.sol and pass2.json
 *
 *   pass 3   in: the complete addresses.sol
 *            hashes the implementations, which bake every host address and so could not be
 *            hashed before now
 *            ASSERTS every pass-1 and pass-2 hash is unchanged, scans for surviving markers,
 *            checks EIP-3860, writes manifest.json
 *
 * Pass 3's assertion is the safety net. If adding the real addresses moved a hash that pass 2
 * computed, the addresses pass 2 wrote are wrong, every implementation was compiled against them,
 * and the run must fail here rather than deploy a stack that disagrees with itself.
 *
 * Reads nothing from the chain except `block.chainid` — no --broadcast, no key.
 */
contract FhevmComputeCreate2Addresses is FhevmCreate2Base {
    function run() external {
        _loadConfig();
        uint256 pass = vm.envUint("FHEVM_PASS");

        vm.createDir(cfg.outDir, true);

        if (pass == 1) {
            _pass1();
        } else if (pass == 2) {
            _pass2();
        } else if (pass == 3) {
            _pass3();
        } else {
            revert("FhevmComputeCreate2Addresses: FHEVM_PASS must be 1, 2 or 3");
        }
    }

    // =======================================================================================
    // Pass 1 — the only address that feeds back into the graph
    // =======================================================================================

    /**
     * @dev §5.1: the dependency graph looks circular and is not. Only `aclAdd` feeds back, and it is
     *      computable from inputs alone:
     *
     *        impl₁  ← EmptyUUPSProxyACL initcode, which references no host address
     *        aclAdd ← ERC1967Proxy initcode ++ (impl₁, initialize(DEPLOYER))
     *
     *      The tempting shortcut — initialize the ACL proxy with the ACLOwner address and drop the
     *      transfer/accept pair — IS a genuine cycle: aclAdd ← ACL initcode ← ACLOwner address ←
     *      ACLOwner initcode ← aclAdd. §6 steps B and C are structural, not ceremony.
     */
    function _pass1() private {
        _banner("pass 1/3 - ACL");

        (address impl1, address aclAdd) = _computeAcl();

        console.log("  impl1 (EmptyUUPSProxyACL)", impl1);
        console.log("  ACL_ADDRESS             ", aclAdd);

        _writeAddresses(aclAdd, _markerAddresses());

        console.log("");
        console.log("  wrote", _addressesPath());
        console.log("  next: rebuild against it, then FHEVM_PASS=2");
    }

    /// @dev Deliberately re-derived in every pass rather than carried in the scratch file: both
    ///      artifacts are marker-independent, so the value must be bit-identical in all three builds,
    ///      and passes 2 and 3 assert exactly that.
    function _computeAcl() private view returns (address impl1, address aclAdd) {
        bytes memory implCode = _initCode(A_EMPTY_ACL);
        impl1 = _predictCreate2Address(R_IMPL_EMPTY_ACL, implCode);

        bytes memory proxyCode = _proxyInitCode(impl1, _aclProxyInitData(cfg.deployer));
        aclAdd = _predictCreate2Address(R_ACL, proxyCode);
    }

    // =======================================================================================
    // Pass 2 — everything that bakes aclAdd and nothing else
    // =======================================================================================

    function _pass2() private {
        _banner("pass 2/3 - proxies, PauserSet, ACLOwner");

        (address impl1, address aclAdd) = _computeAcl();

        // The shared implementation. Compiled against the real aclAdd now, so its initcode — and
        // therefore this address — is final.
        bytes memory sharedImplCode = _initCode(A_EMPTY_SHARED);
        address impl3 = _predictCreate2Address(R_IMPL_EMPTY_SHARED, sharedImplCode);

        // §5.4: the shared-impl proxies share ONE init-code hash — same implementation, same empty
        // `initialize()` — and are distinguished purely by salt. That is the whole reason a shared
        // EmptyUUPSProxy is worth keeping on this path too.
        bytes memory sharedProxyCode = _proxyInitCode(impl3, _sharedProxyInitData());
        string[] memory roles = _sharedProxyRoles();
        address[] memory proxies = new address[](roles.length);
        for (uint256 i = 0; i < roles.length; i++) {
            proxies[i] = _predictCreate2Address(roles[i], sharedProxyCode);
        }

        address pauserSetAdd = _predictCreate2Address(R_PAUSER_SET, _initCode(A_PAUSER_SET));

        // §5.1 step 6: a leaf. Its address is referenced by nothing — but it must still be
        // predictable, because step E hands it to the admin and `verify` checks it.
        address aclOwnerAdd = _predictCreate2Address(
            R_ACL_OWNER,
            _initCode(A_ACL_OWNER, abi.encode(cfg.deployer, aclAdd))
        );

        console.log("  impl1 (EmptyUUPSProxyACL)", impl1);
        console.log("  impl3 (EmptyUUPSProxy)   ", impl3);
        console.log("  ACL_ADDRESS             ", aclAdd);
        for (uint256 i = 0; i < roles.length; i++) {
            console.log(string.concat("  ", roles[i]), proxies[i]);
        }
        console.log("  PAUSER_SET_ADDRESS      ", pauserSetAdd);
        console.log("  ACL_OWNER               ", aclOwnerAdd);

        address[] memory rest = new address[](proxies.length + 1);
        for (uint256 i = 0; i < proxies.length; i++) {
            rest[i] = proxies[i];
        }
        rest[proxies.length] = pauserSetAdd;
        _writeAddresses(aclAdd, rest);

        Pass2 memory p;
        p.sharedImplHash = keccak256(sharedImplCode);
        p.sharedProxyHash = keccak256(sharedProxyCode);
        p.aclAdd = aclAdd;
        p.impl1 = impl1;
        p.impl3 = impl3;
        p.pauserSetAdd = pauserSetAdd;
        p.aclOwnerAdd = aclOwnerAdd;
        p.proxies = proxies;
        _writeScratch(p);

        console.log("");
        console.log("  wrote", _addressesPath());
        console.log("  next: rebuild against it, then FHEVM_PASS=3");
    }

    // =======================================================================================
    // Pass 3 — the implementations, the assertion, the seal
    // =======================================================================================

    function _pass3() private {
        _banner("pass 3/3 - implementations, assert, seal");

        string memory scratch = vm.readFile(_scratchPath());

        // -- the assertion (§5.3) ----------------------------------------------------------
        //
        // Recompute pass 2's inputs against build 3 and require them unchanged. If adding the shared-impl
        // real addresses moved EmptyUUPSProxy's or PauserSet's initcode, then every address pass 2
        // wrote is wrong, and every implementation in this build was compiled against wrong
        // addresses. Fail here.

        (, address aclAdd) = _computeAcl();
        require(aclAdd == vm.parseJsonAddress(scratch, ".aclAdd"), "pass3: aclAdd moved between builds");

        bytes32 sharedImplHash = keccak256(_initCode(A_EMPTY_SHARED));
        bytes32 sharedProxyHash = keccak256(
            _proxyInitCode(vm.parseJsonAddress(scratch, ".impl3"), _sharedProxyInitData())
        );

        require(
            sharedImplHash == vm.parseJsonBytes32(scratch, ".sharedImplHash"),
            "pass3: EmptyUUPSProxy init-code hash moved - pass-2 addresses are invalid"
        );
        require(
            sharedProxyHash == vm.parseJsonBytes32(scratch, ".sharedProxyHash"),
            "pass3: ERC1967Proxy init-code hash moved - pass-2 addresses are invalid"
        );
        require(
            _predictCreate2Address(R_PAUSER_SET, _initCode(A_PAUSER_SET)) ==
                vm.parseJsonAddress(scratch, ".pauserSetAdd"),
            "pass3: PauserSet init-code hash moved - pass-2 addresses are invalid"
        );
        require(
            _predictCreate2Address(R_ACL_OWNER, _initCode(A_ACL_OWNER, abi.encode(cfg.deployer, aclAdd))) ==
                vm.parseJsonAddress(scratch, ".aclOwnerAdd"),
            "pass3: ACLOwner init-code hash moved - pass-2 addresses are invalid"
        );

        console.log("  assertion passed: no pass-2 init-code hash moved");

        // -- the implementations -----------------------------------------------------------
        //
        // Hashed for the first and only time here. They bake every host address, so build 3 is the
        // earliest build in which their initcode is correct. Nothing references their addresses, so
        // computing them this late costs nothing.

        address[] memory impls = _computeImplementations();

        _writeManifest(scratch, impls);

        console.log("");
        console.log("  wrote", _manifestPath());
        console.log("  next: commit and PUSH the manifest, then deploy (plan 9)");
    }

    function _computeImplementations() private view returns (address[] memory impls) {
        string[] memory proxyRoles = _allProxyRoles();
        address[] memory markers = _markerAddresses();
        impls = new address[](proxyRoles.length);

        for (uint256 i = 0; i < impls.length; i++) {
            bytes memory code = _initCode(_implArtifact(i));

            // §11 R3: a ~24 KB runtime plus constructor args approaches the EIP-3860 ceiling, and a
            // create that exceeds it fails on chain, not here. Measure it while it is still free to
            // fix.
            require(code.length <= MAX_INITCODE_SIZE, string.concat("pass3: EIP-3860 overflow in ", proxyRoles[i]));

            // The remapping-took-effect check, and the reason `_markerAddresses` uses a recognisable
            // pattern rather than address(0). Without this, a silently-ignored FOUNDRY_REMAPPINGS
            // deploys marker addresses as if they were real ones — a stack that verifies against
            // itself and works for nobody. scripts/deploy.sh greps the ACL artifact for the same
            // reason; this checks every implementation, against every marker.
            for (uint256 m = 0; m < markers.length; m++) {
                require(
                    !_contains(code, markers[m]),
                    string.concat("pass3: placeholder marker survived the build in ", proxyRoles[i])
                );
            }

            impls[i] = _predictCreate2Address(_implRole(proxyRoles[i]), code);
            console.log(string.concat("  IMPL_", proxyRoles[i]), impls[i]);
        }
    }

    // =======================================================================================
    // addresses.sol
    // =======================================================================================
    //
    // Same file, same constant names, same consumer as the nonce path's ComputeAddresses.s.sol. The
    // two paths differ in how the values are derived and in nothing downstream — which is what makes
    // "this plan adds a second path; it replaces nothing" true at the build level too.

    function _writeAddresses(address aclAdd, address[] memory rest) private {
        require(
            rest.length == _sharedProxyRoles().length + 1,
            "FhevmComputeCreate2Addresses: rest must hold the shared-impl proxies + PauserSet"
        );

        string memory c = string.concat(
            "// SPDX-License-Identifier: BSD-3-Clause-Clear\n",
            "\n",
            "pragma solidity ^0.8.24;\n",
            "\n",
            "// Auto-generated by create2-deploy/script/FhevmComputeCreate2Addresses.s.sol - do not edit by hand.\n",
            "// CREATE2 addresses. Valid ONLY for this (deployer, version, deploymentId) triple:\n",
            "//   deployer     ",
            vm.toString(cfg.deployer),
            "\n//   deploymentId ",
            cfg.deploymentId,
            "\n//   version      ",
            cfg.version,
            "\n"
        );

        string[] memory roles = _sharedProxyRoles();
        c = string.concat(c, _constant(R_ACL, aclAdd));
        for (uint256 i = 0; i < roles.length; i++) {
            c = string.concat(c, _constant(roles[i], rest[i]));
        }
        c = string.concat(c, _constant(R_PAUSER_SET, rest[roles.length]));

        vm.writeFile(_addressesPath(), c);
    }

    function _constant(string memory name, address value) private pure returns (string memory) {
        return string.concat("\naddress constant ", name, " = address(", vm.toString(value), ");\n");
    }

    /// @dev Pass-1 fillers for the host-address constants that are not yet known. Recognisable on sight and
    ///      in a hex dump, so pass 3's scan finds them; `address(0)` would not be — it appears in
    ///      legitimately-compiled bytecode all the time.
    function _markerAddresses() private pure returns (address[] memory m) {
        m = new address[](_allProxyRoles().length);
        for (uint256 i = 0; i < m.length; i++) {
            m[i] = address(uint160(0xdead0000 + i));
        }
    }

    /// @dev Naive 20-byte scan. Fine for a handful of artifacts of ~24 KB in a script that runs once.
    function _contains(bytes memory haystack, address needle) private pure returns (bool) {
        bytes20 n = bytes20(needle);
        if (haystack.length < 20) return false;
        for (uint256 i = 0; i <= haystack.length - 20; i++) {
            bool hit = true;
            for (uint256 j = 0; j < 20; j++) {
                if (haystack[i + j] != n[j]) {
                    hit = false;
                    break;
                }
            }
            if (hit) return true;
        }
        return false;
    }

    // =======================================================================================
    // pass2.json (scratch) and manifest.json (the seal)
    // =======================================================================================

    /// @dev A struct, not a long parameter list: with via_ir off, legacy codegen runs out of stack slots
    ///      well before that — the same "Stack too deep" the nonce path's `_materialize` works around.
    struct Pass2 {
        bytes32 sharedImplHash;
        bytes32 sharedProxyHash;
        address aclAdd;
        address impl1;
        address impl3;
        address pauserSetAdd;
        address aclOwnerAdd;
        address[] proxies;
    }

    function _writeScratch(Pass2 memory p) private {
        string memory o = "pass2";
        vm.serializeBytes32(o, "sharedImplHash", p.sharedImplHash);
        vm.serializeBytes32(o, "sharedProxyHash", p.sharedProxyHash);
        vm.serializeAddress(o, "aclAdd", p.aclAdd);
        vm.serializeAddress(o, "impl1", p.impl1);
        vm.serializeAddress(o, "impl3", p.impl3);
        vm.serializeAddress(o, "pauserSetAdd", p.pauserSetAdd);
        vm.serializeAddress(o, "aclOwnerAdd", p.aclOwnerAdd);
        string memory out = vm.serializeAddress(o, "proxies", p.proxies);
        vm.writeJson(out, _scratchPath());
    }

    /**
     * @dev The seal (§9). Committed and PUSHED before any transaction, for a reason stronger than
     *      audit trail: the addresses ARE a function of the init-code hashes, so retrying a failed
     *      create needs the byte-exact ones, and a resumed run's first act — deciding which
     *      addresses to probe — needs them too.
     *
     *      The shell adds the fields this script cannot see: toolchain pins, the factory's runtime
     *      code hash as observed on THIS chain, and the §11 R1 warning.
     */
    function _writeManifest(string memory scratch, address[] memory impls) private {
        string memory o = "manifest";

        vm.serializeUint(o, "chainId", cfg.chainId);
        vm.serializeString(o, "version", cfg.version);
        vm.serializeString(o, "deploymentId", cfg.deploymentId);
        vm.serializeAddress(o, "deployer", cfg.deployer);
        vm.serializeAddress(o, "admin", cfg.admin);
        vm.serializeAddress(o, "pauser0", cfg.pauser0);
        vm.serializeAddress(o, "factory", CREATE2_FACTORY);
        vm.serializeUint(o, "confirmations", cfg.confirmations);

        string memory addrs = _serializeRoleMap(scratch, impls);
        string memory out = vm.serializeString(o, "address", addrs);
        vm.writeJson(out, _manifestPath());
    }

    /// @dev The `address` map. `salt` and `initCodeHash` maps are built the same way; elided in this
    ///      draft to keep one function readable — see the shape in FhevmCreate2Base's header comment.
    function _serializeRoleMap(string memory scratch, address[] memory impls) private returns (string memory out) {
        string memory o = "addressMap";
        string[] memory proxyRoles = _allProxyRoles();
        address[] memory proxies = vm.parseJsonAddressArray(scratch, ".proxies");

        vm.serializeAddress(o, R_ACL, vm.parseJsonAddress(scratch, ".aclAdd"));
        vm.serializeAddress(o, R_IMPL_EMPTY_ACL, vm.parseJsonAddress(scratch, ".impl1"));
        vm.serializeAddress(o, R_IMPL_EMPTY_SHARED, vm.parseJsonAddress(scratch, ".impl3"));
        vm.serializeAddress(o, R_PAUSER_SET, vm.parseJsonAddress(scratch, ".pauserSetAdd"));
        vm.serializeAddress(o, R_ACL_OWNER, vm.parseJsonAddress(scratch, ".aclOwnerAdd"));

        string[] memory shared = _sharedProxyRoles();
        for (uint256 i = 0; i < shared.length; i++) {
            vm.serializeAddress(o, shared[i], proxies[i]);
        }
        // All but the last, because `out` must be the return of the FINAL serialize call.
        uint256 last = proxyRoles.length - 1;
        for (uint256 i = 0; i < last; i++) {
            vm.serializeAddress(o, _implRole(proxyRoles[i]), impls[i]);
        }
        out = vm.serializeAddress(o, _implRole(proxyRoles[last]), impls[last]);
    }
}
