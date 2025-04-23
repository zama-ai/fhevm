// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract GatewayConfigMock {
    event Initialization(
        address pauser,
        ProtocolMetadata metadata,
        uint256 kmsThreshold,
        KmsNode[] kmsNodes,
        Coprocessor[] coprocessors
    );

    event UpdatePauser(address newPauser);

    event UpdateKmsThreshold(uint256 newKmsThreshold);

    event AddNetwork(Network network);

    function initialize(
        address initialPauser,
        ProtocolMetadata memory initialMetadata,
        uint256 initialKmsThreshold,
        KmsNode[] memory initialKmsNodes,
        Coprocessor[] memory initialCoprocessors
    ) public {
        address pauser;
        ProtocolMetadata memory metadata;
        uint256 kmsThreshold;
        KmsNode[] memory kmsNodes = new KmsNode[](1);
        Coprocessor[] memory coprocessors = new Coprocessor[](1);
        emit Initialization(pauser, metadata, kmsThreshold, kmsNodes, coprocessors);
    }

    function updatePauser(address newPauser) external {
        address newPauser;
        emit UpdatePauser(newPauser);
    }

    function updateKmsThreshold(uint256 newKmsThreshold) external {
        uint256 newKmsThreshold;
        emit UpdateKmsThreshold(newKmsThreshold);
    }

    function addNetwork(Network calldata network) external {
        Network memory network;
        emit AddNetwork(network);
    }
}
