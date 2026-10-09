// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {IKMSGeneration} from "@fhevm-host-contracts/contracts/interfaces/IKMSGeneration.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsThresholds, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";

/**
 * @dev Replays test-vectors/protocol-config-eip712.json (written by scripts/generateProtocolConfigVectors.ts,
 *      ethers only) against ProtocolConfig. The contract accepts a vector signature only if it computes the
 *      same digest, so acceptance proves the domain, typehashes, nodeConfigHash and epochMaterialHash.
 *      JSON fields are read one by one: abi.decode(vm.parseJson(...)) orders struct fields alphabetically.
 */
contract ProtocolConfigVectorsTest is HostContractsDeployerTestUtils {
    address internal constant owner = address(456);

    string internal vectors;

    function setUp() public {
        vectors = vm.readFile(string.concat(vm.projectRoot(), "/test-vectors/protocol-config-eip712.json"));
        vm.chainId(vm.parseJsonUint(vectors, ".domain.chainId"));
        address proxy = vm.parseJsonAddress(vectors, ".domain.verifyingContract");

        _deployACL(owner);
        address emptyProxyImplementation = address(new EmptyUUPSProxy());
        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            proxy
        );
        address implementation = address(new ProtocolConfig());
        vm.prank(owner);
        EmptyUUPSProxy(proxy).upgradeToAndCall(
            implementation,
            abi.encodeCall(
                ProtocolConfig.initializeFromEmptyProxy,
                (_nodeParams(".scenario.genesis.kmsNodeParams", 4), _thresholds(), "", new PcrValues[](0))
            )
        );
        protocolConfig = ProtocolConfig(proxy);
    }

    function test_ReplayScenario() public {
        uint256 switchAContextId = vm.parseJsonUint(vectors, ".scenario.switchA.contextId");
        uint256 switchBContextId = vm.parseJsonUint(vectors, ".scenario.switchB.contextId");
        uint256 switchBEpochId = vm.parseJsonUint(vectors, ".scenario.switchB.epochId");
        uint256 abortedResharingEpochId = vm.parseJsonUint(vectors, ".scenario.abortedResharingEpochId");
        uint256 resharingEpochId = vm.parseJsonUint(vectors, ".scenario.resharingEpochId");

        // 1-2. Define switch A, destroy it, and collect its destruction confirmations.
        vm.startPrank(owner);
        protocolConfig.defineNewKmsContextAndEpoch(
            _nodeParams(".scenario.switchA.kmsNodeParams", 3),
            _thresholds(),
            "",
            new PcrValues[](0)
        );
        protocolConfig.destroyKmsContext(switchAContextId);
        vm.stopPrank();
        for (uint256 i = 0; i < 2; i++) {
            string memory path = string.concat(".messages.contextDestruction[", vm.toString(i), "]");
            uint256[] memory destroyedEpochIds = vm.parseJsonUintArray(
                vectors,
                string.concat(path, ".fields.destroyedEpochIds")
            );
            (address signer, bytes memory signature, bytes memory extraData) = _message(path);
            vm.expectEmit(address(protocolConfig));
            emit IProtocolConfig.KmsContextDestructionConfirmed(
                switchAContextId,
                destroyedEpochIds,
                signer,
                signature,
                extraData
            );
            protocolConfig.confirmKmsContextDestruction(switchAContextId, destroyedEpochIds, signature, extraData);
        }

        // 3. Define switch B (ids leave a gap) and collect its creation confirmations.
        vm.prank(owner);
        protocolConfig.defineNewKmsContextAndEpoch(
            _nodeParams(".scenario.switchB.kmsNodeParams", 4),
            _thresholds(),
            "",
            new PcrValues[](0)
        );
        for (uint256 i = 0; i < 6; i++) {
            string memory path = string.concat(".messages.contextCreation[", vm.toString(i), "]");
            (address signer, bytes memory signature, bytes memory extraData) = _message(path);
            vm.expectEmit(address(protocolConfig));
            emit IProtocolConfig.KmsContextCreationConfirmation(switchBContextId, signer, signature, extraData);
            protocolConfig.confirmKmsContextCreation(switchBContextId, signature, extraData);
        }
        (uint256 activeContextId, ) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertNotEq(activeContextId, switchBContextId);

        // 4. Activate switch B's epoch.
        _activate(0, switchBEpochId);
        (uint256 contextId, uint256 epochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(contextId, switchBContextId);
        assertEq(epochId, switchBEpochId);

        // 5-6. Abort one resharing, then activate the next (ids leave a gap).
        vm.startPrank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        protocolConfig.destroyKmsEpoch(abortedResharingEpochId);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        vm.stopPrank();
        _activate(4, resharingEpochId);
        (, epochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(epochId, resharingEpochId);

        // 7-8. Destroy switch B's epoch and collect both epoch destruction confirmations.
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(switchBEpochId);
        for (uint256 i = 0; i < 2; i++) {
            string memory path = string.concat(".messages.epochDestruction[", vm.toString(i), "]");
            uint256 destroyedEpochId = vm.parseJsonUint(vectors, string.concat(path, ".fields.destroyedEpochId"));
            (address signer, bytes memory signature, bytes memory extraData) = _message(path);
            vm.expectEmit(address(protocolConfig));
            emit IProtocolConfig.KmsEpochDestructionConfirmed(destroyedEpochId, signer, signature, extraData);
            protocolConfig.confirmKmsEpochDestruction(destroyedEpochId, signature, extraData);
        }
    }

    /// @dev Submits the four epochActivation messages starting at `first`. The last one activates the epoch.
    function _activate(uint256 first, uint256 epochId) internal {
        (IProtocolConfig.EpochKeyResult[] memory keys, IProtocolConfig.EpochCrsResult[] memory crsList) = _material();
        bytes32 epochMaterialHash = vm.parseJsonBytes32(vectors, ".epochMaterialHash[0].hash");
        for (uint256 i = first; i < first + 4; i++) {
            string memory path = string.concat(".messages.epochActivation[", vm.toString(i), "]");
            keys[0].signature = vm.parseJsonBytes(vectors, string.concat(path, ".keySignatures[0]"));
            crsList[0].signature = vm.parseJsonBytes(vectors, string.concat(path, ".crsSignatures[0]"));
            (address signer, bytes memory signature, bytes memory extraData) = _message(path);
            vm.expectEmit(address(protocolConfig));
            emit IProtocolConfig.EpochActivationConfirmation(epochId, signer, epochMaterialHash, signature, extraData);
            protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, extraData);
        }
    }

    function _message(
        string memory path
    ) internal view returns (address signer, bytes memory signature, bytes memory extraData) {
        signer = vm.parseJsonAddress(vectors, string.concat(path, ".signer"));
        signature = vm.parseJsonBytes(vectors, string.concat(path, ".signature"));
        extraData = vm.parseJsonBytes(vectors, string.concat(path, ".fields.extraData"));
    }

    function _thresholds() internal view returns (KmsThresholds memory) {
        return
            KmsThresholds({
                publicDecryption: vm.parseJsonUint(vectors, ".scenario.thresholds.publicDecryption"),
                userDecryption: vm.parseJsonUint(vectors, ".scenario.thresholds.userDecryption"),
                kmsGen: vm.parseJsonUint(vectors, ".scenario.thresholds.kmsGen"),
                mpc: vm.parseJsonUint(vectors, ".scenario.thresholds.mpc")
            });
    }

    function _nodeParams(string memory path, uint256 count) internal view returns (KmsNodeParams[] memory params) {
        params = new KmsNodeParams[](count);
        for (uint256 i = 0; i < count; i++) {
            string memory p = string.concat(path, "[", vm.toString(i), "]");
            params[i] = KmsNodeParams({
                txSenderAddress: vm.parseJsonAddress(vectors, string.concat(p, ".txSenderAddress")),
                signerAddress: vm.parseJsonAddress(vectors, string.concat(p, ".signerAddress")),
                ipAddress: vm.parseJsonString(vectors, string.concat(p, ".ipAddress")),
                storageUrl: vm.parseJsonString(vectors, string.concat(p, ".storageUrl")),
                partyId: int32(vm.parseJsonInt(vectors, string.concat(p, ".partyId"))),
                mpcIdentity: vm.parseJsonString(vectors, string.concat(p, ".mpcIdentity")),
                caCert: vm.parseJsonBytes(vectors, string.concat(p, ".caCert")),
                storagePrefix: vm.parseJsonString(vectors, string.concat(p, ".storagePrefix"))
            });
        }
    }

    /// @dev The one key (two digests) and one CRS result of the vectors, without signatures.
    function _material()
        internal
        view
        returns (IProtocolConfig.EpochKeyResult[] memory keys, IProtocolConfig.EpochCrsResult[] memory crsList)
    {
        IKMSGeneration.KeyDigest[] memory keyDigests = new IKMSGeneration.KeyDigest[](2);
        for (uint256 i = 0; i < 2; i++) {
            string memory p = string.concat(".epochMaterialHash[0].keys[0].keyDigests[", vm.toString(i), "]");
            keyDigests[i] = IKMSGeneration.KeyDigest({
                keyType: IKMSGeneration.KeyType(vm.parseJsonUint(vectors, string.concat(p, ".keyType"))),
                digest: vm.parseJsonBytes(vectors, string.concat(p, ".digest"))
            });
        }
        keys = new IProtocolConfig.EpochKeyResult[](1);
        keys[0] = IProtocolConfig.EpochKeyResult({
            prepKeygenId: vm.parseJsonUint(vectors, ".epochMaterialHash[0].keys[0].prepKeygenId"),
            keyId: vm.parseJsonUint(vectors, ".epochMaterialHash[0].keys[0].keyId"),
            keyDigests: keyDigests,
            signature: ""
        });
        crsList = new IProtocolConfig.EpochCrsResult[](1);
        crsList[0] = IProtocolConfig.EpochCrsResult({
            crsId: vm.parseJsonUint(vectors, ".epochMaterialHash[0].crs[0].crsId"),
            maxBitLength: vm.parseJsonUint(vectors, ".epochMaterialHash[0].crs[0].maxBitLength"),
            crsDigest: vm.parseJsonBytes(vectors, ".epochMaterialHash[0].crs[0].crsDigest"),
            signature: ""
        });
    }
}
