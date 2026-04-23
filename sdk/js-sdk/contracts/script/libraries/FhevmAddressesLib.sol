// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {console} from "forge-std/Script.sol";
import {Vm} from "forge-std/Vm.sol";
import {FhevmCheats} from "./FhevmCheats.sol";

import {
    DEPLOYER_PRIVATE_KEY_ENV,
    EMPTY_UUPS_DEPLOYER_PRIVATE_KEY_ENV,
    FHEVM_HOST_CONTRACTS_MNEMONIC,
    FHEVM_HOST_CONTRACTS_MNEMONIC_DERIVATION_PREFIX,
    FHEVM_HOST_CONTRACTS_MNEMONIC_DERIVATION_INDEX,
    FHEVM_HOST_CONTRACTS_MNEMONIC_EMPTY_UUPS_DEPLOYER_INDEX,
    NUM_PAUSERS_ENV,
    PAUSER_ADDRESS_ENV_PREFIX,
    DEFAULT_PAUSER_START_INDEX,
    DEFAULT_PAUSER_COUNT,
    NUM_KMS_NODES_ENV,
    KMS_SIGNER_ADDRESS_ENV_PREFIX,
    DEFAULT_KMS_SIGNER_START_INDEX,
    DEFAULT_KMS_SIGNER_COUNT,
    NUM_COPROCESSORS_ENV,
    COPROCESSOR_SIGNER_ADDRESS_ENV_PREFIX,
    DEFAULT_COPROCESSOR_SIGNER_START_INDEX,
    DEFAULT_COPROCESSOR_SIGNER_COUNT,
    FHEVM_MNEMONIC,
    COPROCESSOR_SIGNERS_MNEMONIC_PREFIX,
    KMS_SIGNERS_MNEMONIC_PREFIX,
    PAUSERS_MNEMONIC_PREFIX,
    CHAIN_ID_GATEWAY_ENV,
    DEFAULT_CHAIN_ID_GATEWAY,
    KMS_THRESHOLD_ENV,
    DEFAULT_KMS_THRESHOLD,
    COPROCESSOR_THRESHOLD_ENV,
    DEFAULT_COPROCESSOR_THRESHOLD,
    FHE_TEST_USER_PRIVATE_KEY_ENV,
    FHEVM_CHEATS_ADDRESS
} from "./Constants.sol";

import {CoprocessorConfig} from "@fhevm/solidity/lib/Impl.sol";
import {EmptyUUPSProxyACL} from "@fhevm/host-contracts/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {EmptyUUPSProxy} from "@fhevm/host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {ACL} from "@fhevm/host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "@fhevm/host-contracts/contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "@fhevm/host-contracts/contracts/KMSVerifier.sol";
import {InputVerifier} from "@fhevm/host-contracts/contracts/InputVerifier.sol";
import {HCULimit} from "@fhevm/host-contracts/contracts/HCULimit.sol";
import {PauserSet} from "@fhevm/host-contracts/contracts/immutable/PauserSet.sol";
import {FHETest} from "../../src/FHETest.sol";
import {FheType} from "@fhevm/solidity/lib/FheType.sol";

/// @notice Holds the six deterministic CREATE addresses produced by the FHEVM
///         host deployment nonce layout.
struct FhevmAddresses {
    address acl;
    address fhevmExecutor;
    address kmsVerifier;
    address inputVerifier;
    address hcuLimit;
    address pauserSet;
}

/// @notice An EOA identity: a private key and its derived address.
struct Signer {
    uint256 privateKey;
    address addr;
}

/// @title  FhevmAddressesLib
/// @notice Pure helpers for computing and persisting the set of FHEVM host
///         addresses baked into `FHEVMHostAddresses.sol`.
///
/// The deployment nonce layout assumed here is:
///
///   N+0  EmptyUUPSProxyACL impl
///   N+1  ERC1967Proxy (ACL)              → acl
///   N+2  EmptyUUPSProxy impl             (FHEVMExecutor slot)
///   N+3  ERC1967Proxy (FHEVMExecutor)    → fhevmExecutor
///   N+4  EmptyUUPSProxy impl             (KMSVerifier slot)
///   N+5  ERC1967Proxy (KMSVerifier)      → kmsVerifier
///   N+6  EmptyUUPSProxy impl             (InputVerifier slot)
///   N+7  ERC1967Proxy (InputVerifier)    → inputVerifier
///   N+8  EmptyUUPSProxy impl             (HCULimit slot)
///   N+9  ERC1967Proxy (HCULimit)         → hcuLimit
///   N+10 PauserSet                       → pauserSet
library FhevmAddressesLib {
    function resolveGatewayChainIdFromEnv(Vm vm) internal view returns (uint64 gatewayChainId) {
        if (vm.envExists(CHAIN_ID_GATEWAY_ENV)) {
            gatewayChainId = uint64(vm.envUint(CHAIN_ID_GATEWAY_ENV));
        } else {
            gatewayChainId = DEFAULT_CHAIN_ID_GATEWAY;
        }
    }

    function resolveKmsThresholdFromEnv(Vm vm) internal view returns (uint256 kmsThreshold) {
        if (vm.envExists(KMS_THRESHOLD_ENV)) {
            kmsThreshold = vm.envUint(KMS_THRESHOLD_ENV);
        } else {
            kmsThreshold = DEFAULT_KMS_THRESHOLD;
        }
    }

    function resolveCoprocessorThresholdFromEnv(Vm vm) internal view returns (uint256 coprocessorThreshold) {
        if (vm.envExists(COPROCESSOR_THRESHOLD_ENV)) {
            coprocessorThreshold = vm.envUint(COPROCESSOR_THRESHOLD_ENV);
        } else {
            coprocessorThreshold = DEFAULT_COPROCESSOR_THRESHOLD;
        }
    }

    /// Resolves the deployer key:
    ///   - if `DEPLOYER_PRIVATE_KEY` is set in the env, use it;
    ///   - otherwise, derive from `FHEVM_HOST_CONTRACTS_MNEMONIC` at path
    ///     `FHEVM_HOST_CONTRACTS_MNEMONIC_DERIVATION_PREFIX` +
    ///     `FHEVM_HOST_CONTRACTS_MNEMONIC_DERIVATION_INDEX`.
    function resolveDeployerFromEnv(Vm vm) internal view returns (Signer memory d) {
        if (vm.envExists(DEPLOYER_PRIVATE_KEY_ENV)) {
            d.privateKey = vm.envUint(DEPLOYER_PRIVATE_KEY_ENV);
        } else {
            d.privateKey = vm.deriveKey(
                FHEVM_HOST_CONTRACTS_MNEMONIC,
                FHEVM_HOST_CONTRACTS_MNEMONIC_DERIVATION_PREFIX,
                FHEVM_HOST_CONTRACTS_MNEMONIC_DERIVATION_INDEX
            );
        }
        d.addr = vm.addr(d.privateKey);
    }

    function resolveFHETestUserFromEnv(Vm vm) internal view returns (Signer memory d) {
        if (vm.envExists(FHE_TEST_USER_PRIVATE_KEY_ENV)) {
            d.privateKey = vm.envUint(FHE_TEST_USER_PRIVATE_KEY_ENV);
            d.addr = vm.addr(d.privateKey);
        }
    }

    /// Resolves the dedicated key used to deploy `EmptyUUPSProxyACL` /
    /// `EmptyUUPSProxy` implementations. Keeping these on a separate key
    /// preserves the main deployer's nonce timeline so the subsequent proxy
    /// CREATEs land at the addresses in FHEVMHostAddresses.sol.
    ///
    ///   - if `EMPTY_UUPS_DEPLOYER_PRIVATE_KEY` is set, use it;
    ///   - otherwise, derive from `FHEVM_HOST_CONTRACTS_MNEMONIC` at index
    ///     `FHEVM_HOST_CONTRACTS_MNEMONIC_EMPTY_UUPS_DEPLOYER_INDEX`.
    function resolveEmptyUupsDeployerFromEnv(Vm vm) internal view returns (Signer memory d) {
        if (vm.envExists(EMPTY_UUPS_DEPLOYER_PRIVATE_KEY_ENV)) {
            d.privateKey = vm.envUint(EMPTY_UUPS_DEPLOYER_PRIVATE_KEY_ENV);
        } else {
            d.privateKey = vm.deriveKey(
                FHEVM_HOST_CONTRACTS_MNEMONIC,
                FHEVM_HOST_CONTRACTS_MNEMONIC_DERIVATION_PREFIX,
                FHEVM_HOST_CONTRACTS_MNEMONIC_EMPTY_UUPS_DEPLOYER_INDEX
            );
        }
        d.addr = vm.addr(d.privateKey);
    }

    /// Resolves N pauser addresses by deriving them from
    /// `FHEVM_MNEMONIC` at `PAUSERS_MNEMONIC_PREFIX + i`.
    /// `N` is read from `NUM_PAUSERS_ENV`, or falls back to
    /// `DEFAULT_PAUSER_COUNT` when the env var is unset.
    function resolvePausersFromEnv(Vm vm) internal view returns (address[] memory list) {
        uint256 count = vm.envExists(NUM_PAUSERS_ENV)
            ? vm.envUint(NUM_PAUSERS_ENV)
            : DEFAULT_PAUSER_COUNT;
        list = new address[](count);
        for (uint256 i = 0; i < count; i++) {
            uint256 pk = vm.deriveKey(FHEVM_MNEMONIC, PAUSERS_MNEMONIC_PREFIX, uint32(i));
            list[i] = vm.addr(pk);
        }
    }

    /// Resolves N KMS signer addresses by deriving them from
    /// `FHEVM_MNEMONIC` at `KMS_SIGNERS_MNEMONIC_PREFIX + i`.
    /// `N` is read from `NUM_KMS_NODES_ENV`, or falls back to
    /// `DEFAULT_KMS_SIGNER_COUNT` when the env var is unset.
    function resolveKmsSignersFromEnv(Vm vm) internal view returns (address[] memory list) {
        uint256 count = vm.envExists(NUM_KMS_NODES_ENV)
            ? vm.envUint(NUM_KMS_NODES_ENV)
            : DEFAULT_KMS_SIGNER_COUNT;
        list = new address[](count);
        for (uint256 i = 0; i < count; i++) {
            uint256 pk = vm.deriveKey(FHEVM_MNEMONIC, KMS_SIGNERS_MNEMONIC_PREFIX, uint32(i));
            list[i] = vm.addr(pk);
        }
    }

    /// Resolves N coprocessor signer addresses by deriving them from
    /// `FHEVM_MNEMONIC` at `COPROCESSOR_SIGNERS_MNEMONIC_PREFIX + i`.
    /// `N` is read from `NUM_COPROCESSORS_ENV`, or falls back to
    /// `DEFAULT_COPROCESSOR_SIGNER_COUNT` when the env var is unset.
    function resolveCoprocessorSignersFromEnv(Vm vm) internal view returns (address[] memory list) {
        uint256 count = vm.envExists(NUM_COPROCESSORS_ENV)
            ? vm.envUint(NUM_COPROCESSORS_ENV)
            : DEFAULT_COPROCESSOR_SIGNER_COUNT;
        list = new address[](count);
        for (uint256 i = 0; i < count; i++) {
            uint256 pk = vm.deriveKey(FHEVM_MNEMONIC, COPROCESSOR_SIGNERS_MNEMONIC_PREFIX, uint32(i));
            list[i] = vm.addr(pk);
        }
    }

    /// Shared resolver for indexed `<prefix>i` address env vars:
    ///   - If `countEnv` is set, the count is fixed and every
    ///     `<prefix>0` .. `<prefix>{N-1}` must exist (reverts on any gap).
    ///   - Otherwise, iterate `<prefix>0`, `<prefix>1`, ... stopping at the
    ///     first missing index (empty array if none set).
    function _resolveAddressListFromEnv(Vm vm, string memory countEnv, string memory addressPrefix)
        private
        view
        returns (address[] memory list)
    {
        uint256 count;
        bool strict = vm.envExists(countEnv);

        if (strict) {
            count = vm.envUint(countEnv);
        } else {
            while (vm.envExists(string.concat(addressPrefix, vm.toString(count)))) {
                count++;
            }
        }

        list = new address[](count);
        for (uint256 i = 0; i < count; i++) {
            string memory name = string.concat(addressPrefix, vm.toString(i));
            if (strict) {
                require(vm.envExists(name), string.concat("Missing env: ", name));
            }
            list[i] = vm.envAddress(name);
        }
    }

    /// Computes the six addresses reachable from `deployer` starting at
    /// `startNonce`, following the FHEVM host nonce layout.
    function compute(Vm vm, address deployer, uint64 startNonce) internal pure returns (FhevmAddresses memory a) {
        // 2 modes:
        // fhevm-repo mode:
        a.pauserSet = vm.computeCreateAddress(deployer, startNonce + 0);
        a.acl = vm.computeCreateAddress(deployer, startNonce + 1);
        a.fhevmExecutor = vm.computeCreateAddress(deployer, startNonce + 3);
        a.kmsVerifier = vm.computeCreateAddress(deployer, startNonce + 4);
        a.inputVerifier = vm.computeCreateAddress(deployer, startNonce + 5);
        a.hcuLimit = vm.computeCreateAddress(deployer, startNonce + 6);

        // // The forge-fhevm mode:
        // a.acl = vm.computeCreateAddress(deployer, startNonce + 1);
        // a.fhevmExecutor = vm.computeCreateAddress(deployer, startNonce + 3);
        // a.kmsVerifier = vm.computeCreateAddress(deployer, startNonce + 5);
        // a.inputVerifier = vm.computeCreateAddress(deployer, startNonce + 7);
        // a.hcuLimit = vm.computeCreateAddress(deployer, startNonce + 9);
        // a.pauserSet = vm.computeCreateAddress(deployer, startNonce + 10);
    }

    /// Writes a Solidity file with six address constants matching the layout
    /// imported by the host contracts (aclAdd, fhevmExecutorAdd, etc).
    function writeAddressesFile(Vm vm, string memory path, FhevmAddresses memory a) internal {
        string memory content = string.concat(
            "// SPDX-License-Identifier: BSD-3-Clause-Clear\n",
            "\n",
            "pragma solidity ^0.8.24;\n",
            "\n",
            "// Auto-generated by script/ComputeAddresses.s.sol - do not edit by hand.\n",
            "\n",
            "address constant aclAdd = address(",
            vm.toString(a.acl),
            ");\n",
            "\n",
            "address constant fhevmExecutorAdd = address(",
            vm.toString(a.fhevmExecutor),
            ");\n",
            "\n",
            "address constant kmsVerifierAdd = address(",
            vm.toString(a.kmsVerifier),
            ");\n",
            "\n",
            "address constant inputVerifierAdd = address(",
            vm.toString(a.inputVerifier),
            ");\n",
            "\n",
            "address constant hcuLimitAdd = address(",
            vm.toString(a.hcuLimit),
            ");\n",
            "\n",
            "address constant pauserSetAdd = address(",
            vm.toString(a.pauserSet),
            ");\n"
        );
        // forge-lint: disable-next-line(unsafe-cheatcode)
        vm.writeFile(path, content);
    }

    function newEmptyUUPSProxyACL() internal returns (address) {
        return address(new EmptyUUPSProxyACL());
    }

    /// Plain CREATE from the current broadcaster (consumes one nonce).
    function newEmptyUUPSProxy() internal returns (address) {
        return address(new EmptyUUPSProxy());
    }

    function deployACLAt(address deployer, address aclAddress, address emptyUupsProxyACL) internal {
        ERC1967Proxy aclProxy =
            new ERC1967Proxy(emptyUupsProxyACL, abi.encodeCall(EmptyUUPSProxyACL.initialize, (deployer)));
        require(address(aclProxy) == aclAddress, "DeployFHEVMHost: ACL proxy address mismatch");
        console.log("ACL empty proxy:           ", address(aclProxy));
    }

    function setACLImplementation(address aclAddress, address aclImplementation) internal {
        UUPSUpgradeable(aclAddress)
            .upgradeToAndCall(address(aclImplementation), abi.encodeCall(ACL.initializeFromEmptyProxy, ()));
        console.log("CleartextACL upgraded:           ", aclAddress);
    }

    function deployFHEVMExecutorAt(address fhevmExecutorAddress, address emptyUupsProxy) internal {
        ERC1967Proxy fhevmProxy =
            new ERC1967Proxy(address(emptyUupsProxy), abi.encodeCall(EmptyUUPSProxy.initialize, ()));
        require(address(fhevmProxy) == fhevmExecutorAddress, "DeployFHEVMHost: FHEVMExecutor proxy address mismatch");
        console.log("FHEVMExecutor empty proxy: ", address(fhevmProxy));
    }

    function setFHEVMExecutorImplementation(address fhevmExecutorAddress, address fhevmImplementation) internal {
        UUPSUpgradeable(fhevmExecutorAddress)
            .upgradeToAndCall(address(fhevmImplementation), abi.encodeCall(FHEVMExecutor.initializeFromEmptyProxy, ()));
        console.log("FHEVMExecutor upgraded:           ", fhevmExecutorAddress);
    }

    function deployKMSVerifierAt(address kmsVerifierAddress, address emptyUupsProxy) internal {
        ERC1967Proxy fhevmProxy =
            new ERC1967Proxy(address(emptyUupsProxy), abi.encodeCall(EmptyUUPSProxy.initialize, ()));
        require(address(fhevmProxy) == kmsVerifierAddress, "DeployFHEVMHost: KMSVerifier proxy address mismatch");
        console.log("KMSVerifier empty proxy: ", address(fhevmProxy));
    }

    function setKMSVerifierImplementation(
        address kmsVerifierAddress,
        address kmsImplementation,
        address[] memory kmsSigners,
        uint256 kmsThreshold,
        address decryptionAddress,
        uint64 chainIdGateway
    ) internal {
        UUPSUpgradeable(kmsVerifierAddress)
            .upgradeToAndCall(
                kmsImplementation,
                abi.encodeCall(
                    KMSVerifier.initializeFromEmptyProxy, (decryptionAddress, chainIdGateway, kmsSigners, kmsThreshold)
                )
            );
        console.log("KMSVerifier upgraded: ", kmsVerifierAddress);
    }

    function deployInputVerifierAt(address inputVerifierAddress, address emptyUupsProxy) internal {
        ERC1967Proxy fhevmProxy =
            new ERC1967Proxy(address(emptyUupsProxy), abi.encodeCall(EmptyUUPSProxy.initialize, ()));
        require(address(fhevmProxy) == inputVerifierAddress, "DeployFHEVMHost: InputVerifier proxy address mismatch");
        console.log("InputVerifier empty proxy: ", address(fhevmProxy));
    }

    function setInputVerifierImplementation(
        address inputVerifierAddress,
        address inputImplementation,
        address[] memory coprocessorSigners,
        uint256 coprocessorThreshold,
        address inputVerificationAddress,
        uint64 chainIdGateway
    ) internal {
        UUPSUpgradeable(inputVerifierAddress)
            .upgradeToAndCall(
                inputImplementation,
                abi.encodeCall(
                    InputVerifier.initializeFromEmptyProxy,
                    (inputVerificationAddress, chainIdGateway, coprocessorSigners, coprocessorThreshold)
                )
            );
        console.log("InputVerifier upgraded: ", inputVerifierAddress);
    }

    function deployHCULimitAt(address hcuLimitAddress, address emptyUupsProxy) internal {
        ERC1967Proxy fhevmProxy =
            new ERC1967Proxy(address(emptyUupsProxy), abi.encodeCall(EmptyUUPSProxy.initialize, ()));
        require(address(fhevmProxy) == hcuLimitAddress, "DeployFHEVMHost: HCULimit proxy address mismatch");
        console.log("HCULimit empty proxy: ", address(fhevmProxy));
    }

    function setHCULimitImplementation(
        address hcuLimitAddress,
        address hcuImplementation,
        uint48 hcuCapPerBlock,
        uint48 maxHCUDepthPerTx,
        uint48 maxHCUPerTx
    ) internal {
        UUPSUpgradeable(hcuLimitAddress)
            .upgradeToAndCall(
                hcuImplementation,
                abi.encodeCall(HCULimit.initializeFromEmptyProxy, (hcuCapPerBlock, maxHCUDepthPerTx, maxHCUPerTx))
            );
        console.log("InputVerifier upgraded: ", hcuLimitAddress);
    }

    function deployPauserSetAt(address pauserSetAddress) internal {
        PauserSet ps = new PauserSet();
        require(address(ps) == pauserSetAddress, "DeployFHEVMHost: PauserSet address mismatch");
        console.log("PauserSet:                 ", address(ps));
    }

    function addPausers(address pauserSetAddress, address[] memory pausers) internal {
        PauserSet ps = PauserSet(pauserSetAddress);

        for (uint256 i = 0; i < pausers.length; i++) {
            ps.addPauser(pausers[i]);
            console.log("Pauser added:              ", pausers[i]);
        }
    }

    function deployFHETestAt(
        address fheTestAddress,
        address aclAddress,
        address fhevmExecutorAddress,
        address kmsVerifierAddress
    ) internal {
        FHETest fheTest = new FHETest();
        require(address(fheTest) == fheTestAddress, "DeployFHEVMHost: FHETest address mismatch");

        fheTest.setCoprocessorConfig(
            CoprocessorConfig({
                ACLAddress: aclAddress, CoprocessorAddress: fhevmExecutorAddress, KMSVerifierAddress: kmsVerifierAddress
            })
        );

        console.log("FHETest deployed:          ", address(fheTest));
        //fheTest.initFheTest(true);
    }

    function deployFhevmCheats(Vm vm, FhevmAddresses memory addresses, address fheTest) internal {
        vm.etch(FHEVM_CHEATS_ADDRESS, type(FhevmCheats).runtimeCode);
        vm.label(FHEVM_CHEATS_ADDRESS, "FhevmCheats");
        FhevmCheats(FHEVM_CHEATS_ADDRESS).setAll(addresses, fheTest);
    }
}
