// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

import "../../lib/FHE.sol";
import {CoprocessorSetup} from "../CoprocessorSetup.sol";

import {ConfidentialOApp} from "../../lib/bridge/ConfidentialOApp.sol";

/**
 * @title ConfidentialMultiChainToken
 * @notice Minimalistic confidential cross-chain token built on top of the {ConfidentialOApp}
 *         base (a LayerZero-enabled ConfidentialBridge omnichain app). Holds encrypted
 *         per-user balances and bridges encrypted amounts across chains using the
 *         burn-and-mint pattern: the source-chain instance burns from the sender, and the
 *         destination-chain instance mints to the recipient.
 *
 * @dev    IMPORTANT: This is a deliberately minimalistic example focused on demonstrating the
 *         cross-chain bridging flow, NOT a fully fledged confidential token. In particular it
 *         omits the local `confidentialTransfer`/`confidentialTransferFrom` (and allowance)
 *         functions of a real confidential token, so balances can only move via {mint} and the
 *         cross-chain {send} flow. Do not use as-is in production; extend it with the missing
 *         token functionality first.
 *
 * @dev    An instance of this contract is deployed on each supported chain. Peers are
 *         configured by the owner via {ConfidentialOAppCore-setPeer}, and the canonical
 *         peer per eid serves both directions (outbound send and inbound receive).
 *         The send side is handled by {ConfidentialOAppSender-_sendHandleToPeer},
 *         while inbound messages are authenticated by {ConfidentialOAppReceiver} (which
 *         checks the caller is the local ConfidentialBridge and that `(srcEid, srcApp)`
 *         is a trusted peer) before dispatching to {_onReceiveHandles} to mint.
 */
contract ConfidentialMultiChainToken is ConfidentialOApp {
    event Bridged(address indexed from, uint32 indexed dstEid, address indexed recipient);
    event Received(uint32 indexed srcEid, bytes32 indexed srcApp, address indexed recipient);
    event Minted(address indexed to);

    /// @dev Encrypted balance per holder.
    mapping(address holder => euint64 balance) private _balances;

    /**
     * @param _owner Initial owner (can configure peers and mint the bootstrap supply).
     * @dev    The local ConfidentialBridge is no longer passed in: {ConfidentialOAppCore}
     *         resolves it from the ACL via {ConfidentialOAppCore-LZConfidentialBridgeAddress},
     *         so the coprocessor/ACL must be configured first (done here).
     */
    constructor(address _owner) Ownable(_owner) {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    }

    /**
     * @notice Bridge an encrypted amount of tokens to `recipient` on the chain at `dstEid`.
     * @dev    The destination peer is resolved internally from the owner-configured peer
     *         registry ({ConfidentialOAppCore-setPeer}); bridging to an eid with no
     *         configured peer reverts with {IConfidentialOAppCore-NoPeer}.
     *         Caller must pass enough `msg.value` to cover the LayerZero native fee — query
     *         {quoteSend} from off-chain.
     * @param dstEid           LayerZero endpoint id of the destination chain.
     * @param amount           External encrypted amount to burn-and-send.
     * @param inputProof       Input proof binding `amount` to `(this contract, msg.sender)`.
     * @param recipient        Recipient on the destination chain.
     * @param mintComposeGas   Gas budget for destination-side lzCompose (the mint via {_onReceiveHandles}).
     */
    function send(
        uint32 dstEid,
        externalEuint64 amount,
        bytes calldata inputProof,
        address recipient,
        uint64 mintComposeGas
    ) external payable {
        euint64 amountToSend = FHE.fromExternal(amount, inputProof);

        euint64 actualAmount = _burn(msg.sender, amountToSend);

        // `_burn` granted ACL allowance to this contract on `actualAmount`,
        // so the bridge's `isAllowed(actualAmount, srcApp=this)` check passes.
        bytes memory payload = abi.encode(recipient);
        _sendHandleToPeer(dstEid, payload, actualAmount, mintComposeGas);

        emit Bridged(msg.sender, dstEid, recipient);
    }

    /**
     * @notice Quote the LayerZero native fee for a {send} call, without sending.
     * @dev    Mirrors {send}'s wire shape so the returned fee matches what {send} would
     *         charge: the payload is an abi-encoded `address` (the recipient) and a single
     *         handle is bridged. The fee is a function only of that message shape and the
     *         execution options (derived from `mintComposeGas`), not of the
     *         `amount`/`recipient` values — so this view takes no amount and quotes with a
     *         single null placeholder handle. Reverts with {IConfidentialOAppCore-NoPeer}
     *         when no peer is configured for `dstEid`.
     * @param dstEid           LayerZero endpoint id of the destination chain.
     * @param mintComposeGas   Gas budget for destination-side lzCompose (the mint via {_onReceiveHandles}).
     * @return nativeFee       The LayerZero native fee to forward as `msg.value` to {send}.
     */
    function quoteSend(uint32 dstEid, uint64 mintComposeGas) external view returns (uint256 nativeFee) {
        // Mirror send's message: payload = abi.encode(recipient) and a
        // single bridged handle. Values don't affect the fee, so a placeholder address
        // (e.g here: msg.sender) prices identically to a real send.
        bytes memory payload = abi.encode(msg.sender);
        nativeFee = _quoteSendHandleToPeer(dstEid, payload, mintComposeGas);
    }

    /**
     * @notice Owner-gated mint of an encrypted `amount` to `to`. Intended for testing /
     *         bootstrap of an initial supply on each chain (the production token economy
     *         is governed by the burn-and-mint bridge flow).
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

    /**
     * @notice Mint to the recipient once {ConfidentialOAppReceiver} has authenticated the
     *         inbound bridge message (trusted local ConfidentialBridge caller and trusted
     *         `(srcEid, srcApp)` peer).
     * @dev    The recipient is carried in `payload` (encoded by the source peer's {send}),
     *         and the bridged amount is the first (and only) derived destination handle,
     *         which the bridge has already granted this contract transient ACL allowance on.
     */
    function _onReceiveHandles(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata /* srcHandleList */,
        bytes32[] calldata dstHandleList,
        bytes32 /* guid */
    ) internal override {
        address recipient = abi.decode(payload, (address));
        euint64 dstAmount = euint64.wrap(dstHandleList[0]);

        _mint(recipient, dstAmount);

        emit Received(srcEid, srcApp, recipient);
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
