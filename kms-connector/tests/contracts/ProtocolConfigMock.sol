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
    }

    struct PcrValues {
        bytes pcr0;
        bytes pcr1;
        bytes pcr2;
    }

    event NewKmsContext(
        uint256 indexed contextId,
        uint256 indexed previousContextId,
        KmsNodeParams[] kmsNodeParams,
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

    event KmsContextDestroyed(uint256 indexed kmsContextId);

    event KmsEpochDestroyed(uint256 indexed epochId);

    function destroyKmsContext(uint256 kmsContextId) external {
        emit KmsContextDestroyed(kmsContextId);
    }

    function destroyKmsEpoch(uint256 epochId) external {
        emit KmsEpochDestroyed(epochId);
    }

    function defineNewKmsContextAndEpoch(
        KmsNodeParams[] calldata kmsNodeParams,
        string[] calldata,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) external {
        uint256 previousContextId = contextCounter;
        contextCounter++;
        emit NewKmsContext(contextCounter, previousContextId, kmsNodeParams, thresholds, softwareVersion, pcrValues);
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
