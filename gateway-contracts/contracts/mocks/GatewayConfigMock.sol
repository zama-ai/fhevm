// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract GatewayConfigMock {
    event Initialization(address pauser, ProtocolMetadata metadata, Coprocessor[] coprocessors);

    event UpdatePauser(address newPauser);

    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);

    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);

    event AddHostChain(HostChain hostChain);

    function initialize(
        address initialPauser,
        ProtocolMetadata calldata initialMetadata,
        Coprocessor[] calldata initialCoprocessors
    ) public {
        address pauser;
        ProtocolMetadata memory metadata;
        Coprocessor[] memory coprocessors = new Coprocessor[](1);

        emit Initialization(pauser, metadata, coprocessors);
    }

    function updatePauser(address newPauser) external {
        emit UpdatePauser(newPauser);
    }

    function addHostChain(HostChain calldata hostChain) external {
        emit AddHostChain(hostChain);
    }
}
