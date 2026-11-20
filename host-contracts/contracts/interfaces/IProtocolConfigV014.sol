// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsThresholds, PcrValues} from "../shared/Structs.sol";

/**
 * @title IProtocolConfigV014
 * @notice The `NewKmsContext` event of ProtocolConfig v0.2.0, released in fhevm v0.14.
 * @dev Off-chain consumers use it to decode the events that a v0.2.0 contract emitted.
 *      No contract implements it.
 */
interface IProtocolConfigV014 {
    /**
     * @notice The v0.2.0 `KmsNodeParams`, before it gained `connectorUrl`.
     */
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

    /**
     * @notice The v0.2.0 `NewKmsContext` event.
     * @param contextId The new context ID.
     * @param previousContextId The previous active context ID.
     * @param kmsNodeParams The KMS nodes of the new context.
     * @param thresholds The thresholds of the new context.
     * @param softwareVersion The KMS software version expected for the context.
     * @param pcrValues Accepted enclave PCR values for the context.
     */
    event NewKmsContext(
        uint256 indexed contextId,
        uint256 indexed previousContextId,
        KmsNodeParams[] kmsNodeParams,
        KmsThresholds thresholds,
        string softwareVersion,
        PcrValues[] pcrValues
    );
}
