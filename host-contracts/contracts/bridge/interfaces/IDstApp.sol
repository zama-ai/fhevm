// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title IDstApp
 * @notice Callback interface implemented by destination apps to receive bridged payloads.
 * @dev `onReceive` is invoked from `HandlesReceiver.lzCompose` inside a dedicated
 *      `lzCompose` transaction, after transient ACL allowance has been granted to the
 *      destination app for every derived handle in `dstHandleList`. Apps should treat
 *      `srcHandleList` entries as opaque bytes blobs (they are source-chain handles,
 *      not usable on this chain) and operate on `dstHandleList` entries by index.
 */
interface IDstApp {
    /**
     * @notice Receive a bridged payload from a source chain.
     * @param srcEid          The LayerZero endpoint id of the source chain.
     * @param srcApp          The source app that initiated the bridge on the source chain,
     *                        as bytes32. For EVM source chains this is a left-zero-padded
     *                        20-byte address (`address(uint160(uint256(srcApp)))` to
     *                        convert). For non-EVM source chains (e.g. Solana) this carries
     *                        the full 32-byte native program/account identifier.
     * @param payload         The opaque app-level payload (as encoded by the source app).
     * @param srcHandleList   The list of source-chain handles, in the order the source app
     *                        passed them to `HandlesSender.send`. Treat as opaque bytes32.
     * @param dstHandleList   The list of destination-chain handles derived from
     *                        `srcHandleList`, one-to-one by index. The HandlesReceiver has
     *                        already granted transient ACL allowance to this app for each.
     *
     * @dev Reverting from `onReceive` reverts only the lzCompose transaction. Bridge state
     *      from the lzReceive step (derivations + `HandleBridged` events) is already
     *      committed on-chain and the coprocessor's association is unaffected.
     *      Note that `lzCompose` can be retried. If the app determines the source chain is
     *      untrusted, it should revert here to prevent app-level state changes.
     */
    function onReceive(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata srcHandleList,
        bytes32[] calldata dstHandleList
    ) external;
}
