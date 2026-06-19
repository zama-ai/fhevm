// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @notice Bridge destination callback (mirror of host-contracts' IDstApp; inlined since the
///         bridge interfaces aren't part of the e2e contract project).
interface IDstApp {
    function onConfidentialBridgeReceived(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata srcHandleList,
        bytes32[] calldata dstHandleList,
        bytes32 guid
    ) external;
}

/// @notice Test dapp exercising the confidential bridge end-to-end (deployed on each chain).
/// @dev Source: {makeHandle}/{makeComputedHandle} produce a handle ACL-allowed to the caller for
///      `ConfidentialBridge.send`. Destination: {onConfidentialBridgeReceived} (called by the bridge's lzCompose, which
///      has granted transient ACL allowance) makes each bridged handle decryptable so the test can
///      assert it — publicly when `payload` is empty, or to a user when `payload` encodes an address.
contract BridgeApp is E2ECoprocessorConfig, IDstApp {
    /// @notice Lets the test read the produced handle from the receipt.
    event HandleMinted(bytes32 handle);

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

    /// @notice Bridge callback: empty `payload` => make each handle publicly decryptable;
    ///         `payload == abi.encode(address user)` => grant persistent ACL allowance to `user`
    ///         (and this contract) so it can be user-decrypted.
    /// @dev The transient ACL allowance granted before this call suffices within the lzCompose tx.
    function onConfidentialBridgeReceived(
        uint32 /* srcEid */,
        bytes32 /* srcApp */,
        bytes calldata payload,
        bytes32[] calldata /* srcHandleList */,
        bytes32[] calldata dstHandleList,
        bytes32 /* guid */
    ) external override {
        address user = payload.length == 32 ? abi.decode(payload, (address)) : address(0);
        for (uint256 i = 0; i < dstHandleList.length; i++) {
            euint64 handle = euint64.wrap(dstHandleList[i]);
            if (user == address(0)) {
                FHE.makePubliclyDecryptable(handle);
            } else {
                FHE.allowThis(handle);
                FHE.allow(handle, user);
            }
        }
    }
}
