// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {console} from "forge-std/Script.sol";
import {VmSafe} from "forge-std/Vm.sol";

import {
    DEPLOYER_PRIVATE_KEY_ENV,
    DEPLOYER_MNEMONIC_ENV,
    EMPTY_UUPS_PRIVATE_KEY_ENV,
    EMPTY_UUPS_MNEMONIC_ENV,
    NUM_COPROCESSORS_ENV,
    COPROCESSORS_MNEMONIC_ENV,
    NUM_KMS_NODES_ENV,
    KMS_NODES_MNEMONIC_ENV,
    HCU_CAP_PER_BLOCK_ENV,
    NUM_PAUSERS_ENV,
    PAUSERS_MNEMONIC_ENV,
    COPROCESSOR_THRESHOLD_ENV,
    KMS_THRESHOLD_ENV,
    MAX_HCU_PER_TX_ENV,
    MAX_HCU_DEPTH_PER_TX_ENV,
    CHAIN_ID_GATEWAY_ENV,
    DECRYPTION_ADDRESS_ENV,
    INPUT_VERIFICATION_ADDRESS_ENV
} from "./EnvNames.sol";

import {AssertLib} from "./AssertLib.sol";
import {Signer, SignerLib} from "./SignerLib.sol";
import {DeployLib} from "./DeployLib.sol";

// Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext input verification"))))`.
address constant INPUT_VERIFICATION_ADDRESS = 0x6189F6c0c3E40B4a3c72ec86262295D78d845297;
// Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext decryption"))))`.
address constant DECRYPTION_ADDRESS = 0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721;

library FhevmConfigLib {
    function resolveDeployerFromEnv(VmSafe vm) internal returns (Signer memory) {
        return SignerLib.resolvePrivateKeyFromEnv(vm, DEPLOYER_PRIVATE_KEY_ENV, DEPLOYER_MNEMONIC_ENV);
    }

    function canResolveEmptyUupsDeployerFromEnv(VmSafe vm) internal returns (bool) {
        return SignerLib.canResolvePrivateKeyFromEnv(vm, EMPTY_UUPS_PRIVATE_KEY_ENV, EMPTY_UUPS_MNEMONIC_ENV);
    }

    function resolveEmptyUupsDeployerFromEnv(VmSafe vm) internal returns (Signer memory) {
        return SignerLib.resolvePrivateKeyFromEnv(vm, EMPTY_UUPS_PRIVATE_KEY_ENV, EMPTY_UUPS_MNEMONIC_ENV);
    }

    function resolveDecryptionAddressFromEnv(VmSafe vm) internal returns (address decryptionAddr) {
        decryptionAddr = _resolveAddressFromEnv(vm, DECRYPTION_ADDRESS_ENV);
    }

    function resolveInputVerificationAddressFromEnv(VmSafe vm) internal returns (address inputVerificationAddr) {
        inputVerificationAddr = _resolveAddressFromEnv(vm, INPUT_VERIFICATION_ADDRESS_ENV);
    }

    function resolveGatewayChainIdFromEnv(VmSafe vm) internal returns (uint64 gatewayChainId) {
        gatewayChainId = _resolveUint64FromEnv(vm, CHAIN_ID_GATEWAY_ENV);
    }

    function resolveKmsThresholdFromEnv(VmSafe vm) internal returns (uint256 kmsThreshold) {
        AssertLib.assertEnvExists(vm, KMS_THRESHOLD_ENV);
        kmsThreshold = vm.envUint(KMS_THRESHOLD_ENV);
    }

    function resolveCoprocessorThresholdFromEnv(VmSafe vm) internal returns (uint256 coprocessorThreshold) {
        AssertLib.assertEnvExists(vm, COPROCESSOR_THRESHOLD_ENV);
        coprocessorThreshold = vm.envUint(COPROCESSOR_THRESHOLD_ENV);
    }

    function resolveDeployers(VmSafe vm) internal returns (Signer[] memory signers) {
        Signer memory deployer = resolveDeployerFromEnv(vm);
        bool hasEmptyUups = canResolveEmptyUupsDeployerFromEnv(vm);

        signers = new Signer[](hasEmptyUups ? 2 : 1);
        signers[0] = deployer;
        if (hasEmptyUups) {
            signers[1] = resolveEmptyUupsDeployerFromEnv(vm);
        }
    }

    /// Produces a JSON representation of the signers returned by
    /// `resolveDeployers`. If the empty-UUPS deployer isn't configured, its
    /// `address` / `privateKey` fields are emitted as the zero address /
    /// zero bytes32 (rather than being omitted) so the JSON shape is stable.
    ///
    /// Output (single-line, no whitespace):
    /// {"deployer":{"address":"0x...","privateKey":"0x..."},
    ///  "emptyUupsDeployer":{"address":"0x...","privateKey":"0x..."}}
    function resolveDeployersAsJson(VmSafe vm) internal returns (string memory) {
        Signer[] memory signers = resolveDeployers(vm);

        Signer memory deployer = signers[0];
        Signer memory emptyUupsDeployer; // defaults to address(0), bytes32(0)
        if (signers.length >= 2) {
            emptyUupsDeployer = signers[1];
        }

        return string.concat(
            "{",
            "\"deployer\":{",
            "\"address\":\"",
            vm.toString(deployer.addr),
            "\",",
            "\"privateKey\":\"",
            vm.toString(bytes32(deployer.privateKey)),
            "\"",
            "},",
            "\"emptyUupsDeployer\":{",
            "\"address\":\"",
            vm.toString(emptyUupsDeployer.addr),
            "\",",
            "\"privateKey\":\"",
            vm.toString(bytes32(emptyUupsDeployer.privateKey)),
            "\"",
            "}",
            "}"
        );
    }

    function resolveHcuConfigFromEnv(VmSafe vm)
        internal
        returns (uint48 hcuCapPerBlock, uint48 maxHCUDepthPerTx, uint48 maxHCUPerTx)
    {
        hcuCapPerBlock = _resolveUint48FromEnv(vm, HCU_CAP_PER_BLOCK_ENV);
        maxHCUDepthPerTx = _resolveUint48FromEnv(vm, MAX_HCU_DEPTH_PER_TX_ENV);
        maxHCUPerTx = _resolveUint48FromEnv(vm, MAX_HCU_PER_TX_ENV);
    }

    function resolveCoprocessorSignersFromEnv(VmSafe vm) internal view returns (Signer[] memory) {
        return SignerLib.resolveListFromEnv(vm, NUM_COPROCESSORS_ENV, COPROCESSORS_MNEMONIC_ENV, 0);
    }

    function resolveKmsSignersFromEnv(VmSafe vm) internal view returns (Signer[] memory) {
        return SignerLib.resolveListFromEnv(vm, NUM_KMS_NODES_ENV, KMS_NODES_MNEMONIC_ENV, 0);
    }

    function resolvePausersFromEnv(VmSafe vm) internal view returns (Signer[] memory) {
        return SignerLib.resolveListFromEnv(vm, NUM_PAUSERS_ENV, PAUSERS_MNEMONIC_ENV, 0);
    }

    function _resolveUint48FromEnv(VmSafe vm, string memory envName) private returns (uint48 value) {
        AssertLib.assertEnvExists(vm, envName);

        uint256 envValue = vm.envUint(envName);
        require(envValue <= type(uint48).max, AssertLib.boxMessage(string.concat(envName, " exceeds uint48")));

        return uint48(envValue);
    }

    function _resolveUint64FromEnv(VmSafe vm, string memory envName) private returns (uint64 value) {
        AssertLib.assertEnvExists(vm, envName);

        uint256 envValue = vm.envUint(envName);
        require(envValue <= type(uint64).max, AssertLib.boxMessage(string.concat(envName, " exceeds uint64")));

        return uint64(envValue);
    }

    function _resolveAddressFromEnv(VmSafe vm, string memory envName) private returns (address value) {
        AssertLib.assertEnvExists(vm, envName);

        return vm.envAddress(envName);
    }
}
