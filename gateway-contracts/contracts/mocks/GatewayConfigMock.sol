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
        ProtocolMetadata metadata,
        Thresholds thresholds,
        KmsNode[] kmsNodes,
        Coprocessor[] coprocessors,
        Custodian[] custodians
    );

    event UpdateKmsNodes(
        KmsNode[] newKmsNodes,
        uint256 newMpcThreshold,
        uint256 newPublicDecryptionThreshold,
        uint256 newUserDecryptionThreshold,
        uint256 newKmsGenThreshold
    );

    event UpdateCoprocessors(Coprocessor[] newCoprocessors, uint256 newCoprocessorThreshold);

    event UpdateCustodians(Custodian[] newCustodians);

    event UpdateMpcThreshold(uint256 newMpcThreshold);

    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);

    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);

    event UpdateKmsGenThreshold(uint256 newKmsGenThreshold);

    event UpdateCoprocessorThreshold(uint256 newCoprocessorThreshold);

    event AddHostChain(HostChain hostChain);

    event PauseAllGatewayContracts();

    event UnpauseAllGatewayContracts();

    function initializeFromEmptyProxy(
        ProtocolMetadata calldata initialMetadata,
        Thresholds calldata initialThresholds,
        KmsNode[] calldata initialKmsNodes,
        Coprocessor[] calldata initialCoprocessors,
        Custodian[] calldata initialCustodians
    ) public {
        ProtocolMetadata memory metadata;
        Thresholds memory thresholds;
        KmsNode[] memory kmsNodes = new KmsNode[](1);
        Coprocessor[] memory coprocessors = new Coprocessor[](1);
        Custodian[] memory custodians = new Custodian[](1);

        emit InitializeGatewayConfig(metadata, thresholds, kmsNodes, coprocessors, custodians);
    }

    function updateKmsNodes(
        KmsNode[] calldata newKmsNodes,
        uint256 newMpcThreshold,
        uint256 newPublicDecryptionThreshold,
        uint256 newUserDecryptionThreshold,
        uint256 newKmsGenThreshold
    ) public {
        emit UpdateKmsNodes(
            newKmsNodes,
            newMpcThreshold,
            newPublicDecryptionThreshold,
            newUserDecryptionThreshold,
            newKmsGenThreshold
        );
    }

    function updateCoprocessors(Coprocessor[] calldata newCoprocessors, uint256 newCoprocessorThreshold) external {
        emit UpdateCoprocessors(newCoprocessors, newCoprocessorThreshold);
    }

    function updateCustodians(Custodian[] calldata newCustodians) external {
        emit UpdateCustodians(newCustodians);
    }

    function updateMpcThreshold(uint256 newMpcThreshold) external {
        emit UpdateMpcThreshold(newMpcThreshold);
    }

    function updatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold) external {
        emit UpdatePublicDecryptionThreshold(newPublicDecryptionThreshold);
    }

    function updateUserDecryptionThreshold(uint256 newUserDecryptionThreshold) external {
        emit UpdateUserDecryptionThreshold(newUserDecryptionThreshold);
    }

    function updateKmsGenThreshold(uint256 newKmsGenThreshold) external {
        emit UpdateKmsGenThreshold(newKmsGenThreshold);
    }

    function updateCoprocessorThreshold(uint256 newCoprocessorThreshold) external {
        emit UpdateCoprocessorThreshold(newCoprocessorThreshold);
    }

    function addHostChain(HostChain calldata hostChain) external {
        emit AddHostChain(hostChain);
    }

    function pauseAllGatewayContracts() external {
        emit PauseAllGatewayContracts();
    }

    function unpauseAllGatewayContracts() external {
        emit UnpauseAllGatewayContracts();
    }
}
