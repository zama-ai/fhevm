// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract GatewayConfigMock {
    event Initialization(
        address pauser,
        ProtocolMetadata metadata,
        KmsConfiguration kmsConfiguration,
        Coprocessor[] coprocessors
    );

    event UpdatePauser(address newPauser);

    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);

    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);

    event UpdateKmsContextGenerationBlockPeriod(uint256 newKmsContextGenerationBlockPeriod);

    event UpdateKmsContextSuspensionBlockPeriod(uint256 newKmsContextSuspensionBlockPeriod);

    event StartKeyResharing(KmsContext activeKmsContext, KmsContext newKmsContext, uint256 generationBlockNumber);

    event ValidateKeyResharing(KmsContext newKmsContext);

    event InvalidateKeyResharing(uint256 kmsContextId);

    event DeactivateKmsContext(uint256 kmsContextId);

    event CompromiseKmsContext(uint256 kmsContextId);

    event NewKmsContext(KmsContext activeKmsContext, KmsContext newKmsContext);

    event DestroyKmsContext(uint256 kmsContextId);

    event SuspendKmsContext(uint256 kmsContextId);

    event ActivateKmsContext(uint256 kmsContextId);

    event StartKmsContextPreActivation(KmsContext newKmsContext, uint256 preActivationBlockNumber);

    event AddHostChain(HostChain hostChain);

    uint256 kmsContextCount;

    function initialize(
        address initialPauser,
        ProtocolMetadata calldata initialMetadata,
        KmsConfiguration calldata initialKmsConfiguration,
        Coprocessor[] calldata initialCoprocessors
    ) public {
        address pauser;
        ProtocolMetadata memory metadata;
        KmsConfiguration memory kmsConfiguration;
        Coprocessor[] memory coprocessors = new Coprocessor[](1);

        emit Initialization(pauser, metadata, kmsConfiguration, coprocessors);
    }

    function updatePauser(address newPauser) external {
        emit UpdatePauser(newPauser);
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
        bytes calldata softwareVersion,
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

    function addHostChain(HostChain calldata hostChain) external {
        emit AddHostChain(hostChain);
    }
}
