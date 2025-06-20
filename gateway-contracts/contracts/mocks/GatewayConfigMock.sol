// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract GatewayConfigMock {
    event InitializeGatewayConfig(
        address pauser,
        ProtocolMetadata metadata,
        uint256 mpcThreshold,
        KmsNode[] kmsNodes,
        Custodian[] custodians
    );

    event ReinitializeGatewayConfigV2(Custodian[] custodians);

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
        Custodian[] memory initialCustodians
    ) public {
        address pauser;
        ProtocolMetadata memory metadata;
        uint256 mpcThreshold;
        KmsNode[] memory kmsNodes = new KmsNode[](1);
        Custodian[] memory custodians = new Custodian[](1);

        emit InitializeGatewayConfig(pauser, metadata, mpcThreshold, kmsNodes, custodians);
    }

    function reinitializeV2(Custodian[] memory custodians) external {
        emit ReinitializeGatewayConfigV2(custodians);
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
