// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";

// Imported for its side effect: `deployCodeTo` resolves an artifact by name out of the CONSUMING project's
// `out/`, and solc only emits an artifact for a file something actually imports. Without this import the
// proxy compiles inside forge-fhevm (where all of `src/` is built) but not in a downstream project, which
// then fails at runtime with "vm.getCode: no matching artifact found".
// forge-lint: disable-next-line(unused-import)
import {DeployableERC1967Proxy} from "./DeployableERC1967Proxy.sol";

import {ACL} from "@fhevm/host-contracts-cleartext/contracts/ACL.sol";
import {FHEVMExecutor} from "@fhevm/host-contracts-cleartext/contracts/FHEVMExecutor.sol";
import {HCULimit} from "@fhevm/host-contracts-cleartext/contracts/HCULimit.sol";
import {InputVerifier} from "@fhevm/host-contracts-cleartext/contracts/InputVerifier.sol";
import {KMSVerifier} from "@fhevm/host-contracts-cleartext/contracts/KMSVerifier.sol";
import {KMSGeneration} from "@fhevm/host-contracts-cleartext/contracts/KMSGeneration.sol";
import {ProtocolConfig} from "@fhevm/host-contracts-cleartext/contracts/ProtocolConfig.sol";
import {PauserSet} from "@fhevm/host-contracts-cleartext/contracts/immutable/PauserSet.sol";
import {EmptyUUPSProxy} from "@fhevm/host-contracts-cleartext/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {EmptyUUPSProxyACL} from "@fhevm/host-contracts-cleartext/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {IProtocolConfig} from "@fhevm/host-contracts-cleartext/contracts/interfaces/IProtocolConfig.sol";
import {KmsNodeParams, PcrValues} from "@fhevm/host-contracts-cleartext/contracts/shared/Structs.sol";

import {CleartextArithmetic} from "@fhevm/host-contracts-cleartext/cleartext/CleartextArithmetic.sol";
import {CleartextDB} from "@fhevm/host-contracts-cleartext/cleartext/CleartextDB.sol";
import {CleartextFHEVMExecutor} from "@fhevm/host-contracts-cleartext/cleartext/CleartextFHEVMExecutor.sol";
import {CleartextInputVerifier} from "@fhevm/host-contracts-cleartext/cleartext/CleartextInputVerifier.sol";
import {CleartextKMSVerifier} from "@fhevm/host-contracts-cleartext/cleartext/CleartextKMSVerifier.sol";

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
} from "@fhevm/host-contracts-cleartext/addresses/FHEVMHostAddresses.sol";

/**
 * Stands the cleartext host stack up at the addresses baked into its own bytecode.
 *
 * Everything here is a REAL deployment — genuine ERC-1967 proxies, genuine initializers. The only cheat is
 * `deployCodeTo`, which runs a proxy's constructor at a chosen address instead of a nonce-derived one. We
 * need that because the addresses are fixed (see src/config/addresses.sol): a contract under test compiles
 * `ZamaConfig`'s local addresses into itself, so the stack has to meet it there.
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
