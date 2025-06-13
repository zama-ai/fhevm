// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract CoprocessorContextsMock {
    event InitCoprocessorContext(uint256 featureSet, Coprocessor[] coprocessors);

    event NewCoprocessorContext(
        CoprocessorContext activeCoprocessorContext,
        CoprocessorContext newCoprocessorContext,
        CoprocessorContextBlockPeriods blockPeriods
    );

    event PreActivateCoprocessorContext(CoprocessorContext newCoprocessorContext, uint256 preActivationBlockNumber);

    event ActivateCoprocessorContext(uint256 contextId);

    event SuspendCoprocessorContext(uint256 contextId, uint256 suspendedBlockNumber);

    event CompromiseCoprocessorContext(uint256 contextId);

    event DeactivateCoprocessorContext(uint256 contextId);

    event DestroyCoprocessorContext(uint256 contextId);

    uint256 coprocessorContextCount;

    function initialize(uint256 initialFeatureSet, Coprocessor[] calldata initialCoprocessors) public {
        uint256 featureSet;
        Coprocessor[] memory coprocessors = new Coprocessor[](1);

        emit InitCoprocessorContext(featureSet, coprocessors);
    }

    function addCoprocessorContext(
        uint256 featureSet,
        CoprocessorContextBlockPeriods calldata blockPeriods,
        Coprocessor[] calldata coprocessors
    ) external {
        CoprocessorContext memory activeCoprocessorContext;
        CoprocessorContext memory newCoprocessorContext;
        uint256 preActivationBlockNumber;

        emit NewCoprocessorContext(activeCoprocessorContext, newCoprocessorContext, blockPeriods);

        emit PreActivateCoprocessorContext(newCoprocessorContext, preActivationBlockNumber);
    }

    function refreshCoprocessorContextStatuses() external {
        uint256 contextId;
        uint256 suspendedBlockNumber;

        emit SuspendCoprocessorContext(contextId, suspendedBlockNumber);

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
