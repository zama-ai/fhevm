// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract GatewayConfigMock {
    event InitializeGatewayConfig(
        address pauser,
        ProtocolMetadata metadata,
        Coprocessor[] coprocessors,
        Custodian[] custodians
    );

    event ReinitializeGatewayConfigV2(Custodian[] custodians);

    event UpdatePauser(address newPauser);

    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);

    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);

    event AddHostChain(HostChain hostChain);

    function initializeFromEmptyProxy(
        address initialPauser,
        ProtocolMetadata memory initialMetadata,
        Coprocessor[] memory initialCoprocessors,
        Custodian[] memory initialCustodians
    ) public {
        address pauser;
        ProtocolMetadata memory metadata;
        Coprocessor[] memory coprocessors = new Coprocessor[](1);
        Custodian[] memory custodians = new Custodian[](1);

        emit InitializeGatewayConfig(pauser, metadata, coprocessors, custodians);
    }

    function reinitializeV2(Custodian[] memory custodians) external {
        emit ReinitializeGatewayConfigV2(custodians);
    }

    function updatePauser(address newPauser) external {
        emit UpdatePauser(newPauser);
    }

    function addHostChain(HostChain calldata hostChain) external {
        emit AddHostChain(hostChain);
    }
}
