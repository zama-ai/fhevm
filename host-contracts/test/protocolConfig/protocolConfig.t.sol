// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {ProtocolConfigUpgradedExample} from "@fhevm-host-contracts/examples/ProtocolConfigUpgradedExample.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {KmsThresholds, KmsNode, KmsNodeParams, PcrValues, ChainUpgradeWindow} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE, KEY_COUNTER_BASE, CRS_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

contract ProtocolConfigTest is HostContractsDeployerTestUtils {
    address internal constant owner = address(456);
    uint256 internal immutable kmsPk0 = _defaultSignerPk(0);
    uint256 internal immutable kmsPk1 = _defaultSignerPk(1);
    uint256 internal immutable kmsPk2 = _defaultSignerPk(2);
    uint256 internal immutable kmsPk3 = _defaultSignerPk(3);
    address internal kmsTxSender0 = address(0xA1);

    function _setupDefault() internal {
        _deployACL(owner);
        /// @dev Distinct per-field values so each getter proves it reads the correct storage slot.
        KmsThresholds memory thresholds = KmsThresholds({publicDecryption: 1, userDecryption: 2, kmsGen: 3, mpc: 4});
        (ProtocolConfig pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(4), thresholds);
        protocolConfig = pc;
    }

    function _setupDefaultWithMpcThreshold(uint256 mpcThreshold) internal {
        _deployACL(owner);
        KmsThresholds memory thresholds = KmsThresholds({
            publicDecryption: 1,
            userDecryption: 2,
            kmsGen: 3,
            mpc: mpcThreshold
        });
        (ProtocolConfig pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(4), thresholds);
        protocolConfig = pc;
    }

    function _setupTwoNodeContext() internal {
        _deployACL(owner);
        (ProtocolConfig pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(2), _defaultThresholds());
        protocolConfig = pc;
    }

    function _upgradeProxyExpectRevert(
        KmsNodeParams[] memory nodes,
        KmsThresholds memory thresholds,
        bytes memory expectedRevert
    ) internal {
        address impl = address(new ProtocolConfig());
        vm.prank(owner);
        vm.expectRevert(expectedRevert);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(ProtocolConfig.initializeFromEmptyProxy, (nodes, thresholds, "", new PcrValues[](0)))
        );
    }

    function _revertThreshold(KmsThresholds memory t, bytes memory expectedRevert) internal {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        _upgradeProxyExpectRevert(_makeKmsNodeParams(1), t, expectedRevert);
    }

    /// @dev Defines a new epoch under the genesis context and activates it as `epochId` with the first
    ///      `signerCount` signers of `_makeKmsNodeParams`. A different assigned id reverts the confirmation.
    function _activateSameSetEpoch(uint256 epochId, uint256 signerCount) internal {
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        for (uint256 i = 0; i < signerCount; i++) {
            _confirmEpochActivationWithDummyMaterial(KMS_CONTEXT_COUNTER_BASE + 1, epochId, _defaultSignerPk(i));
        }
    }

    /// @dev Returns the high-s twin of `pk`'s signature over `digest`, and its s value.
    function _highSSignature(uint256 pk, bytes32 digest) internal pure returns (bytes memory signature, bytes32 highS) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest);
        // secp256k1 group order.
        uint256 n = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141;
        highS = bytes32(n - uint256(s));
        signature = abi.encodePacked(r, highS, v == 27 ? uint8(28) : uint8(27));
    }

    /// @dev Defines a context switch from the active committee to the two-node committee {pkA, pkB}.
    function _defineContextSwitchToSigners(uint256 pkA, uint256 pkB) internal returns (KmsNodeParams[] memory nodes) {
        nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(pkA);
        nodes[1].signerAddress = vm.addr(pkB);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
    }

    /// @dev Defines a context switch from the genesis committee to a committee with the same signers.
    function _defineSameSetContextSwitch() internal returns (uint256 newContextId) {
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        newContextId = protocolConfig.getCurrentKmsContextIdCounter();
    }

    /// @dev Confirms creation of a context switch to a disjoint signer set: signer0 (n - t = 1 of the two old signers) and both new signers.
    function _confirmDisjointSetContextSwitchCreation(uint256 contextId) internal {
        _confirmContextCreation(contextId, kmsPk0, EMPTY_EXTRA_DATA);
        _confirmContextCreation(contextId, kmsPk2, EMPTY_EXTRA_DATA);
        _confirmContextCreation(contextId, kmsPk3, EMPTY_EXTRA_DATA);
    }

    /// @dev Confirms the creation of `newContextId` for `pk` and expects that confirmation to create the
    ///      switch's epoch.
    function _confirmContextCreationExpectingFirstSwitchEpoch(uint256 newContextId, uint256 pk) internal {
        bytes memory signature = _signContextCreation(newContextId, pk, EMPTY_EXTRA_DATA);
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsEpoch(
            newContextId,
            EPOCH_COUNTER_BASE + 2,
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 1,
            block.number - 1
        );
        protocolConfig.confirmKmsContextCreation(newContextId, signature, EMPTY_EXTRA_DATA);
    }

    function _confirmContextCreationWithTwoSigners(uint256 contextId) internal {
        _confirmContextCreation(contextId, kmsPk0, EMPTY_EXTRA_DATA);
        _confirmContextCreation(contextId, kmsPk1, EMPTY_EXTRA_DATA);
    }

    /// @dev Asserts the liveness-guarded context view functions revert for the given context ID.
    ///      getKmsNodeForContext is existence-guarded (readable after destroy), so callers assert it separately.
    function _expectContextGuardedViewsRevert(uint256 contextId) internal {
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsSignersForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.isKmsSignerForContext(contextId, address(0xDEAD));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsNodesForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.isKmsTxSenderForContext(contextId, address(0xDEAD));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getUserDecryptionThresholdForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getPublicDecryptionThresholdForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsGenThresholdForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getMpcThresholdForContext(contextId);
    }

    /// @dev Entry i has key id KEY_COUNTER_BASE + 1 + i and CRS id CRS_COUNTER_BASE + 1 + i.
    function _buildTwoEpochResults()
        internal
        pure
        returns (IProtocolConfig.EpochKeyResult[] memory keys, IProtocolConfig.EpochCrsResult[] memory crsList)
    {
        keys = new IProtocolConfig.EpochKeyResult[](2);
        crsList = new IProtocolConfig.EpochCrsResult[](2);
        for (uint256 i = 0; i < 2; i++) {
            (
                IProtocolConfig.EpochKeyResult[] memory oneKey,
                IProtocolConfig.EpochCrsResult[] memory oneCrs
            ) = _buildEpochResults(KEY_COUNTER_BASE + 1 + i, CRS_COUNTER_BASE + 1 + i);
            keys[i] = oneKey[0];
            crsList[i] = oneCrs[0];
        }
    }

    /// @dev Under a new same-set epoch, signer0 confirms the first payload and signer1 the second payload
    ///      with `extraData1`. The active epoch stays unchanged.
    function _confirmSplitEpochActivationDoesNotActivate(
        IProtocolConfig.EpochKeyResult[] memory keys0,
        IProtocolConfig.EpochCrsResult[] memory crsList0,
        IProtocolConfig.EpochKeyResult[] memory keys1,
        IProtocolConfig.EpochCrsResult[] memory crsList1,
        bytes memory extraData1
    ) internal {
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        _confirmEpochActivation(contextId, epochId, kmsPk0, keys0, crsList0, EMPTY_EXTRA_DATA);
        _confirmEpochActivation(contextId, epochId, kmsPk1, keys1, crsList1, extraData1);

        (, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, EPOCH_COUNTER_BASE + 1);
    }

    function _countLogs(bytes32 topic) internal view returns (uint256 count) {
        Vm.Log[] memory logs = vm.getRecordedLogs();
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == topic) {
                count++;
            }
        }
    }

    // -----------------------------------------------------------------------
    // Init tests
    // -----------------------------------------------------------------------

    function test_initSuccess() public {
        _setupDefault();

        // Version and current context.
        assertEq(protocolConfig.getVersion(), "ProtocolConfig v0.4.0");
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        assertEq(contextId, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(protocolConfig.getCurrentKmsContextId(), contextId);
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, EPOCH_COUNTER_BASE + 1);
        assertTrue(protocolConfig.isValidKmsContext(contextId));

        // Thresholds.
        assertEq(protocolConfig.getPublicDecryptionThreshold(), 1);
        assertEq(protocolConfig.getUserDecryptionThreshold(), 2);
        assertEq(protocolConfig.getKmsGenThreshold(), 3);
        assertEq(protocolConfig.getKmsGenThresholdForContext(contextId), 3);
        assertEq(protocolConfig.getMpcThreshold(), 4);

        // Context node arrays and registered signer/tx sender mappings.
        KmsNodeParams[] memory expectedNodes = _makeKmsNodeParams(4);
        KmsNode[] memory nodes = protocolConfig.getKmsNodesForContext(contextId);
        address[] memory signers = protocolConfig.getKmsSignersForContext(contextId);
        assertEq(nodes.length, expectedNodes.length);
        assertEq(signers.length, expectedNodes.length);
        for (uint256 i = 0; i < expectedNodes.length; i++) {
            assertEq(nodes[i].txSenderAddress, expectedNodes[i].txSenderAddress);
            assertEq(nodes[i].signerAddress, expectedNodes[i].signerAddress);
            assertEq(nodes[i].ipAddress, expectedNodes[i].ipAddress);
            assertEq(nodes[i].storageUrl, expectedNodes[i].storageUrl);
            assertEq(signers[i], expectedNodes[i].signerAddress);
            assertTrue(protocolConfig.isKmsSignerForContext(contextId, expectedNodes[i].signerAddress));
            assertTrue(protocolConfig.isKmsTxSenderForContext(contextId, expectedNodes[i].txSenderAddress));

            // Direct node lookup by tx sender.
            KmsNode memory node = protocolConfig.getKmsNodeForContext(contextId, expectedNodes[i].txSenderAddress);
            assertEq(node.txSenderAddress, expectedNodes[i].txSenderAddress);
            assertEq(node.signerAddress, expectedNodes[i].signerAddress);
            assertEq(node.ipAddress, expectedNodes[i].ipAddress);
            assertEq(node.storageUrl, expectedNodes[i].storageUrl);
        }
        // Negative: unregistered addresses must return false.
        assertFalse(protocolConfig.isKmsSignerForContext(contextId, address(0xDEAD)));
        assertFalse(protocolConfig.isKmsTxSenderForContext(contextId, address(0xDEAD)));
    }

    function test_storageLocationPinned() public {
        _setupDefault();
        /// @dev A pending context makes the counter differ from the latest active context ID.
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());

        bytes32 slot = 0x80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200;
        uint256 stored = uint256(vm.load(address(protocolConfig), slot));
        assertEq(stored, protocolConfig.getCurrentKmsContextIdCounter());
        assertEq(stored, KMS_CONTEXT_COUNTER_BASE + 2);
    }

    // -----------------------------------------------------------------------
    // Validation error tests
    // -----------------------------------------------------------------------

    function test_revertEmptyNodes() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        KmsNodeParams[] memory emptyNodes = new KmsNodeParams[](0);
        _upgradeProxyExpectRevert(
            emptyNodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.EmptyKmsNodes.selector)
        );
    }

    function test_revertNullTxSender() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(1);
        nodes[0].txSenderAddress = address(0);
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsNodeNullTxSender.selector)
        );
    }

    function test_revertNullSigner() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(1);
        nodes[0].signerAddress = address(0);
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsNodeNullSigner.selector)
        );
    }

    function test_revertDuplicateTxSender() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[1].txSenderAddress = nodes[0].txSenderAddress;
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsTxSenderAlreadyRegistered.selector, nodes[0].txSenderAddress)
        );
    }

    function test_revertDuplicateSigner() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[1].signerAddress = nodes[0].signerAddress;
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsSignerAlreadyRegistered.selector, nodes[0].signerAddress)
        );
    }

    function test_revertNullPublicDecryptionThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.publicDecryption = 0;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidNullThreshold.selector, "publicDecryption")
        );
    }

    function test_revertHighPublicDecryptionThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.publicDecryption = 5;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "publicDecryption", 5, 1)
        );
    }

    function test_revertNullUserDecryptionThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.userDecryption = 0;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidNullThreshold.selector, "userDecryption")
        );
    }

    function test_revertHighUserDecryptionThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.userDecryption = 5;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "userDecryption", 5, 1)
        );
    }

    function test_revertNullKmsGenThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.kmsGen = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfigBase.InvalidNullThreshold.selector, "kmsGen"));
    }

    function test_revertHighKmsGenThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.kmsGen = 5;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "kmsGen", 5, 1));
    }

    function test_revertNullMpcThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.mpc = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfigBase.InvalidNullThreshold.selector, "mpc"));
    }

    function test_revertHighMpcThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.mpc = 5;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "mpc", 5, 1));
    }

    function test_revertSignerSetExceedsProofFormatLimit() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        KmsNodeParams[] memory tooManyNodes = _makeKmsNodeParams(256);
        _upgradeProxyExpectRevert(
            tooManyNodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsSignerSetExceedsProofFormatLimit.selector, 256, 255)
        );
    }

    // -----------------------------------------------------------------------
    // Context lifecycle tests
    // -----------------------------------------------------------------------

    function test_defineNewKmsContextAndEpochCreatesPendingContext() public {
        _setupDefault();

        KmsNodeParams[] memory newNodeParams = _makeKmsNodeParams(1);
        PcrValues[] memory pcrValues = new PcrValues[](0);

        KmsThresholds memory thresholds = _defaultThresholds();
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsContext(
            KMS_CONTEXT_COUNTER_BASE + 2,
            KMS_CONTEXT_COUNTER_BASE + 1,
            newNodeParams,
            thresholds,
            "",
            pcrValues
        );
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(newNodeParams, thresholds);

        assertEq(protocolConfig.getCurrentKmsContextId(), KMS_CONTEXT_COUNTER_BASE + 1);
        assertFalse(protocolConfig.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE + 2));
    }

    function test_defineNewKmsContextAndEpochStoresContextAnchor() public {
        _setupDefault();

        KmsNodeParams[] memory params = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultThresholds();
        PcrValues[] memory pcrValues = new PcrValues[](1);
        pcrValues[0] = PcrValues({
            pcr0: abi.encodePacked(uint256(1)),
            pcr1: abi.encodePacked(uint256(2)),
            pcr2: abi.encodePacked(uint256(3))
        });

        uint256 expectedBlock = block.number;
        vm.prank(owner);
        protocolConfig.defineNewKmsContextAndEpoch(params, thresholds, "kms-v1", pcrValues);

        (uint256 emissionBlockNumber, bytes32 contextInfoHash) = protocolConfig.getKmsContextAnchor(
            KMS_CONTEXT_COUNTER_BASE + 2
        );
        assertEq(emissionBlockNumber, expectedBlock);
        assertEq(contextInfoHash, keccak256(abi.encode(params, thresholds, "kms-v1", pcrValues)));
    }

    function test_historicalContextReadable() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 4);

        KmsNodeParams[] memory newNodes = _makeKmsNodeParams(1);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(newNodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 3;
        _activatePendingContextWithDefaultSigners(newContextId, newEpochId, 1);

        uint256 currentId = protocolConfig.getCurrentKmsContextId();
        assertTrue(currentId != firstContextId);
        assertTrue(protocolConfig.isValidKmsContext(firstContextId));
        address[] memory oldSigners = protocolConfig.getKmsSignersForContext(firstContextId);
        assertEq(oldSigners.length, 4);
    }

    function test_destroyContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        (uint256 firstEmissionBlockNumber, bytes32 firstContextInfoHash) = protocolConfig.getKmsContextAnchor(
            firstContextId
        );
        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 4);

        KmsNodeParams[] memory newNodes = _makeKmsNodeParams(1);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(newNodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithDefaultSigners(newContextId, EPOCH_COUNTER_BASE + 3, 1);

        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestroyed(firstContextId);
        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);
        assertFalse(protocolConfig.isValidKmsContext(firstContextId));
        (uint256 destroyedEmissionBlockNumber, bytes32 destroyedContextInfoHash) = protocolConfig.getKmsContextAnchor(
            firstContextId
        );
        assertEq(destroyedEmissionBlockNumber, firstEmissionBlockNumber);
        assertEq(destroyedContextInfoHash, firstContextInfoHash);
    }

    function test_revertDestroyCurrentContext() public {
        _setupDefault();
        uint256 currentId = protocolConfig.getCurrentKmsContextId();
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.LatestActiveKmsContextCannotBeDestroyed.selector, currentId)
        );
        protocolConfig.destroyKmsContext(currentId);
    }

    function testFuzz_revertDestroyInvalidContext(uint256 invalidContextId) public {
        _setupDefault();
        vm.assume(invalidContextId != protocolConfig.getCurrentKmsContextId());
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        protocolConfig.destroyKmsContext(invalidContextId);
    }

    function test_revertDestroyAlreadyDestroyedContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 4);

        KmsNodeParams[] memory newNodes = _makeKmsNodeParams(1);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(newNodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithDefaultSigners(newContextId, EPOCH_COUNTER_BASE + 3, 1);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, firstContextId));
        protocolConfig.destroyKmsContext(firstContextId);
    }

    function test_defineNewEpochForCurrentKmsContextDoesNotActivateImmediately() public {
        _setupTwoNodeContext();
        (uint256 contextId, uint256 epochId) = protocolConfig.getCurrentKmsContextAndEpoch();

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        (uint256 currentContextId, uint256 currentEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(currentContextId, contextId);
        assertEq(currentEpochId, epochId);
    }

    function test_revertDefineNewEpochForCurrentKmsContextNotOwner() public {
        _setupTwoNodeContext();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    function test_fullSameSetResharingFlow() public {
        _setupTwoNodeContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        _confirmEpochActivationWithDummyMaterial(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk0);
        (, uint256 activeEpochBeforeSecondConfirmation) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochBeforeSecondConfirmation, EPOCH_COUNTER_BASE + 1);

        _confirmEpochActivationWithDummyMaterial(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk1);

        (uint256 contextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(contextId, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(activeEpochId, epochId);
    }

    function test_isValidEpochForContext_trueOnFreshDeploy() public {
        _setupTwoNodeContext();
        assertTrue(protocolConfig.isValidEpochForContext(KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 1));
    }

    function test_isValidEpochForContext_falseForWrongContextId() public {
        _setupTwoNodeContext();
        // Active epoch exists, but paired with the wrong context.
        assertFalse(protocolConfig.isValidEpochForContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 1));
    }

    function test_isValidEpochForContext_falseForUnknownEpoch() public {
        _setupTwoNodeContext();
        assertFalse(protocolConfig.isValidEpochForContext(KMS_CONTEXT_COUNTER_BASE + 1, 0));
        assertFalse(protocolConfig.isValidEpochForContext(KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 999));
    }

    function test_isValidEpochForContext_falseForPendingSameSetEpoch() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        assertFalse(protocolConfig.isValidEpochForContext(contextId, pendingEpochId));
        // Previous active epoch still passes, the new one is Pending until activated.
        assertTrue(protocolConfig.isValidEpochForContext(contextId, EPOCH_COUNTER_BASE + 1));
    }

    function test_isValidEpochForContext_trueAfterSameSetActivation() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;
        _activateSameSetEpoch(newEpochId, 2);

        assertTrue(protocolConfig.isValidEpochForContext(contextId, newEpochId));
    }

    function test_isValidEpochForContext_falseForPendingEpochUnderPendingContext() public {
        _setupTwoNodeContext();
        uint256 pendingContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;

        _defineSameSetContextSwitch();

        assertFalse(protocolConfig.isValidEpochForContext(pendingContextId, pendingEpochId));
    }

    function test_isValidEpochForContext_trueAfterContextSwitchActivation() public {
        _setupTwoNodeContext();
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;

        _defineSameSetContextSwitch();
        _activatePendingContextWithDefaultSigners(newContextId, newEpochId, 2);

        assertTrue(protocolConfig.isValidEpochForContext(newContextId, newEpochId));
    }

    function test_isValidEpochForContext_oldPairStillTrueAfterContextSwitch() public {
        _setupTwoNodeContext();
        uint256 oldContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 oldEpochId = EPOCH_COUNTER_BASE + 1;
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;

        _defineSameSetContextSwitch();
        _activatePendingContextWithDefaultSigners(newContextId, newEpochId, 2);

        assertTrue(protocolConfig.isValidEpochForContext(oldContextId, oldEpochId));
    }

    function test_isValidEpochForContext_falseAfterContextDestroyed() public {
        _setupTwoNodeContext();
        uint256 oldContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 oldEpochId = EPOCH_COUNTER_BASE + 1;
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;

        _defineSameSetContextSwitch();
        _activatePendingContextWithDefaultSigners(newContextId, newEpochId, 2);

        // The old context's epoch stays Active after rotation, so the pair is still valid
        assertTrue(protocolConfig.isValidEpochForContext(oldContextId, oldEpochId));

        // but destroying the old context must invalidate its epoch too.
        vm.prank(owner);
        protocolConfig.destroyKmsContext(oldContextId);

        assertFalse(protocolConfig.isValidEpochForContext(oldContextId, oldEpochId));
    }

    function test_destroyEpoch() public {
        _setupTwoNodeContext();
        uint256 oldContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 oldEpochId = EPOCH_COUNTER_BASE + 1;
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;

        // Switch to a newer context+epoch so the old epoch is a superseded (non-current) Active epoch.
        _defineSameSetContextSwitch();
        _activatePendingContextWithDefaultSigners(newContextId, newEpochId, 2);
        assertTrue(protocolConfig.isValidEpochForContext(oldContextId, oldEpochId));

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestroyed(oldEpochId);
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(oldEpochId);

        assertFalse(protocolConfig.isValidEpochForContext(oldContextId, oldEpochId));
    }

    function test_revertDestroyCurrentEpoch() public {
        _setupTwoNodeContext();
        (, uint256 currentEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.LatestActiveKmsEpochCannotBeDestroyed.selector, currentEpochId)
        );
        protocolConfig.destroyKmsEpoch(currentEpochId);
    }

    function testFuzz_revertDestroyInvalidKmsEpoch(uint256 invalidEpochId) public {
        _setupTwoNodeContext();
        (, uint256 currentEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        vm.assume(invalidEpochId != currentEpochId);
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, invalidEpochId));
        protocolConfig.destroyKmsEpoch(invalidEpochId);
    }

    function test_destroyPendingEpochOfActiveContext() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;
        (, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();

        // A Pending epoch under the Active context (same-set rotation) is abortable.
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestroyed(pendingEpochId);
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(pendingEpochId);

        assertFalse(protocolConfig.isValidEpochForContext(contextId, pendingEpochId));
        (, uint256 epochAfter) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(epochAfter, activeEpochId);

        // Governance can re-trigger the rotation and complete it.
        uint256 retriedEpochId = EPOCH_COUNTER_BASE + 3;
        _activateSameSetEpoch(retriedEpochId, 2);
        (, uint256 epochFinal) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(epochFinal, retriedEpochId);
    }

    function test_destroyPendingEpochAfterDivergentConfirmations() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        // Divergent confirmations: the two signers attest to different key ids, so no result reaches quorum.
        (
            IProtocolConfig.EpochKeyResult[] memory keys0,
            IProtocolConfig.EpochCrsResult[] memory crsList0
        ) = _buildEpochResults(KEY_COUNTER_BASE + 1, CRS_COUNTER_BASE + 1);
        (
            IProtocolConfig.EpochKeyResult[] memory keys1,
            IProtocolConfig.EpochCrsResult[] memory crsList1
        ) = _buildEpochResults(KEY_COUNTER_BASE + 2, CRS_COUNTER_BASE + 1);
        _confirmEpochActivation(contextId, pendingEpochId, kmsPk0, keys0, crsList0, EMPTY_EXTRA_DATA);
        _confirmEpochActivation(contextId, pendingEpochId, kmsPk1, keys1, crsList1, EMPTY_EXTRA_DATA);

        // Each signer can confirm only once, so the split confirmations never reach quorum. Resubmitting the other
        // signer's result reverts with EpochActivationAlreadyConfirmed.
        bytes memory signature = _signEpochActivation(
            contextId,
            pendingEpochId,
            kmsPk0,
            keys1,
            crsList1,
            EMPTY_EXTRA_DATA
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.EpochActivationAlreadyConfirmed.selector,
                vm.addr(kmsPk0),
                pendingEpochId
            )
        );
        protocolConfig.confirmEpochActivation(pendingEpochId, keys1, crsList1, signature, EMPTY_EXTRA_DATA);
        (, uint256 epochBefore) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(epochBefore, EPOCH_COUNTER_BASE + 1);

        // Governance aborts the stuck epoch and re-runs the rotation to completion.
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(pendingEpochId);

        uint256 retriedEpochId = EPOCH_COUNTER_BASE + 3;
        _activateSameSetEpoch(retriedEpochId, 2);
        (, uint256 epochFinal) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(epochFinal, retriedEpochId);
    }

    /// @dev A context switch whose activation splits is settled by destroyKmsContext, not
    ///      destroyKmsEpoch. Governance then re-runs the switch to completion.
    function test_destroyContextAfterDivergentSwitchEpochActivation() public {
        _setupTwoNodeContext();
        KmsNodeParams[] memory nodes = _defineContextSwitchToSigners(kmsPk2, kmsPk3);
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;
        _confirmDisjointSetContextSwitchCreation(contextId);

        (
            IProtocolConfig.EpochKeyResult[] memory keys2,
            IProtocolConfig.EpochCrsResult[] memory crsList2
        ) = _buildEpochResults(KEY_COUNTER_BASE + 1, CRS_COUNTER_BASE + 1);
        (
            IProtocolConfig.EpochKeyResult[] memory keys3,
            IProtocolConfig.EpochCrsResult[] memory crsList3
        ) = _buildEpochResults(KEY_COUNTER_BASE + 2, CRS_COUNTER_BASE + 1);
        _confirmEpochActivation(contextId, pendingEpochId, kmsPk2, keys2, crsList2, EMPTY_EXTRA_DATA);
        _confirmEpochActivation(contextId, pendingEpochId, kmsPk3, keys3, crsList3, EMPTY_EXTRA_DATA);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, pendingEpochId));
        protocolConfig.destroyKmsEpoch(pendingEpochId);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(contextId);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 retriedContextId = KMS_CONTEXT_COUNTER_BASE + 3;
        uint256 retriedEpochId = EPOCH_COUNTER_BASE + 3;
        _confirmDisjointSetContextSwitchCreation(retriedContextId);
        _confirmEpochActivationWithDummyMaterial(retriedContextId, retriedEpochId, kmsPk2);
        _confirmEpochActivationWithDummyMaterial(retriedContextId, retriedEpochId, kmsPk3);
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, retriedContextId);
        assertEq(activeEpochId, retriedEpochId);
    }

    function test_revertDestroyPendingEpochOfPendingContext() public {
        _setupTwoNodeContext();
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;

        // A pending context switch is settled by destroyKmsContext, not destroyKmsEpoch.
        _defineSameSetContextSwitch();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, pendingEpochId));
        protocolConfig.destroyKmsEpoch(pendingEpochId);
    }

    function test_destroyPendingContextClearsPairedEpoch() public {
        _setupTwoNodeContext();
        uint256 pendingContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();

        // A pending context switch is never stuck: destroying the pending context settles the
        // whole pair, clearing the paired pending epoch with it.
        _defineSameSetContextSwitch();

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestroyed(pendingContextId);
        vm.prank(owner);
        protocolConfig.destroyKmsContext(pendingContextId);

        assertFalse(protocolConfig.isValidKmsContext(pendingContextId));
        assertFalse(protocolConfig.isValidEpochForContext(pendingContextId, pendingEpochId));
        (uint256 contextAfter, uint256 epochAfter) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(contextAfter, activeContextId);
        assertEq(epochAfter, activeEpochId);
    }

    function test_revertDestroyEpochNotOwner() public {
        _setupTwoNodeContext();
        (, uint256 currentEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.destroyKmsEpoch(currentEpochId);
    }

    function test_revertDestroyAlreadyDestroyedEpoch() public {
        _setupTwoNodeContext();
        uint256 oldEpochId = EPOCH_COUNTER_BASE + 1;
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;

        _defineSameSetContextSwitch();
        _activatePendingContextWithDefaultSigners(newContextId, newEpochId, 2);

        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(oldEpochId);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, oldEpochId));
        protocolConfig.destroyKmsEpoch(oldEpochId);
    }

    function test_defineNewKmsContextAndEpochDoesNotActivateImmediately() public {
        _setupTwoNodeContext();
        (uint256 oldContextId, uint256 oldEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        _defineContextSwitchToSigners(kmsPk2, kmsPk3);

        assertEq(protocolConfig.getCurrentKmsContextId(), oldContextId);
        (uint256 currentContextId, uint256 currentEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(currentContextId, oldContextId);
        assertEq(currentEpochId, oldEpochId);
        assertFalse(protocolConfig.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE + 2));
        assertTrue(protocolConfig.isKmsTxSenderForContext(KMS_CONTEXT_COUNTER_BASE + 2, kmsTxSender0));
        assertEq(protocolConfig.getKmsGenThresholdForContext(KMS_CONTEXT_COUNTER_BASE + 2), 1);
    }

    /// @dev Previous-side quorum boundary, both sides: with n = 4 previous nodes and t = 1, the
    ///      target is n - t = 3. All-new + n - t - 1 previous confirmations stay Pending, the
    ///      (n - t)-th completes creation (NewKmsEpoch), and any later one hits the Created state.
    function test_confirmKmsContextCreationUsesNewSignersAndOldQuorum() public {
        _setupDefaultWithMpcThreshold(1);
        _defineContextSwitchToSigners(0xB2, 0xB3);
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        _confirmContextCreation(newContextId, 0xB2, EMPTY_EXTRA_DATA);
        _confirmContextCreation(newContextId, 0xB3, EMPTY_EXTRA_DATA);
        _confirmContextCreation(newContextId, kmsPk0, EMPTY_EXTRA_DATA);
        _confirmContextCreation(newContextId, kmsPk1, EMPTY_EXTRA_DATA);
        assertFalse(protocolConfig.isValidKmsContext(newContextId));

        // Full-args assertion (indexed kmsContextId/epochId + data) on the quorum-completing event.
        _confirmContextCreationExpectingFirstSwitchEpoch(newContextId, kmsPk2);

        // The context left Pending on the (n - t)-th previous confirmation; a further one is rejected.
        bytes memory signature = _signContextCreation(newContextId, kmsPk3, EMPTY_EXTRA_DATA);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsContextNotPending.selector, newContextId));
        protocolConfig.confirmKmsContextCreation(newContextId, signature, EMPTY_EXTRA_DATA);
    }

    function test_confirmKmsContextCreationRequiresAllNewSigners() public {
        _setupDefaultWithMpcThreshold(3);
        _defineContextSwitchToSigners(0xB2, 0xB3);
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        _confirmContextCreation(newContextId, 0xB2, EMPTY_EXTRA_DATA);
        _confirmContextCreation(newContextId, kmsPk0, EMPTY_EXTRA_DATA);
        _confirmContextCreation(newContextId, kmsPk1, EMPTY_EXTRA_DATA);
        assertFalse(protocolConfig.isValidKmsContext(newContextId));

        // The quorum-completing confirmation (all new signers present) must emit NewKmsEpoch with the
        // pending epoch's full indexed args. Before this last confirmation no such event was emitted.
        _confirmContextCreationExpectingFirstSwitchEpoch(newContextId, 0xB3);
    }

    /// @dev The previous committee has n = 4 and t = 4, so n - t is 0 and the previous quorum is floored at 1.
    ///      The new committee alone cannot complete the switch. The first previous confirmation completes it.
    function test_confirmKmsContextCreationPreviousQuorumFlooredAtOne() public {
        _setupDefaultWithMpcThreshold(4);
        _defineContextSwitchToSigners(0xB2, 0xB3);
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        _confirmContextCreation(newContextId, 0xB2, EMPTY_EXTRA_DATA);
        _confirmContextCreation(newContextId, 0xB3, EMPTY_EXTRA_DATA);

        _confirmContextCreationExpectingFirstSwitchEpoch(newContextId, kmsPk0);
    }

    function test_revertConfirmEpochActivationBeforeCreateKmsContext() public {
        _setupTwoNodeContext();
        _defineContextSwitchToSigners(kmsPk2, kmsPk3);

        // The switch's epoch is only created once creation is confirmed, so activating the
        // would-be epoch ID before that reverts as an unknown epoch.
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, newEpochId));
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](0);
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);
        protocolConfig.confirmEpochActivation(newEpochId, keys, crsList, "", EMPTY_EXTRA_DATA);
    }

    function test_destroyCreatedContext() public {
        _setupTwoNodeContext();

        _defineSameSetContextSwitch();
        uint256 createdContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _confirmContextCreation(createdContextId, kmsPk0, EMPTY_EXTRA_DATA);
        _confirmContextCreationExpectingFirstSwitchEpoch(createdContextId, kmsPk1);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(createdContextId);
        assertFalse(protocolConfig.isValidKmsContext(createdContextId));

        // Destruction clears the epoch created at confirmation, so its activation reverts on the epoch state guard.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, EPOCH_COUNTER_BASE + 2));
        protocolConfig.confirmEpochActivation(
            EPOCH_COUNTER_BASE + 2,
            new IProtocolConfig.EpochKeyResult[](0),
            new IProtocolConfig.EpochCrsResult[](0),
            "",
            EMPTY_EXTRA_DATA
        );
    }

    function test_revertDefineSecondContextSwitchWhileSwitchPending() public {
        _setupTwoNodeContext();

        _defineSameSetContextSwitch();

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                KMS_CONTEXT_COUNTER_BASE + 2,
                EPOCH_COUNTER_BASE + 1
            )
        );
        protocolConfig.defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds(), "", new PcrValues[](0));
    }

    function test_revertDefineSecondContextSwitchWhileSwitchCreated() public {
        _setupTwoNodeContext();

        _defineSameSetContextSwitch();
        _confirmContextCreationWithTwoSigners(KMS_CONTEXT_COUNTER_BASE + 2);

        // The Created context and the epoch created at its confirmation are both still settling.
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                KMS_CONTEXT_COUNTER_BASE + 2,
                EPOCH_COUNTER_BASE + 2
            )
        );
        protocolConfig.defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds(), "", new PcrValues[](0));
    }

    function test_revertDefineNewEpochWhileContextSwitchPending() public {
        _setupTwoNodeContext();

        _defineSameSetContextSwitch();

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                KMS_CONTEXT_COUNTER_BASE + 2,
                EPOCH_COUNTER_BASE + 1
            )
        );
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    function test_revertDefineContextSwitchWhileReshareEpochPending() public {
        _setupTwoNodeContext();

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                KMS_CONTEXT_COUNTER_BASE + 1,
                EPOCH_COUNTER_BASE + 2
            )
        );
        protocolConfig.defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds(), "", new PcrValues[](0));
    }

    function test_revertDefineSecondReshareEpochWhileReshareEpochPending() public {
        _setupTwoNodeContext();

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                KMS_CONTEXT_COUNTER_BASE + 1,
                EPOCH_COUNTER_BASE + 2
            )
        );
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    function test_defineNewContextSwitchAfterDestroyingPendingSwitch() public {
        _setupTwoNodeContext();

        _defineSameSetContextSwitch();

        // Destroying the pending switch must reopen the gate: destroyed entries are None,
        // not in flight.
        vm.prank(owner);
        protocolConfig.destroyKmsContext(KMS_CONTEXT_COUNTER_BASE + 2);

        // The active pair is untouched: no epoch existed for the destroyed switch yet.
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(activeEpochId, EPOCH_COUNTER_BASE + 1);
        assertTrue(protocolConfig.isValidEpochForContext(KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 1));

        _defineSameSetContextSwitch();
        (uint256 emissionBlockNumber, ) = protocolConfig.getKmsContextAnchor(KMS_CONTEXT_COUNTER_BASE + 3);
        assertEq(emissionBlockNumber, block.number);
    }

    function test_defineNewContextSwitchAfterDestroyingCreatedSwitch() public {
        _setupTwoNodeContext();

        _defineSameSetContextSwitch();
        _confirmContextCreationWithTwoSigners(KMS_CONTEXT_COUNTER_BASE + 2);

        // Destroying the created switch clears the epoch created at its confirmation and reopens the gate.
        vm.prank(owner);
        protocolConfig.destroyKmsContext(KMS_CONTEXT_COUNTER_BASE + 2);

        _defineSameSetContextSwitch();
        (uint256 emissionBlockNumber, ) = protocolConfig.getKmsContextAnchor(KMS_CONTEXT_COUNTER_BASE + 3);
        assertEq(emissionBlockNumber, block.number);
    }

    function test_fullContextSwitchFlow() public {
        _setupTwoNodeContext();
        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 2);
        _defineContextSwitchToSigners(kmsPk2, kmsPk3);

        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 3;
        _confirmContextCreationWithTwoSigners(newContextId);
        (uint256 contextBeforeCreation, uint256 epochBeforeCreation) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(contextBeforeCreation, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(epochBeforeCreation, EPOCH_COUNTER_BASE + 2);
        _confirmContextCreation(newContextId, kmsPk2, EMPTY_EXTRA_DATA);
        _confirmContextCreation(newContextId, kmsPk3, EMPTY_EXTRA_DATA);
        (uint256 contextBeforeActivation, uint256 epochBeforeActivation) = protocolConfig
            .getCurrentKmsContextAndEpoch();
        assertEq(contextBeforeActivation, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(epochBeforeActivation, EPOCH_COUNTER_BASE + 2);

        _confirmEpochActivationWithDummyMaterial(newContextId, newEpochId, kmsPk2);
        assertEq(protocolConfig.getCurrentKmsContextId(), KMS_CONTEXT_COUNTER_BASE + 1);

        _confirmEpochActivationWithDummyMaterial(newContextId, newEpochId, kmsPk3);

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, newContextId);
        assertEq(activeEpochId, newEpochId);
    }

    function test_defineNewEpochForCurrentKmsContextEmitsMaterialBlockNumber() public {
        _setupTwoNodeContext();
        uint256 materialEpochId = EPOCH_COUNTER_BASE + 2;
        _activateSameSetEpoch(materialEpochId, 2);

        // Same-set resharing: previousContextId equals the current context.
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsEpoch(
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 3,
            KMS_CONTEXT_COUNTER_BASE + 1,
            materialEpochId,
            block.number - 1
        );
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    function test_revertConfirmKmsContextCreationUnauthorizedAndReplay() public {
        _setupTwoNodeContext();
        _defineSameSetContextSwitch();
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        // A signer in neither committee is rejected.
        bytes memory signature = _signContextCreation(newContextId, 0x999, EMPTY_EXTRA_DATA);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsContextCreationUnauthorized.selector,
                vm.addr(0x999),
                newContextId
            )
        );
        protocolConfig.confirmKmsContextCreation(newContextId, signature, EMPTY_EXTRA_DATA);

        // A signer confirms once, whatever the extraData of the second signature.
        _confirmContextCreation(newContextId, kmsPk0, EMPTY_EXTRA_DATA);
        signature = _signContextCreation(newContextId, kmsPk0, hex"01");
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsContextCreationAlreadyConfirmed.selector,
                vm.addr(kmsPk0),
                newContextId
            )
        );
        protocolConfig.confirmKmsContextCreation(newContextId, signature, hex"01");
    }

    function test_confirmEpochActivationDivergentDigestsAccumulateSeparately() public {
        _setupTwoNodeContext();
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();
        (
            IProtocolConfig.EpochKeyResult[] memory otherKeys,
            IProtocolConfig.EpochCrsResult[] memory otherCrsList
        ) = _dummyEpochResults();
        otherKeys[0].keyDigests[0].digest = hex"01020304";

        _confirmSplitEpochActivationDoesNotActivate(keys, crsList, otherKeys, otherCrsList, EMPTY_EXTRA_DATA);
    }

    function test_revertConfirmEpochActivationUnauthorizedAndReplay() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        // A signer outside the epoch's committee is rejected, even when all its signatures agree.
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();
        bytes memory signature = _signEpochActivation(contextId, epochId, 0x999, keys, crsList, EMPTY_EXTRA_DATA);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.EpochActivationUnauthorized.selector, vm.addr(0x999), epochId)
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, EMPTY_EXTRA_DATA);

        _confirmEpochActivationWithDummyMaterial(contextId, epochId, kmsPk0);

        // Replay with the same material the first confirm used. The signer check passes, so the call reaches
        // the already-confirmed check. An empty array would revert earlier with EmptyEpochActivationResults.
        (keys, crsList) = _dummyEpochResults();
        signature = _signEpochActivation(contextId, epochId, kmsPk0, keys, crsList, EMPTY_EXTRA_DATA);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.EpochActivationAlreadyConfirmed.selector, vm.addr(kmsPk0), epochId)
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, EMPTY_EXTRA_DATA);
    }

    function test_revertConfirmEpochActivationEmptyPayload() public {
        _setupTwoNodeContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        // A live pending epoch passes the state check, so an empty key or CRS list reaches the dedicated revert.
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();
        IProtocolConfig.EpochKeyResult[] memory noKeys = new IProtocolConfig.EpochKeyResult[](0);
        IProtocolConfig.EpochCrsResult[] memory noCrs = new IProtocolConfig.EpochCrsResult[](0);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.EmptyEpochActivationResults.selector, epochId));
        protocolConfig.confirmEpochActivation(epochId, noKeys, noCrs, "", EMPTY_EXTRA_DATA);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.EmptyEpochActivationResults.selector, epochId));
        protocolConfig.confirmEpochActivation(epochId, keys, noCrs, "", EMPTY_EXTRA_DATA);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.EmptyEpochActivationResults.selector, epochId));
        protocolConfig.confirmEpochActivation(epochId, noKeys, crsList, "", EMPTY_EXTRA_DATA);
    }

    function test_activateEpochEventCarriesMaterialIds() public {
        _setupTwoNodeContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        _confirmEpochActivationWithDummyMaterial(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk0);

        vm.recordLogs();
        _confirmEpochActivationWithDummyMaterial(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk1);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        uint256 activateLogIndex = type(uint256).max;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == IProtocolConfig.ActivateEpoch.selector) {
                activateLogIndex = i;
                break;
            }
        }
        assertTrue(activateLogIndex != type(uint256).max);
        assertEq(uint256(logs[activateLogIndex].topics[1]), KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(uint256(logs[activateLogIndex].topics[2]), epochId);

        (uint256 eventKeyId, uint256 eventCrsId) = this._decodeActivateEpochEventData(logs[activateLogIndex].data);
        assertEq(eventKeyId, KEY_COUNTER_BASE + 1);
        assertEq(eventCrsId, CRS_COUNTER_BASE + 1);
    }

    /// @dev `external` (called via `this.`) with a single `calldata` argument so the heavy
    ///      multi-array `abi.decode` runs in its own minimal stack frame; this keeps the
    ///      legacy (non-IR) codegen below the EVM stack limit.
    function _decodeActivateEpochEventData(bytes calldata data) external returns (uint256 keyId, uint256 crsId) {
        (
            IProtocolConfig.EpochKeyResult[] memory eventKeys,
            IProtocolConfig.EpochCrsResult[] memory eventCrsList,
            string[] memory urls
        ) = abi.decode(data, (IProtocolConfig.EpochKeyResult[], IProtocolConfig.EpochCrsResult[], string[]));
        assertEq(eventKeys.length, 1);
        assertEq(eventCrsList.length, 1);
        // URLs must match the activated context's nodes in insertion order.
        assertEq(urls.length, 2);
        assertEq(urls[0], "https://s0.example.com");
        assertEq(urls[1], "https://s1.example.com");
        keyId = eventKeys[0].keyId;
        crsId = eventCrsList[0].crsId;
    }

    function test_epochIdsUseTaggedCounterAndIncrementGlobally() public {
        _setupTwoNodeContext();

        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 2);

        _defineSameSetContextSwitch();

        (, uint256 activeEpochBeforeContextActivation) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochBeforeContextActivation, EPOCH_COUNTER_BASE + 2);
        _confirmContextCreationWithTwoSigners(KMS_CONTEXT_COUNTER_BASE + 2);
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildEpochResults(KEY_COUNTER_BASE + 2, CRS_COUNTER_BASE + 2);
        _confirmEpochActivation(
            KMS_CONTEXT_COUNTER_BASE + 2,
            EPOCH_COUNTER_BASE + 3,
            kmsPk0,
            keys,
            crsList,
            EMPTY_EXTRA_DATA
        );
        _confirmEpochActivation(
            KMS_CONTEXT_COUNTER_BASE + 2,
            EPOCH_COUNTER_BASE + 3,
            kmsPk1,
            keys,
            crsList,
            EMPTY_EXTRA_DATA
        );
        (, uint256 finalActiveEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(finalActiveEpochId, EPOCH_COUNTER_BASE + 3);
    }

    function test_emptyProxyInitializer_emitsNewKmsContext() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();

        address impl = address(new ProtocolConfig());
        KmsNodeParams[] memory nodeParams = _makeKmsNodeParams(2);
        PcrValues[] memory pcrValues = new PcrValues[](0);
        KmsThresholds memory thresholds = _defaultThresholds();

        vm.expectEmit(true, true, false, true, protocolConfigAdd);
        emit IProtocolConfig.NewKmsContext(
            KMS_CONTEXT_COUNTER_BASE + 1,
            KMS_CONTEXT_COUNTER_BASE,
            nodeParams,
            thresholds,
            "",
            pcrValues
        );

        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(ProtocolConfig.initializeFromEmptyProxy, (nodeParams, thresholds, "", pcrValues))
        );
    }

    // -----------------------------------------------------------------------
    // View-function guards (invalid & destroyed contexts)
    // -----------------------------------------------------------------------

    function testFuzz_revertViewFunctionsForInvalidContext(uint256 invalidContextId) public {
        _setupDefault();
        vm.assume(invalidContextId != protocolConfig.getCurrentKmsContextId());
        _expectContextGuardedViewsRevert(invalidContextId);
        // A never-created context does not exist, so the node lookup reverts too.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        protocolConfig.getKmsNodeForContext(invalidContextId, address(0xDEAD));
    }

    function test_revertViewFunctionsForDestroyedContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 4);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());
        _activatePendingContextWithDefaultSigners(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 3, 1);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);

        _expectContextGuardedViewsRevert(firstContextId);
    }

    /// @dev Key/CRS material keeps pointing at the context it was generated under, so its nodes must
    ///      stay readable once that context is destroyed.
    function test_getKmsNodeForContextReadableAfterDestroy() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 4);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());
        _activatePendingContextWithDefaultSigners(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 3, 1);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);

        KmsNodeParams memory expectedNode = _makeKmsNodeParams(4)[0];
        KmsNode memory node = protocolConfig.getKmsNodeForContext(firstContextId, expectedNode.txSenderAddress);
        assertEq(node.txSenderAddress, expectedNode.txSenderAddress);
        assertEq(node.signerAddress, expectedNode.signerAddress);
        assertEq(node.storageUrl, expectedNode.storageUrl);
    }

    // -----------------------------------------------------------------------
    // Threshold getters after context rotation
    // -----------------------------------------------------------------------

    function test_getUserDecryptionThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        // _setupDefault uses userDecryption = 2
        assertEq(protocolConfig.getUserDecryptionThresholdForContext(firstContextId), 2);
        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 4);

        // Rotate to a new context with userDecryption = 1
        KmsThresholds memory newThresholds = KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1});
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithDefaultSigners(secondContextId, EPOCH_COUNTER_BASE + 3, 2);

        // New context returns its own threshold
        assertEq(protocolConfig.getUserDecryptionThresholdForContext(secondContextId), 1);
        // Old context still returns the original threshold
        assertEq(protocolConfig.getUserDecryptionThresholdForContext(firstContextId), 2);
    }

    function test_getKmsGenThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getKmsGenThresholdForContext(firstContextId), 3);
        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 4);

        KmsThresholds memory newThresholds = KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 2, mpc: 1});
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithDefaultSigners(secondContextId, EPOCH_COUNTER_BASE + 3, 2);

        assertEq(protocolConfig.getKmsGenThresholdForContext(secondContextId), 2);
        assertEq(protocolConfig.getKmsGenThresholdForContext(firstContextId), 3);

        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidId));
        protocolConfig.getKmsGenThresholdForContext(invalidId);
    }

    function test_getMpcThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getMpcThresholdForContext(firstContextId), 4);
        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 4);

        KmsThresholds memory newThresholds = KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 2});
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithDefaultSigners(secondContextId, EPOCH_COUNTER_BASE + 3, 2);

        assertEq(protocolConfig.getMpcThresholdForContext(secondContextId), 2);
        assertEq(protocolConfig.getMpcThresholdForContext(firstContextId), 4);

        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidId));
        protocolConfig.getMpcThresholdForContext(invalidId);
    }

    function test_getPublicDecryptionThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(firstContextId), 1);
        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 4);

        KmsThresholds memory newThresholds = KmsThresholds({publicDecryption: 2, userDecryption: 1, kmsGen: 2, mpc: 1});
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithDefaultSigners(secondContextId, EPOCH_COUNTER_BASE + 3, 2);

        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(secondContextId), 2);
        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(firstContextId), 1);

        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidId));
        protocolConfig.getPublicDecryptionThresholdForContext(invalidId);
    }

    function test_thresholdsAfterContextRotation() public {
        _setupDefault();
        _activateSameSetEpoch(EPOCH_COUNTER_BASE + 2, 4);
        // Initial context uses thresholds {1, 2, 3, 4}.
        // Define a new context with different thresholds.
        KmsThresholds memory newThresholds = KmsThresholds({publicDecryption: 2, userDecryption: 1, kmsGen: 2, mpc: 1});

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        _activatePendingContextWithDefaultSigners(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 3, 2);

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

        // onlyFromEmptyProxy fires first (version is 3, not 1) before reinitializer.
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        protocolConfig.initializeFromEmptyProxy(_makeKmsNodeParams(1), _defaultThresholds(), "", new PcrValues[](0));
    }

    // -----------------------------------------------------------------------
    // Access control
    // -----------------------------------------------------------------------

    function test_revertDefineNewKmsContextAndEpochNotOwner() public {
        _setupDefault();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds(), "", new PcrValues[](0));
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

        assertEq(protocolConfig.getVersion(), "ProtocolConfig v0.5.0");
        // State preserved across upgrade.
        assertTrue(protocolConfig.isValidKmsContext(protocolConfig.getCurrentKmsContextId()));
    }

    // -----------------------------------------------------------------------
    // reinitializeV4 upgrade path
    // -----------------------------------------------------------------------

    /// @dev Calls reinitializeV4() on a proxy pinned at the initialized version the v0.3.0
    ///      implementation left behind (4), as the upgrade tooling does. The pre-store check proves
    ///      the hardcoded slot is where OZ Initializable writes.
    function test_reinitializeV4SucceedsOnUpgradePath() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        address impl = address(new ProtocolConfig());
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(impl, "");

        assertEq(uint256(vm.load(protocolConfigAdd, INITIALIZABLE_STORAGE)), 1);
        vm.store(protocolConfigAdd, INITIALIZABLE_STORAGE, bytes32(uint256(4)));

        ProtocolConfig(protocolConfigAdd).reinitializeV4();
        assertEq(uint256(vm.load(protocolConfigAdd, INITIALIZABLE_STORAGE)), 5);

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        ProtocolConfig(protocolConfigAdd).reinitializeV4();
    }

    /// @dev Upgrades a v0.15 proxy in place. v0.15 shares the ProtocolConfigStorage layout and its initializer
    ///      writes no canonical storage, so a fresh proxy pinned at version 4 matches a v0.15 proxy.
    function test_reinitializeV4OnV015StateThenRotatesEpoch() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        vm.store(protocolConfigAdd, INITIALIZABLE_STORAGE, bytes32(uint256(4)));
        address impl = address(new ProtocolConfig());
        vm.prank(owner);
        protocolConfig.upgradeToAndCall(impl, abi.encodeCall(ProtocolConfig.reinitializeV4, ()));

        // The confirm path reads the committee that v0.15 stored, so a rotation completes after the upgrade.
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        _activateSameSetEpoch(epochId, 2);
        (uint256 currentContextId, uint256 currentEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(currentContextId, contextId);
        assertEq(currentEpochId, epochId);
    }

    // -----------------------------------------------------------------------
    // confirmEpochActivation negative branches
    // -----------------------------------------------------------------------

    /// @dev Partial quorum (one of two new signers confirming context creation) must not advance the
    ///      live context id.
    function test_partialContextCreationQuorumDoesNotAdvance() public {
        _setupTwoNodeContext();
        uint256 activeContextIdBefore = protocolConfig.getCurrentKmsContextId();

        _defineSameSetContextSwitch();
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        // Only one signer confirms: quorum not reached.
        _confirmContextCreation(newContextId, kmsPk0, EMPTY_EXTRA_DATA);

        assertEq(protocolConfig.getCurrentKmsContextId(), activeContextIdBefore);
        assertFalse(protocolConfig.isValidKmsContext(newContextId));
    }

    /// @dev n=3 epoch-activation split: two signers agree on one digest, a third diverges. Neither
    ///      group reaches the full-quorum (3) threshold, so the epoch never activates. Distinct from the
    ///      n=2 divergence test, which can never have an agreeing majority.
    function test_divergentDigestSplitThreeSignersDoesNotActivate() public {
        // Deploy a 3-node context directly as the active context (signers kmsPk0/1/2, kmsGen threshold 1).
        _deployACL(owner);
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(3);
        KmsThresholds memory thresholds = KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1});
        (ProtocolConfig pc, ) = _deployProtocolConfig(owner, nodes, thresholds);
        protocolConfig = pc;

        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;

        // Same-set resharing epoch under the active 3-node context.
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (, uint256 activeEpochBefore) = protocolConfig.getCurrentKmsContextAndEpoch();

        // Two signers agree on the matching digest.
        _confirmEpochActivationWithDummyMaterial(contextId, epochId, kmsPk0);
        _confirmEpochActivationWithDummyMaterial(contextId, epochId, kmsPk1);

        // Third signer diverges (different key digest) — accumulates under a separate hash.
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();
        keys[0].keyDigests[0].digest = hex"99887766";
        _confirmEpochActivation(contextId, epochId, kmsPk2, keys, crsList, EMPTY_EXTRA_DATA);

        // No digest group reached the full quorum of 3 — epoch unchanged.
        (uint256 activeContextAfter, uint256 activeEpochAfter) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochAfter, activeEpochBefore);
        assertEq(activeContextAfter, contextId);
    }

    // -----------------------------------------------------------------------
    // Per-confirmation events and malformed-signature negative
    // -----------------------------------------------------------------------

    /// @dev Asserts KmsContextCreationConfirmation carries the recovered signer, the signature and extraData.
    function test_kmsContextCreationConfirmationEvent() public {
        _setupTwoNodeContext();
        uint256 newContextId = _defineSameSetContextSwitch();

        bytes memory extraData = hex"01ab";
        bytes memory signature = _signContextCreation(newContextId, kmsPk1, extraData);
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextCreationConfirmation(newContextId, vm.addr(kmsPk1), signature, extraData);
        protocolConfig.confirmKmsContextCreation(newContextId, signature, extraData);
    }

    /// @dev Asserts EpochActivationConfirmation carries the signer, epochMaterialHash, signature and extraData.
    function test_epochActivationConfirmationEvent() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();
        bytes memory extraData = hex"01ab";
        bytes memory signature = _signEpochActivation(contextId, epochId, kmsPk0, keys, crsList, extraData);
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.EpochActivationConfirmation(
            epochId,
            vm.addr(kmsPk0),
            _hashEpochMaterial(keys, crsList),
            signature,
            extraData
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, extraData);
    }

    /// @dev A malformed (too-short) EpochActivationConfirmation signature must hit ECDSA.recover's length validation.
    function test_revertConfirmEpochActivationMalformedSignature() public {
        _setupTwoNodeContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();

        // 65 bytes is the only valid ECDSA length. ECDSA.recover rejects a 10-byte blob.
        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureLength.selector, 10));
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, hex"00112233445566778899", EMPTY_EXTRA_DATA);
    }

    /// @dev OZ ECDSA rejects the high-s twin of a valid EpochActivationConfirmation signature.
    function test_revertConfirmEpochActivationMalleableSignature() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();

        (bytes memory signature, bytes32 highS) = _highSSignature(
            kmsPk0,
            _hashEpochActivation(
                contextId,
                EPOCH_COUNTER_BASE + 1,
                epochId,
                _hashEpochMaterial(keys, crsList),
                EMPTY_EXTRA_DATA
            )
        );
        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureS.selector, highS));
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, EMPTY_EXTRA_DATA);
    }

    // -----------------------------------------------------------------------
    // EIP-712 replay
    // -----------------------------------------------------------------------

    /// @dev A creation signature for a destroyed switch does not count for the next switch, even when
    ///      governance redefines the same nodes and thresholds.
    function test_revertConfirmKmsContextCreationReplayOntoRedefinedContext() public {
        _setupTwoNodeContext();
        uint256 destroyedContextId = _defineSameSetContextSwitch();
        bytes memory signature = _signContextCreation(destroyedContextId, kmsPk0, EMPTY_EXTRA_DATA);
        protocolConfig.confirmKmsContextCreation(destroyedContextId, signature, EMPTY_EXTRA_DATA);
        vm.prank(owner);
        protocolConfig.destroyKmsContext(destroyedContextId);

        uint256 redefinedContextId = _defineSameSetContextSwitch();
        assertEq(nodeConfigHashes[redefinedContextId], nodeConfigHashes[destroyedContextId]);
        vm.expectPartialRevert(IProtocolConfig.KmsContextCreationUnauthorized.selector);
        protocolConfig.confirmKmsContextCreation(redefinedContextId, signature, EMPTY_EXTRA_DATA);
    }

    /// @dev Each confirmation type has its own type hash. An EpochDestructionConfirmation signature
    ///      over a destroyed context's ID does not count as its ContextDestructionConfirmation.
    function test_revertConfirmKmsContextDestructionCrossTypeReplay() public {
        _setupTwoNodeContext();
        uint256 contextId = _defineSameSetContextSwitch();
        vm.prank(owner);
        protocolConfig.destroyKmsContext(contextId);

        bytes memory signature = _computeSignature(kmsPk0, _hashEpochDestruction(contextId, EMPTY_EXTRA_DATA));
        vm.expectPartialRevert(IProtocolConfig.KmsDestructionUnauthorized.selector);
        protocolConfig.confirmKmsContextDestruction(contextId, signature, EMPTY_EXTRA_DATA);

        signature = _signContextCreation(contextId, kmsPk0, EMPTY_EXTRA_DATA);
        vm.expectPartialRevert(IProtocolConfig.KmsDestructionUnauthorized.selector);
        protocolConfig.confirmKmsContextDestruction(contextId, signature, EMPTY_EXTRA_DATA);
    }

    // -----------------------------------------------------------------------
    // EIP-712 context creation confirmations
    // -----------------------------------------------------------------------

    /// @dev A signer in both committees confirms once and counts toward both sides. New committee
    ///      {signer0, 0xB2}, previous quorum n - t = 3: after 0xB2, signer1 and signer2, both sides
    ///      lack one confirmation, and signer0 alone completes the creation.
    function test_confirmKmsContextCreationSharedSignerCountsInBothCommittees() public {
        _setupDefaultWithMpcThreshold(1);
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[1].signerAddress = vm.addr(0xB2);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        _confirmContextCreation(newContextId, 0xB2, EMPTY_EXTRA_DATA);
        _confirmContextCreation(newContextId, kmsPk1, EMPTY_EXTRA_DATA);
        _confirmContextCreation(newContextId, kmsPk2, EMPTY_EXTRA_DATA);
        assertFalse(protocolConfig.isValidKmsContext(newContextId));

        _confirmContextCreationExpectingFirstSwitchEpoch(newContextId, kmsPk0);
    }

    /// @dev OZ ECDSA rejects the high-s twin of a valid signature and the 64-byte compact form.
    function test_revertConfirmKmsContextCreationMalleableSignature() public {
        _setupTwoNodeContext();
        uint256 newContextId = _defineSameSetContextSwitch();

        bytes32 digest = _hashContextCreation(
            KMS_CONTEXT_COUNTER_BASE + 1,
            newContextId,
            nodeConfigHashes[newContextId],
            EMPTY_EXTRA_DATA
        );
        (bytes memory highSSignature, bytes32 highS) = _highSSignature(kmsPk0, digest);
        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureS.selector, highS));
        protocolConfig.confirmKmsContextCreation(newContextId, highSSignature, EMPTY_EXTRA_DATA);

        (, bytes32 r, bytes32 s) = vm.sign(kmsPk0, digest);
        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureLength.selector, 64));
        protocolConfig.confirmKmsContextCreation(newContextId, abi.encodePacked(r, s), EMPTY_EXTRA_DATA);
    }

    /// @dev nodeConfigHash binds the stored nodes and thresholds. A signature over other thresholds
    ///      recovers an unrelated address, and the same signer over the defined ones is accepted.
    function test_revertConfirmKmsContextCreationOverOtherNodeConfig() public {
        _setupTwoNodeContext();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultThresholds();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, thresholds);
        uint256 previousContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        KmsThresholds memory otherThresholds = KmsThresholds({
            publicDecryption: 1,
            userDecryption: 1,
            kmsGen: 1,
            mpc: 2
        });
        bytes memory signature = _computeSignature(
            kmsPk0,
            _hashContextCreation(
                previousContextId,
                newContextId,
                _hashNodeConfig(nodes, otherThresholds),
                EMPTY_EXTRA_DATA
            )
        );
        vm.expectPartialRevert(IProtocolConfig.KmsContextCreationUnauthorized.selector);
        protocolConfig.confirmKmsContextCreation(newContextId, signature, EMPTY_EXTRA_DATA);

        signature = _computeSignature(
            kmsPk0,
            _hashContextCreation(previousContextId, newContextId, _hashNodeConfig(nodes, thresholds), EMPTY_EXTRA_DATA)
        );
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextCreationConfirmation(newContextId, vm.addr(kmsPk0), signature, EMPTY_EXTRA_DATA);
        protocolConfig.confirmKmsContextCreation(newContextId, signature, EMPTY_EXTRA_DATA);
    }

    /// @dev Confirmations count per digest. Two signers with different extraData never reach the
    ///      all-new-signers quorum, so no epoch is created for the switch.
    function test_confirmKmsContextCreationDivergentExtraDataDoesNotComplete() public {
        _setupTwoNodeContext();
        uint256 newContextId = _defineSameSetContextSwitch();

        _confirmContextCreation(newContextId, kmsPk0, EMPTY_EXTRA_DATA);
        _confirmContextCreation(newContextId, kmsPk1, hex"01");

        // The latest-issued epoch is still the genesis one, so the switch never got its epoch.
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                newContextId,
                EPOCH_COUNTER_BASE + 1
            )
        );
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    /// @dev The previous committee has n = 4 and t = 1, so its quorum is 3. One previous signer
    ///      confirms other extraData, and the other three still complete the switch.
    function test_confirmKmsContextCreationCompletesDespiteDivergentPreviousSigner() public {
        _setupDefaultWithMpcThreshold(1);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        _confirmContextCreation(newContextId, kmsPk3, hex"01");
        _confirmContextCreation(newContextId, kmsPk1, EMPTY_EXTRA_DATA);
        _confirmContextCreation(newContextId, kmsPk2, EMPTY_EXTRA_DATA);

        _confirmContextCreationExpectingFirstSwitchEpoch(newContextId, kmsPk0);
    }

    /// @dev Both quorums must hold on one digest. The previous quorum on extraData X and new-committee
    ///      unanimity on extraData Y do not complete the switch.
    function test_confirmKmsContextCreationQuorumsOnDifferentExtraDataDoNotComplete() public {
        _setupDefaultWithMpcThreshold(1);
        _defineContextSwitchToSigners(0xB2, 0xB3);
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        // The previous quorum is n - t = 3 for the genesis committee.
        _confirmContextCreation(newContextId, kmsPk0, hex"01");
        _confirmContextCreation(newContextId, kmsPk1, hex"01");
        _confirmContextCreation(newContextId, kmsPk2, hex"01");
        _confirmContextCreation(newContextId, 0xB2, hex"02");
        _confirmContextCreation(newContextId, 0xB3, hex"02");

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                newContextId,
                EPOCH_COUNTER_BASE + 1
            )
        );
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    /// @dev previousContextId is the latest active context, not `newContextId - 1`, because a destroyed switch
    ///      leaves a gap in the ids.
    function test_confirmKmsContextCreationPreviousContextIdSkipsDestroyedContext() public {
        _setupTwoNodeContext();
        uint256 destroyedContextId = _defineSameSetContextSwitch();
        vm.prank(owner);
        protocolConfig.destroyKmsContext(destroyedContextId);
        uint256 newContextId = _defineSameSetContextSwitch();

        bytes memory signature = _computeSignature(
            kmsPk0,
            _hashContextCreation(destroyedContextId, newContextId, nodeConfigHashes[newContextId], EMPTY_EXTRA_DATA)
        );
        vm.expectPartialRevert(IProtocolConfig.KmsContextCreationUnauthorized.selector);
        protocolConfig.confirmKmsContextCreation(newContextId, signature, EMPTY_EXTRA_DATA);

        signature = _computeSignature(
            kmsPk0,
            _hashContextCreation(
                KMS_CONTEXT_COUNTER_BASE + 1,
                newContextId,
                nodeConfigHashes[newContextId],
                EMPTY_EXTRA_DATA
            )
        );
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextCreationConfirmation(newContextId, vm.addr(kmsPk0), signature, EMPTY_EXTRA_DATA);
        protocolConfig.confirmKmsContextCreation(newContextId, signature, EMPTY_EXTRA_DATA);
    }

    // -----------------------------------------------------------------------
    // EIP-712 epoch activation confirmations
    // -----------------------------------------------------------------------

    /// @dev Activation needs every signer on one digest. The same material with different extraData
    ///      splits the confirmations.
    function test_confirmEpochActivationDivergentExtraDataDoesNotActivate() public {
        _setupTwoNodeContext();
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();

        _confirmSplitEpochActivationDoesNotActivate(keys, crsList, keys, crsList, hex"01");
    }

    /// @dev previousEpochId is the latest active epoch, not `epochId - 1`, because a destroyed Pending epoch
    ///      leaves a gap in the ids.
    function test_confirmEpochActivationPreviousEpochIdSkipsDestroyedEpoch() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(EPOCH_COUNTER_BASE + 2);
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 3;

        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();
        bytes32 epochMaterialHash = _hashEpochMaterial(keys, crsList);

        // Over `epochId - 1`, the EpochActivationConfirmation signature recovers an address outside the committee.
        bytes memory signature = _computeSignature(
            kmsPk0,
            _hashEpochActivation(contextId, EPOCH_COUNTER_BASE + 2, epochId, epochMaterialHash, EMPTY_EXTRA_DATA)
        );
        vm.expectPartialRevert(IProtocolConfig.EpochActivationUnauthorized.selector);
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, EMPTY_EXTRA_DATA);

        signature = _computeSignature(
            kmsPk0,
            _hashEpochActivation(contextId, EPOCH_COUNTER_BASE + 1, epochId, epochMaterialHash, EMPTY_EXTRA_DATA)
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, EMPTY_EXTRA_DATA);
        _confirmEpochActivationWithDummyMaterial(contextId, epochId, kmsPk1);

        (, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, epochId);
    }

    function test_revertConfirmEpochActivationAfterActive() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        _activateSameSetEpoch(epochId, 2);

        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();
        bytes memory signature = _signEpochActivation(contextId, epochId, kmsPk0, keys, crsList, EMPTY_EXTRA_DATA);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, epochId));
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, EMPTY_EXTRA_DATA);
    }

    /// @dev Both signers submit two key and two CRS results in the same order, so the epoch activates.
    function test_confirmEpochActivationTwoResultsSameOrderActivates() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildTwoEpochResults();
        _confirmEpochActivation(contextId, epochId, kmsPk0, keys, crsList, EMPTY_EXTRA_DATA);
        _confirmEpochActivation(contextId, epochId, kmsPk1, keys, crsList, EMPTY_EXTRA_DATA);
        (, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, epochId);
    }

    /// @dev The key order is part of epochMaterialHash, so swapped keys split the confirmations.
    function test_confirmEpochActivationSwappedKeysDoNotActivate() public {
        _setupTwoNodeContext();
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildTwoEpochResults();
        (IProtocolConfig.EpochKeyResult[] memory swappedKeys, ) = _buildTwoEpochResults();
        (swappedKeys[0], swappedKeys[1]) = (swappedKeys[1], swappedKeys[0]);

        _confirmSplitEpochActivationDoesNotActivate(keys, crsList, swappedKeys, crsList, EMPTY_EXTRA_DATA);
    }

    /// @dev The CRS order is part of epochMaterialHash, so swapped CRS results split the confirmations.
    function test_confirmEpochActivationSwappedCrsDoNotActivate() public {
        _setupTwoNodeContext();
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildTwoEpochResults();
        (, IProtocolConfig.EpochCrsResult[] memory swappedCrsList) = _buildTwoEpochResults();
        (swappedCrsList[0], swappedCrsList[1]) = (swappedCrsList[1], swappedCrsList[0]);

        _confirmSplitEpochActivationDoesNotActivate(keys, crsList, keys, swappedCrsList, EMPTY_EXTRA_DATA);
    }

    // -----------------------------------------------------------------------
    // Coprocessor context tests
    // -----------------------------------------------------------------------

    function _makeChainUpgradeWindows(uint256 count) internal pure returns (ChainUpgradeWindow[] memory windows) {
        windows = new ChainUpgradeWindow[](count);
        for (uint256 i = 0; i < count; i++) {
            windows[i] = ChainUpgradeWindow({
                chainId: uint64(i + 1),
                startBlock: uint64(100 + i),
                endBlock: uint64(200 + i)
            });
        }
    }

    function _defaultSoftwareVersion() internal pure returns (string memory) {
        return "v0.14.0";
    }

    function _defaultGwStartBlock() internal pure returns (uint64) {
        return 8421337;
    }

    function _defaultProposalId() internal pure returns (uint256) {
        return 1;
    }

    function test_proposeCoprocessorUpgrade_singleChain() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        uint64 gwStart = _defaultGwStartBlock();
        string memory version = _defaultSoftwareVersion();
        uint256 proposalId = _defaultProposalId();

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.CoprocessorUpgradeProposed(proposalId, version, windows, gwStart);
        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(proposalId, version, windows, gwStart);
    }

    function test_proposeCoprocessorUpgrade_multiChain() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(3);
        uint64 gwStart = _defaultGwStartBlock();
        string memory version = _defaultSoftwareVersion();
        uint256 proposalId = _defaultProposalId();

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.CoprocessorUpgradeProposed(proposalId, version, windows, gwStart);
        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(proposalId, version, windows, gwStart);
    }

    function test_revertCoprocessor_InvalidProposalId() public {
        _setupDefault();
        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.InvalidProposalId.selector);
        protocolConfig.proposeCoprocessorUpgrade(
            0,
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_EmptySoftwareVersion() public {
        _setupDefault();
        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.EmptySoftwareVersion.selector);
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            "",
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_EmptyChainUpgradeWindows() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = new ChainUpgradeWindow[](0);
        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.EmptyChainUpgradeWindows.selector);
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_ZeroChainId() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        windows[0].chainId = 0;
        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.ZeroChainId.selector);
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_DuplicateChainId() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(2);
        windows[1].chainId = windows[0].chainId;
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.DuplicateChainId.selector, windows[0].chainId));
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_InvalidBlockWindow() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        windows[0].startBlock = 500;
        windows[0].endBlock = 100; // start > end
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.InvalidBlockWindow.selector,
                windows[0].chainId,
                uint64(500),
                uint64(100)
            )
        );
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_ZeroGwStartBlock() public {
        _setupDefault();
        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.ZeroGwStartBlock.selector);
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            0
        );
    }

    function test_proposeCoprocessorUpgrade_sameIdTwice_bothEmit() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        uint256 proposalId = _defaultProposalId();

        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(
            proposalId,
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );

        // Uniqueness is the caller's responsibility — contract does not enforce.
        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.CoprocessorUpgradeProposed(
            proposalId,
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(
            proposalId,
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_proposeCoprocessorUpgrade_maxUint256() public {
        _setupDefault();
        uint256 proposalId = type(uint256).max;

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.CoprocessorUpgradeProposed(
            proposalId,
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(
            proposalId,
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }

    function test_proposeCoprocessorUpgrade_twoDistinctPending_bothEmit() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);

        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(1, _defaultSoftwareVersion(), windows, _defaultGwStartBlock());

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.CoprocessorUpgradeProposed(2, _defaultSoftwareVersion(), windows, _defaultGwStartBlock());
        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(2, _defaultSoftwareVersion(), windows, _defaultGwStartBlock());
    }

    function test_revertProposeCoprocessorUpgradeNotOwner() public {
        _setupDefault();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }

    // -----------------------------------------------------------------------
    // EIP-712 destruction confirmations
    // -----------------------------------------------------------------------

    /// @dev Only the active committee confirms, once per signer, from any sender. extraData is
    ///      digest material only, so signers may confirm different extraData.
    function test_confirmKmsContextDestruction() public {
        _setupTwoNodeContext();
        _defineContextSwitchToSigners(kmsPk2, kmsPk3);
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _confirmDisjointSetContextSwitchCreation(contextId);
        vm.prank(owner);
        protocolConfig.destroyKmsContext(contextId);

        // signer2 belongs to the destroyed committee only.
        bytes memory signature = _signContextDestruction(contextId, kmsPk2, EMPTY_EXTRA_DATA);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionUnauthorized.selector, vm.addr(kmsPk2), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, signature, EMPTY_EXTRA_DATA);

        bytes memory extraData = hex"01ab";
        signature = _signContextDestruction(contextId, kmsPk0, extraData);
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestructionConfirmation(contextId, vm.addr(kmsPk0), signature, extraData);
        vm.prank(address(0x999));
        protocolConfig.confirmKmsContextDestruction(contextId, signature, extraData);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionAlreadyConfirmed.selector, vm.addr(kmsPk0), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, signature, extraData);

        _confirmContextDestruction(contextId, kmsPk1, EMPTY_EXTRA_DATA);
    }

    /// @dev After a switch to {signer2, signer3}, the superseded genesis epoch is destroyed. Its own
    ///      committee no longer confirms, the active one does, once per signer.
    function test_confirmKmsEpochDestruction() public {
        _setupTwoNodeContext();
        _defineContextSwitchToSigners(kmsPk2, kmsPk3);
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _confirmDisjointSetContextSwitchCreation(newContextId);
        _confirmEpochActivationWithDummyMaterial(newContextId, EPOCH_COUNTER_BASE + 2, kmsPk2);
        _confirmEpochActivationWithDummyMaterial(newContextId, EPOCH_COUNTER_BASE + 2, kmsPk3);

        uint256 epochId = EPOCH_COUNTER_BASE + 1;
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(epochId);

        bytes memory signature = _signEpochDestruction(epochId, kmsPk0, EMPTY_EXTRA_DATA);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionUnauthorized.selector, vm.addr(kmsPk0), epochId)
        );
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, EMPTY_EXTRA_DATA);

        bytes memory extraData = hex"01ab";
        signature = _signEpochDestruction(epochId, kmsPk2, extraData);
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestructionConfirmation(epochId, vm.addr(kmsPk2), signature, extraData);
        vm.prank(address(0x999));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, extraData);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionAlreadyConfirmed.selector, vm.addr(kmsPk2), epochId)
        );
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, extraData);
    }

    /// @dev With n = 4 and t = 1 the quorum is 3. A confirmation of another extraData counts apart,
    ///      so the event fires on the third confirmation of one digest.
    function test_kmsEpochDestructionQuorumReachedOnOneDigest() public {
        _setupDefaultWithMpcThreshold(1);
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(epochId);
        bytes memory extraData = hex"01";

        vm.recordLogs();
        _confirmEpochDestruction(epochId, kmsPk0, extraData);
        _confirmEpochDestruction(epochId, kmsPk1, extraData);
        _confirmEpochDestruction(epochId, kmsPk2, EMPTY_EXTRA_DATA);
        assertEq(_countLogs(IProtocolConfig.KmsEpochDestructionQuorumReached.selector), 0);

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestructionQuorumReached(epochId, extraData);
        _confirmEpochDestruction(epochId, kmsPk3, extraData);
    }

    /// @dev Same quorum rule for a destroyed context. n = 4, t = 1, so the quorum is 3.
    function test_kmsContextDestructionQuorumReachedOnOneDigest() public {
        _setupDefaultWithMpcThreshold(1);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.destroyKmsContext(contextId);

        vm.recordLogs();
        _confirmContextDestruction(contextId, kmsPk0, EMPTY_EXTRA_DATA);
        _confirmContextDestruction(contextId, kmsPk1, EMPTY_EXTRA_DATA);
        assertEq(_countLogs(IProtocolConfig.KmsContextDestructionQuorumReached.selector), 0);

        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestructionQuorumReached(contextId, EMPTY_EXTRA_DATA);
        _confirmContextDestruction(contextId, kmsPk2, EMPTY_EXTRA_DATA);
    }

    /// @dev OZ ECDSA rejects the high-s twin of a valid destruction signature.
    function test_revertConfirmKmsDestructionMalleableSignature() public {
        _setupTwoNodeContext();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        (bytes memory signature, bytes32 highS) = _highSSignature(
            kmsPk0,
            _hashContextDestruction(contextId, EMPTY_EXTRA_DATA)
        );
        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureS.selector, highS));
        protocolConfig.confirmKmsContextDestruction(contextId, signature, EMPTY_EXTRA_DATA);

        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        (signature, highS) = _highSSignature(kmsPk0, _hashEpochDestruction(epochId, EMPTY_EXTRA_DATA));
        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureS.selector, highS));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, EMPTY_EXTRA_DATA);
    }
}
