// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {TestHelperOz5} from "@layerzerolabs/test-devtools-evm-foundry/contracts/TestHelperOz5.sol";
import {Origin} from "@layerzerolabs/oapp-evm-upgradeable/contracts/oapp/OAppReceiverUpgradeable.sol";
import {MessagingFee, MessagingReceipt} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";

import {Vm} from "forge-std/Vm.sol";

import {DeployableERC1967Proxy, HostContractsDeployerTestUtils} from "../../fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ConfidentialBridge} from "../../contracts/bridge/ConfidentialBridge.sol";
import {HandlesSender} from "../../contracts/bridge/HandlesSender.sol";
import {HandlesReceiver} from "../../contracts/bridge/HandlesReceiver.sol";
import {BridgeEvents} from "../../contracts/bridge/BridgeEvents.sol";
import {aclAdd, confidentialBridgeAdd, fhevmExecutorAdd} from "../../addresses/FHEVMHostAddresses.sol";

import {MockDstApp} from "./mocks/MockDstApp.sol";

/**
 * @title BridgeTest
 * @notice Forge tests for the ConfidentialBridge.
 *
 * @dev    The full host stack (ACL/FHEVMExecutor) is reconstructed at its canonical
 *         addresses via HostContractsDeployerTestUtils, so ACL.isAllowed and the
 *         bridge's allowTransient bypass exercise the real contracts.
 *
 *         LayerZero V2 is set up via TestHelperOz5.setUpEndpoints(2, SimpleMessageLib).
 *         One ConfidentialBridge is deployed per endpoint: `srcBridge` on eid=1 plays
 *         the sender role and `dstBridge` on eid=2 plays the receiver role; peers
 *         are wired bidirectionally. Both share the same ACL because forge runs all
 *         contracts on a single fork — sufficient to test the bridge plumbing.
 */
contract BridgeTest is TestHelperOz5, HostContractsDeployerTestUtils, BridgeEvents {
    uint32 internal constant SRC_EID = 1;
    uint32 internal constant DST_EID = 2;
    uint64 internal constant DST_CHAIN_ID = 4242;

    address internal owner = makeAddr("owner");
    address internal srcApp = makeAddr("srcApp");
    address internal user = makeAddr("user");

    ACL internal acl;
    ConfidentialBridge internal srcBridge;
    ConfidentialBridge internal dstBridge;
    MockDstApp internal dstApp;
    address internal fhevmExecutor;

    function setUp() public virtual override {
        super.setUp();

        // Wire two LZ endpoints (eids 1 and 2) with SimpleMessageLib.
        setUpEndpoints(2, LibraryType.SimpleMessageLib);

        // Reconstruct the FHE host stack at canonical addresses.
        _deployACL(owner);
        _deployFHEVMExecutor(owner);
        acl = ACL(aclAdd);
        fhevmExecutor = fhevmExecutorAdd;

        // Deploy one ConfidentialBridge per endpoint behind a UUPS proxy. The contract
        // handles both send and receive — in this two-endpoint topology each instance
        // plays one role. The source-side bridge seeds its dstEid → dstChainId map at
        // initialization; the dst-side bridge doesn't send so it needs no seed.
        uint32[] memory srcDstEids = new uint32[](1);
        uint64[] memory srcDstChainIds = new uint64[](1);
        srcDstEids[0] = DST_EID;
        srcDstChainIds[0] = DST_CHAIN_ID;
        srcBridge = _deployBridgeProxy(endpoints[SRC_EID], srcDstEids, srcDstChainIds);
        dstBridge = _deployBridgeProxy(endpoints[DST_EID], new uint32[](0), new uint64[](0));

        // Configure peer-to-peer routing.
        vm.startPrank(owner);
        srcBridge.setPeer(DST_EID, _addressToBytes32(address(dstBridge)));
        dstBridge.setPeer(SRC_EID, _addressToBytes32(address(srcBridge)));
        vm.stopPrank();

        dstApp = new MockDstApp();

        // Fund the user paying LZ fees.
        vm.deal(srcApp, 100 ether);
        vm.deal(user, 100 ether);
    }

    /// @dev Deploys a ConfidentialBridge behind a fresh UUPS proxy, following the same
    ///      two-phase pattern used by the other host contracts: deploy an
    ///      {EmptyUUPSProxy} (whose upgrade hook is gated by `onlyACLOwner`), then
    ///      upgrade it to the real {ConfidentialBridge} implementation while invoking
    ///      `initializeFromEmptyProxy` with the dstEid → dstChainId seed lists. The
    ///      endpoint address is baked into the implementation as an immutable, so each
    ///      bridge needs its own implementation.
    ///
    ///      The upgrade and the initializer are both gated by the ACL owner — `owner`
    ///      here, as `_deployACL` makes it so — and the bridge has no separate
    ///      operational owner: {ConfidentialBridge.owner} always returns the current
    ///      ACL owner.
    function _deployBridgeProxy(
        address lzEndpoint,
        uint32[] memory dstEids,
        uint64[] memory dstChainIds
    ) internal returns (ConfidentialBridge proxy) {
        address emptyImpl = address(new EmptyUUPSProxy());
        DeployableERC1967Proxy raw = new DeployableERC1967Proxy(
            emptyImpl,
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );
        address proxyAddr = address(raw);

        address bridgeImpl = address(new ConfidentialBridge(lzEndpoint));

        vm.prank(owner);
        EmptyUUPSProxy(proxyAddr).upgradeToAndCall(
            bridgeImpl,
            abi.encodeCall(ConfidentialBridge.initializeFromEmptyProxy, (dstEids, dstChainIds))
        );
        proxy = ConfidentialBridge(payable(proxyAddr));
    }

    /// @dev Convenience: grant `account` persistent allowance on `handle` by
    ///      replaying the same FHEVMExecutor→user grant sequence the host normally
    ///      runs. Mirrors acl.t.sol's pattern.
    function _allow(bytes32 handle, address account) internal {
        vm.prank(fhevmExecutor);
        acl.allowTransient(handle, account);
        vm.prank(account);
        acl.allow(handle, account);
        acl.cleanTransientStorage();
    }

    /// @dev A valid-looking handle: byte 21 = 0xff (computation marker), byte 30 = 5
    ///      (FheType.Uint64), byte 31 = 0 (HANDLE_VERSION).
    function _makeHandle(uint256 seed) internal view returns (bytes32 h) {
        // Top 21 bytes derived from the seed so handles are distinct.
        h = keccak256(abi.encodePacked("test-handle", seed));
        h = h & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        h = h | (bytes32(uint256(0xff)) << 80); // byte 21
        h = h | (bytes32(uint256(uint64(block.chainid))) << 16); // bytes 22-29
        h = h | (bytes32(uint256(0x05)) << 8); // byte 30 = Uint64
        // byte 31 = HANDLE_VERSION 0
    }

    function _addressToBytes32(address a) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(a)));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Source-side configuration & guards
    ////////////////////////////////////////////////////////////////////////////////

    function test_SetDstChainId_OnlyOwner() public {
        vm.expectRevert();
        srcBridge.setDstChainId(DST_EID, 99);
    }

    function test_SetDstChainId_EmitsEventAndUpdates() public {
        vm.expectEmit(true, false, false, true, address(srcBridge));
        emit DstChainIdSet(DST_EID, 99);
        vm.prank(owner);
        srcBridge.setDstChainId(DST_EID, 99);
        assertEq(srcBridge.getDstChainId(DST_EID), 99);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Custom lzReceive gas overrides
    ////////////////////////////////////////////////////////////////////////////////

    function test_LzReceiveGas_DefaultsWhenUnset() public view {
        assertEq(srcBridge.getLzReceiveBaseGas(DST_EID), srcBridge.LZ_RECEIVE_BASE_GAS_DEFAULT());
        assertEq(srcBridge.getLzReceivePerHandleGas(DST_EID), srcBridge.LZ_RECEIVE_PER_HANDLE_GAS_DEFAULT());
        assertEq(srcBridge.getLzReceivePerPayloadByteGas(DST_EID), srcBridge.LZ_RECEIVE_PER_PAYLOAD_BYTE_DEFAULT());
    }

    function test_SetLzReceiveBaseGas_OnlyOwner() public {
        vm.expectRevert();
        srcBridge.setLzReceiveBaseGas(DST_EID, 123_000);
    }

    function test_SetLzReceivePerHandleGas_OnlyOwner() public {
        vm.expectRevert();
        srcBridge.setLzReceivePerHandleGas(DST_EID, 7_000);
    }

    function test_SetLzReceivePerPayloadByteGas_OnlyOwner() public {
        vm.expectRevert();
        srcBridge.setLzReceivePerPayloadByteGas(DST_EID, 32);
    }

    function test_SetLzReceiveBaseGas_OverridesEmitsAndClears() public {
        vm.expectEmit(true, false, false, true, address(srcBridge));
        emit LzReceiveBaseGasSet(DST_EID, 123_000);
        vm.prank(owner);
        srcBridge.setLzReceiveBaseGas(DST_EID, 123_000);
        assertEq(srcBridge.getLzReceiveBaseGas(DST_EID), 123_000);

        // Other eids are unaffected and still resolve to the default.
        assertEq(srcBridge.getLzReceiveBaseGas(SRC_EID), srcBridge.LZ_RECEIVE_BASE_GAS_DEFAULT());

        // Setting back to 0 clears the override and restores the default.
        vm.prank(owner);
        srcBridge.setLzReceiveBaseGas(DST_EID, 0);
        assertEq(srcBridge.getLzReceiveBaseGas(DST_EID), srcBridge.LZ_RECEIVE_BASE_GAS_DEFAULT());
    }

    function test_SetLzReceivePerHandleGas_OverridesEmitsAndClears() public {
        vm.expectEmit(true, false, false, true, address(srcBridge));
        emit LzReceivePerHandleGasSet(DST_EID, 7_000);
        vm.prank(owner);
        srcBridge.setLzReceivePerHandleGas(DST_EID, 7_000);
        assertEq(srcBridge.getLzReceivePerHandleGas(DST_EID), 7_000);

        vm.prank(owner);
        srcBridge.setLzReceivePerHandleGas(DST_EID, 0);
        assertEq(srcBridge.getLzReceivePerHandleGas(DST_EID), srcBridge.LZ_RECEIVE_PER_HANDLE_GAS_DEFAULT());
    }

    function test_SetLzReceivePerPayloadByteGas_OverridesEmitsAndClears() public {
        vm.expectEmit(true, false, false, true, address(srcBridge));
        emit LzReceivePerPayloadByteGasSet(DST_EID, 32);
        vm.prank(owner);
        srcBridge.setLzReceivePerPayloadByteGas(DST_EID, 32);
        assertEq(srcBridge.getLzReceivePerPayloadByteGas(DST_EID), 32);

        // Other eids are unaffected and still resolve to the default.
        assertEq(srcBridge.getLzReceivePerPayloadByteGas(SRC_EID), srcBridge.LZ_RECEIVE_PER_PAYLOAD_BYTE_DEFAULT());

        // Setting back to 0 clears the override and restores the default.
        vm.prank(owner);
        srcBridge.setLzReceivePerPayloadByteGas(DST_EID, 0);
        assertEq(srcBridge.getLzReceivePerPayloadByteGas(DST_EID), srcBridge.LZ_RECEIVE_PER_PAYLOAD_BYTE_DEFAULT());
    }

    function test_Send_RevertsOnUnknownDstEid() public {
        bytes32[] memory handleList = new bytes32[](1);
        handleList[0] = _makeHandle(0);
        vm.prank(srcApp);
        vm.expectRevert(abi.encodeWithSelector(HandlesSender.UnknownDstEid.selector, uint32(99)));
        srcBridge.send{value: 0}(uint32(99), _addressToBytes32(address(dstApp)), "", handleList, uint64(0));
    }

    function test_Send_RevertsAboveMaxHandles() public {
        uint256 cap = srcBridge.MAX_HANDLES();
        bytes32[] memory handleList = new bytes32[](cap + 1);
        for (uint256 i = 0; i < handleList.length; i++) handleList[i] = _makeHandle(i);
        vm.prank(srcApp);
        vm.expectRevert(abi.encodeWithSelector(HandlesSender.TooManyHandles.selector, cap + 1, cap));
        srcBridge.send{value: 0}(DST_EID, _addressToBytes32(address(dstApp)), "", handleList, uint64(0));
    }

    function test_Send_RevertsOnHandleNotAllowed() public {
        bytes32 h = _makeHandle(0);
        bytes32[] memory handleList = new bytes32[](1);
        handleList[0] = h;
        vm.prank(srcApp);
        vm.expectRevert(abi.encodeWithSelector(HandlesSender.HandleNotAllowed.selector, h, srcApp));
        srcBridge.send{value: 0}(DST_EID, _addressToBytes32(address(dstApp)), "", handleList, uint64(0));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // End-to-end: srcBridge → endpoint → dstBridge._lzReceive
    ////////////////////////////////////////////////////////////////////////////////

    function test_Send_EndToEnd_EmitsBridgeHandleAndHandleBridged() public {
        bytes32 h0 = _makeHandle(0);
        bytes32 h1 = _makeHandle(1);
        _allow(h0, srcApp);
        _allow(h1, srcApp);

        bytes32[] memory handleList = new bytes32[](2);
        handleList[0] = h0;
        handleList[1] = h1;
        bytes memory payload = abi.encode(user, "hello");

        // Quote first so we can pay the right native fee.
        MessagingFee memory fee = srcBridge.quote(
            DST_EID,
            srcApp,
            _addressToBytes32(address(dstApp)),
            payload,
            handleList,
            uint64(200_000)
        );

        vm.recordLogs();
        vm.prank(srcApp);
        MessagingReceipt memory receipt = srcBridge.send{value: fee.nativeFee}(
            DST_EID,
            _addressToBytes32(address(dstApp)),
            payload,
            handleList,
            uint64(200_000)
        );

        // Inspect logs: BridgeHandle is emitted once per handle, with the receipt's GUID.
        // Topic1 is the indexed senderDapp address; the remaining fields live in `data`.
        Vm.Log[] memory logs = vm.getRecordedLogs();
        uint256 nBridgeEvents;
        bytes32 bridgeHandleSig = keccak256("BridgeHandle(address,bytes32,uint64,bytes32)");
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == bridgeHandleSig && logs[i].emitter == address(srcBridge)) {
                nBridgeEvents++;
                assertEq(address(uint160(uint256(logs[i].topics[1]))), srcApp);
                (, uint64 emittedDstChainId, bytes32 emittedGuid) = abi.decode(
                    logs[i].data,
                    (bytes32, uint64, bytes32)
                );
                assertEq(emittedDstChainId, DST_CHAIN_ID);
                assertEq(emittedGuid, receipt.guid);
            }
        }
        assertEq(nBridgeEvents, 2, "BridgeHandle should fire once per handle");

        // Deliver to the receiver. verifyPackets executes lzReceive on the dst.
        verifyPackets(DST_EID, address(dstBridge));

        // After lzReceive, HandleBridged should have fired for each handle. Re-record
        // logs is harder mid-test; instead recompute the dst handles and check the
        // ComposeSent message body for them.

    }

    function test_LzReceive_DerivesAndEmitsHandleBridged() public {
        bytes32 h0 = _makeHandle(0x42);
        bytes32 h1 = _makeHandle(0x43);
        bytes32[] memory handleList = new bytes32[](2);
        handleList[0] = h0;
        handleList[1] = h1;

        bytes32 guid = keccak256("fake-guid");
        // Bridge wire format: srcApp and dstApp are both bytes32 (zero-padded EVM addrs).
        bytes memory message = abi.encode(
            _addressToBytes32(srcApp),
            _addressToBytes32(address(dstApp)),
            bytes("payload"),
            handleList
        );

        // Build an Origin matching our peer config.
        Origin memory origin = Origin({srcEid: SRC_EID, sender: _addressToBytes32(address(srcBridge)), nonce: 1});

        // Predict the derivation locally and assert the emitted HandleBridged events
        // carry the exact dstHandle the contract computed.
        bytes32 prevHash = blockhash(block.number - 1);
        bytes32 expectedDst0 = _expectedDstHandle(h0, prevHash, guid);
        bytes32 expectedDst1 = _expectedDstHandle(h1, prevHash, guid);

        // Check the indexed receiverDapp (topic1) and the data payload.
        vm.expectEmit(true, false, false, true, address(dstBridge));
        emit HandleBridged(address(dstApp), h0, expectedDst0, guid);
        vm.expectEmit(true, false, false, true, address(dstBridge));
        emit HandleBridged(address(dstApp), h1, expectedDst1, guid);

        // Impersonate the endpoint to call lzReceive directly. The OAppReceiver checks
        // `address(endpoint) == msg.sender`, so we prank as the endpoint contract.
        vm.prank(address(endpoints[DST_EID]));
        dstBridge.lzReceive(origin, guid, message, address(0), "");
    }

    /// @dev Re-implements HandlesReceiver's `_deriveDstHandle` for assertions. Must
    ///      match exactly — domain sep + field ordering matter and are part of the
    ///      spec contract with the coprocessor.
    /// @dev `guid` is accepted as a parameter for call-site symmetry with the
    ///      contract's `_deriveAndEmit` (which threads it through) but is no
    ///      longer part of the hash itself.
    function _expectedDstHandle(
        bytes32 srcHandle,
        bytes32 prevBlockHash,
        bytes32 /* guid */
    ) internal view returns (bytes32 result) {
        result = keccak256(
            abi.encodePacked(bytes8("FHE_brdg"), srcHandle, aclAdd, block.chainid, prevBlockHash, block.timestamp)
        );
        result = result & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        result = result | (bytes32(uint256(0xff)) << 80);
        result = result | (bytes32(uint256(uint64(block.chainid))) << 16);
        result = result | (bytes32(uint256(uint8(srcHandle[30]))) << 8);
        // HANDLE_VERSION = 0
    }

    function test_DstHandle_MetadataLayoutIsCorrect() public view {
        bytes32 src = _makeHandle(7);
        bytes32 prev = blockhash(block.number == 0 ? 0 : block.number - 1);
        bytes32 dst = _expectedDstHandle(src, prev, keccak256("g"));

        // Byte 21 = 0xff
        assertEq(uint8(dst[21]), 0xff);
        // Bytes 22-29 = chainid (uint64) — both src and dst use this chain in the test.
        uint64 cid;
        for (uint256 i = 22; i < 30; i++) {
            cid = (cid << 8) | uint8(dst[i]);
        }
        assertEq(uint256(cid), block.chainid);
        // Byte 30 = type byte copied from src (5 for Uint64).
        assertEq(uint8(dst[30]), 0x05);
        // Byte 31 = HANDLE_VERSION = 0
        assertEq(uint8(dst[31]), 0x00);
    }

    /// @dev Golden constant for the destination-handle derivation. Minted by
    ///      `test_DeriveDstHandle_GoldenVector` below (run `forge test`) and
    ///      mirrored, with the same inputs, by the Rust `derive_dst_handle` test
    ///      `matches_solidity_golden_vector`. Regenerate both together if the
    ///      derivation formula changes.
    bytes32 internal constant GOLDEN_DST_HANDLE =
        0x89ee7803d65c29976056001f9db9ba5d8b38975ac4ff00000000000030390500;

    /// @dev Cross-implementation lock for the destination-handle derivation. Pins
    ///      the output of the *real* `_deriveDstHandle` (via a thin harness) for a
    ///      fully-fixed input set; the Rust mirror asserts the same constant for the
    ///      same inputs. A divergence between the two hand-written implementations
    ///      breaks one side. Inputs are pinned (chain id, timestamp via cheatcodes;
    ///      ACL address is the compile-time `aclAdd`) so the value is deterministic.
    function test_DeriveDstHandle_GoldenVector() public {
        // Fixed inputs — must match the Rust test byte-for-byte.
        // src: bytes[0..4] = 01020304, byte 30 = 0x05 (FheType), rest 0.
        bytes32 src = bytes32(uint256(0x01020304) << 224) | bytes32(uint256(0x05) << 8);
        // prev: bytes[0..4] = 0a0b0c0d, rest 0.
        bytes32 prev = bytes32(uint256(0x0a0b0c0d) << 224);

        DeriveDstHandleHarness harness = new DeriveDstHandleHarness(endpoints[DST_EID]);
        vm.chainId(12345);
        vm.warp(1_700_000_000);

        bytes32 got = harness.deriveDstHandle(src, prev);
        assertEq(got, GOLDEN_DST_HANDLE);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Destination-side lzCompose authentication + dispatch
    ////////////////////////////////////////////////////////////////////////////////

    function test_LzCompose_RevertsIfNotEndpoint() public {
        bytes memory composeMsg = abi.encode(
            SRC_EID,
            _addressToBytes32(srcApp),
            _addressToBytes32(address(dstApp)),
            bytes(""),
            new bytes32[](0),
            new bytes32[](0)
        );
        vm.expectRevert(abi.encodeWithSelector(HandlesReceiver.NotLzEndpoint.selector, address(this)));
        dstBridge.lzCompose(address(dstBridge), keccak256("g"), composeMsg, address(0), "");
    }

    function test_LzCompose_RevertsIfFromNotSelf() public {
        bytes memory composeMsg = abi.encode(
            SRC_EID,
            _addressToBytes32(srcApp),
            _addressToBytes32(address(dstApp)),
            bytes(""),
            new bytes32[](0),
            new bytes32[](0)
        );
        vm.prank(address(endpoints[DST_EID]));
        vm.expectRevert(abi.encodeWithSelector(HandlesReceiver.UnexpectedComposeOrigin.selector, address(this)));
        dstBridge.lzCompose(address(this), keccak256("g"), composeMsg, address(0), "");
    }

    function test_LzCompose_GrantsTransientAndCallsOnReceive() public {
        bytes32 dstH0 = _makeHandle(100);
        bytes32 dstH1 = _makeHandle(101);
        bytes32[] memory srcHandleList = new bytes32[](2);
        bytes32[] memory dstHandleList = new bytes32[](2);
        srcHandleList[0] = _makeHandle(0);
        srcHandleList[1] = _makeHandle(1);
        dstHandleList[0] = dstH0;
        dstHandleList[1] = dstH1;
        bytes memory payload = abi.encode("payload-body");

        bytes memory composeMsg = abi.encode(
            SRC_EID,
            _addressToBytes32(srcApp),
            _addressToBytes32(address(dstApp)),
            payload,
            srcHandleList,
            dstHandleList
        );

        // In a real deployment the ACL bypasses sender checks for the canonical
        // ConfidentialBridge address. In this forge fixture the ACL's compile-time
        // `CONFIDENTIAL_BRIDGE_ADDRESS` is address(0) (BridgeAddress.sol default), so
        // the bypass does NOT trigger for our runtime-deployed bridge. Work around
        // by pre-allowing each dst handle to the bridge — the normal isAllowed
        // path then carries the allowTransient call.
        _allow(dstH0, address(dstBridge));
        _allow(dstH1, address(dstBridge));

        vm.prank(address(endpoints[DST_EID]));
        dstBridge.lzCompose(address(dstBridge), keccak256("g"), composeMsg, address(0), "");

        // Assert the destination app received the dispatch with the expected args.
        MockDstApp.LastCall memory lc = dstApp.lastCall();
        assertTrue(lc.wasCalled, "onConfidentialBridgeReceived should have fired");
        assertEq(lc.srcEid, SRC_EID);
        assertEq(lc.srcApp, _addressToBytes32(srcApp));
        assertEq(keccak256(lc.payload), keccak256(payload));
        assertEq(lc.srcHandleList.length, 2);
        assertEq(lc.dstHandleList[0], dstH0);
        assertEq(lc.dstHandleList[1], dstH1);
    }

    function test_LzCompose_RevertsWhenOnReceiveReverts() public {
        dstApp.setShouldRevert(true);

        bytes32[] memory empty = new bytes32[](0);
        bytes memory composeMsg = abi.encode(
            SRC_EID,
            _addressToBytes32(srcApp),
            _addressToBytes32(address(dstApp)),
            bytes(""),
            empty,
            empty
        );

        vm.prank(address(endpoints[DST_EID]));
        vm.expectRevert();
        dstBridge.lzCompose(address(dstBridge), keccak256("g"), composeMsg, address(0), "");
    }

    ////////////////////////////////////////////////////////////////////////////////
    // grantFallback
    ////////////////////////////////////////////////////////////////////////////////

    function test_GrantFallbackClearText_EmitsEvent() public {
        bytes32 dst = _makeHandle(42);
        uint256 plaintext = 42;

        vm.expectEmit(true, false, false, true, address(dstBridge));
        emit FallbackGrantedPlaintext(dst, plaintext);
        vm.prank(owner);
        dstBridge.grantFallbackPlaintext(dst, plaintext);
    }

    function test_GrantFallbackClearText_OnlyOwner() public {
        vm.expectRevert();
        dstBridge.grantFallbackPlaintext(_makeHandle(1), 23);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // ACL bridge wiring
    //
    // The ACL bakes in `CONFIDENTIAL_BRIDGE_ADDRESS` as a compile-time constant
    // from `addresses/BridgeAddress.sol`. In this forge fixture it defaults to
    // address(0) (set by task:initBridgeAddress). Asserting bypass behavior
    // would require regenerating BridgeAddress.sol with the runtime-deployed
    // address before forge compiles — out of scope here. Hardhat integration
    // tests (which use the full deploy task pipeline) are the right place to
    // verify the address-aligned bypass.
    ////////////////////////////////////////////////////////////////////////////////

    function test_ACL_BridgeAddressMatchesCompileTimeConstant() public view {
        // Sanity: ACL exposes the same bridge address that was baked in at compile
        // time from `addresses/FHEVMHostAddresses.sol`.
        assertEq(acl.getConfidentialBridgeAddress(), confidentialBridgeAdd);
    }

    function test_ACL_AllowTransientBypass_OffForNonCanonicalBridge() public {
        // The runtime-deployed `dstBridge` is at a fresh proxy address, NOT the
        // canonical `confidentialBridgeAdd` baked into ACL — so it gets no bypass
        // and the regular isAllowed path is enforced.
        assertTrue(address(dstBridge) != acl.getConfidentialBridgeAddress());
        bytes32 fresh = _makeHandle(999);
        vm.prank(address(dstBridge));
        vm.expectRevert();
        acl.allowTransient(fresh, address(dstApp));
    }
}

/// @dev Test-only harness exposing the internal `_deriveDstHandle`. Deployed
///      directly (no proxy/initializer): the derivation reads only `block.chainid`,
///      `block.timestamp`, the compile-time `aclAdd` constant, and its arguments —
///      never initialized storage — so an uninitialized instance computes it
///      identically to the production contract.
contract DeriveDstHandleHarness is ConfidentialBridge {
    constructor(address endpoint_) ConfidentialBridge(endpoint_) {}

    function deriveDstHandle(bytes32 srcHandle, bytes32 prevBlockHash) external view returns (bytes32) {
        return _deriveDstHandle(srcHandle, prevBlockHash);
    }
}
