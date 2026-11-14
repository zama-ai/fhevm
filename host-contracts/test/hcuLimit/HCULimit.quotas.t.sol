// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {Errors} from "@openzeppelin/contracts/utils/Errors.sol";
import {HCULimit} from "../../contracts/HCULimit.sol";
import {HCULimitOperators} from "../../contracts/HCULimitOperators.sol";
import {FheType} from "../../contracts/shared/FheType.sol";
import {ACL} from "../../contracts/ACL.sol";
import {HCUConfigurator} from "../../contracts/HCUConfigurator.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {aclAdd, fhevmExecutorAdd, hcuConfiguratorAdd, hcuLimitAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {HostContractsDeployerTestUtils} from "../../fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {MockHCULimit} from "./HCULimit.t.sol";
import {HCULimitV040TestDouble} from "./HCULimitV040TestDouble.sol";
import {HCULimitV050TestDouble} from "./HCULimitV050TestDouble.sol";

/// @dev Quotas on every computation path, and the upgrade from the block cap, its whitelist and the allowlist.
///      Each test deploys HCULimit at hcuLimitAdd, fresh or from an earlier version, since HCUConfigurator writes there.
contract HCULimitQuotasTest is HostContractsDeployerTestUtils {
    MockHCULimit internal hcuLimit = MockHCULimit(hcuLimitAdd);
    HCUConfigurator internal constant hcuConfigurator = HCUConfigurator(hcuConfiguratorAdd);
    address internal constant OWNER = address(456);
    address internal constant DAPP = address(0xDABB);
    uint16 internal constant DEFAULT_QUOTA = 0;
    uint16 internal constant ALLOWLIST_QUOTA = 1;
    uint16 internal constant BLOCK_WHITELIST_QUOTA = 2;
    uint16 internal constant DAPP_QUOTA = 3;
    address internal constant DAPP_ADMIN = address(0xAD);
    address internal constant ALLOWED = address(0xC1);
    address internal constant ALLOWED_BLOCK_WHITELISTED = address(0xC2);
    address internal constant UNKNOWN = address(0xC4);
    uint48 internal constant LEGACY_CAP = 100_000;
    address internal constant executor = fhevmExecutorAdd;
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
        _deployHCUConfigurator(OWNER);
    }

    function test_freshDeploymentLeavesTheDefaultQuotaUnmeteredAndConsumesInitializerVersion() public {
        _deployFresh();
        (uint48 maxHCU, , ) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(maxHCU, type(uint48).max);
        _compute(DAPP);
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        hcuLimit.reinitializeV6();
    }

    /// The upgrade from v0.4.0 moves its global cap into the default quota and keeps the transaction limits; its
    /// block whitelist routes nothing while the allowlist is inactive.
    function test_upgradeFromV040MovesTheGlobalCapIntoTheDefaultQuota() public {
        address proxy = hcuLimitAdd;
        _deployEmptyProxyAt(proxy);
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
        _upgradeFromV050(activated, allowed ? _list(DAPP) : new address[](0), whitelisted ? _list(DAPP) : new address[](0));

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

    /// HCUConfigurator is the only writer of the settings, so the upgrade waits until its empty proxy is upgraded.
    function test_reinitializeV6RefusesTheUpgradeUntilHCUConfiguratorRuns() public {
        // Puts the empty implementation back behind HCUConfigurator's proxy, as before its first upgrade.
        UnsafeUpgrades.upgradeProxy(hcuConfiguratorAdd, address(new EmptyUUPSProxy()), "", OWNER);
        _deployEmptyProxyAt(hcuLimitAdd);
        vm.startPrank(OWNER);
        UnsafeUpgrades.upgradeProxy(
            hcuLimitAdd,
            address(new HCULimitV050TestDouble()),
            abi.encodeCall(
                HCULimitV050TestDouble.initializeFromEmptyProxy,
                (LEGACY_CAP, false, new address[](0), new address[](0))
            )
        );
        address implementation = address(new MockHCULimit());
        // The empty proxy has no getVersion and reverts with no data, which upgradeToAndCall reports as FailedCall.
        vm.expectRevert(Errors.FailedCall.selector);
        hcuLimit.upgradeToAndCall(implementation, abi.encodeCall(HCULimit.reinitializeV6, ()));
        vm.stopPrank();
    }

    /// Rolling back to v0.5.0 restores its cap, limits, whitelist and allowlist, except the allowlist entries of
    /// contracts moved to the default quota. Upgrading again needs no reinitializer: the quotas apply again.
    function test_rollbackToV050AndUpgradeAgain() public {
        _upgradeFromV050(true, _list(DAPP), _list(DAPP));
        address proxy = hcuLimitAdd;
        address other = address(0xD0D0);
        vm.startPrank(OWNER);
        hcuConfigurator.setQuota(DAPP_QUOTA, 1_000, 1_000);
        hcuConfigurator.assignQuota(DAPP_QUOTA, other);
        hcuConfigurator.setQuotaAdmin(DAPP_QUOTA, address(0xAD));
        hcuConfigurator.pauseQuota(DAPP_QUOTA);
        hcuConfigurator.setCrossTxDepthLimit(10_000_000, 1_000_000);
        hcuConfigurator.setQuotaMaxStoredDepth(DEFAULT_QUOTA, 6_000_000);
        hcuConfigurator.assignQuota(DEFAULT_QUOTA, DAPP);

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

    /// A contract allowlisted while the allowlist was inactive is not routed: stopping the default quota stops it.
    function test_inactiveAllowlistRoutesNothing() public {
        _upgradeFromV050(false, _list(ALLOWED), new address[](0));
        vm.prank(OWNER);
        hcuConfigurator.setQuota(DEFAULT_QUOTA, 0, 0);

        assertEq(hcuLimit.getContractQuota(ALLOWED), DEFAULT_QUOTA);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, ALLOWED, DEFAULT_QUOTA));
        _compute(ALLOWED);
    }

    function test_assignmentTakesPrecedenceOverTheAllowlist() public {
        _upgradeFromV050(true, _list(ALLOWED), new address[](0));
        vm.startPrank(OWNER);
        hcuConfigurator.setQuota(DAPP_QUOTA, 1_000, 1_000);
        hcuConfigurator.assignQuota(DAPP_QUOTA, ALLOWED);
        vm.stopPrank();
        assertEq(hcuLimit.getContractQuota(ALLOWED), DAPP_QUOTA);
        _compute(ALLOWED);
        (, , uint48 used) = hcuLimit.getQuota(DAPP_QUOTA);
        assertEq(used, 32);

        // Back in the default quota, it has left the allowlist and stays stopped.
        vm.prank(OWNER);
        hcuConfigurator.assignQuota(DEFAULT_QUOTA, ALLOWED);
        assertEq(hcuLimit.getContractQuota(ALLOWED), DEFAULT_QUOTA);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, ALLOWED, DEFAULT_QUOTA));
        _compute(ALLOWED);
    }

    function test_openingTheDefaultQuotaSuspendsTheAllowlistRouting() public {
        _upgradeFromV050(true, _list(ALLOWED, ALLOWED_BLOCK_WHITELISTED), _list(ALLOWED_BLOCK_WHITELISTED));
        vm.prank(OWNER);
        hcuConfigurator.setQuota(DEFAULT_QUOTA, 1_000, 1_000);
        assertEq(hcuLimit.getContractQuota(ALLOWED), DEFAULT_QUOTA);
        _compute(ALLOWED);
        (, , uint48 used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(used, 32);

        // Stopping it again routes the contracts nobody assigned back to the quotas that replaced the allowlist.
        vm.prank(OWNER);
        hcuConfigurator.setQuota(DEFAULT_QUOTA, 0, 0);
        assertEq(hcuLimit.getContractQuota(ALLOWED), ALLOWLIST_QUOTA);
        assertEq(hcuLimit.getContractQuota(ALLOWED_BLOCK_WHITELISTED), BLOCK_WHITELIST_QUOTA);
        _compute(ALLOWED);
        (, , used) = hcuLimit.getQuota(ALLOWLIST_QUOTA);
        assertEq(used, 32);
    }

    function test_stoppingOrPausingTheAllowlistQuotaStopsTheContractsItRoutes() public {
        _upgradeFromV050(true, _list(ALLOWED), new address[](0));
        vm.prank(OWNER);
        hcuConfigurator.pauseQuota(ALLOWLIST_QUOTA);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaPaused.selector, ALLOWED, ALLOWLIST_QUOTA));
        _compute(ALLOWED);

        vm.startPrank(OWNER);
        hcuConfigurator.unpauseQuota(ALLOWLIST_QUOTA);
        hcuConfigurator.setQuota(ALLOWLIST_QUOTA, 0, 0);
        vm.stopPrank();
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, ALLOWED, ALLOWLIST_QUOTA));
        _compute(ALLOWED);
    }

    function test_quotaAdminCannotTakeAnAllowlistedContract() public {
        _upgradeFromV050(true, _list(ALLOWED), new address[](0));
        vm.startPrank(OWNER);
        hcuConfigurator.setQuota(DAPP_QUOTA, 1_000, 1_000);
        hcuConfigurator.setQuotaAdmin(DAPP_QUOTA, DAPP_ADMIN);
        vm.stopPrank();

        vm.startPrank(DAPP_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.ContractNotInQuota.selector, ALLOWED, DEFAULT_QUOTA));
        hcuConfigurator.grantQuota(DAPP_QUOTA, ALLOWED);
        hcuConfigurator.grantQuota(DAPP_QUOTA, UNKNOWN);
        vm.stopPrank();
        assertEq(hcuLimit.getContractQuota(UNKNOWN), DAPP_QUOTA);
    }

    function test_revokingAnAllowlistedContractStopsIt() public {
        _upgradeFromV050(true, _list(ALLOWED_BLOCK_WHITELISTED), _list(ALLOWED_BLOCK_WHITELISTED));
        vm.startPrank(OWNER);
        hcuConfigurator.setQuota(DAPP_QUOTA, 1_000, 1_000);
        hcuConfigurator.setQuotaAdmin(DAPP_QUOTA, DAPP_ADMIN);
        hcuConfigurator.assignQuota(DAPP_QUOTA, ALLOWED_BLOCK_WHITELISTED);
        vm.stopPrank();

        // Without leaving the allowlist, it would compute unmetered in quota 2 instead.
        vm.prank(DAPP_ADMIN);
        hcuConfigurator.revokeQuota(DAPP_QUOTA, ALLOWED_BLOCK_WHITELISTED);
        vm.expectRevert(
            abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, ALLOWED_BLOCK_WHITELISTED, DEFAULT_QUOTA)
        );
        _compute(ALLOWED_BLOCK_WHITELISTED);
    }

    function test_stoppedQuotaRefusesBeforeChargingAnything() public {
        _deployFresh();
        vm.startPrank(OWNER);
        hcuConfigurator.setQuota(DEFAULT_QUOTA, 32, 32);
        hcuConfigurator.setQuota(DAPP_QUOTA, 1_000, 1_000);
        hcuConfigurator.assignQuota(DAPP_QUOTA, DAPP);
        hcuConfigurator.setQuota(DAPP_QUOTA, 0, 0);
        vm.stopPrank();
        _compute(address(0xBEEF));

        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, DAPP, DAPP_QUOTA));
        _compute(DAPP);
        (, , uint48 usedHCU) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(usedHCU, 32);
        assertEq(hcuLimit.getHCUForTransaction(), 32);
    }

    function test_quotaMembersRemainSubjectToTransactionLimits(bool unmetered) public {
        _deployFresh();
        uint48 maxHCU = unmetered ? type(uint48).max : 1_000;
        vm.startPrank(OWNER);
        hcuConfigurator.setQuota(DAPP_QUOTA, maxHCU, maxHCU);
        hcuConfigurator.assignQuota(DAPP_QUOTA, DAPP);
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
            hcuConfigurator.setQuota(DAPP_QUOTA, 31, 31);
            vm.expectRevert(
                abi.encodeWithSelector(HCULimit.HCUQuotaExceeded.selector, DAPP, DAPP_QUOTA, type(uint256).max)
            );
            _compute(DAPP);
        }
    }

    function test_everyComputationPathEnforcesTheQuotaAndTheExecutor() public {
        _deployFresh();
        vm.prank(OWNER);
        hcuConfigurator.setQuota(DEFAULT_QUOTA, 0, 0);
        bytes[] memory calls = _computationCalls(DAPP);
        for (uint256 i; i < calls.length; ++i) {
            vm.prank(executor);
            (bool success, bytes memory reason) = address(hcuLimit).call(calls[i]);
            assertFalse(success, string(abi.encodePacked("ungated path ", vm.toString(i))));
            assertEq(reason, abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, DAPP, DEFAULT_QUOTA));
        }
        assertEq(hcuLimit.getHCUForTransaction(), 0);
        vm.startPrank(OWNER);
        hcuConfigurator.setQuota(DAPP_QUOTA, type(uint48).max, type(uint48).max);
        hcuConfigurator.assignQuota(DAPP_QUOTA, DAPP);
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

    function _deployFresh() internal {
        _deployEmptyProxyAt(hcuLimitAdd);
        vm.startPrank(OWNER);
        UnsafeUpgrades.upgradeProxy(
            hcuLimitAdd,
            address(new MockHCULimit()),
            abi.encodeCall(HCULimit.initializeFromEmptyProxy, (type(uint48).max, 5_000_000, 20_000_000))
        );
        vm.stopPrank();
    }

    /// @dev Deploys a v0.5.0 HCULimit holding a global cap of LEGACY_CAP and the given allowlist state, and upgrades it
    ///      with reinitializeV6.
    function _upgradeFromV050(bool activated, address[] memory allowlisted, address[] memory whitelisted) internal {
        _deployEmptyProxyAt(hcuLimitAdd);
        vm.startPrank(OWNER);
        UnsafeUpgrades.upgradeProxy(
            hcuLimitAdd,
            address(new HCULimitV050TestDouble()),
            abi.encodeCall(
                HCULimitV050TestDouble.initializeFromEmptyProxy,
                (LEGACY_CAP, activated, allowlisted, whitelisted)
            )
        );
        UnsafeUpgrades.upgradeProxy(
            hcuLimitAdd,
            address(new MockHCULimit()),
            abi.encodeCall(HCULimit.reinitializeV6, ())
        );
        vm.stopPrank();
    }

    function _list(address account) internal pure returns (address[] memory accounts) {
        accounts = new address[](1);
        accounts[0] = account;
    }

    function _list(address first, address second) internal pure returns (address[] memory accounts) {
        accounts = new address[](2);
        accounts[0] = first;
        accounts[1] = second;
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
