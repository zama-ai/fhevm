// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:´.*/
/*              HANDLES-RECEIVER lzReceive PROFILER             */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•°.*/
/*  Forks a real network and measures the *actual* execution    */
/*  gas of the ConfidentialBridge `lzReceive` leg, then fits     */
/*  the HandlesSender formula (base / perHandle / perByte).      */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:°.´:•˚°.*°.˚:*.´+°.•°.*/

import "forge-std/Script.sol";
import "forge-std/console.sol";

import {GUID} from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/GUID.sol";
import {ILayerZeroReceiver, Origin} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroReceiver.sol";

import {
    GasProfilerScript,
    TestParams,
    GasMetrics
} from "@layerzerolabs/script-devtools-evm-foundry/scripts/GasProfiling/GasProfiler.s.sol";

/// @notice Per-target inputs for an `lzReceive` calibration run.
struct CalibParams {
    /// @dev RPC URL of the destination chain to fork (where the receiver bridge lives).
    string rpcUrl;
    /// @dev LayerZero EndpointV2 on the destination chain.
    address endpointAddress;
    /// @dev Destination-chain ConfidentialBridge (the OApp whose `lzReceive` we profile).
    address receiver;
    /// @dev Source peer (the remote ConfidentialBridge), as bytes32. MUST equal the peer
    ///      the destination bridge has registered for `srcEid`, or `lzReceive` reverts.
    bytes32 sender;
    /// @dev Source endpoint id (the peer's eid).
    uint32 srcEid;
    /// @dev Destination endpoint id (this chain).
    uint32 dstEid;
    /// @dev Handle counts to sweep, ascending; MUST include the protocol cap (MAX_HANDLES).
    uint256[] handleCounts;
    /// @dev Payload byte-lengths to sweep, ascending.
    uint256[] payloadLens;
    /// @dev Runs per grid point (median/max are taken across runs).
    uint256 numOfRuns;
    /// @dev Safety margin applied to the fitted coefficients, in basis points (2000 = +20%).
    uint256 marginBps;
}

/// @title HandlesReceiverGasProfiler
/// @notice Calibration engine: reuses {GasProfilerScript}'s fork + measurement primitive
///         (`_profileSinglePayload`, which pranks the endpoint and reads
///         `vm.lastCallGas()`), sweeps a (nHandles, payloadLen) grid in the bridge's
///         wire format, and fits `base + perHandle*n + perByte*len`.
contract HandlesReceiverGasProfiler is GasProfilerScript {
    /// @dev Measured gas grid, _grid[handleIdx][payloadIdx] = max gas across runs.
    uint256[][] private _grid;

    function run_calibrateLzReceive(CalibParams memory p) external {
        require(p.handleCounts.length >= 2, "need >=2 handle counts");
        require(p.payloadLens.length >= 2, "need >=2 payload lengths");

        _initializeEndpoint(p.endpointAddress);

        console.log("=========================================================");
        console.log("Calibrating lzReceive on dstEid:", p.dstEid);
        console.log("receiver:", p.receiver);
        console.log("endpoint:", p.endpointAddress);

        vm.createSelectFork(p.rpcUrl);

        _measureLzReceiveGrid(p);
        _reportAndFit(p.handleCounts, p.payloadLens, p.marginBps);
    }

    /// @dev Sweeps the grid, measuring each (nHandles, payloadLen) `lzReceive` on the fork.
    function _measureLzReceiveGrid(CalibParams memory p) private {
        delete _grid;

        // _profileSinglePayload only reads numOfRuns + msgValue from TestParams.
        TestParams memory tp;
        tp.numOfRuns = p.numOfRuns;
        tp.msgValue = 0;

        uint64 nonce = endpoint.inboundNonce(p.receiver, p.srcEid, p.sender) + 1;

        // Snapshot the pristine, fully-cold fork state. We revert to it before every cell so
        // each lzReceive pays EIP-2929 cold access (production runs each delivery as its own
        // transaction). vm.revertToState restores the warm/cold access list AND all contract
        // storage -- including this profiler's own `_grid`. So results MUST be accumulated in
        // memory (which snapshots do not revert) and only written to storage after the last
        // revert; writing `_grid` inside the loop would be wiped by the next revert.
        uint256 coldSlotsSnapshotId = vm.snapshotState();

        uint256 nH = p.handleCounts.length;
        uint256 nL = p.payloadLens.length;
        uint256[][] memory grid = new uint256[][](nH);
        for (uint256 i = 0; i < nH; i++) {
            grid[i] = new uint256[](nL);
            for (uint256 j = 0; j < nL; j++) {
                grid[i][j] = _measureCell(p, tp, p.handleCounts[i], p.payloadLens[j], nonce, coldSlotsSnapshotId);
                nonce++;
            }
        }

        // All reverts are done; persist the memory grid to storage for the fit/report step.
        for (uint256 i = 0; i < nH; i++) {
            _grid.push();
            for (uint256 j = 0; j < nL; j++) {
                _grid[i].push(grid[i][j]);
            }
        }
    }

    /// @dev Reverts to the cold-slot baseline, then measures one (nHandles, payloadLen) cell.
    ///      Extracted to keep `_measureLzReceiveGrid` under the stack-slot limit.
    function _measureCell(
        CalibParams memory p,
        TestParams memory tp,
        uint256 nHandles,
        uint256 payloadLen,
        uint64 nonce,
        uint256 coldSlotsSnapshotId
    ) private returns (uint256) {
        vm.revertToState(coldSlotsSnapshotId);
        bytes memory callParams = abi.encodeWithSelector(
            ILayerZeroReceiver.lzReceive.selector,
            Origin(p.srcEid, p.sender, nonce),
            GUID.generate(
                nonce,
                p.srcEid,
                address(uint160(uint256(p.sender))),
                p.dstEid,
                bytes32(uint256(uint160(p.receiver)))
            ),
            _encodeReceiveMessage(p.sender, nHandles, payloadLen),
            address(this),
            bytes("")
        );
        GasMetrics memory m = _profileSinglePayload(tp, p.receiver, callParams);
        require(m.successfulRuns > 0, "lzReceive reverted on fork (check receiver/sender/peer wiring & eids)");
        return m.maxGas;
    }

    /// @dev Bridge wire format: abi.encode(srcApp, dstApp, payload, srcHandleList).
    ///      `dstApp` is never called during `lzReceive` (only in `lzCompose`), so any
    ///      address works for gas purposes.
    function _encodeReceiveMessage(
        bytes32 sender,
        uint256 nHandles,
        uint256 payloadLen
    ) private pure returns (bytes memory) {
        bytes32[] memory handleList = new bytes32[](nHandles);
        for (uint256 k = 0; k < nHandles; k++) {
            bytes32 h = keccak256(abi.encodePacked("profiler-handle", k));
            handleList[k] = h;
        }
        bytes memory payload = new bytes(payloadLen);
        return abi.encode(sender, sender, payload, handleList);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Fit + report
    ////////////////////////////////////////////////////////////////////////////////

    function _reportAndFit(
        uint256[] memory handleCounts,
        uint256[] memory payloadLens,
        uint256 marginBps
    ) private view {
        _logGrid(handleCounts, payloadLens);

        uint256 perHandle = _fitPerHandle(handleCounts);
        uint256 perByte = _fitPerByte(payloadLens);
        uint256 base = _fitBase(handleCounts, payloadLens, perHandle, perByte);

        uint256 m = 10_000 + marginBps;
        console.log("--- Raw fitted coefficients (no margin) ---");
        console.log("base     :", base);
        console.log("perHandle:", perHandle);
        console.log("perByte  :", perByte);
        console.log("--- RECOMMENDED (margin bps =", marginBps, ") ---");
        console.log("LZ_RECEIVE_BASE_GAS            =", _ceilDiv(base * m, 10_000));
        console.log("LZ_RECEIVE_PER_HANDLE_GAS      =", _ceilDiv(perHandle * m, 10_000));
        console.log("LZ_RECEIVE_PER_PAYLOAD_BYTE_GAS=", _ceilDiv(perByte * m, 10_000));

        // Reference: recommended budget vs measured at the worst (last,last) corner.
        uint256 iLast = handleCounts.length - 1;
        uint256 jLast = payloadLens.length - 1;
        uint256 recBudget = _ceilDiv(base * m, 10_000) +
            _ceilDiv(perHandle * m, 10_000) *
            handleCounts[iLast] +
            _ceilDiv(perByte * m, 10_000) *
            payloadLens[jLast];
        console.log("worst measured (nMax,lMax)  :", _grid[iLast][jLast]);
        console.log("recommended budget @ corner :", recBudget);
    }

    function _logGrid(uint256[] memory handleCounts, uint256[] memory payloadLens) private view {
        console.log("--- lzReceive gas grid (max across runs) ---");
        for (uint256 i = 0; i < handleCounts.length; i++) {
            string memory row = string.concat("n=", vm.toString(handleCounts[i]), ":");
            for (uint256 j = 0; j < payloadLens.length; j++) {
                row = string.concat(row, " [len=", vm.toString(payloadLens[j]), "]=", vm.toString(_grid[i][j]));
            }
            console.log(row);
        }
    }

    /// @dev Steepest top-segment per-handle slope (last two handle counts), max over payloads.
    function _fitPerHandle(uint256[] memory handleCounts) private view returns (uint256 perHandle) {
        uint256 iLast = handleCounts.length - 1;
        uint256 dN = handleCounts[iLast] - handleCounts[iLast - 1];
        for (uint256 j = 0; j < _grid[0].length; j++) {
            uint256 hi = _grid[iLast][j];
            uint256 lo = _grid[iLast - 1][j];
            uint256 slope = hi > lo ? _ceilDiv(hi - lo, dN) : 0;
            if (slope > perHandle) perHandle = slope;
        }
    }

    /// @dev Steepest top-segment per-byte slope (last two payload lengths), max over handle counts.
    function _fitPerByte(uint256[] memory payloadLens) private view returns (uint256 perByte) {
        uint256 jLast = payloadLens.length - 1;
        uint256 dL = payloadLens[jLast] - payloadLens[jLast - 1];
        for (uint256 i = 0; i < _grid.length; i++) {
            uint256 hi = _grid[i][jLast];
            uint256 lo = _grid[i][jLast - 1];
            uint256 slope = hi > lo ? _ceilDiv(hi - lo, dL) : 0;
            if (slope > perByte) perByte = slope;
        }
    }

    /// @dev Smallest base making `base + perHandle*n + perByte*len` cover every grid point.
    function _fitBase(
        uint256[] memory handleCounts,
        uint256[] memory payloadLens,
        uint256 perHandle,
        uint256 perByte
    ) private view returns (uint256 base) {
        for (uint256 i = 0; i < handleCounts.length; i++) {
            for (uint256 j = 0; j < payloadLens.length; j++) {
                uint256 variable = perHandle * handleCounts[i] + perByte * payloadLens[j];
                uint256 intercept = _grid[i][j] > variable ? _grid[i][j] - variable : 0;
                if (intercept > base) base = intercept;
            }
        }
    }

    function _ceilDiv(uint256 a, uint256 b) private pure returns (uint256) {
        return b == 0 ? 0 : (a + b - 1) / b;
    }
}

/// @title HandlesReceiverProfilerExample
/// @notice Entry point for the `lzReceive` calibration. Mirrors {OFTProfilerExample}.
///
/// @dev    Fill in your deployed bridge addresses (per {LIVE_TESTNET_BRIDGE_RUNBOOK}).
///         The destination bridge MUST already have the source peer registered for
///         `srcEid` (wired via `lz:oapp:wire`), or `lzReceive` reverts the peer check.
///
///         Run (env-driven):
///           PROFILE_RPC_URL=<dst rpc> \
///           PROFILE_RECEIVER=<dst ConfidentialBridge> \
///           PROFILE_SENDER=<src ConfidentialBridge> \
///           PROFILE_SRC_EID=40267 PROFILE_DST_EID=40161 \
///           forge script test/bridge/HandlesReceiverProfiler.s.sol:HandlesReceiverProfilerExample -vv
contract HandlesReceiverProfilerExample is Script {
    HandlesReceiverGasProfiler public profiler;

    /// @dev Canonical LayerZero V2 endpoint on (most) testnets. Mainnet differs per chain.
    address internal constant TESTNET_ENDPOINT = 0x6EDCE65403992e310A62460808c4b910D972f10f;

    function run() external {
        profiler = new HandlesReceiverGasProfiler();

        // Handle-count sweep MUST include MAX_HANDLES (32). Payload sweep spans empty to a
        // large app payload. Ascending order is required by the fitter.
        uint256[] memory handleCounts = new uint256[](4);
        handleCounts[0] = 1;
        handleCounts[1] = 8;
        handleCounts[2] = 16;
        handleCounts[3] = 32;

        uint256[] memory payloadLens = new uint256[](5);
        payloadLens[0] = 0;
        payloadLens[1] = 256;
        payloadLens[2] = 1024;
        payloadLens[3] = 4096;
        payloadLens[4] = 10000;

        CalibParams memory p = CalibParams({
            rpcUrl: vm.envString("PROFILE_RPC_URL"),
            endpointAddress: vm.envOr("PROFILE_ENDPOINT", TESTNET_ENDPOINT),
            receiver: vm.envAddress("PROFILE_RECEIVER"),
            sender: bytes32(uint256(uint160(vm.envAddress("PROFILE_SENDER")))),
            srcEid: uint32(vm.envUint("PROFILE_SRC_EID")),
            dstEid: uint32(vm.envUint("PROFILE_DST_EID")),
            handleCounts: handleCounts,
            payloadLens: payloadLens,
            numOfRuns: vm.envOr("PROFILE_RUNS", uint256(5)),
            marginBps: vm.envOr("PROFILE_MARGIN_BPS", uint256(3000))
        });

        profiler.run_calibrateLzReceive(p);
    }
}
