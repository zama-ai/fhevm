// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

// Imported for its side effect: `deployCodeTo` resolves an artifact by name out of the CONSUMING project's
// `out/`, and solc only emits an artifact for a file something actually imports. Without this import the
// proxy compiles inside this package (where all of `src/` is built) but not in a downstream project, which
// then fails at runtime with "vm.getCode: no matching artifact found".
// forge-lint: disable-next-line(unused-import)
import {DeployableERC1967Proxy} from "./DeployableERC1967Proxy.sol";
// Same side-effect story: `_deployPauserSet` resolves PauserSet by artifact name, and nothing here
// references the type, so only this import makes the consumer's build emit the artifact.
// forge-lint: disable-next-line(unused-import)
import {PauserSet} from "../contracts/immutable/PauserSet.sol";

import {HCULimitNoDepthCap} from "./HCULimitNoDepthCap.sol";

import {ACL} from "../contracts/ACL.sol";
import {FHEVMExecutor} from "../contracts/FHEVMExecutor.sol";
import {HCULimit} from "../contracts/HCULimit.sol";
import {InputVerifier} from "../contracts/InputVerifier.sol";
import {KMSVerifier} from "../contracts/KMSVerifier.sol";
import {KMSGeneration} from "../contracts/KMSGeneration.sol";
import {ProtocolConfig} from "../contracts/ProtocolConfig.sol";
import {EmptyUUPSProxy} from "../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {EmptyUUPSProxyACL} from "../contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {IProtocolConfig} from "../contracts/interfaces/IProtocolConfig.sol";
import {KmsNodeParams, PcrValues} from "../contracts/shared/Structs.sol";

import {CleartextArithmetic} from "../cleartext/CleartextArithmetic.sol";
import {CleartextDB} from "../cleartext/CleartextDB.sol";
import {CleartextFHEVMExecutor} from "../cleartext/CleartextFHEVMExecutor.sol";
import {CleartextInputVerifier} from "../cleartext/CleartextInputVerifier.sol";
import {CleartextKMSVerifier} from "../cleartext/CleartextKMSVerifier.sol";

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
} from "../addresses/FHEVMHostAddresses.sol";

/**
 * Stands the cleartext host stack up at the addresses baked into its own bytecode — the Solidity counterpart
 * of this package's `ts/deployAt.ts`, for Foundry consumers.
 *
 * Deployment belongs to THIS package, for every target: a real chain (`ts/deploy.ts`, CREATE-based), a dev
 * node driven from TypeScript (`ts/deployAt.ts`), or a forge test (this contract). A consumer (such as
 * `forge-fhevm`) supplies only what the package cannot know: WHERE the stack must live (its `addresses.sol`
 * behind the `fhevm-config` remapping) and WHAT to initialize it with (the signer addresses).
 *
 * Everything here is a REAL deployment — genuine ERC-1967 proxies, genuine initializers. The only cheat is
 * `deployCodeTo`, which runs a proxy's constructor at a chosen address instead of a nonce-derived one. We
 * need that because the addresses are fixed: a contract under test compiles `ZamaConfig`'s local addresses
 * into itself, so the stack has to meet it there.
 *
 * ORDER IS LOAD-BEARING, for two reasons:
 *
 *  - ACL FIRST. It is the ownership root. Every other proxy's `_authorizeUpgrade` is `onlyACLOwner`, which
 *    resolves through `ACL.owner()` (see `ACLOwnable`). Until ACL exists and is owned, nothing else can be
 *    upgraded out of its empty-proxy state.
 *  - PROTOCOL CONFIG BEFORE THE VERIFIERS. In this version the KMS signer set and thresholds live in
 *    `ProtocolConfig`; `KMSVerifier.getKmsSigners()` and `getThreshold()` just forward to it.
 */
abstract contract FhevmStack is Test {
    /// @dev Owns ACL, and therefore the whole stack.
    address internal constant PROXY_OWNER = address(0xBEEF);

    string private constant PROXY_ARTIFACT = "DeployableERC1967Proxy.sol:DeployableERC1967Proxy";

    /// @dev HCU per-block cap is maxed out: a block cap throttles long test runs with
    ///      `HCUBlockLimitExceeded`, which is a production concern, not a testing one. The per-tx and depth
    ///      limits stay real, so tests still see the real per-transaction constraints.
    uint48 private constant HCU_CAP_PER_BLOCK = type(uint48).max;
    uint48 private constant HCU_MAX_DEPTH_PER_TX = 5_000_000;
    uint48 private constant HCU_MAX_PER_TX = 20_000_000;

    function _deployFhevmStack(address kmsSigner, address inputSigner) internal {
        _deployACL();
        _deployProtocolConfig(kmsSigner);

        _deployPauserSet();
        _deployBehindEmptyProxy(hcuLimitAdd, address(new HCULimit()), _hcuLimitInit());
        _deployBehindEmptyProxy(
            kmsGenerationAdd,
            address(new KMSGeneration()),
            _noArgsInit(KMSGeneration.initializeFromEmptyProxy.selector)
        );

        // The cleartext layer. The executor outgrew EIP-170, so the arithmetic lives in its own contract
        // and persists results to a shared DB; the executor itself never touches the DB. CleartextDB's only
        // writer is CleartextArithmetic — without that, every FHE op records nothing and decrypts to zero.
        _deployBehindEmptyProxy(
            cleartextArithmeticAdd,
            address(new CleartextArithmetic()),
            _noArgsInit(CleartextArithmetic.initializeFromEmptyProxy.selector)
        );
        _deployBehindEmptyProxy(
            cleartextDbAdd,
            address(new CleartextDB()),
            abi.encodeCall(CleartextDB.initializeFromEmptyProxy, (cleartextArithmeticAdd))
        );

        // The initializer is inherited from FHEVMExecutor; the cleartext variant only overrides the ops.
        _deployBehindEmptyProxy(
            fhevmExecutorAdd,
            address(new CleartextFHEVMExecutor()),
            _noArgsInit(FHEVMExecutor.initializeFromEmptyProxy.selector)
        );

        // The gateway identity the verifiers are initialized with is the EIP-712 domain we must later SIGN
        // against — it is an init argument here, not a baked-in constant. We point each at itself on this
        // chain, which is arbitrary but self-consistent.
        address[] memory inputSigners = new address[](1);
        inputSigners[0] = inputSigner;
        _deployBehindEmptyProxy(
            inputVerifierAdd,
            address(new CleartextInputVerifier()),
            abi.encodeCall(
                InputVerifier.initializeFromEmptyProxy,
                (inputVerifierAdd, uint64(block.chainid), inputSigners, 1)
            )
        );
        _deployBehindEmptyProxy(
            kmsVerifierAdd,
            address(new CleartextKMSVerifier()),
            abi.encodeCall(KMSVerifier.initializeFromEmptyProxy, (kmsVerifierAdd, uint64(block.chainid)))
        );
    }

    /**
     * Drops ONLY the sequential HCU depth cap, keeping every per-transaction and per-block charge.
     *
     * The depth cap bounds real FHE work; in a test it mostly punishes long end-to-end flows whose
     * orchestration is heavier than the calls being validated, surfacing as an opaque revert deep in a chain
     * of handles. Reach for this when a test fails only because it is long, never to paper over a contract
     * that genuinely exceeds the per-transaction limit — that limit stays enforced.
     */
    function disableHCUDepthLimit() internal {
        // Deploy BEFORE arming the prank: `new` is itself a call and would consume it, leaving
        // upgradeToAndCall to run as this test contract and revert on `onlyACLOwner`.
        address relaxed = address(new HCULimitNoDepthCap());

        vm.prank(PROXY_OWNER);
        HCULimit(hcuLimitAdd).upgradeToAndCall(relaxed, "");
    }

    /// @dev ACL is the one contract behind `EmptyUUPSProxyACL` (plain `Ownable2Step`) rather than
    ///      `EmptyUUPSProxy` (`onlyACLOwner`) — it cannot gate on its own owner before it has one.
    ///      `ACL.initializeFromEmptyProxy()` takes no owner and runs `__Ownable_init(owner())`, i.e. it
    ///      PRESERVES the owner the empty proxy set.
    function _deployACL() private {
        address emptyImpl = address(new EmptyUUPSProxyACL());
        deployCodeTo(
            PROXY_ARTIFACT,
            abi.encode(emptyImpl, abi.encodeCall(EmptyUUPSProxyACL.initialize, (PROXY_OWNER))),
            aclAdd
        );

        // Deploy the implementation BEFORE arming the prank: `new ACL()` is itself a call, and an inline
        // `address(new ACL())` argument would consume the prank, leaving upgradeToAndCall to run as this
        // test contract and revert with OwnableUnauthorizedAccount.
        address aclImpl = address(new ACL());

        vm.prank(PROXY_OWNER);
        EmptyUUPSProxyACL(aclAdd).upgradeToAndCall(aclImpl, abi.encodeCall(ACL.initializeFromEmptyProxy, ()));
    }

    function _deployProtocolConfig(address kmsSigner) private {
        KmsNodeParams[] memory nodes = new KmsNodeParams[](1);
        nodes[0] = KmsNodeParams({
            txSenderAddress: address(0xC0FFEE),
            signerAddress: kmsSigner,
            ipAddress: "127.0.0.1",
            storageUrl: "https://kms.local",
            partyId: 1,
            mpcIdentity: "kms-1",
            caCert: "",
            storagePrefix: ""
        });

        IProtocolConfig.KmsThresholds memory thresholds =
            IProtocolConfig.KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1});

        _deployBehindEmptyProxy(
            protocolConfigAdd,
            address(new ProtocolConfig()),
            abi.encodeCall(
                ProtocolConfig.initializeFromEmptyProxy, (nodes, thresholds, "0.0.0-test", new PcrValues[](0))
            )
        );
    }

    /// @dev PauserSet is immutable — no proxy, no initializer. It reads ACL through a compile-time constant.
    function _deployPauserSet() private {
        deployCodeTo("PauserSet.sol:PauserSet", pauserSetAdd);
    }

    /// @dev Stand up an empty proxy at `target`, then upgrade it to the real implementation and initialize
    ///      it in one call — the same empty-proxy -> real transition the production deployment performs.
    function _deployBehindEmptyProxy(address target, address implementation, bytes memory initCall) private {
        address emptyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(PROXY_ARTIFACT, abi.encode(emptyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())), target);

        vm.prank(PROXY_OWNER);
        EmptyUUPSProxy(payable(target)).upgradeToAndCall(implementation, initCall);
    }

    function _hcuLimitInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            HCULimit.initializeFromEmptyProxy, (HCU_CAP_PER_BLOCK, HCU_MAX_DEPTH_PER_TX, HCU_MAX_PER_TX)
        );
    }

    function _noArgsInit(bytes4 selector) private pure returns (bytes memory) {
        return abi.encodePacked(selector);
    }
}
