// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract CoprocessorContextsMock {
    event InitializeCoprocessorContexts(uint256 featureSet, Coprocessor[] coprocessors);

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

    uint256 coprocessorContextCount;

    function initializeFromEmptyProxy(uint256 initialFeatureSet, Coprocessor[] calldata initialCoprocessors) public {
        uint256 featureSet;
        Coprocessor[] memory coprocessors = new Coprocessor[](1);

        emit InitializeCoprocessorContexts(featureSet, coprocessors);
    }

    function addCoprocessorContext(
        uint256 featureSet,
        Coprocessor[] calldata coprocessors,
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

    function compromiseCoprocessorContext(uint256 contextId) external {
        emit CompromiseCoprocessorContext(contextId);
    }

    function destroyCoprocessorContext(uint256 contextId) external {
        emit DestroyCoprocessorContext(contextId);
    }

    function moveSuspendedCoprocessorContextToActive() external {
        uint256 contextId;

        emit DeactivateCoprocessorContext(contextId);

        emit ActivateCoprocessorContext(contextId);
    }
}
