// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {VmSafe} from "forge-std/Vm.sol";
import {console2} from "forge-std/console2.sol";
import {console} from "forge-std/console.sol";

import {EmptyUUPSProxyACL} from "@fhevm/host-contracts/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {EmptyUUPSProxy} from "@fhevm/host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ACL} from "@fhevm/host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "@fhevm/host-contracts/contracts/FHEVMExecutor.sol";
import {InputVerifier} from "@fhevm/host-contracts/contracts/InputVerifier.sol";
import {KMSVerifier} from "@fhevm/host-contracts/contracts/KMSVerifier.sol";
import {HCULimit} from "@fhevm/host-contracts/contracts/HCULimit.sol";
import {PauserSet} from "@fhevm/host-contracts/contracts/immutable/PauserSet.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import {CleartextACL} from "@fhevm/cleartext/CleartextACL.sol";
import {CleartextKMSVerifier} from "@fhevm/cleartext/CleartextKMSVerifier.sol";
import {CleartextFHEVMExecutor} from "@fhevm/cleartext/CleartextFHEVMExecutor.sol";
import {CleartextInputVerifier} from "@fhevm/cleartext/CleartextInputVerifier.sol";
import {CleartextHCULimit} from "@fhevm/cleartext/CleartextHCULimit.sol";

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
    pauserSetAdd
} from "@fhevm/host-contracts/addresses/FHEVMHostAddresses.sol";

library DeployLib {
    
    function preComputeAddressesAuto(VmSafe vm, bool verify) internal returns (FhevmAddresses memory expectedAddresses) {
        Signer memory deployer = FhevmConfigLib.resolveDeployerFromEnv(vm);

        if (FhevmConfigLib.canResolveEmptyUupsDeployerFromEnv(vm)) {
            Signer memory emptyUupsDeployer = FhevmConfigLib.resolveEmptyUupsDeployerFromEnv(vm);
            expectedAddresses = preComputeAddressesTwoKeys(vm, deployer.privateKey, verify);
        } else {
            expectedAddresses =
                preComputeAddressesSingleKey(vm, deployer.privateKey, vm.getNonce(deployer.addr), verify);
        }
    }

    function deployAuto(VmSafe vm) internal {
        Signer memory deployer = FhevmConfigLib.resolveDeployerFromEnv(vm);
        
        FhevmAddresses memory expectedAddresses;
        
        if (FhevmConfigLib.canResolveEmptyUupsDeployerFromEnv(vm)) {
            Signer memory emptyUupsDeployer = FhevmConfigLib.resolveEmptyUupsDeployerFromEnv(vm);
            expectedAddresses = preComputeAddressesTwoKeys(vm, deployer.privateKey, true);
            deployTwoKeys(vm, deployer.privateKey, emptyUupsDeployer.privateKey, expectedAddresses);
        } else {
            expectedAddresses =
                preComputeAddressesSingleKey(vm, deployer.privateKey, vm.getNonce(deployer.addr), true);
            deploySingleKey(vm, deployer.privateKey, expectedAddresses);
        }

        verifyDeploy(expectedAddresses);
    }

    /// Two-key flow. Caller must ensure the deployer is at nonce 0;
    /// produces addresses at 0..6
    function deployTwoKeys(
        VmSafe vm,
        uint256 deployerPrivateKey,
        uint256 emptyUupsDeployerPrivateKey,
        FhevmAddresses memory expectedAddresses
    ) internal {
        _pass1AsHostContracts(vm, deployerPrivateKey, emptyUupsDeployerPrivateKey, expectedAddresses);
        _passe2ApplyCleartextImplementations(vm, deployerPrivateKey, expectedAddresses);
    }

    /// Single-key linear flow. Adapts to the deployer's current nonce N;
    /// produces addresses at N+0..N+10
    function deploySingleKey(VmSafe vm, uint256 deployerPrivateKey, FhevmAddresses memory expectedAddresses) internal {
        _pass1Linear(vm, deployerPrivateKey, expectedAddresses);
        _passe2ApplyCleartextImplementations(vm, deployerPrivateKey, expectedAddresses);
    }

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
    /// Use after `preComputeAddressesTwoKeys` (or any flow expected to land
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
    }

    function preComputeAddressesSingleKey(VmSafe vm, uint256 deployerPrivateKey, uint256 startNonce, bool verify)
        internal
        returns (FhevmAddresses memory a)
    {
        require(deployerPrivateKey != 0, AssertLib.boxMessage("Missing deployer private key"));

        address deployer = vm.addr(deployerPrivateKey);

        a.acl = vm.computeCreateAddress(deployer, startNonce + 1);
        a.fhevmExecutor = vm.computeCreateAddress(deployer, startNonce + 3);
        a.kmsVerifier = vm.computeCreateAddress(deployer, startNonce + 5);
        a.inputVerifier = vm.computeCreateAddress(deployer, startNonce + 7);
        a.hcuLimit = vm.computeCreateAddress(deployer, startNonce + 9);
        a.pauserSet = vm.computeCreateAddress(deployer, startNonce + 10);

        a.verifyingContractAddressDecryption = FhevmConfigLib.resolveDecryptionAddressFromEnv(vm);
        a.verifyingContractAddressInputVerification = FhevmConfigLib.resolveInputVerificationAddressFromEnv(vm);
        a.pausers = FhevmConfigLib.resolvePausersFromEnv(vm);

        if (verify) {
            verifyAgainstFHEVMHostAddresses(a);
        }
    }

    function preComputeAddressesTwoKeys(VmSafe vm, uint256 deployerPrivateKey, bool verify)
        internal
        returns (FhevmAddresses memory a)
    {
        require(deployerPrivateKey != 0, AssertLib.boxMessage("Missing deployer private key"));

        address deployer = vm.addr(deployerPrivateKey);

        a.pauserSet = vm.computeCreateAddress(deployer, 0);
        a.acl = vm.computeCreateAddress(deployer, 1);
        a.fhevmExecutor = vm.computeCreateAddress(deployer, 3);
        a.kmsVerifier = vm.computeCreateAddress(deployer, 4);
        a.inputVerifier = vm.computeCreateAddress(deployer, 5);
        a.hcuLimit = vm.computeCreateAddress(deployer, 6);

        a.verifyingContractAddressDecryption = FhevmConfigLib.resolveDecryptionAddressFromEnv(vm);
        a.verifyingContractAddressInputVerification = FhevmConfigLib.resolveInputVerificationAddressFromEnv(vm);
        a.pausers = FhevmConfigLib.resolvePausersFromEnv(vm);

        if (verify) {
            verifyAgainstFHEVMHostAddresses(a);
        }
    }

    /// Two-key flow (matches the FHEVM host-contracts repo layout):
    ///   - `emptyUupsDeployer` deploys a single shared `EmptyUUPSProxyACL` impl.
    ///   - `deployer` (linear nonces) deploys, in order:
    ///       0. PauserSet
    ///       1. ACL proxy
    ///       2. shared EmptyUUPSProxy impl
    ///       3. FHEVMExecutor proxy
    ///       4. KMSVerifier proxy
    ///       5. InputVerifier proxy
    ///       6. HCULimit proxy
    /// Requires the `deployer` to start at nonce 0 so CREATE addresses match
    /// the constants committed in `FHEVMHostAddresses.sol`.
    function _pass1AsHostContracts(
        VmSafe vm,
        uint256 deployerPrivateKey,
        uint256 emptyUupsDeployerPrivateKey,
        FhevmAddresses memory expectedAddresses
    ) private {
        require(
            deployerPrivateKey != emptyUupsDeployerPrivateKey,
            AssertLib.boxMessage("DeployLib: deployer and emptyUupsDeployer must be different keys")
        );

        EmptyUUPSProxyACL emptyUupsProxyACL;

        vm.startBroadcast(emptyUupsDeployerPrivateKey);
        {
            emptyUupsProxyACL = new EmptyUUPSProxyACL();
        }
        vm.stopBroadcast();

        vm.startBroadcast(deployerPrivateKey);
        {
            address deployer = vm.addr(deployerPrivateKey);

            // 0. PauserSet (nonce 0)
            require(vm.getNonce(deployer) == 0, AssertLib.boxMessage("Expecting nonce=0"));
            deployPauserSetAt(expectedAddresses.pauserSet);

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
        }
        vm.stopBroadcast();
    }

    /// Single-key linear flow. Per-slot `EmptyUUPSProxy` CREATEs — produces
    /// 11 sequential nonces starting at the deployer's current nonce N:
    ///   N+0  EmptyUUPSProxyACL impl
    ///   N+1  ACL proxy
    ///   N+2  EmptyUUPSProxy (FHEVMExecutor slot)
    ///   N+3  FHEVMExecutor proxy
    ///   N+4  EmptyUUPSProxy (KMSVerifier slot)
    ///   N+5  KMSVerifier proxy
    ///   N+6  EmptyUUPSProxy (InputVerifier slot)
    ///   N+7  InputVerifier proxy
    ///   N+8  EmptyUUPSProxy (HCULimit slot)
    ///   N+9  HCULimit proxy
    ///   N+10 PauserSet
    function _pass1Linear(VmSafe vm, uint256 deployerPrivateKey, FhevmAddresses memory expectedAddresses) private {
        vm.startBroadcast(deployerPrivateKey);
        {
            address deployer = vm.addr(deployerPrivateKey);
            uint64 startNonce = vm.getNonce(deployer);

            // N+0. EmptyUUPSProxyACL impl
            EmptyUUPSProxyACL emptyUupsProxyACL = new EmptyUUPSProxyACL();

            // N+1. ACL proxy
            require(vm.getNonce(deployer) == startNonce + 1, AssertLib.boxMessage("Expecting startNonce+1"));
            deployERC1967ProxyAt("ACL", deployer, emptyUupsProxyACL, expectedAddresses.acl);

            // N+2. EmptyUUPSProxy (FHEVMExecutor slot)
            require(vm.getNonce(deployer) == startNonce + 2, AssertLib.boxMessage("Expecting startNonce+2"));
            EmptyUUPSProxy emptyUupsProxy = new EmptyUUPSProxy();

            // N+3. FHEVMExecutor proxy
            require(vm.getNonce(deployer) == startNonce + 3, AssertLib.boxMessage("Expecting startNonce+3"));
            deployERC1967ProxyAt("FHEVMExecutor", emptyUupsProxy, expectedAddresses.fhevmExecutor);

            // N+4. EmptyUUPSProxy (KMSVerifier slot) — reassign, do not redeclare.
            require(vm.getNonce(deployer) == startNonce + 4, AssertLib.boxMessage("Expecting startNonce+4"));
            emptyUupsProxy = new EmptyUUPSProxy();

            // N+5. KMSVerifier proxy
            require(vm.getNonce(deployer) == startNonce + 5, AssertLib.boxMessage("Expecting startNonce+5"));
            deployERC1967ProxyAt("KMSVerifier", emptyUupsProxy, expectedAddresses.kmsVerifier);

            // N+6. EmptyUUPSProxy (InputVerifier slot)
            require(vm.getNonce(deployer) == startNonce + 6, AssertLib.boxMessage("Expecting startNonce+6"));
            emptyUupsProxy = new EmptyUUPSProxy();

            // N+7. InputVerifier proxy
            require(vm.getNonce(deployer) == startNonce + 7, AssertLib.boxMessage("Expecting startNonce+7"));
            deployERC1967ProxyAt("InputVerifier", emptyUupsProxy, expectedAddresses.inputVerifier);

            // N+8. EmptyUUPSProxy (HCULimit slot)
            require(vm.getNonce(deployer) == startNonce + 8, AssertLib.boxMessage("Expecting startNonce+8"));
            emptyUupsProxy = new EmptyUUPSProxy();

            // N+9. HCULimit proxy
            require(vm.getNonce(deployer) == startNonce + 9, AssertLib.boxMessage("Expecting startNonce+9"));
            deployERC1967ProxyAt("HCULimit", emptyUupsProxy, expectedAddresses.hcuLimit);

            // N+10. PauserSet
            require(vm.getNonce(deployer) == startNonce + 10, AssertLib.boxMessage("Expecting startNonce+10"));
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
        Signer[] memory kmsSigners = FhevmConfigLib.resolveKmsSignersFromEnv(vm);
        uint256 kmsThreshold = FhevmConfigLib.resolveKmsThresholdFromEnv(vm);
        address decryptionAddr = FhevmConfigLib.resolveDecryptionAddressFromEnv(vm);

        address[] memory signersAddr = SignerLib.addressesOf(kmsSigners);

        UUPSUpgradeable(kmsVerifierAddress)
            .upgradeToAndCall(
                address(kmsImplementation),
                abi.encodeCall(
                    KMSVerifier.initializeFromEmptyProxy, (decryptionAddr, chainIdGateway, signersAddr, kmsThreshold)
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

            addPausers(vm, fhevmAddresses.pauserSet, fhevmAddresses.pausers);
        }
        vm.stopBroadcast();
    }

    function addPausers(VmSafe vm, address pauserSetAddress, Signer[] memory pausers) private {
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
            "address constant pauserSetAdd = address(",
            vm.toString(a.pauserSet),
            ");\n"
        );
        // forge-lint: disable-next-line(unsafe-cheatcode)
        vm.writeFile(path, content);
    }


}
