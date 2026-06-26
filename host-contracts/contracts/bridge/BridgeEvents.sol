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
    ///         between `dstHandle` and a plaintext value. Could be used when a ciphertext is missing in
    ///         the coprocessor.
    /// @param dstHandle  The destination handle to associate a fallback ciphertext with.
    /// @param plaintext  value corresponding to the dstHandle, trivially-encrypted by the coprocessor.
    event FallbackGrantedPlaintext(bytes32 indexed dstHandle, uint256 plaintext);

    /// @notice Emitted by the HandlesSender when governance updates the dstEid → dstChainId map.
    /// @param dstEid        The LayerZero endpoint id.
    /// @param dstChainId    The chain id.
    event DstChainIdSet(uint32 indexed dstEid, uint64 dstChainId);

    /// @notice Emitted by the HandlesSender when governance sets a custom base `lzReceive`
    ///         gas for a `dstEid`. A value of 0 clears the override, restoring the default.
    /// @param dstEid              The LayerZero endpoint id.
    /// @param lzReceiveBaseGas    The custom base gas (0 means fall back to LZ_RECEIVE_BASE_GAS_DEFAULT).
    event LzReceiveBaseGasSet(uint32 indexed dstEid, uint64 lzReceiveBaseGas);

    /// @notice Emitted by the HandlesSender when governance sets a custom per-handle
    ///         `lzReceive` gas for a `dstEid`. A value of 0 clears the override.
    /// @param dstEid                 The LayerZero endpoint id.
    /// @param lzReceivePerHandleGas  The custom per-handle gas (0 means fall back to LZ_RECEIVE_PER_HANDLE_GAS_DEFAULT).
    event LzReceivePerHandleGasSet(uint32 indexed dstEid, uint64 lzReceivePerHandleGas);

    /// @notice Emitted by the HandlesSender when governance sets a custom per-payload-byte
    ///         `lzReceive` gas for a `dstEid`. A value of 0 clears the override.
    /// @param dstEid                      The LayerZero endpoint id.
    /// @param lzReceivePerPayloadByteGas  The custom per-payload-byte gas (0 means fall back to
    ///                                    LZ_RECEIVE_PER_PAYLOAD_BYTE_DEFAULT).
    event LzReceivePerPayloadByteGasSet(uint32 indexed dstEid, uint64 lzReceivePerPayloadByteGas);
}
