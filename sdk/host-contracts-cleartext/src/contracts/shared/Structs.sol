// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @notice Struct that represents a KMS (Key Management Service) node.
 */
struct KmsNode {
    /// @notice Address of the KMS node's transaction sender (also called KMS connector)
    address txSenderAddress;
    /// @notice Address of the KMS node's signer (used for signing inputs with EIP712 signatures)
    address signerAddress;
    /// @notice IP address of the KMS node
    string ipAddress;
    /// @notice URL address of the KMS node's storage where ciphertexts are stored
    string storageUrl;
}

/**
 * @notice Enclave attestation values for a KMS software build.
 */
struct PcrValues {
    bytes pcr0;
    bytes pcr1;
    bytes pcr2;
}

/**
 * @notice Connector-facing KMS node parameters.
 * @dev Extends the stored KmsNode fields with MPC connection metadata.
 */
struct KmsNodeParams {
    address txSenderAddress;
    address signerAddress;
    string ipAddress;
    string storageUrl;
    int32 partyId;
    string mpcIdentity;
    bytes caCert;
    /// @notice Prefix used by KMS Core when storing public data, matches proto MpcNode.public_storage_prefix.
    string storagePrefix;
}

/**
 * @notice On-chain anchor for a previously emitted NewKmsContext event.
 */
struct KmsContextAnchor {
    /// @notice Block number at which the NewKmsContext event was emitted
    uint256 emissionBlockNumber;
    /// @notice keccak256 hash of the emitted context payload (nodes, thresholds, software version, PCR values)
    bytes32 contextInfoHash;
}

/**
 * @notice Struct that represents a per-host-chain replay window used during a coprocessor blue-green upgrade.
 */
struct ChainUpgradeWindow {
    /// @notice Host chain id the window applies to
    uint64 chainId;
    /// @notice First block GCS replays in dry-run, inclusive
    uint64 startBlock;
    /// @notice Last block GCS replays in dry-run, inclusive
    uint64 endBlock;
}
