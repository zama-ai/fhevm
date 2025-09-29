// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract GatewayConfigMock {
    event InitializeGatewayConfig(
        ProtocolMetadata metadata,
        uint256 mpcThreshold,
        KmsNode[] kmsNodes,
        Coprocessor[] coprocessors,
        Custodian[] custodians
    );

    event UpdateMpcThreshold(uint256 newMpcThreshold);

    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);

    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);

    event AddHostChain(HostChain hostChain);

    event PauseAllGatewayContracts();

    event UnpauseAllGatewayContracts();

    function initializeFromEmptyProxy(
        ProtocolMetadata memory initialMetadata,
        uint256 initialMpcThreshold,
        uint256 initialPublicDecryptionThreshold,
        uint256 initialUserDecryptionThreshold,
        KmsNode[] memory initialKmsNodes,
        Coprocessor[] memory initialCoprocessors,
        Custodian[] memory initialCustodians
    ) public {
        ProtocolMetadata memory metadata;
        uint256 mpcThreshold;
        KmsNode[] memory kmsNodes = new KmsNode[](1);
        Coprocessor[] memory coprocessors = new Coprocessor[](1);
        Custodian[] memory custodians = new Custodian[](1);

        emit InitializeGatewayConfig(metadata, mpcThreshold, kmsNodes, coprocessors, custodians);
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
