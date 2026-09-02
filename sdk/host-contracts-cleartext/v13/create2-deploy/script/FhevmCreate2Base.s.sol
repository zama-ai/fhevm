// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Not wired into the build, not compiled, not tested.

import {Script, console} from "forge-std/Script.sol";

/**
 * @title FhevmCreate2Base
 * @notice Everything the four CREATE2 scripts share: config from the environment, the role table,
 *         salt derivation, init-code assembly, address prediction, the raw factory call, and the
 *         scratch/manifest JSON codec.
 *
 * Implements the shared mechanics: the factory, the salts and
 * the seal. The scripts that inherit from it implement the passes, the steps and their preconditions.
 *
 * ---------------------------------------------------------------------------------------------
 * Configuration — every script reads the same environment. No script takes a CLI argument.
 * ---------------------------------------------------------------------------------------------
 *
 *   FHEVM_VERSION           MAJOR_MINOR baked into every salt, e.g. "0.13". NOT the patch version:
 *                           a patch release must not move the addresses.
 *   FHEVM_DEPLOYMENT_ID     operator-chosen string. Distinct value ⇒ a disjoint address set on the
 *                           same chain; a redeploy always takes a fresh one.
 *   FHEVM_DEPLOYER          deployer ADDRESS, not a key. The compute and verify scripts only predict
 *                           with it; the six broadcasting scripts check it against `msg.sender` and
 *                           let forge authenticate via --account/--sender. No script in this path
 *                           ever holds a key. Addresses are a function of this value, so it
 *                           must be identical on every chain meant to share an address set.
 *   FHEVM_ADMIN             final owner of ACLOwner (step E). Mandatory, no default.
 *   FHEVM_PAUSER_0          optional operator pauser. Unset ⇒ step A' is skipped.
 *
 *   FHEVM_PASS              1 | 2 | 3 — which pass of the three-pass pipeline to run. Compute only.
 *   FHEVM_OUT_DIR           where addresses.sol, pass2.json and manifest.json are written.
 *                           Must be listed in foundry.toml `fs_permissions`.
 *
 *   FHEVM_CONFIRMATIONS     reorg depth for the shell's between-stage waits. Read here only so it
 *                           can be recorded in the manifest.
 *   FHEVM_MIN_BLOCK         refuse to run until the chain has reached this block. Required by each
 *                           of steps A/A', B, C, D and E; pass 0 for the first stage of a run.
 *                           See _requireMinBlock.
 *
 * The factory address is a constant, not an environment variable, deliberately: it is the one input
 * that must never vary per operator or per chain, and the preflight pins its runtime code hash.
 */
abstract contract FhevmCreate2Base is Script {
    // ---------------------------------------------------------------------------------------
    // The factory
    // ---------------------------------------------------------------------------------------

    // `CREATE2_FACTORY` — the canonical deterministic-deployment proxy at 0x4e59b448…, whose
    // calldata is `32-byte salt ++ initcode` — is NOT declared here. forge-std already declares it as
    // `internal constant` on `CommonBase`, which `Script` inherits, so a local copy collides
    // ("Identifier already declared"). Inheriting it is also the better answer: one source for the
    // one input that must never vary per operator or per chain.
    //
    // Inheriting the ADDRESS is not the preflight. The gate is on the factory's RUNTIME
    // CODE HASH, pinned in the manifest and checked per chain, because the realistic failure is a
    // different contract squatting that address on some testnet — which no constant, here or in
    // forge-std, can detect.

    /// @dev ERC-1967 implementation slot. The proxies' runtime code never changes, so this slot —
    ///      not `getCode` — is what says whether step D has run.
    bytes32 internal constant ERC1967_IMPL_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

    /// @dev EIP-3860. Checked per role in pass 3.
    uint256 internal constant MAX_INITCODE_SIZE = 49152;

    // ---------------------------------------------------------------------------------------
    // Roles — `role` is the salt's last field, and the address name
    // ---------------------------------------------------------------------------------------
    //
    // One create per entry in _creates(). The nonce path needs fewer, because it deploys the
    // implementations inside step 4
    // as ordinary CREATEs whose addresses nothing references. Here every create goes through the
    // factory, so every create needs a salt — including the implementations, whose addresses are
    // still referenced by nothing but must be predictable for step D to be assembled offline.

    string internal constant R_IMPL_EMPTY_ACL = "IMPL_EMPTY_UUPS_PROXY_ACL";
    string internal constant R_IMPL_EMPTY_SHARED = "IMPL_EMPTY_UUPS_PROXY";

    string internal constant R_ACL = "ACL_ADDRESS";
    string internal constant R_FHEVM_EXECUTOR = "FHEVM_EXECUTOR_ADDRESS";
    string internal constant R_KMS_VERIFIER = "KMS_VERIFIER_ADDRESS";
    string internal constant R_INPUT_VERIFIER = "INPUT_VERIFIER_ADDRESS";
    string internal constant R_HCU_LIMIT = "HCU_LIMIT_ADDRESS";
    string internal constant R_PROTOCOL_CONFIG = "PROTOCOL_CONFIG_ADDRESS";
    string internal constant R_KMS_GENERATION = "KMS_GENERATION_ADDRESS";
    string internal constant R_CLEARTEXT_ARITHMETIC = "CLEARTEXT_ARITHMETIC_ADDRESS";
    string internal constant R_CLEARTEXT_DB = "CLEARTEXT_DB_ADDRESS";

    string internal constant R_PAUSER_SET = "PAUSER_SET_ADDRESS";
    string internal constant R_ACL_OWNER = "ACL_OWNER";

    /// @dev The non-ACL proxies, in the order step D's ops array uses.
    function _sharedProxyRoles() internal pure returns (string[] memory r) {
        r = new string[](8);
        r[0] = R_FHEVM_EXECUTOR;
        r[1] = R_KMS_VERIFIER;
        r[2] = R_INPUT_VERIFIER;
        r[3] = R_HCU_LIMIT;
        r[4] = R_PROTOCOL_CONFIG;
        r[5] = R_KMS_GENERATION;
        r[6] = R_CLEARTEXT_ARITHMETIC;
        r[7] = R_CLEARTEXT_DB;
    }

    /// @dev Every proxy, ACL first. Same order as step D's ops array.
    function _allProxyRoles() internal pure returns (string[] memory r) {
        string[] memory shared = _sharedProxyRoles();
        r = new string[](shared.length + 1);
        r[0] = R_ACL;
        for (uint256 i = 0; i < shared.length; i++) {
            // A role left empty means _sharedProxyRoles' array size and its assignments disagree — the
            // count is a literal there because Solidity has no array-literal length. An empty role would
            // otherwise flow into salt derivation and produce a plausible-looking wrong address, so it is
            // checked here, where every consumer of the list passes through.
            require(bytes(shared[i]).length != 0, "FhevmCreate2Base: _sharedProxyRoles has an unset entry");
            r[i + 1] = shared[i];
        }
    }

    /// @dev The implementation roles, index-aligned with _allProxyRoles().
    ///      `role(impl_i) = "IMPL_" ++ role(proxy_i)`, so there is one naming rule, not two lists.
    function _implRole(string memory proxyRole) internal pure returns (string memory) {
        return string.concat("IMPL_", proxyRole);
    }

    /**
     * @dev Which EMPTY implementation proxy `i` points at before step D, index-aligned with
     *      _allProxyRoles(). ACL gets its own; the rest share one.
     *
     *      This is the "not yet materialized" state, and it is NOT `address(0)`. An ERC1967Proxy sets
     *      its implementation slot in the constructor — OpenZeppelin's `_setImplementation` even
     *      reverts when the implementation has no code — so the slot of a deployed proxy is never
     *      zero. Anything reading it as "empty means unmaterialized" can never see the run state.
     */
    function _emptyImplRoleFor(uint256 i) internal pure returns (string memory) {
        return i == 0 ? R_IMPL_EMPTY_ACL : R_IMPL_EMPTY_SHARED;
    }

    // ---------------------------------------------------------------------------------------
    // Artifact ids
    // ---------------------------------------------------------------------------------------
    //
    // Path-qualified on purpose. `vm.getCode("ERC1967Proxy.sol:ERC1967Proxy")` is AMBIGUOUS in this
    // project — OpenZeppelin ships a contract of the same name in a file of the same name, and forge
    // aborts on multiple matching artifacts. The nonce path never hits this because it uses
    // `new ERC1967Proxy(...)` and lets solc resolve the import.
    //
    // Paths are relative to the package root, which is where forge runs (foundry.toml lives there).

    string internal constant A_ERC1967_PROXY = "pkg/src/erc1967/ERC1967Proxy.sol:ERC1967Proxy";
    string internal constant A_EMPTY_ACL = "pkg/src/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol:EmptyUUPSProxyACL";
    string internal constant A_EMPTY_SHARED = "pkg/src/contracts/emptyProxy/EmptyUUPSProxy.sol:EmptyUUPSProxy";
    string internal constant A_PAUSER_SET = "pkg/src/contracts/immutable/PauserSet.sol:PauserSet";
    string internal constant A_ACL_OWNER = "pkg/src/upgrade/ACLOwner.sol:ACLOwner";

    /// @dev Implementation artifact for a proxy role, index-aligned with _allProxyRoles().
    ///      The cleartext build substitutes three of the stock implementations — same rule as
    ///      FhevmDeployScript._materialize.
    function _implArtifact(uint256 i) internal pure returns (string memory) {
        if (i == 0) return "pkg/src/contracts/ACL.sol:ACL";
        if (i == 1) return "pkg/src/cleartext/CleartextFHEVMExecutor.sol:CleartextFHEVMExecutor";
        if (i == 2) return "pkg/src/cleartext/CleartextKMSVerifier.sol:CleartextKMSVerifier";
        if (i == 3) return "pkg/src/cleartext/CleartextInputVerifier.sol:CleartextInputVerifier";
        if (i == 4) return "pkg/src/contracts/HCULimit.sol:HCULimit";
        if (i == 5) return "pkg/src/contracts/ProtocolConfig.sol:ProtocolConfig";
        if (i == 6) return "pkg/src/contracts/KMSGeneration.sol:KMSGeneration";
        if (i == 7) return "pkg/src/cleartext/CleartextArithmetic.sol:CleartextArithmetic";
        if (i == 8) return "pkg/src/cleartext/CleartextDB.sol:CleartextDB";
        revert("FhevmCreate2Base: implementation index out of range");
    }

    // ---------------------------------------------------------------------------------------
    // Config
    // ---------------------------------------------------------------------------------------

    struct Cfg {
        uint256 chainId;
        string version;
        string deploymentId;
        address deployer;
        address admin;
        address pauser0; // address(0) ⇒ step A' not configured
        uint256 confirmations;
        string outDir;
    }

    Cfg internal cfg;

    function _loadConfig() internal {
        cfg.chainId = block.chainid;
        cfg.version = vm.envString("FHEVM_VERSION");
        cfg.deploymentId = vm.envString("FHEVM_DEPLOYMENT_ID");
        cfg.deployer = vm.envAddress("FHEVM_DEPLOYER");
        cfg.admin = vm.envAddress("FHEVM_ADMIN");
        cfg.pauser0 = vm.envOr("FHEVM_PAUSER_0", address(0));
        cfg.confirmations = vm.envOr("FHEVM_CONFIRMATIONS", uint256(2));
        cfg.outDir = vm.envString("FHEVM_OUT_DIR");

        // E is required, not optional, and "admin == deployer" would make it a no-op that leaves
        // the deployer as root over the stack forever.
        require(cfg.admin != address(0), "FhevmCreate2Base: FHEVM_ADMIN is mandatory");
        require(cfg.admin != cfg.deployer, "FhevmCreate2Base: FHEVM_ADMIN must differ from the deployer");
        require(cfg.deployer != address(0), "FhevmCreate2Base: FHEVM_DEPLOYER is mandatory");
    }

    function _addressesPath() internal view returns (string memory) {
        return string.concat(cfg.outDir, "/addresses.sol");
    }

    function _scratchPath() internal view returns (string memory) {
        return string.concat(cfg.outDir, "/pass2.json");
    }

    function _manifestPath() internal view returns (string memory) {
        return string.concat(cfg.outDir, "/manifest.json");
    }

    // ---------------------------------------------------------------------------------------
    // Salts
    // ---------------------------------------------------------------------------------------

    /**
     * @dev salt = keccak256(abi.encode("fhevm.cleartext", MAJOR_MINOR, deploymentId, role))
     *
     *      `abi.encode`, not `abi.encodePacked`: with four dynamic strings, packed encoding lets
     *      ("ab","c") and ("a","bc") collide.
     *
     *      NOTE the deployer is NOT in the salt, and that is correct rather than an oversight.
     *      The canonical factory does not namespace by caller either — it passes the salt to CREATE2
     *      verbatim, so `msg.sender` enters the derivation nowhere. The deployer reaches the address
     *      set through exactly one channel: it is an initcode argument to the ACL proxy's
     *      `initialize`, and everything downstream bakes the resulting aclAdd. Two operators
     *      running this with the same version and deploymentId but different deployers get disjoint
     *      addresses; same deployer ⇒ identical addresses on every chain.
     */
    function _salt(string memory role) internal view returns (bytes32) {
        return keccak256(abi.encode("fhevm.cleartext", cfg.version, cfg.deploymentId, role));
    }

    // ---------------------------------------------------------------------------------------
    // Init code and prediction
    // ---------------------------------------------------------------------------------------

    /// @dev Creation code from the artifact, with ABI-encoded constructor args appended.
    ///      `vm.getCode` reads out/ — so it reflects the LAST build, which is exactly what makes the
    ///      three-pass pipeline expressible in one script run three times.
    function _initCode(string memory artifact, bytes memory args) internal view returns (bytes memory) {
        return bytes.concat(vm.getCode(artifact), args);
    }

    function _initCode(string memory artifact) internal view returns (bytes memory) {
        return vm.getCode(artifact);
    }

    /// @dev The ERC1967Proxy initcode for a proxy over `impl`, initialized with `initData`.
    function _proxyInitCode(address impl, bytes memory initData) internal view returns (bytes memory) {
        return _initCode(A_ERC1967_PROXY, abi.encode(impl, initData));
    }

    struct Create {
        string role;
        bytes initCode;
    }

    /**
     * @dev All creates, in the order the two hard edges require. Defined ONCE, here, because two
     *      scripts consume it — FhevmDeployCreates to send them and FhevmStatus to report on them —
     *      and a status board assembled from a second, drifting definition of "the work" would be
     *      worse than no status board.
     *
     *      ORDER. CREATE2 removes address fragility, not logical dependencies. Our ERC1967Proxy
     *      wraps OpenZeppelin's, whose constructor calls upgradeToAndCall → _setImplementation, which
     *      reverts ERC1967InvalidImplementation when the implementation has no code:
     *
     *          [0] impl₁ (EmptyUUPSProxyACL)  MUST precede  [1] the ACL proxy
     *          [2] impl₃ (EmptyUUPSProxy)     MUST precede  [3..10] the remaining proxies
     *
     *      Everything after that — PauserSet, ACLOwner, the implementations — is order-free.
     *      Under --broadcast the edges cost nothing: every create is a separate transaction from ONE
     *      sender, and transactions from one sender execute in nonce order. That is the only job the
     *      nonce still does on this path, and it constrains no address.
     *
     *      impl₁ and impl₃ are read from the MANIFEST rather than re-derived, so that a build whose
     *      impl₁ initcode drifted is flagged on the impl₁ entry alone instead of cascading into the
     *      ACL proxy's. Either way the run aborts; this just names the artifact that moved.
     */
    function _allCreates(string memory manifest) internal view returns (Create[] memory c) {
        address impl1 = _readManifestAddress(manifest, R_IMPL_EMPTY_ACL);
        address impl3 = _readManifestAddress(manifest, R_IMPL_EMPTY_SHARED);
        address aclAdd = _readManifestAddress(manifest, R_ACL);

        string[] memory shared = _sharedProxyRoles();
        string[] memory proxyRoles = _allProxyRoles();

        // They share ONE init-code hash — same implementation, same empty `initialize()` — and
        // are distinguished purely by salt. Built once, outside the loop.
        bytes memory sharedProxyCode = _proxyInitCode(impl3, _sharedProxyInitData());

        // 2 empty implementations + N proxies + PauserSet + ACLOwner + N implementations.
        c = new Create[](2 * _allProxyRoles().length + 4);
        uint256 n;

        c[n++] = Create(R_IMPL_EMPTY_ACL, _initCode(A_EMPTY_ACL));
        c[n++] = Create(R_ACL, _proxyInitCode(impl1, _aclProxyInitData(cfg.deployer)));
        c[n++] = Create(R_IMPL_EMPTY_SHARED, _initCode(A_EMPTY_SHARED));
        for (uint256 i = 0; i < shared.length; i++) {
            c[n++] = Create(shared[i], sharedProxyCode);
        }
        c[n++] = Create(R_PAUSER_SET, _initCode(A_PAUSER_SET));
        c[n++] = Create(R_ACL_OWNER, _initCode(A_ACL_OWNER, abi.encode(cfg.deployer, aclAdd)));
        for (uint256 i = 0; i < proxyRoles.length; i++) {
            c[n++] = Create(_implRole(proxyRoles[i]), _initCode(_implArtifact(i)));
        }

        require(n == c.length, "FhevmCreate2Base: create table length mismatch");
    }

    /**
     * @dev Where a role's contract WILL live: `keccak(0xff ++ factory ++ salt ++ keccak(initCode))`.
     *
     *      A pure function of (factory, salt, initcode) — no chain access, no deployer, no nonce.
     *      That is what makes it safe to call before anything is deployed, and it is why the whole
     *      seal-then-deploy shape works: predict everything, write the addresses into the contracts,
     *      rebuild, deploy to exactly what was predicted.
     *
     *      Contrast `vm.computeCreateAddress(deployer, nonce)` on the nonce path, which needs the
     *      deployer's live nonce and is invalidated by any transaction that moves it.
     */
    function _predictCreate2Address(string memory role, bytes memory initCode) internal view returns (address) {
        return vm.computeCreate2Address(_salt(role), keccak256(initCode), CREATE2_FACTORY);
    }

    // ---------------------------------------------------------------------------------------
    // Initializer calldata
    // ---------------------------------------------------------------------------------------
    //
    // Encoded by signature rather than `abi.encodeCall`, so this draft compiles against forge-std
    // alone. The production script should import the interfaces under
    // pkg/forge/src/_internal/interfaces/ and use `abi.encodeCall`: these two selectors are load-
    // bearing (they are inside the initcode, therefore inside the address), and a typo here produces
    // a wrong address set that every later check happily agrees with.

    function _aclProxyInitData(address deployer) internal pure returns (bytes memory) {
        // The DEPLOYER, not the admin. `PauserSet.addPauser` is `onlyACLOwner`, so step A is
        // only sendable by whoever this names — and a multisig admin cannot sign mid-run.
        return abi.encodeWithSignature("initialize(address)", deployer);
    }

    function _sharedProxyInitData() internal pure returns (bytes memory) {
        return abi.encodeWithSignature("initialize()");
    }

    // ---------------------------------------------------------------------------------------
    // The factory call
    // ---------------------------------------------------------------------------------------

    /**
     * @dev One CREATE2 through the factory. Must be called inside a broadcast.
     *
     *      The return data is deliberately discarded: nothing may parse the factory's return
     *      data." The caller verifies by checking code at the predicted address, which is the same
     *      check `verify` and a resumed run perform, so there is one verification path rather than
     *      two that can disagree.
     *
     *      A raw call rather than `new C{salt: s}(args)` because the initcode here comes from
     *      `vm.getCode`, not from a type. That is what lets one loop handle all creates.
     */
    function _factoryCreate2(bytes32 salt, bytes memory initCode) internal {
        require(initCode.length > 0, "FhevmCreate2Base: empty initcode (artifact not built?)");
        require(initCode.length <= MAX_INITCODE_SIZE, "FhevmCreate2Base: initcode exceeds EIP-3860 limit");
        (bool ok, ) = CREATE2_FACTORY.call(bytes.concat(salt, initCode));
        require(ok, "FhevmCreate2Base: factory call reverted");
    }

    // ---------------------------------------------------------------------------------------
    // Predicates
    // ---------------------------------------------------------------------------------------

    function _deployed(address a) internal view returns (bool) {
        return a.code.length != 0;
    }

    /// @dev The ERC-1967 implementation slot of a proxy, or address(0) if still an empty proxy.
    function _implementationOf(address proxy) internal view returns (address) {
        return address(uint160(uint256(vm.load(proxy, ERC1967_IMPL_SLOT))));
    }

    // ---------------------------------------------------------------------------------------
    // The reorg gate
    // ---------------------------------------------------------------------------------------

    /**
     * @dev Refuse to run until the chain has reached `FHEVM_MIN_BLOCK`.
     *
     *      Every step A–E reads chain state to decide what to do, and every one of those reads is
     *      about a transaction some EARLIER step sent. Sepolia reorgs. A predicate evaluated
     *      one block after the transaction it is asking about can be answering from a block that is
     *      about to be orphaned — and this path's predicates are not merely informational, they
     *      decide whether a step is skipped. A reorged-away `addPauser` that the predicate reported
     *      as done is a stack that reaches the terminal conditions with no pauser.
     *
     *      The orchestrator sets this to (block of the previous step's last transaction) +
     *      FHEVM_CONFIRMATIONS. Enforcing it HERE rather than only in the shell is the same argument
     *      as every other gate in this path: the shell is one orchestrator, a TS driver will be
     *      another, and an operator running a single stage by hand is a third. A `sleep` in one of
     *      them binds only that one.
     *
     *      REQUIRED, with no default. `0` is a legitimate value — it is what the first stage of a run
     *      passes — but it has to be passed, so that skipping the wait is a decision someone made
     *      rather than an environment variable someone forgot.
     *
     *      This is a simulation-time check, and that is NOT a weakness worth hedging about: the two
     *      risks are not independent. For a transaction sent by this run to land on a chain where the
     *      predicate it was based on is false, that chain must have reorged out a block that was
     *      already `FHEVM_CONFIRMATIONS` deep when it was read — and one deeper still by inclusion.
     *      The inclusion-side risk is strictly dominated by the read-side risk this gate covers. What
     *      matters is the DEPTH, not when it is evaluated.
     *
     *      What the depth is worth, though, is a real question and the orchestrator's to answer.
     *      Fifteen blocks is ~3 minutes at 12s slots; PoS finality is two epochs, 64 slots, ~12.8
     *      minutes. A depth heuristic is a quarter of the way there. On the TESTNETS this path is
     *      restricted to, that gap is not academic — Holesky went weeks without finalizing in
     *      February 2024. So `deploy-testnet.sh` additionally waits for the `finalized` tag and only
     *      falls back to depth on a chain that does not serve it; see its reorg-gate section. A
     *      Solidity script cannot read `finalized` through `block.number`, which is why the
     *      finality wait lives there and the depth floor lives here.
     */
    function _requireMinBlock() internal view {
        uint256 minBlock;
        try vm.envUint("FHEVM_MIN_BLOCK") returns (uint256 v) {
            minBlock = v;
        } catch {
            revert("FhevmCreate2Base: FHEVM_MIN_BLOCK is required (pass 0 for the first stage of a run)");
        }

        if (block.number < minBlock) {
            console.log("  current block ", block.number);
            console.log("  required block", minBlock);
            console.log("  blocks to wait", minBlock - block.number);
            revert("FhevmCreate2Base: chain has not reached FHEVM_MIN_BLOCK - the previous step is not buried yet");
        }
    }

    // ---------------------------------------------------------------------------------------
    // Scratch + manifest JSON
    // ---------------------------------------------------------------------------------------
    //
    // Shape (three maps keyed by role, rather than one map of objects — forge's serializeJson nests
    // awkwardly, and parallel maps read back with a single string.concat):
    //
    //   { "chainId": .., "version": "..", "deploymentId": "..", "deployer": "0x..",
    //     "admin": "0x..", "factory": "0x4e59..", "confirmations": ..,
    //     "roles":        ["ACL_ADDRESS", ...],
    //     "salt":         { "ACL_ADDRESS": "0x.." },
    //     "initCodeHash": { "ACL_ADDRESS": "0x.." },
    //     "address":      { "ACL_ADDRESS": "0x.." } }
    //
    // This is NOT resume state. Resume is `getCode(addr) != ""` against the chain. The file
    // exists because the addresses are a function of the init-code hashes, so a retry of a
    // failed create needs the byte-exact hash that produced the address — and because a run that
    // starts at step D needs to know which implementations "sealed" means.

    function _readManifestAddress(string memory json, string memory role) internal pure returns (address) {
        return vm.parseJsonAddress(json, string.concat(".address.", role));
    }

    function _readManifestInitCodeHash(string memory json, string memory role) internal pure returns (bytes32) {
        return vm.parseJsonBytes32(json, string.concat(".initCodeHash.", role));
    }

    function _loadManifest() internal view returns (string memory) {
        string memory path = _manifestPath();
        require(vm.exists(path), string.concat("FhevmCreate2Base: no manifest at ", path, " - run the compute stage"));
        return vm.readFile(path);
    }

    // ---------------------------------------------------------------------------------------
    // Logging
    // ---------------------------------------------------------------------------------------

    function _logRole(string memory role, address predicted, bool done) internal pure {
        console.log(string.concat(done ? "  [have] " : "  [ new] ", role), predicted);
    }

    function _banner(string memory title) internal view {
        console.log("");
        console.log(string.concat("=== ", title, " ==="));
        console.log("  chain        ", cfg.chainId);
        console.log("  deployer     ", cfg.deployer);
        console.log("  admin        ", cfg.admin);
        console.log(string.concat("  deploymentId  ", cfg.deploymentId, " @ v", cfg.version));
        console.log("");
    }
}
