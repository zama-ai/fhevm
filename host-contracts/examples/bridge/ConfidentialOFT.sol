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
 *         configured by the owner via `setTrustedPeer`. The ConfidentialBridge contract
 *         on this chain dispatches the cross-chain send and, on the destination chain,
 *         invokes `onReceive` via lzCompose.
 */
contract ConfidentialOFT is Ownable2Step, IDstApp {
    event Bridged(address indexed from, uint32 indexed dstEid, address indexed recipient);
    event Received(uint32 indexed srcEid, bytes32 indexed srcApp, address indexed recipient);
    event TrustedPeerSet(uint32 indexed srcEid, bytes32 indexed srcApp, bool trusted);
    event Minted(address indexed to);

    error UntrustedPeer(uint32 srcEid, bytes32 srcApp);
    error OnlyConfidentialBridge(address caller);

    /// @notice ConfidentialBridge on this chain. Used both to dispatch outbound sends and
    ///         to authenticate inbound `onReceive` calls (the bridge is its own lzCompose
    ///         dispatcher, so the bridge address is also the only authorized caller of
    ///         `onReceive`).
    ConfidentialBridge public immutable confidentialBridge;

    /// @dev Per-chain trusted peer apps. Bridging from an untrusted (srcEid, srcApp)
    ///      pair is rejected in `onReceive`.
    ///      Keyed by `bytes32 srcApp` rather than `address` to support non-EVM peers
    ///      (e.g. Solana program IDs). For EVM peers, pass
    ///      `bytes32(uint256(uint160(remoteEvmAddress)))`.
    mapping(uint32 srcEid => mapping(bytes32 srcApp => bool trusted)) private _trustedPeers;

    /// @dev Encrypted balance per holder.
    mapping(address holder => euint64 balance) private _balances;

    constructor(address _confidentialBridge, address _owner) Ownable(_owner) {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
        confidentialBridge = ConfidentialBridge(_confidentialBridge);
    }

    /**
     * @notice Bridge an encrypted amount of tokens to `recipient` on the chain at `dstEid`.
     * @dev    Caller must currently have ACL allowance on `amount`. Pass enough
     *         `msg.value` to cover the LayerZero native fee — query
     *         `ConfidentialBridge.quote(...)` from off-chain.
     * @param dstEid           LayerZero endpoint id of the destination chain.
     * @param dstApp           Peer ConfidentialOFT on the destination chain as bytes32.
     *                         For EVM destinations pass
     *                         `bytes32(uint256(uint160(remoteOftAddress)))`.
     * @param amount           Encrypted amount handle to burn-and-send.
     * @param recipient        Recipient on the destination chain.
     * @param mintComposeGas   Gas budget for destination-side lzCompose (the `onReceive`).
     */
    function send(
        uint32 dstEid,
        bytes32 dstApp,
        euint64 amount,
        address recipient,
        uint128 mintComposeGas
    ) external payable {
        require(FHE.isSenderAllowed(amount), "ConfidentialOFT: sender not allowed on amount");

        euint64 actualAmount = _burn(msg.sender, amount);

        // `_burn` granted persistent ACL allowance to this contract on `actualAmount`,
        // so the bridge's `isAllowed(actualAmount, srcApp=this)` check passes.
        bytes memory payload = abi.encode(recipient, actualAmount);
        bytes32[] memory handleList = new bytes32[](1);
        handleList[0] = euint64.unwrap(actualAmount);

        // Empty `options` lets the bridge build defaults using its lzReceiveGas formula
        // and `mintComposeGas`.
        confidentialBridge.send{value: msg.value}(dstEid, dstApp, payload, handleList, mintComposeGas, "");

        emit Bridged(msg.sender, dstEid, recipient);
    }

    /**
     * @notice ConfidentialBridge dispatches here in lzCompose to mint to the recipient.
     * @dev    Authentication:
     *         - msg.sender must be the trusted ConfidentialBridge on this chain.
     *         - (srcEid, srcApp) must be a peer the owner has trusted.
     *         Reverting from `onReceive` only blocks the app-level mint; the bridge's
     *         protocol-level association (already settled by the coprocessor) is
     *         independent and unaffected.
     */
    function onReceive(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata /* srcHandleList */,
        bytes32[] calldata dstHandleList
    ) external override {
        if (msg.sender != address(confidentialBridge)) revert OnlyConfidentialBridge(msg.sender);
        if (!_trustedPeers[srcEid][srcApp]) revert UntrustedPeer(srcEid, srcApp);

        (address recipient, ) = abi.decode(payload, (address, bytes32));
        euint64 dstAmount = euint64.wrap(dstHandleList[0]);

        _mint(recipient, dstAmount);

        emit Received(srcEid, srcApp, recipient);
    }

    /// @notice Configure a per-chain trusted peer app. Must be set before that peer
    ///         can mint into this contract via the bridge.
    /// @param srcApp Peer app on the source chain as bytes32. EVM peers: pass
    ///        `bytes32(uint256(uint160(remoteAddress)))`.
    function setTrustedPeer(uint32 srcEid, bytes32 srcApp, bool trusted) external onlyOwner {
        _trustedPeers[srcEid][srcApp] = trusted;
        emit TrustedPeerSet(srcEid, srcApp, trusted);
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

    /// @notice Returns whether `(srcEid, srcApp)` is a trusted bridging peer.
    function isTrustedPeer(uint32 srcEid, bytes32 srcApp) external view returns (bool) {
        return _trustedPeers[srcEid][srcApp];
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
