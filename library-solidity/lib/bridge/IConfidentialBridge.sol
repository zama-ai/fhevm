// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title   LayerZero messaging structs (local copies)
 * @notice  `MessagingFee` and `MessagingReceipt` are the fee-quote and send-receipt types the
 *          bridge returns. They originate in LayerZero's `ILayerZeroEndpointV2`, but are copied
 *          here field-for-field so that apps using `fhevm/solidity` do not have to install the
 *          LayerZero packages just to bridge. The layout is identical, so they are ABI-compatible
 *          with the real bridge.
 */
struct MessagingFee {
    uint256 nativeFee;
    uint256 lzTokenFee;
}

struct MessagingReceipt {
    bytes32 guid;
    uint64 nonce;
    MessagingFee fee;
}

/**
 * @title   IConfidentialBridge
 * @notice  Minimal view of the host `ConfidentialBridge` that the library wrapper needs.
 * @dev     Must stay in sync with `host-contracts/contracts/bridge/HandlesSender.sol`.
 */
interface IConfidentialBridge {
    function send(
        uint32 dstEid,
        bytes32 dstApp,
        bytes calldata payload,
        bytes32[] calldata handleList,
        uint64 lzComposeGas
    ) external payable returns (MessagingReceipt memory receipt);

    function quote(
        uint32 dstEid,
        address srcApp,
        bytes32 dstApp,
        bytes calldata payload,
        bytes32[] calldata handleList,
        uint64 lzComposeGas
    ) external view returns (MessagingFee memory fee);
}
