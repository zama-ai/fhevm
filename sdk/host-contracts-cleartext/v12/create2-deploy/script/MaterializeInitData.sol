// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md.

import {ACL} from "../../pkg/src/contracts/ACL.sol";
import {FHEVMExecutor} from "../../pkg/src/contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "../../pkg/src/contracts/KMSVerifier.sol";
import {InputVerifier} from "../../pkg/src/contracts/InputVerifier.sol";
import {HCULimit} from "../../pkg/src/contracts/HCULimit.sol";
import {CleartextArithmetic} from "../../pkg/src/cleartext/CleartextArithmetic.sol";
import {CleartextDB} from "../../pkg/src/cleartext/CleartextDB.sol";
import {LocalHostBootstrap} from "../../pkg/forge/src/_internal/LocalHostBootstrap.sol";

/**
 * @title MaterializeInitData
 * @notice The `initializeFromEmptyProxy` payloads step D carries.
 *
 * Structurally this is FhevmDeployScript._materialize with the `new Impl()` expressions removed — on
 * the CREATE2 path the implementations were already created in the creates stage, so step D only has
 * to name them. The arguments are identical, and come from the same place.
 *
 * ---------------------------------------------------------------------------------------------
 * Why LocalHostBootstrap and not literals
 * ---------------------------------------------------------------------------------------------
 *
 * LocalHostBootstrap is the generated Solidity mirror of DEFAULT_BOOTSTRAP_CONFIG in
 * pkg/ts/constants.ts, so this script and the TypeScript `deploy()` with no config produce the same
 * stack: same gateway chain id, same EIP-712 verifying contracts, four coprocessor signers, four KMS
 * nodes, thresholds equal to the node count.
 *
 * That is not a convenience default. The signer pools are derived from FHEVM_MNEMONIC at the HD paths
 * the js-sdk cleartext relayer derives its own keys from, and the relayer looks a signer up by the
 * address the chain reports. Register any other signer and the stack still deploys, verifies nothing,
 * and fails only in use. Regenerate LocalHostBootstrap to change them, rather than editing here.
 *
 * §10 still applies: the bootstrap config is CHOSEN, not inherited. `seal` records the full set in
 * the manifest so that what a deployment used is a fact on record rather than whatever the build
 * happened to bake in.
 *
 * ---------------------------------------------------------------------------------------------
 * Why abi.encodeCall and not abi.encodeWithSignature
 * ---------------------------------------------------------------------------------------------
 *
 * `encodeCall` type-checks the arguments against the real function; `encodeWithSignature` does not.
 * `uint64 chainId` silently encoded from a `uint256` literal, or an argument order quietly swapped,
 * is exactly the kind of thing that deploys, verifies against itself, and fails only when the relayer
 * shows up.
 *
 * Each payload gets its own encoder below. That is not decoration: with all the ops and their
 * init-args live in one frame, legacy codegen runs out of stack slots ("Stack too deep"), and scripts compile
 * with via_ir off — the same reason FhevmDeployScript splits them.
 *
 * Note these payloads are NOT part of any address. They travel as calldata to ACLOwner.upgrade, not
 * as initcode, so getting one wrong is recoverable in a way that getting FhevmCreate2Base's two
 * initializer signatures wrong is not.
 */
library MaterializeInitData {
    /// @dev Indexed by _allProxyRoles() position: 0 ACL, 1 FHEVMExecutor, 2 KMSVerifier,
    ///      3 InputVerifier, 4 HCULimit, 5 CleartextArithmetic, 6 CleartextDB.
    function initData(uint256 i, address cleartextArithmeticAdd) internal pure returns (bytes memory) {
        if (i == 0) return abi.encodeCall(ACL.initializeFromEmptyProxy, ());
        if (i == 1) return abi.encodeCall(FHEVMExecutor.initializeFromEmptyProxy, ());
        if (i == 2) return _kmsVerifier();
        if (i == 3) return _inputVerifier();
        if (i == 4) return _hcuLimit();
        if (i == 5) return abi.encodeCall(CleartextArithmetic.initializeFromEmptyProxy, ());
        // CleartextDB's initial writer is CleartextArithmetic — the one argument here that is a host
        // address rather than bootstrap config, hence the parameter.
        if (i == 6) return abi.encodeCall(CleartextDB.initializeFromEmptyProxy, (cleartextArithmeticAdd));
        revert("MaterializeInitData: index out of range");
    }

    /// @dev This generation's KMSVerifier carries its own signer set and threshold. 0.13 moves them out
    ///      to ProtocolConfig and reduces this call to the EIP-712 domain alone.
    function _kmsVerifier() private pure returns (bytes memory) {
        return
            abi.encodeCall(
                KMSVerifier.initializeFromEmptyProxy,
                (
                    LocalHostBootstrap.DECRYPTION_ADDRESS,
                    LocalHostBootstrap.GATEWAY_CHAIN_ID,
                    LocalHostBootstrap.kmsSigners(),
                    LocalHostBootstrap.KMS_NODE_COUNT
                )
            );
    }

    function _inputVerifier() private pure returns (bytes memory) {
        return
            abi.encodeCall(
                InputVerifier.initializeFromEmptyProxy,
                (
                    LocalHostBootstrap.INPUT_VERIFICATION_ADDRESS,
                    LocalHostBootstrap.GATEWAY_CHAIN_ID,
                    LocalHostBootstrap.coprocessorSigners(),
                    LocalHostBootstrap.COPROCESSOR_THRESHOLD
                )
            );
    }

    function _hcuLimit() private pure returns (bytes memory) {
        return
            abi.encodeCall(
                HCULimit.initializeFromEmptyProxy,
                (
                    LocalHostBootstrap.HCU_CAP_PER_BLOCK,
                    LocalHostBootstrap.MAX_HCU_DEPTH_PER_TX,
                    LocalHostBootstrap.MAX_HCU_PER_TX
                )
            );
    }
}
