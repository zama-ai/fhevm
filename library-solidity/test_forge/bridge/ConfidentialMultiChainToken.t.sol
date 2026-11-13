// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {TestHelperOz5} from "@layerzerolabs/test-devtools-evm-foundry/contracts/TestHelperOz5.sol";

import {Vm} from "forge-std/Vm.sol";

import {DeployableERC1967Proxy, HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ConfidentialBridge} from "@fhevm-host-contracts/contracts/bridge/ConfidentialBridge.sol";
import {aclAdd, fhevmExecutorAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

import {euint64, externalEuint64} from "encrypted-types/EncryptedTypes.sol";

import {FHE} from "../../lib/FHE.sol";
import {IConfidentialOAppCore} from "../../lib/bridge/ConfidentialOAppCore.sol";
import {ConfidentialOAppReceiver} from "../../lib/bridge/ConfidentialOAppReceiver.sol";
import {ConfidentialMultiChainToken} from "../../examples/bridge/ConfidentialMultiChainToken.sol";

import {TestableACL} from "./TestableACL.sol";

/**
 * @title ConfidentialMultiChainTokenTest
 * @notice Bridge-integration tests for {ConfidentialMultiChainToken}.
 *
 * @dev    Two full token instances are deployed and peered together, one per LZ endpoint:
 *         {srcToken} on `SRC_EID` and {dstToken} on `DST_EID`, each backed by its own
 *         {ConfidentialBridge}. This mirrors the real burn-and-mint topology (an instance per
 *         chain) so the outbound {ConfidentialMultiChainToken-send} on the source and the inbound
 *         mint on the destination are exercised against distinct contracts.
 *
 * @dev    ENCRYPTED-VALUE TRICK (no coprocessor input proof needed): `mint` and `send` both begin
 *         with `FHE.fromExternal(externalEuint64, inputProof)`. To avoid having to craft a real input
 *         proof with coprocessor signatures, we use `FHE.fromExternal` proof-less path (see {FHE-fromExternal}):
 *         when `inputProof` is empty it treats the `externalEuint64` as a plain, already-verified
 *         handle and only requires that the handle is ACL-allowed to `msg.sender`. So a caller who
 *         already holds an allowed `euint64` can re-wrap it via `FHE.toExternal(...)` and pass it
 *         with an empty proof. We use exactly this to drive `mint` (owner mints from an allowed
 *         handle) and `send` (a holder bridges their already-allowed balance handle), covering the
 *         encrypted arithmetic (`_mint`/`_burn`) and the end-to-end bridge round-trip.
 *
 *         As in {ConfidentialMultiChainRandomGeneratorTest}, the ACL is upgraded to a
 *         {TestableACL} so the (production-hardcoded) bridge address is runtime-settable, which
 *         also grants the selected bridge the genuine `allowTransient` privilege used by the real
 *         `lzCompose` dispatch leg.
 */
contract ConfidentialMultiChainTokenTest is TestHelperOz5, HostContractsDeployerTestUtils {
    uint32 internal constant SRC_EID = 1;
    uint32 internal constant DST_EID = 2;
    uint32 internal constant UNCONFIGURED_EID = 9;
    uint64 internal constant SRC_CHAIN_ID = 1111;
    uint64 internal constant DST_CHAIN_ID = 4242;

    // Compose event signature (hoisted to a constant, mirroring the RandomGenerator suite).
    bytes32 internal constant COMPOSE_SENT_SIG = keccak256("ComposeSent(address,address,bytes32,uint16,bytes)");

    address internal owner = makeAddr("owner");
    address internal alice = makeAddr("alice");
    address internal bob = makeAddr("bob");

    bytes32 internal constant UNTRUSTED_PEER = bytes32(uint256(0xBEEF));

    TestableACL internal acl;
    ConfidentialBridge internal srcBridge;
    ConfidentialBridge internal dstBridge;
    ConfidentialMultiChainToken internal srcToken;
    ConfidentialMultiChainToken internal dstToken;

    function setUp() public virtual override {
        super.setUp();
        setUpEndpoints(2, LibraryType.SimpleMessageLib);

        _deployACL(owner);
        address testableImpl = address(new TestableACL());
        vm.prank(owner);
        ACL(aclAdd).upgradeToAndCall(testableImpl, "");
        acl = TestableACL(aclAdd);

        _deployFHEVMExecutor(owner);
        _deployHCULimit(owner);

        // srcBridge knows DST_EID (outbound send/quote direction); dstBridge knows SRC_EID (inbound).
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

        vm.startPrank(owner);
        srcBridge.setPeer(DST_EID, _addressToBytes32(address(dstBridge)));
        dstBridge.setPeer(SRC_EID, _addressToBytes32(address(srcBridge)));
        vm.stopPrank();

        // Deploy one token instance per chain and peer them together.
        // setPeer resolves the bridge from the ACL, so point _useBridge at the
        // bridge that knows the target eid before each call.
        srcToken = new ConfidentialMultiChainToken(owner);
        dstToken = new ConfidentialMultiChainToken(owner);

        _useBridge(address(srcBridge));
        vm.prank(owner);
        srcToken.setPeer(DST_EID, _addressToBytes32(address(dstToken)));

        _useBridge(address(dstBridge));
        vm.prank(owner);
        dstToken.setPeer(SRC_EID, _addressToBytes32(address(srcToken)));

        vm.deal(alice, 100 ether);
        vm.deal(bob, 100 ether);
        vm.deal(address(srcToken), 100 ether);
        vm.deal(address(dstToken), 100 ether);
    }

    /// @dev Point the cOApp bridge resolution (and allowTransient privilege) at `bridge`.
    function _useBridge(address bridge) internal {
        acl.setConfidentialBridgeAddressForTest(bridge);
    }

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

    function _makeHandle(uint256 seed) internal view returns (bytes32 h) {
        h = keccak256(abi.encodePacked("cmct-handle", seed));
        h = h & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        h = h | (bytes32(uint256(0xff)) << 80);
        h = h | (bytes32(uint256(uint64(block.chainid))) << 16);
        h = h | (bytes32(uint256(0x05)) << 8); // FheType.Uint64
    }

    function _addressToBytes32(address a) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(a)));
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
    // setPeer governance
    ////////////////////////////////////////////////////////////////////////////////

    function test_SetPeer_OnlyOwner() public {
        _useBridge(address(srcBridge));
        vm.expectRevert();
        srcToken.setPeer(DST_EID, UNTRUSTED_PEER);
    }

    function test_SetPeer_StoresAndClears() public {
        _useBridge(address(srcBridge));
        // Peered to dstToken in setUp; clearing then re-setting exercises store + clear.
        assertEq(srcToken.peers(DST_EID), _addressToBytes32(address(dstToken)));

        vm.prank(owner);
        srcToken.setPeer(DST_EID, bytes32(0));
        assertEq(srcToken.peers(DST_EID), bytes32(0));

        vm.prank(owner);
        srcToken.setPeer(DST_EID, _addressToBytes32(address(dstToken)));
        assertEq(srcToken.peers(DST_EID), _addressToBytes32(address(dstToken)));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // quoteSend (no input proof required)
    ////////////////////////////////////////////////////////////////////////////////

    function test_QuoteSend_RevertsWhenPeerNotSet() public {
        _useBridge(address(srcBridge));
        vm.expectRevert(abi.encodeWithSelector(IConfidentialOAppCore.NoPeer.selector, UNCONFIGURED_EID));
        srcToken.quoteSend(UNCONFIGURED_EID, uint64(150_000));
    }

    function test_QuoteSend_ReturnsFeeWhenPeerConfigured() public {
        _useBridge(address(srcBridge));
        uint256 fee = srcToken.quoteSend(DST_EID, uint64(150_000));
        assertGt(fee, 0, "native fee should be non-zero");
    }

    ////////////////////////////////////////////////////////////////////////////////
    // mint (empty-proof trick: owner mints from an already-allowed handle)
    ////////////////////////////////////////////////////////////////////////////////

    /// @dev The owner mints an encrypted amount to alice without any coprocessor input proof. The
    ///      amount is an already-verified `euint64` handle re-wrapped to `externalEuint64` via
    ///      {FHE-toExternal} and passed with an empty proof; {FHE-fromExternal} then only requires
    ///      the handle be ACL-allowed to the caller (owner). `_mint` runs `FHE.add(0, amount)` and
    ///      grants alice and the token the decryption rights on the resulting balance (what `userDecrypt`
    ///      gates on).
    function test_Mint_FromAllowedHandleWithEmptyProof() public {
        bytes32 amountHandle = _makeHandle(1);
        // fromExternal(empty proof) checks the handle is allowed to the caller (owner);
        // _mint's FHE.add requires the token contract be allowed on the amount handle.
        _allow(amountHandle, owner);
        _allow(amountHandle, address(srcToken));

        externalEuint64 amount = FHE.toExternal(euint64.wrap(amountHandle));

        vm.prank(owner);
        srcToken.mint(alice, amount, "");

        euint64 balance = srcToken.balanceOf(alice);
        assertTrue(euint64.unwrap(balance) != bytes32(0), "alice balance handle should be set");
        assertTrue(acl.isAllowed(euint64.unwrap(balance), alice), "alice must be allowed on her balance");
        assertTrue(acl.isAllowed(euint64.unwrap(balance), address(srcToken)), "token must be allowed on the balance");
    }

    function test_Mint_OnlyOwner() public {
        bytes32 amountHandle = _makeHandle(2);
        _allow(amountHandle, alice);
        _allow(amountHandle, address(srcToken));

        externalEuint64 amount = FHE.toExternal(euint64.wrap(amountHandle));

        vm.prank(alice);
        vm.expectRevert();
        srcToken.mint(alice, amount, "");
    }

    ////////////////////////////////////////////////////////////////////////////////
    // send (empty-proof trick + full source->destination round-trip)
    ////////////////////////////////////////////////////////////////////////////////

    /// @dev Full burn-and-mint round-trip driven entirely without an input proof:
    ///      1. owner mints a balance to alice on `srcToken` (empty-proof trick),
    ///      2. alice bridges that already-allowed balance handle to bob via `srcToken.send`
    ///         (re-wrapped with {FHE-toExternal} + empty proof); `_burn` runs the encrypted
    ///         le/select/sub and the source bridge ships the derived amount,
    ///      3. the LZ packet is delivered and the real (privileged) `dstBridge.lzCompose` dispatches
    ///         to `dstToken`, which mints to bob.
    ///      Finally bob holds a decryptable balance on `dstToken`.
    function test_Send_EndToEnd_BurnsOnSrcAndMintsToRecipientOnDst() public {
        // 1. Bootstrap alice's balance on the source token.
        bytes32 amountHandle = _makeHandle(1);
        _allow(amountHandle, owner);
        _allow(amountHandle, address(srcToken));
        vm.prank(owner);
        srcToken.mint(alice, FHE.toExternal(euint64.wrap(amountHandle)), "");
        euint64 aliceBalance = srcToken.balanceOf(alice);

        // 2. alice bridges her already-allowed balance handle to bob on the destination chain.
        _useBridge(address(srcBridge));
        uint64 mintComposeGas = 100_000;
        uint256 fee = srcToken.quoteSend(DST_EID, mintComposeGas);

        externalEuint64 sendAmount = FHE.toExternal(aliceBalance);

        vm.recordLogs();
        vm.expectEmit(true, true, true, true, address(srcToken));
        emit ConfidentialMultiChainToken.Bridged(alice, DST_EID, bob);
        vm.prank(alice);
        srcToken.send{value: fee}(DST_EID, sendAmount, "", bob, mintComposeGas);

        // 3. Deliver the LZ packet (derives dst handles) and drive the real destination compose leg.
        _useBridge(address(dstBridge));
        verifyPackets(DST_EID, address(dstBridge));

        (address composeFrom, bytes32 guid, bytes memory composeMsg) = _findComposeSent(vm.getRecordedLogs());
        vm.prank(address(endpoints[DST_EID]));
        dstBridge.lzCompose(composeFrom, guid, composeMsg, address(0), "");

        // 4. bob now holds a decryptable balance on the destination token.
        euint64 bobBalance = dstToken.balanceOf(bob);
        assertTrue(euint64.unwrap(bobBalance) != bytes32(0), "bob balance handle should be set on dstToken");
        assertTrue(acl.isAllowed(euint64.unwrap(bobBalance), bob), "bob must be allowed on his dst balance");
        assertTrue(
            acl.isAllowed(euint64.unwrap(bobBalance), address(dstToken)),
            "dstToken must be allowed on bob's balance"
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    // onConfidentialBridgeReceived authentication
    ////////////////////////////////////////////////////////////////////////////////

    function test_OnReceive_RevertsIfCallerNotBridge() public {
        _useBridge(address(dstBridge));
        bytes32[] memory empty = new bytes32[](0);
        vm.expectRevert(
            abi.encodeWithSelector(ConfidentialOAppReceiver.OnlyConfidentialBridge.selector, address(this))
        );
        dstToken.onConfidentialBridgeReceived(
            SRC_EID,
            _addressToBytes32(address(srcToken)),
            abi.encode(bob),
            empty,
            empty,
            bytes32(0)
        );
    }

    function test_OnReceive_RevertsIfPeerUntrusted() public {
        _useBridge(address(dstBridge));
        // dstToken is peered to srcToken for SRC_EID (in setUp); present a different srcApp.
        bytes32[] memory empty = new bytes32[](0);
        vm.prank(address(dstBridge));
        vm.expectRevert(abi.encodeWithSelector(ConfidentialOAppReceiver.OnlyPeer.selector, SRC_EID, UNTRUSTED_PEER));
        dstToken.onConfidentialBridgeReceived(SRC_EID, UNTRUSTED_PEER, abi.encode(bob), empty, empty, bytes32(0));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Inbound mint: direct call proceeds past authentication and grants ACL allowance
    ////////////////////////////////////////////////////////////////////////////////

    /// @dev Direct call + pre-allow-to-app. After authentication the token runs
    ///      `_mint(recipient, dstAmount)` = `FHE.add(nullBalance, dstAmount)` then grants the new
    ///      balance handle to the recipient. We pre-allow the bridged `dstAmount` to the token so the
    ///      FHEVMExecutor add succeeds (the direct call bypasses the bridge's transient grant), then
    ///      assert the recipient can decrypt the resulting balance.
    function test_OnReceive_MintsAndGrantsBalanceToRecipient() public {
        _useBridge(address(dstBridge));

        bytes32 srcApp = _addressToBytes32(address(srcToken));
        bytes32 dstAmount = _makeHandle(1);
        bytes32[] memory srcList = new bytes32[](1);
        bytes32[] memory dstList = new bytes32[](1);
        srcList[0] = _makeHandle(0);
        dstList[0] = dstAmount;

        _allow(dstAmount, address(dstToken));

        vm.prank(address(dstBridge));
        dstToken.onConfidentialBridgeReceived(SRC_EID, srcApp, abi.encode(bob), srcList, dstList, bytes32(0));

        euint64 balance = dstToken.balanceOf(bob);
        assertTrue(euint64.unwrap(balance) != bytes32(0), "recipient balance handle should be set");
        assertTrue(acl.isAllowed(euint64.unwrap(balance), bob), "recipient must be allowed on new balance");
        assertTrue(acl.isAllowed(euint64.unwrap(balance), address(dstToken)), "token must be allowed on new balance");
    }
}
