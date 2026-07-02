// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IConfidentialBridge} from "../Impl.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {FHE} from "../FHE.sol";

interface IConfidentialOAppCore {
    /// @notice No peer is configured for the requested endpoint id.
    error NoPeer(uint32 eid);

    /// @notice The LayerZero ConfidentialBridge does not support the requested endpoint id (not routable).
    error UnsupportedEid(uint32 eid);

    /// @notice Event emitted when a peer ConfidentialOApp (cOApp) is set for a corresponding LayerZero endpoint
    event PeerSet(uint32 eid, bytes32 peer);

    /**
     * @notice Retrieves the LayerZero ConfidentialBridge address associated with the cOApp.
     * @return cBridge The ConfidentialBridge address.
     */
    function LZConfidentialBridgeAddress() external view returns (address cBridge);

    /**
     * @notice Retrieves the peer (cOApp) associated with a corresponding LayerZero endpoint.
     * @param eid The LayerZero endpoint ID.
     * @return peer The peer address (cOApp instance) associated with the corresponding endpoint.
     */
    function peers(uint32 eid) external view returns (bytes32 peer);

    /**
     * @notice Sets the peer address (cOApp instance) for a corresponding endpoint.
     * @param eid The endpoint ID.
     * @param peer The address of the peer to be associated with the corresponding endpoint.
     */
    function setPeer(uint32 eid, bytes32 peer) external;
}

/**
 * @title   ConfidentialOAppCore
 * @notice  Shared peer registry for a confidential omnichain app (cOApp) relying on a LayerZero-enabled ConfidentialBridge.
 *          A "peer" is the trusted counterpart instance of this cOApp on another chain, addressed by its LayerZero endpoint id (`eid`).
 *          The same registry is used by both the send side ({ConfidentialOAppSender}) and the receive side ({ConfidentialOAppReceiver}), so an
 *          app configures each peer once and it applies in both directions.
 * @dev     App identifiers are `bytes32` so non-EVM peers (e.g. Solana program ids) fit; EVM
 *          peers are the address left-padded to `bytes32`.
 */
abstract contract ConfidentialOAppCore is IConfidentialOAppCore, Ownable {
    /// @notice The trusted peer app per endpoint id (`bytes32(0)` if unset).
    mapping(uint32 eid => bytes32 peer) public peers;

    /**
     * @notice Retrieves the LayerZero ConfidentialBridge address associated with the cOApp.
     * @return cBridge The ConfidentialBridge address.
     * @dev Resolves the ConfidentialBridge from the ACL via {FHE-getLZConfidentialBridgeAddress},
     *      which reverts if the bridge is not deployed on the current chain.
     */
    function LZConfidentialBridgeAddress() public view virtual override returns (address cBridge) {
        cBridge = FHE.getLZConfidentialBridgeAddress();
    }

    /**
     * @notice Configure the canonical peer cOApp on the chain at `eid`. Must be set before
     *         this contract can send to that eid or receive a message from it via the bridge.
     *         Pass bytes32(0) to clear a peer.
     * @param eid LayerZero endpoint id of the remote chain.
     * @param peer Peer app on the remote chain as bytes32. EVM peers: pass
     *              `bytes32(uint256(uint160(remoteAddress)))`.
     *
     * @dev Only the owner/admin of the cOApp can call this function.
     * @dev Indicates that the peer is trusted to send LayerZero messages to this cOApp.
     * @dev Set this to bytes32(0) to remove the peer address.
     * @dev Peer is a bytes32 to accommodate non-evm chains.
     * @dev When setting a non-zero `peer`, this reverts if the ConfidentialBridge is not deployed on the
     *      current chain (`ConfidentialBridgeNotDeployed`), or if it is deployed but not wired to the
     *      destination chain identified by `eid` (`UnsupportedEid`). Clearing a peer (bytes32(0)) skips these checks.
     */
    function setPeer(uint32 eid, bytes32 peer) public virtual onlyOwner {
        _setPeer(eid, peer);
    }

    /**
     * @notice Sets the peer address (cOApp instance) for a corresponding endpoint.
     * @param _eid The endpoint ID.
     * @param _peer The address of the peer to be associated with the corresponding endpoint.
     *
     * @dev Indicates that the peer is trusted to send LayerZero messages to this cOApp.
     * @dev Set this to bytes32(0) to remove the peer address.
     * @dev Peer is a bytes32 to accommodate non-evm chains.
     */
    function _setPeer(uint32 _eid, bytes32 _peer) internal virtual {
        // When configuring a (non-zero) peer, ensure the ConfidentialBridge can route to `_eid`;
        // a zero destination chain id means the eid is unsupported. Clearing a peer (bytes32(0)) is always allowed.
        if (_peer != bytes32(0) && IConfidentialBridge(LZConfidentialBridgeAddress()).getDstChainId(_eid) == 0) {
            revert UnsupportedEid(_eid);
        }
        peers[_eid] = _peer;
        emit PeerSet(_eid, _peer);
    }

    /**
     * @notice Returns the configured peer for `eid`, reverting {NoPeer} if none is set (used on the send side).
     * @param eid The LayerZero endpoint id of the remote chain.
     * @return peer The trusted peer app on the remote chain as bytes32.
     */
    function _getPeerOrRevert(uint32 eid) internal view returns (bytes32 peer) {
        peer = peers[eid];
        if (peer == bytes32(0)) revert NoPeer(eid);
    }
}
