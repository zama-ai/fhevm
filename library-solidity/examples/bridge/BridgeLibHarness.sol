// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, ebool, euint8, euint16, euint32, euint64, euint128, euint256, eaddress} from "../../lib/FHE.sol";
import {CoprocessorConfig} from "../../lib/Impl.sol";
import {MessagingFee, MessagingReceipt} from "../../lib/bridge/IConfidentialBridge.sol";

/**
 * @title   BridgeLibHarness
 * @notice  Minimal contract whose only job is to call the library's send-side functions
 *          (`FHE.bridge` / `FHE.quoteBridge`) so tests can drive every typed overload. It passes
 *          an arbitrary `bytes32` handle straight through (no encrypted math), letting a test
 *          confirm the wrapper looks the bridge up from the ACL and forwards every argument and
 *          `msg.value` unchanged — without needing a running coprocessor.
 * @dev     Test-only helper; not part of the published library.
 */
contract BridgeLibHarness {
    constructor(address acl) {
        // Only ACLAddress is read by the bridge path; the coprocessor/KMS addresses are
        // irrelevant here, so any non-zero placeholder is fine.
        FHE.setCoprocessor(
            CoprocessorConfig({ACLAddress: acl, CoprocessorAddress: address(1), KMSVerifierAddress: address(2)})
        );
    }

    // ---------------- single-handle, per type (multi-type coverage) ----------------

    function bridgeBool(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32 h,
        uint128 g
    ) external payable returns (MessagingReceipt memory) {
        return FHE.bridge(e, a, p, ebool.wrap(h), g, msg.value);
    }

    function bridge8(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32 h,
        uint128 g
    ) external payable returns (MessagingReceipt memory) {
        return FHE.bridge(e, a, p, euint8.wrap(h), g, msg.value);
    }

    function bridge16(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32 h,
        uint128 g
    ) external payable returns (MessagingReceipt memory) {
        return FHE.bridge(e, a, p, euint16.wrap(h), g, msg.value);
    }

    function bridge32(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32 h,
        uint128 g
    ) external payable returns (MessagingReceipt memory) {
        return FHE.bridge(e, a, p, euint32.wrap(h), g, msg.value);
    }

    function bridge64(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32 h,
        uint128 g
    ) external payable returns (MessagingReceipt memory) {
        return FHE.bridge(e, a, p, euint64.wrap(h), g, msg.value);
    }

    function bridge128(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32 h,
        uint128 g
    ) external payable returns (MessagingReceipt memory) {
        return FHE.bridge(e, a, p, euint128.wrap(h), g, msg.value);
    }

    function bridge256(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32 h,
        uint128 g
    ) external payable returns (MessagingReceipt memory) {
        return FHE.bridge(e, a, p, euint256.wrap(h), g, msg.value);
    }

    function bridgeAddr(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32 h,
        uint128 g
    ) external payable returns (MessagingReceipt memory) {
        return FHE.bridge(e, a, p, eaddress.wrap(h), g, msg.value);
    }

    // ---------------- multi-handle, typed arrays (multi-handle coverage) ----------------

    function bridgeArray64(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32[] calldata hs,
        uint128 g
    ) external payable returns (MessagingReceipt memory) {
        euint64[] memory typed = new euint64[](hs.length);
        for (uint256 i = 0; i < hs.length; i++) typed[i] = euint64.wrap(hs[i]);
        return FHE.bridge(e, a, p, typed, g, msg.value);
    }

    function bridgeArray256(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32[] calldata hs,
        uint128 g
    ) external payable returns (MessagingReceipt memory) {
        euint256[] memory typed = new euint256[](hs.length);
        for (uint256 i = 0; i < hs.length; i++) typed[i] = euint256.wrap(hs[i]);
        return FHE.bridge(e, a, p, typed, g, msg.value);
    }

    // ---------------- low-level escape hatches ----------------

    function bridgeList(
        uint32 e,
        bytes32 a,
        bytes calldata p,
        bytes32[] calldata hs,
        uint128 g,
        bytes calldata o
    ) external payable returns (MessagingReceipt memory) {
        return FHE.bridge(e, a, p, hs, g, o, msg.value);
    }

    // ---------------- quotes ----------------

    function quote64(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32 h,
        uint128 g
    ) external view returns (MessagingFee memory) {
        return FHE.quoteBridge(e, address(this), a, p, euint64.wrap(h), g);
    }

    function quote32(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32 h,
        uint128 g
    ) external view returns (MessagingFee memory) {
        return FHE.quoteBridge(e, address(this), a, p, euint32.wrap(h), g);
    }

    function quoteArray64(
        uint32 e,
        address a,
        bytes calldata p,
        bytes32[] calldata hs,
        uint128 g
    ) external view returns (MessagingFee memory) {
        euint64[] memory typed = new euint64[](hs.length);
        for (uint256 i = 0; i < hs.length; i++) typed[i] = euint64.wrap(hs[i]);
        return FHE.quoteBridge(e, address(this), a, p, typed, g);
    }

    function quoteList(
        uint32 e,
        bytes32 a,
        bytes calldata p,
        bytes32[] calldata hs,
        uint128 g,
        bytes calldata o
    ) external view returns (MessagingFee memory) {
        return FHE.quoteBridge(e, address(this), a, p, hs, g, o);
    }
}
