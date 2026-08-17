// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "forge-std/Script.sol";
import "forge-std/console.sol";

import {TestParams} from "@layerzerolabs/script-devtools-evm-foundry/scripts/GasProfiling/GasProfiler.s.sol";

import {HandlesReceiverGasProfiler, CalibParams} from "./HandlesReceiverProfiler.s.sol";

/// @notice Inputs for an exhaustive budget-coverage check of the fitted `lzReceive` gas formula.
struct VerifyParams {
    /// @dev RPC URL of the destination chain to fork (where the receiver bridge lives).
    string rpcUrl;
    /// @dev LayerZero EndpointV2 on the destination chain.
    address endpointAddress;
    /// @dev Destination-chain ConfidentialBridge (the OApp whose `lzReceive` we profile).
    address receiver;
    /// @dev Source peer (the remote ConfidentialBridge), as bytes32. MUST match the peer the
    ///      destination bridge registered for `srcEid`, or `lzReceive` reverts.
    bytes32 sender;
    /// @dev Source endpoint id (the peer's eid).
    uint32 srcEid;
    /// @dev Destination endpoint id (this chain).
    uint32 dstEid;
    /// @dev Inclusive handle-count range to sweep (every integer is checked).
    uint256 minHandles;
    uint256 maxHandles;
    /// @dev Inclusive payload-byte range to sweep, walked in `payloadStep` increments. The
    ///      `maxPayloadLen` boundary is always checked even if the step does not land on it.
    uint256 minPayloadLen;
    uint256 maxPayloadLen;
    uint256 payloadStep;
    /// @dev Runs per grid point. The measurement is deterministic (each run reverts to the same
    ///      cold snapshot), so 1 is sufficient; raise only to double-check stability.
    uint256 numOfRuns;
    /// @dev The fitted formula being validated: budget = base + perHandle*n + perByte*len.
    ///      Use the FINAL (margin-included) values you intend to deploy.
    uint256 base;
    uint256 perHandle;
    uint256 perByte;
}

/// @title HandlesReceiverBudgetVerifier
/// @notice Companion to {HandlesReceiverGasProfiler}: confirms a fitted three-coefficient
///         `lzReceive` gas formula (`base + perHandle*nHandles + perByte*payloadLen`) is an
///         UPPER BOUND on real measured gas for every `(nHandles, payloadLen)` in a range, by
///         measuring each grid point through the same real `EndpointV2.lzReceive` path used to
///         calibrate (so the check pays the identical production envelope: `_clearPayload`
///         bookkeeping + the bridge's `lzReceive`).
///
/// @dev    Why a separate sweep is needed: the profiler fits coefficients from a COARSE grid.
///         `lzReceive` gas is monotonically non-decreasing in both `nHandles` and `payloadLen`,
///         and convex in `payloadLen` (memory-expansion cost grows super-linearly). For a convex
///         measured surface and a linear budget, `budget - measured` is concave, so its minimum
///         over any cell is attained at a sampled point -- which is why checking a fine grid is a
///         strong guarantee. To make the guarantee exhaustive set `payloadStep = 1` (slow).
contract HandlesReceiverBudgetVerifier is HandlesReceiverGasProfiler {
    /// @dev Running coverage stats accumulated across the sweep (kept in one memory struct to
    ///      stay clear of "stack too deep").
    struct VerifyState {
        uint256 cells;
        uint256 violations;
        // Worst (smallest) signed slack = budget - measured, and where it occurred.
        int256 minSlack;
        uint256 minSlackN;
        uint256 minSlackLen;
        uint256 minSlackMeasured;
        uint256 minSlackBudget;
        // First under-budget cell encountered.
        uint256 firstBadN;
        uint256 firstBadLen;
        uint256 firstBadMeasured;
        uint256 firstBadBudget;
    }

    function run_verifyBudget(VerifyParams memory v) external {
        require(v.maxHandles >= v.minHandles && v.minHandles >= 1, "bad handle range");
        require(v.maxPayloadLen >= v.minPayloadLen, "bad payload range");
        require(v.payloadStep >= 1, "payloadStep must be >=1");

        _initializeEndpoint(v.endpointAddress);
        _logHeader(v);

        vm.createSelectFork(v.rpcUrl);

        VerifyState memory s = _sweep(v);

        _logResult(s);
    }

    /// @dev Walks the full `(nHandles, payloadLen)` grid, measuring each cell through the real
    ///      endpoint and comparing against the fitted budget.
    function _sweep(VerifyParams memory v) private returns (VerifyState memory s) {
        // _measureCell only reads sender/srcEid/dstEid/receiver from CalibParams.
        CalibParams memory cp;
        cp.endpointAddress = v.endpointAddress;
        cp.receiver = v.receiver;
        cp.sender = v.sender;
        cp.srcEid = v.srcEid;
        cp.dstEid = v.dstEid;

        TestParams memory tp;
        tp.numOfRuns = v.numOfRuns == 0 ? 1 : v.numOfRuns;
        tp.msgValue = 0;

        uint64 nonce = endpoint.inboundNonce(v.receiver, v.srcEid, v.sender) + 1;
        uint256 coldSlotsSnapshotId = vm.snapshotState();

        s.minSlack = type(int256).max;

        // Snapshot the free-memory pointer once everything that must survive the sweep (v, cp,
        // tp, s) is allocated. Each cell allocates a fresh message/callParams buffer (up to
        // ~10KB) that Solidity never frees; over thousands of cells the free pointer would climb
        // and memory-expansion gas would grow quadratically -> MemoryOOG. We reclaim that scratch
        // by restoring the pointer after recording each cell. Safe because nothing live lives
        // above `freePtr`: measured gas is already copied to the stack, and `_record` only mutates
        // the pre-existing `s`.
        uint256 freePtr;
        assembly ("memory-safe") {
            freePtr := mload(0x40)
        }

        for (uint256 n = v.minHandles; n <= v.maxHandles; n++) {
            uint256 len = v.minPayloadLen;
            while (true) {
                uint256 measured = _measureCell(cp, tp, n, len, nonce, coldSlotsSnapshotId);
                uint256 budget = v.base + v.perHandle * n + v.perByte * len;
                _record(s, n, len, measured, budget);

                assembly ("memory-safe") {
                    mstore(0x40, freePtr)
                }

                if (len == v.maxPayloadLen) break;
                uint256 next = len + v.payloadStep;
                len = next > v.maxPayloadLen ? v.maxPayloadLen : next;
            }
        }
    }

    function _record(VerifyState memory s, uint256 n, uint256 len, uint256 measured, uint256 budget) private pure {
        s.cells++;

        int256 slack = int256(budget) - int256(measured);
        if (slack < s.minSlack) {
            s.minSlack = slack;
            s.minSlackN = n;
            s.minSlackLen = len;
            s.minSlackMeasured = measured;
            s.minSlackBudget = budget;
        }

        if (budget < measured) {
            if (s.violations == 0) {
                s.firstBadN = n;
                s.firstBadLen = len;
                s.firstBadMeasured = measured;
                s.firstBadBudget = budget;
            }
            s.violations++;
        }
    }

    function _logHeader(VerifyParams memory v) private pure {
        console.log("=========================================================");
        console.log("Verifying lzReceive budget on dstEid:", v.dstEid);
        console.log("receiver:", v.receiver);
        console.log("formula: base + perHandle*n + perByte*len");
        console.log("base     :", v.base);
        console.log("perHandle:", v.perHandle);
        console.log("perByte  :", v.perByte);
        console.log("nHandles in:", v.minHandles, "..", v.maxHandles);
        console.log("payloadLen in:", v.minPayloadLen, "..", v.maxPayloadLen);
        console.log("payloadStep:", v.payloadStep);
        console.log("---------------------------------------------------------");
    }

    function _logResult(VerifyState memory s) private pure {
        console.log("---------------------------------------------------------");
        console.log("cells checked :", s.cells);
        console.log("violations    :", s.violations);
        console.log("tightest cell  n / len:", s.minSlackN, s.minSlackLen);
        console.log("   measured / budget  :", s.minSlackMeasured, s.minSlackBudget);
        if (s.minSlack < 0) {
            console.log("   slack: NEGATIVE by", uint256(-s.minSlack));
        } else {
            console.log("   slack: +", uint256(s.minSlack));
        }

        if (s.violations == 0) {
            console.log("RESULT: PASS - formula upper-bounds every measured cell.");
        } else {
            console.log("RESULT: FAIL - first under-budget cell  n / len:", s.firstBadN, s.firstBadLen);
            console.log("   measured / budget:", s.firstBadMeasured, s.firstBadBudget);
            // Surface a clear non-zero exit for CI.
            revert("budget insufficient: see FAIL log above");
        }
    }
}

/// @title HandlesReceiverBudgetVerifierExample
/// @notice Entry point for verifying the fitted `lzReceive` coefficients cover the full
///         `(nHandles in [1,32], payloadLen in [0,10000])` domain.
///
/// @dev    Run (env-driven), reusing the same wiring vars as the profiler plus the three fitted
///         coefficients you obtained (use the margin-included RECOMMENDED values):
///
///           PROFILE_RPC_URL=<dst rpc> \
///           PROFILE_RECEIVER=<dst ConfidentialBridge> \
///           PROFILE_SENDER=<src ConfidentialBridge> \
///           PROFILE_SRC_EID=40267 PROFILE_DST_EID=40161 \
///           VERIFY_BASE_GAS=<base> \
///           VERIFY_PER_HANDLE_GAS=<perHandle> \
///           VERIFY_PER_PAYLOAD_BYTE_GAS=<perByte> \
///           forge script scripts/HandlesReceiverBudgetVerifier.s.sol:HandlesReceiverBudgetVerifierExample -vv
///
///         Optional knobs: VERIFY_MIN_HANDLES (1), VERIFY_MAX_HANDLES (32),
///         VERIFY_MIN_PAYLOAD (0), VERIFY_MAX_PAYLOAD (10000), VERIFY_PAYLOAD_STEP (100),
///         PROFILE_RUNS (1). Set VERIFY_PAYLOAD_STEP=1 for a fully exhaustive (slow) sweep.
contract HandlesReceiverBudgetVerifierExample is Script {
    HandlesReceiverBudgetVerifier public verifier;

    /// @dev Canonical LayerZero V2 endpoint on (most) testnets. Mainnet differs per chain.
    address internal constant TESTNET_ENDPOINT = 0x6EDCE65403992e310A62460808c4b910D972f10f;

    function run() external {
        verifier = new HandlesReceiverBudgetVerifier();

        VerifyParams memory v = VerifyParams({
            rpcUrl: vm.envString("PROFILE_RPC_URL"),
            endpointAddress: vm.envOr("PROFILE_ENDPOINT", TESTNET_ENDPOINT),
            receiver: vm.envAddress("PROFILE_RECEIVER"),
            sender: bytes32(uint256(uint160(vm.envAddress("PROFILE_SENDER")))),
            srcEid: uint32(vm.envUint("PROFILE_SRC_EID")),
            dstEid: uint32(vm.envUint("PROFILE_DST_EID")),
            minHandles: vm.envOr("VERIFY_MIN_HANDLES", uint256(1)),
            maxHandles: vm.envOr("VERIFY_MAX_HANDLES", uint256(32)),
            minPayloadLen: vm.envOr("VERIFY_MIN_PAYLOAD", uint256(0)),
            maxPayloadLen: vm.envOr("VERIFY_MAX_PAYLOAD", uint256(10000)),
            payloadStep: vm.envOr("VERIFY_PAYLOAD_STEP", uint256(100)),
            numOfRuns: vm.envOr("PROFILE_RUNS", uint256(1)),
            base: vm.envUint("VERIFY_BASE_GAS"),
            perHandle: vm.envUint("VERIFY_PER_HANDLE_GAS"),
            perByte: vm.envUint("VERIFY_PER_PAYLOAD_BYTE_GAS")
        });

        verifier.run_verifyBudget(v);
    }
}
