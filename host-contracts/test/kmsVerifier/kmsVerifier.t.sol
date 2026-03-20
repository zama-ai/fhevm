// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test, Vm} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import {KMSVerifier} from "../../contracts/KMSVerifier.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {fhevmExecutorAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";
import {aclAdd} from "../../addresses/FHEVMHostAddresses.sol";

contract KMSVerifierTest is Test {
    KMSVerifier internal kmsVerifier;

    uint256 internal constant KMS_CONTEXT_COUNTER_BASE = uint256(0x07) << 248;
    uint256 internal constant EPOCH_COUNTER_BASE = uint256(0x08) << 248;
    bytes32 internal constant KMS_VERIFIER_STORAGE_SLOT =
        0x7e81a744be86773af8644dd7304fa1dc9350ccabf16cfcaa614ddb78b4ce8900;
    bytes32 internal constant OZ_INITIALIZABLE_STORAGE_SLOT =
        0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;
    bytes32 internal constant NEW_CONTEXT_SET_TOPIC =
        keccak256(
            "NewContextSet(uint256,uint256,string,(bytes,bytes,bytes)[],(string,int32,bytes,string,bytes,string,string,bytes[])[])"
        );

    uint256 internal constant initialThreshold = 1;
    address internal constant verifyingContractSource = address(10000);
    address internal constant owner = address(456);

    // EIP-712 type hashes (mirrors contract constants)
    bytes32 internal constant EIP712_DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
    bytes32 internal constant CONTEXT_CREATION_TYPEHASH = keccak256("ContextCreationConfirmation(uint256 contextId)");
    bytes32 internal constant KEY_DIGEST_TYPEHASH = keccak256("KeyDigest(uint8 keyType,bytes digest)");
    bytes32 internal constant KEYGEN_VERIFICATION_TYPEHASH =
        keccak256(
            "KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)"
        );

    /// @dev Signer variables.
    uint256 internal constant privateKeySigner0 = 0x022;
    uint256 internal constant privateKeySigner1 = 0x03;
    uint256 internal constant privateKeySigner2 = 0x04;
    uint256 internal constant privateKeySigner3 = 0x05;
    uint256 internal constant privateKeySigner4 = 0x06;
    address[] internal activeSigners;

    mapping(address => uint256) internal signerPrivateKeys;
    address internal signer0;
    address internal signer1;
    address internal signer2;
    address internal signer3;
    address internal signer4;

    /// @dev Proxy and implementation variables
    address internal proxy;
    address internal implementation;

    // =========================================================================
    //  Helpers
    // =========================================================================

    function _computeSignature(uint256 privateKey, bytes32 digest) internal pure returns (bytes memory signature) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, digest);
        return abi.encodePacked(r, s, v);
    }

    function _computeDigest(
        bytes32[] memory handlesList,
        bytes memory decryptedResult,
        bytes memory extraData
    ) internal view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                kmsVerifier.DECRYPTION_RESULT_TYPEHASH(),
                keccak256(abi.encodePacked(handlesList)),
                keccak256(decryptedResult),
                keccak256(abi.encodePacked(extraData))
            )
        );
        bytes32 hashTypeData = MessageHashUtils.toTypedDataHash(_computeDecryptionDomainSeparator(), structHash);
        return hashTypeData;
    }

    /// @dev Decryption domain separator (unchanged from existing code).
    function _computeDecryptionDomainSeparator() internal view returns (bytes32) {
        (, string memory name, string memory version, uint256 chainId, address verifyingContract, , ) = kmsVerifier
            .eip712Domain();
        return
            keccak256(
                abi.encode(
                    EIP712_DOMAIN_TYPEHASH,
                    keccak256(bytes(name)),
                    keccak256(bytes(version)),
                    chainId,
                    verifyingContract
                )
            );
    }

    /// @dev KMSVerifier native domain separator.
    function _computeKmsVerifierDomainSeparator() internal view returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EIP712_DOMAIN_TYPEHASH,
                    keccak256("KMSVerifier"),
                    keccak256("1"),
                    block.chainid,
                    address(kmsVerifier)
                )
            );
    }

    function _computeContextCreationDigest(uint256 contextId) internal view returns (bytes32) {
        bytes32 structHash = keccak256(abi.encode(CONTEXT_CREATION_TYPEHASH, contextId));
        return keccak256(abi.encodePacked("\x19\x01", _computeKmsVerifierDomainSeparator(), structHash));
    }

    function _buildExtraData(uint8 version, uint256 contextId, uint256 epochId) internal pure returns (bytes memory) {
        return abi.encodePacked(version, contextId, epochId);
    }

    function _computeKeygenStructHash(
        uint256 prepKeygenId,
        uint256 keyId,
        KMSVerifier.KeyDigest[] memory keyDigests,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        bytes32[] memory keyDigestHashes = new bytes32[](keyDigests.length);
        for (uint256 i = 0; i < keyDigests.length; i++) {
            keyDigestHashes[i] = keccak256(
                abi.encode(KEY_DIGEST_TYPEHASH, keyDigests[i].keyType, keccak256(keyDigests[i].digest))
            );
        }
        return
            keccak256(
                abi.encode(
                    KEYGEN_VERIFICATION_TYPEHASH,
                    prepKeygenId,
                    keyId,
                    keccak256(abi.encodePacked(keyDigestHashes)),
                    keccak256(extraData)
                )
            );
    }

    function _computeKeygenDigest(
        uint256 prepKeygenId,
        uint256 keyId,
        KMSVerifier.KeyDigest[] memory keyDigests,
        bytes memory extraData
    ) internal view returns (bytes32) {
        bytes32 structHash = _computeKeygenStructHash(prepKeygenId, keyId, keyDigests, extraData);
        return keccak256(abi.encodePacked("\x19\x01", _computeKmsVerifierDomainSeparator(), structHash));
    }

    function _mockKeyDigests() internal pure returns (KMSVerifier.KeyDigest[] memory) {
        KMSVerifier.KeyDigest[] memory kd = new KMSVerifier.KeyDigest[](1);
        kd[0] = KMSVerifier.KeyDigest({keyType: KMSVerifier.KeyType.Public, digest: hex"aabbccdd"});
        return kd;
    }

    /// @dev Builds MpcNode array from private keys, using the actual secp256k1 public key as verificationKey.
    ///      The contract derives addresses via keccak256(verificationKey), which matches vm.addr(pk).
    function _mockMpcNodes(uint256[] memory privateKeys) internal returns (KMSVerifier.MpcNode[] memory) {
        KMSVerifier.MpcNode[] memory nodes = new KMSVerifier.MpcNode[](privateKeys.length);
        for (uint256 i = 0; i < privateKeys.length; i++) {
            Vm.Wallet memory w = vm.createWallet(privateKeys[i]);
            nodes[i] = KMSVerifier.MpcNode({
                mpcIdentity: string(abi.encodePacked("node-", i)),
                partyId: int32(int256(i)),
                verificationKey: abi.encodePacked(w.publicKeyX, w.publicKeyY),
                externalUrl: "https://example.com",
                caCert: hex"deadbeef",
                publicStorageUrl: "https://storage.example.com",
                publicStoragePrefix: "prefix",
                extraVerificationKeys: new bytes[](0)
            });
        }
        return nodes;
    }

    function _mockPcrValues() internal pure returns (KMSVerifier.PcrValues[] memory) {
        KMSVerifier.PcrValues[] memory pcrs = new KMSVerifier.PcrValues[](1);
        pcrs[0] = KMSVerifier.PcrValues({pcr0: hex"aa", pcr1: hex"bb", pcr2: hex"cc"});
        return pcrs;
    }

    function _emptyPcrValues() internal pure returns (KMSVerifier.PcrValues[] memory) {
        return new KMSVerifier.PcrValues[](0);
    }

    /// @dev Convenience: builds MpcNodes for a single private key.
    function _singleMpcNode(uint256 privateKey) internal returns (KMSVerifier.MpcNode[] memory) {
        uint256[] memory pks = new uint256[](1);
        pks[0] = privateKey;
        return _mockMpcNodes(pks);
    }

    /// @dev Convenience: builds MpcNodes for two private keys.
    function _twoMpcNodes(uint256 pk1, uint256 pk2) internal returns (KMSVerifier.MpcNode[] memory) {
        uint256[] memory pks = new uint256[](2);
        pks[0] = pk1;
        pks[1] = pk2;
        return _mockMpcNodes(pks);
    }

    /// @dev Builds MpcNodes from the currently configured activeSigners.
    function _activeSignerMpcNodes() internal returns (KMSVerifier.MpcNode[] memory) {
        uint256[] memory pks = new uint256[](activeSigners.length);
        for (uint256 i = 0; i < activeSigners.length; i++) {
            pks[i] = signerPrivateKeys[activeSigners[i]];
        }
        return _mockMpcNodes(pks);
    }

    /// @dev Confirms context creation for the given signers.
    function _confirmContextCreationForAll(uint256 contextId, address[] memory ctxSigners) internal {
        bytes32 digest = _computeContextCreationDigest(contextId);
        for (uint256 i = 0; i < ctxSigners.length; i++) {
            bytes memory sig = _computeSignature(signerPrivateKeys[ctxSigners[i]], digest);
            kmsVerifier.confirmContextCreation(contextId, sig);
        }
    }

    /// @dev Confirms epoch result for the given signers.
    function _confirmEpochResultForAll(uint256 contextId, uint256 epochId, address[] memory ctxSigners) internal {
        KMSVerifier.KeyDigest[] memory keyDigests = _mockKeyDigests();
        bytes memory extraData = _buildExtraData(0x01, contextId, epochId);
        bytes32 digest = _computeKeygenDigest(1, 2, keyDigests, extraData);
        for (uint256 i = 0; i < ctxSigners.length; i++) {
            bytes memory sig = _computeSignature(signerPrivateKeys[ctxSigners[i]], digest);
            kmsVerifier.confirmEpochResult(epochId, 1, 2, keyDigests, extraData, sig);
        }
    }

    /// @dev Full activation: confirmContextCreation + confirmEpochResult for all given signers.
    function _activateContext(uint256 contextId, uint256 epochId, address[] memory ctxSigners) internal {
        _confirmContextCreationForAll(contextId, ctxSigners);
        _confirmEpochResultForAll(contextId, epochId, ctxSigners);
    }

    function _deployProxy() internal {
        proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );
    }

    function _deployAndEtchACL() internal {
        address _acl = address(new ACL());
        bytes memory code = _acl.code;
        vm.etch(aclAdd, code);
        vm.store(
            aclAdd,
            0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300, // OwnableStorageLocation
            bytes32(uint256(uint160(owner)))
        );
    }

    function _upgradeProxy(address[] memory signers) internal {
        implementation = address(new KMSVerifier());
        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(
                kmsVerifier.initializeFromEmptyProxy,
                (
                    verifyingContractSource,
                    uint64(block.chainid),
                    signers,
                    initialThreshold,
                    new KMSVerifier.MpcNode[](0),
                    "",
                    new KMSVerifier.PcrValues[](0)
                )
            ),
            owner
        );
        kmsVerifier = KMSVerifier(proxy);
    }

    function _upgradeProxyWithSigners(uint256 numberSigners) internal {
        assert(numberSigners > 0 && numberSigners < 6);
        if (numberSigners >= 1) activeSigners.push(signer0);
        if (numberSigners >= 2) activeSigners.push(signer1);
        if (numberSigners >= 3) activeSigners.push(signer2);
        if (numberSigners >= 4) activeSigners.push(signer3);
        if (numberSigners == 5) activeSigners.push(signer4);
        _upgradeProxy(activeSigners);
    }

    /// @dev Sets up two ACTIVE contexts. Context 1: signer0-2 (threshold 1). Context 2: signer3 (threshold 1).
    function _setupTwoContexts() internal returns (uint256 ctx1, uint256 ctx2) {
        _upgradeProxyWithSigners(3);
        ctx1 = kmsVerifier.getCurrentKmsContextId();
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "v1.0", _mockPcrValues());
        ctx2 = ctx1 + 1;
        uint256 epoch2 = EPOCH_COUNTER_BASE + 2;
        address[] memory ctx2Signers = new address[](1);
        ctx2Signers[0] = signer3;
        _activateContext(ctx2, epoch2, ctx2Signers);
    }

    function _deployUninitializedKMSVerifierProxy() internal returns (address proxyAddr, KMSVerifier kv) {
        proxyAddr = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );
        address impl = address(new KMSVerifier());
        UnsafeUpgrades.upgradeProxy(proxyAddr, impl, "", owner);
        kv = KMSVerifier(proxyAddr);
    }

    function _generateMockHandlesList(uint256 numberHandles) internal pure returns (bytes32[] memory) {
        assert(numberHandles < 250);
        bytes32[] memory handlesList = new bytes32[](numberHandles);
        for (uint256 i = 0; i < numberHandles; i++) {
            handlesList[i] = bytes32(uint256(i + 1));
        }
        return handlesList;
    }

    function _buildSingleSignerProof(
        uint256 signerKey,
        bytes memory extraData
    ) internal view returns (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) {
        handlesList = _generateMockHandlesList(3);
        decryptedResult = abi.encodePacked(keccak256("test"));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        bytes memory signature = _computeSignature(signerKey, digest);
        proof = abi.encodePacked(uint8(1), signature, extraData);
    }

    function _initializeSigners() internal {
        signer0 = vm.addr(privateKeySigner0);
        signer1 = vm.addr(privateKeySigner1);
        signer2 = vm.addr(privateKeySigner2);
        signer3 = vm.addr(privateKeySigner3);
        signer4 = vm.addr(privateKeySigner4);
        signerPrivateKeys[signer0] = privateKeySigner0;
        signerPrivateKeys[signer1] = privateKeySigner1;
        signerPrivateKeys[signer2] = privateKeySigner2;
        signerPrivateKeys[signer3] = privateKeySigner3;
        signerPrivateKeys[signer4] = privateKeySigner4;
    }

    function _hasNewContextSetEvent(Vm.Log[] memory logs) internal pure returns (bool) {
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics.length > 0 && logs[i].topics[0] == NEW_CONTEXT_SET_TOPIC) {
                return true;
            }
        }
        return false;
    }

    function setUp() public {
        _deployProxy();
        _deployAndEtchACL();
        _initializeSigners();
    }

    // =========================================================================
    //  Basic proxy / version tests
    // =========================================================================

    function test_PostProxyUpgradeCheck() public {
        _upgradeProxyWithSigners(3);
        assertEq(kmsVerifier.getVersion(), "KMSVerifier v0.3.0");
        assertEq(kmsVerifier.getThreshold(), initialThreshold);
    }

    function test_GetKmsSignersWorkAsExpected() public {
        _upgradeProxyWithSigners(3);
        address[] memory signers = kmsVerifier.getKmsSigners();
        assertEq(signers.length, 3);
        assertEq(signers[0], signer0);
        assertEq(signers[1], signer1);
        assertEq(signers[2], signer2);
        for (uint256 i = 0; i < 3; i++) {
            assertTrue(kmsVerifier.isSigner(signers[i]));
        }
    }

    function test_OnlyOwnerCanDefineNewContextAndEpoch(address randomAccount) public {
        vm.assume(randomAccount != owner);
        _upgradeProxyWithSigners(3);
        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        vm.prank(randomAccount);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());
    }

    function test_OwnerCanAddNewSigner() public {
        _upgradeProxyWithSigners(3);
        uint256 expectedContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "v1.0", _mockPcrValues());
        // Context is PENDING — getSignersForKmsContext returns empty for non-ACTIVE contexts
        address[] memory ctx2Signers = kmsVerifier.getSignersForKmsContext(expectedContextId);
        assertEq(ctx2Signers.length, 0);
        // Active context hasn't changed
        assertEq(kmsVerifier.getCurrentKmsContextId(), KMS_CONTEXT_COUNTER_BASE + 1);
    }

    function test_OwnerCannotAddSameSignerTwice() public {
        _upgradeProxyWithSigners(3);
        // Two MpcNodes with the same verificationKey derive the same address → KMSAlreadySigner
        KMSVerifier.MpcNode[] memory nodes = _twoMpcNodes(privateKeySigner3, privateKeySigner3);
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.KMSAlreadySigner.selector);
        kmsVerifier.defineNewContextAndEpoch(nodes, 1, "", _emptyPcrValues());
    }

    function test_OwnerCannotDefineContextWithEmptyNodes() public {
        _upgradeProxyWithSigners(3);
        KMSVerifier.MpcNode[] memory emptyNodes = new KMSVerifier.MpcNode[](0);
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.SignersSetIsEmpty.selector);
        kmsVerifier.defineNewContextAndEpoch(emptyNodes, 0, "", _emptyPcrValues());
    }

    /// @dev This function exists for the test below to call it externally.
    function upgrade(address randomAccount) external {
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", randomAccount);
    }

    function test_OnlyOwnerCanAuthorizeUpgrade(address randomAccount) public {
        _upgradeProxyWithSigners(3);
        vm.assume(randomAccount != owner);
        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        this.upgrade(randomAccount);
    }

    function test_OnlyOwnerCanAuthorizeUpgrade() public {
        _upgradeProxyWithSigners(3);
        this.upgrade(owner);
    }

    // =========================================================================
    //  Decryption verification tests
    // =========================================================================

    function test_VerifyDecryptionEIP712KMSSignaturesWork() public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);
        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfDigestIsInvalid() public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 invalidDigest = bytes32("420");
        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, invalidDigest);
        signatures[1] = _computeSignature(privateKeySigner2, invalidDigest);
        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );
        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfNoSignerAdded() public {
        _upgradeProxyWithSigners(1);
        bytes32[] memory handlesList = new bytes32[](3);
        handlesList[0] = bytes32(uint256(4));
        handlesList[1] = bytes32(uint256(5));
        handlesList[2] = bytes32(uint256(323));
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);
        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );
        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfNoSignatureProvided() public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes memory decryptionProof = abi.encodePacked(uint8(0), extraData);
        vm.expectPartialRevert(KMSVerifier.KMSZeroSignature.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfNumberOfSignaturesIsInferiorToThreshold() public {
        _upgradeProxyWithSigners(3);
        // Define new context with threshold=2 and activate it
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_activeSignerMpcNodes(), 2, "v1.0", _mockPcrValues());
        uint256 newCtx = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpoch = EPOCH_COUNTER_BASE + 2;
        _activateContext(newCtx, newEpoch, activeSigners);
        assertEq(kmsVerifier.getThreshold(), 2);

        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        bytes[] memory signatures = new bytes[](1);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        bytes memory decryptionProof = abi.encodePacked(uint8(signatures.length), signatures[0], extraData);
        vm.expectPartialRevert(KMSVerifier.KMSSignatureThresholdNotReached.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfSameSignerIsUsedTwice() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_activeSignerMpcNodes(), 2, "v1.0", _mockPcrValues());
        uint256 newCtx = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpoch = EPOCH_COUNTER_BASE + 2;
        _activateContext(newCtx, newEpoch, activeSigners);
        assertEq(kmsVerifier.getThreshold(), 2);

        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner1, digest);
        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );
        assertFalse(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailsIfEmptyDecryptionProof() public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory decryptionProof = new bytes(0);
        vm.expectRevert(KMSVerifier.EmptyDecryptionProof.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailsIfDeserializingDecryptionProofFail(
        uint256 randomValue
    ) public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory decryptionProof = abi.encodePacked(uint8(3), randomValue);
        vm.expectRevert(KMSVerifier.DeserializingDecryptionProofFail.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    /// @dev This function exists for the test below to call it externally.
    function emptyUpgrade() public {
        address[] memory emptySigners = new address[](0);
        implementation = address(new KMSVerifier());
        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(
                KMSVerifier.initializeFromEmptyProxy,
                (
                    verifyingContractSource,
                    uint64(block.chainid),
                    emptySigners,
                    initialThreshold,
                    new KMSVerifier.MpcNode[](0),
                    "",
                    new KMSVerifier.PcrValues[](0)
                )
            ),
            owner
        );
    }

    function test_CannotReinitializeIfInitialSignersSetIsEmpty() public {
        vm.expectPartialRevert(KMSVerifier.SignersSetIsEmpty.selector);
        this.emptyUpgrade();
    }

    // =========================================================================
    //  Context management tests
    // =========================================================================

    function test_DefineNewContextAndEpochCreatesContextAndEpoch() public {
        _upgradeProxyWithSigners(3); // context 1 ACTIVE
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();
        assertEq(ctx1, KMS_CONTEXT_COUNTER_BASE + 1);

        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(
            _twoMpcNodes(privateKeySigner3, privateKeySigner4),
            1,
            "v1.0",
            _mockPcrValues()
        );

        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 epoch2 = EPOCH_COUNTER_BASE + 2;

        // Active context hasn't changed
        assertEq(kmsVerifier.getCurrentKmsContextId(), ctx1);

        // PENDING context returns empty from getSignersForKmsContext
        address[] memory ctx2Signers = kmsVerifier.getSignersForKmsContext(ctx2);
        assertEq(ctx2Signers.length, 0);
    }

    function test_FullContextSwitchFlow() public {
        _upgradeProxyWithSigners(3);
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();

        address[] memory newSigners = new address[](1);
        newSigners[0] = signer3;
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "v1.0", _mockPcrValues());
        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 epoch2 = EPOCH_COUNTER_BASE + 2;

        // Confirm context creation
        _confirmContextCreationForAll(ctx2, newSigners);
        // Still not active
        assertEq(kmsVerifier.getCurrentKmsContextId(), ctx1);

        // Confirm epoch result
        _confirmEpochResultForAll(ctx2, epoch2, newSigners);

        // Active context switched
        (uint256 activeCtx, uint256 activeEpoch) = kmsVerifier.getCurrentKmsContext();
        assertEq(activeCtx, ctx2);
        assertEq(activeEpoch, epoch2);
    }

    function test_GetSignersForKmsContextReturnsEmptyForInvalidContexts() public {
        (uint256 ctx1, ) = _setupTwoContexts();
        vm.prank(owner);
        kmsVerifier.destroyKmsContext(ctx1);
        assertEq(kmsVerifier.getSignersForKmsContext(ctx1).length, 0);
        assertEq(kmsVerifier.getSignersForKmsContext(KMS_CONTEXT_COUNTER_BASE + 999).length, 0);
    }

    function test_DestroyKmsContextOnlyCallableByGovernance() public {
        (uint256 ctx1, ) = _setupTwoContexts();
        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        vm.prank(address(0xdead));
        kmsVerifier.destroyKmsContext(ctx1);
    }

    function test_DestroyKmsContextMarksAsDestroyed() public {
        (uint256 ctx1, ) = _setupTwoContexts();
        vm.expectEmit(true, true, true, true);
        emit KMSVerifier.KMSContextDestroyed(ctx1);
        vm.prank(owner);
        kmsVerifier.destroyKmsContext(ctx1);
        assertEq(kmsVerifier.getSignersForKmsContext(ctx1).length, 0);
    }

    function test_DestroyActiveContextReverts() public {
        _upgradeProxyWithSigners(3);
        uint256 currentCtx = kmsVerifier.getCurrentKmsContextId();
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.ActiveContextCannotBeDestroyed.selector, currentCtx));
        kmsVerifier.destroyKmsContext(currentCtx);
    }

    function test_DestroyNonExistentContextReverts() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, 0));
        kmsVerifier.destroyKmsContext(0);
    }

    // =========================================================================
    //  Verification with context tests
    // =========================================================================

    function test_VerificationSucceedsForOldContextWithOldSigners() public {
        (uint256 ctx1, ) = _setupTwoContexts();
        // ctx1 is still ACTIVE (state), just not the "current" context anymore
        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx1);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));
    }

    function test_VerificationFailsForDestroyedContext() public {
        (uint256 ctx1, ) = _setupTwoContexts();
        vm.prank(owner);
        kmsVerifier.destroyKmsContext(ctx1);
        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx1);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, ctx1));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function test_VerificationFailsWithUnsupportedExtraDataVersion() public {
        _upgradeProxyWithSigners(3);
        bytes memory extraData = abi.encodePacked(uint8(0x02));
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.UnsupportedExtraDataVersion.selector, uint8(0x02)));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function test_VerificationFailsWithMalformedV1ExtraData() public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0x01), uint8(0x42));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        bytes memory sig = _computeSignature(privateKeySigner0, digest);
        bytes memory decryptionProof = abi.encodePacked(uint8(1), sig, extraData);
        vm.expectRevert(KMSVerifier.DeserializingExtraDataFail.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    // =========================================================================
    //  Additional edge cases
    // =========================================================================

    function test_CannotDestroyAlreadyDestroyedContext() public {
        (uint256 ctx1, ) = _setupTwoContexts();
        vm.startPrank(owner);
        kmsVerifier.destroyKmsContext(ctx1);
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, ctx1));
        kmsVerifier.destroyKmsContext(ctx1);
        vm.stopPrank();
    }

    function test_VerificationFailsForInvalidContextWithV1ExtraData() public {
        _upgradeProxyWithSigners(3);
        uint256 nonExistentCtx = KMS_CONTEXT_COUNTER_BASE + 999;
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            abi.encodePacked(uint8(0x01), nonExistentCtx)
        );
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, nonExistentCtx));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function test_VerificationSucceedsWithEmptyExtraData() public {
        _upgradeProxyWithSigners(3);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            new bytes(0)
        );
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));
    }

    function test_VerificationSucceedsWithV1ExtraDataForCurrentContext() public {
        _upgradeProxyWithSigners(3);
        uint256 currentCtx = kmsVerifier.getCurrentKmsContextId();
        bytes memory extraData = abi.encodePacked(uint8(0x01), currentCtx, uint256(12345));
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));
    }

    function test_V0ExtraDataWithTrailingBytesUsesCurrentContext() public {
        _upgradeProxyWithSigners(3);
        bytes memory extraData = abi.encodePacked(uint8(0x00), uint256(12345));
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));
    }

    function test_GetContextSignersAndThresholdFromExtraData() public {
        _upgradeProxyWithSigners(3);
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();
        (address[] memory v0Signers, uint256 v0Threshold) = kmsVerifier.getContextSignersAndThresholdFromExtraData(
            hex"00"
        );
        assertEq(v0Signers.length, 3);
        assertEq(v0Signers[0], signer0);
        assertEq(v0Threshold, 1);

        // Rotate to context 2 and activate it
        address[] memory newSigners = new address[](1);
        newSigners[0] = signer3;
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "v1.0", _mockPcrValues());
        _activateContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2, newSigners);

        // v1 pointing at old context 1 still works
        bytes memory v1ExtraData = abi.encodePacked(uint8(0x01), ctx1);
        (address[] memory oldCtxSigners, ) = kmsVerifier.getContextSignersAndThresholdFromExtraData(v1ExtraData);
        assertEq(oldCtxSigners.length, 3);

        // Destroy ctx1 → v1 reverts
        vm.prank(owner);
        kmsVerifier.destroyKmsContext(ctx1);
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, ctx1));
        kmsVerifier.getContextSignersAndThresholdFromExtraData(v1ExtraData);
    }

    function test_CrossContextSignerRejection() public {
        (uint256 ctx1, ) = _setupTwoContexts();
        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx1);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner3,
            extraData
        );
        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    // =========================================================================
    //  Initialization / reinit tests
    // =========================================================================

    function test_InitializeFromEmptyProxyDoesNotEmitNewContextSet() public {
        vm.recordLogs();
        _upgradeProxyWithSigners(3);
        assertFalse(_hasNewContextSetEvent(vm.getRecordedLogs()), "NewContextSet should not be emitted during init");
        assertEq(kmsVerifier.getCurrentKmsContextId(), KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(kmsVerifier.getKmsSigners().length, 3);
        assertEq(kmsVerifier.getThreshold(), initialThreshold);
    }

    function test_InitSetsGenesisContextAndEpochActive() public {
        _upgradeProxyWithSigners(3);
        uint256 genesisCtx = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 genesisEpoch = EPOCH_COUNTER_BASE + 1;
        (uint256 activeCtx, uint256 activeEpoch) = kmsVerifier.getCurrentKmsContext();
        assertEq(activeCtx, genesisCtx);
        assertEq(activeEpoch, genesisEpoch);
    }

    function test_ReinitializeV2MigratesExistingSigners() public {
        (address proxyAddr, KMSVerifier kv) = _deployUninitializedKMSVerifierProxy();

        bytes32 signersLenSlot = bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 1);
        vm.store(proxyAddr, signersLenSlot, bytes32(uint256(2)));
        bytes32 signersDataSlot = keccak256(abi.encode(signersLenSlot));
        vm.store(proxyAddr, signersDataSlot, bytes32(uint256(uint160(signer0))));
        vm.store(proxyAddr, bytes32(uint256(signersDataSlot) + 1), bytes32(uint256(uint160(signer1))));
        vm.store(proxyAddr, bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 2), bytes32(uint256(1)));
        vm.store(proxyAddr, OZ_INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(2)));

        vm.recordLogs();
        vm.prank(owner);
        kv.reinitializeV2();
        assertFalse(_hasNewContextSetEvent(vm.getRecordedLogs()));

        // After V2 only, the context exists but is not yet ACTIVE (V3 activates it).
        // getSignersForKmsContext returns empty for non-ACTIVE contexts.
        uint256 expectedCtxId = KMS_CONTEXT_COUNTER_BASE + 1;
        assertEq(kv.getSignersForKmsContext(expectedCtxId).length, 0);
        // Verify signers are accessible after full V2→V3 migration
        vm.prank(owner);
        kv.reinitializeV3();
        address[] memory ctxSigners = kv.getSignersForKmsContext(expectedCtxId);
        assertEq(ctxSigners.length, 2);
        assertEq(ctxSigners[0], signer0);
        assertEq(ctxSigners[1], signer1);
    }

    function test_ReinitializeV2CannotBeCalledTwice() public {
        (address proxyAddr, KMSVerifier kv) = _deployUninitializedKMSVerifierProxy();
        bytes32 signersLenSlot = bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 1);
        vm.store(proxyAddr, signersLenSlot, bytes32(uint256(1)));
        bytes32 signersDataSlot = keccak256(abi.encode(signersLenSlot));
        vm.store(proxyAddr, signersDataSlot, bytes32(uint256(uint160(signer0))));
        vm.store(proxyAddr, bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 2), bytes32(uint256(1)));
        vm.store(proxyAddr, OZ_INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(2)));

        vm.prank(owner);
        kv.reinitializeV2();

        vm.prank(owner);
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        kv.reinitializeV2();
    }

    function test_ReinitializeV2RevertsWithEmptyLegacySigners() public {
        (address proxyAddr, KMSVerifier kv) = _deployUninitializedKMSVerifierProxy();
        vm.store(proxyAddr, OZ_INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(2)));
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.SignersSetIsEmpty.selector);
        kv.reinitializeV2();
    }

    function test_ReinitializeV2RevertsWithZeroThreshold() public {
        (address proxyAddr, KMSVerifier kv) = _deployUninitializedKMSVerifierProxy();
        bytes32 signersLenSlot = bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 1);
        vm.store(proxyAddr, signersLenSlot, bytes32(uint256(1)));
        bytes32 signersDataSlot = keccak256(abi.encode(signersLenSlot));
        vm.store(proxyAddr, signersDataSlot, bytes32(uint256(uint160(signer0))));
        vm.store(proxyAddr, OZ_INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(2)));
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.ThresholdIsNull.selector);
        kv.reinitializeV2();
    }

    function test_ReinitializeV3CreatesGenesisEpoch() public {
        // Simulate V2 state: call reinitializeV2 first, then reinitializeV3
        (address proxyAddr, KMSVerifier kv) = _deployUninitializedKMSVerifierProxy();
        bytes32 signersLenSlot = bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 1);
        vm.store(proxyAddr, signersLenSlot, bytes32(uint256(1)));
        bytes32 signersDataSlot = keccak256(abi.encode(signersLenSlot));
        vm.store(proxyAddr, signersDataSlot, bytes32(uint256(uint160(signer0))));
        vm.store(proxyAddr, bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 2), bytes32(uint256(1)));
        vm.store(proxyAddr, OZ_INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(2)));

        vm.prank(owner);
        kv.reinitializeV2();

        // Now reinitializeV3
        vm.prank(owner);
        kv.reinitializeV3();

        uint256 expectedCtxId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 expectedEpochId = EPOCH_COUNTER_BASE + 1;
        (uint256 activeCtx, uint256 activeEpoch) = kv.getCurrentKmsContext();
        assertEq(activeCtx, expectedCtxId);
        assertEq(activeEpoch, expectedEpochId);
    }

    function test_ReinitializeV3RevertsWithoutV2() public {
        (address proxyAddr, KMSVerifier kv) = _deployUninitializedKMSVerifierProxy();
        // Set initialized to 3 (V2 version) but don't actually run V2
        vm.store(proxyAddr, OZ_INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(3)));

        vm.prank(owner);
        vm.expectPartialRevert(KMSVerifier.InvalidKMSContext.selector);
        kv.reinitializeV3();
    }

    // =========================================================================
    //  Epoch lifecycle tests
    // =========================================================================

    function test_DefineNewContextAndEpochEmitsNewContextSet() public {
        _upgradeProxyWithSigners(3);
        uint256 expectedCtx = KMS_CONTEXT_COUNTER_BASE + 2;
        KMSVerifier.MpcNode[] memory nodes = _singleMpcNode(privateKeySigner3);
        KMSVerifier.PcrValues[] memory pcrs = _mockPcrValues();
        vm.expectEmit(true, false, false, true);
        emit KMSVerifier.NewContextSet(expectedCtx, 1, "v1.0", pcrs, nodes);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(nodes, 1, "v1.0", pcrs);
    }

    function test_DefineNewContextAndEpochRevertsIfContextInFlight() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());

        // Second call should revert — context already in-flight
        vm.prank(owner);
        vm.expectPartialRevert(KMSVerifier.ContextInFlight.selector);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner4), 1, "", _emptyPcrValues());
    }

    function test_DefineNewContextAndEpochRevertsIfEpochInFlight() public {
        _upgradeProxyWithSigners(3);
        // Start a same-set epoch transition → creates in-flight epoch
        vm.prank(owner);
        kmsVerifier.defineNewEpochForCurrentContext();

        // defineNewContextAndEpoch should revert — epoch already in-flight
        vm.prank(owner);
        vm.expectPartialRevert(KMSVerifier.EpochInFlight.selector);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());
    }

    function test_DefineNewEpochForCurrentContextCreatesEpoch() public {
        _upgradeProxyWithSigners(3);
        (uint256 activeCtx, uint256 activeEpoch) = kmsVerifier.getCurrentKmsContext();

        vm.prank(owner);
        vm.expectEmit(true, true, false, true);
        uint256 expectedNewEpoch = EPOCH_COUNTER_BASE + 2;
        emit KMSVerifier.NewEpochForCurrentContext(activeCtx, expectedNewEpoch, activeEpoch);
        kmsVerifier.defineNewEpochForCurrentContext();

        // Active context and epoch haven't changed
        (uint256 curCtx, uint256 curEpoch) = kmsVerifier.getCurrentKmsContext();
        assertEq(curCtx, activeCtx);
        assertEq(curEpoch, activeEpoch);
    }

    function test_DefineNewEpochForCurrentContextRevertsIfNoActiveContext() public {
        // Deploy uninitialized — no activeContextId
        (address proxyAddr, KMSVerifier kv) = _deployUninitializedKMSVerifierProxy();
        vm.store(proxyAddr, OZ_INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(4)));
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.NoActiveContext.selector);
        kv.defineNewEpochForCurrentContext();
    }

    function test_DefineNewEpochForCurrentContextRevertsIfEpochInFlight() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewEpochForCurrentContext();
        // Second call should revert
        vm.prank(owner);
        vm.expectPartialRevert(KMSVerifier.EpochInFlight.selector);
        kmsVerifier.defineNewEpochForCurrentContext();
    }

    function test_DefineNewEpochForCurrentContextRevertsIfContextInFlight() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());
        // defineNewEpochForCurrentContext should revert — epoch in-flight (pendingEpochId is set by defineNewContextAndEpoch)
        vm.prank(owner);
        vm.expectPartialRevert(KMSVerifier.EpochInFlight.selector);
        kmsVerifier.defineNewEpochForCurrentContext();
    }

    function test_SameSetEpochTransitionFullFlow() public {
        _upgradeProxyWithSigners(3);
        (uint256 activeCtx, uint256 activeEpoch) = kmsVerifier.getCurrentKmsContext();

        // Initiate same-set epoch transition
        vm.prank(owner);
        kmsVerifier.defineNewEpochForCurrentContext();
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;

        // Confirm epoch result for all signers
        _confirmEpochResultForAll(activeCtx, newEpochId, activeSigners);

        // Epoch and context should be active, epochId updated
        (uint256 curCtx, uint256 curEpoch) = kmsVerifier.getCurrentKmsContext();
        assertEq(curCtx, activeCtx); // same context
        assertEq(curEpoch, newEpochId); // new epoch
    }

    function test_ConfirmContextCreationRequiresPendingState() public {
        _upgradeProxyWithSigners(3);
        uint256 activeCtx = kmsVerifier.getCurrentKmsContextId();
        // Trying to confirm a context that's already ACTIVE
        bytes32 digest = _computeContextCreationDigest(activeCtx);
        bytes memory sig = _computeSignature(privateKeySigner0, digest);
        vm.expectPartialRevert(KMSVerifier.InvalidContextState.selector);
        kmsVerifier.confirmContextCreation(activeCtx, sig);
    }

    function test_ConfirmContextCreationRejectsNonSigner() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());
        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;

        // signer0 is NOT in ctx2's signer set
        bytes32 digest = _computeContextCreationDigest(ctx2);
        bytes memory sig = _computeSignature(privateKeySigner0, digest);
        vm.expectPartialRevert(KMSVerifier.NotContextSigner.selector);
        kmsVerifier.confirmContextCreation(ctx2, sig);
    }

    function test_ConfirmContextCreationRejectsDuplicate() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(
            _twoMpcNodes(privateKeySigner3, privateKeySigner4),
            1,
            "",
            _emptyPcrValues()
        );
        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;

        bytes32 digest = _computeContextCreationDigest(ctx2);
        bytes memory sig3 = _computeSignature(privateKeySigner3, digest);
        kmsVerifier.confirmContextCreation(ctx2, sig3);
        // Duplicate
        vm.expectPartialRevert(KMSVerifier.AlreadyConfirmedContextCreation.selector);
        kmsVerifier.confirmContextCreation(ctx2, sig3);
    }

    function test_ConfirmContextCreationBlockedByDestroyedContext() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());
        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;

        // Destroy ctx2 (it's the pending context, not the active one)
        vm.prank(owner);
        kmsVerifier.destroyKmsContext(ctx2);

        // contexts[ctx2].state is now DESTROYED, so the PENDING check rejects it
        bytes32 digest = _computeContextCreationDigest(ctx2);
        bytes memory sig = _computeSignature(privateKeySigner3, digest);
        vm.expectPartialRevert(KMSVerifier.InvalidContextState.selector);
        kmsVerifier.confirmContextCreation(ctx2, sig);
    }

    function test_ConfirmEpochResultRejectsWrongEpoch() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());
        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;
        address[] memory ctx2Signers = new address[](1);
        ctx2Signers[0] = signer3;
        _confirmContextCreationForAll(ctx2, ctx2Signers);

        // Use wrong epoch ID
        uint256 wrongEpochId = EPOCH_COUNTER_BASE + 999;
        KMSVerifier.KeyDigest[] memory kd = _mockKeyDigests();
        bytes memory extraData = _buildExtraData(0x01, ctx2, wrongEpochId);
        bytes32 digest = _computeKeygenDigest(1, 2, kd, extraData);
        bytes memory sig = _computeSignature(privateKeySigner3, digest);
        vm.expectPartialRevert(KMSVerifier.NotPendingEpoch.selector);
        kmsVerifier.confirmEpochResult(wrongEpochId, 1, 2, kd, extraData, sig);
    }

    function test_InconsistentEpochResultRevert() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(
            _twoMpcNodes(privateKeySigner3, privateKeySigner4),
            1,
            "",
            _emptyPcrValues()
        );
        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 epoch2 = EPOCH_COUNTER_BASE + 2;
        address[] memory ctx2Signers = new address[](2);
        ctx2Signers[0] = signer3;
        ctx2Signers[1] = signer4;
        _confirmContextCreationForAll(ctx2, ctx2Signers);

        // First signer confirms with keyId=2
        KMSVerifier.KeyDigest[] memory kd = _mockKeyDigests();
        bytes memory extraData = _buildExtraData(0x01, ctx2, epoch2);
        bytes32 digest1 = _computeKeygenDigest(1, 2, kd, extraData);
        bytes memory sig1 = _computeSignature(privateKeySigner3, digest1);
        kmsVerifier.confirmEpochResult(epoch2, 1, 2, kd, extraData, sig1);

        // Second signer confirms with keyId=999 → different keygen digest → InconsistentEpochResult
        bytes32 digest2 = _computeKeygenDigest(1, 999, kd, extraData);
        bytes memory sig2 = _computeSignature(privateKeySigner4, digest2);
        vm.expectPartialRevert(KMSVerifier.InconsistentEpochResult.selector);
        kmsVerifier.confirmEpochResult(epoch2, 1, 999, kd, extraData, sig2);
    }

    /// @dev Helper: sets up a CREATED context (ctx2 with signer3) ready for epoch confirmation.
    function _setupCreatedContext() internal returns (uint256 ctx2, uint256 epoch2) {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());
        ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;
        epoch2 = EPOCH_COUNTER_BASE + 2;
        address[] memory ctx2Signers = new address[](1);
        ctx2Signers[0] = signer3;
        _confirmContextCreationForAll(ctx2, ctx2Signers);
    }

    function test_ExtraDataTooShortReverts() public {
        (uint256 ctx2, uint256 epoch2) = _setupCreatedContext();
        KMSVerifier.KeyDigest[] memory kd = _mockKeyDigests();
        bytes memory shortData = hex"01";
        bytes32 d = _computeKeygenDigest(1, 2, kd, shortData);
        bytes memory sig = _computeSignature(privateKeySigner3, d);
        vm.expectPartialRevert(KMSVerifier.InvalidExtraDataLength.selector);
        kmsVerifier.confirmEpochResult(epoch2, 1, 2, kd, shortData, sig);
    }

    function test_ExtraDataVersionZeroReverts() public {
        (uint256 ctx2, uint256 epoch2) = _setupCreatedContext();
        KMSVerifier.KeyDigest[] memory kd = _mockKeyDigests();
        bytes memory v0Data = _buildExtraData(0x00, ctx2, epoch2);
        bytes32 d = _computeKeygenDigest(1, 2, kd, v0Data);
        bytes memory sig = _computeSignature(privateKeySigner3, d);
        vm.expectPartialRevert(KMSVerifier.UnsupportedExtraDataVersion.selector);
        kmsVerifier.confirmEpochResult(epoch2, 1, 2, kd, v0Data, sig);
    }

    function test_ExtraDataWrongContextReverts() public {
        (uint256 ctx2, uint256 epoch2) = _setupCreatedContext();
        KMSVerifier.KeyDigest[] memory kd = _mockKeyDigests();
        bytes memory wrongCtx = _buildExtraData(0x01, ctx2 + 1, epoch2);
        bytes32 d = _computeKeygenDigest(1, 2, kd, wrongCtx);
        bytes memory sig = _computeSignature(privateKeySigner3, d);
        vm.expectPartialRevert(KMSVerifier.InvalidExtraDataContext.selector);
        kmsVerifier.confirmEpochResult(epoch2, 1, 2, kd, wrongCtx, sig);
    }

    function test_ExtraDataWrongEpochReverts() public {
        (uint256 ctx2, uint256 epoch2) = _setupCreatedContext();
        KMSVerifier.KeyDigest[] memory kd = _mockKeyDigests();
        bytes memory wrongEp = _buildExtraData(0x01, ctx2, epoch2 + 1);
        bytes32 d = _computeKeygenDigest(1, 2, kd, wrongEp);
        bytes memory sig = _computeSignature(privateKeySigner3, d);
        vm.expectPartialRevert(KMSVerifier.InvalidExtraDataEpoch.selector);
        kmsVerifier.confirmEpochResult(epoch2, 1, 2, kd, wrongEp, sig);
    }

    function test_DecryptionVerificationRequiresActiveContext() public {
        _upgradeProxyWithSigners(3);
        // Define new context but don't activate it
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());
        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;

        // Try to verify using v1 extraData pointing at the PENDING ctx2
        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx2);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner3,
            extraData
        );
        // Should fail because ctx2 is PENDING, not ACTIVE
        vm.expectPartialRevert(KMSVerifier.InvalidContextState.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function test_DestroyPendingContextClearsPendingState() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());
        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;

        // Destroy the pending context
        vm.prank(owner);
        kmsVerifier.destroyKmsContext(ctx2);

        // Should be able to define a new context now (pending state cleared)
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner4), 1, "", _emptyPcrValues()); // should not revert
    }

    function test_ConfirmEpochResultRejectsDuplicate() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(
            _twoMpcNodes(privateKeySigner3, privateKeySigner4),
            1,
            "",
            _emptyPcrValues()
        );
        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 epoch2 = EPOCH_COUNTER_BASE + 2;
        address[] memory ctx2Signers = new address[](2);
        ctx2Signers[0] = signer3;
        ctx2Signers[1] = signer4;
        _confirmContextCreationForAll(ctx2, ctx2Signers);

        KMSVerifier.KeyDigest[] memory kd = _mockKeyDigests();
        bytes memory extraData = _buildExtraData(0x01, ctx2, epoch2);
        bytes32 digest = _computeKeygenDigest(1, 2, kd, extraData);
        bytes memory sig3 = _computeSignature(privateKeySigner3, digest);
        kmsVerifier.confirmEpochResult(epoch2, 1, 2, kd, extraData, sig3);
        // Duplicate
        vm.expectPartialRevert(KMSVerifier.AlreadyConfirmedEpochResult.selector);
        kmsVerifier.confirmEpochResult(epoch2, 1, 2, kd, extraData, sig3);
    }

    function test_ConfirmEpochResultRequiresCreatedOrActiveContext() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());
        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 epoch2 = EPOCH_COUNTER_BASE + 2;

        // Context is PENDING (not yet CREATED) — confirmEpochResult should fail
        KMSVerifier.KeyDigest[] memory kd = _mockKeyDigests();
        bytes memory extraData = _buildExtraData(0x01, ctx2, epoch2);
        bytes32 digest = _computeKeygenDigest(1, 2, kd, extraData);
        bytes memory sig = _computeSignature(privateKeySigner3, digest);
        vm.expectPartialRevert(KMSVerifier.InvalidContextState.selector);
        kmsVerifier.confirmEpochResult(epoch2, 1, 2, kd, extraData, sig);
    }

    // =========================================================================
    //  Context metadata tests
    // =========================================================================

    function test_ContextCreatedEmitsPreviousIds() public {
        _upgradeProxyWithSigners(3);
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();
        (, uint256 epoch1) = kmsVerifier.getCurrentKmsContext();

        vm.prank(owner);
        kmsVerifier.defineNewContextAndEpoch(_singleMpcNode(privateKeySigner3), 1, "", _emptyPcrValues());
        uint256 ctx2 = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 epoch2 = EPOCH_COUNTER_BASE + 2;

        address[] memory ctx2Signers = new address[](1);
        ctx2Signers[0] = signer3;
        // Expect ContextCreated with previous context/epoch IDs
        vm.expectEmit(true, true, false, true);
        emit KMSVerifier.ContextCreated(ctx2, epoch2, ctx1, epoch1);
        _confirmContextCreationForAll(ctx2, ctx2Signers);
    }

    function test_NewEpochForCurrentContextEmitsPreviousIds() public {
        _upgradeProxyWithSigners(3);
        (uint256 activeCtx, uint256 activeEpoch) = kmsVerifier.getCurrentKmsContext();
        uint256 expectedNewEpoch = EPOCH_COUNTER_BASE + 2;

        vm.expectEmit(true, true, false, true);
        emit KMSVerifier.NewEpochForCurrentContext(activeCtx, expectedNewEpoch, activeEpoch);
        vm.prank(owner);
        kmsVerifier.defineNewEpochForCurrentContext();
    }
}
