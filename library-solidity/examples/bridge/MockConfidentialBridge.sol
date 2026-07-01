// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IConfidentialBridge, MessagingFee, MessagingReceipt} from "../../lib/bridge/IConfidentialBridge.sol";
import {IDstApp} from "../../lib/bridge/IConfidentialBridge.sol";

/**
 * @title   MockConfidentialBridge
 * @notice  Test double for the host `ConfidentialBridge`. Records the arguments and
 *          `msg.value` of the last {send} call so tests can assert that the `FHE.bridge`
 *          wrapper forwards everything verbatim, and lets a test drive the receive side by
 *          replaying {IDstApp.onConfidentialBridgeReceived} on a destination app.
 * @dev     Test-only stand-in for the real on-chain `ConfidentialBridge`; not part of the
 *          published library. It does no real cross-chain messaging — it just records what was
 *          sent and lets a test replay a delivery into a receiver.
 */
contract MockConfidentialBridge is IConfidentialBridge {
    struct SendCall {
        uint32 dstEid;
        bytes32 dstApp;
        bytes payload;
        bytes32[] handleList;
        uint64 lzComposeGas;
        uint256 value;
        address caller;
    }

    SendCall private _lastSend;
    bool public sendCalled;

    /// @dev Native fee returned by {quote}; settable so tests can exercise fee plumbing.
    uint256 public quotedNativeFee;

    /// @dev Fee {send} actually charges. When 0 (default) it charges the full `msg.value` (no
    ///      excess); when set below `msg.value` it refunds the difference to the caller — mirroring
    ///      how the LayerZero endpoint pushes back native-fee change — so refund paths are testable.
    uint256 public chargedFee;

    event Sent(uint32 indexed dstEid, bytes32 indexed dstApp, address indexed caller, uint256 value);

    function setQuotedNativeFee(uint256 fee) external {
        quotedNativeFee = fee;
    }

    function setChargedFee(uint256 fee) external {
        chargedFee = fee;
    }

    function send(
        uint32 dstEid,
        bytes32 dstApp,
        bytes calldata payload,
        bytes32[] calldata handleList,
        uint64 lzComposeGas
    ) external payable override returns (MessagingReceipt memory receipt) {
        _lastSend = SendCall({
            dstEid: dstEid,
            dstApp: dstApp,
            payload: payload,
            handleList: handleList,
            lzComposeGas: lzComposeGas,
            value: msg.value,
            caller: msg.sender
        });
        sendCalled = true;
        emit Sent(dstEid, dstApp, msg.sender, msg.value);

        // Charge `chargedFee` (or the full value when unset) and refund any excess to the caller,
        // as the real endpoint does — so the receipt's `fee.nativeFee` can be below `msg.value`.
        uint256 fee = chargedFee == 0 ? msg.value : chargedFee;
        if (msg.value > fee) {
            (bool ok, ) = payable(msg.sender).call{value: msg.value - fee}("");
            require(ok, "mock refund failed");
        }

        // Deterministic receipt so tests can assert pass-through of the return value.
        receipt = MessagingReceipt({
            guid: keccak256(abi.encode(dstEid, dstApp, payload)),
            nonce: 1,
            fee: MessagingFee({nativeFee: fee, lzTokenFee: 0})
        });
    }

    function quote(
        uint32,
        address,
        bytes32,
        bytes calldata,
        bytes32[] calldata,
        uint64
    ) external view override returns (MessagingFee memory fee) {
        fee = MessagingFee({nativeFee: quotedNativeFee, lzTokenFee: 0});
    }

    // -------- assertions surface --------

    function lastSend() external view returns (SendCall memory) {
        return _lastSend;
    }

    /// @notice Replay a derived-handle delivery onto `dstApp`, impersonating the bridge.
    function deliver(
        address dstApp,
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata srcHandleList,
        bytes32[] calldata dstHandleList,
        bytes32 guid
    ) external {
        IDstApp(dstApp).onConfidentialBridgeReceived(srcEid, srcApp, payload, srcHandleList, dstHandleList, guid);
    }
}
