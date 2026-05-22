// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract BridgeEvents {
    /// @notice Emitted by the HandlesSender on source chain for each handle requested for
    ///         bridging.
    /// @param senderDapp  The dapp requesting the bridging.
    /// @param srcHandle   The source-chain handle approved for bridging.
    /// @param dstChainId  The chain id of the destination chain.
    /// @param guid        The LayerZero message GUID assigned to the cross-chain send.
    event BridgeHandle(address indexed senderDapp, bytes32 srcHandle, uint64 dstChainId, bytes32 guid);

    /// @notice Emitted by the HandlesReceiver on the destination chain after deriving a new
    ///         destination handle from the source handle.
    /// @param receiverDapp The dapp receiving the bridged handle.
    /// @param srcHandle    The source-chain handle that was bridged.
    /// @param dstHandle    The newly derived destination-chain handle.
    /// @param guid         The LayerZero message GUID, used to correlate with the source event.
    event HandleBridged(address indexed receiverDapp, bytes32 srcHandle, bytes32 dstHandle, bytes32 guid);

    /// @notice Emitted by the HandlesReceiver when governance authorizes a fallback association
    ///         between `dstHandle` and the ciphertext identified by `ciphertextHash`. The
    ///         coprocessor uses this when a matching pair of bridge events did not arrive or
    ///         when the source ciphertext is missing. This is a permission, not an assertion:
    ///         if a node already has a real association it prefers that.
    /// @param dstHandle  The destination handle to associate a fallback ciphertext with.
    /// @param plainText  value corresponding to the dstHandle, trivially encrypted by coprocessor.
    event FallbackGrantedPlainText(bytes32 indexed dstHandle, uint256 plainText);

    /// @notice Emitted by the HandlesSender when governance updates the dstEid → dstChainId map.
    /// @param dstEid        The LayerZero endpoint id.
    /// @param dstChainId    The chain id.
    event DstChainIdSet(uint32 indexed dstEid, uint64 dstChainId);
}
