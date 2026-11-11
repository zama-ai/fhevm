// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CoprocessorConfigTestUtils} from "../coprocessorConfig/coprocessorConfig.t.sol";
import {CoprocessorConfig} from "@fhevm-host-contracts/contracts/CoprocessorConfig.sol";
import {CoprocessorConfigReplica} from "@fhevm-host-contracts/contracts/CoprocessorConfigReplica.sol";
import {ICoprocessorConfigReplica} from "@fhevm-host-contracts/contracts/interfaces/ICoprocessorConfigReplica.sol";
import {ICoprocessorConfigBase} from "@fhevm-host-contracts/contracts/interfaces/ICoprocessorConfigBase.sol";
import {Coprocessor} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {COPROCESSOR_CONTEXT_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";

contract CoprocessorConfigReplicaTest is CoprocessorConfigTestUtils {
    CoprocessorConfigReplica internal replica;

    /// @dev The canonical coprocessor context a replica bootstrapped later starts from.
    uint256 internal constant CANONICAL_COPROCESSOR_CONTEXT_ID = COPROCESSOR_CONTEXT_COUNTER_BASE + 3;

    function _deployReplica(
        uint256 contextId,
        Coprocessor[] memory coprocessors,
        uint256 majorityThreshold
    ) internal returns (address proxy) {
        proxy = _deployEmptyCoprocessorConfigProxy();
        address impl = address(new CoprocessorConfigReplica());
        vm.prank(owner);
        EmptyUUPSProxy(proxy).upgradeToAndCall(
            impl,
            abi.encodeCall(
                CoprocessorConfigReplica.initializeFromCanonical,
                (contextId, coprocessors, majorityThreshold)
            )
        );
    }

    function _setupReplica() internal {
        _deployACL(owner);
        replica = CoprocessorConfigReplica(_deployReplica(CANONICAL_COPROCESSOR_CONTEXT_ID, _makeCoprocessors(3), 2));
    }

    function _mirrorCoprocessorContext(
        uint256 contextId,
        Coprocessor[] memory coprocessors,
        uint256 majorityThreshold
    ) internal {
        vm.prank(owner);
        replica.mirrorCoprocessorContext(contextId, coprocessors, majorityThreshold);
    }

    function _assertCoprocessorContext(
        uint256 contextId,
        Coprocessor[] memory expected,
        uint256 expectedMajorityThreshold
    ) internal view {
        Coprocessor[] memory stored = replica.getCoprocessorsForContext(contextId);
        assertEq(stored.length, expected.length);
        for (uint256 i = 0; i < expected.length; i++) {
            assertEq(stored[i].txSenderAddress, expected[i].txSenderAddress);
            assertEq(stored[i].signerAddress, expected[i].signerAddress);
            assertEq(stored[i].storageUrl, expected[i].storageUrl);
            assertEq(stored[i].prfEndpointUrl, expected[i].prfEndpointUrl);
            assertTrue(replica.isCoprocessorSignerForContext(contextId, expected[i].signerAddress));
            assertTrue(replica.isCoprocessorTxSenderForContext(contextId, expected[i].txSenderAddress));
        }
        assertEq(replica.getCoprocessorMajorityThresholdForContext(contextId), expectedMajorityThreshold);
    }

    // -----------------------------------------------------------------------
    // Initialization
    // -----------------------------------------------------------------------

    function test_initializeFromCanonicalPreservesCoprocessorContextId() public {
        Coprocessor[] memory coprocessors = _makeCoprocessors(3);
        _deployACL(owner);
        address proxy = _deployEmptyCoprocessorConfigProxy();
        address impl = address(new CoprocessorConfigReplica());

        vm.expectEmit(true, false, false, true, proxy);
        emit ICoprocessorConfigReplica.MirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID, coprocessors, 2);
        vm.prank(owner);
        EmptyUUPSProxy(proxy).upgradeToAndCall(
            impl,
            abi.encodeCall(
                CoprocessorConfigReplica.initializeFromCanonical,
                (CANONICAL_COPROCESSOR_CONTEXT_ID, coprocessors, 2)
            )
        );
        replica = CoprocessorConfigReplica(proxy);

        assertEq(replica.getCurrentCoprocessorContextId(), CANONICAL_COPROCESSOR_CONTEXT_ID);
        assertTrue(replica.isValidCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID));
        _assertCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID, coprocessors, 2);
        // Earlier canonical contexts were never mirrored.
        assertFalse(replica.isValidCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID - 1));
    }

    function test_revertInitializeFromCanonicalInvalidCoprocessorContextId() public {
        _deployACL(owner);
        address proxy = _deployEmptyCoprocessorConfigProxy();
        address impl = address(new CoprocessorConfigReplica());
        uint256 invalidId = COPROCESSOR_CONTEXT_COUNTER_BASE;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICoprocessorConfigBase.InvalidCoprocessorContext.selector, invalidId));
        EmptyUUPSProxy(proxy).upgradeToAndCall(
            impl,
            abi.encodeCall(CoprocessorConfigReplica.initializeFromCanonical, (invalidId, _makeCoprocessors(1), 1))
        );
    }

    function test_revertInitializeFromCanonicalInvalidCoprocessors() public {
        _deployACL(owner);
        address proxy = _deployEmptyCoprocessorConfigProxy();
        address impl = address(new CoprocessorConfigReplica());

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(ICoprocessorConfigBase.InvalidCoprocessorMajorityThreshold.selector, 1, 2)
        );
        EmptyUUPSProxy(proxy).upgradeToAndCall(
            impl,
            abi.encodeCall(
                CoprocessorConfigReplica.initializeFromCanonical,
                (FIRST_COPROCESSOR_CONTEXT_ID, _makeCoprocessors(2), 1)
            )
        );
    }

    function test_revertInitializeFromCanonicalTwice() public {
        _setupReplica();
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        replica.initializeFromCanonical(CANONICAL_COPROCESSOR_CONTEXT_ID + 1, _makeCoprocessors(1), 1);
    }

    function test_getVersion() public {
        _setupReplica();
        assertEq(replica.getVersion(), "CoprocessorConfigReplica v0.1.0");
    }

    // -----------------------------------------------------------------------
    // mirrorCoprocessorContext
    // -----------------------------------------------------------------------

    function test_mirrorCoprocessorContextAllowsGaps() public {
        _setupReplica();
        Coprocessor[] memory coprocessors = _makeCoprocessors(4);
        uint256 contextId = CANONICAL_COPROCESSOR_CONTEXT_ID + 2;

        vm.expectEmit(true, false, false, true, address(replica));
        emit ICoprocessorConfigReplica.MirrorCoprocessorContext(contextId, coprocessors, 3);
        _mirrorCoprocessorContext(contextId, coprocessors, 3);

        assertEq(replica.getCurrentCoprocessorContextId(), contextId);
        _assertCoprocessorContext(contextId, coprocessors, 3);
        assertTrue(replica.isValidCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID));
        assertFalse(replica.isValidCoprocessorContext(contextId - 1));
    }

    function test_revertMirrorCoprocessorContextNonIncreasing() public {
        _setupReplica();
        uint256[2] memory staleIds = [CANONICAL_COPROCESSOR_CONTEXT_ID, CANONICAL_COPROCESSOR_CONTEXT_ID - 1];
        for (uint256 i = 0; i < staleIds.length; i++) {
            vm.prank(owner);
            vm.expectRevert(
                abi.encodeWithSelector(
                    ICoprocessorConfigReplica.NonIncreasingCoprocessorContextId.selector,
                    staleIds[i],
                    CANONICAL_COPROCESSOR_CONTEXT_ID
                )
            );
            replica.mirrorCoprocessorContext(staleIds[i], _makeCoprocessors(1), 1);
        }
    }

    function test_revertMirrorCoprocessorContextInvalidSet() public {
        _setupReplica();
        Coprocessor[] memory coprocessors = _makeCoprocessors(2);
        coprocessors[1].signerAddress = coprocessors[0].signerAddress;

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                ICoprocessorConfigBase.CoprocessorSignerAlreadyRegistered.selector,
                coprocessors[0].signerAddress
            )
        );
        replica.mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 1, coprocessors, 2);
    }

    function test_revertMirrorCoprocessorContextNotOwner() public {
        _setupReplica();
        vm.prank(notOwner);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, notOwner));
        replica.mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 1, _makeCoprocessors(1), 1);
    }

    // -----------------------------------------------------------------------
    // mirrorCoprocessorContextDestruction
    // -----------------------------------------------------------------------

    function test_mirrorCoprocessorContextDestruction() public {
        _setupReplica();
        _mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 1, _makeCoprocessors(1), 1);

        vm.expectEmit(true, false, false, true, address(replica));
        emit ICoprocessorConfigBase.CoprocessorContextDestroyed(CANONICAL_COPROCESSOR_CONTEXT_ID);
        vm.prank(owner);
        replica.mirrorCoprocessorContextDestruction(CANONICAL_COPROCESSOR_CONTEXT_ID);

        assertFalse(replica.isValidCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID));
        _assertCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID, _makeCoprocessors(3), 2);
    }

    function test_revertMirrorCoprocessorContextDestructionCurrent() public {
        _setupReplica();
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                ICoprocessorConfigBase.CurrentCoprocessorContextCannotBeDestroyed.selector,
                CANONICAL_COPROCESSOR_CONTEXT_ID
            )
        );
        replica.mirrorCoprocessorContextDestruction(CANONICAL_COPROCESSOR_CONTEXT_ID);
    }

    function test_revertMirrorCoprocessorContextDestructionNeverMirrored() public {
        _setupReplica();
        _mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 2, _makeCoprocessors(1), 1);
        uint256 skippedId = CANONICAL_COPROCESSOR_CONTEXT_ID + 1;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICoprocessorConfigBase.InvalidCoprocessorContext.selector, skippedId));
        replica.mirrorCoprocessorContextDestruction(skippedId);
    }

    function test_revertMirrorCoprocessorContextDestructionTwice() public {
        _setupReplica();
        _mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 1, _makeCoprocessors(1), 1);
        vm.prank(owner);
        replica.mirrorCoprocessorContextDestruction(CANONICAL_COPROCESSOR_CONTEXT_ID);

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                ICoprocessorConfigBase.InvalidCoprocessorContext.selector,
                CANONICAL_COPROCESSOR_CONTEXT_ID
            )
        );
        replica.mirrorCoprocessorContextDestruction(CANONICAL_COPROCESSOR_CONTEXT_ID);
    }

    function test_revertMirrorCoprocessorContextDestructionNotOwner() public {
        _setupReplica();
        _mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 1, _makeCoprocessors(1), 1);
        vm.prank(notOwner);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, notOwner));
        replica.mirrorCoprocessorContextDestruction(CANONICAL_COPROCESSOR_CONTEXT_ID);
    }

    // -----------------------------------------------------------------------
    // Canonical and replica agree
    // -----------------------------------------------------------------------

    /// @dev A replica fed the canonical reads returns the same reads as the canonical contract.
    function test_replicaMatchesCanonicalReads() public {
        _deployACL(owner);
        address canonicalProxy = _deployEmptyCoprocessorConfigProxy();
        address canonicalImpl = address(new CoprocessorConfig());
        vm.prank(owner);
        EmptyUUPSProxy(canonicalProxy).upgradeToAndCall(
            canonicalImpl,
            abi.encodeCall(CoprocessorConfig.initializeFromEmptyProxy, (_makeCoprocessors(3), 2))
        );
        CoprocessorConfig canonical = CoprocessorConfig(canonicalProxy);
        vm.prank(owner);
        canonical.defineNewCoprocessorContext(_makeCoprocessors(4), 3);
        uint256 contextId = canonical.getCurrentCoprocessorContextId();

        replica = CoprocessorConfigReplica(
            _deployReplica(
                contextId,
                canonical.getCoprocessorsForContext(contextId),
                canonical.getCoprocessorMajorityThresholdForContext(contextId)
            )
        );
        assertEq(replica.getCurrentCoprocessorContextId(), contextId);
        _assertCoprocessorContext(contextId, _makeCoprocessors(4), 3);
    }
}
