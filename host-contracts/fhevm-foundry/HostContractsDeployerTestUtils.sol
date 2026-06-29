// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "@fhevm-host-contracts/contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "@fhevm-host-contracts/contracts/KMSVerifier.sol";
import {InputVerifier} from "@fhevm-host-contracts/contracts/InputVerifier.sol";
import {HCULimit} from "@fhevm-host-contracts/contracts/HCULimit.sol";
import {PauserSet} from "@fhevm-host-contracts/contracts/immutable/PauserSet.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {EmptyUUPSProxyACL} from "@fhevm-host-contracts/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {KMSGeneration} from "@fhevm-host-contracts/contracts/KMSGeneration.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {IKMSGeneration} from "@fhevm-host-contracts/contracts/interfaces/IKMSGeneration.sol";
import {KmsNode, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {PREP_KEYGEN_COUNTER_BASE, KEY_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {aclAdd, fhevmExecutorAdd, hcuLimitAdd, inputVerifierAdd, kmsVerifierAdd, pauserSetAdd, protocolConfigAdd, kmsGenerationAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

/**
 * @dev Thin wrapper so `deployCodeTo` can load locally compiled bytecode for the OZ proxy.
 * Foundry only exposes artifacts that live inside this repo, hence the re-exposed constructor.
 */
contract DeployableERC1967Proxy is ERC1967Proxy {
    constructor(address implementation, bytes memory data) ERC1967Proxy(implementation, data) {}
}

/**
 * @dev Test harness that reconstructs the on-chain host stack inside Foundry.
 *
 * Host contracts (ACL, FHEVMExecutor, KMS/Input verifiers, HCULimit, PauserSet) are deployed on mainnet
 * behind deterministic UUPS proxies anchored at addresses defined in `FHEVMHostAddresses.sol`. Rather than
 * mocking behaviours piecemeal, this helper redeploys each proxy + implementation pair exactly how production
 * does:
 *  - write the appropriate empty proxy runtime to the canonical address using `deployCodeTo`;
 *  - perform the privileged upgrade calls with the expected initializer payloads;
 *  - label the proxy and implementation for nicer traces.
 *
 * Tests that inherit this contract can call the `_deploy*` helpers to stitch together a realistic environment
 * where cross-contract permission checks (ACLOwnable, slot reads, etc.) behave the same as on-chain.
 */
abstract contract HostContractsDeployerTestUtils is Test {
    bytes32 internal constant EIP712_DOMAIN_TYPE_HASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
    bytes32 internal constant EIP712_KEY_DIGEST_TYPE_HASH = keccak256("KeyDigest(uint8 keyType,bytes digest)");
    bytes32 internal constant EIP712_KEYGEN_TYPE_HASH =
        keccak256(
            "KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)"
        );
    bytes32 internal constant EIP712_CRSGEN_TYPE_HASH =
        keccak256("CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)");

    /// @dev Shared ProtocolConfig handle bound by each suite's setUp; used by the hoisted EIP-712 helpers below.
    ProtocolConfig internal protocolConfig;

    function _deployACL(address owner) internal returns (ACL aclProxy, address aclImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxyACL());

        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxyACL.initialize, (owner))),
            aclAdd
        );
        vm.label(aclAdd, "ACL Proxy");

        aclImplementation = address(new ACL());
        vm.label(aclImplementation, "ACL Implementation");

        vm.prank(owner);
        EmptyUUPSProxyACL(aclAdd).upgradeToAndCall(aclImplementation, abi.encodeCall(ACL.initializeFromEmptyProxy, ()));

        aclProxy = ACL(aclAdd);
    }

    function _deployFHEVMExecutor(
        address owner
    ) internal returns (FHEVMExecutor fhevmExecutorProxy, address fhevmExecutorImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            fhevmExecutorAdd
        );
        vm.label(fhevmExecutorAdd, "FHEVMExecutor Proxy");

        fhevmExecutorImplementation = address(new FHEVMExecutor());
        vm.label(fhevmExecutorImplementation, "FHEVMExecutor Implementation");

        vm.prank(owner);
        EmptyUUPSProxy(fhevmExecutorAdd).upgradeToAndCall(
            fhevmExecutorImplementation,
            abi.encodeCall(FHEVMExecutor.initializeFromEmptyProxy, ())
        );

        fhevmExecutorProxy = FHEVMExecutor(fhevmExecutorAdd);
    }

    function _deployKMSVerifier(
        address owner,
        address verifyingContractSource,
        uint64 chainIDSource
    ) internal returns (KMSVerifier kmsVerifierProxy, address kmsVerifierImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            kmsVerifierAdd
        );
        vm.label(kmsVerifierAdd, "KMSVerifier Proxy");

        kmsVerifierImplementation = address(new KMSVerifier());
        vm.label(kmsVerifierImplementation, "KMSVerifier Implementation");

        vm.prank(owner);
        EmptyUUPSProxy(kmsVerifierAdd).upgradeToAndCall(
            kmsVerifierImplementation,
            abi.encodeCall(KMSVerifier.initializeFromEmptyProxy, (verifyingContractSource, chainIDSource))
        );

        kmsVerifierProxy = KMSVerifier(kmsVerifierAdd);
    }

    function _deployInputVerifier(
        address owner,
        address verifyingContractSource,
        uint64 chainIDSource,
        address[] memory initialSigners,
        uint256 initialThreshold
    ) internal returns (InputVerifier inputVerifierProxy, address inputVerifierImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            inputVerifierAdd
        );
        vm.label(inputVerifierAdd, "InputVerifier Proxy");

        inputVerifierImplementation = address(new InputVerifier());
        vm.label(inputVerifierImplementation, "InputVerifier Implementation");

        vm.prank(owner);
        EmptyUUPSProxy(inputVerifierAdd).upgradeToAndCall(
            inputVerifierImplementation,
            abi.encodeCall(
                InputVerifier.initializeFromEmptyProxy,
                (verifyingContractSource, chainIDSource, initialSigners, initialThreshold)
            )
        );

        inputVerifierProxy = InputVerifier(inputVerifierAdd);
    }

    function _deployHCULimit(address owner) internal returns (HCULimit hcuLimitProxy, address hcuLimitImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            hcuLimitAdd
        );
        vm.label(hcuLimitAdd, "HCULimit Proxy");

        hcuLimitImplementation = address(new HCULimit());
        vm.label(hcuLimitImplementation, "HCULimit Implementation");

        vm.prank(owner);
        EmptyUUPSProxy(hcuLimitAdd).upgradeToAndCall(
            hcuLimitImplementation,
            abi.encodeCall(HCULimit.initializeFromEmptyProxy, (type(uint48).max, 5_000_000, 20_000_000))
        );

        hcuLimitProxy = HCULimit(hcuLimitAdd);
    }

    function _deployFullHostStack(
        address owner,
        address pauser,
        address kmsVerifyingSource,
        address inputVerifyingSource,
        uint64 chainIDSource,
        KmsNodeParams[] memory initialKmsNodeParams,
        IProtocolConfig.KmsThresholds memory initialThresholds,
        address[] memory inputSigners,
        uint256 inputThreshold
    ) internal {
        (ACL aclProxy, ) = _deployACL(owner);
        PauserSet pauserSet = _deployPauserSet();
        (FHEVMExecutor fheExecutor, ) = _deployFHEVMExecutor(owner);
        _deployHCULimit(owner);
        (ProtocolConfig protocolConfigProxy, ) = _deployProtocolConfig(owner, initialKmsNodeParams, initialThresholds);
        _deployKMSGeneration(owner);
        _deployKMSVerifier(owner, kmsVerifyingSource, chainIDSource);
        _deployInputVerifier(owner, inputVerifyingSource, chainIDSource, inputSigners, inputThreshold);

        vm.prank(owner);
        pauserSet.addPauser(pauser);

        require(fheExecutor.getACLAddress() == aclAdd, "executor ACL wiring");
        require(fheExecutor.getHCULimitAddress() == hcuLimitAdd, "executor HCU wiring");
        require(aclProxy.getPauserSetAddress() == pauserSetAdd, "ACL PauserSet wiring");
        (uint256 activeProtocolContextId, ) = protocolConfigProxy.getCurrentKmsContextAndEpoch();
        require(activeProtocolContextId != 0, "ProtocolConfig wiring");
        require(
            protocolConfigProxy.getPublicDecryptionThreshold() == initialThresholds.publicDecryption,
            "KMS threshold wiring"
        );
        require(InputVerifier(inputVerifierAdd).getThreshold() == inputThreshold, "Input threshold wiring");
    }

    function _deployProtocolConfig(
        address owner,
        KmsNodeParams[] memory initialKmsNodeParams,
        IProtocolConfig.KmsThresholds memory initialThresholds
    ) internal returns (ProtocolConfig protocolConfigProxy, address protocolConfigImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            protocolConfigAdd
        );
        vm.label(protocolConfigAdd, "ProtocolConfig Proxy");

        protocolConfigImplementation = address(new ProtocolConfig());
        vm.label(protocolConfigImplementation, "ProtocolConfig Implementation");

        PcrValues[] memory pcrValues = new PcrValues[](0);
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            protocolConfigImplementation,
            abi.encodeCall(
                ProtocolConfig.initializeFromEmptyProxy,
                (initialKmsNodeParams, initialThresholds, "", pcrValues)
            )
        );

        protocolConfigProxy = ProtocolConfig(protocolConfigAdd);
    }

    function _deployProtocolConfigMirror(
        address owner,
        uint256 initialContextId,
        uint256 initialEpochId,
        KmsNodeParams[] memory initialKmsNodeParams,
        IProtocolConfig.KmsThresholds memory initialThresholds
    ) internal returns (ProtocolConfig protocolConfigProxy, address protocolConfigImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            protocolConfigAdd
        );
        vm.label(protocolConfigAdd, "ProtocolConfig Mirror Proxy");

        protocolConfigImplementation = address(new ProtocolConfig());
        vm.label(protocolConfigImplementation, "ProtocolConfig Mirror Implementation");

        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            protocolConfigImplementation,
            abi.encodeCall(
                ProtocolConfig.initializeFromCanonical,
                (initialContextId, initialEpochId, initialKmsNodeParams, initialThresholds)
            )
        );

        protocolConfigProxy = ProtocolConfig(protocolConfigAdd);
    }

    function _deployProtocolConfigMirror(
        address owner,
        uint256 initialContextId,
        KmsNodeParams[] memory initialKmsNodeParams,
        IProtocolConfig.KmsThresholds memory initialThresholds
    ) internal returns (ProtocolConfig protocolConfigProxy, address protocolConfigImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            protocolConfigAdd
        );
        vm.label(protocolConfigAdd, "ProtocolConfig Mirror Proxy");

        protocolConfigImplementation = address(new ProtocolConfig());
        vm.label(protocolConfigImplementation, "ProtocolConfig Mirror Implementation");

        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            protocolConfigImplementation,
            abi.encodeCall(
                ProtocolConfig.initializeFromCanonical,
                (initialContextId, initialEpochId, initialKmsNodeParams, initialThresholds)
            )
        );

        protocolConfigProxy = ProtocolConfig(protocolConfigAdd);
    }

    function _deployKMSGeneration(
        address owner
    ) internal returns (KMSGeneration kmsGenerationProxy, address kmsGenerationImplementation) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());

        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            kmsGenerationAdd
        );
        vm.label(kmsGenerationAdd, "KMSGeneration Proxy");

        kmsGenerationImplementation = address(new KMSGeneration());
        vm.label(kmsGenerationImplementation, "KMSGeneration Implementation");

        vm.prank(owner);
        EmptyUUPSProxy(kmsGenerationAdd).upgradeToAndCall(
            kmsGenerationImplementation,
            abi.encodeCall(KMSGeneration.initializeFromEmptyProxy, ())
        );

        kmsGenerationProxy = KMSGeneration(kmsGenerationAdd);
    }

    function _deployPauserSet() internal returns (PauserSet pauserSet) {
        vm.etch(pauserSetAdd, address(new PauserSet()).code);
        vm.label(pauserSetAdd, "PauserSet");
        pauserSet = PauserSet(pauserSetAdd);
    }

    function _defaultThresholds() internal pure returns (IProtocolConfig.KmsThresholds memory) {
        return IProtocolConfig.KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1});
    }

    function _computeSignature(uint256 privateKey, bytes32 digest) internal pure returns (bytes memory signature) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, digest);
        return abi.encodePacked(r, s, v);
    }

    function _makeKmsNodeParams(uint256 count) internal pure returns (KmsNodeParams[] memory params) {
        params = new KmsNodeParams[](count);
        for (uint256 i = 0; i < count; i++) {
            string memory ipAddress = string.concat("127.0.0.", vm.toString(i + 1));
            params[i] = KmsNodeParams({
                txSenderAddress: address(uint160(0xA1 + i)),
                signerAddress: vm.addr((i + 1) * 0x100),
                ipAddress: ipAddress,
                storageUrl: string.concat("https://s", vm.toString(i), ".example.com"),
                partyId: int32(uint32(i)),
                mpcIdentity: ipAddress,
                caCert: "",
                storagePrefix: string.concat("kms/node", vm.toString(i))
            });
        }
    }

    function _makeKmsNodeParamsFromSigners(
        address[] memory signers
    ) internal pure returns (KmsNodeParams[] memory params) {
        params = _makeKmsNodeParams(signers.length);
        for (uint256 i = 0; i < signers.length; i++) {
            params[i].signerAddress = signers[i];
        }
    }

    /// @dev Projection of _makeKmsNodeParams onto the stored KmsNode shape, for getKmsNodesForContext assertions.
    function _makeKmsNodes(uint256 count) internal pure returns (KmsNode[] memory nodes) {
        KmsNodeParams[] memory params = _makeKmsNodeParams(count);
        nodes = new KmsNode[](count);
        for (uint256 i = 0; i < count; i++) {
            nodes[i] = KmsNode({
                txSenderAddress: params[i].txSenderAddress,
                signerAddress: params[i].signerAddress,
                ipAddress: params[i].ipAddress,
                storageUrl: params[i].storageUrl
            });
        }
    }

    function _mockKeyDigests() internal pure returns (IKMSGeneration.KeyDigest[] memory) {
        IKMSGeneration.KeyDigest[] memory digests = new IKMSGeneration.KeyDigest[](1);
        digests[0] = IKMSGeneration.KeyDigest({keyType: IKMSGeneration.KeyType.Server, digest: hex"aabbccdd"});
        return digests;
    }

    function _prepKeygenIdForKeyId(uint256 keyId) internal pure returns (uint256) {
        return PREP_KEYGEN_COUNTER_BASE + (keyId - KEY_COUNTER_BASE);
    }

    function _computeProtocolConfigDomainSeparator() internal view returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EIP712_DOMAIN_TYPE_HASH,
                    keccak256(bytes("ProtocolConfig")),
                    keccak256(bytes("1")),
                    block.chainid,
                    address(protocolConfig)
                )
            );
    }

    /// @dev Shared keygen struct-hash builder, parameterized by EIP-712 domain separator.
    function _hashKeygenWithDomain(
        bytes32 domainSeparator,
        uint256 prepKeygenId,
        uint256 keyId,
        IKMSGeneration.KeyDigest[] memory keyDigests,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        bytes32[] memory digestHashes = new bytes32[](keyDigests.length);
        for (uint256 i = 0; i < keyDigests.length; i++) {
            digestHashes[i] = keccak256(
                abi.encode(EIP712_KEY_DIGEST_TYPE_HASH, keyDigests[i].keyType, keccak256(keyDigests[i].digest))
            );
        }
        bytes32 structHash = keccak256(
            abi.encode(
                EIP712_KEYGEN_TYPE_HASH,
                prepKeygenId,
                keyId,
                keccak256(abi.encodePacked(digestHashes)),
                keccak256(extraData)
            )
        );
        return MessageHashUtils.toTypedDataHash(domainSeparator, structHash);
    }

    /// @dev Shared crsgen struct-hash builder, parameterized by EIP-712 domain separator.
    function _hashCrsgenWithDomain(
        bytes32 domainSeparator,
        uint256 crsId,
        uint256 maxBitLength,
        bytes memory crsDigest,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                EIP712_CRSGEN_TYPE_HASH,
                crsId,
                maxBitLength,
                keccak256(abi.encodePacked(crsDigest)),
                keccak256(extraData)
            )
        );
        return MessageHashUtils.toTypedDataHash(domainSeparator, structHash);
    }

    function _hashProtocolConfigKeygen(
        uint256 prepKeygenId,
        uint256 keyId,
        IKMSGeneration.KeyDigest[] memory keyDigests,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return
            _hashKeygenWithDomain(_computeProtocolConfigDomainSeparator(), prepKeygenId, keyId, keyDigests, extraData);
    }

    function _hashProtocolConfigCrsgen(
        uint256 crsId,
        uint256 maxBitLength,
        bytes memory crsDigest,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return
            _hashCrsgenWithDomain(_computeProtocolConfigDomainSeparator(), crsId, maxBitLength, crsDigest, extraData);
    }

    function _defineNewKmsContextAndEpoch(
        KmsNodeParams[] memory nodes,
        IProtocolConfig.KmsThresholds memory thresholds
    ) internal {
        PcrValues[] memory pcrValues = new PcrValues[](0);
        protocolConfig.defineNewKmsContextAndEpoch(nodes, thresholds, "", pcrValues);
    }

    function _confirmContextCreation(uint256 contextId, address txSender) internal {
        vm.prank(txSender);
        protocolConfig.confirmKmsContextCreation(contextId);
    }

    function _confirmEpochActivation(
        uint256 contextId,
        uint256 epochId,
        uint256 pk,
        address txSender,
        uint256 keyId,
        uint256 crsId
    ) internal {
        bytes memory extraData = abi.encodePacked(uint8(0x02), contextId, epochId);

        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](keyId == 0 ? 0 : 1);
        if (keyId != 0) {
            IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
            uint256 prepKeygenId = _prepKeygenIdForKeyId(keyId);
            keys[0] = IProtocolConfig.EpochKeyResult({
                prepKeygenId: prepKeygenId,
                keyId: keyId,
                keyDigests: keyDigests,
                signature: _computeSignature(pk, _hashProtocolConfigKeygen(prepKeygenId, keyId, keyDigests, extraData))
            });
        }

        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](crsId == 0 ? 0 : 1);
        if (crsId != 0) {
            crsList[0] = IProtocolConfig.EpochCrsResult({
                crsId: crsId,
                maxBitLength: 4096,
                crsDigest: hex"deadbeef",
                signature: _computeSignature(pk, _hashProtocolConfigCrsgen(crsId, 4096, hex"deadbeef", extraData))
            });
        }

        vm.prank(txSender);
        protocolConfig.confirmEpochActivation(epochId, keys, crsList);
    }
}
