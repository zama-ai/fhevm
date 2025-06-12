// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract CoprocessorContextsMock {
    event InitCoprocessorContext(
        string featureSet,
        CoprocessorContextBlockPeriods contextBlockPeriods,
        Coprocessor[] coprocessors
    );

    event UpdateCoprocessorContextSuspensionBlockPeriod(uint256 newContextSuspensionBlockPeriod);

    event NewCoprocessorContext(CoprocessorContext activeCoprocessorContext, CoprocessorContext newCoprocessorContext);

    event PreActivateCoprocessorContext(CoprocessorContext newCoprocessorContext, uint256 preActivationBlockNumber);

    event ActivateCoprocessorContext(uint256 contextId);

    event SuspendCoprocessorContext(uint256 contextId);

    event CompromiseCoprocessorContext(uint256 contextId);

    event DeactivateCoprocessorContext(uint256 contextId);

    event DestroyCoprocessorContext(uint256 contextId);

    uint256 coprocessorContextCount;

    function initialize(
        CoprocessorContextBlockPeriods calldata initialContextBlockPeriods,
        string calldata initialFeatureSet,
        Coprocessor[] calldata initialCoprocessors
    ) public {
        string memory featureSet;
        CoprocessorContextBlockPeriods memory contextBlockPeriods;
        Coprocessor[] memory coprocessors = new Coprocessor[](1);

        emit InitCoprocessorContext(featureSet, contextBlockPeriods, coprocessors);
    }

    function updateCoprocessorContextSuspensionBlockPeriod(
        uint256 newCoprocessorContextSuspensionBlockPeriod
    ) external {
        uint256 newContextSuspensionBlockPeriod;

        emit UpdateCoprocessorContextSuspensionBlockPeriod(newContextSuspensionBlockPeriod);
    }

    function addCoprocessorContext(
        uint256 preActivationBlockPeriod,
        string memory featureSet,
        Coprocessor[] calldata coprocessors
    ) external {
        CoprocessorContext memory newCoprocessorContext;
        uint256 preActivationBlockNumber;

        emit PreActivateCoprocessorContext(newCoprocessorContext, preActivationBlockNumber);
    }

    function refreshCoprocessorContextStatuses() external {
        uint256 contextId;

        emit SuspendCoprocessorContext(contextId);

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
