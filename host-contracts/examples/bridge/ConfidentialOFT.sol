// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable2Step.sol";

import "../../lib/FHE.sol";
import {CoprocessorSetup} from "../../lib/CoprocessorSetup.sol";

import {ConfidentialBridge} from "../../contracts/bridge/ConfidentialBridge.sol";
import {IDstApp} from "../../contracts/bridge/interfaces/IDstApp.sol";

/**
 * @title ConfidentialOFT
 * @notice Reference confidential cross-chain token using the LayerZero handle bridge.
 *         Holds encrypted per-user balances and bridges encrypted amounts across chains
 *         using the burn-and-mint pattern: the source-chain instance burns from the
 *         sender, and the destination-chain instance mints to the recipient.
 *
 * @dev    An instance of this contract is deployed on each supported chain. Peers are
 *         configured by the owner via `setTrustedPeer`. The ConfidentialBridge contract
 *         on this chain dispatches the cross-chain send and, on the destination chain,
 *         invokes `onReceive` via lzCompose.
 *
 *         This example is intentionally minimal: it focuses on the bridge integration
 *         and not on ERC7984 ergonomics (operators, approvals, etc.) which the spec
 *         calls out as orthogonal.
 */
contract ConfidentialOFT is Ownable2Step, IDstApp {
    event Bridged(address indexed from, uint32 indexed dstEid, address indexed recipient);
    event Received(uint32 indexed srcEid, address indexed srcApp, address indexed recipient);
    event TrustedPeerSet(uint32 indexed srcEid, address indexed srcApp, bool trusted);

    error UntrustedPeer(uint32 srcEid, address srcApp);
    error OnlyConfidentialBridge(address caller);

    /// @notice ConfidentialBridge on this chain. Used both to dispatch outbound sends and
    ///         to authenticate inbound `onReceive` calls (the bridge is its own lzCompose
    ///         dispatcher, so the bridge address is also the only authorized caller of
    ///         `onReceive`).
    ConfidentialBridge public immutable confidentialBridge;

    /// @dev Per-chain trusted peer apps. Bridging from an untrusted (srcEid, srcApp)
    ///      pair is rejected in `onReceive`. The recipient's destination balance handle
    ///      ends up associated with no ciphertext from a compromised chain otherwise —
    ///      a targeted DoS — so apps choose what they trust (RFC 008 §Example).
    mapping(uint32 srcEid => mapping(address srcApp => bool trusted)) private _trustedPeers;

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
     * @param dstApp           Peer ConfidentialOFT address on the destination chain.
     * @param amount           Encrypted amount handle to burn-and-send.
     * @param recipient        Recipient on the destination chain.
     * @param mintComposeGas   Gas budget for destination-side lzCompose (the `onReceive`).
     */
    function send(
        uint32 dstEid,
        address dstApp,
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
        address srcApp,
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
    function setTrustedPeer(uint32 srcEid, address srcApp, bool trusted) external onlyOwner {
        _trustedPeers[srcEid][srcApp] = trusted;
        emit TrustedPeerSet(srcEid, srcApp, trusted);
    }

    /// @notice Returns the encrypted balance handle for `holder`.
    function balanceOf(address holder) external view returns (euint64) {
        return _balances[holder];
    }

    /// @notice Returns whether `(srcEid, srcApp)` is a trusted bridging peer.
    function isTrustedPeer(uint32 srcEid, address srcApp) external view returns (bool) {
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
