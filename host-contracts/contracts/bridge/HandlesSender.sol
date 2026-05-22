// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {OAppSender, MessagingFee, MessagingReceipt} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/OAppSender.sol";
import {OptionsBuilder} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/libs/OptionsBuilder.sol";

import {ACL} from "../ACL.sol";
import {aclAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {BridgeEvents} from "./BridgeEvents.sol";

/**
 * @title HandlesSender
 * @notice Source-side mixin for confidential handle bridging. Implements the LayerZero V2
 *         OApp send path: for each handle in the caller's list it checks the source
 *         chain's ACL and emits a `BridgeHandle` event, then sends the payload and
 *         handle list to the destination chain via `_lzSend`.
 *
 * @dev    Abstract: the {ConfidentialBridge} concrete contract derives from this and
 *         from {HandlesReceiver}, and is the only deployed artifact. The OApp endpoint
 *         and ownership are initialized by the derived constructor — this contract
 *         intentionally provides none.
 *
 * @dev    The handle list is passed explicitly by the caller (not extracted from the
 *         payload) so the payload encoding stays fully under the source app's control.
 *         A protocol-level cap `MAX_HANDLES` bounds the per-message gas cost.
 *
 * @dev    Two execution-control modes are supported and are mutually exclusive:
 *         - `options` empty: the contract builds default options using its own
 *           `lzReceiveGas` formula (sized from handle count) and the caller-supplied
 *           `lzComposeGas`.
 *         - `options` non-empty: the caller supplies raw LayerZero options. In this
 *           case `lzComposeGas` must be zero (the caller has full control via options).
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
abstract contract HandlesSender is OAppSender, BridgeEvents {
    using OptionsBuilder for bytes;

    /// @notice Maximum number of handles per bridge call.
    uint256 public constant MAX_HANDLES = 32;

    /// @notice Base gas reserved for `lzReceive` on the destination, independent of
    ///         the handle count. Covers payload decoding, event emission overhead, and
    ///         the `sendCompose` call.
    uint128 public constant LZ_RECEIVE_BASE_GAS = 80_000;

    /// @notice Per-handle gas reserved for `lzReceive` on the destination. Covers
    ///         deriving the destination handle, emitting one `HandleBridged` event,
    ///         and appending to the in-memory `DstHandleList`.
    uint128 public constant LZ_RECEIVE_PER_HANDLE_GAS = 60_000;

    /// @notice Returned when the handle list exceeds the per-call cap.
    error TooManyHandles(uint256 length, uint256 maxAllowed);

    /// @notice Returned when the destination chain id is not registered for `dstEid`.
    error UnknownDstEid(uint32 dstEid);

    /// @notice Returned when the caller is not allowed to use a handle.
    error HandleNotAllowed(bytes32 handle, address srcApp);

    /// @notice Returned when caller-supplied options conflict with `lzComposeGas != 0`.
    /// @dev    Options carry per-message gas budgets; supplying both would be ambiguous.
    error ComposeGasMustBeZeroWithRawOptions();

    /// @notice ACL contract on this (source) chain.
    ACL private constant ACL_CONTRACT = ACL(aclAdd);

    /// @notice LayerZero endpoint id → destination chain id used in handle derivation.
    ///         Configured by the owner. A value of 0 means the endpoint id is not
    ///         registered and `send` will revert for it.
    mapping(uint32 dstEid => uint64 dstChainId) private _dstChainIdForEid;

    /// @notice OApp version tuple. HandlesSender is send-only: receiver side is `0`.
    /// @dev    Virtual so the combined {ConfidentialBridge} can return `(1, 2)`.
    function oAppVersion() public pure virtual override returns (uint64 senderVersion, uint64 receiverVersion) {
        return (1, 0);
    }

    /**
     * @notice Bridge `payload` and the handles it references to `dstEid`.
     *
     * @param dstEid         LayerZero endpoint id of the destination chain.
     * @param dstApp         Destination app address that should receive `payload` in its
     *                       `onReceive` callback.
     * @param payload        Opaque app-level payload; encoding is fully app-defined.
     * @param handleList     Source-chain handles referenced by `payload`. Order is
     *                       preserved on the destination, so apps can index into
     *                       `dstHandleList` by position.
     * @param lzComposeGas   Gas budget for the destination-side `lzCompose` (which runs
     *                       the destination app's `onReceive`). Must be 0 if `options`
     *                       is non-empty.
     * @param options        Raw LayerZero options; if empty the contract builds default
     *                       options from `LZ_RECEIVE_BASE_GAS + handleList.length *
     *                       LZ_RECEIVE_PER_HANDLE_GAS` and `lzComposeGas`.
     *
     * @return receipt LayerZero messaging receipt (includes the GUID used in events).
     *
     * @dev    Reverts if any handle is not ACL-allowed for `msg.sender` on this chain.
     *         Native fee is paid via `msg.value`; refund returns to `msg.sender`.
     */
    function send(
        uint32 dstEid,
        address dstApp,
        bytes calldata payload,
        bytes32[] calldata handleList,
        uint128 lzComposeGas,
        bytes calldata options
    ) external payable returns (MessagingReceipt memory receipt) {
        uint256 nHandles = handleList.length;
        if (nHandles > MAX_HANDLES) revert TooManyHandles(nHandles, MAX_HANDLES);

        uint64 dstChainId = _dstChainIdForEid[dstEid];
        if (dstChainId == 0) revert UnknownDstEid(dstEid);

        // Check ACL allowance for every handle up-front so we revert before paying the
        // LayerZero native fee on misconfigured calls.
        _checkAllAllowed(handleList);

        receipt = _dispatch(dstEid, dstApp, payload, handleList, lzComposeGas, options);

        // Emit BridgeHandle once the LayerZero-assigned GUID is finalized. The
        // coprocessor records one outstanding `SrcHandle → DstChainId` approval per
        // event and pins the associated source ciphertext (RFC 008 §Handle verification).
        _emitBridgeHandle(handleList, dstChainId, receipt.guid);
    }

    /// @dev Per-handle ACL check, extracted to keep `send`'s frame within the 16-slot
    ///      stack limit (without-via-ir builds otherwise trip stack-too-deep).
    function _checkAllAllowed(bytes32[] calldata handleList) private view {
        uint256 n = handleList.length;
        for (uint256 i = 0; i < n; i++) {
            if (!ACL_CONTRACT.isAllowed(handleList[i], msg.sender)) {
                revert HandleNotAllowed(handleList[i], msg.sender);
            }
        }
    }

    /// @dev LayerZero send dispatch, extracted so `send` does not carry `message`,
    ///      `finalOptions`, and `fee` into the event-emission phase. `msg.sender` is
    ///      preserved across internal calls and used as the source app in the encoded
    ///      message and as the native-fee refund address.
    function _dispatch(
        uint32 dstEid,
        address dstApp,
        bytes calldata payload,
        bytes32[] calldata handleList,
        uint128 lzComposeGas,
        bytes calldata options
    ) private returns (MessagingReceipt memory receipt) {
        bytes memory finalOptions = _resolveOptions(options, handleList.length, lzComposeGas);
        bytes memory message = abi.encode(msg.sender, dstApp, payload, handleList);
        MessagingFee memory fee = _quote(dstEid, message, finalOptions, false);
        receipt = _lzSend(dstEid, message, finalOptions, fee, payable(msg.sender));
    }

    /// @dev Event-emission loop, extracted from `send` for the same stack-pressure
    ///      reason as `_checkAllAllowed`.
    function _emitBridgeHandle(bytes32[] calldata handleList, uint64 dstChainId, bytes32 guid) private {
        uint256 n = handleList.length;
        for (uint256 i = 0; i < n; i++) {
            emit BridgeHandle(msg.sender, handleList[i], dstChainId, guid);
        }
    }

    /**
     * @notice Quote the native fee for a `send` call without sending.
     * @dev    Useful for callers wishing to compute msg.value before invoking `send`.
     */
    function quote(
        uint32 dstEid,
        address srcApp,
        address dstApp,
        bytes calldata payload,
        bytes32[] calldata handleList,
        uint128 lzComposeGas,
        bytes calldata options
    ) external view returns (MessagingFee memory fee) {
        if (_dstChainIdForEid[dstEid] == 0) revert UnknownDstEid(dstEid);

        bytes memory finalOptions = _resolveOptions(options, handleList.length, lzComposeGas);
        bytes memory message = abi.encode(srcApp, dstApp, payload, handleList);
        fee = _quote(dstEid, message, finalOptions, false);
    }

    /**
     * @notice Set the destination chain id corresponding to a `dstEid`.
     * @dev    Keeping this on the HandlesSender (instead of the coprocessor) keeps the
     *         coprocessor bridge-agnostic: it consumes `dstChainId` from emitted events
     *         and does not need to know about LayerZero endpoint ids.
     */
    function setDstChainId(uint32 dstEid, uint64 dstChainId) external onlyOwner {
        _dstChainIdForEid[dstEid] = dstChainId;
        emit DstChainIdSet(dstEid, dstChainId);
    }

    /// @notice Returns the destination chain id registered for `dstEid`, or 0 if unset.
    function getDstChainId(uint32 dstEid) external view returns (uint256) {
        return _dstChainIdForEid[dstEid];
    }

    function _resolveOptions(
        bytes calldata options,
        uint256 nHandles,
        uint128 lzComposeGas
    ) private pure returns (bytes memory) {
        if (options.length == 0) {
            uint128 lzReceiveGas = LZ_RECEIVE_BASE_GAS + uint128(nHandles) * LZ_RECEIVE_PER_HANDLE_GAS;
            bytes memory built = OptionsBuilder.newOptions().addExecutorLzReceiveOption(lzReceiveGas, 0);
            // Compose option only added when a compose gas budget is requested.
            // Compose index 0 because HandlesReceiver dispatches a single compose msg.
            if (lzComposeGas > 0) {
                built = built.addExecutorLzComposeOption(0, lzComposeGas, 0);
            }
            return built;
        } else {
            // Raw options mode: caller fully controls options; lzComposeGas would be
            // redundant and is required to be zero to avoid ambiguity.
            if (lzComposeGas != 0) revert ComposeGasMustBeZeroWithRawOptions();
            return options;
        }
    }
}
