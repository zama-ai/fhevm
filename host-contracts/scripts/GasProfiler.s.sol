// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:´.*/
/*                         GAS PROFILER                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•°.*/
/*  Profiles gas usage for LayerZero's lzReceive & lzCompose    */
/*  methods across multiple runs and suggests options to use.   */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•°.*/
/*--------------------------------------------------------------*/

import "forge-std/Script.sol";
import "forge-std/console.sol";

import { GUID } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/GUID.sol";
import { ILayerZeroComposer } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroComposer.sol";
import { ILayerZeroEndpointV2 } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { ILayerZeroReceiver, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroReceiver.sol";

import { OptionsBuilder } from "@layerzerolabs/oapp-evm/contracts/oapp/libs/OptionsBuilder.sol";

/// @notice Input parameters for running a gas profiling test.
/// @dev Each field is used to define the test environment and inputs.
struct TestParams {
    /// @dev The source endpoint ID.
    uint32 srcEid;
    /// @dev The sender address, encoded as bytes32.
    bytes32 sender;
    /// @dev The destination endpoint ID.
    uint32 dstEid;
    /// @dev The receiver contract address on the destination chain.
    address receiver;
    /// @dev The array of payloads to be tested.
    bytes[] payloads;
    /// @dev The message value (msg.value) sent with each call.
    uint256 msgValue;
    /// @dev The number of runs per payload to gather statistical metrics.
    uint256 numOfRuns;
}

/// @notice Contains gas usage metrics for a single payload test scenario.
/// @dev This includes statistics about gas usage across multiple runs.
struct GasMetrics {
    /// @dev The average gas used across successful runs.
    uint256 averageGas;
    /// @dev The median gas used across successful runs.
    uint256 medianGas;
    /// @dev The maximum gas used in successful runs.
    uint256 maxGas;
    /// @dev The minimum gas used in successful runs.
    uint256 minGas;
    /// @dev The total msg.value sent during the runs.
    uint256 totalMsgValue;
    /// @dev The number of successful runs recorded.
    uint256 successfulRuns;
}

/// @title GasProfilerScript
/// @notice Profiles gas usage for LayerZero's `lzReceive` and `lzCompose` methods over multiple runs,
///         and suggests an enforcedOption based on aggregated metrics.
contract GasProfilerScript is Script {
    using OptionsBuilder for bytes;

    /// @dev Reference to the LayerZero endpoint contract.
    ILayerZeroEndpointV2 public endpoint;

    /// @notice Runs gas profiling for `lzReceive` on a given destination.
    /// @param rpcUrl The RPC URL of the chain to fork.
    /// @param endpointAddress The address of the LayerZero endpoint contract.
    /// @param params The test parameters including payloads, number of runs, etc.
    function run_lzReceive(string memory rpcUrl, address endpointAddress, TestParams memory params) external {
        _initializeEndpoint(endpointAddress);
        console.log("Starting gas profiling for lzReceive on dstEid:", params.dstEid);

        vm.createSelectFork(rpcUrl);

        uint64 nextNonce = ILayerZeroReceiver(params.receiver).nextNonce(params.srcEid, params.sender);

        GasMetrics[] memory metrics = new GasMetrics[](params.payloads.length);

        for (uint256 i = 0; i < params.payloads.length; i++) {
            bytes memory currentPayload = params.payloads[i];
            metrics[i] = _profileSinglePayload(
                params,
                params.receiver,
                abi.encodeWithSelector(
                    ILayerZeroReceiver(params.receiver).lzReceive.selector,
                    Origin(params.srcEid, params.sender, nextNonce),
                    GUID.generate(
                        nextNonce,
                        params.srcEid,
                        address(uint160(uint256(params.sender))),
                        params.dstEid,
                        bytes32(uint256(uint160(params.receiver)))
                    ),
                    currentPayload,
                    address(this),
                    ""
                )
            );
        }

        console.log("---------------------------------------------------------");
        _logAggregatedMetrics(metrics, true, uint128(params.msgValue));
        console.log("---------------------------------------------------------");
        console.log("Finished gas profling for lzReceive on dstEid:", params.dstEid);
        console.log("---------------------------------------------------------");
    }

    /// @notice Runs gas profiling for `lzCompose` on a given destination.
    /// @param rpcUrl The RPC URL of the chain to fork.
    /// @param endpointAddress The address of the LayerZero endpoint contract.
    /// @param composerAddress The address of the LayerZero composer contract.
    /// @param params The test parameters including payloads, number of runs, etc.
    function run_lzCompose(
        string memory rpcUrl,
        address endpointAddress,
        address composerAddress,
        TestParams memory params
    ) external {
        _initializeEndpoint(endpointAddress);
        console.log("Starting gas profiling for lzCompose on dstEid:", params.dstEid);

        vm.createSelectFork(rpcUrl);

        uint64 nextNonce = ILayerZeroReceiver(params.receiver).nextNonce(params.srcEid, params.sender);

        GasMetrics[] memory metrics = new GasMetrics[](params.payloads.length);

        for (uint256 i = 0; i < params.payloads.length; i++) {
            bytes memory currentPayload = params.payloads[i];
            metrics[i] = _profileSinglePayload(
                params,
                composerAddress,
                abi.encodeWithSelector(
                    ILayerZeroComposer(composerAddress).lzCompose.selector,
                    params.receiver,
                    GUID.generate(
                        nextNonce,
                        params.srcEid,
                        address(uint160(uint256(params.sender))),
                        params.dstEid,
                        bytes32(uint256(uint160(params.receiver)))
                    ),
                    currentPayload,
                    address(this),
                    ""
                )
            );
        }

        console.log("---------------------------------------------------------");
        _logAggregatedMetrics(metrics, false, uint128(params.msgValue));
        console.log("---------------------------------------------------------");
        console.log("Finished gas profling for lzCompose on dstEid:", params.dstEid);
        console.log("---------------------------------------------------------");
    }

    /// @notice Initializes the LayerZero endpoint contract.
    /// @param endpointAddress The address of the LayerZero endpoint contract to set.
    function _initializeEndpoint(address endpointAddress) internal {
        endpoint = ILayerZeroEndpointV2(endpointAddress);
    }

    /// @notice Profiles gas usage for a single payload by running the call multiple times.
    /// @dev Reverts state after each run and accumulates gas usage statistics.
    ///
    /// @param params The test parameters including payloads, number of runs, etc.
    /// @param caller The contract address to call (either receiver or composer).
    /// @param callParams The encoded call parameters, including payload and GUID.
    ///
    /// @return metric A GasMetrics struct containing aggregated gas usage data.
    function _profileSinglePayload(
        TestParams memory params,
        address caller,
        bytes memory callParams
    ) internal returns (GasMetrics memory metric) {
        uint256[] memory gasUsedArray = new uint256[](params.numOfRuns);
        uint256 totalGasUsed = 0;
        uint256 successfulRuns = 0;

        vm.deal(address(endpoint), 100 ether);

        uint256 snapshotId = vm.snapshotState();
        for (uint256 i = 0; i < params.numOfRuns; i++) {
            vm.revertToState(snapshotId);

            vm.prank(address(endpoint));
            (bool success, ) = caller.call{ value: params.msgValue }(callParams);
            uint256 gasUsed = vm.lastCallGas().gasTotalUsed;

            if (success) {
                gasUsedArray[successfulRuns] = gasUsed;
                totalGasUsed += gasUsed;
                successfulRuns++;
            }
        }

        if (successfulRuns > 0) {
            metric.averageGas = totalGasUsed / successfulRuns;
            metric.medianGas = _calculateMedian(gasUsedArray, successfulRuns);
            metric.maxGas = _calculateMaximum(gasUsedArray, successfulRuns);
            metric.minGas = _calculateMinimum(gasUsedArray, successfulRuns);
            metric.totalMsgValue = params.msgValue;
            metric.successfulRuns = successfulRuns;
        } else {
            console.log("All runs failed for a payload.");
        }
    }

    /// @notice Aggregates and logs gas metrics across all payloads, suggesting recommended enforcedOptions.
    /// @dev Adds a ~20% overhead to the average gas cost for enforcedOption recommendations.
    ///
    /// @param metrics The array of GasMetrics for each payload tested.
    /// @param isLzReceive A boolean indicating if the function profiled was `lzReceive` (true) or `lzCompose` (false).
    /// @param msgValue The msg.value used during the test runs.
    function _logAggregatedMetrics(GasMetrics[] memory metrics, bool isLzReceive, uint128 msgValue) internal pure {
        uint256 totalAverageGas = 0;
        uint256 overallMinGas = type(uint256).max;
        uint256 overallMaxGas = 0;
        uint256 totalSuccessfulRuns = 0;
        uint256 countedMetrics = 0;

        for (uint256 i = 0; i < metrics.length; i++) {
            GasMetrics memory metric = metrics[i];
            if (metric.successfulRuns == 0) {
                continue;
            }
            totalAverageGas += metric.averageGas;

            if (metric.minGas < overallMinGas) {
                overallMinGas = metric.minGas;
            }
            if (metric.maxGas > overallMaxGas) {
                overallMaxGas = metric.maxGas;
            }
            totalSuccessfulRuns += metric.successfulRuns;
            countedMetrics++;
        }

        if (countedMetrics > 0) {
            uint256 overallAverageGas = totalAverageGas / countedMetrics;
            console.log("Aggregated Gas Metrics Across All Payloads:");
            console.log("Overall Average Gas Used:", overallAverageGas);
            console.log("Overall Minimum Gas Used:", overallMinGas);
            console.log("Overall Maximum Gas Used:", overallMaxGas);

            // Heuristic: Add ~20% overhead of max gas to the recommended gas limit.
            uint128 recommendedGasLimit = uint128((overallMaxGas * 120) / 100);

            bytes memory recommendedEnforcedOption;
            if (isLzReceive) {
                recommendedEnforcedOption = OptionsBuilder.newOptions().addExecutorLzReceiveOption(
                    recommendedGasLimit,
                    msgValue
                );
            } else {
                recommendedEnforcedOption = OptionsBuilder.newOptions().addExecutorLzComposeOption(
                    0,
                    recommendedGasLimit,
                    msgValue
                );
            }

            console.log("Estimated options:");
            console.logBytes(recommendedEnforcedOption);
        } else {
            console.log("No successful runs to aggregate metrics.");
        }
    }

    /// @notice Calculates the median gas usage from an array of gas usage values.
    /// @param array The array of gas values.
    /// @param length The number of valid entries in the array.
    ///
    /// @return The median gas usage value.
    function _calculateMedian(uint256[] memory array, uint256 length) internal pure returns (uint256) {
        _sortArray(array, length);
        if (length % 2 == 1) {
            return array[length / 2];
        } else {
            return (array[length / 2 - 1] + array[length / 2]) / 2;
        }
    }

    /// @notice Calculates the maximum gas usage from an array of gas usage values.
    /// @param array The array of gas values.
    /// @param length The number of valid entries in the array.
    ///
    /// @return The maximum gas usage value.
    function _calculateMaximum(uint256[] memory array, uint256 length) internal pure returns (uint256) {
        uint256 maxVal = 0;
        for (uint256 i = 0; i < length; i++) {
            if (array[i] > maxVal) {
                maxVal = array[i];
            }
        }
        return maxVal;
    }

    /// @notice Calculates the minimum gas usage from an array of gas usage values.
    /// @param array The array of gas values.
    /// @param length The number of valid entries in the array.
    ///
    /// @return The minimum gas usage value.
    function _calculateMinimum(uint256[] memory array, uint256 length) internal pure returns (uint256) {
        uint256 minVal = type(uint256).max;
        for (uint256 i = 0; i < length; i++) {
            if (array[i] < minVal) {
                minVal = array[i];
            }
        }
        return minVal;
    }

    /// @notice Sorts an array of uint256 values in ascending order.
    /// @dev Uses an insertion sort algorithm for simplicity.
    /// @param array The array of gas usage values.
    /// @param length The number of valid entries in the array.
    function _sortArray(uint256[] memory array, uint256 length) internal pure {
        for (uint256 i = 1; i < length; i++) {
            uint256 key = array[i];
            uint256 j = i;
            while (j > 0 && array[j - 1] > key) {
                array[j] = array[j - 1];
                j--;
            }
            array[j] = key;
        }
    }
}