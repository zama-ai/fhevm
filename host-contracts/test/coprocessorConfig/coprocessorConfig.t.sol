// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {HostContractsDeployerTestUtils, DeployableERC1967Proxy} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {CoprocessorConfig} from "@fhevm-host-contracts/contracts/CoprocessorConfig.sol";
import {ICoprocessorConfig} from "@fhevm-host-contracts/contracts/interfaces/ICoprocessorConfig.sol";
import {ICoprocessorConfigBase} from "@fhevm-host-contracts/contracts/interfaces/ICoprocessorConfigBase.sol";
import {ChainUpgradeWindow, Coprocessor} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {COPROCESSOR_CONTEXT_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";

/// @dev Shared helpers for the CoprocessorConfig and CoprocessorConfigReplica tests.
abstract contract CoprocessorConfigTestUtils is HostContractsDeployerTestUtils {
    address internal constant owner = address(456);
    address internal constant notOwner = address(0x999);
    uint256 internal constant FIRST_COPROCESSOR_CONTEXT_ID = COPROCESSOR_CONTEXT_COUNTER_BASE + 1;

    /// @dev `count` coprocessors with distinct tx senders, signers and storage and PRF endpoint URLs.
    function _makeCoprocessors(uint256 count) internal pure returns (Coprocessor[] memory coprocessors) {
        coprocessors = new Coprocessor[](count);
        for (uint256 i = 0; i < count; i++) {
            coprocessors[i] = Coprocessor({
                txSenderAddress: address(uint160(0xC1 + i)),
                signerAddress: vm.addr((i + 1) * 0x200),
                storageUrl: string.concat("https://coprocessor", vm.toString(i), ".example.com"),
                prfEndpointUrl: string.concat("https://prf", vm.toString(i), ".example.com")
            });
        }
    }

    /// @dev An empty UUPS proxy owned by the ACL owner, ready to be upgraded to a CoprocessorConfig implementation.
    function _deployEmptyCoprocessorConfigProxy() internal returns (address proxy) {
        address emptyProxyImplementation = address(new EmptyUUPSProxy());
        proxy = address(
            new DeployableERC1967Proxy(emptyProxyImplementation, abi.encodeCall(EmptyUUPSProxy.initialize, ()))
        );
    }
}

contract CoprocessorConfigTest is CoprocessorConfigTestUtils {
    CoprocessorConfig internal coprocessorConfig;

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.CoprocessorConfig")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant COPROCESSOR_CONFIG_STORAGE_LOCATION =
        0x0ecd3baf695d010fc2a14d3a2bf238b1c414d6e381b2e8efb81b7b2c7a9ef000;

    function _setupCoprocessors(Coprocessor[] memory coprocessors, uint256 majorityThreshold) internal {
        _deployACL(owner);
        address proxy = _deployEmptyCoprocessorConfigProxy();
        address impl = address(new CoprocessorConfig());
        vm.prank(owner);
        EmptyUUPSProxy(proxy).upgradeToAndCall(
            impl,
            abi.encodeCall(CoprocessorConfig.initializeFromEmptyProxy, (coprocessors, majorityThreshold))
        );
        coprocessorConfig = CoprocessorConfig(proxy);
    }

    function _setupDefaultCoprocessors() internal {
        _setupCoprocessors(_makeCoprocessors(3), 2);
    }

    function _defineCoprocessorContext(Coprocessor[] memory coprocessors, uint256 majorityThreshold) internal {
        vm.prank(owner);
        coprocessorConfig.defineNewCoprocessorContext(coprocessors, majorityThreshold);
    }

    function _expectDefineCoprocessorContextRevert(
        Coprocessor[] memory coprocessors,
        uint256 majorityThreshold,
        bytes memory expectedRevert
    ) internal {
        vm.prank(owner);
        vm.expectRevert(expectedRevert);
        coprocessorConfig.defineNewCoprocessorContext(coprocessors, majorityThreshold);
    }

    function _assertCoprocessorContext(
        uint256 contextId,
        Coprocessor[] memory expected,
        uint256 expectedMajorityThreshold
    ) internal view {
        Coprocessor[] memory stored = coprocessorConfig.getCoprocessorsForContext(contextId);
        assertEq(stored.length, expected.length);
        for (uint256 i = 0; i < expected.length; i++) {
            assertEq(stored[i].txSenderAddress, expected[i].txSenderAddress);
            assertEq(stored[i].signerAddress, expected[i].signerAddress);
            assertEq(stored[i].storageUrl, expected[i].storageUrl);
            assertEq(stored[i].prfEndpointUrl, expected[i].prfEndpointUrl);
            assertTrue(coprocessorConfig.isCoprocessorSignerForContext(contextId, expected[i].signerAddress));
            assertTrue(coprocessorConfig.isCoprocessorTxSenderForContext(contextId, expected[i].txSenderAddress));
        }
        assertEq(coprocessorConfig.getCoprocessorMajorityThresholdForContext(contextId), expectedMajorityThreshold);
    }

    // -----------------------------------------------------------------------
    // Initialization
    // -----------------------------------------------------------------------

    function test_initializeFromEmptyProxyDefinesFirstContext() public {
        Coprocessor[] memory coprocessors = _makeCoprocessors(3);
        _deployACL(owner);
        address proxy = _deployEmptyCoprocessorConfigProxy();
        address impl = address(new CoprocessorConfig());

        vm.expectEmit(true, true, false, true, proxy);
        emit ICoprocessorConfig.NewCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID, 0, coprocessors, 2);

        vm.prank(owner);
        EmptyUUPSProxy(proxy).upgradeToAndCall(
            impl,
            abi.encodeCall(CoprocessorConfig.initializeFromEmptyProxy, (coprocessors, 2))
        );
        coprocessorConfig = CoprocessorConfig(proxy);

        assertEq(coprocessorConfig.getCurrentCoprocessorContextId(), FIRST_COPROCESSOR_CONTEXT_ID);
        assertTrue(coprocessorConfig.isValidCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID));
        _assertCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID, coprocessors, 2);
    }

    function test_revertInitializeFromEmptyProxyInvalidCoprocessors() public {
        _deployACL(owner);
        address proxy = _deployEmptyCoprocessorConfigProxy();
        address impl = address(new CoprocessorConfig());

        vm.prank(owner);
        vm.expectRevert(ICoprocessorConfigBase.EmptyCoprocessors.selector);
        EmptyUUPSProxy(proxy).upgradeToAndCall(
            impl,
            abi.encodeCall(CoprocessorConfig.initializeFromEmptyProxy, (new Coprocessor[](0), 1))
        );
    }

    function test_revertInitializeFromEmptyProxyTwice() public {
        _setupDefaultCoprocessors();
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        coprocessorConfig.initializeFromEmptyProxy(_makeCoprocessors(1), 1);
    }

    function test_revertInitializeImplementationDirectly() public {
        CoprocessorConfig impl = new CoprocessorConfig();
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        impl.initializeFromEmptyProxy(_makeCoprocessors(1), 1);
    }

    function test_coprocessorStorageLocationPinned() public {
        _setupDefaultCoprocessors();
        uint256 stored = uint256(vm.load(address(coprocessorConfig), COPROCESSOR_CONFIG_STORAGE_LOCATION));
        assertEq(stored, coprocessorConfig.getCurrentCoprocessorContextId());
        assertEq(stored, FIRST_COPROCESSOR_CONTEXT_ID);
    }

    function test_getVersion() public {
        _setupDefaultCoprocessors();
        assertEq(coprocessorConfig.getVersion(), "CoprocessorConfig v0.1.0");
    }

    // -----------------------------------------------------------------------
    // Coprocessor upgrade proposals
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
        _setupDefaultCoprocessors();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        uint64 gwStart = _defaultGwStartBlock();
        string memory version = _defaultSoftwareVersion();
        uint256 proposalId = _defaultProposalId();

        vm.expectEmit(true, false, false, true, address(coprocessorConfig));
        emit ICoprocessorConfig.CoprocessorUpgradeProposed(proposalId, version, windows, gwStart);
        vm.prank(owner);
        coprocessorConfig.proposeCoprocessorUpgrade(proposalId, version, windows, gwStart);
    }

    function test_proposeCoprocessorUpgrade_multiChain() public {
        _setupDefaultCoprocessors();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(3);
        uint64 gwStart = _defaultGwStartBlock();
        string memory version = _defaultSoftwareVersion();
        uint256 proposalId = _defaultProposalId();

        vm.expectEmit(true, false, false, true, address(coprocessorConfig));
        emit ICoprocessorConfig.CoprocessorUpgradeProposed(proposalId, version, windows, gwStart);
        vm.prank(owner);
        coprocessorConfig.proposeCoprocessorUpgrade(proposalId, version, windows, gwStart);
    }

    function test_revertCoprocessor_InvalidProposalId() public {
        _setupDefaultCoprocessors();
        vm.prank(owner);
        vm.expectRevert(ICoprocessorConfig.InvalidProposalId.selector);
        coprocessorConfig.proposeCoprocessorUpgrade(
            0,
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_EmptySoftwareVersion() public {
        _setupDefaultCoprocessors();
        vm.prank(owner);
        vm.expectRevert(ICoprocessorConfig.EmptySoftwareVersion.selector);
        coprocessorConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            "",
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_EmptyChainUpgradeWindows() public {
        _setupDefaultCoprocessors();
        ChainUpgradeWindow[] memory windows = new ChainUpgradeWindow[](0);
        vm.prank(owner);
        vm.expectRevert(ICoprocessorConfig.EmptyChainUpgradeWindows.selector);
        coprocessorConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_ZeroChainId() public {
        _setupDefaultCoprocessors();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        windows[0].chainId = 0;
        vm.prank(owner);
        vm.expectRevert(ICoprocessorConfig.ZeroChainId.selector);
        coprocessorConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_DuplicateChainId() public {
        _setupDefaultCoprocessors();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(2);
        windows[1].chainId = windows[0].chainId;
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICoprocessorConfig.DuplicateChainId.selector, windows[0].chainId));
        coprocessorConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_InvalidBlockWindow() public {
        _setupDefaultCoprocessors();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        windows[0].startBlock = 500;
        windows[0].endBlock = 100; // start > end
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                ICoprocessorConfig.InvalidBlockWindow.selector,
                windows[0].chainId,
                uint64(500),
                uint64(100)
            )
        );
        coprocessorConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_ZeroGwStartBlock() public {
        _setupDefaultCoprocessors();
        vm.prank(owner);
        vm.expectRevert(ICoprocessorConfig.ZeroGwStartBlock.selector);
        coprocessorConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            0
        );
    }

    function test_proposeCoprocessorUpgrade_sameIdTwice_bothEmit() public {
        _setupDefaultCoprocessors();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        uint256 proposalId = _defaultProposalId();

        vm.prank(owner);
        coprocessorConfig.proposeCoprocessorUpgrade(
            proposalId,
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );

        // Uniqueness is the caller's responsibility — contract does not enforce.
        vm.expectEmit(true, false, false, true, address(coprocessorConfig));
        emit ICoprocessorConfig.CoprocessorUpgradeProposed(
            proposalId,
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
        vm.prank(owner);
        coprocessorConfig.proposeCoprocessorUpgrade(
            proposalId,
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_proposeCoprocessorUpgrade_maxUint256() public {
        _setupDefaultCoprocessors();
        uint256 proposalId = type(uint256).max;

        vm.expectEmit(true, false, false, true, address(coprocessorConfig));
        emit ICoprocessorConfig.CoprocessorUpgradeProposed(
            proposalId,
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
        vm.prank(owner);
        coprocessorConfig.proposeCoprocessorUpgrade(
            proposalId,
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }

    function test_proposeCoprocessorUpgrade_twoDistinctPending_bothEmit() public {
        _setupDefaultCoprocessors();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);

        vm.prank(owner);
        coprocessorConfig.proposeCoprocessorUpgrade(1, _defaultSoftwareVersion(), windows, _defaultGwStartBlock());

        vm.expectEmit(true, false, false, true, address(coprocessorConfig));
        emit ICoprocessorConfig.CoprocessorUpgradeProposed(
            2,
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
        vm.prank(owner);
        coprocessorConfig.proposeCoprocessorUpgrade(2, _defaultSoftwareVersion(), windows, _defaultGwStartBlock());
    }

    function test_revertProposeCoprocessorUpgradeNotOwner() public {
        _setupDefaultCoprocessors();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        coprocessorConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }

    // -----------------------------------------------------------------------
    // Coprocessor contexts: defineNewCoprocessorContext
    // -----------------------------------------------------------------------

    function test_defineNewCoprocessorContext() public {
        _setupDefaultCoprocessors();
        Coprocessor[] memory coprocessors = _makeCoprocessors(4);

        vm.expectEmit(true, true, false, true, address(coprocessorConfig));
        emit ICoprocessorConfig.NewCoprocessorContext(
            FIRST_COPROCESSOR_CONTEXT_ID + 1,
            FIRST_COPROCESSOR_CONTEXT_ID,
            coprocessors,
            3
        );
        _defineCoprocessorContext(coprocessors, 3);

        assertEq(coprocessorConfig.getCurrentCoprocessorContextId(), FIRST_COPROCESSOR_CONTEXT_ID + 1);
        _assertCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID + 1, coprocessors, 3);
        // The previous context stays valid and unchanged.
        assertTrue(coprocessorConfig.isValidCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID));
        _assertCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID, _makeCoprocessors(3), 2);
        // Membership is per context: the fourth coprocessor only belongs to the new one.
        assertFalse(
            coprocessorConfig.isCoprocessorSignerForContext(FIRST_COPROCESSOR_CONTEXT_ID, coprocessors[3].signerAddress)
        );
        assertFalse(
            coprocessorConfig.isCoprocessorTxSenderForContext(
                FIRST_COPROCESSOR_CONTEXT_ID,
                coprocessors[3].txSenderAddress
            )
        );
    }

    function test_defineNewCoprocessorContextReusesAddresses() public {
        _setupDefaultCoprocessors();
        _defineCoprocessorContext(_makeCoprocessors(3), 3);
        _assertCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID + 1, _makeCoprocessors(3), 3);
    }

    function test_defineNewCoprocessorContextSameTxSenderAndSigner() public {
        _setupDefaultCoprocessors();
        Coprocessor[] memory coprocessors = _makeCoprocessors(1);
        coprocessors[0].txSenderAddress = coprocessors[0].signerAddress;
        _defineCoprocessorContext(coprocessors, 1);
        _assertCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID + 1, coprocessors, 1);
    }

    function test_defineNewCoprocessorContextAcceptsEmptyPrfEndpointUrl() public {
        _setupDefaultCoprocessors();
        Coprocessor[] memory coprocessors = _makeCoprocessors(1);
        coprocessors[0].prfEndpointUrl = "";
        _defineCoprocessorContext(coprocessors, 1);
        _assertCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID + 1, coprocessors, 1);
    }

    function test_defineNewCoprocessorContextMaxSize() public {
        _setupDefaultCoprocessors();
        _defineCoprocessorContext(_makeCoprocessors(255), 128);
        assertEq(coprocessorConfig.getCoprocessorsForContext(FIRST_COPROCESSOR_CONTEXT_ID + 1).length, 255);
    }

    function test_revertDefineNewCoprocessorContextNotOwner() public {
        _setupDefaultCoprocessors();
        vm.prank(notOwner);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, notOwner));
        coprocessorConfig.defineNewCoprocessorContext(_makeCoprocessors(1), 1);
    }

    function test_revertDefineNewCoprocessorContextEmpty() public {
        _setupDefaultCoprocessors();
        _expectDefineCoprocessorContextRevert(
            new Coprocessor[](0),
            1,
            abi.encodeWithSelector(ICoprocessorConfigBase.EmptyCoprocessors.selector)
        );
    }

    function test_revertDefineNewCoprocessorContextExceedsProofFormatLimit() public {
        _setupDefaultCoprocessors();
        _expectDefineCoprocessorContextRevert(
            _makeCoprocessors(256),
            129,
            abi.encodeWithSelector(ICoprocessorConfigBase.CoprocessorSetExceedsProofFormatLimit.selector, 256, 255)
        );
    }

    function test_revertDefineNewCoprocessorContextNullTxSender() public {
        _setupDefaultCoprocessors();
        Coprocessor[] memory coprocessors = _makeCoprocessors(2);
        coprocessors[1].txSenderAddress = address(0);
        _expectDefineCoprocessorContextRevert(
            coprocessors,
            2,
            abi.encodeWithSelector(ICoprocessorConfigBase.CoprocessorNullTxSender.selector)
        );
    }

    function test_revertDefineNewCoprocessorContextNullSigner() public {
        _setupDefaultCoprocessors();
        Coprocessor[] memory coprocessors = _makeCoprocessors(2);
        coprocessors[1].signerAddress = address(0);
        _expectDefineCoprocessorContextRevert(
            coprocessors,
            2,
            abi.encodeWithSelector(ICoprocessorConfigBase.CoprocessorNullSigner.selector)
        );
    }

    function test_revertDefineNewCoprocessorContextEmptyStorageUrl() public {
        _setupDefaultCoprocessors();
        Coprocessor[] memory coprocessors = _makeCoprocessors(2);
        coprocessors[1].storageUrl = "";
        _expectDefineCoprocessorContextRevert(
            coprocessors,
            2,
            abi.encodeWithSelector(ICoprocessorConfigBase.CoprocessorEmptyStorageUrl.selector)
        );
    }

    function test_revertDefineNewCoprocessorContextDuplicateTxSender() public {
        _setupDefaultCoprocessors();
        Coprocessor[] memory coprocessors = _makeCoprocessors(3);
        coprocessors[2].txSenderAddress = coprocessors[0].txSenderAddress;
        _expectDefineCoprocessorContextRevert(
            coprocessors,
            2,
            abi.encodeWithSelector(
                ICoprocessorConfigBase.CoprocessorTxSenderAlreadyRegistered.selector,
                coprocessors[0].txSenderAddress
            )
        );
    }

    function test_revertDefineNewCoprocessorContextDuplicateSigner() public {
        _setupDefaultCoprocessors();
        Coprocessor[] memory coprocessors = _makeCoprocessors(3);
        coprocessors[2].signerAddress = coprocessors[0].signerAddress;
        _expectDefineCoprocessorContextRevert(
            coprocessors,
            2,
            abi.encodeWithSelector(
                ICoprocessorConfigBase.CoprocessorSignerAlreadyRegistered.selector,
                coprocessors[0].signerAddress
            )
        );
    }

    function test_revertDefineNewCoprocessorContextThresholdNotMajority() public {
        _setupDefaultCoprocessors();
        _expectDefineCoprocessorContextRevert(
            _makeCoprocessors(4),
            2,
            abi.encodeWithSelector(ICoprocessorConfigBase.InvalidCoprocessorMajorityThreshold.selector, 2, 4)
        );
    }

    function test_revertDefineNewCoprocessorContextNullThreshold() public {
        _setupDefaultCoprocessors();
        _expectDefineCoprocessorContextRevert(
            _makeCoprocessors(1),
            0,
            abi.encodeWithSelector(ICoprocessorConfigBase.InvalidCoprocessorMajorityThreshold.selector, 0, 1)
        );
    }

    function test_revertDefineNewCoprocessorContextThresholdAboveCount() public {
        _setupDefaultCoprocessors();
        _expectDefineCoprocessorContextRevert(
            _makeCoprocessors(3),
            4,
            abi.encodeWithSelector(ICoprocessorConfigBase.InvalidCoprocessorMajorityThreshold.selector, 4, 3)
        );
    }

    function test_revertDefineNewCoprocessorContextMaxThreshold() public {
        _setupDefaultCoprocessors();
        uint256 threshold = type(uint256).max;
        _expectDefineCoprocessorContextRevert(
            _makeCoprocessors(3),
            threshold,
            abi.encodeWithSelector(ICoprocessorConfigBase.InvalidCoprocessorMajorityThreshold.selector, threshold, 3)
        );
    }

    /// @dev A threshold is accepted exactly when it is more than half the set size and at most that size.
    function testFuzz_majorityThresholdBound(uint8 rawCount, uint256 threshold) public {
        uint256 count = bound(rawCount, 1, 20);
        threshold = bound(threshold, 0, 25);
        _setupDefaultCoprocessors();
        Coprocessor[] memory coprocessors = _makeCoprocessors(count);
        if (threshold * 2 > count && threshold <= count) {
            _defineCoprocessorContext(coprocessors, threshold);
            assertEq(
                coprocessorConfig.getCoprocessorMajorityThresholdForContext(FIRST_COPROCESSOR_CONTEXT_ID + 1),
                threshold
            );
        } else {
            _expectDefineCoprocessorContextRevert(
                coprocessors,
                threshold,
                abi.encodeWithSelector(
                    ICoprocessorConfigBase.InvalidCoprocessorMajorityThreshold.selector,
                    threshold,
                    count
                )
            );
        }
    }

    // -----------------------------------------------------------------------
    // Coprocessor contexts: destroyCoprocessorContext
    // -----------------------------------------------------------------------

    function test_destroyCoprocessorContextKeepsDataReadable() public {
        _setupDefaultCoprocessors();
        _defineCoprocessorContext(_makeCoprocessors(1), 1);

        vm.expectEmit(true, false, false, true, address(coprocessorConfig));
        emit ICoprocessorConfigBase.CoprocessorContextDestroyed(FIRST_COPROCESSOR_CONTEXT_ID);
        vm.prank(owner);
        coprocessorConfig.destroyCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID);

        assertFalse(coprocessorConfig.isValidCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID));
        assertTrue(coprocessorConfig.isValidCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID + 1));
        assertEq(coprocessorConfig.getCurrentCoprocessorContextId(), FIRST_COPROCESSOR_CONTEXT_ID + 1);
        _assertCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID, _makeCoprocessors(3), 2);
    }

    function test_revertDestroyCurrentCoprocessorContext() public {
        _setupDefaultCoprocessors();
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                ICoprocessorConfigBase.CurrentCoprocessorContextCannotBeDestroyed.selector,
                FIRST_COPROCESSOR_CONTEXT_ID
            )
        );
        coprocessorConfig.destroyCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID);
    }

    function test_revertDestroyUnknownCoprocessorContext() public {
        _setupDefaultCoprocessors();
        uint256[3] memory unknownIds = [uint256(0), COPROCESSOR_CONTEXT_COUNTER_BASE, FIRST_COPROCESSOR_CONTEXT_ID + 1];
        for (uint256 i = 0; i < unknownIds.length; i++) {
            vm.prank(owner);
            vm.expectRevert(
                abi.encodeWithSelector(ICoprocessorConfigBase.InvalidCoprocessorContext.selector, unknownIds[i])
            );
            coprocessorConfig.destroyCoprocessorContext(unknownIds[i]);
        }
    }

    function test_revertDestroyCoprocessorContextTwice() public {
        _setupDefaultCoprocessors();
        _defineCoprocessorContext(_makeCoprocessors(1), 1);
        vm.prank(owner);
        coprocessorConfig.destroyCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID);

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                ICoprocessorConfigBase.InvalidCoprocessorContext.selector,
                FIRST_COPROCESSOR_CONTEXT_ID
            )
        );
        coprocessorConfig.destroyCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID);
    }

    function test_revertDestroyCoprocessorContextNotOwner() public {
        _setupDefaultCoprocessors();
        _defineCoprocessorContext(_makeCoprocessors(1), 1);
        vm.prank(notOwner);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, notOwner));
        coprocessorConfig.destroyCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID);
    }

    // -----------------------------------------------------------------------
    // Coprocessor contexts: Reads of unknown contexts
    // -----------------------------------------------------------------------

    function test_readsOfUnknownCoprocessorContext() public {
        _setupDefaultCoprocessors();
        uint256[3] memory unknownIds = [uint256(0), COPROCESSOR_CONTEXT_COUNTER_BASE, FIRST_COPROCESSOR_CONTEXT_ID + 1];
        address signer = _makeCoprocessors(1)[0].signerAddress;
        address txSender = _makeCoprocessors(1)[0].txSenderAddress;
        for (uint256 i = 0; i < unknownIds.length; i++) {
            uint256 id = unknownIds[i];
            assertFalse(coprocessorConfig.isValidCoprocessorContext(id));
            assertFalse(coprocessorConfig.isCoprocessorSignerForContext(id, signer));
            assertFalse(coprocessorConfig.isCoprocessorTxSenderForContext(id, txSender));

            vm.expectRevert(abi.encodeWithSelector(ICoprocessorConfigBase.InvalidCoprocessorContext.selector, id));
            coprocessorConfig.getCoprocessorsForContext(id);
            vm.expectRevert(abi.encodeWithSelector(ICoprocessorConfigBase.InvalidCoprocessorContext.selector, id));
            coprocessorConfig.getCoprocessorMajorityThresholdForContext(id);
        }
    }
}
