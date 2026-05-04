// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import {KMSGeneration} from "@fhevm-host-contracts/contracts/KMSGeneration.sol";
import {IKMSGeneration} from "@fhevm-host-contracts/contracts/interfaces/IKMSGeneration.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsNode} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {protocolConfigAdd, kmsGenerationAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {HostContractsDeployerTestUtils, DeployableERC1967Proxy} from "@fhevm-host-contracts/fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {KMSGenerationUpgradedExample} from "@fhevm-host-contracts/examples/KMSGenerationUpgradedExample.sol";

contract KMSGenerationHarness is KMSGeneration {
    function extractContextIdFromExtraData(bytes memory extraData) external view returns (uint256) {
        return _extractContextIdFromExtraData(extraData);
    }
}

contract KMSGenerationTest is HostContractsDeployerTestUtils {
    KMSGeneration internal kmsGeneration;
    KMSGenerationHarness internal kmsGenerationHarness;
    ProtocolConfig internal protocolConfig;

    struct MigrationFixture {
        address migrateProxy;
        address kmsGenImpl;
        KMSGeneration.MigrationState state;
    }

    // Counter bases (matching contract)
    uint256 internal constant PREP_KEYGEN_COUNTER_BASE = uint256(3) << 248;
    uint256 internal constant KEY_COUNTER_BASE = uint256(4) << 248;
    uint256 internal constant CRS_COUNTER_BASE = uint256(5) << 248;
    // EIP-712 type hashes
    bytes32 internal constant EIP712_DOMAIN_TYPE_HASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
    string internal constant EIP712_DOMAIN_NAME = "KMSGeneration";
    string internal constant EIP712_DOMAIN_VERSION = "1";
    bytes32 internal constant EIP712_PREP_KEYGEN_TYPE_HASH =
        keccak256("PrepKeygenVerification(uint256 prepKeygenId,bytes extraData)");
    bytes32 internal constant EIP712_KEY_DIGEST_TYPE_HASH = keccak256("KeyDigest(uint8 keyType,bytes digest)");
    bytes32 internal constant EIP712_KEYGEN_TYPE_HASH =
        keccak256(
            "KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)"
        );
    bytes32 internal constant EIP712_CRSGEN_TYPE_HASH =
        keccak256("CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)");

    address internal constant owner = address(456);

    // KMS node private keys for signing
    uint256 internal constant kmsPk0 = 0x100;
    uint256 internal constant kmsPk1 = 0x200;
    uint256 internal constant kmsPk2 = 0x300;
    uint256 internal constant kmsPk3 = 0x400;
    address internal kmsSigner0;
    address internal kmsSigner1;
    address internal kmsSigner2;
    address internal kmsSigner3;
    address internal kmsTxSender0 = address(0xA1);
    address internal kmsTxSender1 = address(0xA2);
    address internal kmsTxSender2 = address(0xA3);
    address internal kmsTxSender3 = address(0xA4);

    function setUp() public {
        kmsSigner0 = vm.addr(kmsPk0);
        kmsSigner1 = vm.addr(kmsPk1);
        kmsSigner2 = vm.addr(kmsPk2);
        kmsSigner3 = vm.addr(kmsPk3);

        // Deploy ACL
        _deployACL(owner);

        // Deploy ProtocolConfig with our test KMS nodes
        KmsNode[] memory nodes = _makeKmsNodes(2);
        _deployProtocolConfig(owner, nodes, _defaultThresholds());
        protocolConfig = ProtocolConfig(protocolConfigAdd);

        // Deploy KMSGeneration
        _deployKMSGeneration(owner);
        kmsGeneration = KMSGeneration(kmsGenerationAdd);
        kmsGenerationHarness = new KMSGenerationHarness();
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    function _assertNoEventEmitted(bytes32 eventSelector, string memory message) internal {
        Vm.Log[] memory logs = vm.getRecordedLogs();
        for (uint256 i = 0; i < logs.length; i++) {
            assertTrue(logs[i].topics[0] != eventSelector, message);
        }
    }

    function _deployMigrationProxyAndState()
        internal
        returns (address migrateProxy, address kmsGenImpl, KMSGeneration.MigrationState memory state)
    {
        (migrateProxy, kmsGenImpl) = _deployMigrationProxy();
        state = _defaultMigrationState(migrateProxy);
    }

    function _deployMigrationFixture() internal returns (MigrationFixture memory fixture) {
        (fixture.migrateProxy, fixture.kmsGenImpl) = _deployMigrationProxy();
        fixture.state = _defaultMigrationState(fixture.migrateProxy);
    }

    function _computeDomainSeparator(address verifier) internal view returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EIP712_DOMAIN_TYPE_HASH,
                    keccak256(bytes(EIP712_DOMAIN_NAME)),
                    keccak256(bytes(EIP712_DOMAIN_VERSION)),
                    block.chainid,
                    verifier
                )
            );
    }

    function _buildExtraData() internal view returns (bytes memory) {
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        return _buildExtraDataForContextId(contextId);
    }

    function _buildExtraDataForContextId(uint256 contextId) internal pure returns (bytes memory) {
        return abi.encodePacked(uint8(0x01), contextId);
    }

    function _hashPrepKeygen(uint256 prepKeygenId, bytes memory extraData) internal view returns (bytes32) {
        return _hashPrepKeygen(kmsGenerationAdd, prepKeygenId, extraData);
    }

    function _hashPrepKeygen(
        address verifier,
        uint256 prepKeygenId,
        bytes memory extraData
    ) internal view returns (bytes32) {
        bytes32 structHash = keccak256(abi.encode(EIP712_PREP_KEYGEN_TYPE_HASH, prepKeygenId, keccak256(extraData)));
        return MessageHashUtils.toTypedDataHash(_computeDomainSeparator(verifier), structHash);
    }

    function _mockKeyDigests() internal pure returns (IKMSGeneration.KeyDigest[] memory) {
        IKMSGeneration.KeyDigest[] memory digests = new IKMSGeneration.KeyDigest[](1);
        digests[0] = IKMSGeneration.KeyDigest({keyType: IKMSGeneration.KeyType.Server, digest: hex"aabbccdd"});
        return digests;
    }

    function _hashKeygen(
        uint256 prepKeygenId,
        uint256 keyId,
        IKMSGeneration.KeyDigest[] memory keyDigests,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return _hashKeygen(kmsGenerationAdd, prepKeygenId, keyId, keyDigests, extraData);
    }

    function _hashKeygen(
        address verifier,
        uint256 prepKeygenId,
        uint256 keyId,
        IKMSGeneration.KeyDigest[] memory keyDigests,
        bytes memory extraData
    ) internal view returns (bytes32) {
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
        return MessageHashUtils.toTypedDataHash(_computeDomainSeparator(verifier), structHash);
    }

    function _hashCrsgen(
        uint256 crsId,
        uint256 maxBitLength,
        bytes memory crsDigest,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return _hashCrsgen(kmsGenerationAdd, crsId, maxBitLength, crsDigest, extraData);
    }

    function _hashCrsgen(
        address verifier,
        uint256 crsId,
        uint256 maxBitLength,
        bytes memory crsDigest,
        bytes memory extraData
    ) internal view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                EIP712_CRSGEN_TYPE_HASH,
                crsId,
                maxBitLength,
                keccak256(abi.encodePacked(crsDigest)),
                keccak256(extraData)
            )
        );
        return MessageHashUtils.toTypedDataHash(_computeDomainSeparator(verifier), structHash);
    }

    function _primaryStorageUrls() internal pure returns (string[] memory urls) {
        urls = new string[](1);
        urls[0] = "https://s0.example.com";
    }

    /// @dev Define a new KMS context with 4 nodes and kmsGen threshold 3.
    function _switchToMultiSignerContext() internal {
        KmsNode[] memory nodes = _makeKmsNodes(4);
        IProtocolConfig.KmsThresholds memory thresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 3,
            userDecryption: 3,
            kmsGen: 3,
            mpc: 3
        });
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(nodes, thresholds);
    }

    /// @dev Hash + sign + prank + call prepKeygenResponse for a single KMS node.
    function _doPrepKeygenResponse(uint256 prepKeygenId, uint256 pk, address sender) internal {
        bytes memory extraData = _buildExtraData();
        bytes32 digest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory sig = _computeSignature(pk, digest);
        vm.prank(sender);
        kmsGeneration.prepKeygenResponse(prepKeygenId, sig);
    }

    /// @dev Hash + sign + prank + call keygenResponse for a single KMS node (uses _mockKeyDigests).
    function _doKeygenResponse(uint256 prepKeygenId, uint256 keyId, uint256 pk, address sender) internal {
        bytes memory extraData = _buildExtraData();
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes32 digest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory sig = _computeSignature(pk, digest);
        vm.prank(sender);
        kmsGeneration.keygenResponse(keyId, digests, sig);
    }

    /// @dev Hash + sign + prank + call crsgenResponse for a single KMS node.
    function _doCrsgenResponse(
        uint256 crsId,
        uint256 maxBitLength,
        bytes memory crsDigestData,
        uint256 pk,
        address sender
    ) internal {
        bytes memory extraData = _buildExtraData();
        bytes32 digest = _hashCrsgen(crsId, maxBitLength, crsDigestData, extraData);
        bytes memory sig = _computeSignature(pk, digest);
        vm.prank(sender);
        kmsGeneration.crsgenResponse(crsId, crsDigestData, sig);
    }

    /// @dev Convenience overload using default maxBitLength=4096 and crsDigest=0xdeadbeef.
    function _doCrsgenResponse(uint256 crsId, uint256 pk, address sender) internal {
        _doCrsgenResponse(crsId, 4096, hex"deadbeef", pk, sender);
    }

    /// @dev Run a full keygen cycle: keygen() -> prepKeygenResponse -> keygenResponse for one node
    function _runFullKeygenCycle() internal returns (uint256 prepKeygenId, uint256 keyId) {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        keyId = KEY_COUNTER_BASE + 1;
        assertEq(kmsGeneration.getKeyCounter(), keyId);
        assertFalse(kmsGeneration.isRequestDone(keyId));

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        assertFalse(kmsGeneration.isRequestDone(keyId));
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);
        assertTrue(kmsGeneration.isRequestDone(keyId));
    }

    /// @dev Run a full CRS cycle
    function _runFullCrsCycle() internal returns (uint256 crsId) {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        crsId = CRS_COUNTER_BASE + 1;
        assertEq(kmsGeneration.getCrsCounter(), crsId);
        assertFalse(kmsGeneration.isRequestDone(crsId));
        _doCrsgenResponse(crsId, kmsPk0, kmsTxSender0);
        assertTrue(kmsGeneration.isRequestDone(crsId));
    }

    function _defaultMigrationState(
        address verifier
    ) internal view returns (KMSGeneration.MigrationState memory state) {
        IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        bytes memory extraData = _buildExtraDataForContextId(contextId);
        uint256 activePrepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 activeKeyId = KEY_COUNTER_BASE + 1;
        uint256 activeCrsId = CRS_COUNTER_BASE + 1;

        address[] memory keyTxSenders = new address[](1);
        keyTxSenders[0] = kmsTxSender0;
        address[] memory crsTxSenders = new address[](1);
        crsTxSenders[0] = kmsTxSender0;
        address[] memory prepTxSenders = new address[](1);
        prepTxSenders[0] = kmsTxSender0;

        state = KMSGeneration.MigrationState({
            prepKeygenCounter: PREP_KEYGEN_COUNTER_BASE + 1,
            keyCounter: KEY_COUNTER_BASE + 1,
            crsCounter: CRS_COUNTER_BASE + 1,
            activeKeyId: activeKeyId,
            activeCrsId: activeCrsId,
            activePrepKeygenId: activePrepKeygenId,
            activeKeyDigests: keyDigests,
            activeCrsDigest: hex"deadbeef",
            keyConsensusTxSenders: keyTxSenders,
            keyConsensusDigest: _hashKeygen(verifier, activePrepKeygenId, activeKeyId, keyDigests, extraData),
            crsConsensusTxSenders: crsTxSenders,
            crsConsensusDigest: _hashCrsgen(verifier, activeCrsId, 4096, hex"deadbeef", extraData),
            prepKeygenConsensusTxSenders: prepTxSenders,
            prepKeygenConsensusDigest: _hashPrepKeygen(verifier, activePrepKeygenId, extraData),
            crsMaxBitLength: 4096,
            prepKeygenParamsType: IKMSGeneration.ParamsType.Default,
            crsParamsType: IKMSGeneration.ParamsType.Default,
            contextId: contextId
        });
    }

    function _deployMigrationProxy() internal returns (address migrateProxy, address kmsGenImpl) {
        address emptyImpl = address(new EmptyUUPSProxy());
        migrateProxy = address(new DeployableERC1967Proxy(emptyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())));
        kmsGenImpl = address(new KMSGeneration());
    }

    function _deployMigratedKmsGeneration(
        KMSGeneration.MigrationState memory state
    ) internal returns (KMSGeneration migrated) {
        (address migrateProxy, address kmsGenImpl) = _deployMigrationProxy();
        _upgradeMigrationProxy(migrateProxy, kmsGenImpl, state);
        migrated = KMSGeneration(migrateProxy);
    }

    function _deployMigratedKmsGeneration()
        internal
        returns (KMSGeneration migrated, KMSGeneration.MigrationState memory state)
    {
        (address migrateProxy, address kmsGenImpl) = _deployMigrationProxy();
        state = _defaultMigrationState(migrateProxy);
        _upgradeMigrationProxy(migrateProxy, kmsGenImpl, state);
        migrated = KMSGeneration(migrateProxy);
    }

    function _upgradeMigrationProxy(
        address migrateProxy,
        address kmsGenImpl,
        KMSGeneration.MigrationState memory state
    ) internal {
        vm.prank(owner);
        EmptyUUPSProxy(migrateProxy).upgradeToAndCall(
            kmsGenImpl,
            abi.encodeCall(KMSGeneration.initializeFromMigration, (state))
        );
    }

    function _expectMigrationRevert(MigrationFixture memory fixture, bytes memory expectedRevert) internal {
        vm.expectRevert(expectedRevert);
        _upgradeMigrationProxy(fixture.migrateProxy, fixture.kmsGenImpl, fixture.state);
    }

    function _assumeNotCurrentKmsTxSender(address caller) internal view {
        vm.assume(!protocolConfig.isKmsTxSenderForContext(protocolConfig.getCurrentKmsContextId(), caller));
    }

    // -----------------------------------------------------------------------
    // Init tests
    // -----------------------------------------------------------------------

    function test_initSuccess() public view {
        assertEq(kmsGeneration.getVersion(), "KMSGeneration v0.1.0");
        assertEq(kmsGeneration.getActiveKeyId(), 0);
        assertEq(kmsGeneration.getActiveCrsId(), 0);
        assertEq(kmsGeneration.getKeyCounter(), KEY_COUNTER_BASE);
        assertEq(kmsGeneration.getCrsCounter(), CRS_COUNTER_BASE);
    }

    function test_revertDoubleInitFromEmptyProxy() public {
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        kmsGeneration.initializeFromEmptyProxy();
    }

    function test_revertMigrationAfterInit() public {
        KMSGeneration.MigrationState memory state;
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        kmsGeneration.initializeFromMigration(state);
    }

    function testFuzz_revertExtractContextIdMalformedV1ExtraData(bytes calldata malformedSuffix) public {
        vm.assume(malformedSuffix.length < 32);
        bytes memory extraData = abi.encodePacked(uint8(0x01), malformedSuffix);
        vm.expectRevert(IKMSGeneration.DeserializingExtraDataFail.selector);
        kmsGenerationHarness.extractContextIdFromExtraData(extraData);
    }

    function testFuzz_revertExtractContextIdUnsupportedExtraDataVersion(bytes calldata extraData) public {
        vm.assume(extraData.length != 0);
        vm.assume(uint8(extraData[0]) != 0x00);
        vm.assume(uint8(extraData[0]) != 0x01);
        uint8 version = uint8(extraData[0]);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.UnsupportedExtraDataVersion.selector, version));
        kmsGenerationHarness.extractContextIdFromExtraData(extraData);
    }

    function test_extractContextIdEmptyExtraDataUsesCurrentContext() public view {
        assertEq(kmsGenerationHarness.extractContextIdFromExtraData(""), protocolConfig.getCurrentKmsContextId());
    }

    // -----------------------------------------------------------------------
    // Full keygen cycle
    // -----------------------------------------------------------------------

    function test_fullKeygenCycle() public {
        (, uint256 keyId) = _runFullKeygenCycle();
        assertEq(kmsGeneration.getActiveKeyId(), keyId);
        assertEq(uint256(kmsGeneration.getKeyParamsType(keyId)), uint256(IKMSGeneration.ParamsType.Default));
    }

    function test_keygenMaterials() public {
        (, uint256 keyId) = _runFullKeygenCycle();
        (string[] memory urls, IKMSGeneration.KeyDigest[] memory digests) = kmsGeneration.getKeyMaterials(keyId);
        assertEq(urls.length, 1);
        assertEq(urls[0], "https://s0.example.com");
        assertEq(digests.length, 1);
    }

    function test_keygenConsensusTxSenders() public {
        _runFullKeygenCycle();
        uint256 keyId = KEY_COUNTER_BASE + 1;
        address[] memory txSenders = kmsGeneration.getConsensusTxSenders(keyId);
        assertEq(txSenders.length, 1);
        assertEq(txSenders[0], kmsTxSender0);
    }

    function test_crsConsensusTxSenders() public {
        _runFullCrsCycle();
        uint256 crsId = CRS_COUNTER_BASE + 1;
        address[] memory txSenders = kmsGeneration.getConsensusTxSenders(crsId);
        assertEq(txSenders.length, 1);
        assertEq(txSenders[0], kmsTxSender0);
    }

    // -----------------------------------------------------------------------
    // Full CRS cycle
    // -----------------------------------------------------------------------

    function test_fullCrsCycle() public {
        uint256 crsId = _runFullCrsCycle();
        assertEq(kmsGeneration.getActiveCrsId(), crsId);
        assertEq(uint256(kmsGeneration.getCrsParamsType(crsId)), uint256(IKMSGeneration.ParamsType.Default));
    }

    function test_emitKeygenLifecycleEvents() public {
        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();

        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.PrepKeygenRequest(prepKeygenId, IKMSGeneration.ParamsType.Default, extraData);
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        bytes32 prepDigest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory prepSig = _computeSignature(kmsPk0, prepDigest);
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.PrepKeygenResponse(prepKeygenId, prepSig, kmsTxSender0);
        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId, prepSig);

        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _computeSignature(kmsPk0, keyDigest);
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.KeygenResponse(keyId, digests, keySig, kmsTxSender0);
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.ActivateKey(keyId, _primaryStorageUrls(), digests);
        vm.prank(kmsTxSender0);
        kmsGeneration.keygenResponse(keyId, digests, keySig);
    }

    function test_emitCrsLifecycleEvents() public {
        uint256 crsId = CRS_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        bytes memory crsDigestData = hex"deadbeef";

        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.CrsgenRequest(crsId, 4096, IKMSGeneration.ParamsType.Default, extraData);
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        bytes32 digest = _hashCrsgen(crsId, 4096, crsDigestData, extraData);
        bytes memory sig = _computeSignature(kmsPk0, digest);
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.CrsgenResponse(crsId, crsDigestData, sig, kmsTxSender0);
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.ActivateCrs(crsId, _primaryStorageUrls(), crsDigestData);
        vm.prank(kmsTxSender0);
        kmsGeneration.crsgenResponse(crsId, crsDigestData, sig);
    }

    function test_crsMaterials() public {
        uint256 crsId = _runFullCrsCycle();
        (string[] memory urls, bytes memory crsDigest) = kmsGeneration.getCrsMaterials(crsId);
        assertEq(urls.length, 1);
        assertEq(urls[0], "https://s0.example.com");
        assertEq(crsDigest, hex"deadbeef");
    }

    // -----------------------------------------------------------------------
    // Sequential ordering
    // -----------------------------------------------------------------------

    function test_revertKeygenOngoing() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 expectedKeyId = KEY_COUNTER_BASE + 1;
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeygenOngoing.selector, expectedKeyId));
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);
    }

    function test_revertCrsgenOngoing() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 expectedCrsId = CRS_COUNTER_BASE + 1;
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsgenOngoing.selector, expectedCrsId));
        kmsGeneration.crsgenRequest(2048, IKMSGeneration.ParamsType.Default);
    }

    function test_revertKeygenResponseBeforePrepKeygenConsensus() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        vm.prank(kmsTxSender0);
        vm.expectRevert(IKMSGeneration.KeyManagementRequestPending.selector);
        kmsGeneration.keygenResponse(KEY_COUNTER_BASE + 1, _mockKeyDigests(), hex"");
    }

    function test_revertPrepKeygenResponseSignedWithStaleContextExtraData() public {
        bytes memory oldExtraData = _buildExtraData();

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);
        vm.prank(owner);
        kmsGeneration.abortKeygen(PREP_KEYGEN_COUNTER_BASE + 1);

        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeKmsNodes(2), _defaultThresholds());

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 2;
        bytes32 replayDigest = _hashPrepKeygen(prepKeygenId, oldExtraData);
        bytes memory replaySig = _computeSignature(kmsPk0, replayDigest);

        vm.prank(kmsTxSender0);
        vm.expectPartialRevert(IKMSGeneration.KmsSignerDoesNotMatchTxSender.selector);
        kmsGeneration.prepKeygenResponse(prepKeygenId, replaySig);
    }

    function test_revertKeygenResponseSignedWithStaleContextExtraData() public {
        bytes memory oldExtraData = _buildExtraData();

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);
        vm.prank(owner);
        kmsGeneration.abortKeygen(PREP_KEYGEN_COUNTER_BASE + 1);

        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeKmsNodes(2), _defaultThresholds());

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 keyId = KEY_COUNTER_BASE + 2;
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);

        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes32 replayDigest = _hashKeygen(prepKeygenId, keyId, digests, oldExtraData);
        bytes memory replaySig = _computeSignature(kmsPk0, replayDigest);

        vm.prank(kmsTxSender0);
        vm.expectPartialRevert(IKMSGeneration.KmsSignerDoesNotMatchTxSender.selector);
        kmsGeneration.keygenResponse(keyId, digests, replaySig);
    }

    function test_revertCrsgenResponseSignedWithStaleContextExtraData() public {
        bytes memory oldExtraData = _buildExtraData();

        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
        vm.prank(owner);
        kmsGeneration.abortCrsgen(CRS_COUNTER_BASE + 1);

        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeKmsNodes(2), _defaultThresholds());

        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 2;
        bytes memory crsDigestData = hex"deadbeef";
        bytes32 replayDigest = _hashCrsgen(crsId, 4096, crsDigestData, oldExtraData);
        bytes memory replaySig = _computeSignature(kmsPk0, replayDigest);

        vm.prank(kmsTxSender0);
        vm.expectPartialRevert(IKMSGeneration.KmsSignerDoesNotMatchTxSender.selector);
        kmsGeneration.crsgenResponse(crsId, crsDigestData, replaySig);
    }

    // -----------------------------------------------------------------------
    // Access control
    // -----------------------------------------------------------------------

    function testFuzz_revertKeygenNotOwner(address caller) public {
        vm.assume(caller != owner);
        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);
    }

    function testFuzz_revertCrsgenNotOwner(address caller) public {
        vm.assume(caller != owner);
        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
    }

    function testFuzz_revertPrepKeygenResponseNotTxSender(address caller) public {
        _assumeNotCurrentKmsTxSender(caller);
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotKmsTxSender.selector, caller));
        kmsGeneration.prepKeygenResponse(PREP_KEYGEN_COUNTER_BASE + 1, hex"");
    }

    function testFuzz_revertKeygenResponseNotTxSender(address caller) public {
        _assumeNotCurrentKmsTxSender(caller);
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotKmsTxSender.selector, caller));
        kmsGeneration.keygenResponse(KEY_COUNTER_BASE + 1, digests, hex"");
    }

    function testFuzz_revertCrsgenResponseNotTxSender(address caller) public {
        _assumeNotCurrentKmsTxSender(caller);
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotKmsTxSender.selector, caller));
        kmsGeneration.crsgenResponse(CRS_COUNTER_BASE + 1, hex"deadbeef", hex"");
    }

    // -----------------------------------------------------------------------
    // Duplicate signature rejection
    // -----------------------------------------------------------------------

    function test_revertDuplicatePrepKeygenSignature() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);

        bytes memory extraData = _buildExtraData();
        bytes32 digest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory sig = _computeSignature(kmsPk0, digest);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsAlreadySignedForPrepKeygen.selector, prepKeygenId, kmsSigner0)
        );
        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId, sig);
    }

    function test_revertDuplicateKeygenSignature() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);

        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes memory extraData = _buildExtraData();
        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _computeSignature(kmsPk0, keyDigest);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KmsAlreadySignedForKeygen.selector, keyId, kmsSigner0));
        vm.prank(kmsTxSender0);
        kmsGeneration.keygenResponse(keyId, digests, keySig);
    }

    function test_revertDuplicateCrsgenSignature() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;

        _doCrsgenResponse(crsId, kmsPk0, kmsTxSender0);

        bytes memory extraData = _buildExtraData();
        bytes32 digest = _hashCrsgen(crsId, 4096, hex"deadbeef", extraData);
        bytes memory sig = _computeSignature(kmsPk0, digest);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KmsAlreadySignedForCrsgen.selector, crsId, kmsSigner0));
        vm.prank(kmsTxSender0);
        kmsGeneration.crsgenResponse(crsId, hex"deadbeef", sig);
    }

    // -----------------------------------------------------------------------
    // Invalid request ID
    // -----------------------------------------------------------------------

    function testFuzz_revertPrepKeygenNotRequested(uint256 prepKeygenId) public {
        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.PrepKeygenNotRequested.selector, prepKeygenId));
        kmsGeneration.prepKeygenResponse(prepKeygenId, hex"");
    }

    function testFuzz_revertKeygenNotRequested(uint256 keyId) public {
        IKMSGeneration.KeyDigest[] memory d = _mockKeyDigests();
        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeygenNotRequested.selector, keyId));
        kmsGeneration.keygenResponse(keyId, d, hex"");
    }

    function testFuzz_revertCrsgenNotRequested(uint256 crsId) public {
        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsgenNotRequested.selector, crsId));
        kmsGeneration.crsgenResponse(crsId, hex"deadbeef", hex"");
    }

    function test_revertKmsSignerDoesNotMatchTxSender() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);

        // Sign with kmsSigner1's key but send from kmsTxSender0 (mismatch)
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes memory extraData = _buildExtraData();
        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _computeSignature(kmsPk1, keyDigest);

        vm.prank(kmsTxSender0);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsSignerDoesNotMatchTxSender.selector, kmsSigner1, kmsTxSender0)
        );
        kmsGeneration.keygenResponse(keyId, digests, keySig);
    }

    function test_revertKmsSignerDoesNotMatchTxSenderPrepKeygen() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        // Sign with kmsSigner0's key but send from kmsTxSender1
        bytes32 prepDigest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory prepSig = _computeSignature(kmsPk0, prepDigest);

        vm.prank(kmsTxSender1);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsSignerDoesNotMatchTxSender.selector, kmsSigner0, kmsTxSender1)
        );
        kmsGeneration.prepKeygenResponse(prepKeygenId, prepSig);
    }

    function test_revertKmsSignerDoesNotMatchTxSenderCrsgen() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        bytes memory crsDigestData = hex"deadbeef";
        // Sign with kmsSigner0's key but send from kmsTxSender1
        bytes32 digest = _hashCrsgen(crsId, 4096, crsDigestData, extraData);
        bytes memory sig = _computeSignature(kmsPk0, digest);

        vm.prank(kmsTxSender1);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsSignerDoesNotMatchTxSender.selector, kmsSigner0, kmsTxSender1)
        );
        kmsGeneration.crsgenResponse(crsId, crsDigestData, sig);
    }

    function test_revertEmptyKeyDigests() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 keyId = KEY_COUNTER_BASE + 1;
        IKMSGeneration.KeyDigest[] memory digests = new IKMSGeneration.KeyDigest[](0);

        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.EmptyKeyDigests.selector, keyId));
        kmsGeneration.keygenResponse(keyId, digests, hex"");
    }

    // -----------------------------------------------------------------------
    // Abort flows
    // -----------------------------------------------------------------------

    function test_abortKeygen() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.AbortKeygen(prepKeygenId);
        vm.prank(owner);
        kmsGeneration.abortKeygen(prepKeygenId);

        assertTrue(kmsGeneration.isRequestDone(prepKeygenId));
        assertTrue(kmsGeneration.isRequestDone(keyId));
    }

    function test_abortKeygenAfterPrepConsensus() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        assertFalse(kmsGeneration.isRequestDone(keyId));

        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.AbortKeygen(prepKeygenId);
        vm.prank(owner);
        kmsGeneration.abortKeygen(prepKeygenId);

        assertTrue(kmsGeneration.isRequestDone(prepKeygenId));
        assertTrue(kmsGeneration.isRequestDone(keyId));

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Test);
    }

    function test_abortCrsgen() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;
        assertFalse(kmsGeneration.isRequestDone(crsId));
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.AbortCrsgen(crsId);
        vm.prank(owner);
        kmsGeneration.abortCrsgen(crsId);

        assertTrue(kmsGeneration.isRequestDone(crsId));
    }

    function testFuzz_revertAbortKeygenInvalidId(uint256 prepKeygenId) public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortKeygenInvalidId.selector, prepKeygenId));
        kmsGeneration.abortKeygen(prepKeygenId);
    }

    function test_revertAbortKeygenAlreadyDone() public {
        (uint256 prepKeygenId, ) = _runFullKeygenCycle();
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortKeygenAlreadyDone.selector, prepKeygenId));
        kmsGeneration.abortKeygen(prepKeygenId);
    }

    function testFuzz_revertAbortCrsgenInvalidId(uint256 crsId) public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortCrsgenInvalidId.selector, crsId));
        kmsGeneration.abortCrsgen(crsId);
    }

    function test_revertAbortCrsgenAlreadyDone() public {
        uint256 crsId = _runFullCrsCycle();
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortCrsgenAlreadyDone.selector, crsId));
        kmsGeneration.abortCrsgen(crsId);
    }

    function testFuzz_abortKeygenNotOwner(address caller) public {
        vm.assume(caller != owner);
        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        kmsGeneration.abortKeygen(PREP_KEYGEN_COUNTER_BASE + 1);
    }

    function testFuzz_abortCrsgenNotOwner(address caller) public {
        vm.assume(caller != owner);
        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        kmsGeneration.abortCrsgen(CRS_COUNTER_BASE + 1);
    }

    function test_keygenAfterAbort() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 abortedPrepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortKeygen(abortedPrepKeygenId);

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Test);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 keyId = KEY_COUNTER_BASE + 2;

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);

        assertEq(kmsGeneration.getActiveKeyId(), keyId);
        assertEq(uint256(kmsGeneration.getKeyParamsType(keyId)), uint256(IKMSGeneration.ParamsType.Test));
    }

    function test_revertGetAbortedKeyViews() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortKeygen(prepKeygenId);

        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyAborted.selector, keyId));
        kmsGeneration.getKeyParamsType(keyId);

        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyAborted.selector, keyId));
        kmsGeneration.getKeyMaterials(keyId);
    }

    function test_crsgenAfterAbort() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 abortedCrsId = CRS_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortCrsgen(abortedCrsId);

        vm.prank(owner);
        kmsGeneration.crsgenRequest(8192, IKMSGeneration.ParamsType.Test);

        uint256 crsId = CRS_COUNTER_BASE + 2;

        _doCrsgenResponse(crsId, 8192, hex"cafebabe", kmsPk0, kmsTxSender0);

        assertEq(kmsGeneration.getActiveCrsId(), crsId);
        assertEq(uint256(kmsGeneration.getCrsParamsType(crsId)), uint256(IKMSGeneration.ParamsType.Test));
    }

    function test_revertGetAbortedCrsViews() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortCrsgen(crsId);

        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsAborted.selector, crsId));
        kmsGeneration.getCrsParamsType(crsId);

        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsAborted.selector, crsId));
        kmsGeneration.getCrsMaterials(crsId);
    }

    // -----------------------------------------------------------------------
    // Nonexistent ID view reverts (never started)
    // -----------------------------------------------------------------------

    function testFuzz_revertGetKeyParamsTypeNonexistent(uint256 keyId) public {
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyNotGenerated.selector, keyId));
        kmsGeneration.getKeyParamsType(keyId);
    }

    function testFuzz_revertGetKeyMaterialsNonexistent(uint256 keyId) public {
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyNotGenerated.selector, keyId));
        kmsGeneration.getKeyMaterials(keyId);
    }

    function testFuzz_revertGetCrsParamsTypeNonexistent(uint256 crsId) public {
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsNotGenerated.selector, crsId));
        kmsGeneration.getCrsParamsType(crsId);
    }

    function testFuzz_revertGetCrsMaterialsNonexistent(uint256 crsId) public {
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsNotGenerated.selector, crsId));
        kmsGeneration.getCrsMaterials(crsId);
    }

    // -----------------------------------------------------------------------
    // Second keygen cycle after first completes
    // -----------------------------------------------------------------------

    function test_secondKeygenCycleAfterFirst() public {
        _runFullKeygenCycle();

        // Start second keygen
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Test);

        uint256 prepKeygenId2 = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 keyId2 = KEY_COUNTER_BASE + 2;

        _doPrepKeygenResponse(prepKeygenId2, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId2, keyId2, kmsPk0, kmsTxSender0);

        assertEq(kmsGeneration.getActiveKeyId(), keyId2);
        assertEq(uint256(kmsGeneration.getKeyParamsType(keyId2)), uint256(IKMSGeneration.ParamsType.Test));
    }

    // -----------------------------------------------------------------------
    // Upgrade (V1 -> V2)
    // -----------------------------------------------------------------------

    function test_upgradeToV2() public {
        // Verify initial version
        assertEq(kmsGeneration.getVersion(), "KMSGeneration v0.1.0");
        assertEq(kmsGeneration.getActiveKeyId(), 0);
        assertEq(kmsGeneration.getActiveCrsId(), 0);

        // Deploy the V2 implementation and upgrade
        address v2Impl = address(new KMSGenerationUpgradedExample());
        vm.prank(owner);
        kmsGeneration.upgradeToAndCall(v2Impl, "");

        // Verify new version
        string memory newVersion = kmsGeneration.getVersion();
        assertEq(newVersion, "KMSGeneration v0.2.0");

        // Verify state is preserved
        assertEq(kmsGeneration.getActiveKeyId(), 0);
        assertEq(kmsGeneration.getActiveCrsId(), 0);
    }

    // -----------------------------------------------------------------------
    // Migration reinitializer
    // -----------------------------------------------------------------------

    function test_migrationReinitializer() public {
        (KMSGeneration migrated, ) = _deployMigratedKmsGeneration();
        assertEq(migrated.getVersion(), "KMSGeneration v0.1.0");
        assertEq(migrated.getActiveKeyId(), KEY_COUNTER_BASE + 1);
        assertEq(migrated.getActiveCrsId(), CRS_COUNTER_BASE + 1);

        // Verify material lookups work post-migration
        (string[] memory keyUrls, IKMSGeneration.KeyDigest[] memory migratedDigests) = migrated.getKeyMaterials(
            KEY_COUNTER_BASE + 1
        );
        assertEq(keyUrls.length, 1);
        assertEq(keyUrls[0], "https://s0.example.com");
        assertEq(migratedDigests.length, 1);

        (string[] memory crsUrls, bytes memory crsDigest) = migrated.getCrsMaterials(CRS_COUNTER_BASE + 1);
        assertEq(crsUrls.length, 1);
        assertEq(crsUrls[0], "https://s0.example.com");
        assertEq(crsDigest, hex"deadbeef");

        // Verify params types
        assertEq(uint256(migrated.getKeyParamsType(KEY_COUNTER_BASE + 1)), uint256(IKMSGeneration.ParamsType.Default));
        assertEq(uint256(migrated.getCrsParamsType(CRS_COUNTER_BASE + 1)), uint256(IKMSGeneration.ParamsType.Default));

        // Verify consensus tx senders
        address[] memory keyConsTxSenders = migrated.getConsensusTxSenders(KEY_COUNTER_BASE + 1);
        assertEq(keyConsTxSenders.length, 1);
        assertEq(keyConsTxSenders[0], kmsTxSender0);
    }

    function test_migrationRestoresDuplicateSignerProtection() public {
        (KMSGeneration migrated, KMSGeneration.MigrationState memory state) = _deployMigratedKmsGeneration();

        bytes32 keyDigest = _hashKeygen(
            address(migrated),
            state.activePrepKeygenId,
            state.activeKeyId,
            _mockKeyDigests(),
            _buildExtraDataForContextId(state.contextId)
        );
        bytes memory keySig = _computeSignature(kmsPk0, keyDigest);

        vm.prank(kmsTxSender0);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsAlreadySignedForKeygen.selector, state.activeKeyId, kmsSigner0)
        );
        migrated.keygenResponse(state.activeKeyId, _mockKeyDigests(), keySig);
    }

    function test_migrationRejectsEmptyKeyDigests() public {
        MigrationFixture memory fixture = _deployMigrationFixture();
        fixture.state.activeKeyDigests = new IKMSGeneration.KeyDigest[](0);

        _expectMigrationRevert(
            fixture,
            abi.encodeWithSelector(KMSGeneration.InvalidMigrationMaterial.selector, fixture.state.activeKeyId)
        );
    }

    function test_migrationRejectsEmptyCrsDigest() public {
        MigrationFixture memory fixture = _deployMigrationFixture();
        fixture.state.activeCrsDigest = "";

        _expectMigrationRevert(
            fixture,
            abi.encodeWithSelector(KMSGeneration.InvalidMigrationMaterial.selector, fixture.state.activeCrsId)
        );
    }

    function test_migrationRejectsActiveKeyWithoutConsensusTxSenders() public {
        MigrationFixture memory fixture = _deployMigrationFixture();
        fixture.state.keyConsensusTxSenders = new address[](0);

        _expectMigrationRevert(
            fixture,
            abi.encodeWithSelector(KMSGeneration.InvalidMigrationConsensusState.selector, fixture.state.activeKeyId)
        );
    }

    function test_migrationRejectsActivePrepKeygenWithoutConsensusDigest() public {
        MigrationFixture memory fixture = _deployMigrationFixture();
        fixture.state.prepKeygenConsensusDigest = bytes32(0);

        _expectMigrationRevert(
            fixture,
            abi.encodeWithSelector(
                KMSGeneration.InvalidMigrationConsensusState.selector,
                fixture.state.activePrepKeygenId
            )
        );
    }

    function test_migrationRejectsBelowKmsGenThresholdQuorum() public {
        // Raise kmsGen threshold to 3 on a freshly-defined context before building migration state.
        _switchToMultiSignerContext();

        MigrationFixture memory fixture = _deployMigrationFixture();
        // _defaultMigrationState ships only 1 tx sender per request, below the kmsGen threshold of 3.
        // The first validated request (the active key) is the one that reverts.
        _expectMigrationRevert(
            fixture,
            abi.encodeWithSelector(KMSGeneration.InvalidMigrationConsensusState.selector, fixture.state.activeKeyId)
        );
    }

    function test_migrationAcceptsExactlyKmsGenThresholdQuorum() public {
        _switchToMultiSignerContext();

        (
            address migrateProxy,
            address kmsGenImpl,
            KMSGeneration.MigrationState memory state
        ) = _deployMigrationProxyAndState();

        address[] memory threeSenders = new address[](3);
        threeSenders[0] = kmsTxSender0;
        threeSenders[1] = kmsTxSender1;
        threeSenders[2] = kmsTxSender2;
        state.keyConsensusTxSenders = threeSenders;
        state.crsConsensusTxSenders = threeSenders;
        state.prepKeygenConsensusTxSenders = threeSenders;

        _upgradeMigrationProxy(migrateProxy, kmsGenImpl, state);

        address[] memory recorded = KMSGeneration(migrateProxy).getConsensusTxSenders(state.activeKeyId);
        assertEq(recorded.length, 3);
    }

    function testFuzz_migrationRejectsUnknownConsensusTxSender(address unknownTxSender) public {
        MigrationFixture memory fixture = _deployMigrationFixture();
        vm.assume(!protocolConfig.isKmsTxSenderForContext(fixture.state.contextId, unknownTxSender));
        fixture.state.keyConsensusTxSenders[0] = unknownTxSender;

        _expectMigrationRevert(
            fixture,
            abi.encodeWithSelector(
                KMSGeneration.UnknownMigrationConsensusTxSender.selector,
                fixture.state.activeKeyId,
                unknownTxSender
            )
        );
    }

    function test_migrationLateSignerStillUpdatesConsensusTxSenders() public {
        (KMSGeneration migrated, KMSGeneration.MigrationState memory state) = _deployMigratedKmsGeneration();

        bytes32 keyDigest = _hashKeygen(
            address(migrated),
            state.activePrepKeygenId,
            state.activeKeyId,
            _mockKeyDigests(),
            _buildExtraDataForContextId(state.contextId)
        );
        bytes memory keySig = _computeSignature(kmsPk1, keyDigest);

        vm.recordLogs();
        vm.prank(kmsTxSender1);
        migrated.keygenResponse(state.activeKeyId, _mockKeyDigests(), keySig);
        _assertNoEventEmitted(
            IKMSGeneration.ActivateKey.selector,
            "late migrated keygen response should not emit ActivateKey"
        );

        address[] memory keyConsTxSenders = migrated.getConsensusTxSenders(state.activeKeyId);
        assertEq(keyConsTxSenders.length, 2);
        assertEq(keyConsTxSenders[0], kmsTxSender0);
        assertEq(keyConsTxSenders[1], kmsTxSender1);
    }

    function test_migrationRejectsKeyCounterAheadOfImportedState() public {
        MigrationFixture memory fixture = _deployMigrationFixture();
        fixture.state.keyCounter++;

        _expectMigrationRevert(fixture, abi.encodeWithSelector(KMSGeneration.InvalidMigrationCounterState.selector));
    }

    function test_migrationRejectsCrsCounterAheadOfImportedState() public {
        MigrationFixture memory fixture = _deployMigrationFixture();
        fixture.state.crsCounter++;

        _expectMigrationRevert(fixture, abi.encodeWithSelector(KMSGeneration.InvalidMigrationCounterState.selector));
    }

    function test_migrationRejectsPrepCounterAheadOfImportedState() public {
        MigrationFixture memory fixture = _deployMigrationFixture();
        fixture.state.prepKeygenCounter++;

        _expectMigrationRevert(fixture, abi.encodeWithSelector(KMSGeneration.InvalidMigrationCounterState.selector));
    }

    // -----------------------------------------------------------------------
    // Multi-signer post-consensus ignore (4 nodes, kmsGen threshold 3)
    // -----------------------------------------------------------------------

    function test_postConsensusPrepKeygenResponseIgnored() public {
        _switchToMultiSignerContext();

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        // Responses 1-3 reach consensus (threshold = 3)
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doPrepKeygenResponse(prepKeygenId, kmsPk1, kmsTxSender1);
        _doPrepKeygenResponse(prepKeygenId, kmsPk2, kmsTxSender2);

        // 4th response should be silently ignored (no KeygenRequest event, no revert)
        vm.recordLogs();
        _doPrepKeygenResponse(prepKeygenId, kmsPk3, kmsTxSender3);
        _assertNoEventEmitted(
            IKMSGeneration.KeygenRequest.selector,
            "4th prepKeygen response should not emit KeygenRequest"
        );
    }

    function test_postConsensusKeygenResponseIgnored() public {
        _switchToMultiSignerContext();

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        // Complete prepKeygen consensus (3 of 4)
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doPrepKeygenResponse(prepKeygenId, kmsPk1, kmsTxSender1);
        _doPrepKeygenResponse(prepKeygenId, kmsPk2, kmsTxSender2);

        // Keygen responses 1-3 reach consensus
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk1, kmsTxSender1);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk2, kmsTxSender2);

        // 4th response should be silently ignored (no ActivateKey event, no revert)
        vm.recordLogs();
        _doKeygenResponse(prepKeygenId, keyId, kmsPk3, kmsTxSender3);
        _assertNoEventEmitted(IKMSGeneration.ActivateKey.selector, "4th keygen response should not emit ActivateKey");
    }

    function test_postConsensusCrsgenResponseIgnored() public {
        _switchToMultiSignerContext();

        uint256 crsId = CRS_COUNTER_BASE + 1;

        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        // Responses 1-3 reach consensus
        _doCrsgenResponse(crsId, kmsPk0, kmsTxSender0);
        _doCrsgenResponse(crsId, kmsPk1, kmsTxSender1);
        _doCrsgenResponse(crsId, kmsPk2, kmsTxSender2);

        // 4th response should be silently ignored (no ActivateCrs event, no revert)
        vm.recordLogs();
        _doCrsgenResponse(crsId, kmsPk3, kmsTxSender3);
        _assertNoEventEmitted(IKMSGeneration.ActivateCrs.selector, "4th crsgen response should not emit ActivateCrs");
    }

    function test_fullKeygenCycleMultiSigner() public {
        _switchToMultiSignerContext();

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        // prepKeygen: first 2 responses don't trigger KeygenRequest
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doPrepKeygenResponse(prepKeygenId, kmsPk1, kmsTxSender1);

        // 3rd prepKeygen response triggers KeygenRequest event (consensus reached)
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.KeygenRequest(prepKeygenId, keyId, extraData);
        _doPrepKeygenResponse(prepKeygenId, kmsPk2, kmsTxSender2);

        // keygen: first 2 don't trigger ActivateKey
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk1, kmsTxSender1);

        // 3rd keygen response triggers ActivateKey
        string[] memory urls = new string[](3);
        urls[0] = "https://s0.example.com";
        urls[1] = "https://s1.example.com";
        urls[2] = "https://s2.example.com";
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.ActivateKey(keyId, urls, digests);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk2, kmsTxSender2);

        assertEq(kmsGeneration.getActiveKeyId(), keyId);
    }
}
