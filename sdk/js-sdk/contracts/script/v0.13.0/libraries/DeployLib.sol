// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {VmSafe} from "forge-std/Vm.sol";
import {console} from "forge-std/console.sol";

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import {EmptyUUPSProxyACL} from "../../../src/v0.13.0/host-contracts/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {EmptyUUPSProxy} from "../../../src/v0.13.0/host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ACL} from "../../../src/v0.13.0/host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "../../../src/v0.13.0/host-contracts/contracts/FHEVMExecutor.sol";
import {InputVerifier} from "../../../src/v0.13.0/host-contracts/contracts/InputVerifier.sol";
import {KMSVerifier} from "../../../src/v0.13.0/host-contracts/contracts/KMSVerifier.sol";
import {HCULimit} from "../../../src/v0.13.0/host-contracts/contracts/HCULimit.sol";
import {PauserSet} from "../../../src/v0.13.0/host-contracts/contracts/immutable/PauserSet.sol";
import {ProtocolConfig} from "../../../src/v0.13.0/host-contracts/contracts/ProtocolConfig.sol";
import {KMSGeneration} from "../../../src/v0.13.0/host-contracts/contracts/KMSGeneration.sol";
import {KmsNode} from "../../../src/v0.13.0/host-contracts/contracts/shared/Structs.sol";
import {IProtocolConfig} from "../../../src/v0.13.0/host-contracts/contracts/interfaces/IProtocolConfig.sol";

import {CleartextACL} from "../../../src/v0.13.0/cleartext/CleartextACL.sol";
import {CleartextKMSVerifier} from "../../../src/v0.13.0/cleartext/CleartextKMSVerifier.sol";
import {CleartextFHEVMExecutor} from "../../../src/v0.13.0/cleartext/CleartextFHEVMExecutor.sol";
import {CleartextInputVerifier} from "../../../src/v0.13.0/cleartext/CleartextInputVerifier.sol";
import {CleartextHCULimit} from "../../../src/v0.13.0/cleartext/CleartextHCULimit.sol";
import {CleartextProtocolConfig} from "../../../src/v0.13.0/cleartext/CleartextProtocolConfig.sol";
import {CleartextKMSGeneration} from "../../../src/v0.13.0/cleartext/CleartextKMSGeneration.sol";

import {FhevmAddresses} from "./structs/FhevmAddressesStruct.sol";
import {AssertLib} from "./AssertLib.sol";
import {Signer, SignerLib} from "./SignerLib.sol";
import {FhevmConfigLib} from "./FhevmConfigLib.sol";

import {
    aclAdd,
    fhevmExecutorAdd,
    kmsVerifierAdd,
    inputVerifierAdd,
    hcuLimitAdd,
    pauserSetAdd,
    kmsGenerationAdd,
    protocolConfigAdd
} from "../../../src/v0.13.0/host-contracts/addresses/FHEVMHostAddresses.sol";

library DeployLib {
    
    function preComputeAddresses(VmSafe vm, bool verify) internal view returns (FhevmAddresses memory expectedAddresses) {
        Signer memory deployer = FhevmConfigLib.resolveDeployerFromEnv(vm);
        expectedAddresses = _preComputeAddresses(vm, deployer.privateKey, verify);
    }

    function _preComputeAddresses(VmSafe vm, uint256 deployerPrivateKey, bool verify)
        private view
        returns (FhevmAddresses memory a)
    {
        require(deployerPrivateKey != 0, AssertLib.boxMessage("Missing deployer private key"));

        address deployer = vm.addr(deployerPrivateKey);

        a.acl = vm.computeCreateAddress(deployer, 1);
        a.fhevmExecutor = vm.computeCreateAddress(deployer, 3);
        a.kmsVerifier = vm.computeCreateAddress(deployer, 4);
        a.inputVerifier = vm.computeCreateAddress(deployer, 5);
        a.hcuLimit = vm.computeCreateAddress(deployer, 6);
        a.protocolConfig = vm.computeCreateAddress(deployer, 7);
        a.kmsGeneration = vm.computeCreateAddress(deployer, 8);
        a.pauserSet = vm.computeCreateAddress(deployer, 9);

        a.verifyingContractAddressDecryption = FhevmConfigLib.resolveDecryptionAddressFromEnv(vm);
        a.verifyingContractAddressInputVerification = FhevmConfigLib.resolveInputVerificationAddressFromEnv(vm);
        a.pausers = FhevmConfigLib.resolvePausersFromEnv(vm);

        if (verify) {
            verifyAgainstFHEVMHostAddresses(a);
        }
    }

    function deployAuto(VmSafe vm) internal {
        Signer memory deployer = FhevmConfigLib.resolveDeployerFromEnv(vm);
        
        FhevmAddresses memory expectedAddresses = _preComputeAddresses(vm, deployer.privateKey, true);

        _deployEmptyProxies(vm, deployer.privateKey, expectedAddresses);
        _passe2ApplyCleartextImplementations(vm, deployer.privateKey, expectedAddresses);
        
        verifyDeploy(expectedAddresses);
    }

    /// Two-key flow. Caller must ensure the deployer is at nonce 0;
    /// produces addresses at 0..6
    // function deployTwoKeys(
    //     VmSafe vm,
    //     uint256 deployerPrivateKey,
    //     FhevmAddresses memory expectedAddresses
    // ) internal {
    //     _pass1AsHostContracts(vm, deployerPrivateKey, expectedAddresses);
    //     _passe2ApplyCleartextImplementations(vm, deployerPrivateKey, expectedAddresses);
    // }

    /// Verifies that the contracts deployed at the addresses in `a` reference
    /// each other consistently — i.e. on-chain getters return the addresses
    /// the caller expects. Reverts with a banner naming the first mismatch.
    ///
    /// Cross-references checked (all by reading the live chain):
    ///   ACL(a.acl).getFHEVMExecutorAddress()                     == a.fhevmExecutor
    ///   ACL(a.acl).getPauserSetAddress()                         == a.pauserSet
    ///   FHEVMExecutor(a.fhevmExecutor).getACLAddress()           == a.acl
    ///   FHEVMExecutor(a.fhevmExecutor).getHCULimitAddress()      == a.hcuLimit
    ///   FHEVMExecutor(a.fhevmExecutor).getInputVerifierAddress() == a.inputVerifier
    ///   HCULimit(a.hcuLimit).getFHEVMExecutorAddress()           == a.fhevmExecutor
    ///
    /// KMSVerifier and InputVerifier don't expose sister-contract getters; if
    /// you want to verify their config (threshold / signer list), check
    /// `getThreshold()` / `getKmsSigners()` / `getCoprocessorSigners()`
    /// separately.
    function verifyDeploy(FhevmAddresses memory a) internal view {
        require(
            ACL(a.acl).getFHEVMExecutorAddress() == a.fhevmExecutor,
            AssertLib.boxMessage("ACL.getFHEVMExecutorAddress() != a.fhevmExecutor")
        );
        require(
            ACL(a.acl).getPauserSetAddress() == a.pauserSet,
            AssertLib.boxMessage("ACL.getPauserSetAddress() != a.pauserSet")
        );

        FHEVMExecutor exec = FHEVMExecutor(a.fhevmExecutor);
        require(exec.getACLAddress() == a.acl, AssertLib.boxMessage("FHEVMExecutor.getACLAddress() != a.acl"));
        require(
            exec.getHCULimitAddress() == a.hcuLimit,
            AssertLib.boxMessage("FHEVMExecutor.getHCULimitAddress() != a.hcuLimit")
        );
        require(
            exec.getInputVerifierAddress() == a.inputVerifier,
            AssertLib.boxMessage("FHEVMExecutor.getInputVerifierAddress() != a.inputVerifier")
        );

        require(
            HCULimit(a.hcuLimit).getFHEVMExecutorAddress() == a.fhevmExecutor,
            AssertLib.boxMessage("HCULimit.getFHEVMExecutorAddress() != a.fhevmExecutor")
        );
    }

    /// Verifies that the six host-stack addresses on `a` match the constants
    /// committed in `FHEVMHostAddresses.sol`. Reverts with a banner-formatted
    /// message naming the first field that doesn't match.
    ///
    /// Use after `preComputeAddresses1` (or any flow expected to land
    /// on the canonical host-contracts addresses) to catch drift between the
    /// computed layout and the hardcoded constants — e.g. when the deployer
    /// mnemonic / index changed but `FHEVMHostAddresses.sol` wasn't regenerated.
    function verifyAgainstFHEVMHostAddresses(FhevmAddresses memory a) internal pure {
        require(a.acl == aclAdd, AssertLib.boxMessage("FhevmAddresses.acl != FHEVMHostAddresses.aclAdd"));
        require(
            a.fhevmExecutor == fhevmExecutorAdd,
            AssertLib.boxMessage("FhevmAddresses.fhevmExecutor != FHEVMHostAddresses.fhevmExecutorAdd")
        );
        require(
            a.kmsVerifier == kmsVerifierAdd,
            AssertLib.boxMessage("FhevmAddresses.kmsVerifier != FHEVMHostAddresses.kmsVerifierAdd")
        );
        require(
            a.inputVerifier == inputVerifierAdd,
            AssertLib.boxMessage("FhevmAddresses.inputVerifier != FHEVMHostAddresses.inputVerifierAdd")
        );
        require(
            a.hcuLimit == hcuLimitAdd,
            AssertLib.boxMessage("FhevmAddresses.hcuLimit != FHEVMHostAddresses.hcuLimitAdd")
        );
        require(
            a.pauserSet == pauserSetAdd,
            AssertLib.boxMessage("FhevmAddresses.pauserSet != FHEVMHostAddresses.pauserSetAdd")
        );
        require(
            a.protocolConfig == protocolConfigAdd,
            AssertLib.boxMessage("FhevmAddresses.protocolConfig != FHEVMHostAddresses.protocolConfigAdd")
        );
        require(
            a.kmsGeneration == kmsGenerationAdd,
            AssertLib.boxMessage("FhevmAddresses.kmsGeneration != FHEVMHostAddresses.kmsGenerationAdd")
        );
    }

    /// Two-key flow (matches the FHEVM host-contracts repo layout):
    ///   - `emptyUupsDeployer` deploys a single shared `EmptyUUPSProxyACL` impl.
    ///   - `deployer` (linear nonces) deploys, in order:
    ///       1. ACL proxy
    ///       2. shared EmptyUUPSProxy impl
    ///       3. FHEVMExecutor proxy
    ///       4. KMSVerifier proxy
    ///       5. InputVerifier proxy
    ///       6. HCULimit proxy
    ///       7. ProtocolConfig proxy
    ///       8. KmsGeneration proxy
    ///       9. PauserSet
    /// Requires the `deployer` to start at nonce 0 so CREATE addresses match
    /// the constants committed in `FHEVMHostAddresses.sol`.
    function _deployEmptyProxies(
        VmSafe vm,
        uint256 deployerPrivateKey,
        FhevmAddresses memory expectedAddresses
    ) private {
        vm.startBroadcast(deployerPrivateKey);
        {
            address deployer = vm.addr(deployerPrivateKey);

            EmptyUUPSProxyACL emptyUupsProxyACL = new EmptyUUPSProxyACL();

            // 1. ACL proxy (nonce 1) — uses the EmptyUUPSProxyACL impl from the secondary key.
            require(vm.getNonce(deployer) == 1, AssertLib.boxMessage("Expecting nonce=1"));
            deployERC1967ProxyAt("ACL", deployer, emptyUupsProxyACL, expectedAddresses.acl);

            // 2. Shared EmptyUUPSProxy (nonce 2) — reused by FHEVMExecutor / KMSVerifier / InputVerifier / HCULimit.
            require(vm.getNonce(deployer) == 2, AssertLib.boxMessage("Expecting nonce=2"));
            EmptyUUPSProxy emptyUupsProxy = new EmptyUUPSProxy();

            // 3. FHEVMExecutor proxy (nonce 3)
            require(vm.getNonce(deployer) == 3, AssertLib.boxMessage("Expecting nonce=3"));
            deployERC1967ProxyAt("FHEVMExecutor", emptyUupsProxy, expectedAddresses.fhevmExecutor);

            // 4. KMSVerifier proxy (nonce 4)
            require(vm.getNonce(deployer) == 4, AssertLib.boxMessage("Expecting nonce=4"));
            deployERC1967ProxyAt("KMSVerifier", emptyUupsProxy, expectedAddresses.kmsVerifier);

            // 5. InputVerifier proxy (nonce 5)
            require(vm.getNonce(deployer) == 5, AssertLib.boxMessage("Expecting nonce=5"));
            deployERC1967ProxyAt("InputVerifier", emptyUupsProxy, expectedAddresses.inputVerifier);

            // 6. HCULimit proxy (nonce 6)
            require(vm.getNonce(deployer) == 6, AssertLib.boxMessage("Expecting nonce=6"));
            deployERC1967ProxyAt("HCULimit", emptyUupsProxy, expectedAddresses.hcuLimit);

            // 7. ProtocolConfig proxy (nonce 7)
            require(vm.getNonce(deployer) == 7, AssertLib.boxMessage("Expecting nonce=7"));
            deployERC1967ProxyAt("ProtocolConfig", emptyUupsProxy, expectedAddresses.protocolConfig);

            // 8. KmsGeneration proxy (nonce 8)
            require(vm.getNonce(deployer) == 8, AssertLib.boxMessage("Expecting nonce=8"));
            deployERC1967ProxyAt("KmsGeneration", emptyUupsProxy, expectedAddresses.kmsGeneration);

            // 9. PauserSet proxy (nonce 9)
            require(vm.getNonce(deployer) == 9, AssertLib.boxMessage("Expecting nonce=9"));
            deployPauserSetAt(expectedAddresses.pauserSet);
        }
        vm.stopBroadcast();
    }

    function deployPauserSetAt(address expectedAddress) private {
        PauserSet ps = new PauserSet();
        if (expectedAddress != address(0)) {
            require(address(ps) == expectedAddress, AssertLib.boxMessage("DeployLib: PauserSet address mismatch"));
        }
        console.log("PauserSet:                 ", address(ps));
    }

    function deployERC1967ProxyAt(
        string memory name,
        address deployer,
        EmptyUUPSProxyACL emptyUupsProxyACL,
        address expectedAddress
    ) private {
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(emptyUupsProxyACL), abi.encodeCall(EmptyUUPSProxyACL.initialize, (deployer))
        );

        if (expectedAddress != address(0)) {
            require(
                address(proxy) == expectedAddress,
                AssertLib.boxMessage(string.concat("ERC1967Proxy: ", name, " proxy address mismatch"))
            );
        }
        console.log(string.concat(name, " empty proxy:           "), address(proxy));
    }

    function deployERC1967ProxyAt(string memory name, EmptyUUPSProxy emptyUupsProxy, address expectedAddress) private {
        ERC1967Proxy proxy = new ERC1967Proxy(address(emptyUupsProxy), abi.encodeCall(EmptyUUPSProxy.initialize, ()));

        if (expectedAddress != address(0)) {
            require(
                address(proxy) == expectedAddress,
                AssertLib.boxMessage(string.concat("ERC1967Proxy: ", name, " proxy address mismatch"))
            );
        }
        console.log(string.concat(name, " empty proxy:           "), address(proxy));
    }

    // ======== Implementation ========

    function setACLImplementation(VmSafe, address aclAddress, ACL aclImplementation) internal {
        UUPSUpgradeable(aclAddress)
            .upgradeToAndCall(address(aclImplementation), abi.encodeCall(ACL.initializeFromEmptyProxy, ()));
    }

    function setFHEVMExecutorImplementation(VmSafe, address fhevmExecutorAddress, FHEVMExecutor fhevmImplementation)
        internal
    {
        UUPSUpgradeable(fhevmExecutorAddress)
            .upgradeToAndCall(address(fhevmImplementation), abi.encodeCall(FHEVMExecutor.initializeFromEmptyProxy, ()));
    }

    function setKMSVerifierImplementation(VmSafe vm, address kmsVerifierAddress, KMSVerifier kmsImplementation)
        internal
    {
        uint64 chainIdGateway = FhevmConfigLib.resolveGatewayChainIdFromEnv(vm);
        address decryptionAddr = FhevmConfigLib.resolveDecryptionAddressFromEnv(vm);

        UUPSUpgradeable(kmsVerifierAddress)
            .upgradeToAndCall(
                address(kmsImplementation),
                abi.encodeCall(
                    KMSVerifier.initializeFromEmptyProxy, (decryptionAddr, chainIdGateway)
                )
            );
    }

    function setInputVerifierImplementation(VmSafe vm, address inputVerifierAddress, InputVerifier inputImplementation)
        internal
    {
        uint64 chainIdGateway = FhevmConfigLib.resolveGatewayChainIdFromEnv(vm);
        Signer[] memory coprocessorSigners = FhevmConfigLib.resolveCoprocessorSignersFromEnv(vm);
        uint256 coprocessorThreshold = FhevmConfigLib.resolveCoprocessorThresholdFromEnv(vm);
        address inputVerificationAddr = FhevmConfigLib.resolveInputVerificationAddressFromEnv(vm);

        address[] memory signersAddr = SignerLib.addressesOf(coprocessorSigners);

        UUPSUpgradeable(inputVerifierAddress)
            .upgradeToAndCall(
                address(inputImplementation),
                abi.encodeCall(
                    InputVerifier.initializeFromEmptyProxy,
                    (inputVerificationAddr, chainIdGateway, signersAddr, coprocessorThreshold)
                )
            );
    }

    function setHCULimitImplementation(VmSafe vm, address hcuLimitAddress, HCULimit hcuImplementation) internal {
        (uint48 hcuCapPerBlock, uint48 maxHCUDepthPerTx, uint48 maxHCUPerTx) =
            FhevmConfigLib.resolveHcuConfigFromEnv(vm);
        UUPSUpgradeable(hcuLimitAddress)
            .upgradeToAndCall(
                address(hcuImplementation),
                abi.encodeCall(HCULimit.initializeFromEmptyProxy, (hcuCapPerBlock, maxHCUDepthPerTx, maxHCUPerTx))
            );
    }

    function setProtocolConfigImplementation(VmSafe vm, address protocolConfigAddress, ProtocolConfig protocolConfigImplementation) internal {
        KmsNode[] memory initialKmsNodes = FhevmConfigLib.resolveKmsNodesFromEnv(vm);
        uint256 kmsThreshold = FhevmConfigLib.resolveKmsThresholdFromEnv(vm);

        IProtocolConfig.KmsThresholds memory initialThresholds;
        initialThresholds.kmsGen = kmsThreshold;
        initialThresholds.mpc = kmsThreshold;
        initialThresholds.userDecryption = kmsThreshold;
        initialThresholds.publicDecryption = kmsThreshold;

        UUPSUpgradeable(protocolConfigAddress)
            .upgradeToAndCall(address(protocolConfigImplementation), abi.encodeCall(ProtocolConfig.initializeFromEmptyProxy, (initialKmsNodes, initialThresholds)));
    }

    function setKmsGenerationImplementation(VmSafe, address kmsGenerationAddress, KMSGeneration kmsGenerationImplementation) internal {
        UUPSUpgradeable(kmsGenerationAddress)
            .upgradeToAndCall(address(kmsGenerationImplementation), abi.encodeCall(KMSGeneration.initializeFromEmptyProxy, ()));
    }


    /// Upgrades all six FHEVM proxies to their cleartext implementations and
    /// registers pausers, all under one `vm.startBroadcast(deployerKey)` block.
    ///
    /// Env vars consumed (via FhevmConfigLib):
    ///   - $CHAIN_ID_GATEWAY
    ///   - $KMS_NODES_MNEMONIC + $NUM_KMS_NODES
    ///   - $COPROCESSORS_MNEMONIC + $NUM_COPROCESSORS
    ///   - $PAUSERS_MNEMONIC + $NUM_PAUSERS
    ///   - $KMS_THRESHOLD, $COPROCESSOR_THRESHOLD
    ///   - $HCU_CAP_PER_BLOCK, $MAX_HCU_DEPTH_PER_TX, $MAX_HCU_PER_TX
    function _passe2ApplyCleartextImplementations(VmSafe vm, uint256 deployerKey, FhevmAddresses memory fhevmAddresses)
        private
    {
        vm.startBroadcast(deployerKey);
        {
            setACLImplementation(vm, fhevmAddresses.acl, new CleartextACL());
            setFHEVMExecutorImplementation(vm, fhevmAddresses.fhevmExecutor, new CleartextFHEVMExecutor());
            setKMSVerifierImplementation(vm, fhevmAddresses.kmsVerifier, new CleartextKMSVerifier());
            setInputVerifierImplementation(vm, fhevmAddresses.inputVerifier, new CleartextInputVerifier());
            setHCULimitImplementation(vm, fhevmAddresses.hcuLimit, new CleartextHCULimit());
            setProtocolConfigImplementation(vm, fhevmAddresses.protocolConfig, new CleartextProtocolConfig());
            setKmsGenerationImplementation(vm, fhevmAddresses.kmsGeneration, new CleartextKMSGeneration());

            addPausers(vm, fhevmAddresses.pauserSet, fhevmAddresses.pausers);
        }
        vm.stopBroadcast();
    }

    function addPausers(VmSafe, address pauserSetAddress, Signer[] memory pausers) private {
        address[] memory pausersAddr = SignerLib.addressesOf(pausers);

        PauserSet ps = PauserSet(pauserSetAddress);

        for (uint256 i = 0; i < pausersAddr.length; i++) {
            ps.addPauser(pausersAddr[i]);
            console.log(string.concat("pauser:           "), pausersAddr[i]);
        }
    }

    /// Writes a Solidity file with six address constants matching the layout
    /// imported by the host contracts (aclAdd, fhevmExecutorAdd, etc).
    function writeAddressesFile(VmSafe vm, string memory path, FhevmAddresses memory a) internal {
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
            "address constant protocolConfigAdd = address(",
            vm.toString(a.protocolConfig),
            ");\n",
            "\n",
            "address constant kmsGenerationAdd = address(",
            vm.toString(a.kmsGeneration),
            ");\n",
            "\n",
            "address constant pauserSetAdd = address(",
            vm.toString(a.pauserSet),
            ");\n"
        );
        // forge-lint: disable-next-line(unsafe-cheatcode)
        vm.writeFile(path, content);
    }


}
