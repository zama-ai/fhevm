// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract GatewayConfigMock {
    struct Thresholds {
        uint256 mpcThreshold;
        uint256 publicDecryptionThreshold;
        uint256 userDecryptionThreshold;
        uint256 kmsGenThreshold;
        uint256 coprocessorThreshold;
    }

    event InitializeGatewayConfig(
        uint256 indexed kmsContextId,
        ProtocolMetadata metadata,
        Thresholds thresholds,
        KmsNode[] kmsNodes,
        Coprocessor[] coprocessors,
        Custodian[] custodians
    );

    event UpdateKmsContext(
        uint256 indexed newContextId,
        KmsNode[] newKmsNodes,
        uint256 newMpcThreshold,
        uint256 newPublicDecryptionThreshold,
        uint256 newUserDecryptionThreshold,
        uint256 newKmsGenThreshold
    );

    event DestroyKmsContext(uint256 indexed kmsContextId);

    event UpdateCoprocessors(Coprocessor[] newCoprocessors, uint256 newCoprocessorThreshold);

    event UpdateCustodians(Custodian[] newCustodians);

    event UpdateMpcThresholdForContext(uint256 indexed contextId, uint256 newMpcThreshold);

    event UpdatePublicDecryptionThresholdForContext(uint256 indexed contextId, uint256 newPublicDecryptionThreshold);

    event UpdateUserDecryptionThresholdForContext(uint256 indexed contextId, uint256 newUserDecryptionThreshold);

    event UpdateKmsGenThresholdForContext(uint256 indexed contextId, uint256 newKmsGenThreshold);

    event UpdateCoprocessorThreshold(uint256 newCoprocessorThreshold);

    event AddHostChain(HostChain hostChain);

    event DisableHostChain(uint256 indexed chainId);

    event EnableHostChain(uint256 indexed chainId);

    event RemoveHostChain(uint256 indexed chainId);

    event PauseAllGatewayContracts();

    event UnpauseAllGatewayContracts();

    function initializeFromEmptyProxy(
        uint256 initialKmsContextId,
        ProtocolMetadata calldata initialMetadata,
        Thresholds calldata initialThresholds,
        KmsNode[] calldata initialKmsNodes,
        Coprocessor[] calldata initialCoprocessors,
        Custodian[] calldata initialCustodians
    ) public {
        uint256 kmsContextId;
        ProtocolMetadata memory metadata;
        Thresholds memory thresholds;
        KmsNode[] memory kmsNodes = new KmsNode[](1);
        Coprocessor[] memory coprocessors = new Coprocessor[](1);
        Custodian[] memory custodians = new Custodian[](1);

        emit InitializeGatewayConfig(kmsContextId, metadata, thresholds, kmsNodes, coprocessors, custodians);
    }

    function updateKmsContext(
        uint256 newContextId,
        KmsNode[] calldata newKmsNodes,
        uint256 newMpcThreshold,
        uint256 newPublicDecryptionThreshold,
        uint256 newUserDecryptionThreshold,
        uint256 newKmsGenThreshold
    ) public {
        emit UpdateKmsContext(
            newContextId,
            newKmsNodes,
            newMpcThreshold,
            newPublicDecryptionThreshold,
            newUserDecryptionThreshold,
            newKmsGenThreshold
        );
    }

    function destroyKmsContext(uint256 kmsContextId) external {
        emit DestroyKmsContext(kmsContextId);
    }

    function updateCoprocessors(Coprocessor[] calldata newCoprocessors, uint256 newCoprocessorThreshold) external {
        emit UpdateCoprocessors(newCoprocessors, newCoprocessorThreshold);
    }

    function updateCustodians(Custodian[] calldata newCustodians) external {
        emit UpdateCustodians(newCustodians);
    }

    function updateMpcThresholdForContext(uint256 contextId, uint256 newMpcThreshold) external {
        emit UpdateMpcThresholdForContext(contextId, newMpcThreshold);
    }

    function updatePublicDecryptionThresholdForContext(
        uint256 contextId,
        uint256 newPublicDecryptionThreshold
    ) external {
        emit UpdatePublicDecryptionThresholdForContext(contextId, newPublicDecryptionThreshold);
    }

    function updateUserDecryptionThresholdForContext(uint256 contextId, uint256 newUserDecryptionThreshold) external {
        emit UpdateUserDecryptionThresholdForContext(contextId, newUserDecryptionThreshold);
    }

    function updateKmsGenThresholdForContext(uint256 contextId, uint256 newKmsGenThreshold) external {
        emit UpdateKmsGenThresholdForContext(contextId, newKmsGenThreshold);
    }

    function updateCoprocessorThreshold(uint256 newCoprocessorThreshold) external {
        emit UpdateCoprocessorThreshold(newCoprocessorThreshold);
    }

    function addHostChain(HostChain calldata hostChain) external {
        emit AddHostChain(hostChain);
    }

    function disableHostChain(uint256 chainId) external {
        emit DisableHostChain(chainId);
    }

    function enableHostChain(uint256 chainId) external {
        emit EnableHostChain(chainId);
    }

    function removeHostChain(uint256 chainId) external {
        emit RemoveHostChain(chainId);
    }

    function pauseAllGatewayContracts() external {
        emit PauseAllGatewayContracts();
    }

    function unpauseAllGatewayContracts() external {
        emit UnpauseAllGatewayContracts();
    }
}
