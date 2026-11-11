// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {StdInvariant} from "forge-std/StdInvariant.sol";

import {HCULimit} from "../../contracts/HCULimit.sol";
import {HCUConfigurator} from "../../contracts/HCUConfigurator.sol";
import {ACL} from "../../contracts/ACL.sol";
import {HostContractsDeployerTestUtils} from "../../fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {FheType} from "../../contracts/shared/FheType.sol";
import {aclAdd, hcuConfiguratorAdd, hcuLimitAdd} from "../../addresses/FHEVMHostAddresses.sol";

contract HCULimitInvariantHandler is Test {
    HCULimit internal immutable hcuLimit;
    HCUConfigurator internal immutable hcuConfigurator;
    address internal immutable owner;
    address internal immutable fhevmExecutor;

    uint16 internal constant DEFAULT_QUOTA = 0;
    // Quotas 1 and 2 replace the allowlist and take no admin.
    uint16 internal constant UNCAPPED_QUOTA = 3;
    uint16 internal constant PARTNER_QUOTA = 4;
    address internal constant DEFAULT_CALLER = address(0xBEEF);
    address internal constant UNCAPPED_CALLER = address(0xCAFE);
    address internal constant PARTNER_ADMIN = address(0xAD);
    bytes32 internal constant INPUT_HANDLE = bytes32(uint256(0xA11CE));
    uint48 internal constant NO_CAP = type(uint48).max;
    uint256 internal constant MAX_BPS = 10_000;
    uint256 internal constant CAST_HCU = 32;
    uint48 internal constant MAX_HCU_PER_TX = 1_000_000;

    uint256 internal resultNonce = 1;
    bool public uncappedViolation;
    bool public accountingViolation;
    bool public decayViolation;
    bool public pausedViolation;
    bool public adminMoveViolation;
    bool public tierViolation;
    bool public capacityViolation;

    constructor(HCULimit _hcuLimit, HCUConfigurator _hcuConfigurator, address _owner) {
        hcuLimit = _hcuLimit;
        hcuConfigurator = _hcuConfigurator;
        owner = _owner;
        fhevmExecutor = _hcuLimit.getFHEVMExecutorAddress();

        vm.startPrank(owner);
        // Reserved must fit a full transaction, so capacities can be small only with a small maxHCUPerTx.
        hcuConfigurator.setMaxHCUPerTx(MAX_HCU_PER_TX);
        hcuConfigurator.setQuota(UNCAPPED_QUOTA, type(uint48).max, type(uint48).max);
        hcuConfigurator.assignQuota(UNCAPPED_QUOTA, UNCAPPED_CALLER);
        hcuConfigurator.setTier(1, 5_000);
        hcuConfigurator.setQuotaTier(UNCAPPED_QUOTA, 1);
        hcuConfigurator.setQuota(PARTNER_QUOTA, 1_000_000, 1_000_000);
        hcuConfigurator.setQuotaAdmin(PARTNER_QUOTA, PARTNER_ADMIN);
        vm.stopPrank();
    }

    function setPaused(uint8 rawQuota, bool paused) external {
        uint16[3] memory quotaIds = [DEFAULT_QUOTA, UNCAPPED_QUOTA, PARTNER_QUOTA];
        uint16 quotaId = quotaIds[bound(uint256(rawQuota), 0, 2)];
        vm.prank(owner);
        if (paused) hcuConfigurator.pauseQuota(quotaId);
        else hcuConfigurator.unpauseQuota(quotaId);
    }

    /// @dev The admin may only move a contract from the default quota into its quota or back, never while it is paused.
    function adminMove(address caller, bool grant) external {
        caller = _sanitizeCaller(caller);
        uint16 before = hcuLimit.getContractQuota(caller);
        bool paused = hcuLimit.isQuotaPaused(PARTNER_QUOTA);
        (uint48 defaultMax, , ) = hcuLimit.getQuota(DEFAULT_QUOTA);
        vm.prank(PARTNER_ADMIN);
        if (grant) {
            try hcuConfigurator.grantQuota(PARTNER_QUOTA, caller) {} catch {}
        } else {
            try hcuConfigurator.revokeQuota(PARTNER_QUOTA, caller) {} catch {}
        }
        uint16 afterMove = hcuLimit.getContractQuota(caller);
        if (afterMove == before) return;
        // A grant only succeeds while the default quota is stopped, so joining never gives a contract less room.
        bool allowed = grant
            ? before == DEFAULT_QUOTA && afterMove == PARTNER_QUOTA && defaultMax == 0
            : before == PARTNER_QUOTA && afterMove == DEFAULT_QUOTA;
        if (paused || !allowed) adminMoveViolation = true;
    }

    function setDefaultQuota(uint48 maxHCU, uint48 decayPerBlock) external {
        // A metered quota needs a decay, a stopped one has none.
        if (maxHCU == 0) decayPerBlock = 0;
        else if (decayPerBlock == 0) decayPerBlock = 1;
        vm.prank(owner);
        hcuConfigurator.setQuota(DEFAULT_QUOTA, maxHCU, decayPerBlock);
    }

    /// @dev A fuzzed maximum of 0 leaves the capacity unmetered; otherwise it is at least maxHCUPerTx, which Reserved
    ///      must fit, and drains a quarter of its maximum per block, so a burst can pass a threshold. Tier 1 holds the
    ///      uncapped quota.
    function setCapacityAndTier(uint48 maxHCU, uint16 tierBps) external {
        vm.startPrank(owner);
        if (maxHCU == 0) {
            hcuConfigurator.setCapacity(NO_CAP, NO_CAP);
        } else {
            maxHCU = uint48(bound(uint256(maxHCU), MAX_HCU_PER_TX, uint256(NO_CAP) - 1));
            hcuConfigurator.setCapacity(maxHCU, maxHCU / 4 + 1);
        }
        hcuConfigurator.setTier(1, uint16(bound(uint256(tierBps), 0, MAX_BPS)));
        vm.stopPrank();
    }

    function assignCaller(address caller, bool uncapped) external {
        caller = _sanitizeCaller(caller);
        vm.prank(owner);
        hcuConfigurator.assignQuota(uncapped ? UNCAPPED_QUOTA : DEFAULT_QUOTA, caller);
    }

    function mine(uint8 rawBlocks) external {
        uint256 blocks = bound(uint256(rawBlocks), 1, 8);
        (, uint48 decayPerBlock, uint48 beforeUsedHCU) = hcuLimit.getQuota(DEFAULT_QUOTA);
        vm.roll(block.number + blocks);
        (, , uint48 afterUsedHCU) = hcuLimit.getQuota(DEFAULT_QUOTA);
        uint256 decay = blocks * decayPerBlock;
        uint256 expected = decay >= beforeUsedHCU ? 0 : beforeUsedHCU - decay;
        if (afterUsedHCU != expected) {
            decayViolation = true;
        }
    }

    function callerBurst(address caller, uint8 rawOps) external {
        _runBurst(_sanitizeCaller(caller), bound(uint256(rawOps), 1, 24));
    }

    function defaultBurst(uint8 rawOps) external {
        _runBurst(DEFAULT_CALLER, bound(uint256(rawOps), 1, 24));
    }

    function uncappedBurst(uint8 rawOps) external {
        _runBurst(UNCAPPED_CALLER, bound(uint256(rawOps), 1, 24));
    }

    /**
     * @dev Accounting-focused burst:
     *      - uses `checkHCUForCast` (fixed 32 HCU/op) for deterministic delta checks;
     *      - keeps bursts small on purpose (not a cap-exhaustion stress test).
     */
    function _runBurst(address caller, uint256 ops) internal {
        uint16 quotaId = hcuLimit.getContractQuota(caller);
        bool paused = hcuLimit.isQuotaPaused(quotaId);
        bool inDefault = quotaId == DEFAULT_QUOTA;
        (, , uint48 beforeUsedHCU) = hcuLimit.getQuota(DEFAULT_QUOTA);
        (uint48 capacityMax, , , uint48 loadBefore) = hcuLimit.getCapacity();
        // With the capacity metered, the chain's load can refuse an uncapped quota too.
        bool neverRefused = quotaId == UNCAPPED_QUOTA && !paused && capacityMax == NO_CAP;
        uint256 successes = _castBurst(caller, ops, neverRefused, hcuLimit.getQuotaTier(quotaId), capacityMax);

        // Every admitted operation charges the capacity, whatever its quota, and a refused one charges nothing.
        (, , , uint48 loadAfter) = hcuLimit.getCapacity();
        // While the capacity is unmetered nothing is charged, and getCapacity keeps showing the last stored load.
        uint256 expectedLoad = capacityMax == NO_CAP ? loadBefore : loadBefore + successes * CAST_HCU;
        if (loadAfter != expectedLoad) {
            capacityViolation = true;
        }

        if (paused && successes != 0) {
            pausedViolation = true;
        }

        (, , uint48 afterUsedHCU) = hcuLimit.getQuota(DEFAULT_QUOTA);
        (, , uint48 uncappedUsedHCU) = hcuLimit.getQuota(UNCAPPED_QUOTA);
        if (uncappedUsedHCU != 0) {
            uncappedViolation = true;
        }

        if (afterUsedHCU < beforeUsedHCU) {
            accountingViolation = true;
            return;
        }

        (uint48 defaultMax, , ) = hcuLimit.getQuota(DEFAULT_QUOTA);
        if (inDefault && successes != 0 && afterUsedHCU > defaultMax) {
            accountingViolation = true;
        }
        uint256 expectedDelta = (!inDefault || defaultMax == NO_CAP) ? 0 : successes * CAST_HCU;
        if (uint256(afterUsedHCU - beforeUsedHCU) != expectedDelta) {
            accountingViolation = true;
        }
    }

    /// @dev Whether the meter of Reserved or of any tier down to `tier` holds more than its threshold; false while the
    ///      capacity is unmetered.
    function _aMeterPassesItsThreshold(uint8 tier, uint48 capacityMax) internal view returns (bool) {
        if (capacityMax == NO_CAP) return false;
        for (uint8 k = 0; k <= tier; ++k) {
            (uint16 maxLoadBps, uint48 load) = hcuLimit.getTier(k);
            if (load > (uint256(capacityMax) * maxLoadBps) / MAX_BPS) return true;
        }
        return false;
    }

    /// @dev Runs `ops` casts by `caller` and returns how many succeeded.
    function _castBurst(
        address caller,
        uint256 ops,
        bool neverRefused,
        uint8 tier,
        uint48 capacityMax
    ) internal returns (uint256 successes) {
        vm.startPrank(fhevmExecutor);
        for (uint256 i; i < ops; ++i) {
            try hcuLimit.checkHCUForCast(FheType.Uint8, INPUT_HANDLE, _nextResultHandle(), caller) {} catch {
                if (neverRefused) uncappedViolation = true;
                continue;
            }
            successes++;
            if (_aMeterPassesItsThreshold(tier, capacityMax)) tierViolation = true;
        }
        vm.stopPrank();
    }

    function _sanitizeCaller(address caller) internal pure returns (address) {
        if (caller == address(0)) {
            return address(1);
        }
        if (caller == DEFAULT_CALLER || caller == UNCAPPED_CALLER || caller == PARTNER_ADMIN) {
            return address(uint160(caller) + 2);
        }
        return caller;
    }

    function _nextResultHandle() internal returns (bytes32) {
        resultNonce++;
        return bytes32(resultNonce);
    }
}

contract HCULimitInvariantTest is StdInvariant, HostContractsDeployerTestUtils {
    HCULimit internal hcuLimit;
    HCULimitInvariantHandler internal handler;

    address internal constant owner = address(456);

    function setUp() public {
        // Invariants targeted by this harness:
        // 1) callers in an uncapped quota are never refused or metered.
        // 2) capped accounting is consistent with successful casts and stays within the cap.
        // 3) usage decays by decayPerBlock for each mined block, floored at 0.
        // 4) no operation from a paused quota succeeds, metered or not.
        // 5) a quota admin only moves contracts between the default quota and its quota, never while it is paused.
        // 6) after an operation succeeds, the meters of Reserved down to the caller's tier are within their thresholds.
        // 7) the capacity grows by exactly the HCU of admitted operations, from every quota, paused or refused ones excluded.
        // This harness intentionally validates accounting invariants only, not cap-exhaustion behavior.
        _deployAndEtchACL();

        _deployHCULimit(owner);
        hcuLimit = HCULimit(hcuLimitAdd);
        _deployHCUConfigurator(owner);
        handler = new HCULimitInvariantHandler(hcuLimit, HCUConfigurator(hcuConfiguratorAdd), owner);

        targetContract(address(handler));

        bytes4[] memory selectors = new bytes4[](9);
        selectors[0] = HCULimitInvariantHandler.setDefaultQuota.selector;
        selectors[1] = HCULimitInvariantHandler.defaultBurst.selector;
        selectors[2] = HCULimitInvariantHandler.uncappedBurst.selector;
        selectors[3] = HCULimitInvariantHandler.mine.selector;
        selectors[4] = HCULimitInvariantHandler.assignCaller.selector;
        selectors[5] = HCULimitInvariantHandler.callerBurst.selector;
        selectors[6] = HCULimitInvariantHandler.setPaused.selector;
        selectors[7] = HCULimitInvariantHandler.adminMove.selector;
        selectors[8] = HCULimitInvariantHandler.setCapacityAndTier.selector;

        targetSelector(FuzzSelector({addr: address(handler), selectors: selectors}));
    }

    function invariant_uncappedCallsAreNeverMetered() public view {
        assertFalse(handler.uncappedViolation());
    }

    function invariant_cappedAccountingIsConsistent() public view {
        assertFalse(handler.accountingViolation());
    }

    function invariant_usageDecaysWithMinedBlocks() public view {
        assertFalse(handler.decayViolation());
    }

    function invariant_pausedQuotaAdmitsNoOperation() public view {
        assertFalse(handler.pausedViolation());
    }

    function invariant_adminMovesOnlyBetweenDefaultAndItsQuota() public view {
        assertFalse(handler.adminMoveViolation());
    }

    function invariant_loadStaysWithinTheCallersTier() public view {
        assertFalse(handler.tierViolation());
    }

    function invariant_capacityChargesExactlyTheAdmittedOperations() public view {
        assertFalse(handler.capacityViolation());
    }

    function _deployAndEtchACL() internal {
        address deployedACL = address(new ACL());
        vm.etch(aclAdd, deployedACL.code);

        vm.store(
            aclAdd,
            0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300, // OwnableStorageLocation
            bytes32(uint256(uint160(owner)))
        );
    }
}
