// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {TestHelperOz5} from "@layerzerolabs/test-devtools-evm-foundry/contracts/TestHelperOz5.sol";

import {Vm} from "forge-std/Vm.sol";

import {DeployableERC1967Proxy, HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ConfidentialBridge} from "@fhevm-host-contracts/contracts/bridge/ConfidentialBridge.sol";
import {aclAdd, fhevmExecutorAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

import {euint32} from "encrypted-types/EncryptedTypes.sol";

import {IConfidentialOAppCore} from "../../lib/bridge/ConfidentialOAppCore.sol";
import {ConfidentialOAppReceiver} from "../../lib/bridge/ConfidentialOAppReceiver.sol";
import {ConfidentialMultiChainRandomGenerator} from "../../examples/bridge/ConfidentialMultiChainRandomGenerator.sol";

import {TestableACL} from "./TestableACL.sol";

/**
 * @title ConfidentialMultiChainRandomGeneratorTest
 * @notice Tests for {ConfidentialMultiChainRandomGenerator}, a cOApp built on the {ConfidentialOApp}
 *         base that generates encrypted handles on-chain and bridges them via the {ConfidentialBridge}.
 *
 * @dev    Two LZ endpoints (eids 1 and 2) are wired with SimpleMessageLib, one {ConfidentialBridge}
 *         per endpoint. Because the {ConfidentialOApp} base resolves the bridge from
 *         `ACL.getConfidentialBridgeAddress()` (a constant that is `address(0)` by default on local networks),
 *         the ACL is upgraded to a {TestableACL} whose bridge address is runtime-settable via
 *         `_useBridge(...)`; that same double grants the selected bridge the genuine `allowTransient`
 *         privilege, enabling the real-`lzCompose` dispatch leg. All value/decrypt assertions are done
 *         at the ACL-allowance level (a real `userDecrypt` gates on exactly this permission state).
 */
contract ConfidentialMultiChainRandomGeneratorTest is TestHelperOz5, HostContractsDeployerTestUtils {
    uint32 internal constant SRC_EID = 1;
    uint32 internal constant DST_EID = 2;
    uint32 internal constant UNCONFIGURED_EID = 9;
    uint64 internal constant SRC_CHAIN_ID = 1111;
    uint64 internal constant DST_CHAIN_ID = 4242;

    // Event-signature topics (hoisted to constants to keep test frames off the stack-too-deep limit).
    bytes32 internal constant BRIDGE_HANDLE_SIG = keccak256("BridgeHandle(address,bytes32,uint64,bytes32)");
    bytes32 internal constant HANDLES_LIST_SENT_SIG = keccak256("HandlesListSent(uint32,bytes32,bytes32[],bytes32)");
    bytes32 internal constant HANDLE_BRIDGED_SIG = keccak256("HandleBridged(address,bytes32,bytes32,bytes32)");
    bytes32 internal constant COMPOSE_SENT_SIG = keccak256("ComposeSent(address,address,bytes32,uint16,bytes)");

    address internal owner = makeAddr("owner");
    address internal user = makeAddr("user");

    TestableACL internal acl;
    ConfidentialBridge internal srcBridge;
    ConfidentialBridge internal dstBridge;
    ConfidentialMultiChainRandomGenerator internal appSrc;
    ConfidentialMultiChainRandomGenerator internal appDst;

    function setUp() public virtual override {
        super.setUp();
        setUpEndpoints(2, LibraryType.SimpleMessageLib);

        // Deploy the real ACL at its canonical address, then upgrade it to the TestableACL double so
        // the bridge address becomes runtime-settable (and grantable the allowTransient privilege).
        _deployACL(owner);
        address testableImpl = address(new TestableACL());
        vm.prank(owner);
        ACL(aclAdd).upgradeToAndCall(testableImpl, "");
        acl = TestableACL(aclAdd);

        _deployFHEVMExecutor(owner);
        // HCULimit is consulted by FHEVMExecutor.fheRand (triggered by FHE.randEuint32), so it must exist.
        _deployHCULimit(owner);

        // Seed each bridge with the opposite eid's dstChainId so both can act as source.
        uint32[] memory srcEids = new uint32[](1);
        uint64[] memory srcChainIds = new uint64[](1);
        srcEids[0] = DST_EID;
        srcChainIds[0] = DST_CHAIN_ID;
        srcBridge = _deployBridgeProxy(endpoints[SRC_EID], srcEids, srcChainIds);

        uint32[] memory dstEids = new uint32[](1);
        uint64[] memory dstChainIds = new uint64[](1);
        dstEids[0] = SRC_EID;
        dstChainIds[0] = SRC_CHAIN_ID;
        dstBridge = _deployBridgeProxy(endpoints[DST_EID], dstEids, dstChainIds);

        // Wire bridge peers bidirectionally.
        vm.startPrank(owner);
        srcBridge.setPeer(DST_EID, _addressToBytes32(address(dstBridge)));
        dstBridge.setPeer(SRC_EID, _addressToBytes32(address(srcBridge)));
        vm.stopPrank();

        appSrc = new ConfidentialMultiChainRandomGenerator(owner);
        appDst = new ConfidentialMultiChainRandomGenerator(owner);

        // App peers. setPeer calls bridge.getDstChainId(eid), so resolve to the bridge that knows the eid.
        _useBridge(address(srcBridge));
        vm.prank(owner);
        appSrc.setPeer(DST_EID, _addressToBytes32(address(appDst)));
        _useBridge(address(dstBridge));
        vm.prank(owner);
        appDst.setPeer(SRC_EID, _addressToBytes32(address(appSrc)));

        vm.deal(user, 100 ether);
        vm.deal(address(appSrc), 100 ether);
        vm.deal(address(appDst), 100 ether);
    }

    /// @dev Point the cOApp bridge resolution (and allowTransient privilege) at `bridge`.
    function _useBridge(address bridge) internal {
        acl.setConfidentialBridgeAddressForTest(bridge);
    }

    /// @dev Deploys a ConfidentialBridge behind a fresh UUPS proxy (see Bridge.t.sol pattern).
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

    /// @dev Grant `account` persistent ACL allowance on `handle` by replaying the
    ///      FHEVMExecutor → user grant sequence the host normally runs.
    function _allow(bytes32 handle, address account) internal {
        vm.prank(fhevmExecutorAdd);
        acl.allowTransient(handle, account);
        vm.prank(account);
        acl.allow(handle, account);
        acl.cleanTransientStorage();
    }

    /// @dev A valid-looking Uint32 handle, distinct per `seed`.
    function _makeHandle(uint256 seed) internal view returns (bytes32 h) {
        h = keccak256(abi.encodePacked("cmcrg-handle", seed));
        h = h & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        h = h | (bytes32(uint256(0xff)) << 80);
        h = h | (bytes32(uint256(uint64(block.chainid))) << 16);
        h = h | (bytes32(uint256(0x04)) << 8); // FheType.Uint32
    }

    function _addressToBytes32(address a) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(a)));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // setPeer governance (inherited from ConfidentialOAppCore)
    ////////////////////////////////////////////////////////////////////////////////

    function test_SetPeer_OnlyOwner() public {
        _useBridge(address(srcBridge));
        vm.expectRevert();
        appSrc.setPeer(DST_EID, _addressToBytes32(address(0xBEEF)));
    }

    function test_SetPeer_StoresAndClears() public {
        bytes32 peer = _addressToBytes32(address(0xBEEF));
        // srcBridge knows DST_EID, so setting a peer for it passes the UnsupportedEid check.
        _useBridge(address(srcBridge));
        vm.prank(owner);
        appSrc.setPeer(DST_EID, peer);
        assertEq(appSrc.peers(DST_EID), peer);

        // Clearing a peer skips the bridge routing check entirely.
        vm.prank(owner);
        appSrc.setPeer(DST_EID, bytes32(0));
        assertEq(appSrc.peers(DST_EID), bytes32(0));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // generateAndSendHandlesList guards
    ////////////////////////////////////////////////////////////////////////////////

    function test_SendHandles_RevertsOnEmptyCount() public {
        _useBridge(address(srcBridge));
        vm.prank(user);
        vm.expectRevert(ConfidentialMultiChainRandomGenerator.EmptyHandleList.selector);
        appSrc.generateAndSendHandlesList{value: 0}(DST_EID, 0, "", uint64(0));
    }

    function test_SendHandles_RevertsWhenPeerNotSet() public {
        _useBridge(address(srcBridge));
        vm.prank(user);
        vm.expectRevert(abi.encodeWithSelector(IConfidentialOAppCore.NoPeer.selector, UNCONFIGURED_EID));
        appSrc.generateAndSendHandlesList{value: 0}(UNCONFIGURED_EID, 1, "", uint64(0));
    }

    function test_QuoteSendHandles_RevertsWhenPeerNotSet() public {
        _useBridge(address(srcBridge));
        vm.expectRevert(abi.encodeWithSelector(IConfidentialOAppCore.NoPeer.selector, UNCONFIGURED_EID));
        appSrc.quoteGenerateAndSendHandlesList(UNCONFIGURED_EID, 1, "", uint64(0));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // generateAndSendHandlesList: source path through the real bridge
    ////////////////////////////////////////////////////////////////////////////////

    /// @dev Covers the source-send path plus the destination `lzReceive` delivery (handle derivation):
    ///      asserts the source-side `HandlesListSent` + per-handle `BridgeHandle` events, and the
    ///      destination-side per-handle `HandleBridged` events emitted during `verifyPackets`. This
    ///      stops at `lzReceive`; the destination app is reached in the full end-to-end test below.
    function test_GenerateAndSend_EmitsSentBridgeHandleAndHandleBridgedEvents() public {
        uint256 count = 2;
        bytes memory customPayload = abi.encode(user, "greetings");

        _useBridge(address(srcBridge));
        uint256 fee = appSrc.quoteGenerateAndSendHandlesList(DST_EID, count, customPayload, uint64(200_000));

        vm.recordLogs();
        vm.prank(user);
        (bytes32 guid, ) = appSrc.generateAndSendHandlesList{value: fee}(
            DST_EID,
            count,
            customPayload,
            uint64(200_000)
        );
        assertTrue(guid != bytes32(0), "guid should be assigned");

        // Deliver the lzReceive leg on the destination (derivation + HandleBridged events).
        _useBridge(address(dstBridge));
        verifyPackets(DST_EID, address(dstBridge));

        _assertSendAndReceiveEvents(vm.getRecordedLogs(), guid, count);
    }

    /// @dev Scans recorded logs for a single source-send + destination-lzReceive round and asserts:
    ///      `count` source-side `BridgeHandle` events, the `HandlesListSent` event (routing + guid),
    ///      and `count` destination-side `HandleBridged` events (each tied to `guid`, targeting the
    ///      dst app, and carrying a `srcHandle` that was actually sent). Extracted into its own frame
    ///      to stay within the stack-too-deep limit on non-via-ir builds.
    function _assertSendAndReceiveEvents(Vm.Log[] memory logs, bytes32 guid, uint256 count) internal view {
        uint256 nBridgeHandle;
        uint256 nHandleBridged;
        bool sawSentEvent;
        bytes32[] memory sentHandles;
        for (uint256 i = 0; i < logs.length; i++) {
            bytes32 sig = logs[i].topics[0];
            if (sig == BRIDGE_HANDLE_SIG && logs[i].emitter == address(srcBridge)) {
                // Source-side: bridge fires one BridgeHandle per handle it forwards.
                nBridgeHandle++;
            } else if (sig == HANDLES_LIST_SENT_SIG && logs[i].emitter == address(appSrc)) {
                sawSentEvent = true;
                assertEq(uint32(uint256(logs[i].topics[1])), DST_EID);
                assertEq(logs[i].topics[2], _addressToBytes32(address(appDst)));
                bytes32 emittedGuid;
                (sentHandles, emittedGuid) = abi.decode(logs[i].data, (bytes32[], bytes32));
                assertEq(sentHandles.length, count);
                assertEq(emittedGuid, guid);
            } else if (sig == HANDLE_BRIDGED_SIG && logs[i].emitter == address(dstBridge)) {
                // Destination lzReceive leg: bridge derives a dst handle and fires HandleBridged.
                nHandleBridged++;
                assertEq(logs[i].topics[1], _addressToBytes32(address(appDst)), "receiverDapp is the dst app");
                (bytes32 srcHandle, , bytes32 hbGuid) = abi.decode(logs[i].data, (bytes32, bytes32, bytes32));
                assertEq(hbGuid, guid, "HandleBridged guid matches the send");
                bool found;
                for (uint256 j = 0; j < sentHandles.length; j++) {
                    if (sentHandles[j] == srcHandle) {
                        found = true;
                        break;
                    }
                }
                assertTrue(found, "HandleBridged srcHandle must be one of the sent handles");
            }
        }
        assertEq(nBridgeHandle, count, "BridgeHandle fires once per handle (source)");
        assertTrue(sawSentEvent, "HandlesListSent should be emitted");
        assertEq(nHandleBridged, count, "HandleBridged fires once per handle (destination lzReceive)");
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Full end-to-end: source send -> lzReceive -> real lzCompose -> destination app
    ////////////////////////////////////////////////////////////////////////////////

    /// @dev True end-to-end flow: `generateAndSendHandlesList` on the source app -> LZ packet ->
    ///      `lzReceive` (derivation) -> real `ConfidentialBridge.lzCompose` dispatch -> destination
    ///      app receipt. `dstBridge` is genuinely privileged via {TestableACL}, so its `lzCompose`
    ///      `allowTransient` grants land with no pre-allow workaround.
    function test_SendHandles_FullEndToEnd_ReachesDestinationApp() public {
        uint256 count = 2;
        bytes memory customPayload = abi.encode(user, "e2e-greetings");

        _useBridge(address(srcBridge));
        uint256 fee = appSrc.quoteGenerateAndSendHandlesList(DST_EID, count, customPayload, uint64(300_000));

        vm.recordLogs();
        vm.prank(user);
        (bytes32 guid, ) = appSrc.generateAndSendHandlesList{value: fee}(
            DST_EID,
            count,
            customPayload,
            uint64(300_000)
        );

        // lzReceive derives the dst handles and queues the compose message via `sendCompose`.
        _useBridge(address(dstBridge));
        verifyPackets(DST_EID, address(dstBridge));

        // Capture the real compose message (differs from the lzReceive payload).
        (address composeFrom, bytes32 composeGuid, bytes memory composeMsg) = _findComposeSent(vm.getRecordedLogs());
        assertEq(composeGuid, guid, "compose guid matches the send guid");

        // Drive the real destination dispatch through the (privileged) bridge.
        vm.prank(address(endpoints[DST_EID]));
        dstBridge.lzCompose(composeFrom, guid, composeMsg, address(0), "");

        // Recover exactly what the destination app committed from the delivered compose message.
        (, , , bytes memory payload, bytes32[] memory srcList, bytes32[] memory dstList) = abi.decode(
            composeMsg,
            (uint32, bytes32, bytes32, bytes, bytes32[], bytes32[])
        );
        assertEq(srcList.length, count);
        assertEq(dstList.length, count);

        // The destination app recorded the delivery, keyed by guid ...
        assertEq(appDst.resultBridgedHash(guid), keccak256(abi.encode(srcList, dstList, payload)));
        // ... and granted persistent decryption rights on every derived handle (owner + app).
        for (uint256 i = 0; i < dstList.length; i++) {
            assertTrue(acl.isAllowed(dstList[i], owner), "owner must be allowed on each received handle");
            assertTrue(acl.isAllowed(dstList[i], address(appDst)), "app must be allowed on each received handle");
        }
    }

    /// @dev Extracts (from, guid, message) of the first {IMessagingComposer-ComposeSent} event.
    function _findComposeSent(
        Vm.Log[] memory logs
    ) internal pure returns (address from, bytes32 guid, bytes memory message) {
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == COMPOSE_SENT_SIG) {
                (from, , guid, , message) = abi.decode(logs[i].data, (address, address, bytes32, uint16, bytes));
                return (from, guid, message);
            }
        }
        revert("ComposeSent event not found");
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Receive-side authentication (inherited from ConfidentialOAppReceiver)
    ////////////////////////////////////////////////////////////////////////////////

    function test_OnReceive_RevertsIfCallerNotBridge() public {
        _useBridge(address(dstBridge));
        bytes32[] memory empty = new bytes32[](0);
        vm.expectRevert(
            abi.encodeWithSelector(ConfidentialOAppReceiver.OnlyConfidentialBridge.selector, address(this))
        );
        appDst.onConfidentialBridgeReceived(SRC_EID, _addressToBytes32(address(appSrc)), "", empty, empty, bytes32(0));
    }

    function test_OnReceive_RevertsIfPeerUntrusted() public {
        _useBridge(address(dstBridge));
        bytes32[] memory empty = new bytes32[](0);
        bytes32 badPeer = _addressToBytes32(address(0xDEAD));
        vm.prank(address(dstBridge));
        vm.expectRevert(abi.encodeWithSelector(ConfidentialOAppReceiver.OnlyPeer.selector, SRC_EID, badPeer));
        appDst.onConfidentialBridgeReceived(SRC_EID, badPeer, "", empty, empty, bytes32(0));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Receive: direct call + pre-allow-to-app — assert ACL allowance
    ////////////////////////////////////////////////////////////////////////////////

    function test_OnReceive_RecordsHandlesAndGrantsToOwner() public {
        _useBridge(address(dstBridge));

        bytes32[] memory srcList = new bytes32[](2);
        bytes32[] memory dstList = new bytes32[](2);
        srcList[0] = _makeHandle(0);
        srcList[1] = _makeHandle(1);
        dstList[0] = _makeHandle(100);
        dstList[1] = _makeHandle(101);
        bytes memory payload = abi.encode("opaque-app-data");
        bytes32 guid = keccak256("received-guid");

        // Calling onConfidentialBridgeReceived directly bypasses the bridge's transient grant, so the
        // app needs allowance on each dst handle to re-grant it (FHE.allow*).
        _allow(dstList[0], address(appDst));
        _allow(dstList[1], address(appDst));

        euint32[] memory expectedHandles = new euint32[](2);
        expectedHandles[0] = euint32.wrap(dstList[0]);
        expectedHandles[1] = euint32.wrap(dstList[1]);

        vm.expectEmit(true, true, false, true, address(appDst));
        emit ConfidentialMultiChainRandomGenerator.HandlesListReceived(
            SRC_EID,
            _addressToBytes32(address(appSrc)),
            expectedHandles,
            guid
        );
        vm.prank(address(dstBridge));
        appDst.onConfidentialBridgeReceived(
            SRC_EID,
            _addressToBytes32(address(appSrc)),
            payload,
            srcList,
            dstList,
            guid
        );

        // The delivery is committed as a single hash over (srcList, dstList, payload), keyed by guid.
        assertEq(appDst.resultBridgedHash(guid), keccak256(abi.encode(srcList, dstList, payload)));

        // The app owner and the app itself both hold persistent decryption rights now (what a real
        // userDecrypt would gate on).
        assertTrue(acl.isAllowed(dstList[0], owner));
        assertTrue(acl.isAllowed(dstList[0], address(appDst)));
        assertTrue(acl.isAllowed(dstList[1], owner));
        assertTrue(acl.isAllowed(dstList[1], address(appDst)));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Real lzCompose dispatch (destination leg) — bridge genuinely privileged via TestableACL
    ////////////////////////////////////////////////////////////////////////////////

    /// @dev Drives the real {ConfidentialBridge.lzCompose} dispatch leg on the destination: a
    ///      hand-crafted compose message is delivered to `dstBridge.lzCompose`, which grants transient
    ///      ACL allowance for each derived handle (works because TestableACL makes dstBridge privileged)
    ///      and calls appDst.onConfidentialBridgeReceived. Unlike the direct-call receive test, this
    ///      exercises the real bridge->app path and needs no pre-allow workaround. This is only the
    ///      destination compose leg, not a full source-send round-trip (the source-send + lzReceive leg
    ///      is covered by {test_SendHandles_EndToEnd_EmitsSentEventAndBridgeHandle}).
    function test_LzCompose_RealBridgeDispatchesToApp() public {
        _useBridge(address(dstBridge));

        bytes32[] memory srcList = new bytes32[](2);
        bytes32[] memory dstList = new bytes32[](2);
        srcList[0] = _makeHandle(0);
        srcList[1] = _makeHandle(1);
        dstList[0] = _makeHandle(200);
        dstList[1] = _makeHandle(201);
        bytes memory payload = abi.encode("opaque-app-data");

        bytes memory composeMsg = abi.encode(
            SRC_EID,
            _addressToBytes32(address(appSrc)),
            _addressToBytes32(address(appDst)),
            payload,
            srcList,
            dstList
        );

        bytes32 guid = keccak256("compose-guid");
        vm.prank(address(endpoints[DST_EID]));
        dstBridge.lzCompose(address(dstBridge), guid, composeMsg, address(0), "");

        assertEq(appDst.resultBridgedHash(guid), keccak256(abi.encode(srcList, dstList, payload)));
        assertTrue(acl.isAllowed(dstList[0], owner));
        assertTrue(acl.isAllowed(dstList[0], address(appDst)));
        assertTrue(acl.isAllowed(dstList[1], owner));
        assertTrue(acl.isAllowed(dstList[1], address(appDst)));
    }
}
