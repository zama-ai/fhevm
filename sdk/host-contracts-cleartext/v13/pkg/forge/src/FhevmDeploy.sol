// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Vm.sol";

import {
    ACL_ADDRESS,
    CLEARTEXT_ARITHMETIC_ADDRESS,
    CLEARTEXT_DB_ADDRESS,
    DEPLOYER_ADDRESS,
    DEPLOYER_START_NONCE,
    FHEVM_EXECUTOR_ADDRESS,
    HCU_LIMIT_ADDRESS,
    INPUT_VERIFIER_ADDRESS,
    KMS_GENERATION_ADDRESS,
    KMS_VERIFIER_ADDRESS,
    MNEMONIC,
    PAUSER_SET_ADDRESS,
    PROTOCOL_CONFIG_ADDRESS
} from "./_internal/LocalHostAddresses.sol";

import {
    ACL_CREATION_CODE,
    ACL_OWNER_CREATION_CODE,
    CLEARTEXT_ARITHMETIC_CREATION_CODE,
    CLEARTEXT_DB_CREATION_CODE,
    CLEARTEXT_FHEVM_EXECUTOR_CREATION_CODE,
    CLEARTEXT_INPUT_VERIFIER_CREATION_CODE,
    CLEARTEXT_KMS_VERIFIER_CREATION_CODE,
    EMPTY_UUPS_PROXY_ACL_CREATION_CODE,
    EMPTY_UUPS_PROXY_CREATION_CODE,
    ERC1967_PROXY_CREATION_CODE,
    HCU_LIMIT_CREATION_CODE,
    KMS_GENERATION_CREATION_CODE,
    PAUSER_SET_RUNTIME_CODE,
    PROTOCOL_CONFIG_CREATION_CODE
} from "./_internal/LocalHostBytecode.sol";

import {LocalHostBootstrap} from "./_internal/LocalHostBootstrap.sol";

import {IACL} from "./_internal/interfaces/IACL.sol";
import {ACLOwner, IACLOwner} from "./_internal/interfaces/IACLOwner.sol";
import {ICleartextArithmetic} from "./_internal/interfaces/ICleartextArithmetic.sol";
import {ICleartextDB} from "./_internal/interfaces/ICleartextDB.sol";
import {ICleartextFHEVMExecutor} from "./_internal/interfaces/ICleartextFHEVMExecutor.sol";
import {ICleartextInputVerifier} from "./_internal/interfaces/ICleartextInputVerifier.sol";
import {ICleartextKMSVerifier} from "./_internal/interfaces/ICleartextKMSVerifier.sol";
import {IEmptyUUPSProxy} from "./_internal/interfaces/IEmptyUUPSProxy.sol";
import {IEmptyUUPSProxyACL} from "./_internal/interfaces/IEmptyUUPSProxyACL.sol";
import {IHCULimit} from "./_internal/interfaces/IHCULimit.sol";
import {IKMSGeneration} from "./_internal/interfaces/IKMSGeneration.sol";
import {IPauserSet} from "./_internal/interfaces/IPauserSet.sol";
import {IProtocolConfig} from "./_internal/interfaces/IProtocolConfig.sol";

/**
 * @title  FhevmDeploy
 * @notice Stands up a cleartext FHEVM stack at the canonical localhost addresses.
 * @dev
 * Inherit and call `deployFhevm()`. It is idempotent, so calling it from `setUp()` in a base contract and
 * again from a derived one is safe.
 *
 * ## Why the sequence looks the way it does
 *
 * This is a transcription of the TypeScript `deploy()` in `ts/deploy.ts`, not a shortcut around it. Every
 * address is `CREATE(deployer, nonce)`, so the stack only lands on the addresses in
 * `LocalHostAddresses.sol` — the ones `ZamaConfig.sol` compiles into consumers — if these transactions
 * happen in exactly this order from exactly that account starting at exactly that nonce. Deploying
 * "equivalently" in a different order produces a stack whose bytecode points at the wrong places, and the
 * bytecode here is pre-compiled against those addresses so it cannot adapt.
 *
 * Every creation is therefore checked against its expected address rather than trusted, and every member
 * except `deployFhevm()` is `private` — including the bootstrap arguments. That is deliberate rather than
 * restrictive: the signer sets in particular are the contract with the js-sdk cleartext relayer, which
 * derives its keys from FHEVM_MNEMONIC at fixed HD paths and looks them up by the address the chain
 * reports. A configurable stack is a stack that can be configured into one the SDK holds no key for, so
 * there is exactly one stack this contract can produce, and it is the one everything else expects.
 *
 * To deploy something else, change `LocalHostBootstrap` (generated) and regenerate — not this file.
 *
 * ## What a consumer may import
 *
 * Everything generated lives under `_internal/` and is not part of the API. Import from *this file*
 * instead — Solidity re-exports imported symbols, so the interfaces and address constants this contract
 * already pulls in are reachable through it:
 *
 * ```solidity
 * import {FhevmDeploy, IACL, ACL_ADDRESS} from "host-contracts-cleartext-forge/FhevmDeploy.sol";
 *
 * contract MyTest is Test, FhevmDeploy {
 *     function setUp() public { deployFhevm(); }
 *     function test_x() public { IACL(ACL_ADDRESS).isAllowed(handle, user); }
 * }
 * ```
 *
 * Reaching into `_internal/` works — Solidity has no directory visibility — but nothing there is stable,
 * and `LocalHostBytecode.sol` in particular is a trap: those blobs are pre-compiled against the canonical
 * addresses, so deploying them by hand bypasses the nonce-ordering guards and yields a stack whose
 * bytecode points at addresses nothing lives at.
 *
 * The five phases, mirroring `ts/deploy.ts`:
 *   1. the nine empty proxies (nonces 0-10), each an ERC-1967 proxy over an empty UUPS implementation
 *   2. PauserSet (nonce 11)
 *   3. ACLOwner, registered as a pauser and handed ACL ownership two-step
 *   4. the nine real implementations, deployed permissionlessly
 *   5. one atomic `ACLOwner.upgrade` swapping every proxy empty -> real and running its initializer
 *
 * All five are `private`, as is everything else here bar the entry point.
 */
abstract contract FhevmDeploy {
    /// @dev `Vm`, not `VmSafe`: standing the stack up needs the mutating cheatcodes (prank, etch, setNonce).
    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @dev ERC-1967 implementation slot: keccak256("eip1967.proxy.implementation") - 1.
    bytes32 private constant _ERC1967_IMPL_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

    /// @dev Set once `deployFhevm()` has run, so it is safe to call from several `setUp()` bodies.
    bool private _fhevmDeployed;

    /// @dev The standing ACLOwner, owner of ACL once phase 3 completes. Read it through `fhevmACLOwner()`.
    address private _fhevmACLOwner;
    // Deliberately runtime state rather than a generated constant. On a fresh deploy it happens to be
    // CREATE(deployer, 12), but the ACLOwner *survives an upgrade*: `updateV12ToV13` takes the standing
    // one as a parameter and deploys none, so on an upgraded stack it came from the previous generation's
    // deploy at an unrelated nonce. A constant would be right here and wrong there.

    /// @dev Account that owns the ACLOwner and may therefore upgrade the stack.
    function _fhevmAdmin() private pure returns (address) {
        return DEPLOYER_ADDRESS;
    }

    /**
     * @dev Bootstrap arguments for the initializers that take them, mirroring
     *      `DEFAUT_BOOTSTRAP_CONFIG_V13` via the generated `LocalHostBootstrap`.
     */
    function _fhevmKmsVerifierConfig() private pure returns (address verifyingContract, uint64 chainId) {
        return (LocalHostBootstrap.DECRYPTION_ADDRESS, LocalHostBootstrap.GATEWAY_CHAIN_ID);
    }

    function _fhevmInputVerifierConfig()
        private
        pure
        returns (address verifyingContract, uint64 chainId, address[] memory signers, uint256 threshold)
    {
        return (
            LocalHostBootstrap.INPUT_VERIFICATION_ADDRESS,
            LocalHostBootstrap.GATEWAY_CHAIN_ID,
            LocalHostBootstrap.coprocessorSigners(),
            LocalHostBootstrap.COPROCESSOR_THRESHOLD
        );
    }

    function _fhevmHcuLimitConfig()
        private
        pure
        returns (uint48 capPerBlock, uint48 maxDepthPerTx, uint48 maxPerTx)
    {
        return (
            LocalHostBootstrap.HCU_CAP_PER_BLOCK,
            LocalHostBootstrap.MAX_HCU_DEPTH_PER_TX,
            LocalHostBootstrap.MAX_HCU_PER_TX
        );
    }

    /// @dev The initial KMS context seeded into ProtocolConfig.
    function _fhevmProtocolConfig()
        private
        pure
        returns (IProtocolConfig.KmsNode[] memory nodes, IProtocolConfig.KmsThresholds memory thresholds)
    {
        address[] memory signers = LocalHostBootstrap.kmsSigners();
        address[] memory txSenders = LocalHostBootstrap.kmsTxSenders();
        string[] memory ips = LocalHostBootstrap.kmsIpAddresses();
        string[] memory urls = LocalHostBootstrap.kmsStorageUrls();

        nodes = new IProtocolConfig.KmsNode[](LocalHostBootstrap.KMS_NODE_COUNT);
        for (uint256 i = 0; i < nodes.length; i++) {
            nodes[i] = IProtocolConfig.KmsNode({
                txSenderAddress: txSenders[i],
                signerAddress: signers[i],
                ipAddress: ips[i],
                storageUrl: urls[i]
            });
        }

        // Every threshold is the node count, as ts/constants.ts DEFAULT_KMS_THRESHOLDS has it.
        uint256 count = LocalHostBootstrap.KMS_NODE_COUNT;
        thresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: count,
            userDecryption: count,
            kmsGen: count,
            mpc: count
        });
    }

    ////////////////////////////////////////////////////////////////////////////
    // Entry point
    ////////////////////////////////////////////////////////////////////////////

    /**
     * @notice The mnemonic the stack is **deployed** from — the deployer and admin accounts.
     * @dev Not the signer mnemonic. The KMS and coprocessor signing keys the js-sdk relayer uses come
     *      from FHEVM_MNEMONIC ("test test test ...") at m/44'/60'/0'/{2,3,4}/i; this one only funds and
     *      signs the deploy. Two mnemonics, two jobs — swapping them gives a stack whose addresses look
     *      right and whose signatures never verify.
     */
    function fhevmMnemonic() internal pure returns (string memory) {
        return MNEMONIC;
    }

    /**
     * @notice The standing ACLOwner: owner of ACL, and the only account that can upgrade the stack.
     * @dev Exposed because it is the one thing a caller needs that is neither a constant nor derivable:
     *      an upgrade tool built on top of this has to address the ACLOwner established by the deploy,
     *      and the same contract carries across generations. `ACL.owner()` answers the same question on
     *      chain, but only after the fact and only if ownership has not since moved.
     *
     *      Reverts rather than returning zero before `deployFhevm()` has run — a zero here would other-
     *      wise surface much later as a call into an empty address.
     */
    function fhevmACLOwner() internal view returns (address) {
        require(_fhevmACLOwner != address(0), "FhevmDeploy: call deployFhevm() before fhevmACLOwner()");
        return _fhevmACLOwner;
    }

    function deployFhevm() internal virtual {
        if (_fhevmDeployed) {
            return;
        }
        _fhevmDeployed = true;

        require(
            vm.getNonce(DEPLOYER_ADDRESS) == DEPLOYER_START_NONCE,
            "FhevmDeploy: deployer nonce must be DEPLOYER_START_NONCE; every address derives from it"
        );

        vm.startPrank(DEPLOYER_ADDRESS);
        _deployEmptyProxies();
        _deployPauserSet();
        vm.stopPrank();

        _setupACLOwner();
        address[] memory implementations = _deployImplementations();
        _materialize(implementations);
    }

    ////////////////////////////////////////////////////////////////////////////
    // Phase 1 — the empty proxies (nonces 0-10)
    ////////////////////////////////////////////////////////////////////////////

    /**
     * @dev Sealed: these eleven creations are what place the stack at the canonical addresses, so there is
     *      no override of this that keeps the guarantee. Change the address set instead, by regenerating
     *      LocalHostAddresses.sol and LocalHostBytecode.sol together.
     *
     *      ACL is special: its proxy sits over `EmptyUUPSProxyACL`, whose `initialize` takes the initial
     *      owner. Every other proxy shares one `EmptyUUPSProxy` implementation, deployed once at nonce 2.
     */
    function _deployEmptyProxies() private {
        address emptyACLImpl = _create(EMPTY_UUPS_PROXY_ACL_CREATION_CODE, "EmptyUUPSProxyACL");
        _createProxy(
            emptyACLImpl,
            abi.encodeCall(IEmptyUUPSProxyACL.initialize, (DEPLOYER_ADDRESS)),
            ACL_ADDRESS,
            "ACL proxy"
        );

        address emptyImpl = _create(EMPTY_UUPS_PROXY_CREATION_CODE, "EmptyUUPSProxy");
        bytes memory initEmpty = abi.encodeCall(IEmptyUUPSProxy.initialize, ());

        _createProxy(emptyImpl, initEmpty, FHEVM_EXECUTOR_ADDRESS, "FHEVMExecutor proxy");
        _createProxy(emptyImpl, initEmpty, KMS_VERIFIER_ADDRESS, "KMSVerifier proxy");
        _createProxy(emptyImpl, initEmpty, INPUT_VERIFIER_ADDRESS, "InputVerifier proxy");
        _createProxy(emptyImpl, initEmpty, HCU_LIMIT_ADDRESS, "HCULimit proxy");
        _createProxy(emptyImpl, initEmpty, PROTOCOL_CONFIG_ADDRESS, "ProtocolConfig proxy");
        _createProxy(emptyImpl, initEmpty, KMS_GENERATION_ADDRESS, "KMSGeneration proxy");
        _createProxy(emptyImpl, initEmpty, CLEARTEXT_ARITHMETIC_ADDRESS, "CleartextArithmetic proxy");
        _createProxy(emptyImpl, initEmpty, CLEARTEXT_DB_ADDRESS, "CleartextDB proxy");
    }

    ////////////////////////////////////////////////////////////////////////////
    // Phase 2 — PauserSet (nonce 11)
    ////////////////////////////////////////////////////////////////////////////

    /**
     * @dev PauserSet has no constructor and no immutables, so its runtime blob is complete and etching it
     *      is equivalent to constructing it. The nonce is still consumed so the address matches: `vm.etch`
     *      does not advance it, hence the explicit bump.
     */
    function _deployPauserSet() private {
        vm.etch(PAUSER_SET_ADDRESS, PAUSER_SET_RUNTIME_CODE);
        vm.setNonce(DEPLOYER_ADDRESS, uint64(vm.getNonce(DEPLOYER_ADDRESS) + 1));
    }

    ////////////////////////////////////////////////////////////////////////////
    // Phase 3 — ACLOwner takes ownership of ACL
    ////////////////////////////////////////////////////////////////////////////

    /**
     * @dev Order is load-bearing: `PauserSet.addPauser` is `onlyACLOwner`, so ACLOwner must be registered
     *      as a pauser while the deployer still owns ACL. Ownership then moves two-step — the deployer
     *      offers, and the admin accepts through ACLOwner.
     */
    function _setupACLOwner() private {
        vm.startPrank(DEPLOYER_ADDRESS);
        _fhevmACLOwner =
            _create(abi.encodePacked(ACL_OWNER_CREATION_CODE, abi.encode(_fhevmAdmin(), ACL_ADDRESS)), "ACLOwner");
        IPauserSet(PAUSER_SET_ADDRESS).addPauser(_fhevmACLOwner);
        IACL(ACL_ADDRESS).transferOwnership(_fhevmACLOwner);
        vm.stopPrank();

        vm.prank(_fhevmAdmin());
        IACLOwner(_fhevmACLOwner).acceptACLOwnership();
    }

    ////////////////////////////////////////////////////////////////////////////
    // Phase 4 — the real implementations
    ////////////////////////////////////////////////////////////////////////////

    /// @dev Permissionless, and their addresses are never referenced, so no determinism is required here.
    function _deployImplementations() private returns (address[] memory implementations) {
        implementations = new address[](9);
        implementations[0] = _create(ACL_CREATION_CODE, "ACL impl");
        implementations[1] = _create(CLEARTEXT_FHEVM_EXECUTOR_CREATION_CODE, "FHEVMExecutor impl");
        implementations[2] = _create(CLEARTEXT_KMS_VERIFIER_CREATION_CODE, "KMSVerifier impl");
        implementations[3] = _create(CLEARTEXT_INPUT_VERIFIER_CREATION_CODE, "InputVerifier impl");
        implementations[4] = _create(HCU_LIMIT_CREATION_CODE, "HCULimit impl");
        implementations[5] = _create(PROTOCOL_CONFIG_CREATION_CODE, "ProtocolConfig impl");
        implementations[6] = _create(KMS_GENERATION_CREATION_CODE, "KMSGeneration impl");
        implementations[7] = _create(CLEARTEXT_ARITHMETIC_CREATION_CODE, "CleartextArithmetic impl");
        implementations[8] = _create(CLEARTEXT_DB_CREATION_CODE, "CleartextDB impl");
    }

    ////////////////////////////////////////////////////////////////////////////
    // Phase 5 — one atomic upgrade
    ////////////////////////////////////////////////////////////////////////////

    /**
     * @dev A single `ACLOwner.upgrade` so the stack is never half-materialized: each op swaps a proxy from
     *      the empty implementation to the real one and runs its initializer in the same call.
     */
    function _materialize(address[] memory implementations) private {
        (address kmsVerifyingContract, uint64 kmsChainId) = _fhevmKmsVerifierConfig();
        (address inputVerifyingContract, uint64 inputChainId, address[] memory signers, uint256 threshold) =
            _fhevmInputVerifierConfig();
        (uint48 capPerBlock, uint48 maxDepthPerTx, uint48 maxPerTx) = _fhevmHcuLimitConfig();

        ACLOwner.Op[] memory ops = new ACLOwner.Op[](9);
        ops[0] = ACLOwner.Op(ACL_ADDRESS, implementations[0], abi.encodeCall(IACL.initializeFromEmptyProxy, ()));
        ops[1] = ACLOwner.Op(
            FHEVM_EXECUTOR_ADDRESS,
            implementations[1],
            abi.encodeCall(ICleartextFHEVMExecutor.initializeFromEmptyProxy, ())
        );
        ops[2] = ACLOwner.Op(
            KMS_VERIFIER_ADDRESS,
            implementations[2],
            abi.encodeCall(ICleartextKMSVerifier.initializeFromEmptyProxy, (kmsVerifyingContract, kmsChainId))
        );
        ops[3] = ACLOwner.Op(
            INPUT_VERIFIER_ADDRESS,
            implementations[3],
            abi.encodeCall(
                ICleartextInputVerifier.initializeFromEmptyProxy,
                (inputVerifyingContract, inputChainId, signers, threshold)
            )
        );
        ops[4] = ACLOwner.Op(
            HCU_LIMIT_ADDRESS,
            implementations[4],
            abi.encodeCall(IHCULimit.initializeFromEmptyProxy, (capPerBlock, maxDepthPerTx, maxPerTx))
        );
        ops[5] = ACLOwner.Op(PROTOCOL_CONFIG_ADDRESS, implementations[5], _protocolConfigInitData());
        ops[6] = ACLOwner.Op(
            KMS_GENERATION_ADDRESS, implementations[6], abi.encodeCall(IKMSGeneration.initializeFromEmptyProxy, ())
        );
        ops[7] = ACLOwner.Op(
            CLEARTEXT_ARITHMETIC_ADDRESS,
            implementations[7],
            abi.encodeCall(ICleartextArithmetic.initializeFromEmptyProxy, ())
        );
        ops[8] = ACLOwner.Op(
            CLEARTEXT_DB_ADDRESS,
            implementations[8],
            abi.encodeCall(ICleartextDB.initializeFromEmptyProxy, (CLEARTEXT_ARITHMETIC_ADDRESS))
        );

        vm.prank(_fhevmAdmin());
        IACLOwner(_fhevmACLOwner).upgrade(ops);
    }

    /// @dev Encodes whatever `_fhevmProtocolConfig()` returns; the seed itself is the override point.
    function _protocolConfigInitData() private pure returns (bytes memory) {
        (IProtocolConfig.KmsNode[] memory nodes, IProtocolConfig.KmsThresholds memory thresholds) =
            _fhevmProtocolConfig();
        return abi.encodeCall(IProtocolConfig.initializeFromEmptyProxy, (nodes, thresholds));
    }

    ////////////////////////////////////////////////////////////////////////////
    // Primitives
    ////////////////////////////////////////////////////////////////////////////

    /// @dev Raw CREATE, because the bytecode arrives as bytes rather than as a compiled contract type.
    function _create(bytes memory creationCode, string memory what) private returns (address addr) {
        assembly {
            addr := create(0, add(creationCode, 0x20), mload(creationCode))
        }
        require(addr != address(0), string.concat("FhevmDeploy: failed to deploy ", what));
    }

    /**
     * @dev An ERC-1967 proxy over `implementation`, checked against the address the pre-compiled bytecode
     *      expects. A mismatch means the nonce sequence diverged, and every later address is wrong too —
     *      so fail here rather than let a subtly broken stack look deployed.
     */
    function _createProxy(address implementation, bytes memory initData, address expected, string memory what)
        private
        returns (address addr)
    {
        addr = _create(abi.encodePacked(ERC1967_PROXY_CREATION_CODE, abi.encode(implementation, initData)), what);
        require(addr == expected, string.concat("FhevmDeploy: ", what, " landed at the wrong address"));
        require(
            vm.load(addr, _ERC1967_IMPL_SLOT) == bytes32(uint256(uint160(implementation))),
            string.concat("FhevmDeploy: ", what, " implementation slot not set")
        );
    }
}
