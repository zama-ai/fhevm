// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable2Step.sol";

import "../../lib/FHE.sol";
import {CoprocessorSetup} from "../../lib/CoprocessorSetup.sol";

import {ConfidentialBridge} from "../../contracts/bridge/ConfidentialBridge.sol";
import {IDstApp} from "../../contracts/bridge/interfaces/IDstApp.sol";

/**
 * @title ConfidentialOFT
 * @notice Minimalistic confidential cross-chain token using the LayerZero handle bridge.
 *         Holds encrypted per-user balances and bridges encrypted amounts across chains
 *         using the burn-and-mint pattern: the source-chain instance burns from the
 *         sender, and the destination-chain instance mints to the recipient.
 *
 * @dev    An instance of this contract is deployed on each supported chain. Peers are
 *         configured by the owner via `setPeer`.
 *         The ConfidentialBridge contract on this chain dispatches the cross-chain send and,
 *         on the destination chain, invokes `onConfidentialBridgeReceived` via lzCompose.
 */
contract ConfidentialOFT is Ownable2Step, IDstApp {
    event Bridged(address indexed from, uint32 indexed dstEid, address indexed recipient);
    event Received(uint32 indexed srcEid, bytes32 indexed srcApp, address indexed recipient);
    event PeerSet(uint32 indexed eid, bytes32 indexed peer);
    event Minted(address indexed to);

    error UntrustedPeer(uint32 srcEid, bytes32 srcApp);
    error PeerNotSet(uint32 dstEid);
    error OnlyConfidentialBridge(address caller);
    error UnauthorizedUseOfEncryptedAmount(euint64 amount, address sender);

    /// @notice ConfidentialBridge on this chain. Used both to dispatch outbound sends and
    ///         to authenticate inbound `onConfidentialBridgeReceived` calls (the bridge is its own lzCompose
    ///         dispatcher, so the bridge address is also the only authorized caller of
    ///         `onConfidentialBridgeReceived`).
    ConfidentialBridge public immutable confidentialBridge;

    /// @dev The canonical peer app on each remote chain, keyed by eid. A single peer per
    ///      eid serves both directions: outbound `send` dispatches to `_peers[dstEid]`,
    ///      and inbound `onConfidentialBridgeReceived` rejects any `(srcEid, srcApp)` that doesn't match
    ///      `_peers[srcEid]`.
    ///      Stored as `bytes32` rather than `address` to support non-EVM peers
    ///      (e.g. Solana program IDs). For EVM peers, pass
    ///      `bytes32(uint256(uint160(remoteEvmAddress)))`.
    mapping(uint32 eid => bytes32 peer) private _peers;

    /// @dev Encrypted balance per holder.
    mapping(address holder => euint64 balance) private _balances;

    constructor(address _confidentialBridge, address _owner) Ownable(_owner) {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
        confidentialBridge = ConfidentialBridge(_confidentialBridge);
    }

    /**
     * @notice Bridge an encrypted amount of tokens to `recipient` on the chain at `dstEid`.
     * @dev    The destination peer is resolved internally from the owner-configured peer
     *         registry (`setPeer`); bridging to an eid with no configured peer reverts
     *         with `PeerNotSet`. Caller must currently have ACL allowance on `amount`.
     *         Pass enough `msg.value` to cover the LayerZero native fee — query
     *         `ConfidentialBridge.quote(...)` from off-chain.
     * @param dstEid           LayerZero endpoint id of the destination chain.
     * @param amount           Encrypted amount handle to burn-and-send.
     * @param recipient        Recipient on the destination chain.
     * @param mintComposeGas   Gas budget for destination-side lzCompose (the `onConfidentialBridgeReceived`).
     */
    function send(
        uint32 dstEid,
        euint64 amount,
        address recipient,
        uint64 mintComposeGas
    ) external payable {
        if(!FHE.isSenderAllowed(amount)) revert UnauthorizedUseOfEncryptedAmount(amount, msg.sender);

        bytes32 dstApp = _peers[dstEid];
        if (dstApp == bytes32(0)) revert PeerNotSet(dstEid);

        euint64 actualAmount = _burn(msg.sender, amount);

        // `_burn` granted persistent ACL allowance to this contract on `actualAmount`,
        // so the bridge's `isAllowed(actualAmount, srcApp=this)` check passes.
        bytes memory payload = abi.encode(recipient, actualAmount);
        bytes32[] memory handleList = new bytes32[](1);
        handleList[0] = euint64.unwrap(actualAmount);

        confidentialBridge.send{value: msg.value}(dstEid, dstApp, payload, handleList, mintComposeGas);

        emit Bridged(msg.sender, dstEid, recipient);
    }

    /**
     * @notice ConfidentialBridge dispatches here in lzCompose to mint to the recipient.
     * @dev    Authentication:
     *         - msg.sender must be the trusted ConfidentialBridge on this chain.
     *         - (srcEid, srcApp) must be a peer the owner has trusted.
     *         Reverting from `onConfidentialBridgeReceived` only blocks the app-level mint; the bridge's
     *         protocol-level association (already settled by the coprocessor) is
     *         independent and unaffected.
     */
    function onConfidentialBridgeReceived(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata /* srcHandleList */,
        bytes32[] calldata dstHandleList,
        bytes32 /* guid */
    ) external override {
        if (msg.sender != address(confidentialBridge)) revert OnlyConfidentialBridge(msg.sender);
        bytes32 trustedPeer = _peers[srcEid];
        if (trustedPeer == bytes32(0) || trustedPeer != srcApp) revert UntrustedPeer(srcEid, srcApp);

        (address recipient, ) = abi.decode(payload, (address, bytes32));
        euint64 dstAmount = euint64.wrap(dstHandleList[0]);

        _mint(recipient, dstAmount);

        emit Received(srcEid, srcApp, recipient);
    }

    /// @notice Configure the canonical peer app on the chain at `eid`. Must be set before
    ///         this contract can `send` to that eid or accept a mint from it via the bridge.
    ///         Pass `bytes32(0)` to clear a peer.
    /// @param eid  LayerZero endpoint id of the remote chain.
    /// @param peer Peer app on the remote chain as bytes32. EVM peers: pass
    ///        `bytes32(uint256(uint160(remoteAddress)))`.
    function setPeer(uint32 eid, bytes32 peer) external onlyOwner {
        _peers[eid] = peer;
        emit PeerSet(eid, peer);
    }

    /**
     * @notice Owner-gated mint of an encrypted `amount` to `to`. Intended for testing /
     *         bootstrap of an initial supply on each chain (the production token economy
     *         is governed by the burn-and-mint bridge flow).
     * @dev    `encryptedAmount` + `inputProof` are produced off-chain by the relayer
     *         (or by the mock-coprocessor's `mock:encrypt` CLI for local/testnet
     *         testing) and are bound to `(this contract, msg.sender)` by the on-chain
     *         InputVerifier via EIP-712 — so the owner wallet must be the same one
     *         that requested the encryption.
     *         The new balance handle is granted persistent ACL allowance to both `this`
     *         (for future burns) and `to` (for client-side decryption); see `_mint`.
     * @dev    Owner should beware total sum of minted amounts on all chains must not exceed type(uint64).max.
     */
    function mint(address to, externalEuint64 encryptedAmount, bytes calldata inputProof) external onlyOwner {
        euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);
        _mint(to, amount);
        emit Minted(to);
    }

    /// @notice Returns the encrypted balance handle for `holder`.
    function balanceOf(address holder) external view returns (euint64) {
        return _balances[holder];
    }

    /// @notice Returns the configured peer app on the chain at `eid` (bytes32(0) if unset).
    function peers(uint32 eid) external view returns (bytes32) {
        return _peers[eid];
    }

    /**
     * @dev Burn `amount` from `from` and return the actually burned amount (which equals
     *      `amount` only when balance suffices; otherwise 0). Standard safe-math pattern
     *      mirroring EncryptedERC20.
     */
    function _burn(address from, euint64 amount) internal returns (euint64 actualAmount) {
        euint64 fromBalance = _balances[from];
        ebool canBurn = FHE.le(amount, fromBalance);
        actualAmount = FHE.select(canBurn, amount, FHE.asEuint64(0));

        euint64 newBalance = FHE.sub(fromBalance, actualAmount);
        _balances[from] = newBalance;
        FHE.allowThis(newBalance);
        FHE.allow(newBalance, from);
        FHE.allowThis(actualAmount);
    }

    function _mint(address to, euint64 amount) internal {
        euint64 newBalance = FHE.add(_balances[to], amount);
        _balances[to] = newBalance;
        FHE.allowThis(newBalance);
        FHE.allow(newBalance, to);
    }
}
