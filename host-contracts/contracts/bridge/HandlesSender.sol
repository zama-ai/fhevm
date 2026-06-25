// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {OAppSenderUpgradeable, MessagingFee, MessagingReceipt} from "@layerzerolabs/oapp-evm-upgradeable/contracts/oapp/OAppSenderUpgradeable.sol";
import {OptionsBuilder} from "@layerzerolabs/oapp-evm/contracts/oapp/libs/OptionsBuilder.sol";

import {ACL} from "../ACL.sol";
import {aclAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {ACLOwnable} from "../shared/ACLOwnable.sol";
import {BridgeEvents} from "./BridgeEvents.sol";

/**
 * @title HandlesSender
 * @notice Source-side mixin for confidential handle bridging. Implements the LayerZero V2
 *         OApp send path: for each handle in the caller's list it checks the source
 *         chain's ACL and emits a `BridgeHandle` event, then sends the payload and
 *         handle list to the destination chain via `_lzSend`.
 *
 * @dev    Abstract: the {ConfidentialBridge} concrete contract derives from this and
 *         from {HandlesReceiver}, and is the only deployed contract. The OApp endpoint
 *         and ownership are initialized by the derived initializer — this contract
 *         intentionally provides none.
 *
 * @dev    The handle list is passed explicitly by the caller (not extracted from the
 *         payload) so the payload encoding stays fully under the source app's control.
 *         A protocol-level cap `MAX_HANDLES` bounds the per-message gas cost.
 *
 * @dev    LayerZero execution options are built internally: the `lzReceive` gas
 *         is computed from the bridge's own formula (base + per-handle * handleCount +
 *         per-payload-byte * payloadLength, with optional per-`dstEid` governance
 *         overrides), and the `lzCompose` gas is the caller-supplied `lzComposeGas`.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
abstract contract HandlesSender is OAppSenderUpgradeable, ACLOwnable, BridgeEvents {
    using OptionsBuilder for bytes;

    /// @notice Maximum number of handles per bridge call.
    uint256 public constant MAX_HANDLES = 32;

    /// @notice Default base gas reserved for `lzReceive` on the destination, independent
    ///         of the handle count. Covers payload decoding, event emission overhead, and
    ///         the `sendCompose` call. Used for any `dstEid` without a custom override
    ///         (see {setLzReceiveBaseGas}).
    uint64 public constant LZ_RECEIVE_BASE_GAS_DEFAULT = 52_955;

    /// @notice Default per-handle gas reserved for `lzReceive` on the destination. Covers
    ///         deriving the destination handle, emitting one `HandleBridged` event, and
    ///         appending to the in-memory `DstHandleList`. Used for any `dstEid` without a
    ///         custom override (see {setLzReceivePerHandleGas}).
    uint64 public constant LZ_RECEIVE_PER_HANDLE_GAS_DEFAULT = 4_596;

    /// @notice Default per-payload-byte gas reserved for `lzReceive` on the destination.
    ///         Covers the payload-proportional work that is independent of handle count:
    ///         copying the payload on decode and re-encode, hashing it into the compose
    ///         queue, and the `ComposeSent` event emitted by `sendCompose`.
    ///         Used for any `dstEid` without a custom override
    ///         (see {setLzReceivePerPayloadByteGas}).
    uint64 public constant LZ_RECEIVE_PER_PAYLOAD_BYTE_DEFAULT = 19;

    /// @notice Returned when the handle list exceeds the per-call cap.
    error TooManyHandles(uint256 length, uint256 maxAllowed);

    /// @notice Returned when `send` is called with an empty handle list.
    error EmptyHandleList();

    /// @notice Returned when the destination chain id is not registered for `dstEid`.
    error UnknownDstEid(uint32 dstEid);

    /// @notice Returned when the caller is not allowed to use a handle.
    error HandleNotAllowed(bytes32 handle, address srcApp);

    /// @notice ACL contract on this (source) chain.
    ACL private constant ACL_CONTRACT = ACL(aclAdd);

    /// @notice Optional per-dstEid overrides for the `lzReceive` gas formula. A field value
    ///         of 0 means unset, in which case the matching `_DEFAULT` constant is used.
    /// @param baseGas            Custom base gas (overrides {LZ_RECEIVE_BASE_GAS_DEFAULT}).
    /// @param perHandleGas       Custom per-handle gas (overrides {LZ_RECEIVE_PER_HANDLE_GAS_DEFAULT}).
    /// @param perPayloadByteGas  Custom per-payload-byte gas (overrides
    ///                           {LZ_RECEIVE_PER_PAYLOAD_BYTE_DEFAULT}).
    struct CustomLzReceiveGas {
        uint64 baseGas;
        uint64 perHandleGas;
        uint64 perPayloadByteGas;
    }

    /// @custom:storage-location erc7201:fhevm.storage.HandlesSender
    struct HandlesSenderStorage {
        /// @dev LayerZero endpoint id → destination chain id used in handle derivation.
        ///      Configured by the ACL owner via {setDstChainId}. A value of 0 means the
        ///      endpoint id is not registered and `send` will revert for it.
        mapping(uint32 dstEid => uint64 dstChainId) dstChainIdForEid;
        /// @dev Optional per-dstEid override for the `lzReceive` gas formula. Set via
        ///      {setLzReceiveBaseGas} / {setLzReceivePerHandleGas} /
        ///      {setLzReceivePerPayloadByteGas}; unset fields fall back to the `_DEFAULT`
        ///      constants.
        mapping(uint32 dstEid => CustomLzReceiveGas) customLzReceiveGas;
    }

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.HandlesSender")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant HANDLES_SENDER_STORAGE_LOCATION =
        0x10e1ba6929f9b113e703e9abb104ab627cb3d8e7dfab4ac4ce63791f885d8900;

    function _getHandlesSenderStorage() private pure returns (HandlesSenderStorage storage $) {
        assembly {
            $.slot := HANDLES_SENDER_STORAGE_LOCATION
        }
    }

    /// @notice OApp version tuple. HandlesSender is send-only: receiver side is `0`.
    /// @dev    Virtual so the combined {ConfidentialBridge} can return `(1, 2)`.
    function oAppVersion() public pure virtual override returns (uint64 senderVersion, uint64 receiverVersion) {
        return (1, 0);
    }

    /**
     * @notice Bridge `payload` and the handles it references to `dstEid`.
     *
     * @param dstEid         LayerZero endpoint id of the destination chain.
     * @param dstApp         Destination app on the destination chain that should receive
     *                       `payload` in its `onConfidentialBridgeReceived` callback. Bytes32 (rather than
     *                       `address`) so non-EVM destinations (e.g. Solana, which uses
     *                       32-byte program IDs) can be addressed without a future
     *                       protocol change. EVM callers pass
     *                       `bytes32(uint256(uint160(dstAppAddress)))`.
     * @param payload        Opaque app-level payload; encoding is fully app-defined.
     * @param handleList     Source-chain handles referenced by `payload`. Order is
     *                       preserved on the destination, so apps can index into
     *                       `dstHandleList` by position.
     * @param lzComposeGas   Gas budget for the destination-side `lzCompose` (which runs
     *                       the destination app's `onConfidentialBridgeReceived`). This is
     *                       entirely app-specific, so the bridge does not enforce a floor.
     *                       Pass 0 to omit the executor compose option entirely: the compose
     *                       message is still queued on the destination (by `_lzReceive`), but
     *                       the executor will NOT auto-run `lzCompose` — it must be triggered
     *                       manually (permissionlessly) via the endpoint. Apps that require
     *                       the callback should enforce their own minimum.
     *
     * @return receipt LayerZero messaging receipt (includes the GUID used in events).
     *
     * @dev    The `lzReceive` execution gas is computed internally as
     *         `baseGas(dstEid) + handleList.length * perHandleGas(dstEid) + payload.length *
     *         perPayloadByteGas(dstEid)`, where each term is the `dstEid`'s custom override
     *         if set, else the corresponding
     *         {LZ_RECEIVE_BASE_GAS_DEFAULT}/{LZ_RECEIVE_PER_HANDLE_GAS_DEFAULT}/{LZ_RECEIVE_PER_PAYLOAD_BYTE_DEFAULT}
     *         constant.
     * @dev    Reverts with {EmptyHandleList} if `handleList` is empty and
     *         with {TooManyHandles} if it exceeds {MAX_HANDLES}.
     * @dev    Reverts if any handle is not ACL-allowed for `msg.sender` on this chain.
     *         Native fee is paid via `msg.value`; refund returns to `msg.sender`.
     */
    function send(
        uint32 dstEid,
        bytes32 dstApp,
        bytes calldata payload,
        bytes32[] calldata handleList,
        uint64 lzComposeGas
    ) external payable virtual returns (MessagingReceipt memory receipt) {
        uint256 nHandles = handleList.length;
        if (nHandles == 0) revert EmptyHandleList();
        if (nHandles > MAX_HANDLES) revert TooManyHandles(nHandles, MAX_HANDLES);

        uint64 dstChainId = _getHandlesSenderStorage().dstChainIdForEid[dstEid];
        if (dstChainId == 0) revert UnknownDstEid(dstEid);

        // Check ACL allowance for every handle up-front so we revert before paying the
        // LayerZero native fee on misconfigured calls.
        _checkAllAllowed(handleList);

        receipt = _dispatch(dstEid, dstApp, payload, handleList, lzComposeGas);

        // Emit BridgeHandle once the LayerZero-assigned GUID is finalized. The
        // coprocessor records one outstanding `SrcHandle → DstChainId` approval per
        // event and pins the associated source ciphertext (RFC 008 §Handle verification).
        _emitBridgeHandle(handleList, dstChainId, receipt.guid);
    }

    /// @dev Per-handle ACL check, extracted to keep `send`'s frame within the 16-slot
    ///      stack limit (without-via-ir builds otherwise trip stack-too-deep).
    function _checkAllAllowed(bytes32[] calldata handleList) internal view virtual {
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
    ///
    ///      The wire-format `srcApp` field is bytes32 (declared so on the receive side
    ///      too) for forward-compat with non-EVM source chains. For EVM sources the
    ///      `bytes32(uint256(uint160(msg.sender)))` cast produces byte-identical
    ///      output to encoding as `address`, so this is purely a type-level signal
    ///      preserving the upper 12 bytes for chains that need them.
    function _dispatch(
        uint32 dstEid,
        bytes32 dstApp,
        bytes calldata payload,
        bytes32[] calldata handleList,
        uint64 lzComposeGas
    ) internal virtual returns (MessagingReceipt memory receipt) {
        bytes memory finalOptions = _buildOptions(
            dstEid,
            uint64(handleList.length),
            uint64(payload.length),
            lzComposeGas
        );
        bytes32 srcApp = bytes32(uint256(uint160(msg.sender)));
        bytes memory message = abi.encode(srcApp, dstApp, payload, handleList);
        MessagingFee memory fee = _quote(dstEid, message, finalOptions, false);
        receipt = _lzSend(dstEid, message, finalOptions, fee, payable(msg.sender));
    }

    /// @dev Event-emission loop, extracted from `send` for the same stack-pressure
    ///      reason as `_checkAllAllowed`.
    function _emitBridgeHandle(bytes32[] calldata handleList, uint64 dstChainId, bytes32 guid) internal virtual {
        uint256 n = handleList.length;
        for (uint256 i = 0; i < n; i++) {
            emit BridgeHandle(msg.sender, handleList[i], dstChainId, guid);
        }
    }

    /**
     * @notice Quote the native fee for a `send` call without sending.
     * @dev    Useful for callers wishing to compute msg.value before invoking `send`.
     * @dev    Applies the same input validation as {send} — reverts with {EmptyHandleList},
     *         {TooManyHandles}, or {UnknownDstEid} under the same conditions — so a
     *         successful quote guarantees those `send` guards will pass.
     *         The ACL allowance check is intentionally NOT applied: this lets callers
     *         estimate `msg.value` before the transaction that grants ACL access to the
     *         handles being bridged.
     */
    /// @param srcApp        The source app paying the fee (kept as `address` for caller
    ///                       convenience — quote is an EVM-side view). Padded internally
    ///                       to match the bytes32 wire format used by `send`.
    /// @param dstApp        Destination app on the destination chain, as bytes32. See
    ///                       {send} for the encoding convention.
    function quote(
        uint32 dstEid,
        address srcApp,
        bytes32 dstApp,
        bytes calldata payload,
        bytes32[] calldata handleList,
        uint64 lzComposeGas
    ) external view virtual returns (MessagingFee memory fee) {
        uint256 nHandles = handleList.length;
        if (nHandles == 0) revert EmptyHandleList();
        if (nHandles > MAX_HANDLES) revert TooManyHandles(nHandles, MAX_HANDLES);

        if (_getHandlesSenderStorage().dstChainIdForEid[dstEid] == 0) revert UnknownDstEid(dstEid);

        bytes memory finalOptions = _buildOptions(dstEid, uint64(nHandles), uint64(payload.length), lzComposeGas);
        bytes memory message = abi.encode(bytes32(uint256(uint160(srcApp))), dstApp, payload, handleList);
        fee = _quote(dstEid, message, finalOptions, false);
    }

    /**
     * @notice Set the destination chain id corresponding to a `dstEid`.
     * @dev    Keeping this on the HandlesSender (instead of the coprocessor) keeps the
     *         coprocessor bridge-agnostic: it consumes `dstChainId` from emitted events
     *         and does not need to know about LayerZero endpoint ids.
     */
    function setDstChainId(uint32 dstEid, uint64 dstChainId) external virtual onlyACLOwner {
        _setDstChainId(dstEid, dstChainId);
    }

    function _setDstChainId(uint32 dstEid, uint64 dstChainId) internal virtual {
        _getHandlesSenderStorage().dstChainIdForEid[dstEid] = dstChainId;
        emit DstChainIdSet(dstEid, dstChainId);
    }

    /// @notice Returns the destination chain id registered for `dstEid`, or 0 if unset.
    function getDstChainId(uint32 dstEid) external view virtual returns (uint256) {
        return _getHandlesSenderStorage().dstChainIdForEid[dstEid];
    }

    /**
     * @notice Set a custom base `lzReceive` gas for `dstEid`, overriding
     *         {LZ_RECEIVE_BASE_GAS_DEFAULT}.
     * @dev    Pass 0 to clear the override and fall back to the default constant.
     */
    function setLzReceiveBaseGas(uint32 dstEid, uint64 lzReceiveBaseGas) external virtual onlyACLOwner {
        _getHandlesSenderStorage().customLzReceiveGas[dstEid].baseGas = lzReceiveBaseGas;
        emit LzReceiveBaseGasSet(dstEid, lzReceiveBaseGas);
    }

    /**
     * @notice Set a custom per-handle `lzReceive` gas for `dstEid`, overriding
     *         {LZ_RECEIVE_PER_HANDLE_GAS_DEFAULT}.
     * @dev    Pass 0 to clear the override and fall back to the default constant.
     */
    function setLzReceivePerHandleGas(uint32 dstEid, uint64 lzReceivePerHandleGas) external virtual onlyACLOwner {
        _getHandlesSenderStorage().customLzReceiveGas[dstEid].perHandleGas = lzReceivePerHandleGas;
        emit LzReceivePerHandleGasSet(dstEid, lzReceivePerHandleGas);
    }

    /**
     * @notice Set a custom per-payload-byte `lzReceive` gas for `dstEid`, overriding
     *         {LZ_RECEIVE_PER_PAYLOAD_BYTE_DEFAULT}.
     * @dev    Pass 0 to clear the override and fall back to the default constant.
     */
    function setLzReceivePerPayloadByteGas(
        uint32 dstEid,
        uint64 lzReceivePerPayloadByteGas
    ) external virtual onlyACLOwner {
        _getHandlesSenderStorage().customLzReceiveGas[dstEid].perPayloadByteGas = lzReceivePerPayloadByteGas;
        emit LzReceivePerPayloadByteGasSet(dstEid, lzReceivePerPayloadByteGas);
    }

    /// @notice Returns the effective base `lzReceive` gas for `dstEid`: the custom override
    ///         if one is set, otherwise {LZ_RECEIVE_BASE_GAS_DEFAULT}.
    function getLzReceiveBaseGas(uint32 dstEid) external view virtual returns (uint64) {
        return _effectiveLzReceiveBaseGas(dstEid);
    }

    /// @notice Returns the effective per-handle `lzReceive` gas for `dstEid`: the custom
    ///         override if one is set, otherwise {LZ_RECEIVE_PER_HANDLE_GAS_DEFAULT}.
    function getLzReceivePerHandleGas(uint32 dstEid) external view virtual returns (uint64) {
        return _effectiveLzReceivePerHandleGas(dstEid);
    }

    /// @notice Returns the effective per-payload-byte `lzReceive` gas for `dstEid`: the
    ///         custom override if one is set, otherwise {LZ_RECEIVE_PER_PAYLOAD_BYTE_DEFAULT}.
    function getLzReceivePerPayloadByteGas(uint32 dstEid) external view virtual returns (uint64) {
        return _effectiveLzReceivePerPayloadByteGas(dstEid);
    }

    /// @dev Base gas used for `dstEid`: custom override when non-zero, else the default.
    function _effectiveLzReceiveBaseGas(uint32 dstEid) internal view virtual returns (uint64) {
        uint64 custom = _getHandlesSenderStorage().customLzReceiveGas[dstEid].baseGas;
        return custom == 0 ? LZ_RECEIVE_BASE_GAS_DEFAULT : custom;
    }

    /// @dev Per-handle gas used for `dstEid`: custom override when non-zero, else default.
    function _effectiveLzReceivePerHandleGas(uint32 dstEid) internal view virtual returns (uint64) {
        uint64 custom = _getHandlesSenderStorage().customLzReceiveGas[dstEid].perHandleGas;
        return custom == 0 ? LZ_RECEIVE_PER_HANDLE_GAS_DEFAULT : custom;
    }

    /// @dev Per-payload-byte gas used for `dstEid`: custom override when non-zero, else default.
    function _effectiveLzReceivePerPayloadByteGas(uint32 dstEid) internal view virtual returns (uint64) {
        uint64 custom = _getHandlesSenderStorage().customLzReceiveGas[dstEid].perPayloadByteGas;
        return custom == 0 ? LZ_RECEIVE_PER_PAYLOAD_BYTE_DEFAULT : custom;
    }

    /// @dev Builds the LayerZero execution options for a send. The `lzReceive` gas is the
    ///      bridge formula (`baseGas + nHandles * perHandleGas + payloadLen * perPayloadByteGas`,
    ///      with per-`dstEid` governance overrides); the `lzCompose` gas is the
    ///      caller-supplied budget. A zero `lzComposeGas` omits the compose option entirely,
    ///      so the executor will not auto-run `lzCompose` (the compose stays queued for
    ///      manual execution — see {send}).
    function _buildOptions(
        uint32 dstEid,
        uint64 nHandles,
        uint64 payloadLen,
        uint64 lzComposeGas
    ) internal view virtual returns (bytes memory) {
        uint64 lzReceiveGas = _effectiveLzReceiveBaseGas(dstEid) +
            nHandles *
            _effectiveLzReceivePerHandleGas(dstEid) +
            payloadLen *
            _effectiveLzReceivePerPayloadByteGas(dstEid);
        bytes memory built = OptionsBuilder.newOptions().addExecutorLzReceiveOption(lzReceiveGas, 0);
        // Compose index 0 because HandlesReceiver dispatches a single compose msg. Omitted
        // when the caller passes 0 (compose gas is app-specific; see {send}).
        if (lzComposeGas > 0) {
            built = built.addExecutorLzComposeOption(0, lzComposeGas, 0);
        }
        return built;
    }
}
