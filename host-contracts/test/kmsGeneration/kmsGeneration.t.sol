// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {IERC5267} from "@openzeppelin/contracts/interfaces/IERC5267.sol";

import {KMSGeneration} from "@fhevm-host-contracts/contracts/KMSGeneration.sol";
import {IKMSGeneration} from "@fhevm-host-contracts/contracts/interfaces/IKMSGeneration.sol";
import {IKMSGenerationMigration} from "@fhevm-host-contracts/contracts/interfaces/IKMSGenerationMigration.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsNode} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {aclAdd, protocolConfigAdd, kmsGenerationAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {
    HostContractsDeployerTestUtils,
    DeployableERC1967Proxy
} from "@fhevm-host-contracts/fhevm-foundry/HostContractsDeployerTestUtils.sol";

contract KMSGenerationTest is HostContractsDeployerTestUtils {
    KMSGeneration internal kmsGeneration;
    ProtocolConfig internal protocolConfig;

    // Counter bases (matching contract)
    uint256 internal constant PREP_KEYGEN_COUNTER_BASE = uint256(3) << 248;
    uint256 internal constant KEY_COUNTER_BASE = uint256(4) << 248;
    uint256 internal constant CRS_COUNTER_BASE = uint256(5) << 248;
    uint256 internal constant KMS_CONTEXT_COUNTER_BASE = uint256(0x07) << 248;

    // EIP-712 type hashes (with extraData)
    bytes32 internal constant EIP712_PREP_KEYGEN_TYPE_HASH =
        keccak256("PrepKeygenVerification(uint256 prepKeygenId,bytes extraData)");
    bytes32 internal constant EIP712_KEY_DIGEST_TYPE_HASH = keccak256("KeyDigest(uint8 keyType,bytes digest)");
    bytes32 internal constant EIP712_KEYGEN_TYPE_HASH = keccak256(
        "KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)"
    );
    bytes32 internal constant EIP712_CRSGEN_TYPE_HASH =
        keccak256("CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)");

    address internal constant owner = address(456);

    // KMS node private keys for signing
    uint256 internal constant kmsPk0 = 0x100;
    uint256 internal constant kmsPk1 = 0x200;
    address internal kmsSigner0;
    address internal kmsSigner1;
    address internal kmsTxSender0 = address(0xA1);
    address internal kmsTxSender1 = address(0xA2);

    function setUp() public {
        kmsSigner0 = vm.addr(kmsPk0);
        kmsSigner1 = vm.addr(kmsPk1);

        // Deploy ACL
        _deployACL(owner);

        // Deploy ProtocolConfig with our test KMS nodes
        KmsNode[] memory nodes = new KmsNode[](2);
        nodes[0] = KmsNode({
            txSenderAddress: kmsTxSender0,
            signerAddress: kmsSigner0,
            ipAddress: "127.0.0.1",
            storageUrl: "https://s0.example.com"
        });
        nodes[1] = KmsNode({
            txSenderAddress: kmsTxSender1,
            signerAddress: kmsSigner1,
            ipAddress: "127.0.0.2",
            storageUrl: "https://s1.example.com"
        });
        IProtocolConfig.KmsThresholds memory thresholds =
            IProtocolConfig.KmsThresholds({decryptionThreshold: 1, kmsGenThreshold: 1});
        _deployProtocolConfig(owner, nodes, thresholds);
        protocolConfig = ProtocolConfig(protocolConfigAdd);

        // Deploy KMSGeneration
        _deployKMSGeneration(owner);
        kmsGeneration = KMSGeneration(kmsGenerationAdd);
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    function _computeDomainSeparator(address verifier) internal view returns (bytes32) {
        (, string memory name, string memory version, uint256 chainId, address verifyingContract,,) =
            IERC5267(verifier).eip712Domain();

        return keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes(name)),
                keccak256(bytes(version)),
                chainId,
                verifyingContract
            )
        );
    }

    function _computeDomainSeparator() internal view returns (bytes32) {
        return _computeDomainSeparator(kmsGenerationAdd);
    }

    function _signDigest(uint256 privateKey, bytes32 digest) internal pure returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, digest);
        return abi.encodePacked(r, s, v);
    }

    function _buildExtraData() internal view returns (bytes memory) {
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        return abi.encodePacked(uint8(0x02), contextId, uint256(0));
    }

    function _hashPrepKeygen(uint256 prepKeygenId, bytes memory extraData) internal view returns (bytes32) {
        return _hashPrepKeygen(kmsGenerationAdd, prepKeygenId, extraData);
    }

    function _hashPrepKeygen(address verifier, uint256 prepKeygenId, bytes memory extraData)
        internal
        view
        returns (bytes32)
    {
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

    function _hashCrsgen(uint256 crsId, uint256 maxBitLength, bytes memory crsDigest, bytes memory extraData)
        internal
        view
        returns (bytes32)
    {
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

    /// @dev Run a full keygen cycle: keygen() -> prepKeygenResponse -> keygenResponse for one node
    function _runFullKeygenCycle() internal returns (uint256 keyId) {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        keyId = KEY_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();

        // Prep-keygen response
        bytes32 prepDigest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory prepSig = _signDigest(kmsPk0, prepDigest);
        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId, prepSig);

        // Keygen response
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _signDigest(kmsPk0, keyDigest);
        vm.prank(kmsTxSender0);
        kmsGeneration.keygenResponse(keyId, digests, keySig);
    }

    /// @dev Run a full CRS cycle
    function _runFullCrsCycle() internal returns (uint256 crsId) {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        crsId = CRS_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        bytes memory crsDigestData = hex"deadbeef";

        bytes32 digest = _hashCrsgen(crsId, 4096, crsDigestData, extraData);
        bytes memory sig = _signDigest(kmsPk0, digest);
        vm.prank(kmsTxSender0);
        kmsGeneration.crsgenResponse(crsId, crsDigestData, sig);
    }

    function _defaultMigrationState() internal view returns (IKMSGenerationMigration.MigrationState memory state) {
        IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        bytes memory extraData = abi.encodePacked(uint8(0x02), contextId, uint256(0));

        address[] memory keyTxSenders = new address[](1);
        keyTxSenders[0] = kmsTxSender0;
        address[] memory crsTxSenders = new address[](1);
        crsTxSenders[0] = kmsTxSender0;
        address[] memory prepTxSenders = new address[](1);
        prepTxSenders[0] = kmsTxSender0;

        state = IKMSGenerationMigration.MigrationState({
            prepKeygenCounter: PREP_KEYGEN_COUNTER_BASE + 1,
            keyCounter: KEY_COUNTER_BASE + 1,
            crsCounter: CRS_COUNTER_BASE + 1,
            activeKeyId: KEY_COUNTER_BASE + 1,
            activeCrsId: CRS_COUNTER_BASE + 1,
            activePrepKeygenId: PREP_KEYGEN_COUNTER_BASE + 1,
            activeKeyIdForPairing: KEY_COUNTER_BASE + 1,
            activeKeyDigests: keyDigests,
            activeCrsDigest: hex"deadbeef",
            keyConsensusTxSenders: keyTxSenders,
            keyConsensusDigest: keccak256("key-consensus"),
            crsConsensusTxSenders: crsTxSenders,
            crsConsensusDigest: keccak256("crs-consensus"),
            prepKeygenConsensusTxSenders: prepTxSenders,
            prepKeygenConsensusDigest: keccak256("prep-consensus"),
            isPrepKeygenDone: true,
            isKeygenDone: true,
            isCrsgenDone: true,
            crsMaxBitLength: 4096,
            prepKeygenParamsType: IKMSGeneration.ParamsType.Default,
            crsParamsType: IKMSGeneration.ParamsType.Default,
            prepKeygenExtraData: extraData,
            keygenExtraData: extraData,
            crsgenExtraData: extraData
        });
    }

    function _deployMigratedKmsGeneration(IKMSGenerationMigration.MigrationState memory state)
        internal
        returns (KMSGeneration migrated)
    {
        address emptyImpl = address(new EmptyUUPSProxy());
        address migrateProxy =
            address(new DeployableERC1967Proxy(emptyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())));

        address kmsGenImpl = address(new KMSGeneration());

        vm.prank(owner);
        EmptyUUPSProxy(migrateProxy).upgradeToAndCall(
            kmsGenImpl, abi.encodeCall(IKMSGenerationMigration.initializeFromMigration, (state))
        );

        migrated = KMSGeneration(migrateProxy);
    }

    // -----------------------------------------------------------------------
    // Init tests
    // -----------------------------------------------------------------------

    function test_initSuccess() public view {
        assertEq(kmsGeneration.getVersion(), "KMSGeneration v0.1.0");
        assertEq(kmsGeneration.getActiveKeyId(), 0);
        assertEq(kmsGeneration.getActiveCrsId(), 0);
        assertFalse(kmsGeneration.hasPendingKeyManagementRequest());
    }

    // -----------------------------------------------------------------------
    // Full keygen cycle
    // -----------------------------------------------------------------------

    function test_fullKeygenCycle() public {
        uint256 keyId = _runFullKeygenCycle();
        assertEq(kmsGeneration.getActiveKeyId(), keyId);
        assertEq(uint256(kmsGeneration.getKeyParamsType(keyId)), uint256(IKMSGeneration.ParamsType.Default));
        assertFalse(kmsGeneration.hasPendingKeyManagementRequest());
    }

    function test_keygenMaterials() public {
        uint256 keyId = _runFullKeygenCycle();
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

    // -----------------------------------------------------------------------
    // Full CRS cycle
    // -----------------------------------------------------------------------

    function test_fullCrsCycle() public {
        uint256 crsId = _runFullCrsCycle();
        assertEq(kmsGeneration.getActiveCrsId(), crsId);
        assertEq(uint256(kmsGeneration.getCrsParamsType(crsId)), uint256(IKMSGeneration.ParamsType.Default));
        assertFalse(kmsGeneration.hasPendingKeyManagementRequest());
    }

    function test_emitKeygenLifecycleEvents() public {
        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();

        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.PrepKeygenRequest(prepKeygenId, 0, IKMSGeneration.ParamsType.Default, extraData);
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        bytes32 prepDigest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory prepSig = _signDigest(kmsPk0, prepDigest);
        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId, prepSig);

        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _signDigest(kmsPk0, keyDigest);
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
        bytes memory sig = _signDigest(kmsPk0, digest);
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
    // hasPendingKeyManagementRequest
    // -----------------------------------------------------------------------

    function test_pendingKeyManagementRequest() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);
        assertTrue(kmsGeneration.hasPendingKeyManagementRequest());
    }

    function test_pendingCrsRequest() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
        assertTrue(kmsGeneration.hasPendingKeyManagementRequest());
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

    function test_revertReplayAcrossContexts() public {
        bytes memory oldExtraData = _buildExtraData();

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);
        vm.prank(owner);
        kmsGeneration.abortKeygen(KEY_COUNTER_BASE + 1);

        KmsNode[] memory newNodes = new KmsNode[](2);
        newNodes[0] = KmsNode({
            txSenderAddress: kmsTxSender0,
            signerAddress: kmsSigner0,
            ipAddress: "127.0.0.1",
            storageUrl: "https://s0.example.com"
        });
        newNodes[1] = KmsNode({
            txSenderAddress: kmsTxSender1,
            signerAddress: kmsSigner1,
            ipAddress: "127.0.0.2",
            storageUrl: "https://s1.example.com"
        });

        vm.prank(owner);
        protocolConfig.defineNewKmsContext(
            newNodes, IProtocolConfig.KmsThresholds({decryptionThreshold: 1, kmsGenThreshold: 1})
        );

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 2;
        bytes32 replayDigest = _hashPrepKeygen(prepKeygenId, oldExtraData);
        bytes memory replaySig = _signDigest(kmsPk0, replayDigest);

        vm.prank(kmsTxSender0);
        vm.expectRevert();
        kmsGeneration.prepKeygenResponse(prepKeygenId, replaySig);
    }

    // -----------------------------------------------------------------------
    // Access control
    // -----------------------------------------------------------------------

    function test_revertKeygenNotOwner() public {
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);
    }

    function test_revertCrsgenNotOwner() public {
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
    }

    function test_revertPrepKeygenResponseNotTxSender() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotKmsTxSender.selector, address(0x999)));
        kmsGeneration.prepKeygenResponse(PREP_KEYGEN_COUNTER_BASE + 1, hex"");
    }

    function test_revertKeygenResponseNotTxSender() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotKmsTxSender.selector, address(0x999)));
        kmsGeneration.keygenResponse(KEY_COUNTER_BASE + 1, digests, hex"");
    }

    function test_revertCrsgenResponseNotTxSender() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotKmsTxSender.selector, address(0x999)));
        kmsGeneration.crsgenResponse(CRS_COUNTER_BASE + 1, hex"deadbeef", hex"");
    }

    // -----------------------------------------------------------------------
    // Duplicate signature rejection
    // -----------------------------------------------------------------------

    function test_revertDuplicatePrepKeygenSignature() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        bytes32 digest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory sig = _signDigest(kmsPk0, digest);

        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId, sig);

        vm.prank(kmsTxSender0);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsAlreadySignedForPrepKeygen.selector, prepKeygenId, kmsSigner0)
        );
        kmsGeneration.prepKeygenResponse(prepKeygenId, sig);
    }

    function test_revertDuplicateKeygenSignature() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        bytes32 prepDigest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory prepSig = _signDigest(kmsPk0, prepDigest);

        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId, prepSig);

        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _signDigest(kmsPk0, keyDigest);

        vm.prank(kmsTxSender0);
        kmsGeneration.keygenResponse(keyId, digests, keySig);

        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KmsAlreadySignedForKeygen.selector, keyId, kmsSigner0));
        kmsGeneration.keygenResponse(keyId, digests, keySig);
    }

    function test_revertDuplicateCrsgenSignature() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        bytes memory crsDigestData = hex"deadbeef";
        bytes32 digest = _hashCrsgen(crsId, 4096, crsDigestData, extraData);
        bytes memory sig = _signDigest(kmsPk0, digest);

        vm.prank(kmsTxSender0);
        kmsGeneration.crsgenResponse(crsId, crsDigestData, sig);

        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KmsAlreadySignedForCrsgen.selector, crsId, kmsSigner0));
        kmsGeneration.crsgenResponse(crsId, crsDigestData, sig);
    }

    // -----------------------------------------------------------------------
    // Invalid request ID
    // -----------------------------------------------------------------------

    function test_revertPrepKeygenNotRequested() public {
        vm.prank(kmsTxSender0);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.PrepKeygenNotRequested.selector, PREP_KEYGEN_COUNTER_BASE + 99)
        );
        kmsGeneration.prepKeygenResponse(PREP_KEYGEN_COUNTER_BASE + 99, hex"");
    }

    function test_revertKeygenNotRequested() public {
        IKMSGeneration.KeyDigest[] memory d = _mockKeyDigests();
        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeygenNotRequested.selector, KEY_COUNTER_BASE + 99));
        kmsGeneration.keygenResponse(KEY_COUNTER_BASE + 99, d, hex"");
    }

    function test_revertCrsgenNotRequested() public {
        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsgenNotRequested.selector, CRS_COUNTER_BASE + 99));
        kmsGeneration.crsgenResponse(CRS_COUNTER_BASE + 99, hex"deadbeef", hex"");
    }

    function test_revertKmsSignerDoesNotMatchTxSender() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        bytes32 prepDigest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory prepSig = _signDigest(kmsPk0, prepDigest);

        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId, prepSig);

        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _signDigest(kmsPk1, keyDigest);

        vm.prank(kmsTxSender0);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsSignerDoesNotMatchTxSender.selector, kmsSigner1, kmsTxSender0)
        );
        kmsGeneration.keygenResponse(keyId, digests, keySig);
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
        assertTrue(kmsGeneration.hasPendingKeyManagementRequest());

        uint256 keyId = KEY_COUNTER_BASE + 1;
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.AbortKeygen(keyId);
        vm.prank(owner);
        kmsGeneration.abortKeygen(keyId);

        assertFalse(kmsGeneration.hasPendingKeyManagementRequest());
    }

    function test_abortCrsgen() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
        assertTrue(kmsGeneration.hasPendingKeyManagementRequest());

        uint256 crsId = CRS_COUNTER_BASE + 1;
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.AbortCrsgen(crsId);
        vm.prank(owner);
        kmsGeneration.abortCrsgen(crsId);

        assertFalse(kmsGeneration.hasPendingKeyManagementRequest());
    }

    function test_revertAbortKeygenInvalidId() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortKeygenInvalidId.selector, KEY_COUNTER_BASE + 99));
        kmsGeneration.abortKeygen(KEY_COUNTER_BASE + 99);
    }

    function test_revertAbortKeygenAlreadyDone() public {
        uint256 keyId = _runFullKeygenCycle();
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortKeygenAlreadyDone.selector, keyId));
        kmsGeneration.abortKeygen(keyId);
    }

    function test_revertAbortCrsgenInvalidId() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortCrsgenInvalidId.selector, CRS_COUNTER_BASE + 99));
        kmsGeneration.abortCrsgen(CRS_COUNTER_BASE + 99);
    }

    function test_revertAbortCrsgenAlreadyDone() public {
        uint256 crsId = _runFullCrsCycle();
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortCrsgenAlreadyDone.selector, crsId));
        kmsGeneration.abortCrsgen(crsId);
    }

    function test_abortKeygenNotOwner() public {
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        kmsGeneration.abortKeygen(KEY_COUNTER_BASE + 1);
    }

    function test_abortCrsgenNotOwner() public {
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        kmsGeneration.abortCrsgen(CRS_COUNTER_BASE + 1);
    }

    function test_keygenAfterAbort() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 abortedKeyId = KEY_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortKeygen(abortedKeyId);

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Test);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 keyId = KEY_COUNTER_BASE + 2;
        bytes memory extraData = _buildExtraData();
        bytes32 prepDigest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory prepSig = _signDigest(kmsPk0, prepDigest);
        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId, prepSig);

        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _signDigest(kmsPk0, keyDigest);
        vm.prank(kmsTxSender0);
        kmsGeneration.keygenResponse(keyId, digests, keySig);

        assertEq(kmsGeneration.getActiveKeyId(), keyId);
        assertEq(uint256(kmsGeneration.getKeyParamsType(keyId)), uint256(IKMSGeneration.ParamsType.Test));
        assertFalse(kmsGeneration.hasPendingKeyManagementRequest());
    }

    function test_revertAbortedKeyViews() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        uint256 keyId = KEY_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortKeygen(keyId);

        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyNotGenerated.selector, keyId));
        kmsGeneration.getKeyParamsType(keyId);

        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyNotGenerated.selector, keyId));
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
        bytes memory extraData = _buildExtraData();
        bytes memory crsDigestData = hex"cafebabe";
        bytes32 digest = _hashCrsgen(crsId, 8192, crsDigestData, extraData);
        bytes memory sig = _signDigest(kmsPk0, digest);
        vm.prank(kmsTxSender0);
        kmsGeneration.crsgenResponse(crsId, crsDigestData, sig);

        assertEq(kmsGeneration.getActiveCrsId(), crsId);
        assertEq(uint256(kmsGeneration.getCrsParamsType(crsId)), uint256(IKMSGeneration.ParamsType.Test));
        assertFalse(kmsGeneration.hasPendingKeyManagementRequest());
    }

    function test_revertAbortedCrsViews() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortCrsgen(crsId);

        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsNotGenerated.selector, crsId));
        kmsGeneration.getCrsParamsType(crsId);

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
        bytes memory extraData = _buildExtraData();

        bytes32 prepDigest = _hashPrepKeygen(prepKeygenId2, extraData);
        bytes memory prepSig = _signDigest(kmsPk0, prepDigest);
        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId2, prepSig);

        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes32 keyDigest = _hashKeygen(prepKeygenId2, keyId2, digests, extraData);
        bytes memory keySig = _signDigest(kmsPk0, keyDigest);
        vm.prank(kmsTxSender0);
        kmsGeneration.keygenResponse(keyId2, digests, keySig);

        assertEq(kmsGeneration.getActiveKeyId(), keyId2);
        assertEq(uint256(kmsGeneration.getKeyParamsType(keyId2)), uint256(IKMSGeneration.ParamsType.Test));
    }

    // -----------------------------------------------------------------------
    // In-flight guard integration
    // -----------------------------------------------------------------------

    function test_protocolConfigBlocksContextWhenPending() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);

        KmsNode[] memory newNodes = new KmsNode[](1);
        newNodes[0] =
            KmsNode({txSenderAddress: address(0xC1), signerAddress: address(0xD1), ipAddress: "", storageUrl: ""});
        IProtocolConfig.KmsThresholds memory thresholds =
            IProtocolConfig.KmsThresholds({decryptionThreshold: 1, kmsGenThreshold: 1});

        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.KeyManagementRequestInFlight.selector);
        protocolConfig.defineNewKmsContext(newNodes, thresholds);
    }

    // -----------------------------------------------------------------------
    // Migration reinitializer
    // -----------------------------------------------------------------------

    function test_migrationReinitializer() public {
        IKMSGenerationMigration.MigrationState memory state = _defaultMigrationState();
        KMSGeneration migrated = _deployMigratedKmsGeneration(state);
        assertEq(migrated.getVersion(), "KMSGeneration v0.1.0");
        assertEq(migrated.getActiveKeyId(), KEY_COUNTER_BASE + 1);
        assertEq(migrated.getActiveCrsId(), CRS_COUNTER_BASE + 1);
        assertFalse(migrated.hasPendingKeyManagementRequest());

        // Verify material lookups work post-migration
        (string[] memory keyUrls, IKMSGeneration.KeyDigest[] memory migratedDigests) =
            migrated.getKeyMaterials(KEY_COUNTER_BASE + 1);
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
        IKMSGenerationMigration.MigrationState memory state = _defaultMigrationState();
        KMSGeneration migrated = _deployMigratedKmsGeneration(state);

        bytes32 keyDigest = _hashKeygen(
            address(migrated), state.activePrepKeygenId, state.activeKeyId, _mockKeyDigests(), state.keygenExtraData
        );
        bytes memory keySig = _signDigest(kmsPk0, keyDigest);

        vm.prank(kmsTxSender0);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsAlreadySignedForKeygen.selector, state.activeKeyId, kmsSigner0)
        );
        migrated.keygenResponse(state.activeKeyId, _mockKeyDigests(), keySig);
    }

    function test_migrationStoresUnpairedActiveKeyParamsType() public {
        IKMSGenerationMigration.MigrationState memory state = _defaultMigrationState();
        state.activePrepKeygenId = 0;
        state.activeKeyIdForPairing = 0;
        state.prepKeygenConsensusTxSenders = new address[](0);
        state.prepKeygenConsensusDigest = bytes32(0);
        state.isPrepKeygenDone = false;
        state.prepKeygenExtraData = hex"";
        state.prepKeygenParamsType = IKMSGeneration.ParamsType.Test;

        KMSGeneration migrated = _deployMigratedKmsGeneration(state);

        assertEq(uint256(migrated.getKeyParamsType(state.activeKeyId)), uint256(IKMSGeneration.ParamsType.Test));
    }

    function test_migrationAllowsEmptyExtraDataFallback() public {
        IKMSGenerationMigration.MigrationState memory state = _defaultMigrationState();
        state.prepKeygenExtraData = hex"";
        state.keygenExtraData = hex"";
        state.crsgenExtraData = hex"";

        KMSGeneration migrated = _deployMigratedKmsGeneration(state);

        (string[] memory keyUrls,) = migrated.getKeyMaterials(state.activeKeyId);
        assertEq(keyUrls.length, 1);
        assertEq(keyUrls[0], "https://s0.example.com");

        (string[] memory crsUrls,) = migrated.getCrsMaterials(state.activeCrsId);
        assertEq(crsUrls.length, 1);
        assertEq(crsUrls[0], "https://s0.example.com");
    }

    function test_revertMigrationMalformedVersionedExtraData() public {
        IKMSGenerationMigration.MigrationState memory state = _defaultMigrationState();
        bytes memory malformedExtraData = abi.encodePacked(uint8(0x02), uint8(0x42));
        state.prepKeygenExtraData = malformedExtraData;
        state.keygenExtraData = malformedExtraData;
        state.crsgenExtraData = malformedExtraData;

        address emptyImpl = address(new EmptyUUPSProxy());
        address migrateProxy =
            address(new DeployableERC1967Proxy(emptyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())));
        address kmsGenImpl = address(new KMSGeneration());

        vm.prank(owner);
        vm.expectRevert(IKMSGeneration.DeserializingExtraDataFail.selector);
        EmptyUUPSProxy(migrateProxy).upgradeToAndCall(
            kmsGenImpl, abi.encodeCall(IKMSGenerationMigration.initializeFromMigration, (state))
        );
    }

    function test_migrationAllowsLegacyExplicitContextExtraData() public {
        IKMSGenerationMigration.MigrationState memory state = _defaultMigrationState();
        bytes memory legacyExtraData = abi.encodePacked(uint8(0x01), bytes32(protocolConfig.getCurrentKmsContextId()));
        state.prepKeygenExtraData = legacyExtraData;
        state.keygenExtraData = legacyExtraData;
        state.crsgenExtraData = legacyExtraData;

        KMSGeneration migrated = _deployMigratedKmsGeneration(state);

        (string[] memory keyUrls,) = migrated.getKeyMaterials(state.activeKeyId);
        assertEq(keyUrls.length, 1);
        assertEq(keyUrls[0], "https://s0.example.com");

        (string[] memory crsUrls,) = migrated.getCrsMaterials(state.activeCrsId);
        assertEq(crsUrls.length, 1);
        assertEq(crsUrls[0], "https://s0.example.com");
    }

    function test_revertMigrationUnsupportedExtraDataVersion() public {
        IKMSGenerationMigration.MigrationState memory state = _defaultMigrationState();
        bytes memory unsupportedExtraData = abi.encodePacked(uint8(0x03), bytes32(state.activeKeyId));
        state.prepKeygenExtraData = unsupportedExtraData;
        state.keygenExtraData = unsupportedExtraData;
        state.crsgenExtraData = unsupportedExtraData;

        address emptyImpl = address(new EmptyUUPSProxy());
        address migrateProxy =
            address(new DeployableERC1967Proxy(emptyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())));
        address kmsGenImpl = address(new KMSGeneration());

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.UnsupportedExtraDataVersion.selector, uint8(0x03)));
        EmptyUUPSProxy(migrateProxy).upgradeToAndCall(
            kmsGenImpl, abi.encodeCall(IKMSGenerationMigration.initializeFromMigration, (state))
        );
    }
}
