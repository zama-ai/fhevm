// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable2Step.sol";

import {FHE, euint64, externalEuint64, ebool} from "../../lib/FHE.sol";
import {ZamaConfig} from "../../config/ZamaConfig.sol";
import {ConfidentialOAppSender} from "../../lib/bridge/ConfidentialOAppSender.sol";
import {ConfidentialOAppReceiver} from "../../lib/bridge/ConfidentialOAppReceiver.sol";
import {MessagingFee, MessagingReceipt} from "../../lib/bridge/IConfidentialBridge.sol";

/**
 * @title   ConfidentialOFTViaLib
 * @notice  Example confidential token that can be bridged across chains, built entirely on the
 *          `fhevm/solidity` library. It is a confidential omnichain app (OApp): it inherits
 *          {ConfidentialOAppSender} for the send half and {ConfidentialOAppReceiver} for the
 *          receive half, which together give it:
 *            - Sending: `_bridge(...)` burns an encrypted amount here and has it minted on the
 *              destination chain. The app never touches LayerZero or the bridge contract.
 *            - Receiving: one hook ({_onReceiveHandles}) mints the incoming amount; the base
 *              authenticates the bridge and the source peer first.
 *
 * @dev     An instance is deployed on each supported chain and wired to its peers once via
 *          {setPeer} (the peer applies to both directions). The app only deals with its own
 *          burn/mint logic and a `euint64` amount.
 */
contract ConfidentialOFTViaLib is Ownable2Step, ConfidentialOAppSender, ConfidentialOAppReceiver {
    event Bridged(address indexed from, uint32 indexed dstEid, address indexed recipient);
    event Received(uint32 indexed srcEid, address indexed recipient);

    error UnauthorizedUseOfEncryptedAmount(euint64 amount, address sender);
    error SweepFailed();

    mapping(address holder => euint64 balance) private _balances;

    constructor(address _owner) Ownable(_owner) {
        FHE.setCoprocessor(ZamaConfig.getEthereumCoprocessorConfig());
        // Both directions resolve the ConfidentialBridge from the ACL configured above:
        // the send path through `FHE.bridge` and the receive path through
        // {ConfidentialOAppReceiver}. Nothing else to wire — and the two anchors can never diverge.
    }

    /// @dev Accept native-fee refunds the bridge / LayerZero endpoint may push back on `send`.
    receive() external payable {}

    // ----------------------------- Send side -----------------------------

    /**
     * @notice Burn `amount` locally and bridge it to `recipient` on `dstEid`.
     * @dev    Quote the LayerZero fee with {quoteSend} and forward it as `msg.value`; any excess
     *         is refunded to this contract (see `receive()` / {sweep}).
     *         NOTE: FHE cannot revert on an encrypted comparison, so if `amount` exceeds the
     *         caller's balance the burn yields an encrypted 0 and a zero-value message is still
     *         bridged (the caller pays the fee for a no-op `_mint`). Callers should confirm a
     *         sufficient balance off-chain before sending. This matches the host
     *         `ConfidentialOFT` example's burn-and-mint semantics.
     * @param mintComposeGas Gas budget for the destination mint (lzCompose leg).
     */
    function send(
        uint32 dstEid,
        euint64 amount,
        address recipient,
        uint128 mintComposeGas
    ) external payable returns (MessagingReceipt memory) {
        if (!FHE.isSenderAllowed(amount)) revert UnauthorizedUseOfEncryptedAmount(amount, msg.sender);

        euint64 actualAmount = _burn(msg.sender, amount);

        // The bridged amount is delivered out-of-band in the bridge's handle list (minted from
        // `handles[0]` on receipt), so the payload only needs to carry the recipient.
        // `mintComposeGas` is validated (must be non-zero) by {ConfidentialOAppSender-_bridge}.
        bytes memory payload = abi.encode(recipient);

        emit Bridged(msg.sender, dstEid, recipient);
        return _bridge(dstEid, payload, actualAmount, mintComposeGas, msg.value);
    }

    /// @notice Quote the native fee to {send} `amount` to `recipient` on `dstEid`; pass the
    ///         returned `nativeFee` as `msg.value` to {send}.
    function quoteSend(
        uint32 dstEid,
        euint64 amount,
        address recipient,
        uint128 mintComposeGas
    ) external view returns (MessagingFee memory) {
        return _quoteBridge(dstEid, abi.encode(recipient), amount, mintComposeGas);
    }

    // ---------------------------- Receive side ----------------------------

    /// @inheritdoc ConfidentialOAppReceiver
    function _onReceiveHandles(
        uint32 srcEid,
        bytes32 /* srcApp */,
        bytes calldata payload,
        bytes32[] calldata handles,
        bytes32 /* guid */
    ) internal override {
        // The bridge delivers each message's compose exactly once (a retry only follows a revert),
        // so this unconditional mint is safe. If your receive hook is NOT idempotent, track `guid`
        // and dedupe to avoid a double-mint on retry.
        address recipient = abi.decode(payload, (address));
        _mint(recipient, euint64.wrap(handles[0]));
        emit Received(srcEid, recipient);
    }

    // ------------------------------- Admin --------------------------------

    /// @notice Register the remote OFT peer for `eid` (applies to both send and receive).
    ///         For an EVM peer, pass the remote address left-padded to bytes32.
    function setPeer(uint32 eid, bytes32 peer) external onlyOwner {
        _setPeer(eid, peer);
    }

    function mint(address to, externalEuint64 encryptedAmount, bytes calldata inputProof) external onlyOwner {
        euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);
        _mint(to, amount);
    }

    function balanceOf(address holder) external view returns (euint64) {
        return _balances[holder];
    }

    /// @notice Withdraw native balance accumulated from bridge fee refunds (see `receive()`),
    ///         so overpaid fees aren't trapped in the contract.
    function sweep(address to) external onlyOwner {
        (bool ok, ) = to.call{value: address(this).balance}("");
        if (!ok) revert SweepFailed();
    }

    // ------------------------------ Internals -----------------------------

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
