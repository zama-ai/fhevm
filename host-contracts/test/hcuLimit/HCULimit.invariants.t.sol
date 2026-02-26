// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {StdInvariant} from "forge-std/StdInvariant.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";

import {HCULimit} from "../../contracts/HCULimit.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {FheType} from "../../contracts/shared/FheType.sol";
import {aclAdd} from "../../addresses/FHEVMHostAddresses.sol";

contract HCULimitInvariantHandler is Test {
    HCULimit internal immutable hcuLimit;
    address internal immutable owner;
    address internal immutable fhevmExecutor;

    address internal constant NON_WHITELISTED_CALLER = address(0xBEEF);
    address internal constant WHITELISTED_CALLER = address(0xCAFE);
    bytes32 internal constant INPUT_HANDLE = bytes32(uint256(0xA11CE));

    uint256 internal resultNonce = 1;
    bool public whitelistViolation;
    bool public nonWhitelistedAccountingViolation;
    bool public resetViolation;

    constructor(HCULimit _hcuLimit, address _owner) {
        hcuLimit = _hcuLimit;
        owner = _owner;
        fhevmExecutor = _hcuLimit.getFHEVMExecutorAddress();

        vm.startPrank(owner);
        hcuLimit.addToBlockHCUWhitelist(WHITELISTED_CALLER);
        vm.stopPrank();
    }

    function setCap(uint48 cap) external {
        // Must satisfy invariant: hcuPerBlock >= maxHCUPerTx (20_000_000).
        cap = uint48(bound(uint256(cap), 20_000_000, uint256(type(uint48).max)));
        vm.prank(owner);
        hcuLimit.setHCUPerBlock(cap);
    }

    function setCallerWhitelist(address caller, bool isWhitelisted) external {
        caller = _sanitizeCaller(caller);
        bool currentlyWhitelisted = hcuLimit.isBlockHCUWhitelisted(caller);
        if (isWhitelisted == currentlyWhitelisted) return;
        vm.prank(owner);
        if (isWhitelisted) {
            hcuLimit.addToBlockHCUWhitelist(caller);
        } else {
            hcuLimit.removeFromBlockHCUWhitelist(caller);
        }
    }

    function mine(uint8 rawBlocks) external {
        uint256 blocks = bound(uint256(rawBlocks), 1, 8);
        vm.roll(block.number + blocks);
        (, uint48 usedHCU) = hcuLimit.getBlockMeter();
        if (usedHCU != 0) {
            resetViolation = true;
        }
    }

    function callerBurst(address caller, uint8 rawOps) external {
        caller = _sanitizeCaller(caller);
        uint256 ops = bound(uint256(rawOps), 1, 24);
        _runBurst(caller, ops);
    }

    function nonWhitelistedBurst(uint8 rawOps) external {
        uint256 ops = bound(uint256(rawOps), 1, 24);
        _runBurst(NON_WHITELISTED_CALLER, ops);
    }

    function whitelistedBurst(uint8 rawOps) external {
        uint256 ops = bound(uint256(rawOps), 1, 24);
        _runBurst(WHITELISTED_CALLER, ops);
    }

    /**
     * @dev Accounting-focused burst:
     *      - uses `checkHCUForCast` (fixed 32 HCU/op) for deterministic delta checks;
     *      - keeps bursts small on purpose (not a cap-exhaustion stress test).
     */
    function _runBurst(address caller, uint256 ops) internal {
        bool isWhitelisted = hcuLimit.isBlockHCUWhitelisted(caller);
        uint48 cap = hcuLimit.getGlobalHCUCapPerBlock();
        (, uint48 beforeUsedHCU) = hcuLimit.getBlockMeter();
        uint256 successes;

        vm.startPrank(fhevmExecutor);
        for (uint256 i; i < ops; ++i) {
            try hcuLimit.checkHCUForCast(FheType.Uint8, INPUT_HANDLE, _nextResultHandle(), caller) {}
            catch {
                if (isWhitelisted) {
                    whitelistViolation = true;
                }
                continue;
            }
            successes++;
        }
        vm.stopPrank();

        (, uint48 afterUsedHCU) = hcuLimit.getBlockMeter();

        if (afterUsedHCU < beforeUsedHCU) {
            nonWhitelistedAccountingViolation = true;
            return;
        }

        uint256 delta = uint256(afterUsedHCU - beforeUsedHCU);

        if (isWhitelisted) {
            if (delta != 0 || successes != ops) {
                whitelistViolation = true;
            }
            return;
        }

        if (delta != successes * 32) {
            nonWhitelistedAccountingViolation = true;
        }
    }

    function _sanitizeCaller(address caller) internal pure returns (address) {
        if (caller == address(0)) {
            return address(1);
        }
        if (caller == NON_WHITELISTED_CALLER || caller == WHITELISTED_CALLER) {
            return address(uint160(caller) + 2);
        }
        return caller;
    }

    function _nextResultHandle() internal returns (bytes32) {
        resultNonce++;
        return bytes32(resultNonce);
    }
}

contract HCULimitInvariantTest is StdInvariant, Test {
    HCULimit internal hcuLimit;
    HCULimitInvariantHandler internal handler;

    address internal constant owner = address(456);

    function setUp() public {
        // Invariants targeted by this harness:
        // 1) block meter block number equals current block.
        // 2) whitelisted callers never consume the public block meter.
        // 3) non-whitelisted accounting is consistent with successful casts.
        // 4) meter resets after block advancement.
        // This harness intentionally validates accounting invariants only, not cap-exhaustion behavior.
        _deployAndEtchACL();

        address proxy =
            UnsafeUpgrades.deployUUPSProxy(address(new EmptyUUPSProxy()), abi.encodeCall(EmptyUUPSProxy.initialize, ()));

        address implementation = address(new HCULimit());
        vm.startPrank(owner);
        UnsafeUpgrades.upgradeProxy(
            proxy, implementation, abi.encodeCall(HCULimit.initializeFromEmptyProxy, (type(uint48).max, 5_000_000, 20_000_000))
        );
        vm.stopPrank();

        hcuLimit = HCULimit(proxy);
        handler = new HCULimitInvariantHandler(hcuLimit, owner);

        targetContract(address(handler));

        bytes4[] memory selectors = new bytes4[](6);
        selectors[0] = HCULimitInvariantHandler.setCap.selector;
        selectors[1] = HCULimitInvariantHandler.nonWhitelistedBurst.selector;
        selectors[2] = HCULimitInvariantHandler.whitelistedBurst.selector;
        selectors[3] = HCULimitInvariantHandler.mine.selector;
        selectors[4] = HCULimitInvariantHandler.setCallerWhitelist.selector;
        selectors[5] = HCULimitInvariantHandler.callerBurst.selector;

        targetSelector(FuzzSelector({addr: address(handler), selectors: selectors}));
    }

    function invariant_blockMeterMatchesCurrentBlock() public view {
        // Invariant: getBlockMeter must always report the current block number.
        (uint48 blockNumber,) = hcuLimit.getBlockMeter();
        assertEq(blockNumber, uint48(block.number));
    }

    function invariant_whitelistedCallsDoNotConsumePublicMeter() public view {
        // Invariant: calls from whitelisted callers must never increase the public block meter.
        assertFalse(handler.whitelistViolation());
    }

    function invariant_nonWhitelistedAccountingIsConsistent() public view {
        // Invariant: for non-whitelisted callers, meter growth equals successful op count times cast-op cost.
        assertFalse(handler.nonWhitelistedAccountingViolation());
    }

    function invariant_meterResetsAfterMinedBlocks() public view {
        // Invariant: once the block number advances, observed usedHCU for the new block is zero until a new op succeeds.
        assertFalse(handler.resetViolation());
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
