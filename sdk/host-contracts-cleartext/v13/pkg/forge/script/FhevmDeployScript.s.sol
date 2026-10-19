// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Script, console} from "forge-std/Script.sol";
import {ERC1967Proxy} from "../../src/erc1967/ERC1967Proxy.sol";
import {EmptyUUPSProxyACL} from "../../src/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {EmptyUUPSProxy} from "../../src/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ACL} from "../../src/contracts/ACL.sol";
import {FHEVMExecutor} from "../../src/contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "../../src/contracts/KMSVerifier.sol";
import {InputVerifier} from "../../src/contracts/InputVerifier.sol";
import {HCULimit} from "../../src/contracts/HCULimit.sol";
import {ProtocolConfig} from "../../src/contracts/ProtocolConfig.sol";
import {KMSGeneration} from "../../src/contracts/KMSGeneration.sol";
import {PauserSet} from "../../src/contracts/immutable/PauserSet.sol";
import {IProtocolConfig} from "../../src/contracts/interfaces/IProtocolConfig.sol";
import {KmsNode} from "../../src/contracts/shared/Structs.sol";
import {CleartextFHEVMExecutor} from "../../src/cleartext/CleartextFHEVMExecutor.sol";
import {CleartextKMSVerifier} from "../../src/cleartext/CleartextKMSVerifier.sol";
import {CleartextInputVerifier} from "../../src/cleartext/CleartextInputVerifier.sol";
import {CleartextArithmetic} from "../../src/cleartext/CleartextArithmetic.sol";
import {CleartextDB} from "../../src/cleartext/CleartextDB.sol";
import {ACLOwner} from "../../src/upgrade/ACLOwner.sol";
import {PROXY_COUNT} from "../src/_internal/LocalHostAddresses.sol";
import {LocalHostBootstrap} from "../src/_internal/LocalHostBootstrap.sol";

import {
    aclAdd,
    fhevmExecutorAdd,
    kmsVerifierAdd,
    inputVerifierAdd,
    hcuLimitAdd,
    protocolConfigAdd,
    kmsGenerationAdd,
    pauserSetAdd,
    cleartextArithmeticAdd,
    cleartextDbAdd
} from "../../src/addresses/FHEVMHostAddresses.sol";

/**
 * @title FhevmDeployScript
 * @notice Step 2 of the two-phase FHEVM host deployment.
 *
 * Must be run AFTER ComputeAddresses.s.sol has written the generated
 * `fhevm-config-0.13.0/addresses.sol` and `forge build` has been executed with
 * that directory remapped, so that all implementation contracts carry the
 * correct baked-in addresses.
 *
 * Usage (from the package root):
 *
 *   FOUNDRY_REMAPPINGS=fhevm-config-0.13.0/=internal/.deploy-config/ \
 *     forge script pkg/forge/script/FhevmDeployScript.s.sol \
 *       --rpc-url <rpc> \
 *       --broadcast \
 *       --private-key $DEPLOYER_PRIVATE_KEY
 *
 * Required environment variables:
 *   DEPLOYER_PRIVATE_KEY             — deployer private key
 *
 * Optional environment variables:
 *   PAUSER_ADDRESS_0                 — Address to grant pauser role (skipped if unset)
 *
 * ---------------------------------------------------------------------------
 * Bootstrap arguments
 * ---------------------------------------------------------------------------
 *
 * Every initializer argument comes from `LocalHostBootstrap`, the generated
 * Solidity mirror of `DEFAULT_BOOTSTRAP_CONFIG` in pkg/ts/constants.ts. So
 * this script and the TypeScript `deploy()` with no config produce the same
 * stack: same gateway chain id, same EIP-712 verifying contracts, four
 * coprocessor signers, four KMS nodes, and KMS thresholds equal to the node
 * count.
 *
 * That is not merely a convenience default. The signer pools are derived from
 * FHEVM_MNEMONIC at the HD paths the js-sdk cleartext relayer derives its own
 * keys from, and the relayer looks a signer up by the address the chain
 * reports. Register any other signer and the stack still deploys, verifies
 * nothing, and fails only in use — so these values are what make the result
 * SDK-compatible, and they are deliberately not configurable here. Regenerate
 * LocalHostBootstrap to change them (`npm run generate:local-host-bytecode`).
 *
 * ---------------------------------------------------------------------------
 * Differences from the forge-fhevm original this is adapted from
 * ---------------------------------------------------------------------------
 *
 * 1. Nine proxies, not five: v13 adds ProtocolConfig and KMSGeneration, and the
 *    cleartext build adds CleartextArithmetic and CleartextDB.
 *
 * 2. ONE shared EmptyUUPSProxy implementation at nonce+2 serves every
 *    non-ACL proxy, where forge-fhevm deploys a fresh implementation per
 *    slot. That is what makes the proxy nonces contiguous (+3..+10) instead of
 *    odd-numbered, and it must match ComputeAddresses.s.sol exactly.
 *
 * 3. v13 moved the KMS signer set out of KMSVerifier into ProtocolConfig, so
 *    `KMSVerifier.initializeFromEmptyProxy` takes only (verifyingContract,
 *    chainId) here, and the signers/thresholds are seeded as a KMS context on
 *    ProtocolConfig instead.
 *
 * 4. The cleartext implementations replace three of the stock ones
 *    (FHEVMExecutor, KMSVerifier, InputVerifier). Their initializers are
 *    referenced through the *declaring* contract, because solc will not resolve
 *    an inherited function as a function pointer through the derived type.
 *
 * 5. A standing ACLOwner takes ownership of the stack, and every proxy are
 *    materialized in ONE atomic `ACLOwner.upgrade(ops)`. forge-fhevm has no
 *    equivalent because ACLOwner does not exist in the generation it targets —
 *    v12 has no upgrade/ directory at all — so it upgrades each proxy directly
 *    from the deployer EOA. Matching pkg/ts/deploy.ts matters for two reasons:
 *    the stack is never left half-materialized (separate per-proxy upgrades can fail
 *    midway, leaving some proxies real and some still empty), and
 *    `updateV12ToV13` requires the ACL owner to already be an ACLOwner rather
 *    than an EOA.
 *
 * ---------------------------------------------------------------------------
 * Two invariants worth stating, because breaking either fails silently
 * ---------------------------------------------------------------------------
 *
 * Ownership: `EmptyUUPSProxy._authorizeUpgrade` is `onlyACLOwner`, i.e. it
 * checks `Ownable2StepUpgradeable(aclAdd).owner()`. The ACL proxy is
 * initialized with `deployer` as owner at nonce+1, and `ACL.initializeFromEmptyProxy`
 * preserves it via `__Ownable_init(owner())`, so the deployer retains upgrade
 * authority over every proxy for the whole run. It also satisfies
 * `PauserSet.addPauser`, which is `onlyACLOwner` too.
 *
 * Nonces: under `--broadcast` every action is its own transaction, so a plain
 * *call* consumes a deployer nonce exactly as a CREATE does. Nonces +0..+11
 * below are therefore an unbroken run of creations — no call, and nothing else
 * sent from the deployer, may be interleaved before PauserSet lands, or every
 * remaining address shifts while the compiled bytecode keeps pointing at the
 * old ones.
 */
contract FhevmDeployScript is Script {
    function run() external {
        uint256 deployerKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(deployerKey);

        vm.startBroadcast(deployerKey);

        // ----------------------------------------------------------------
        // Step 1: Deploy the 9 empty UUPS proxies (nonce+0 .. nonce+10).
        //
        // Deployment order must exactly match the nonce offsets computed in
        // ComputeAddresses.s.sol.
        //
        // ACL must come first: every other EmptyUUPSProxy imports aclAdd via
        // ACLOwnable and checks the ACL owner in _authorizeUpgrade.
        // ----------------------------------------------------------------
        _deployEmptyProxies(deployer);

        // ----------------------------------------------------------------
        // Step 2: Deploy PauserSet (immutable, no proxy).
        //
        // nonce+11: PauserSet → must equal pauserSetAdd
        // ----------------------------------------------------------------
        {
            PauserSet ps = new PauserSet();
            require(address(ps) == pauserSetAdd, "FhevmDeployScript: PauserSet address mismatch");
            console.log("PauserSet:                  ", address(ps));
        }

        // ----------------------------------------------------------------
        // Step 3: Install the standing ACLOwner and hand it ACL ownership.
        //
        // nonce+12: ACLOwner. Nothing compiled in refers to it, so unlike the
        // creations above its address is free to move.
        // ----------------------------------------------------------------
        ACLOwner aclOwner = _setupACLOwner(deployer);

        // ----------------------------------------------------------------
        // Step 4: Materialize every proxy in one atomic upgrade.
        //
        // The generated addresses.sol is complete, so all implementations
        // compile with the correct baked-in addresses.
        // ----------------------------------------------------------------
        _materialize(aclOwner);

        vm.stopBroadcast();

        console.log("");
        console.log("--- Deployment complete ---");
        console.log("aclAdd:                 ", aclAdd);
        console.log("fhevmExecutorAdd:       ", fhevmExecutorAdd);
        console.log("kmsVerifierAdd:         ", kmsVerifierAdd);
        console.log("inputVerifierAdd:       ", inputVerifierAdd);
        console.log("hcuLimitAdd:            ", hcuLimitAdd);
        console.log("protocolConfigAdd:      ", protocolConfigAdd);
        console.log("kmsGenerationAdd:       ", kmsGenerationAdd);
        console.log("cleartextArithmeticAdd: ", cleartextArithmeticAdd);
        console.log("cleartextDbAdd:         ", cleartextDbAdd);
        console.log("pauserSetAdd:           ", pauserSetAdd);
        console.log("aclOwner:               ", address(aclOwner));
    }

    // ------------------------------------------------------------------
    // Step 1 helpers
    // ------------------------------------------------------------------

    function _deployEmptyProxies(address deployer) private {
        // nonce+0: EmptyUUPSProxyACL implementation
        // nonce+1: ACL proxy → must equal aclAdd
        {
            EmptyUUPSProxyACL aclImpl = new EmptyUUPSProxyACL();
            ERC1967Proxy aclProxy =
                new ERC1967Proxy(address(aclImpl), abi.encodeCall(EmptyUUPSProxyACL.initialize, (deployer)));
            require(address(aclProxy) == aclAdd, "FhevmDeployScript: ACL proxy address mismatch");
            console.log("ACL empty proxy:            ", address(aclProxy));
        }

        // nonce+2: the single EmptyUUPSProxy implementation shared by every
        // remaining proxy. Deployed once, unlike forge-fhevm's per-slot impl.
        EmptyUUPSProxy sharedImpl = new EmptyUUPSProxy();
        console.log("EmptyUUPSProxy shared impl: ", address(sharedImpl));

        _emptyProxy(sharedImpl, fhevmExecutorAdd, "FHEVMExecutor"); // nonce+3
        _emptyProxy(sharedImpl, kmsVerifierAdd, "KMSVerifier"); // nonce+4
        _emptyProxy(sharedImpl, inputVerifierAdd, "InputVerifier"); // nonce+5
        _emptyProxy(sharedImpl, hcuLimitAdd, "HCULimit"); // nonce+6
        _emptyProxy(sharedImpl, protocolConfigAdd, "ProtocolConfig"); // nonce+7
        _emptyProxy(sharedImpl, kmsGenerationAdd, "KMSGeneration"); // nonce+8
        _emptyProxy(sharedImpl, cleartextArithmeticAdd, "CleartextArithmetic"); // nonce+9
        _emptyProxy(sharedImpl, cleartextDbAdd, "CleartextDB"); // nonce+10
    }

    /// @dev One ERC1967 proxy over the shared empty implementation, checked against its computed address.
    function _emptyProxy(EmptyUUPSProxy sharedImpl, address expected, string memory name) private {
        ERC1967Proxy proxy = new ERC1967Proxy(address(sharedImpl), abi.encodeCall(EmptyUUPSProxy.initialize, ()));
        require(address(proxy) == expected, string.concat("FhevmDeployScript: ", name, " proxy address mismatch"));
        console.log(string.concat(name, " empty proxy:"), address(proxy));
    }

    // ------------------------------------------------------------------
    // Step 3 — the standing ACLOwner
    // ------------------------------------------------------------------

    /**
     * @dev Order is load-bearing. `PauserSet.addPauser` is `onlyACLOwner`, so both the ACLOwner itself and
     *      any operator pauser must be registered while `deployer` still owns ACL — after the transfer the
     *      deployer can no longer call it directly, only via `aclOwner.execute(...)`. Ownership then moves
     *      two-step: the deployer offers, and the ACLOwner accepts on its own behalf.
     */
    function _setupACLOwner(address deployer) private returns (ACLOwner aclOwner) {
        aclOwner = new ACLOwner(deployer, aclAdd);
        console.log("ACLOwner:                   ", address(aclOwner));

        PauserSet(pauserSetAdd).addPauser(address(aclOwner));

        // Optional operator pauser — must land before ownership moves, for the reason above.
        try vm.envAddress("PAUSER_ADDRESS_0") returns (address pauser) {
            PauserSet(pauserSetAdd).addPauser(pauser);
            console.log("Pauser added:               ", pauser);
        } catch {
            // PAUSER_ADDRESS_0 not set — skipping
        }

        ACL(aclAdd).transferOwnership(address(aclOwner));
        aclOwner.acceptACLOwnership();
        console.log("ACL ownership accepted by:   ", address(aclOwner));
    }

    // ------------------------------------------------------------------
    // Step 4 — implementations and one atomic upgrade
    // ------------------------------------------------------------------

    /**
     * @dev A single `ACLOwner.upgrade` so the stack is never half-materialized: each op points a proxy at
     *      its real implementation and runs the initializer in the same call, and the batch reverts whole.
     *
     *      The implementations are permissionless and their addresses are never referenced, so unlike the
     *      creations in step 1 nothing here depends on the nonce they land at.
     *
     *      Each initializer that takes arguments gets its own encoder below. That is not decoration: with
     *      all the ops and their init-args live in one frame, legacy codegen runs out of stack slots ("Stack
     *      too deep"), and scripts compile with via_ir off.
     */
    function _materialize(ACLOwner aclOwner) private {
        ACLOwner.Op[] memory ops = new ACLOwner.Op[](PROXY_COUNT);

        ops[0] = ACLOwner.Op(aclAdd, address(new ACL()), abi.encodeCall(ACL.initializeFromEmptyProxy, ()));
        ops[1] = ACLOwner.Op(
            fhevmExecutorAdd,
            address(new CleartextFHEVMExecutor()),
            abi.encodeCall(FHEVMExecutor.initializeFromEmptyProxy, ())
        );
        ops[2] = ACLOwner.Op(kmsVerifierAdd, address(new CleartextKMSVerifier()), _kmsVerifierInit());
        ops[3] = ACLOwner.Op(inputVerifierAdd, address(new CleartextInputVerifier()), _inputVerifierInit());
        ops[4] = ACLOwner.Op(hcuLimitAdd, address(new HCULimit()), _hcuLimitInit());
        ops[5] = ACLOwner.Op(protocolConfigAdd, address(new ProtocolConfig()), _protocolConfigInit());
        ops[6] = ACLOwner.Op(
            kmsGenerationAdd, address(new KMSGeneration()), abi.encodeCall(KMSGeneration.initializeFromEmptyProxy, ())
        );
        ops[7] = ACLOwner.Op(
            cleartextArithmeticAdd,
            address(new CleartextArithmetic()),
            abi.encodeCall(CleartextArithmetic.initializeFromEmptyProxy, ())
        );
        // CleartextDB's initial writer is CleartextArithmetic.
        ops[8] = ACLOwner.Op(
            cleartextDbAdd,
            address(new CleartextDB()),
            abi.encodeCall(CleartextDB.initializeFromEmptyProxy, (cleartextArithmeticAdd))
        );

        aclOwner.upgrade(ops);
        console.log("Every proxy materialized in one ACLOwner.upgrade");
    }

    /// @dev v13: signers moved to ProtocolConfig, so only the EIP-712 domain is set here.
    function _kmsVerifierInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            KMSVerifier.initializeFromEmptyProxy,
            (LocalHostBootstrap.DECRYPTION_ADDRESS, LocalHostBootstrap.GATEWAY_CHAIN_ID)
        );
    }

    function _inputVerifierInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            InputVerifier.initializeFromEmptyProxy,
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
            HCULimit.initializeFromEmptyProxy,
            (
                LocalHostBootstrap.HCU_CAP_PER_BLOCK,
                LocalHostBootstrap.MAX_HCU_DEPTH_PER_TX,
                LocalHostBootstrap.MAX_HCU_PER_TX
            )
        );
    }

    /// @dev v13 only: seeds the initial KMS context that KMSVerifier used to hold itself.
    function _protocolConfigInit() private pure returns (bytes memory) {
        return abi.encodeCall(ProtocolConfig.initializeFromEmptyProxy, (_defaultKmsNodes(), _defaultKmsThresholds()));
    }

    /// @dev The four default KMS nodes, assembled from LocalHostBootstrap's parallel arrays.
    function _defaultKmsNodes() private pure returns (KmsNode[] memory nodes) {
        address[] memory signers = LocalHostBootstrap.kmsSigners();
        address[] memory txSenders = LocalHostBootstrap.kmsTxSenders();
        string[] memory ips = LocalHostBootstrap.kmsIpAddresses();
        string[] memory urls = LocalHostBootstrap.kmsStorageUrls();

        nodes = new KmsNode[](LocalHostBootstrap.KMS_NODE_COUNT);
        for (uint256 i = 0; i < nodes.length; i++) {
            nodes[i] = KmsNode({
                txSenderAddress: txSenders[i], signerAddress: signers[i], ipAddress: ips[i], storageUrl: urls[i]
            });
        }
    }

    /// @dev Every threshold is the node count, as ts/constants.ts DEFAULT_KMS_THRESHOLDS has it.
    function _defaultKmsThresholds() private pure returns (IProtocolConfig.KmsThresholds memory) {
        uint256 count = LocalHostBootstrap.KMS_NODE_COUNT;
        return
            IProtocolConfig.KmsThresholds({publicDecryption: count, userDecryption: count, kmsGen: count, mpc: count});
    }
}
