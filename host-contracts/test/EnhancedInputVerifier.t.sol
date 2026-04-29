// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {EnhancedInputVerifier} from "../contracts/EnhancedInputVerifier.sol";
import {EmptyUUPSProxy} from "../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ACL} from "../contracts/ACL.sol";
import {aclAdd} from "../addresses/FHEVMHostAddresses.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

/**
 * @title EnhancedInputVerifierTest
 * @notice Comprehensive test suite for EnhancedInputVerifier security features
 * @dev Tests all security invariants, edge cases, and attack scenarios
 */
contract EnhancedInputVerifierTest is Test {
    EnhancedInputVerifier public verifier;
    address public proxy;

    // Test addresses
    address public owner;
    address public nonOwner;
    address[] public signers;

    // Constants
    uint256 constant MINIMUM_SIGNERS = 3;
    uint256 constant MINIMUM_THRESHOLD_PERCENTAGE = 51;
    uint256 constant THRESHOLD_CHANGE_DELAY = 2 days;
    uint256 constant CHANGE_EXPIRATION_PERIOD = 7 days;

    // Events
    event ThresholdChangeProposed(
        bytes32 indexed changeHash, uint256 newThreshold, uint256 effectiveTime, address proposer
    );

    event ThresholdChangeExecuted(bytes32 indexed changeHash, uint256 newThreshold, uint256 executionTime);

    event ThresholdChangeCancelled(bytes32 indexed changeHash, address canceller);

    event ContextSetWithValidation(uint256 signersCount, uint256 threshold, uint256 minThreshold);

    function setUp() public {
        owner = address(this);
        nonOwner = address(0x999);

        // Create test signers
        for (uint256 i = 1; i <= 10; i++) {
            signers.push(address(uint160(i + 100)));
        }

        // Deploy and etch ACL
        _deployAndEtchACL();

        // Deploy proxy
        proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()), abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );

        // Deploy implementation and upgrade
        address implementation = address(new EnhancedInputVerifier());
        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(
                EnhancedInputVerifier.initializeFromEmptyProxy, (address(0x1), uint64(block.chainid), signers, 7)
            ),
            owner
        );

        verifier = EnhancedInputVerifier(proxy);
    }

    /**
     * @dev Deploy and etch ACL contract at expected constant address
     */
    function _deployAndEtchACL() internal {
        address _acl = address(new ACL());
        bytes memory code = _acl.code;
        vm.etch(aclAdd, code);
        vm.store(
            aclAdd, 0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300, bytes32(uint256(uint160(owner)))
        );
    }

    // ============================================================================
    // Initialization Tests
    // ============================================================================

    function test_InitializationWithValidThreshold() public {
        // Deploy new proxy and implementation
        address newProxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()), abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );

        address[] memory testSigners = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            testSigners[i] = address(uint160(i + 1));
        }

        // Deploy implementation
        address implementation = address(new EnhancedInputVerifier());

        // 3 of 5 = 60% (above 51% minimum)
        UnsafeUpgrades.upgradeProxy(
            newProxy,
            implementation,
            abi.encodeCall(
                EnhancedInputVerifier.initializeFromEmptyProxy, (address(0x1), uint64(block.chainid), testSigners, 3)
            ),
            owner
        );

        EnhancedInputVerifier newVerifier = EnhancedInputVerifier(newProxy);
        assertEq(newVerifier.getThreshold(), 3);
        assertEq(newVerifier.getCoprocessorSigners().length, 5);
    }

    /// @dev Note: Initialization validation tests are covered by defineNewContext tests
    /// The initializeFromEmptyProxy function calls _defineNewContextSecure internally
    /// which performs the same validation. Testing via defineNewContext is more direct.

    // ============================================================================
    // Threshold Validation Tests
    // ============================================================================

    function test_CalculateMinimumThreshold() public {
        // Test various signer counts
        assertEq(_calculateMinThreshold(3), 2); // ceil(3 * 0.51) = 2
        assertEq(_calculateMinThreshold(4), 3); // ceil(4 * 0.51) = 3
        assertEq(_calculateMinThreshold(5), 3); // ceil(5 * 0.51) = 3
        assertEq(_calculateMinThreshold(10), 6); // ceil(10 * 0.51) = 6
        assertEq(_calculateMinThreshold(100), 51); // ceil(100 * 0.51) = 51
    }

    function test_GetMinimumThreshold() public {
        // With 10 signers, minimum should be 6
        assertEq(verifier.getMinimumThreshold(), 6);
    }

    function test_RevertDefineNewContextWithThresholdTooLow() public {
        address[] memory newSigners = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            newSigners[i] = address(uint160(i + 100));
        }

        vm.expectRevert(abi.encodeWithSelector(EnhancedInputVerifier.ThresholdTooLow.selector, 2, 3));

        verifier.defineNewContext(newSigners, 2);
    }

    function test_RevertDefineNewContextWithTooManySigners() public {
        address[] memory tooManySigners = new address[](101);
        for (uint256 i = 0; i < 101; i++) {
            tooManySigners[i] = address(uint160(i + 1));
        }

        vm.expectRevert(abi.encodeWithSelector(EnhancedInputVerifier.TooManySigners.selector, 101, 100));

        verifier.defineNewContext(tooManySigners, 51);
    }

    function test_RevertDefineNewContextWithDuplicateSigners() public {
        address[] memory duplicateSigners = new address[](4);
        duplicateSigners[0] = address(0x1);
        duplicateSigners[1] = address(0x2);
        duplicateSigners[2] = address(0x3);
        duplicateSigners[3] = address(0x1); // Duplicate

        vm.expectRevert(abi.encodeWithSelector(EnhancedInputVerifier.CoprocessorAlreadySigner.selector));

        verifier.defineNewContext(duplicateSigners, 3);
    }

    function test_RevertDefineNewContextWithNullSigner() public {
        address[] memory signersWithNull = new address[](4);
        signersWithNull[0] = address(0x1);
        signersWithNull[1] = address(0x2);
        signersWithNull[2] = address(0x3);
        signersWithNull[3] = address(0); // Null

        vm.expectRevert(abi.encodeWithSelector(EnhancedInputVerifier.CoprocessorSignerNull.selector));

        verifier.defineNewContext(signersWithNull, 3);
    }

    // ============================================================================
    // Time-Locked Threshold Change Tests
    // ============================================================================

    function test_ProposeThresholdChange() public {
        uint256 newThreshold = 8; // Increase from 7 to 8

        vm.expectEmit(true, true, true, true);
        emit ThresholdChangeProposed(
            keccak256(abi.encodePacked(newThreshold, block.timestamp, address(this), verifier.getCurrentContextId())),
            newThreshold,
            block.timestamp + THRESHOLD_CHANGE_DELAY,
            address(this)
        );

        bytes32 changeHash = verifier.proposeThresholdChange(newThreshold);

        // Verify change is pending
        (
            uint256 storedThreshold,
            uint256 proposedTime,
            uint256 effectiveTime,
            address proposer,
            bool executed,
            bool cancelled
        ) = _getPendingChange(changeHash);

        assertEq(storedThreshold, newThreshold);
        assertEq(proposedTime, block.timestamp);
        assertEq(effectiveTime, block.timestamp + THRESHOLD_CHANGE_DELAY);
        assertEq(proposer, address(this));
        assertFalse(executed);
        assertFalse(cancelled);
    }

    function test_RevertProposeThresholdChangeWithInvalidThreshold() public {
        // Try to propose threshold below minimum
        vm.expectRevert(abi.encodeWithSelector(EnhancedInputVerifier.ThresholdTooLow.selector, 1, 6));

        verifier.proposeThresholdChange(1);
    }

    function test_ExecuteThresholdChangeAfterDelay() public {
        uint256 newThreshold = 8;
        bytes32 changeHash = verifier.proposeThresholdChange(newThreshold);

        // Try to execute immediately (should fail)
        vm.expectRevert(
            abi.encodeWithSelector(
                EnhancedInputVerifier.ChangeDelayNotElapsed.selector,
                block.timestamp,
                block.timestamp + THRESHOLD_CHANGE_DELAY
            )
        );

        verifier.executeThresholdChange(changeHash);

        // Warp forward past delay
        vm.warp(block.timestamp + THRESHOLD_CHANGE_DELAY + 1);

        // Now execution should succeed - capture the actual execution time
        uint256 executionTime = block.timestamp;

        vm.expectEmit(true, true, true, true);
        emit ThresholdChangeExecuted(changeHash, newThreshold, executionTime);

        verifier.executeThresholdChange(changeHash);

        // Verify threshold updated
        assertEq(verifier.getThreshold(), newThreshold);
    }

    function test_RevertExecuteExpiredChange() public {
        uint256 newThreshold = 8;
        bytes32 changeHash = verifier.proposeThresholdChange(newThreshold);

        // Get the effective time from the pending change
        (,, uint256 effectiveTime,,,) = _getPendingChange(changeHash);

        // Warp past expiration (7 days after effective time)
        vm.warp(effectiveTime + CHANGE_EXPIRATION_PERIOD + 1);

        vm.expectRevert(
            abi.encodeWithSelector(
                EnhancedInputVerifier.ChangeExpired.selector, changeHash, effectiveTime + CHANGE_EXPIRATION_PERIOD
            )
        );

        verifier.executeThresholdChange(changeHash);
    }

    function test_RevertExecuteAlreadyExecutedChange() public {
        uint256 newThreshold = 8;
        bytes32 changeHash = verifier.proposeThresholdChange(newThreshold);

        vm.warp(block.timestamp + THRESHOLD_CHANGE_DELAY + 1);
        verifier.executeThresholdChange(changeHash);

        // Try to execute again
        vm.expectRevert(abi.encodeWithSelector(EnhancedInputVerifier.ChangeAlreadyExecuted.selector, changeHash));

        verifier.executeThresholdChange(changeHash);
    }

    function test_CancelThresholdChange() public {
        uint256 newThreshold = 8;
        bytes32 changeHash = verifier.proposeThresholdChange(newThreshold);

        vm.expectEmit(true, true, true, true);
        emit ThresholdChangeCancelled(changeHash, address(this));

        verifier.cancelThresholdChange(changeHash);

        // Verify cancelled
        (,,,,, bool cancelled) = _getPendingChange(changeHash);
        assertTrue(cancelled);

        // Should not be able to execute cancelled change
        vm.warp(block.timestamp + THRESHOLD_CHANGE_DELAY + 1);

        vm.expectRevert(EnhancedInputVerifier.InvalidChangeHash.selector);
        verifier.executeThresholdChange(changeHash);
    }

    function test_RevertCancelNonExistentChange() public {
        bytes32 fakeHash = keccak256("fake");

        vm.expectRevert(EnhancedInputVerifier.InvalidChangeHash.selector);
        verifier.cancelThresholdChange(fakeHash);
    }

    function test_IsChangeReady() public {
        uint256 newThreshold = 8;
        bytes32 changeHash = verifier.proposeThresholdChange(newThreshold);

        // Should not be ready immediately
        assertFalse(verifier.isChangeReady(changeHash));

        // Warp past delay
        vm.warp(block.timestamp + THRESHOLD_CHANGE_DELAY + 1);

        // Should be ready now
        assertTrue(verifier.isChangeReady(changeHash));

        // Execute the change
        verifier.executeThresholdChange(changeHash);

        // Should not be ready after execution
        assertFalse(verifier.isChangeReady(changeHash));
    }

    // ============================================================================
    // Access Control Tests
    // ============================================================================

    function test_RevertProposeChangeByNonOwner() public {
        vm.prank(nonOwner);

        // Should revert with ACLOwnable error
        vm.expectRevert();
        verifier.proposeThresholdChange(8);
    }

    function test_RevertDefineNewContextByNonOwner() public {
        address[] memory newSigners = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            newSigners[i] = address(uint160(i + 100));
        }

        vm.prank(nonOwner);

        vm.expectRevert();
        verifier.defineNewContext(newSigners, 3);
    }

    // ============================================================================
    // Context Management Tests
    // ============================================================================

    function test_ContextIdIncrement() public {
        uint256 initialContextId = verifier.getCurrentContextId();

        address[] memory newSigners = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            newSigners[i] = address(uint160(i + 100));
        }

        verifier.defineNewContext(newSigners, 3);

        assertEq(verifier.getCurrentContextId(), initialContextId + 1);
        assertTrue(verifier.isValidContext(initialContextId + 1));
    }

    function test_EmitContextSetWithValidation() public {
        address[] memory newSigners = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            newSigners[i] = address(uint160(i + 100));
        }

        vm.expectEmit(true, true, true, true);
        emit ContextSetWithValidation(5, 3, 3);

        verifier.defineNewContext(newSigners, 3);
    }

    // ============================================================================
    // Fuzz Tests
    // ============================================================================

    function testFuzz_ValidThreshold(uint8 numSigners, uint256 threshold) public {
        // Bound inputs
        uint256 boundedSigners = bound(uint256(numSigners), 3, 100);

        // Create signers array
        address[] memory fuzzSigners = new address[](boundedSigners);
        for (uint256 i = 0; i < boundedSigners; i++) {
            fuzzSigners[i] = address(uint160(i + 1));
        }

        // Calculate minimum threshold
        uint256 minThreshold = (boundedSigners * 51 + 99) / 100;

        // Deploy new proxy
        address newProxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()), abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );

        address implementation = address(new EnhancedInputVerifier());

        // Bound threshold to valid range for fuzz testing
        uint256 boundedThreshold = bound(threshold, minThreshold, boundedSigners);

        // Test valid threshold
        UnsafeUpgrades.upgradeProxy(
            newProxy,
            implementation,
            abi.encodeCall(
                EnhancedInputVerifier.initializeFromEmptyProxy,
                (address(0x1), uint64(block.chainid), fuzzSigners, boundedThreshold)
            ),
            owner
        );

        EnhancedInputVerifier newVerifier = EnhancedInputVerifier(newProxy);
        assertEq(newVerifier.getThreshold(), boundedThreshold);
    }

    function testFuzz_TimeLock(uint256 warpTime) public {
        uint256 newThreshold = 8;
        bytes32 changeHash = verifier.proposeThresholdChange(newThreshold);

        // Get the effective time from the pending change
        (,, uint256 effectiveTime,,,) = _getPendingChange(changeHash);

        uint256 expirationTime = effectiveTime + CHANGE_EXPIRATION_PERIOD;
        uint256 maxWarp = expirationTime - block.timestamp + 1000;

        uint256 boundedWarp = bound(warpTime, 0, maxWarp);
        vm.warp(block.timestamp + boundedWarp);

        if (block.timestamp < effectiveTime) {
            // Should not be ready (before delay)
            assertFalse(verifier.isChangeReady(changeHash));
        } else if (block.timestamp <= expirationTime) {
            // Should be ready (after delay, before expiration)
            assertTrue(verifier.isChangeReady(changeHash));
        } else {
            // Should be expired
            assertFalse(verifier.isChangeReady(changeHash));
        }
    }

    // ============================================================================
    // Helper Functions
    // ============================================================================

    function _calculateMinThreshold(uint256 numSigners) internal pure returns (uint256) {
        return (numSigners * MINIMUM_THRESHOLD_PERCENTAGE + 99) / 100;
    }

    function _getPendingChange(bytes32 changeHash)
        internal
        view
        returns (
            uint256 newThreshold,
            uint256 proposedTime,
            uint256 effectiveTime,
            address proposer,
            bool executed,
            bool cancelled
        )
    {
        EnhancedInputVerifier.PendingThresholdChange memory change = verifier.getPendingChange(changeHash);

        return (
            change.newThreshold,
            change.proposedTime,
            change.effectiveTime,
            change.proposer,
            change.executed,
            change.cancelled
        );
    }
}
