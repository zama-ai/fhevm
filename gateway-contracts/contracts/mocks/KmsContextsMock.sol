// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract KmsContextsMock {
    event InitializeKmsContexts(
        DecryptionThresholds decryptionThresholds,
        bytes8 softwareVersion,
        uint256 mpcThreshold,
        KmsNode[] kmsNodes
    );

    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);

    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);

    event NewKmsContext(KmsContext activeKmsContext, KmsContext newKmsContext, KmsBlockPeriods blockPeriods);

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
        bytes8 initialSoftwareVersion,
        uint256 initialMpcThreshold,
        KmsNode[] calldata initialKmsNodes,
        DecryptionThresholds calldata initialDecryptionThresholds
    ) public {
        DecryptionThresholds memory decryptionThresholds;
        bytes8 softwareVersion;
        uint256 mpcThreshold;
        KmsNode[] memory kmsNodes = new KmsNode[](1);

        emit InitializeKmsContexts(decryptionThresholds, softwareVersion, mpcThreshold, kmsNodes);
    }

    function updatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold) external {
        emit UpdatePublicDecryptionThreshold(newPublicDecryptionThreshold);
    }

    function updateUserDecryptionThreshold(uint256 newUserDecryptionThreshold) external {
        emit UpdateUserDecryptionThreshold(newUserDecryptionThreshold);
    }

    function addKmsContext(
        bytes8 softwareVersion,
        bool reshareKeys,
        uint256 mpcThreshold,
        KmsNode[] calldata kmsNodes,
        KmsBlockPeriods calldata blockPeriods,
        DecryptionThresholds calldata decryptionThresholds
    ) external {
        KmsContext memory activeKmsContext;
        KmsContext memory newKmsContext;
        uint256 generationBlockNumber;

        emit NewKmsContext(activeKmsContext, newKmsContext, blockPeriods);

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
