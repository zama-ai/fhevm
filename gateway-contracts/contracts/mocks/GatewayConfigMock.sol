// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract GatewayConfigMock {
    struct V3UpgradeInput {
        address txSenderAddress;
        string s3BucketUrl;
    }

    event InitializeGatewayConfig(
        address pauser,
        ProtocolMetadata metadata,
        uint256 mpcThreshold,
        KmsNodeV2[] kmsNodes,
        Coprocessor[] coprocessors,
        Custodian[] custodians
    );

    event ReinitializeGatewayConfigV3(KmsNodeV1[] kmsNodesV1, KmsNodeV2[] kmsNodesV2);

    event UpdatePauser(address newPauser);

    event UpdateMpcThreshold(uint256 newMpcThreshold);

    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);

    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);

    event AddHostChain(HostChain hostChain);

    event PauseAllGatewayContracts();

    event UnpauseAllGatewayContracts();

    function initializeFromEmptyProxy(
        address initialPauser,
        ProtocolMetadata memory initialMetadata,
        uint256 initialMpcThreshold,
        uint256 initialPublicDecryptionThreshold,
        uint256 initialUserDecryptionThreshold,
        KmsNodeV2[] memory initialKmsNodes,
        Coprocessor[] memory initialCoprocessors,
        Custodian[] memory initialCustodians
    ) public {
        address pauser;
        ProtocolMetadata memory metadata;
        uint256 mpcThreshold;
        KmsNodeV2[] memory kmsNodes = new KmsNodeV2[](1);
        Coprocessor[] memory coprocessors = new Coprocessor[](1);
        Custodian[] memory custodians = new Custodian[](1);

        emit InitializeGatewayConfig(pauser, metadata, mpcThreshold, kmsNodes, coprocessors, custodians);
    }

    function reinitializeV3(V3UpgradeInput[] memory v3UpgradeInputs) public {
        KmsNodeV1[] memory kmsNodesV1 = new KmsNodeV1[](1);
        KmsNodeV2[] memory kmsNodesV2 = new KmsNodeV2[](1);

        emit ReinitializeGatewayConfigV3(kmsNodesV1, kmsNodesV2);
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

    function pauseAllGatewayContracts() external {
        emit PauseAllGatewayContracts();
    }

    function unpauseAllGatewayContracts() external {
        emit UnpauseAllGatewayContracts();
    }
}
