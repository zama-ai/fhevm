// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @notice Minimal stand-in for the `ProtocolConfig` contract used by the gw-listener integration
/// tests. The event declarations and the state-changing function signatures mirror
/// `IProtocolConfig`/`ProtocolConfig` exactly (selectors + event topics), so the real bindings can
/// be used to call this contract. The real auth / state-machine checks are dropped; context and
/// epoch ids are assigned from internal counters.
contract ProtocolConfigMock {
    // Match the high-bit "namespace" pattern used by DecryptionMock / KMSGenerationMock so ids
    // across event types don't collide in the DB.
    uint256 contextCounter = (7 << 248) + 1;
    uint256 epochCounter = 8 << 248;
    enum KeyType {
        Server,
        Public
    }

    struct KeyDigest {
        KeyType keyType;
        bytes digest;
    }

    struct KmsThresholds {
        uint256 publicDecryption;
        uint256 userDecryption;
        uint256 kmsGen;
        uint256 mpc;
    }

    struct KmsNodeParams {
        address txSenderAddress;
        address signerAddress;
        string ipAddress;
        string storageUrl;
        int32 partyId;
        string mpcIdentity;
        bytes caCert;
        string storagePrefix;
        string connectorUrl;
    }

    /// The `KmsNodeParams` of `ProtocolConfig` v0.2.0 (fhevm v0.14), before `connectorUrl`.
    struct KmsNodeParamsV014 {
        address txSenderAddress;
        address signerAddress;
        string ipAddress;
        string storageUrl;
        int32 partyId;
        string mpcIdentity;
        bytes caCert;
        string storagePrefix;
    }

    struct PcrValues {
        bytes pcr0;
        bytes pcr1;
        bytes pcr2;
    }

    struct EpochKeyResult {
        uint256 prepKeygenId;
        uint256 keyId;
        KeyDigest[] keyDigests;
    }

    struct EpochCrsResult {
        uint256 crsId;
        uint256 maxBitLength;
        bytes crsDigest;
    }

    struct KmsContextAnchor {
        uint256 emissionBlockNumber;
        bytes32 contextInfoHash;
    }

    mapping(uint256 => KmsContextAnchor) kmsContextAnchors;

    event NewKmsContext(
        uint256 indexed contextId,
        uint256 indexed previousContextId,
        KmsNodeParams[] kmsNodeParams,
        KmsThresholds thresholds,
        string softwareVersion,
        PcrValues[] pcrValues
    );

    /// The `NewKmsContext` event of `ProtocolConfig` v0.2.0 (fhevm v0.14).
    event NewKmsContext(
        uint256 indexed contextId,
        uint256 indexed previousContextId,
        KmsNodeParamsV014[] kmsNodeParams,
        KmsThresholds thresholds,
        string softwareVersion,
        PcrValues[] pcrValues
    );

    event NewKmsEpoch(
        uint256 indexed kmsContextId,
        uint256 indexed epochId,
        uint256 previousContextId,
        uint256 previousEpochId,
        uint256 materialBlockNumber
    );

    event KmsContextCreationConfirmation(
        uint256 indexed kmsContextId,
        address indexed signer,
        bytes signature,
        bytes extraData
    );

    event EpochActivationConfirmation(
        uint256 indexed epochId,
        address indexed signer,
        bytes32 epochMaterialHash,
        bytes signature,
        bytes extraData
    );

    event KmsContextDestructionConfirmation(
        uint256 indexed destroyedContextId,
        address indexed signer,
        bytes signature,
        bytes extraData
    );

    event KmsEpochDestructionConfirmation(
        uint256 indexed destroyedEpochId,
        address indexed signer,
        bytes signature,
        bytes extraData
    );

    event KmsContextDestroyed(uint256 indexed kmsContextId);

    event KmsEpochDestroyed(uint256 indexed epochId);

    function destroyKmsContext(uint256 kmsContextId) external {
        emit KmsContextDestroyed(kmsContextId);
    }

    function destroyKmsEpoch(uint256 epochId) external {
        emit KmsEpochDestroyed(epochId);
    }

    function confirmKmsContextCreation(
        uint256 kmsContextId,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        emit KmsContextCreationConfirmation(kmsContextId, msg.sender, signature, extraData);
    }

    function confirmEpochActivation(
        uint256 epochId,
        EpochKeyResult[] calldata,
        EpochCrsResult[] calldata,
        bytes calldata,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        emit EpochActivationConfirmation(epochId, msg.sender, bytes32(0), signature, extraData);
    }

    function confirmKmsContextDestruction(
        uint256 destroyedContextId,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        emit KmsContextDestructionConfirmation(destroyedContextId, msg.sender, signature, extraData);
    }

    function confirmKmsEpochDestruction(
        uint256 destroyedEpochId,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        emit KmsEpochDestructionConfirmation(destroyedEpochId, msg.sender, signature, extraData);
    }

    function defineNewKmsContextAndEpoch(
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) external {
        uint256 previousContextId = contextCounter;
        contextCounter++;
        emit NewKmsContext(contextCounter, previousContextId, kmsNodeParams, thresholds, softwareVersion, pcrValues);
    }

    /// Defines a context as `ProtocolConfig` v0.2.0 did: v0.14 event and anchor over v0.14 nodes.
    function defineKmsContextV014() external {
        KmsNodeParamsV014[] memory kmsNodeParams = new KmsNodeParamsV014[](1);
        kmsNodeParams[0] = KmsNodeParamsV014({
            txSenderAddress: address(uint160(0xA1)),
            signerAddress: address(uint160(0xB1)),
            ipAddress: "http://kms-0.example.com:50001",
            storageUrl: "https://s0.example.com",
            partyId: 1,
            mpcIdentity: "kms-0",
            caCert: hex"ca",
            storagePrefix: "PUB-p1"
        });
        KmsThresholds memory thresholds = KmsThresholds(1, 2, 2, 1);
        string memory softwareVersion = "v0.14.0";
        PcrValues[] memory pcrValues = new PcrValues[](1);
        pcrValues[0] = PcrValues(hex"01", hex"02", hex"03");

        uint256 previousContextId = contextCounter;
        contextCounter++;
        kmsContextAnchors[contextCounter] = KmsContextAnchor(
            block.number,
            keccak256(abi.encode(kmsNodeParams, thresholds, softwareVersion, pcrValues))
        );
        emit NewKmsContext(contextCounter, previousContextId, kmsNodeParams, thresholds, softwareVersion, pcrValues);
    }

    function getKmsContextAnchor(
        uint256 contextId
    ) external view returns (uint256 emissionBlockNumber, bytes32 contextInfoHash) {
        KmsContextAnchor memory anchor = kmsContextAnchors[contextId];
        return (anchor.emissionBlockNumber, anchor.contextInfoHash);
    }

    function defineNewEpochForCurrentKmsContext() external {
        uint256 previousEpochId = epochCounter;
        epochCounter++;
        emit NewKmsEpoch(contextCounter, epochCounter, contextCounter, previousEpochId, block.number);
    }

    function getCurrentKmsContextAndEpoch() external pure returns (uint256, uint256) {
        return (1, 1);
    }

    function isValidKmsContext(uint256) external pure returns (bool) {
        return true;
    }

    function isValidEpochForContext(uint256, uint256) external pure returns (bool) {
        return true;
    }
}
