// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, euint64} from "../../lib/FHE.sol";
import {CoprocessorConfig} from "../../lib/Impl.sol";
import {MessagingReceipt} from "../../lib/bridge/IConfidentialBridge.sol";
import {ConfidentialOAppSender} from "../../lib/bridge/ConfidentialOAppSender.sol";
import {ConfidentialOAppReceiver} from "../../lib/bridge/ConfidentialOAppReceiver.sol";

/**
 * @title   OAppHarness
 * @notice  Minimal confidential OApp (both {ConfidentialOAppSender} and
 *          {ConfidentialOAppReceiver}) used to exercise the abstracts: it sends through the
 *          sender helper and records the last inbound delivery so a test can assert auth +
 *          dispatch. The constructor wires the ACL (which resolves the trusted bridge). `setPeer`
 *          is exposed without access control on purpose (test-only); `setTrustAllPeers` flips an
 *          {isPeer} override to exercise the custom-trust-policy extension point.
 * @dev     Test-only helper; not part of the published library.
 */
contract OAppHarness is ConfidentialOAppSender, ConfidentialOAppReceiver {
    uint32 public lastSrcEid;
    bytes32 public lastSrcApp;
    bytes public lastPayload;
    bytes32[] public lastHandles;
    bytes32 public lastGuid;
    uint256 public receiveCount;
    bool public trustAllPeers;

    constructor(address acl) {
        FHE.setCoprocessor(
            CoprocessorConfig({ACLAddress: acl, CoprocessorAddress: address(1), KMSVerifierAddress: address(2)})
        );
    }

    function setPeer(uint32 eid, bytes32 peer) external {
        _setPeer(eid, peer);
    }

    function setTrustAllPeers(bool v) external {
        trustAllPeers = v;
    }

    /// @dev Send a single raw handle to the peer on `dstEid` via the sender helper.
    function bridgeToPeer(
        uint32 dstEid,
        bytes calldata payload,
        bytes32 handle,
        uint128 lzComposeGas
    ) external payable returns (MessagingReceipt memory) {
        return _bridge(dstEid, payload, euint64.wrap(handle), lzComposeGas, msg.value);
    }

    function lastHandlesLength() external view returns (uint256) {
        return lastHandles.length;
    }

    /// @dev Custom trust policy: accept any peer when `trustAllPeers`, else the default registry.
    function isPeer(uint32 eid, bytes32 peer) public view override returns (bool) {
        if (trustAllPeers) return true;
        return super.isPeer(eid, peer);
    }

    function _onReceiveHandles(
        uint32 srcEid,
        bytes32 srcApp,
        bytes calldata payload,
        bytes32[] calldata handles,
        bytes32 guid
    ) internal override {
        lastSrcEid = srcEid;
        lastSrcApp = srcApp;
        lastPayload = payload;
        lastGuid = guid;
        delete lastHandles;
        for (uint256 i = 0; i < handles.length; i++) {
            lastHandles.push(handles[i]);
        }
        receiveCount++;
    }
}
