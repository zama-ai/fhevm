// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract BridgeEvents {
    /// @notice Emitted by the HandlesSender on the source chain for each handle approved for
    ///         bridging. Serves as the source-side trust anchor: the coprocessor records one
    ///         outstanding `SrcHandle → DstChainId` approval and pins the associated source
    ///         ciphertext. Each event is consumed one-shot by the first matching `HandleBridged`
    ///         event for the same `(srcHandle, dstChainId)` (correlated by guid).
    /// @param srcHandle   The source-chain handle approved for bridging.
    /// @param dstChainId  The chain id of the destination chain (binding for the approval).
    /// @param guid        The LayerZero message GUID assigned to the cross-chain send.
    event BridgeHandle(bytes32 srcHandle, uint256 dstChainId, bytes32 guid);

    /// @notice Emitted by the HandlesReceiver on the destination chain after deriving a new
    ///         destination handle from the source handle. The coprocessor re-derives the
    ///         destination handle locally and, on a match, consumes the corresponding outstanding
    ///         `BridgeHandle` approval and associates the source ciphertext with `dstHandle`.
    /// @param srcHandle  The source-chain handle that was bridged.
    /// @param dstHandle  The newly derived destination-chain handle.
    /// @param guid       The LayerZero message GUID, used to correlate with the source event.
    event HandleBridged(bytes32 srcHandle, bytes32 dstHandle, bytes32 guid);

    /// @notice Emitted by the HandlesReceiver when governance authorizes a fallback association
    ///         between `dstHandle` and the ciphertext identified by `ciphertextHash`. The
    ///         coprocessor uses this when a matching pair of bridge events did not arrive or
    ///         when the source ciphertext is missing. This is a permission, not an assertion:
    ///         if a node already has a real association it prefers that.
    /// @param dstHandle       The destination handle to associate a fallback ciphertext with.
    /// @param ciphertextHash  Hash identifying the ciphertext the coprocessor may use.
    event FallbackGranted(bytes32 indexed dstHandle, bytes32 ciphertextHash);

    /// @notice Emitted by the HandlesSender when governance updates the dstEid → dstChainId map.
    /// @param dstEid           The LayerZero endpoint id.
    /// @param oldDstChainId    The previous chain id (0 if unset).
    /// @param newDstChainId    The new chain id (0 clears the mapping).
    event DstChainIdSet(uint32 indexed dstEid, uint256 oldDstChainId, uint256 newDstChainId);
}
