// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @notice Bridge destination callback (mirror of host-contracts' IDstApp; inlined since the
///         bridge interfaces aren't part of the e2e contract project).
interface IDstApp {
    function onReceive(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata srcHandleList,
        bytes32[] calldata dstHandleList
    ) external;
}

/// @notice Test dapp exercising the confidential bridge end-to-end (deployed on each chain).
/// @dev Source: {makeHandle} produces a handle ACL-allowed to the caller for `ConfidentialBridge.send`.
///      Destination: {onReceive} (called by the bridge's lzCompose, which has granted transient
///      ACL allowance) makes each bridged handle publicly decryptable so the test can assert it.
contract BridgeProbe is E2ECoprocessorConfig, IDstApp {
    /// @notice Lets the test read the produced handle from the receipt.
    event HandleMinted(bytes32 handle);

    /// @notice Verify an encrypted input and allow it for the caller (the future send sender) and this contract.
    function makeHandle(externalEuint64 encryptedAmount, bytes calldata inputProof) external returns (bytes32 handle) {
        euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);
        FHE.allowThis(amount);
        FHE.allow(amount, msg.sender);
        handle = euint64.unwrap(amount);
        emit HandleMinted(handle);
    }

    /// @notice Bridge callback: make every derived destination handle publicly decryptable.
    /// @dev The transient ACL allowance granted before this call suffices within the lzCompose tx.
    function onReceive(
        uint32 /* srcEid */,
        bytes32 /* srcApp */,
        bytes calldata /* payload */,
        bytes32[] calldata /* srcHandleList */,
        bytes32[] calldata dstHandleList
    ) external override {
        for (uint256 i = 0; i < dstHandleList.length; i++) {
            FHE.makePubliclyDecryptable(euint64.wrap(dstHandleList[i]));
        }
    }
}
