// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract KmsContextsMock {
    event Initialization(
        DecryptionThresholds decryptionThresholds,
        KmsBlockPeriods blockPeriods,
        bytes8 softwareVersion,
        uint256 mpcThreshold,
        KmsNode[] kmsNodes
    );

    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);

    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);

    event UpdateKmsContextGenerationBlockPeriod(uint256 newKmsContextGenerationBlockPeriod);

    event UpdateKmsContextSuspensionBlockPeriod(uint256 newKmsContextSuspensionBlockPeriod);

    event NewKmsContext(KmsContext activeKmsContext, KmsContext newKmsContext);

    event StartKeyResharing(KmsContext activeKmsContext, KmsContext newKmsContext, uint256 generationBlockNumber);

    event ValidateKeyResharing(KmsContext newKmsContext);

    event InvalidateKeyResharing(uint256 contextId);

    event PreActivateKmsContext(KmsContext newKmsContext, uint256 preActivationBlockNumber);

    event ActivateKmsContext(uint256 contextId);

    event SuspendKmsContext(uint256 contextId);

    event CompromiseKmsContext(uint256 contextId);

    event DeactivateKmsContext(uint256 contextId);

    event DestroyKmsContext(uint256 contextId);

    uint256 kmsContextCount;

    function initializeFromEmptyProxy(
        DecryptionThresholds calldata initialDecryptionThresholds,
        KmsBlockPeriods calldata initialBlockPeriods,
        bytes8 initialSoftwareVersion,
        uint256 initialMpcThreshold,
        KmsNode[] calldata initialKmsNodes
    ) public {
        DecryptionThresholds memory decryptionThresholds;
        KmsBlockPeriods memory blockPeriods;
        bytes8 softwareVersion;
        uint256 mpcThreshold;
        KmsNode[] memory kmsNodes = new KmsNode[](1);

        emit Initialization(decryptionThresholds, blockPeriods, softwareVersion, mpcThreshold, kmsNodes);
    }

    function updatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold) external {
        emit UpdatePublicDecryptionThreshold(newPublicDecryptionThreshold);
    }

    function updateUserDecryptionThreshold(uint256 newUserDecryptionThreshold) external {
        emit UpdateUserDecryptionThreshold(newUserDecryptionThreshold);
    }

    function updateKmsContextGenerationBlockPeriod(uint256 newKmsContextGenerationBlockPeriod) external {
        emit UpdateKmsContextGenerationBlockPeriod(newKmsContextGenerationBlockPeriod);
    }

    function updateKmsContextSuspensionBlockPeriod(uint256 newKmsContextSuspensionBlockPeriod) external {
        emit UpdateKmsContextSuspensionBlockPeriod(newKmsContextSuspensionBlockPeriod);
    }

    function addKmsContext(
        uint256 preActivationBlockPeriod,
        bytes8 softwareVersion,
        bool reshareKeys,
        uint256 mpcThreshold,
        KmsNode[] calldata kmsNodes,
        DecryptionThresholds calldata decryptionThresholds
    ) external {
        KmsContext memory activeKmsContext;
        KmsContext memory newKmsContext;
        uint256 generationBlockNumber;

        emit NewKmsContext(activeKmsContext, newKmsContext);

        emit StartKeyResharing(activeKmsContext, newKmsContext, generationBlockNumber);
    }

    function validateKeyResharing(uint256 contextId, bytes calldata signature) external {
        KmsContext memory newKmsContext;

        emit ValidateKeyResharing(newKmsContext);
    }

    function refreshKmsContextStatuses() external {
        uint256 contextId;

        emit InvalidateKeyResharing(contextId);

        emit DestroyKmsContext(contextId);

        emit SuspendKmsContext(contextId);

        emit ActivateKmsContext(contextId);

        emit DeactivateKmsContext(contextId);
    }

    function compromiseKmsContext(uint256 contextId) external {
        emit CompromiseKmsContext(contextId);
    }

    function destroyKmsContext(uint256 contextId) external {
        emit DestroyKmsContext(contextId);
    }

    function moveSuspendedKmsContextToActive() external {
        uint256 contextId;

        emit DeactivateKmsContext(contextId);

        emit ActivateKmsContext(contextId);
    }
}
