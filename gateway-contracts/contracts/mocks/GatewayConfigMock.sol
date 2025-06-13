// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract GatewayConfigMock {
    event Initialization(
        address pauser,
        ProtocolMetadata metadata,
        uint256 mpcThreshold,
        KmsNode[] kmsNodes,
        Coprocessor[] coprocessors
    );

    event UpdatePauser(address newPauser);

    event UpdateMpcThreshold(uint256 newMpcThreshold);

    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);

    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);

    event AddHostChain(HostChain hostChain);

    function initializeFromEmptyProxy(
        address initialPauser,
        ProtocolMetadata memory initialMetadata,
        uint256 initialMpcThreshold,
        uint256 initialPublicDecryptionThreshold,
        uint256 initialUserDecryptionThreshold,
        KmsNode[] memory initialKmsNodes,
        Coprocessor[] memory initialCoprocessors
    ) public {
        address pauser;
        ProtocolMetadata memory metadata;
        uint256 mpcThreshold;
        KmsNode[] memory kmsNodes = new KmsNode[](1);
        Coprocessor[] memory coprocessors = new Coprocessor[](1);

        emit Initialization(pauser, metadata, mpcThreshold, kmsNodes, coprocessors);
    }

    function updatePauser(address newPauser) external {
        emit UpdatePauser(newPauser);
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
}
