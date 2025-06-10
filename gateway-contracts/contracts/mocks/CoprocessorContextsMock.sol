// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract CoprocessorContextsMock {
    event InitializeCoprocessorContexts(uint256 featureSet, CoprocessorV2[] coprocessors);

    event NewCoprocessorContext(
        CoprocessorContext activeCoprocessorContext,
        CoprocessorContext newCoprocessorContext,
        CoprocessorContextTimePeriods timePeriods
    );

    event PreActivateCoprocessorContext(CoprocessorContext newCoprocessorContext, uint256 activationBlockTimestamp);

    event SuspendCoprocessorContext(uint256 contextId, uint256 deactivatedBlockTimestamp);

    event ActivateCoprocessorContext(uint256 contextId);

    event DeactivateCoprocessorContext(uint256 contextId);

    event CompromiseCoprocessorContext(uint256 contextId);

    event DestroyCoprocessorContext(uint256 contextId);

    uint256 coprocessorContextCount = 0;

    function initializeFromEmptyProxy(uint256 initialFeatureSet, CoprocessorV2[] calldata initialCoprocessors) public {
        uint256 featureSet;
        CoprocessorV2[] memory coprocessors = new CoprocessorV2[](1);

        emit InitializeCoprocessorContexts(featureSet, coprocessors);
    }

    function addCoprocessorContext(
        uint256 featureSet,
        CoprocessorV2[] calldata coprocessors,
        CoprocessorContextTimePeriods calldata timePeriods
    ) external {
        CoprocessorContext memory activeCoprocessorContext;
        CoprocessorContext memory newCoprocessorContext;
        uint256 activationBlockTimestamp;

        emit NewCoprocessorContext(activeCoprocessorContext, newCoprocessorContext, timePeriods);

        emit PreActivateCoprocessorContext(newCoprocessorContext, activationBlockTimestamp);
    }

    function refreshCoprocessorContextStatuses() external {
        uint256 contextId;
        uint256 deactivatedBlockTimestamp;

        emit SuspendCoprocessorContext(contextId, deactivatedBlockTimestamp);

        emit ActivateCoprocessorContext(contextId);

        emit DeactivateCoprocessorContext(contextId);
    }

    function forceUpdateCoprocessorContextToStatus(uint256 contextId, ContextStatus status) external {
        uint256 deactivatedBlockTimestamp;

        emit SuspendCoprocessorContext(contextId, deactivatedBlockTimestamp);

        emit ActivateCoprocessorContext(contextId);

        emit DeactivateCoprocessorContext(contextId);

        emit CompromiseCoprocessorContext(contextId);

        emit DestroyCoprocessorContext(contextId);
    }

    function swapSuspendedCoprocessorContextWithActive(uint256 suspendedTimePeriod) external {
        uint256 contextId;
        uint256 deactivatedBlockTimestamp;

        emit SuspendCoprocessorContext(contextId, deactivatedBlockTimestamp);

        emit ActivateCoprocessorContext(contextId);
    }
}
