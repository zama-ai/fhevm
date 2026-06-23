// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {TestHelperOz5} from "@layerzerolabs/test-devtools-evm-foundry/contracts/TestHelperOz5.sol";
import {MessagingFee, MessagingReceipt} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";

import {Vm} from "forge-std/Vm.sol";
import {euint32} from "encrypted-types/EncryptedTypes.sol";

import {DeployableERC1967Proxy, HostContractsDeployerTestUtils} from "../../fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ConfidentialBridge} from "../../contracts/bridge/ConfidentialBridge.sol";
import {BridgeEvents} from "../../contracts/bridge/BridgeEvents.sol";
import {HandlesListConfidentialOApp} from "../../examples/bridge/HandlesListConfidentialOApp.sol";
import {aclAdd, fhevmExecutorAdd} from "../../addresses/FHEVMHostAddresses.sol";

/**
 * @title HandlesListConfidentialOAppTest
 * @notice Tests for the {HandlesListConfidentialOApp} example OApp, which generates a list of
 *         encrypted handles on-chain and bridges them through the {ConfidentialBridge}
 *         in either direction.
 *
 * @dev    Two LZ endpoints (eids 1 and 2) are wired with SimpleMessageLib, with one
 *         {ConfidentialBridge} per endpoint and both seeded with the *opposite* eid's
 *         dstChainId so either bridge can play the source role. A
 *         {HandlesListConfidentialOApp} instance is deployed on each chain (`appSrc`,
 *         `appDst`) and the two are registered as peers of each other, demonstrating
 *         that the same contract sends both src→dst and dst→src.
 */
contract HandlesListConfidentialOAppTest is TestHelperOz5, HostContractsDeployerTestUtils, BridgeEvents {
    uint32 internal constant SRC_EID = 1;
    uint32 internal constant DST_EID = 2;
    uint32 internal constant UNCONFIGURED_EID = 9;
    uint64 internal constant SRC_CHAIN_ID = 1111;
    uint64 internal constant DST_CHAIN_ID = 4242;

    address internal owner = makeAddr("owner");
    address internal user = makeAddr("user");

    ACL internal acl;
    ConfidentialBridge internal srcBridge;
    ConfidentialBridge internal dstBridge;
    HandlesListConfidentialOApp internal appSrc;
    HandlesListConfidentialOApp internal appDst;
    address internal fhevmExecutor;

    function setUp() public virtual override {
        super.setUp();
        setUpEndpoints(2, LibraryType.SimpleMessageLib);

        _deployACL(owner);
        _deployFHEVMExecutor(owner);
        // HCULimit is consulted by FHEVMExecutor.fheRand (which `sendHandles` triggers
        // via FHE.randEuint32), so it must exist in the host stack.
        _deployHCULimit(owner);
        acl = ACL(aclAdd);
        fhevmExecutor = fhevmExecutorAdd;

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

        // Deploy one example app per chain and register them as peers of each other.
        appSrc = new HandlesListConfidentialOApp(address(srcBridge), owner);
        appDst = new HandlesListConfidentialOApp(address(dstBridge), owner);
        vm.startPrank(owner);
        appSrc.setPeer(DST_EID, _addressToBytes32(address(appDst)));
        appDst.setPeer(SRC_EID, _addressToBytes32(address(appSrc)));
        vm.stopPrank();

        vm.deal(user, 100 ether);
        vm.deal(address(appSrc), 100 ether);
        vm.deal(address(appDst), 100 ether);
    }

    /// @dev Deploys a ConfidentialBridge behind a fresh UUPS proxy. See
    ///      `Bridge.t.sol:_deployBridgeProxy` for the underlying pattern.
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
        vm.prank(fhevmExecutor);
        acl.allowTransient(handle, account);
        vm.prank(account);
        acl.allow(handle, account);
        acl.cleanTransientStorage();
    }

    /// @dev A valid-looking Uint64 handle, distinct per `seed`.
    function _makeHandle(uint256 seed) internal view returns (bytes32 h) {
        h = keccak256(abi.encodePacked("shl-handle", seed));
        h = h & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        h = h | (bytes32(uint256(0xff)) << 80);
        h = h | (bytes32(uint256(uint64(block.chainid))) << 16);
        h = h | (bytes32(uint256(0x05)) << 8); // FheType.Uint64
    }

    function _addressToBytes32(address a) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(a)));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // setPeer governance
    ////////////////////////////////////////////////////////////////////////////////

    function test_SetPeer_OnlyOwner() public {
        vm.expectRevert();
        appSrc.setPeer(DST_EID, _addressToBytes32(address(0xBEEF)));
    }

    function test_SetPeer_StoresAndClears() public {
        bytes32 peer = _addressToBytes32(address(0xBEEF));
        vm.prank(owner);
        appSrc.setPeer(UNCONFIGURED_EID, peer);
        assertEq(appSrc.peers(UNCONFIGURED_EID), peer);

        vm.prank(owner);
        appSrc.setPeer(UNCONFIGURED_EID, bytes32(0));
        assertEq(appSrc.peers(UNCONFIGURED_EID), bytes32(0));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // sendHandles guards
    ////////////////////////////////////////////////////////////////////////////////

    function test_SendHandles_RevertsWhenPeerNotSet() public {
        vm.prank(user);
        vm.expectRevert(abi.encodeWithSelector(HandlesListConfidentialOApp.PeerNotSet.selector, UNCONFIGURED_EID));
        appSrc.generateAndSendHandlesList{value: 0}(UNCONFIGURED_EID, 1, uint64(0));
    }

    function test_QuoteSendHandles_RevertsWhenPeerNotSet() public {
        vm.expectRevert(abi.encodeWithSelector(HandlesListConfidentialOApp.PeerNotSet.selector, UNCONFIGURED_EID));
        appSrc.quoteGenerateAndSendHandlesList(UNCONFIGURED_EID, 1, uint64(0));
    }

    /// @dev A zero count has nothing to bridge and is rejected before any LZ work.
    function test_SendHandles_RevertsOnEmptyCount() public {
        vm.prank(user);
        vm.expectRevert(HandlesListConfidentialOApp.EmptyHandleList.selector);
        appSrc.generateAndSendHandlesList{value: 0}(DST_EID, 0, uint64(0));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // generateAndSendHandlesList end-to-end (source path through the real bridge)
    ////////////////////////////////////////////////////////////////////////////////

    /// @dev The handles are generated on-chain via FHE.randEuint32 and self-allowed, so
    ///      no caller-side ACL setup is required — `countHandles` is the only handle input.
    function test_SendHandles_EndToEnd_EmitsSentEventAndBridgeHandle() public {
        uint256 count = 2;

        MessagingFee memory fee = appSrc.quoteGenerateAndSendHandlesList(DST_EID, count, uint64(200_000));

        vm.recordLogs();
        vm.prank(user);
        MessagingReceipt memory receipt = appSrc.generateAndSendHandlesList{value: fee.nativeFee}(
            DST_EID,
            count,
            uint64(200_000)
        );
        assertTrue(receipt.guid != bytes32(0), "guid should be assigned");

        Vm.Log[] memory logs = vm.getRecordedLogs();
        bytes32 bridgeHandleSig = keccak256("BridgeHandle(address,bytes32,uint64,bytes32)");
        // euint32 is a user-defined value type over bytes32, so the canonical event
        // signature uses bytes32[] for the handles array.
        bytes32 sentSig = keccak256("HandlesListConfidentialOAppSent(uint32,bytes32,bytes32[],bytes32)");
        uint256 nBridgeHandle;
        bool sawSentEvent;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == bridgeHandleSig && logs[i].emitter == address(srcBridge)) {
                nBridgeHandle++;
            }
            if (logs[i].topics[0] == sentSig && logs[i].emitter == address(appSrc)) {
                sawSentEvent = true;
                assertEq(uint32(uint256(logs[i].topics[1])), DST_EID);
                assertEq(logs[i].topics[2], _addressToBytes32(address(appDst)));
                (bytes32[] memory handlesSent, bytes32 emittedGuid) = abi.decode(logs[i].data, (bytes32[], bytes32));
                assertEq(handlesSent.length, count);
                assertEq(emittedGuid, receipt.guid);
            }
        }
        assertEq(nBridgeHandle, count, "BridgeHandle fires once per handle");
        assertTrue(sawSentEvent, "HandlesListConfidentialOAppSent should be emitted");

        // Deliver the lzReceive on the destination (derivation + HandleBridged events).
        verifyPackets(DST_EID, address(dstBridge));
    }

    /// @dev Demonstrates the reverse direction: the same contract type bridges dst→src.
    function test_SendHandles_ReverseDirection() public {
        MessagingFee memory fee = appDst.quoteGenerateAndSendHandlesList(SRC_EID, 1, uint64(150_000));
        vm.prank(user);
        MessagingReceipt memory receipt = appDst.generateAndSendHandlesList{value: fee.nativeFee}(
            SRC_EID,
            1,
            uint64(150_000)
        );
        assertTrue(receipt.guid != bytes32(0));

        verifyPackets(SRC_EID, address(srcBridge));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // onConfidentialBridgeReceived authentication
    ////////////////////////////////////////////////////////////////////////////////

    function test_OnReceive_RevertsIfCallerNotBridge() public {
        bytes32[] memory empty = new bytes32[](0);
        vm.expectRevert(abi.encodeWithSelector(HandlesListConfidentialOApp.OnlyConfidentialBridge.selector, address(this)));
        appDst.onConfidentialBridgeReceived(SRC_EID, _addressToBytes32(address(appSrc)), "", empty, empty, bytes32(0));
    }

    function test_OnReceive_RevertsIfPeerUntrusted() public {
        bytes32[] memory empty = new bytes32[](0);
        bytes32 badPeer = _addressToBytes32(address(0xDEAD));
        vm.prank(address(dstBridge));
        vm.expectRevert(abi.encodeWithSelector(HandlesListConfidentialOApp.UntrustedPeer.selector, SRC_EID, badPeer));
        appDst.onConfidentialBridgeReceived(SRC_EID, badPeer, "", empty, empty, bytes32(0));
    }

    function test_OnReceive_RecordsHandlesAndGrantsToUser() public {
        bytes32[] memory srcList = new bytes32[](2);
        bytes32[] memory dstList = new bytes32[](2);
        srcList[0] = _makeHandle(0);
        srcList[1] = _makeHandle(1);
        dstList[0] = _makeHandle(100);
        dstList[1] = _makeHandle(101);
        // The source instance encodes the initiating user as the payload.
        bytes memory payload = abi.encode(user);
        bytes32 guid = keccak256("received-guid");

        // Calling onConfidentialBridgeReceived directly bypasses the bridge's transient
        // grant, so the app needs allowance on each dst handle to re-grant it (FHE.allow*).
        _allow(dstList[0], address(appDst));
        _allow(dstList[1], address(appDst));

        euint32[] memory expectedHandles = new euint32[](2);
        expectedHandles[0] = euint32.wrap(dstList[0]);
        expectedHandles[1] = euint32.wrap(dstList[1]);

        vm.expectEmit(true, true, false, true, address(appDst));
        emit HandlesListConfidentialOApp.HandlesListConfidentialOAppReceived(
            SRC_EID,
            _addressToBytes32(address(appSrc)),
            expectedHandles,
            guid
        );
        vm.prank(address(dstBridge));
        appDst.onConfidentialBridgeReceived(SRC_EID, _addressToBytes32(address(appSrc)), payload, srcList, dstList, guid);

        bytes32[] memory storedDst = appDst.lastReceivedDstHandleList();
        bytes32[] memory storedSrc = appDst.lastReceivedSrcHandleList();
        assertEq(storedDst.length, 2);
        assertEq(storedDst[0], dstList[0]);
        assertEq(storedDst[1], dstList[1]);
        assertEq(storedSrc[0], srcList[0]);
        assertEq(keccak256(appDst.lastReceivedPayload()), keccak256(payload));

        // The initiating user and the app both hold persistent decryption rights now.
        assertTrue(acl.isAllowed(dstList[0], user));
        assertTrue(acl.isAllowed(dstList[1], user));
        assertTrue(acl.isAllowed(dstList[0], address(appDst)));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Full destination dispatch: bridge.lzCompose → appDst.onConfidentialBridgeReceived
    ////////////////////////////////////////////////////////////////////////////////

    /// @dev Drives the real {ConfidentialBridge.lzCompose} path (which grants transient
    ///      ACL allowance for each derived handle and calls `onConfidentialBridgeReceived`). Because the
    ///      runtime-deployed bridge is not the ACL's canonical bridge address, the
    ///      allowTransient bypass is off, so we pre-allow each dst handle to the bridge —
    ///      mirroring `Bridge.t.sol:test_LzCompose_GrantsTransientAndCallsOnReceive`.
    function test_LzCompose_EndToEnd_DispatchesToApp() public {
        bytes32 dstH0 = _makeHandle(100);
        bytes32 dstH1 = _makeHandle(101);
        bytes32[] memory srcList = new bytes32[](2);
        bytes32[] memory dstList = new bytes32[](2);
        srcList[0] = _makeHandle(0);
        srcList[1] = _makeHandle(1);
        dstList[0] = dstH0;
        dstList[1] = dstH1;
        // The bridge payload carries the initiating user (abi-encoded address).
        bytes memory payload = abi.encode(user);

        bytes memory composeMsg = abi.encode(
            SRC_EID,
            _addressToBytes32(address(appSrc)),
            _addressToBytes32(address(appDst)),
            payload,
            srcList,
            dstList
        );

        _allow(dstH0, address(dstBridge));
        _allow(dstH1, address(dstBridge));

        vm.prank(address(endpoints[DST_EID]));
        dstBridge.lzCompose(address(dstBridge), keccak256("g"), composeMsg, address(0), "");

        bytes32[] memory storedDst = appDst.lastReceivedDstHandleList();
        assertEq(storedDst.length, 2);
        assertEq(storedDst[0], dstH0);
        assertEq(storedDst[1], dstH1);
        assertEq(keccak256(appDst.lastReceivedPayload()), keccak256(payload));

        // The bridge granted the app transient allowance, which let it re-grant
        // persistent decryption rights to the initiating user.
        assertTrue(acl.isAllowed(dstH0, user));
        assertTrue(acl.isAllowed(dstH1, user));
    }
}
