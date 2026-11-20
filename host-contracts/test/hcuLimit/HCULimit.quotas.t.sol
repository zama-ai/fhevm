// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {HCULimit} from "../../contracts/HCULimit.sol";
import {HCULimitOperators} from "../../contracts/HCULimitOperators.sol";
import {FheType} from "../../contracts/shared/FheType.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {aclAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {MockHCULimit} from "./HCULimit.t.sol";
import {HCULimitV040TestDouble} from "./HCULimitV040TestDouble.sol";
import {HCULimitV050TestDouble} from "./HCULimitV050TestDouble.sol";

/// @dev Quotas on every computation path, and the upgrade from the block cap, its whitelist and the allowlist.
contract HCULimitQuotasTest is Test {
    MockHCULimit internal hcuLimit;
    address internal constant OWNER = address(456);
    address internal constant DAPP = address(0xDABB);
    uint16 internal constant DEFAULT_QUOTA = 0;
    uint16 internal constant ALLOWLIST_QUOTA = 1;
    uint16 internal constant BLOCK_WHITELIST_QUOTA = 2;
    uint16 internal constant DAPP_QUOTA = 3;
    uint48 internal constant LEGACY_CAP = 100_000;
    address internal executor;
    bytes32 internal constant INPUT = bytes32(uint256(1));
    bytes32 internal constant RESULT = bytes32(uint256(2));

    function setUp() public {
        vm.etch(aclAdd, address(new ACL()).code);
        vm.store(
            aclAdd,
            // ERC-7201: openzeppelin.storage.Ownable; ACL's _owner is at offset zero.
            0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300,
            bytes32(uint256(uint160(OWNER)))
        );
        address proxy = _emptyProxy();
        vm.startPrank(OWNER);
        UnsafeUpgrades.upgradeProxy(
            proxy,
            address(new MockHCULimit()),
            abi.encodeCall(HCULimit.initializeFromEmptyProxy, (type(uint48).max, 5_000_000, 20_000_000))
        );
        vm.stopPrank();
        hcuLimit = MockHCULimit(proxy);
        executor = hcuLimit.getFHEVMExecutorAddress();
    }

    function test_freshDeploymentLeavesTheDefaultQuotaUnmeteredAndConsumesInitializerVersion() public {
        (uint48 maxHCU, , ) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(maxHCU, type(uint48).max);
        _compute(DAPP);
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        hcuLimit.reinitializeV6();
    }

    /// The upgrade from v0.4.0 moves its global cap into the default quota and keeps the transaction limits; its
    /// block whitelist routes nothing while the allowlist is inactive.
    function test_upgradeFromV040MovesTheGlobalCapIntoTheDefaultQuota() public {
        address proxy = _emptyProxy();
        vm.startPrank(OWNER);
        UnsafeUpgrades.upgradeProxy(
            proxy,
            address(new HCULimitV040TestDouble()),
            abi.encodeCall(HCULimitV040TestDouble.initializeFromEmptyProxy, (DAPP))
        );
        UnsafeUpgrades.upgradeProxy(proxy, address(new MockHCULimit()), abi.encodeCall(HCULimit.reinitializeV6, ()));
        vm.stopPrank();
        MockHCULimit upgraded = MockHCULimit(proxy);
        (uint48 maxHCU, uint48 decayPerBlock, uint48 usedHCU) = upgraded.getQuota(DEFAULT_QUOTA);
        assertEq(maxHCU, 30_000_000);
        assertEq(decayPerBlock, 30_000_000);
        assertEq(usedHCU, 0);
        assertEq(upgraded.getMaxHCUPerTx(), 20_000_000);
        assertEq(upgraded.getMaxHCUDepthPerTx(), 5_000_000);
        assertEq(upgraded.getContractQuota(DAPP), DEFAULT_QUOTA);
        assertEq(upgraded.getVersion(), "HCULimit v0.6.0");
        (uint48 maxCrossTxDepth, uint48 crossTxDecayPerBlock) = upgraded.getCrossTxDepthLimit();
        assertEq(maxCrossTxDepth, 0);
        assertEq(crossTxDecayPerBlock, 0);
        vm.prank(executor);
        upgraded.checkHCUForCast(FheType.Uint8, INPUT, RESULT, DAPP);
        (, , usedHCU) = upgraded.getQuota(DEFAULT_QUOTA);
        assertEq(usedHCU, 32);
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        upgraded.reinitializeV6();
    }

    /// Each combination of the allowlist's three fields a deployed v0.5.0 can hold, and where the upgrade sends the
    /// contract: the default quota with the global cap, the allowlist quota, the unmetered block-whitelist quota, or
    /// the stopped default quota.
    function test_upgradeRoutesEveryAllowlistAndBlockWhitelistCombination(
        bool activated,
        bool allowed,
        bool whitelisted
    ) public {
        _upgradeFromV050(activated, allowed, whitelisted);

        // The old global cap moves to the default quota, or, with the allowlist active, to the allowlist quota.
        (uint48 defaultMax, uint48 defaultDecay, ) = hcuLimit.getQuota(DEFAULT_QUOTA);
        (uint48 allowlistMax, uint48 allowlistDecay, ) = hcuLimit.getQuota(ALLOWLIST_QUOTA);
        (uint48 whitelistMax, , ) = hcuLimit.getQuota(BLOCK_WHITELIST_QUOTA);
        assertEq(defaultMax, activated ? 0 : LEGACY_CAP);
        assertEq(defaultDecay, activated ? 0 : LEGACY_CAP);
        assertEq(allowlistMax, activated ? LEGACY_CAP : 0);
        assertEq(allowlistDecay, activated ? LEGACY_CAP : 0);
        assertEq(whitelistMax, activated ? type(uint48).max : 0);

        uint16 expected = !activated || !allowed
            ? DEFAULT_QUOTA
            : (whitelisted ? BLOCK_WHITELIST_QUOTA : ALLOWLIST_QUOTA);
        assertEq(hcuLimit.getContractQuota(DAPP), expected);
        if (activated && !allowed) {
            vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, DAPP, DEFAULT_QUOTA));
            _compute(DAPP);
            assertEq(hcuLimit.getHCUForTransaction(), 0);
            return;
        }
        _compute(DAPP);
        assertEq(hcuLimit.getHCUForTransaction(), 32);
        (, , uint48 usedHCU) = hcuLimit.getQuota(expected);
        assertEq(usedHCU, expected == BLOCK_WHITELIST_QUOTA ? 0 : 32);
    }

    /// Rolling back to v0.5.0 restores its cap, limits, whitelist and allowlist, except the allowlist entries of
    /// contracts moved to the default quota. Upgrading again needs no reinitializer: the quotas apply again.
    function test_rollbackToV050AndUpgradeAgain() public {
        _upgradeFromV050(true, true, true);
        address proxy = address(hcuLimit);
        address other = address(0xD0D0);
        vm.startPrank(OWNER);
        hcuLimit.setQuota(DAPP_QUOTA, 1_000, 1_000);
        hcuLimit.assignQuota(DAPP_QUOTA, other);
        hcuLimit.setQuotaAdmin(DAPP_QUOTA, address(0xAD));
        hcuLimit.pauseQuota(DAPP_QUOTA);
        hcuLimit.setCrossTxDepthLimit(10_000_000, 1_000_000);
        hcuLimit.setQuotaMaxStoredDepth(DEFAULT_QUOTA, 6_000_000);
        hcuLimit.assignQuota(DEFAULT_QUOTA, DAPP);

        UnsafeUpgrades.upgradeProxy(proxy, address(new HCULimitV050TestDouble()), "");
        HCULimitV050TestDouble rolledBack = HCULimitV050TestDouble(proxy);
        assertEq(rolledBack.getGlobalHCUCapPerBlock(), LEGACY_CAP);
        assertEq(rolledBack.getMaxHCUPerTx(), 20_000_000);
        assertEq(rolledBack.getMaxHCUDepthPerTx(), 5_000_000);
        assertTrue(rolledBack.isAllowlistActivated());
        assertTrue(rolledBack.isBlockHCUWhitelisted(DAPP));
        assertFalse(rolledBack.isAllowlisted(DAPP));

        UnsafeUpgrades.upgradeProxy(proxy, address(new MockHCULimit()), "");
        vm.stopPrank();
        assertEq(hcuLimit.getContractQuota(other), DAPP_QUOTA);
        assertEq(hcuLimit.getContractQuota(DAPP), DEFAULT_QUOTA);
        assertTrue(hcuLimit.isQuotaPaused(DAPP_QUOTA));
        assertEq(hcuLimit.getQuotaMaxStoredDepth(DEFAULT_QUOTA), 6_000_000);
        (uint48 maxHCU, , ) = hcuLimit.getQuota(ALLOWLIST_QUOTA);
        assertEq(maxHCU, LEGACY_CAP);
    }

    function test_stoppedQuotaRefusesBeforeChargingAnything() public {
        vm.startPrank(OWNER);
        hcuLimit.setQuota(DEFAULT_QUOTA, 32, 32);
        hcuLimit.setQuota(DAPP_QUOTA, 1_000, 1_000);
        hcuLimit.assignQuota(DAPP_QUOTA, DAPP);
        hcuLimit.setQuota(DAPP_QUOTA, 0, 0);
        vm.stopPrank();
        _compute(address(0xBEEF));

        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, DAPP, DAPP_QUOTA));
        _compute(DAPP);
        (, , uint48 usedHCU) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(usedHCU, 32);
        assertEq(hcuLimit.getHCUForTransaction(), 32);
    }

    function test_quotaMembersRemainSubjectToTransactionLimits(bool unmetered) public {
        uint48 maxHCU = unmetered ? type(uint48).max : 1_000;
        vm.startPrank(OWNER);
        hcuLimit.setQuota(DAPP_QUOTA, maxHCU, maxHCU);
        hcuLimit.assignQuota(DAPP_QUOTA, DAPP);
        vm.stopPrank();
        hcuLimit.setHCUForTransaction(20_000_000);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        _compute(DAPP);
        hcuLimit.setHCUForTransaction(0);
        hcuLimit.setHCUForHandle(INPUT, 5_000_000);
        vm.expectRevert(HCULimit.HCUTransactionDepthLimitExceeded.selector);
        _compute(DAPP);
        hcuLimit.setHCUForHandle(INPUT, 0);
        if (unmetered) {
            _compute(DAPP);
        } else {
            vm.prank(OWNER);
            hcuLimit.setQuota(DAPP_QUOTA, 31, 31);
            vm.expectRevert(
                abi.encodeWithSelector(HCULimit.HCUQuotaExceeded.selector, DAPP, DAPP_QUOTA, type(uint256).max)
            );
            _compute(DAPP);
        }
    }

    function test_everyComputationPathEnforcesTheQuotaAndTheExecutor() public {
        vm.prank(OWNER);
        hcuLimit.setQuota(DEFAULT_QUOTA, 0, 0);
        bytes[] memory calls = _computationCalls(DAPP);
        for (uint256 i; i < calls.length; ++i) {
            vm.prank(executor);
            (bool success, bytes memory reason) = address(hcuLimit).call(calls[i]);
            assertFalse(success, string(abi.encodePacked("ungated path ", vm.toString(i))));
            assertEq(reason, abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, DAPP, DEFAULT_QUOTA));
        }
        assertEq(hcuLimit.getHCUForTransaction(), 0);
        vm.startPrank(OWNER);
        hcuLimit.setQuota(DAPP_QUOTA, type(uint48).max, type(uint48).max);
        hcuLimit.assignQuota(DAPP_QUOTA, DAPP);
        vm.stopPrank();
        for (uint256 i; i < calls.length; ++i) {
            // A forged dapp argument must not bypass executor-only entry points.
            (bool success, bytes memory reason) = address(hcuLimit).call(calls[i]);
            assertFalse(success);
            assertEq(reason, abi.encodeWithSelector(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector));
            hcuLimit.setHCUForTransaction(0);
            hcuLimit.setHCUForHandle(RESULT, 0);
            vm.prank(executor);
            (success, reason) = address(hcuLimit).call(calls[i]);
            assertTrue(success, string(abi.encodePacked("quota path failed ", vm.toString(i), vm.toString(reason))));
        }
    }

    function _emptyProxy() internal returns (address) {
        return
            UnsafeUpgrades.deployUUPSProxy(
                address(new EmptyUUPSProxy()),
                abi.encodeCall(EmptyUUPSProxy.initialize, ())
            );
    }

    /// @dev Replaces the proxy under test with a v0.5.0 one holding a global cap of LEGACY_CAP and DAPP's allowlist
    ///      state, upgraded with reinitializeV6.
    function _upgradeFromV050(bool activated, bool allowed, bool whitelisted) internal {
        address proxy = _emptyProxy();
        address[] memory allowlisted = new address[](allowed ? 1 : 0);
        if (allowed) allowlisted[0] = DAPP;
        address[] memory blockWhitelisted = new address[](whitelisted ? 1 : 0);
        if (whitelisted) blockWhitelisted[0] = DAPP;
        vm.startPrank(OWNER);
        UnsafeUpgrades.upgradeProxy(
            proxy,
            address(new HCULimitV050TestDouble()),
            abi.encodeCall(
                HCULimitV050TestDouble.initializeFromEmptyProxy,
                (LEGACY_CAP, activated, allowlisted, blockWhitelisted)
            )
        );
        UnsafeUpgrades.upgradeProxy(proxy, address(new MockHCULimit()), abi.encodeCall(HCULimit.reinitializeV6, ()));
        vm.stopPrank();
        hcuLimit = MockHCULimit(proxy);
    }

    function _compute(address caller) internal {
        vm.prank(executor);
        hcuLimit.checkHCUForCast(FheType.Uint8, INPUT, RESULT, caller);
    }

    function _computationCalls(address caller) internal pure returns (bytes[] memory calls) {
        bytes4[20] memory binarySelectors = [
            HCULimitOperators.checkHCUForFheAdd.selector,
            HCULimitOperators.checkHCUForFheSub.selector,
            HCULimitOperators.checkHCUForFheMul.selector,
            HCULimitOperators.checkHCUForFheDiv.selector,
            HCULimitOperators.checkHCUForFheRem.selector,
            HCULimitOperators.checkHCUForFheBitAnd.selector,
            HCULimitOperators.checkHCUForFheBitOr.selector,
            HCULimitOperators.checkHCUForFheBitXor.selector,
            HCULimitOperators.checkHCUForFheShl.selector,
            HCULimitOperators.checkHCUForFheShr.selector,
            HCULimitOperators.checkHCUForFheRotl.selector,
            HCULimitOperators.checkHCUForFheRotr.selector,
            HCULimitOperators.checkHCUForFheEq.selector,
            HCULimitOperators.checkHCUForFheNe.selector,
            HCULimitOperators.checkHCUForFheGe.selector,
            HCULimitOperators.checkHCUForFheGt.selector,
            HCULimitOperators.checkHCUForFheLe.selector,
            HCULimitOperators.checkHCUForFheLt.selector,
            HCULimitOperators.checkHCUForFheMin.selector,
            HCULimitOperators.checkHCUForFheMax.selector
        ];
        calls = new bytes[](49);
        uint256 n;
        for (uint256 i; i < binarySelectors.length; ++i) {
            calls[n++] = abi.encodeWithSelector(
                binarySelectors[i],
                FheType.Uint8,
                bytes1(0x01),
                INPUT,
                INPUT,
                RESULT,
                caller
            );
            if (
                binarySelectors[i] != HCULimitOperators.checkHCUForFheDiv.selector &&
                binarySelectors[i] != HCULimitOperators.checkHCUForFheRem.selector
            ) {
                calls[n++] = abi.encodeWithSelector(
                    binarySelectors[i],
                    FheType.Uint8,
                    bytes1(0x00),
                    INPUT,
                    INPUT,
                    RESULT,
                    caller
                );
            }
        }
        calls[n++] = abi.encodeCall(HCULimitOperators.checkHCUForFheNeg, (FheType.Uint8, INPUT, RESULT, caller));
        calls[n++] = abi.encodeCall(HCULimitOperators.checkHCUForFheNot, (FheType.Uint8, INPUT, RESULT, caller));
        calls[n++] = abi.encodeCall(HCULimitOperators.checkHCUForCast, (FheType.Uint8, INPUT, RESULT, caller));
        calls[n++] = abi.encodeCall(HCULimitOperators.checkHCUForTrivialEncrypt, (FheType.Uint8, RESULT, caller));
        calls[n++] = abi.encodeCall(
            HCULimitOperators.checkHCUForIfThenElse,
            (FheType.Uint8, INPUT, INPUT, INPUT, RESULT, caller)
        );
        calls[n++] = abi.encodeCall(HCULimitOperators.checkHCUForFheRand, (FheType.Uint8, RESULT, caller));
        calls[n++] = abi.encodeCall(HCULimitOperators.checkHCUForFheRandBounded, (FheType.Uint8, RESULT, caller));
        bytes32[] memory values = new bytes32[](2);
        values[0] = INPUT;
        values[1] = INPUT;
        calls[n++] = abi.encodeCall(HCULimitOperators.checkHCUForFheSum, (FheType.Uint8, values, RESULT, caller));
        calls[n++] = abi.encodeCall(
            HCULimitOperators.checkHCUForFheIsIn,
            (FheType.Uint8, INPUT, values, RESULT, caller)
        );
        calls[n++] = abi.encodeCall(
            HCULimitOperators.checkHCUForFheMulDiv,
            (FheType.Uint8, 0x01, INPUT, INPUT, RESULT, caller)
        );
        calls[n++] = abi.encodeCall(
            HCULimitOperators.checkHCUForFheMulDiv,
            (FheType.Uint8, 0x03, INPUT, INPUT, RESULT, caller)
        );
    }
}
