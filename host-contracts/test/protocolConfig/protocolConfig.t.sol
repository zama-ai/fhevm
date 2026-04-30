// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {ProtocolConfigUpgradedExample} from "@fhevm-host-contracts/examples/ProtocolConfigUpgradedExample.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsNode} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {KMS_CONTEXT_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

contract ProtocolConfigTest is HostContractsDeployerTestUtils {
    ProtocolConfig internal protocolConfig;

    address internal constant owner = address(456);
    address internal constant txSender0 = address(0xA1);
    address internal constant txSender1 = address(0xA2);
    address internal constant signer0 = address(0xB1);
    address internal constant signer1 = address(0xB2);

    function _makeNodes(uint256 count) internal pure returns (KmsNode[] memory) {
        return _makeNodes(count, 0);
    }

    function _makeNodes(uint256 count, uint256 startIdx) internal pure returns (KmsNode[] memory) {
        KmsNode[] memory nodes = new KmsNode[](count);
        for (uint256 i = 0; i < count; i++) {
            nodes[i] = KmsNode({
                txSenderAddress: address(uint160(0xA1 + startIdx + i)),
                signerAddress: address(uint160(0xB1 + startIdx + i)),
                ipAddress: "127.0.0.1",
                storageUrl: "https://storage.example.com"
            });
        }
        return nodes;
    }

    function _deployEmptyProtocolConfigProxy() internal {
        address emptyProxyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            protocolConfigAdd
        );
    }

    function _setupEmptyProxy() internal {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
    }

    function _setupDefault() internal {
        _deployACL(owner);
        /// @dev Distinct per-field values so each getter proves it reads the correct storage slot.
        IProtocolConfig.KmsThresholds memory thresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 2,
            kmsGen: 3,
            mpc: 4
        });
        (ProtocolConfig pc, ) = _deployProtocolConfig(owner, _makeNodes(4), thresholds);
        protocolConfig = pc;
    }

    function _setupMigration(uint256 migratedContextId) internal {
        _setupEmptyProxy();
        address impl = address(new ProtocolConfig());
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfig.initializeFromMigration,
                (migratedContextId, _makeNodes(2), _defaultThresholds())
            )
        );
        protocolConfig = ProtocolConfig(protocolConfigAdd);
    }

    function _upgradeProxyExpectRevert(
        KmsNode[] memory nodes,
        IProtocolConfig.KmsThresholds memory thresholds,
        bytes memory expectedRevert
    ) internal {
        address impl = address(new ProtocolConfig());
        vm.prank(owner);
        vm.expectRevert(expectedRevert);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(ProtocolConfig.initializeFromEmptyProxy, (nodes, thresholds))
        );
    }

    function _revertThreshold(IProtocolConfig.KmsThresholds memory t, bytes memory expectedRevert) internal {
        _setupEmptyProxy();
        _upgradeProxyExpectRevert(_makeNodes(1), t, expectedRevert);
    }

    /// @dev Asserts all seven context-guarded view functions revert for the given context ID.
    function _expectAllViewsRevertForContext(uint256 contextId) internal {
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsSignersForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.isKmsSignerForContext(contextId, signer0);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsNodesForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.isKmsTxSenderForContext(contextId, txSender0);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsNodeForContext(contextId, txSender0);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getUserDecryptionThresholdForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getPublicDecryptionThresholdForContext(contextId);
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
        // Negative: unregistered addresses must return false.
        assertFalse(protocolConfig.isKmsSignerForContext(contextId, address(0xDEAD)));
        assertFalse(protocolConfig.isKmsTxSenderForContext(contextId, address(0xDEAD)));
    }

    function test_initThresholds() public {
        _setupDefault();
        assertEq(protocolConfig.getPublicDecryptionThreshold(), 1);
        assertEq(protocolConfig.getUserDecryptionThreshold(), 2);
        assertEq(protocolConfig.getKmsGenThreshold(), 3);
        assertEq(protocolConfig.getMpcThreshold(), 4);
    }

    function test_initNodesMetadata() public {
        _setupDefault();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        KmsNode memory node = protocolConfig.getKmsNodeForContext(contextId, txSender0);
        assertEq(node.txSenderAddress, txSender0);
        assertEq(node.signerAddress, signer0);
        assertEq(node.ipAddress, "127.0.0.1");
        assertEq(node.storageUrl, "https://storage.example.com");
    }

    function test_initGetKmsNodesForContext() public {
        _setupDefault();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        KmsNode[] memory nodes = protocolConfig.getKmsNodesForContext(contextId);
        assertEq(nodes.length, 4);
        assertEq(nodes[0].txSenderAddress, txSender0);
        assertEq(nodes[0].signerAddress, signer0);
        assertEq(nodes[1].txSenderAddress, txSender1);
        assertEq(nodes[1].signerAddress, signer1);
        assertEq(nodes[2].txSenderAddress, address(0xA3));
        assertEq(nodes[2].signerAddress, address(0xB3));
        assertEq(nodes[3].txSenderAddress, address(0xA4));
        assertEq(nodes[3].signerAddress, address(0xB4));
    }

    function test_initGetSignersForContext() public {
        _setupDefault();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        address[] memory signers = protocolConfig.getKmsSignersForContext(contextId);
        assertEq(signers.length, 4);
        assertEq(signers[0], signer0);
        assertEq(signers[1], signer1);
        assertEq(signers[2], address(0xB3));
        assertEq(signers[3], address(0xB4));
    }

    // -----------------------------------------------------------------------
    // Validation error tests
    // -----------------------------------------------------------------------

    function test_revertEmptyNodes() public {
        _setupEmptyProxy();
        KmsNode[] memory emptyNodes = new KmsNode[](0);
        _upgradeProxyExpectRevert(
            emptyNodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.EmptyKmsNodes.selector)
        );
    }

    function test_revertNullTxSender() public {
        _setupEmptyProxy();
        KmsNode[] memory nodes = new KmsNode[](1);
        nodes[0] = KmsNode({txSenderAddress: address(0), signerAddress: signer0, ipAddress: "", storageUrl: ""});
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.KmsNodeNullTxSender.selector)
        );
    }

    function test_revertNullSigner() public {
        _setupEmptyProxy();
        KmsNode[] memory nodes = new KmsNode[](1);
        nodes[0] = KmsNode({txSenderAddress: txSender0, signerAddress: address(0), ipAddress: "", storageUrl: ""});
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.KmsNodeNullSigner.selector)
        );
    }

    function test_revertDuplicateTxSender() public {
        _setupEmptyProxy();
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
        _setupEmptyProxy();
        KmsNode[] memory nodes = new KmsNode[](2);
        nodes[0] = KmsNode({txSenderAddress: txSender0, signerAddress: signer0, ipAddress: "", storageUrl: ""});
        nodes[1] = KmsNode({txSenderAddress: txSender1, signerAddress: signer0, ipAddress: "", storageUrl: ""});
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.KmsSignerAlreadyRegistered.selector, signer0)
        );
    }

    function test_revertNullPublicDecryptionThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.publicDecryption = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "publicDecryption"));
    }

    function test_revertHighPublicDecryptionThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.publicDecryption = 5;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "publicDecryption", 5, 1)
        );
    }

    function test_revertNullUserDecryptionThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.userDecryption = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "userDecryption"));
    }

    function test_revertHighUserDecryptionThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.userDecryption = 5;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "userDecryption", 5, 1)
        );
    }

    function test_revertNullKmsGenThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.kmsGen = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "kmsGen"));
    }

    function test_revertHighKmsGenThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.kmsGen = 5;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "kmsGen", 5, 1));
    }

    function test_revertNullMpcThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.mpc = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "mpc"));
    }

    function test_revertHighMpcThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.mpc = 5;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "mpc", 5, 1));
    }

    // -----------------------------------------------------------------------
    // Context lifecycle tests
    // -----------------------------------------------------------------------

    function test_defineNewContext() public {
        _setupDefault();

        KmsNode[] memory newNodes = _makeNodes(1, 4);

        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();
        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsContext(KMS_CONTEXT_COUNTER_BASE + 2, newNodes, thresholds);
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(newNodes, thresholds);

        uint256 newContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(newContextId, KMS_CONTEXT_COUNTER_BASE + 2);
        assertTrue(protocolConfig.isValidKmsContext(newContextId));
        assertTrue(protocolConfig.isKmsSignerForContext(newContextId, newNodes[0].signerAddress));
        assertEq(protocolConfig.getKmsNodesForContext(newContextId).length, 1);
    }

    function test_historicalContextReadable() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();

        KmsNode[] memory newNodes = _makeNodes(1, 4);

        vm.prank(owner);
        protocolConfig.defineNewKmsContext(newNodes, _defaultThresholds());

        uint256 currentId = protocolConfig.getCurrentKmsContextId();
        assertTrue(currentId != firstContextId);
        assertTrue(protocolConfig.isValidKmsContext(firstContextId));
        address[] memory oldSigners = protocolConfig.getKmsSignersForContext(firstContextId);
        assertEq(oldSigners.length, 4);
    }

    function test_destroyContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();

        KmsNode[] memory newNodes = _makeNodes(1, 4);
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(newNodes, _defaultThresholds());

        vm.expectEmit(true, false, false, true, address(protocolConfig));
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

        KmsNode[] memory newNodes = _makeNodes(1, 4);
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(newNodes, _defaultThresholds());

        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, firstContextId));
        protocolConfig.destroyKmsContext(firstContextId);
    }

    // -----------------------------------------------------------------------
    // Migration initializer
    // -----------------------------------------------------------------------

    function test_migrationInitializer() public {
        uint256 migratedContextId = KMS_CONTEXT_COUNTER_BASE + 3;
        _setupMigration(migratedContextId);

        assertEq(protocolConfig.getVersion(), "ProtocolConfig v0.1.0");
        assertEq(protocolConfig.getCurrentKmsContextId(), migratedContextId);
        assertTrue(protocolConfig.isValidKmsContext(migratedContextId));
        assertEq(protocolConfig.getKmsSignersForContext(migratedContextId).length, 2);
        assertEq(protocolConfig.getPublicDecryptionThreshold(), 1);
        assertEq(protocolConfig.getUserDecryptionThreshold(), 1);
        assertEq(protocolConfig.getKmsGenThreshold(), 1);
        assertEq(protocolConfig.getMpcThreshold(), 1);
    }

    function test_revertMigrationInitializerInvalidContextId() public {
        _setupEmptyProxy();

        address impl = address(new ProtocolConfig());
        KmsNode[] memory nodes = _makeNodes(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();
        uint256 invalidContextId = KMS_CONTEXT_COUNTER_BASE;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidContextId));
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(ProtocolConfig.initializeFromMigration, (invalidContextId, nodes, thresholds))
        );
    }

    function test_migrationGapContextsRemainInvalid() public {
        _setupMigration(KMS_CONTEXT_COUNTER_BASE + 3);

        uint256 gapContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        assertFalse(protocolConfig.isValidKmsContext(gapContextId));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, gapContextId));
        protocolConfig.getKmsNodesForContext(gapContextId);
    }

    // -----------------------------------------------------------------------
    // View-function guards (invalid & destroyed contexts)
    // -----------------------------------------------------------------------

    function test_revertViewFunctionsForInvalidContext() public {
        _setupDefault();
        _expectAllViewsRevertForContext(KMS_CONTEXT_COUNTER_BASE + 999);
    }

    function test_revertViewFunctionsForDestroyedContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();

        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeNodes(1, 4), _defaultThresholds());

        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);

        _expectAllViewsRevertForContext(firstContextId);
    }

    // -----------------------------------------------------------------------
    // Threshold getters after context rotation
    // -----------------------------------------------------------------------

    function test_getUserDecryptionThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        // _setupDefault uses userDecryption = 2
        assertEq(protocolConfig.getUserDecryptionThresholdForContext(firstContextId), 2);

        // Rotate to a new context with userDecryption = 1
        IProtocolConfig.KmsThresholds memory newThresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 1,
            kmsGen: 1,
            mpc: 1
        });
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeNodes(2, 4), newThresholds);
        uint256 secondContextId = protocolConfig.getCurrentKmsContextId();

        // New context returns its own threshold
        assertEq(protocolConfig.getUserDecryptionThresholdForContext(secondContextId), 1);
        // Old context still returns the original threshold
        assertEq(protocolConfig.getUserDecryptionThresholdForContext(firstContextId), 2);
    }

    function test_getPublicDecryptionThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(firstContextId), 1);

        IProtocolConfig.KmsThresholds memory newThresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 2,
            userDecryption: 1,
            kmsGen: 2,
            mpc: 1
        });
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeNodes(2, 4), newThresholds);
        uint256 secondContextId = protocolConfig.getCurrentKmsContextId();

        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(secondContextId), 2);
        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(firstContextId), 1);

        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidId));
        protocolConfig.getPublicDecryptionThresholdForContext(invalidId);
    }

    function test_thresholdsAfterContextRotation() public {
        _setupDefault();
        // Initial context uses thresholds {1, 2, 3, 4}.
        // Define a new context with different thresholds.
        IProtocolConfig.KmsThresholds memory newThresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 2,
            userDecryption: 1,
            kmsGen: 2,
            mpc: 1
        });

        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeNodes(2, 4), newThresholds);

        assertEq(protocolConfig.getPublicDecryptionThreshold(), 2);
        assertEq(protocolConfig.getUserDecryptionThreshold(), 1);
        assertEq(protocolConfig.getKmsGenThreshold(), 2);
        assertEq(protocolConfig.getMpcThreshold(), 1);
    }

    // -----------------------------------------------------------------------
    // Re-initialization protection
    // -----------------------------------------------------------------------

    function test_revertDoubleInit() public {
        _setupDefault();

        // onlyFromEmptyProxy fires first (version is 2, not 1) before reinitializer.
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        protocolConfig.initializeFromEmptyProxy(_makeNodes(1), _defaultThresholds());
    }

    function test_revertMigrationAfterInit() public {
        _setupDefault();

        uint256 migratedId = KMS_CONTEXT_COUNTER_BASE + 5;
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        protocolConfig.initializeFromMigration(migratedId, _makeNodes(1), _defaultThresholds());
    }

    // -----------------------------------------------------------------------
    // Access control
    // -----------------------------------------------------------------------

    function test_revertDefineContextNotOwner() public {
        _setupDefault();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.defineNewKmsContext(_makeNodes(1, 4), _defaultThresholds());
    }

    function test_revertDestroyContextNotOwner() public {
        _setupDefault();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.destroyKmsContext(KMS_CONTEXT_COUNTER_BASE + 1);
    }

    function test_revertUpgradeNotOwner() public {
        _setupDefault();

        address newImpl = address(new ProtocolConfigUpgradedExample());
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.upgradeToAndCall(newImpl, "");
    }

    function test_upgradeSuccess() public {
        _setupDefault();

        address newImpl = address(new ProtocolConfigUpgradedExample());
        vm.prank(owner);
        protocolConfig.upgradeToAndCall(newImpl, "");

        assertEq(protocolConfig.getVersion(), "ProtocolConfig v0.2.0");
        // State preserved across upgrade.
        assertTrue(protocolConfig.isValidKmsContext(protocolConfig.getCurrentKmsContextId()));
    }
}
