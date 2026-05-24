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

/**
 * @notice Struct that represents a coprocessor context, a versioned identity of the coprocessor fleet.
 */
struct CoprocessorContext {
    /// @notice Gateway block GCS's gateway-listener resumes from
    uint64 gwStartBlock;
    /// @notice L1 block number at which the context was defined
    uint64 activatedAtBlock;
    /// @notice Whether this context has been destroyed
    bool destroyed;
    /// @notice Coprocessor software version this context corresponds to (e.g. "v0.14.0")
    string softwareVersion;
    /// @notice Per-host-chain replay windows
    ChainUpgradeWindow[] chainUpgradeWindows;
}
