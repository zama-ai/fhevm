// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";

import {ProtocolConfig} from "../../contracts/ProtocolConfig.sol";
import {IProtocolConfig} from "../../contracts/interfaces/IProtocolConfig.sol";
import {KmsNode} from "../../contracts/shared/Structs.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";
import {aclAdd, kmsGenerationAdd} from "../../addresses/FHEVMHostAddresses.sol";

contract ProtocolConfigTest is Test {
    ProtocolConfig internal protocolConfig;

    uint256 internal constant KMS_CONTEXT_COUNTER_BASE = uint256(0x07) << 248;

    address internal constant owner = address(456);
    address internal constant txSender0 = address(0xA1);
    address internal constant txSender1 = address(0xA2);
    address internal constant signer0 = address(0xB1);
    address internal constant signer1 = address(0xB2);

    address internal proxy;
    address internal implementation;

    function _deployAndEtchACL() internal {
        address _acl = address(new ACL());
        bytes memory code = _acl.code;
        vm.etch(aclAdd, code);
        bytes32 ownableSlot = 0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300;
        vm.store(aclAdd, ownableSlot, bytes32(uint256(uint160(owner))));
    }

    function _makeNodes(uint256 count) internal pure returns (KmsNode[] memory) {
        KmsNode[] memory nodes = new KmsNode[](count);
        for (uint256 i = 0; i < count; i++) {
            nodes[i] = KmsNode({
                txSenderAddress: address(uint160(0xA1 + i)),
                signerAddress: address(uint160(0xB1 + i)),
                ipAddress: "127.0.0.1",
                storageUrl: "https://storage.example.com"
            });
        }
        return nodes;
    }

    function _defaultThresholds() internal pure returns (IProtocolConfig.KmsThresholds memory) {
        return IProtocolConfig.KmsThresholds({decryptionThreshold: 1, kmsGenThreshold: 1});
    }

    function _deployProxy() internal {
        proxy =
            UnsafeUpgrades.deployUUPSProxy(address(new EmptyUUPSProxy()), abi.encodeCall(EmptyUUPSProxy.initialize, ()));
    }

    function _upgradeProxy(KmsNode[] memory nodes, IProtocolConfig.KmsThresholds memory thresholds) internal {
        implementation = address(new ProtocolConfig());
        UnsafeUpgrades.upgradeProxy(
            proxy, implementation, abi.encodeCall(ProtocolConfig.initializeFromEmptyProxy, (nodes, thresholds)), owner
        );
        protocolConfig = ProtocolConfig(proxy);
    }

    function _setupDefault() internal {
        _deployAndEtchACL();
        _deployProxy();
        _upgradeProxy(_makeNodes(2), _defaultThresholds());
        _etchKmsGenerationNotPending();
    }

    function _etchKmsGenerationPending() internal {
        vm.etch(kmsGenerationAdd, address(new MockKMSGenerationPending()).code);
    }

    function _etchKmsGenerationNotPending() internal {
        vm.etch(kmsGenerationAdd, address(new MockKMSGenerationNotPending()).code);
    }

    function _upgradeProxyExpectRevert(
        KmsNode[] memory nodes,
        IProtocolConfig.KmsThresholds memory thresholds,
        bytes memory expectedRevert
    ) internal {
        implementation = address(new ProtocolConfig());
        vm.prank(owner);
        vm.expectRevert(expectedRevert);
        EmptyUUPSProxy(proxy).upgradeToAndCall(
            implementation, abi.encodeCall(ProtocolConfig.initializeFromEmptyProxy, (nodes, thresholds))
        );
    }

    // -----------------------------------------------------------------------
    // Init tests
    // -----------------------------------------------------------------------

    function test_initSuccess() public {
        _setupDefault();
        assertEq(protocolConfig.getVersion(), "ProtocolConfig v0.1.0");
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        assertEq(contextId, KMS_CONTEXT_COUNTER_BASE + 1);
        assertTrue(protocolConfig.isValidKmsContext(contextId));
    }

    function test_initSignersRegistered() public {
        _setupDefault();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        assertTrue(protocolConfig.isKmsSignerForContext(contextId, signer0));
        assertTrue(protocolConfig.isKmsSignerForContext(contextId, signer1));
        assertTrue(protocolConfig.isKmsTxSenderForContext(contextId, txSender0));
        assertTrue(protocolConfig.isKmsTxSenderForContext(contextId, txSender1));
    }

    function test_initThresholds() public {
        _setupDefault();
        assertEq(protocolConfig.getDecryptionThreshold(), 1);
        assertEq(protocolConfig.getKmsGenThreshold(), 1);
    }

    function test_initNodesMetadata() public {
        _setupDefault();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        KmsNode memory node = protocolConfig.getKmsNodeForContext(contextId, txSender0);
        assertEq(node.txSenderAddress, txSender0);
        assertEq(node.signerAddress, signer0);
    }

    function test_initGetKmsNodesForContext() public {
        _setupDefault();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        KmsNode[] memory nodes = protocolConfig.getKmsNodesForContext(contextId);
        assertEq(nodes.length, 2);
    }

    function test_initGetSignersForContext() public {
        _setupDefault();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        address[] memory signers = protocolConfig.getKmsSignersForContext(contextId);
        assertEq(signers.length, 2);
        assertEq(signers[0], signer0);
        assertEq(signers[1], signer1);
    }

    // -----------------------------------------------------------------------
    // Validation error tests
    // -----------------------------------------------------------------------

    function test_revertEmptyNodes() public {
        _deployAndEtchACL();
        _deployProxy();
        KmsNode[] memory emptyNodes = new KmsNode[](0);
        _upgradeProxyExpectRevert(
            emptyNodes, _defaultThresholds(), abi.encodeWithSelector(IProtocolConfig.EmptyKmsNodes.selector)
        );
    }

    function test_revertNullTxSender() public {
        _deployAndEtchACL();
        _deployProxy();
        KmsNode[] memory nodes = new KmsNode[](1);
        nodes[0] = KmsNode({txSenderAddress: address(0), signerAddress: signer0, ipAddress: "", storageUrl: ""});
        _upgradeProxyExpectRevert(
            nodes, _defaultThresholds(), abi.encodeWithSelector(IProtocolConfig.KmsNodeNullTxSender.selector)
        );
    }

    function test_revertNullSigner() public {
        _deployAndEtchACL();
        _deployProxy();
        KmsNode[] memory nodes = new KmsNode[](1);
        nodes[0] = KmsNode({txSenderAddress: txSender0, signerAddress: address(0), ipAddress: "", storageUrl: ""});
        _upgradeProxyExpectRevert(
            nodes, _defaultThresholds(), abi.encodeWithSelector(IProtocolConfig.KmsNodeNullSigner.selector)
        );
    }

    function test_revertDuplicateTxSender() public {
        _deployAndEtchACL();
        _deployProxy();
        KmsNode[] memory nodes = new KmsNode[](2);
        nodes[0] = KmsNode({txSenderAddress: txSender0, signerAddress: signer0, ipAddress: "", storageUrl: ""});
        nodes[1] = KmsNode({txSenderAddress: txSender0, signerAddress: signer1, ipAddress: "", storageUrl: ""});
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.KmsTxSenderAlreadyRegistered.selector, txSender0)
        );
    }

    function test_revertDuplicateSigner() public {
        _deployAndEtchACL();
        _deployProxy();
        KmsNode[] memory nodes = new KmsNode[](2);
        nodes[0] = KmsNode({txSenderAddress: txSender0, signerAddress: signer0, ipAddress: "", storageUrl: ""});
        nodes[1] = KmsNode({txSenderAddress: txSender1, signerAddress: signer0, ipAddress: "", storageUrl: ""});
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.KmsSignerAlreadyRegistered.selector, signer0)
        );
    }

    function test_revertNullDecryptionThreshold() public {
        _deployAndEtchACL();
        _deployProxy();
        IProtocolConfig.KmsThresholds memory t =
            IProtocolConfig.KmsThresholds({decryptionThreshold: 0, kmsGenThreshold: 1});
        _upgradeProxyExpectRevert(
            _makeNodes(1), t, abi.encodeWithSelector(IProtocolConfig.InvalidNullDecryptionThreshold.selector)
        );
    }

    function test_revertHighDecryptionThreshold() public {
        _deployAndEtchACL();
        _deployProxy();
        IProtocolConfig.KmsThresholds memory t =
            IProtocolConfig.KmsThresholds({decryptionThreshold: 5, kmsGenThreshold: 1});
        _upgradeProxyExpectRevert(
            _makeNodes(1), t, abi.encodeWithSelector(IProtocolConfig.InvalidHighDecryptionThreshold.selector, 5, 1)
        );
    }

    function test_revertNullKmsGenThreshold() public {
        _deployAndEtchACL();
        _deployProxy();
        IProtocolConfig.KmsThresholds memory t =
            IProtocolConfig.KmsThresholds({decryptionThreshold: 1, kmsGenThreshold: 0});
        _upgradeProxyExpectRevert(
            _makeNodes(1), t, abi.encodeWithSelector(IProtocolConfig.InvalidNullKmsGenThreshold.selector)
        );
    }

    function test_revertHighKmsGenThreshold() public {
        _deployAndEtchACL();
        _deployProxy();
        IProtocolConfig.KmsThresholds memory t =
            IProtocolConfig.KmsThresholds({decryptionThreshold: 1, kmsGenThreshold: 5});
        _upgradeProxyExpectRevert(
            _makeNodes(1), t, abi.encodeWithSelector(IProtocolConfig.InvalidHighKmsGenThreshold.selector, 5, 1)
        );
    }

    // -----------------------------------------------------------------------
    // Context lifecycle tests
    // -----------------------------------------------------------------------

    function test_defineNewContext() public {
        _setupDefault();

        KmsNode[] memory newNodes = new KmsNode[](1);
        newNodes[0] = KmsNode({
            txSenderAddress: address(0xC1),
            signerAddress: address(0xD1),
            ipAddress: "10.0.0.1",
            storageUrl: "https://new-storage.example.com"
        });

        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();
        vm.expectEmit(true, true, true, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsContext(KMS_CONTEXT_COUNTER_BASE + 2, newNodes, thresholds);
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(newNodes, thresholds);

        uint256 newContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(newContextId, KMS_CONTEXT_COUNTER_BASE + 2);
        assertTrue(protocolConfig.isKmsSignerForContext(newContextId, address(0xD1)));
    }

    function test_historicalContextReadable() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();

        KmsNode[] memory newNodes = new KmsNode[](1);
        newNodes[0] =
            KmsNode({txSenderAddress: address(0xC1), signerAddress: address(0xD1), ipAddress: "", storageUrl: ""});

        vm.prank(owner);
        protocolConfig.defineNewKmsContext(newNodes, _defaultThresholds());

        assertTrue(protocolConfig.isValidKmsContext(firstContextId));
        address[] memory oldSigners = protocolConfig.getKmsSignersForContext(firstContextId);
        assertEq(oldSigners.length, 2);
    }

    function test_destroyContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();

        KmsNode[] memory newNodes = new KmsNode[](1);
        newNodes[0] =
            KmsNode({txSenderAddress: address(0xC1), signerAddress: address(0xD1), ipAddress: "", storageUrl: ""});
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(newNodes, _defaultThresholds());

        vm.expectEmit(true, true, true, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestroyed(firstContextId);
        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);
        assertFalse(protocolConfig.isValidKmsContext(firstContextId));
    }

    function test_revertDestroyCurrentContext() public {
        _setupDefault();
        uint256 currentId = protocolConfig.getCurrentKmsContextId();
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.CurrentKmsContextCannotBeDestroyed.selector, currentId));
        protocolConfig.destroyKmsContext(currentId);
    }

    function test_revertDestroyInvalidContext() public {
        _setupDefault();
        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidId));
        protocolConfig.destroyKmsContext(invalidId);
    }

    function test_revertDestroyAlreadyDestroyedContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();

        KmsNode[] memory newNodes = new KmsNode[](1);
        newNodes[0] =
            KmsNode({txSenderAddress: address(0xC1), signerAddress: address(0xD1), ipAddress: "", storageUrl: ""});
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(newNodes, _defaultThresholds());

        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, firstContextId));
        protocolConfig.destroyKmsContext(firstContextId);
    }

    // -----------------------------------------------------------------------
    // In-flight guard test
    // -----------------------------------------------------------------------

    function test_revertDefineContextWhenRequestInFlight() public {
        _setupDefault();
        _etchKmsGenerationPending();

        KmsNode[] memory newNodes = _makeNodes(1);
        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.KeyManagementRequestInFlight.selector);
        protocolConfig.defineNewKmsContext(newNodes, _defaultThresholds());
    }

    function test_defineContextWhenNoRequestInFlight() public {
        _setupDefault();

        KmsNode[] memory newNodes = new KmsNode[](1);
        newNodes[0] =
            KmsNode({txSenderAddress: address(0xC1), signerAddress: address(0xD1), ipAddress: "", storageUrl: ""});

        vm.prank(owner);
        protocolConfig.defineNewKmsContext(newNodes, _defaultThresholds());
        assertEq(protocolConfig.getCurrentKmsContextId(), KMS_CONTEXT_COUNTER_BASE + 2);
    }

    // -----------------------------------------------------------------------
    // Migration initializer
    // -----------------------------------------------------------------------

    function test_migrationInitializer() public {
        _deployAndEtchACL();

        proxy =
            UnsafeUpgrades.deployUUPSProxy(address(new EmptyUUPSProxy()), abi.encodeCall(EmptyUUPSProxy.initialize, ()));

        implementation = address(new ProtocolConfig());
        KmsNode[] memory nodes = _makeNodes(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();

        // Simulate migrating from an old KMSVerifier where the context counter reached BASE + 3
        uint256 migratedContextId = KMS_CONTEXT_COUNTER_BASE + 3;

        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(ProtocolConfig.initializeFromMigration, (migratedContextId, nodes, thresholds)),
            owner
        );

        protocolConfig = ProtocolConfig(proxy);
        assertEq(protocolConfig.getVersion(), "ProtocolConfig v0.1.0");
        assertEq(protocolConfig.getCurrentKmsContextId(), migratedContextId);
        assertTrue(protocolConfig.isValidKmsContext(migratedContextId));
        assertEq(protocolConfig.getKmsSignersForContext(migratedContextId).length, 2);
        assertEq(protocolConfig.getDecryptionThreshold(), 1);
        assertEq(protocolConfig.getKmsGenThreshold(), 1);
    }

    function test_revertMigrationInitializerInvalidContextId() public {
        _deployAndEtchACL();

        proxy =
            UnsafeUpgrades.deployUUPSProxy(address(new EmptyUUPSProxy()), abi.encodeCall(EmptyUUPSProxy.initialize, ()));

        implementation = address(new ProtocolConfig());
        KmsNode[] memory nodes = _makeNodes(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();
        uint256 invalidContextId = KMS_CONTEXT_COUNTER_BASE;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidContextId));
        EmptyUUPSProxy(proxy).upgradeToAndCall(
            implementation, abi.encodeCall(ProtocolConfig.initializeFromMigration, (invalidContextId, nodes, thresholds))
        );
    }

    function test_migrationGapContextsRemainInvalid() public {
        _deployAndEtchACL();

        proxy =
            UnsafeUpgrades.deployUUPSProxy(address(new EmptyUUPSProxy()), abi.encodeCall(EmptyUUPSProxy.initialize, ()));

        implementation = address(new ProtocolConfig());
        KmsNode[] memory nodes = _makeNodes(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();
        uint256 migratedContextId = KMS_CONTEXT_COUNTER_BASE + 3;

        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(ProtocolConfig.initializeFromMigration, (migratedContextId, nodes, thresholds)),
            owner
        );

        protocolConfig = ProtocolConfig(proxy);

        uint256 gapContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        assertFalse(protocolConfig.isValidKmsContext(gapContextId));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, gapContextId));
        protocolConfig.getKmsNodesForContext(gapContextId);
    }

    function test_revertViewFunctionsForInvalidContext() public {
        _setupDefault();

        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidId));
        protocolConfig.isKmsTxSenderForContext(invalidId, txSender0);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidId));
        protocolConfig.getKmsNodeForContext(invalidId, txSender0);
    }

    // -----------------------------------------------------------------------
    // Access control
    // -----------------------------------------------------------------------

    function test_revertDefineContextNotOwner() public {
        _setupDefault();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.defineNewKmsContext(_makeNodes(1), _defaultThresholds());
    }

    function test_revertDestroyContextNotOwner() public {
        _setupDefault();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.destroyKmsContext(KMS_CONTEXT_COUNTER_BASE + 1);
    }
}

contract MockKMSGenerationPending {
    function hasPendingKeyManagementRequest() external pure returns (bool) {
        return true;
    }
}

contract MockKMSGenerationNotPending {
    function hasPendingKeyManagementRequest() external pure returns (bool) {
        return false;
    }
}
