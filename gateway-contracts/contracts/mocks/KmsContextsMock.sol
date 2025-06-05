// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract KmsContextsMock {
    event Initialization(KmsConfiguration kmsConfiguration);

    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);

    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);

    event UpdateKmsContextGenerationBlockPeriod(uint256 newKmsContextGenerationBlockPeriod);

    event UpdateKmsContextSuspensionBlockPeriod(uint256 newKmsContextSuspensionBlockPeriod);

    event NewKmsContext(KmsContext activeKmsContext, KmsContext newKmsContext);

    event StartKeyResharing(KmsContext activeKmsContext, KmsContext newKmsContext, uint256 generationBlockNumber);

    event ValidateKeyResharing(KmsContext newKmsContext);

    event InvalidateKeyResharing(uint256 kmsContextId);

    event PreActivateKmsContext(KmsContext newKmsContext, uint256 preActivationBlockNumber);

    event ActivateKmsContext(uint256 kmsContextId);

    event SuspendKmsContext(uint256 kmsContextId);

    event CompromiseKmsContext(uint256 kmsContextId);

    event DeactivateKmsContext(uint256 kmsContextId);

    event DestroyKmsContext(uint256 kmsContextId);

    uint256 kmsContextCount;

    function initialize(KmsConfiguration calldata initialKmsConfiguration) public {
        KmsConfiguration memory kmsConfiguration;

        emit Initialization(kmsConfiguration);
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

    function validateKeyResharing(uint256 kmsContextId, bytes calldata signature) external {
        KmsContext memory newKmsContext;

        emit ValidateKeyResharing(newKmsContext);
    }

    function refreshKmsContextStatuses() external {
        uint256 kmsContextId;

        emit InvalidateKeyResharing(kmsContextId);

        emit DestroyKmsContext(kmsContextId);

        emit SuspendKmsContext(kmsContextId);

        emit ActivateKmsContext(kmsContextId);

        emit DeactivateKmsContext(kmsContextId);
    }

    function compromiseKmsContext(uint256 kmsContextId) external {
        emit CompromiseKmsContext(kmsContextId);
    }

    function destroyKmsContext(uint256 kmsContextId) external {
        emit DestroyKmsContext(kmsContextId);
    }

    function moveSuspendedKmsContextToActive() external {
        uint256 kmsContextId;

        emit DeactivateKmsContext(kmsContextId);

        emit ActivateKmsContext(kmsContextId);
    }
}
