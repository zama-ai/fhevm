// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";
import {ConfidentialOAppSender} from "@fhevm/solidity/lib/bridge/ConfidentialOAppSender.sol";
import {ConfidentialOAppReceiver} from "@fhevm/solidity/lib/bridge/ConfidentialOAppReceiver.sol";
import {MessagingReceipt} from "@fhevm/solidity/lib/bridge/IConfidentialBridge.sol";

/// @notice Test dapp exercising the confidential bridge end-to-end THROUGH the `@fhevm/solidity`
///         OApp wrappers (rather than calling the host `ConfidentialBridge` directly).
/// @dev It sends via {ConfidentialOAppSender-_bridge} (peer resolved from the OApp registry) and
///      receives via {ConfidentialOAppReceiver}, which authenticates `msg.sender == bridge` and
///      `isPeer(srcEid, srcApp)` before dispatching to {_onReceiveHandles}. Peers are wired once
///      with {setPeer} (one entry serves both directions). `setPeer` is intentionally unguarded
///      (test-only).
///
///      Source: {makeHandle}/{makeComputedHandle} produce a handle ACL-allowed to the caller and
///      this contract (so the bridge's `isAllowed(handle, this)` check passes on send).
///      Destination: {_onReceiveHandles} (post-auth) makes each bridged handle decryptable so the
///      test can assert it — publicly when `payload` is empty, or to a user when `payload` encodes
///      an address.
contract BridgeApp is E2ECoprocessorConfig, ConfidentialOAppSender, ConfidentialOAppReceiver {
    /// @notice Lets the test read the produced handle from the receipt.
    event HandleMinted(bytes32 handle);

    /// @notice Trust the remote app as this app's peer on `eid` (serves send + receive).
    function setPeer(uint32 eid, bytes32 peer) external {
        _setPeer(eid, peer);
    }

    /// @notice Verify an encrypted input and register it as a bridgeable handle.
    function makeHandle(externalEuint64 encryptedAmount, bytes calldata inputProof) external returns (bytes32) {
        return _register(FHE.fromExternal(encryptedAmount, inputProof));
    }

    /// @notice Like {makeHandle} but registers a computed handle (`amount + addend`), so the e2e
    ///         covers a computation-result ciphertext, not just a raw input one.
    function makeComputedHandle(
        externalEuint64 encryptedAmount,
        bytes calldata inputProof,
        uint64 addend
    ) external returns (bytes32) {
        return _register(FHE.add(FHE.fromExternal(encryptedAmount, inputProof), FHE.asEuint64(addend)));
    }

    /// @dev Allow `value` for the caller (the future send sender) and this contract, and return its handle.
    function _register(euint64 value) private returns (bytes32 handle) {
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        handle = euint64.unwrap(value);
        emit HandleMinted(handle);
    }

    /// @notice Bridge `handles` to this app's peer on `dstEid`, via the OApp sender wrapper.
    /// @dev Uses the multi-handle {ConfidentialOAppSender-_bridge}, which resolves the peer from
    ///      the registry; this contract must already hold ACL allowance on every handle (see
    ///      {makeHandle}).
    function send(
        uint32 dstEid,
        bytes32[] calldata handles,
        bytes calldata payload,
        uint64 lzComposeGas
    ) external payable returns (MessagingReceipt memory) {
        return _bridge(dstEid, payload, handles, lzComposeGas, msg.value);
    }

    /// @notice Receive hook (post-auth): empty `payload` => make each handle publicly decryptable;
    ///         `payload == abi.encode(address user)` => grant persistent ACL allowance to `user`.
    /// @dev Only reached after {ConfidentialOAppReceiver} has verified the caller is the bridge and
    ///      `(srcEid, srcApp)` is a registered peer. Transient ACL allowance is already granted.
    function _onReceiveHandles(
        uint32 /* srcEid */,
        bytes32 /* srcApp */,
        bytes calldata payload,
        bytes32[] calldata handles,
        bytes32 /* guid */
    ) internal override {
        address user = payload.length == 32 ? abi.decode(payload, (address)) : address(0);
        for (uint256 i = 0; i < handles.length; i++) {
            euint64 handle = euint64.wrap(handles[i]);
            if (user == address(0)) {
                FHE.makePubliclyDecryptable(handle);
            } else {
                FHE.allowThis(handle);
                FHE.allow(handle, user);
            }
        }
    }
}
