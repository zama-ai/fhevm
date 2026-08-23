// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";

import {
    ACL_ADDRESS,
    CLEARTEXT_ARITHMETIC_ADDRESS,
    CLEARTEXT_DB_ADDRESS,
    DEPLOYER_ADDRESS,
    DEPLOYER_ADDRESS_INDEX,
    DEPLOYER_START_NONCE,
    FHEVM_EXECUTOR_ADDRESS,
    HCU_LIMIT_ADDRESS,
    INPUT_VERIFIER_ADDRESS,
    KMS_GENERATION_ADDRESS,
    KMS_VERIFIER_ADDRESS,
    MNEMONIC,
    PAUSER_SET_ADDRESS,
    PROTOCOL_CONFIG_ADDRESS
} from "../src/_internal/LocalHostAddresses.sol";

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
} from "../src/_internal/LocalHostBytecode.sol";

import {LocalHostBootstrap} from "../src/_internal/LocalHostBootstrap.sol";

import {IACL} from "../src/_internal/interfaces/IACL.sol";
import {ACLOwner, IACLOwner} from "../src/_internal/interfaces/IACLOwner.sol";
import {ICleartextArithmetic} from "../src/_internal/interfaces/ICleartextArithmetic.sol";
import {ICleartextDB} from "../src/_internal/interfaces/ICleartextDB.sol";
import {ICleartextFHEVMExecutor} from "../src/_internal/interfaces/ICleartextFHEVMExecutor.sol";
import {ICleartextInputVerifier} from "../src/_internal/interfaces/ICleartextInputVerifier.sol";
import {ICleartextKMSVerifier} from "../src/_internal/interfaces/ICleartextKMSVerifier.sol";
import {IEmptyUUPSProxy} from "../src/_internal/interfaces/IEmptyUUPSProxy.sol";
import {IEmptyUUPSProxyACL} from "../src/_internal/interfaces/IEmptyUUPSProxyACL.sol";
import {IHCULimit} from "../src/_internal/interfaces/IHCULimit.sol";
import {IKMSGeneration} from "../src/_internal/interfaces/IKMSGeneration.sol";
import {IPauserSet} from "../src/_internal/interfaces/IPauserSet.sol";
import {IProtocolConfig} from "../src/_internal/interfaces/IProtocolConfig.sol";

/**
 * @title DeployLocalStack
 * @notice Deploys the canonical local cleartext stack from the PRE-COMPILED blobs, onto a live node.
 *
 * The broadcast twin of `pkg/forge/src/FhevmDeploy.sol`: same phases, same order, same blobs from
 * `_internal/LocalHostBytecode.sol`. The difference is only that `FhevmDeploy` runs inside a forge test
 * with cheatcodes, and this sends real transactions.
 *
 * Why it is faster than scripts/deploy.sh: the addresses are already compiled into these blobs, so there
 * is no "compute addresses, then compile pkg/src against them" round. Nothing in this script's import
 * graph reaches pkg/src except the one shared FheType enum the generated interfaces need.
 *
 * Driven by scripts/anvil-local.sh. Requires DEPLOYER_PRIVATE_KEY, or derives it from the package
 * mnemonic at DEPLOYER_ADDRESS_INDEX.
 *
 * ## Why PauserSet is CREATEd here rather than `anvil_setCode`d
 *
 * `FhevmDeploy` installs PauserSet with `vm.etch` and then bumps the nonce by hand, because etch places
 * code without consuming one. The obvious translation — `anvil_setCode` plus `anvil_setNonce` over RPC —
 * does not work under `--broadcast`, and it fails quietly:
 *
 *   `forge script` simulates locally and derives each CREATE address from its own view of the nonce.
 *   An `anvil_setNonce` changes the node, not the simulation. Measured: after setting the nonce to 5 by
 *   RPC, `vm.getNonce` still reported 1 and the next CREATE landed at nonce 1.
 *
 * If PauserSet did not consume nonce 11, ACLOwner would take it and land on PauserSet's address. So the
 * nonce has to be consumed by a real creation. Only PauserSet's *runtime* blob is shipped (it has no
 * constructor, so `CODE_KIND` classifies it as runtime), and `_asCreationCode` below wraps that runtime in
 * a 12-byte prelude that returns it — a real CREATE whose result is byte-for-byte the shipped blob.
 *
 * That also makes this script node-agnostic: no anvil-only RPC anywhere, so it works against any chain.
 */
contract DeployLocalStack is Script {
    /// @dev ERC-1967 implementation slot: keccak256("eip1967.proxy.implementation") - 1.
    bytes32 private constant _ERC1967_IMPL_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

    function run() external {
        uint256 deployerKey = vm.envOr("DEPLOYER_PRIVATE_KEY", uint256(0));
        if (deployerKey == 0) {
            deployerKey = vm.deriveKey(MNEMONIC, DEPLOYER_ADDRESS_INDEX);
        }
        address deployer = vm.addr(deployerKey);

        require(
            deployer == DEPLOYER_ADDRESS,
            "DeployLocalStack: DEPLOYER_PRIVATE_KEY is not the canonical deployer these blobs were built for"
        );
        require(
            vm.getNonce(deployer) == DEPLOYER_START_NONCE,
            "DeployLocalStack: deployer nonce must be 0; every address in the blobs derives from it"
        );
        require(ACL_ADDRESS.code.length == 0, "DeployLocalStack: ACL already has code; the chain is not fresh");

        vm.startBroadcast(deployerKey);
        _deployEmptyProxies(deployer);
        _deployPauserSet();
        address aclOwner = _setupACLOwner(deployer);
        _materialize(aclOwner);
        vm.stopBroadcast();

        console.log("ACL:                 ", ACL_ADDRESS);
        console.log("FHEVMExecutor:       ", FHEVM_EXECUTOR_ADDRESS);
        console.log("KMSVerifier:         ", KMS_VERIFIER_ADDRESS);
        console.log("InputVerifier:       ", INPUT_VERIFIER_ADDRESS);
        console.log("HCULimit:            ", HCU_LIMIT_ADDRESS);
        console.log("ProtocolConfig:      ", PROTOCOL_CONFIG_ADDRESS);
        console.log("KMSGeneration:       ", KMS_GENERATION_ADDRESS);
        console.log("CleartextArithmetic: ", CLEARTEXT_ARITHMETIC_ADDRESS);
        console.log("CleartextDB:         ", CLEARTEXT_DB_ADDRESS);
        console.log("PauserSet:           ", PAUSER_SET_ADDRESS);
        console.log("ACLOwner:            ", aclOwner);
    }

    ////////////////////////////////////////////////////////////////////////////
    // Phase 1 — the empty proxies (nonces 0-10)
    ////////////////////////////////////////////////////////////////////////////

    function _deployEmptyProxies(address deployer) private {
        address emptyAclImpl = _create(EMPTY_UUPS_PROXY_ACL_CREATION_CODE, "EmptyUUPSProxyACL");
        _createProxy(emptyAclImpl, abi.encodeCall(IEmptyUUPSProxyACL.initialize, (deployer)), ACL_ADDRESS, "ACL proxy");

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

    function _deployPauserSet() private {
        address deployed = _create(_asCreationCode(PAUSER_SET_RUNTIME_CODE), "PauserSet");
        require(deployed == PAUSER_SET_ADDRESS, "DeployLocalStack: PauserSet landed at the wrong address");
        require(
            keccak256(deployed.code) == keccak256(PAUSER_SET_RUNTIME_CODE),
            "DeployLocalStack: deployed PauserSet code differs from the shipped runtime blob"
        );
    }

    /**
     * @dev Wraps runtime code in the minimal init code that returns it, so a real CREATE consumes a nonce
     *      and deposits exactly `runtime`. Twelve bytes of prelude:
     *
     *        61 LLLL  PUSH2 len
     *        80       DUP1
     *        60 0c    PUSH1 12        (offset of the payload within this init code)
     *        60 00    PUSH1 0         (memory destination)
     *        39       CODECOPY
     *        60 00    PUSH1 0
     *        f3       RETURN
     *
     *      PUSH2 because PauserSet's runtime is 2330 bytes, well past what PUSH1 can express.
     */
    function _asCreationCode(bytes memory runtime) private pure returns (bytes memory) {
        require(runtime.length <= type(uint16).max, "DeployLocalStack: runtime too large for the prelude");
        return abi.encodePacked(bytes1(0x61), uint16(runtime.length), hex"80600c6000396000f3", runtime);
    }

    ////////////////////////////////////////////////////////////////////////////
    // Phase 3 — ACLOwner takes ownership of ACL (nonce 12, then calls)
    ////////////////////////////////////////////////////////////////////////////

    /**
     * @dev Order is load-bearing: `PauserSet.addPauser` is `onlyACLOwner`, so the ACLOwner has to be
     *      registered while `deployer` still owns ACL. Ownership then moves two-step.
     */
    function _setupACLOwner(address deployer) private returns (address aclOwner) {
        aclOwner = _create(abi.encodePacked(ACL_OWNER_CREATION_CODE, abi.encode(deployer, ACL_ADDRESS)), "ACLOwner");
        IPauserSet(PAUSER_SET_ADDRESS).addPauser(aclOwner);
        IACL(ACL_ADDRESS).transferOwnership(aclOwner);
        IACLOwner(aclOwner).acceptACLOwnership();
    }

    ////////////////////////////////////////////////////////////////////////////
    // Phase 4 — implementations, then one atomic upgrade
    ////////////////////////////////////////////////////////////////////////////

    /**
     * @dev A single `ACLOwner.upgrade` so the stack is never half-materialized. The implementations are
     *      permissionless and nothing references their addresses, so unlike phase 1 the nonce they land
     *      at does not matter.
     *
     *      Each initializer that takes arguments gets its own encoder, because with nine ops and their
     *      arguments live in one frame legacy codegen runs out of stack slots, and scripts compile with
     *      via_ir off.
     */
    function _materialize(address aclOwner) private {
        ACLOwner.Op[] memory ops = new ACLOwner.Op[](9);
        ops[0] = ACLOwner.Op(
            ACL_ADDRESS, _create(ACL_CREATION_CODE, "ACL impl"), abi.encodeCall(IACL.initializeFromEmptyProxy, ())
        );
        ops[1] = ACLOwner.Op(
            FHEVM_EXECUTOR_ADDRESS,
            _create(CLEARTEXT_FHEVM_EXECUTOR_CREATION_CODE, "FHEVMExecutor impl"),
            abi.encodeCall(ICleartextFHEVMExecutor.initializeFromEmptyProxy, ())
        );
        ops[2] = ACLOwner.Op(
            KMS_VERIFIER_ADDRESS, _create(CLEARTEXT_KMS_VERIFIER_CREATION_CODE, "KMSVerifier impl"), _kmsVerifierInit()
        );
        ops[3] = ACLOwner.Op(
            INPUT_VERIFIER_ADDRESS,
            _create(CLEARTEXT_INPUT_VERIFIER_CREATION_CODE, "InputVerifier impl"),
            _inputVerifierInit()
        );
        ops[4] = ACLOwner.Op(HCU_LIMIT_ADDRESS, _create(HCU_LIMIT_CREATION_CODE, "HCULimit impl"), _hcuLimitInit());
        ops[5] = ACLOwner.Op(
            PROTOCOL_CONFIG_ADDRESS, _create(PROTOCOL_CONFIG_CREATION_CODE, "ProtocolConfig impl"), _protocolConfigInit()
        );
        ops[6] = ACLOwner.Op(
            KMS_GENERATION_ADDRESS,
            _create(KMS_GENERATION_CREATION_CODE, "KMSGeneration impl"),
            abi.encodeCall(IKMSGeneration.initializeFromEmptyProxy, ())
        );
        ops[7] = ACLOwner.Op(
            CLEARTEXT_ARITHMETIC_ADDRESS,
            _create(CLEARTEXT_ARITHMETIC_CREATION_CODE, "CleartextArithmetic impl"),
            abi.encodeCall(ICleartextArithmetic.initializeFromEmptyProxy, ())
        );
        // CleartextDB's initial writer is CleartextArithmetic.
        ops[8] = ACLOwner.Op(
            CLEARTEXT_DB_ADDRESS,
            _create(CLEARTEXT_DB_CREATION_CODE, "CleartextDB impl"),
            abi.encodeCall(ICleartextDB.initializeFromEmptyProxy, (CLEARTEXT_ARITHMETIC_ADDRESS))
        );

        IACLOwner(aclOwner).upgrade(ops);
    }

    function _kmsVerifierInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            ICleartextKMSVerifier.initializeFromEmptyProxy,
            (LocalHostBootstrap.DECRYPTION_ADDRESS, LocalHostBootstrap.GATEWAY_CHAIN_ID)
        );
    }

    function _inputVerifierInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            ICleartextInputVerifier.initializeFromEmptyProxy,
            (
                LocalHostBootstrap.INPUT_VERIFICATION_ADDRESS,
                LocalHostBootstrap.GATEWAY_CHAIN_ID,
                LocalHostBootstrap.coprocessorSigners(),
                LocalHostBootstrap.COPROCESSOR_THRESHOLD
            )
        );
    }

    function _hcuLimitInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            IHCULimit.initializeFromEmptyProxy,
            (
                LocalHostBootstrap.HCU_CAP_PER_BLOCK,
                LocalHostBootstrap.MAX_HCU_DEPTH_PER_TX,
                LocalHostBootstrap.MAX_HCU_PER_TX
            )
        );
    }

    function _protocolConfigInit() private pure returns (bytes memory) {
        address[] memory signers = LocalHostBootstrap.kmsSigners();
        address[] memory txSenders = LocalHostBootstrap.kmsTxSenders();
        string[] memory ips = LocalHostBootstrap.kmsIpAddresses();
        string[] memory urls = LocalHostBootstrap.kmsStorageUrls();

        IProtocolConfig.KmsNode[] memory nodes = new IProtocolConfig.KmsNode[](LocalHostBootstrap.KMS_NODE_COUNT);
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
        return abi.encodeCall(
            IProtocolConfig.initializeFromEmptyProxy,
            (
                nodes,
                IProtocolConfig.KmsThresholds({
                    publicDecryption: count,
                    userDecryption: count,
                    kmsGen: count,
                    mpc: count
                })
            )
        );
    }

    ////////////////////////////////////////////////////////////////////////////

    /// @dev Raw CREATE, because the bytecode arrives as bytes rather than as a compiled contract type.
    function _create(bytes memory creationCode, string memory what) private returns (address addr) {
        assembly {
            addr := create(0, add(creationCode, 0x20), mload(creationCode))
        }
        require(addr != address(0), string.concat("DeployLocalStack: failed to deploy ", what));
    }

    /**
     * @dev An ERC-1967 proxy over `implementation`, checked against the address the pre-compiled bytecode
     *      expects. A mismatch means the nonce sequence diverged and every later address is wrong too.
     */
    function _createProxy(address implementation, bytes memory initData, address expected, string memory what)
        private
    {
        address addr =
            _create(abi.encodePacked(ERC1967_PROXY_CREATION_CODE, abi.encode(implementation, initData)), what);
        require(addr == expected, string.concat("DeployLocalStack: ", what, " landed at the wrong address"));
        require(
            vm.load(addr, _ERC1967_IMPL_SLOT) == bytes32(uint256(uint160(implementation))),
            string.concat("DeployLocalStack: ", what, " implementation slot not set")
        );
    }
}
