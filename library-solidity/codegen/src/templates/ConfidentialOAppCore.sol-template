// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title   ConfidentialOAppCore
 * @notice  Shared peer registry for a confidential omnichain app (OApp). A "peer" is the
 *          trusted counterpart instance of this app on another chain, addressed by its
 *          LayerZero endpoint id (`eid`). The same registry is used by both the send side
 *          ({ConfidentialOAppSender}) and the receive side ({ConfidentialOAppReceiver}), so an
 *          app configures each peer once and it applies in both directions.
 * @dev     App identifiers are `bytes32` so non-EVM peers (e.g. Solana program ids) fit; EVM
 *          peers are the address left-padded to `bytes32`. Subclasses expose {_setPeer} behind
 *          their own access control (e.g. `onlyOwner`).
 */
abstract contract ConfidentialOAppCore {
    /// @notice The trusted peer app per endpoint id (`bytes32(0)` if unset).
    mapping(uint32 eid => bytes32 peer) public peers;

    event PeerSet(uint32 indexed eid, bytes32 indexed peer);

    /// @notice No peer is configured for the requested endpoint id.
    error NoPeer(uint32 eid);

    /// @dev Register (or clear, with `bytes32(0)`) the peer for `eid`.
    function _setPeer(uint32 eid, bytes32 peer) internal virtual {
        peers[eid] = peer;
        emit PeerSet(eid, peer);
    }

    /**
     * @notice Whether `peer` is the trusted counterpart on `eid`. Default: exact match against
     *         the registered peer (an unset peer is never trusted). Override for a custom policy.
     */
    function isPeer(uint32 eid, bytes32 peer) public view virtual returns (bool) {
        bytes32 registered = peers[eid];
        return registered != bytes32(0) && registered == peer;
    }

    /// @dev Returns the peer for `eid`, reverting {NoPeer} if none is set (used on the send side).
    function _getPeerOrRevert(uint32 eid) internal view returns (bytes32 peer) {
        peer = peers[eid];
        if (peer == bytes32(0)) revert NoPeer(eid);
    }
}
