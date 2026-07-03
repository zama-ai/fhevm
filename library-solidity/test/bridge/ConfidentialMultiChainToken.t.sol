// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {TestHelperOz5} from "@layerzerolabs/test-devtools-evm-foundry/contracts/TestHelperOz5.sol";

import {DeployableERC1967Proxy, HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ConfidentialBridge} from "@fhevm-host-contracts/contracts/bridge/ConfidentialBridge.sol";
import {aclAdd, fhevmExecutorAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

import {euint64} from "encrypted-types/EncryptedTypes.sol";

import {IConfidentialOAppCore} from "../../lib/bridge/ConfidentialOAppCore.sol";
import {ConfidentialOAppReceiver} from "../../lib/bridge/ConfidentialOAppReceiver.sol";
import {ConfidentialMultiChainToken} from "../../examples/bridge/ConfidentialMultiChainToken.sol";

import {TestableACL} from "./TestableACL.sol";

/**
 * @title ConfidentialMultiChainTokenTest
 * @notice Bridge-integration tests for {ConfidentialMultiChainToken}.
 *
 * @dev    SCOPE / LIMITATION: this token's `mint` and `send` both begin with
 *         `FHE.fromExternal(externalEuint64, inputProof)`, which requires a real coprocessor
 *         input proof (verified on-chain by the InputVerifier). Crafting such a proof needs the
 *         off-chain coprocessor/KMS tooling (e.g. the hardhat-fhevm mock or forge-fhevm), which is
 *         NOT available in this pure-Foundry harness. Consequently the encrypted value paths
 *         (`mint`, `send`, and the balance arithmetic they drive) are covered by the TypeScript /
 *         hardhat-fhevm test suite, while this Foundry suite covers the paths reachable without an
 *         input proof:
 *         - {ConfidentialOAppCore-setPeer} governance
 *         - {ConfidentialMultiChainToken-quoteSend} (peer resolution + fee quoting)
 *         - {ConfidentialOAppReceiver} inbound authentication (bridge-only, trusted-peer-only)
 *         - the inbound mint dispatch proceeding past authentication
 *
 *         As in {ConfidentialMultiChainRandomGeneratorTest}, the ACL is upgraded to a
 *         {TestableACL} so the (production-hardcoded) bridge address is runtime-settable.
 */
contract ConfidentialMultiChainTokenTest is TestHelperOz5, HostContractsDeployerTestUtils {
    uint32 internal constant SRC_EID = 1;
    uint32 internal constant DST_EID = 2;
    uint32 internal constant UNCONFIGURED_EID = 9;
    uint64 internal constant SRC_CHAIN_ID = 1111;
    uint64 internal constant DST_CHAIN_ID = 4242;

    address internal owner = makeAddr("owner");
    address internal alice = makeAddr("alice");
    address internal bob = makeAddr("bob");

    bytes32 internal constant SRC_PEER = bytes32(uint256(0xBEEF));

    TestableACL internal acl;
    ConfidentialBridge internal srcBridge;
    ConfidentialBridge internal dstBridge;
    ConfidentialMultiChainToken internal token;

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

        token = new ConfidentialMultiChainToken(owner);

        vm.deal(alice, 100 ether);
        vm.deal(address(token), 100 ether);
    }

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

    ////////////////////////////////////////////////////////////////////////////////
    // setPeer governance
    ////////////////////////////////////////////////////////////////////////////////

    function test_SetPeer_OnlyOwner() public {
        _useBridge(address(srcBridge));
        vm.expectRevert();
        token.setPeer(DST_EID, SRC_PEER);
    }

    function test_SetPeer_StoresAndClears() public {
        _useBridge(address(srcBridge));
        assertEq(token.peers(DST_EID), bytes32(0));
        vm.prank(owner);
        token.setPeer(DST_EID, SRC_PEER);
        assertEq(token.peers(DST_EID), SRC_PEER);
        vm.prank(owner);
        token.setPeer(DST_EID, bytes32(0));
        assertEq(token.peers(DST_EID), bytes32(0));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // quoteSend (no input proof required)
    ////////////////////////////////////////////////////////////////////////////////

    function test_QuoteSend_RevertsWhenPeerNotSet() public {
        _useBridge(address(srcBridge));
        vm.expectRevert(abi.encodeWithSelector(IConfidentialOAppCore.NoPeer.selector, UNCONFIGURED_EID));
        token.quoteSend(UNCONFIGURED_EID, uint64(150_000));
    }

    function test_QuoteSend_ReturnsFeeWhenPeerConfigured() public {
        _useBridge(address(srcBridge));
        vm.prank(owner);
        token.setPeer(DST_EID, _addressToBytes32(address(0xABCD)));

        uint256 fee = token.quoteSend(DST_EID, uint64(150_000));
        assertGt(fee, 0, "native fee should be non-zero");
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
        token.onConfidentialBridgeReceived(SRC_EID, SRC_PEER, abi.encode(bob), empty, empty, bytes32(0));
    }

    function test_OnReceive_RevertsIfPeerUntrusted() public {
        _useBridge(address(dstBridge));
        // Configure a trusted peer for SRC_EID, then present a different srcApp.
        vm.prank(owner);
        token.setPeer(SRC_EID, _addressToBytes32(address(0xCAFE)));

        bytes32[] memory empty = new bytes32[](0);
        bytes32 untrusted = SRC_PEER;
        vm.prank(address(dstBridge));
        vm.expectRevert(abi.encodeWithSelector(ConfidentialOAppReceiver.OnlyPeer.selector, SRC_EID, untrusted));
        token.onConfidentialBridgeReceived(SRC_EID, untrusted, abi.encode(bob), empty, empty, bytes32(0));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Inbound mint: proceeds past authentication and grants ACL allowance to recipient
    ////////////////////////////////////////////////////////////////////////////////

    /// @dev Direct call + pre-allow-to-app (approach #1). After authentication the token runs
    ///      `_mint(recipient, dstAmount)` = `FHE.add(nullBalance, dstAmount)` then grants the new
    ///      balance handle to the recipient. We pre-allow the bridged `dstAmount` to the token so the
    ///      FHEVMExecutor add succeeds, then assert the recipient can decrypt the resulting balance
    ///      (exactly what a real userDecrypt gates on).
    function test_OnReceive_MintsAndGrantsBalanceToRecipient() public {
        _useBridge(address(dstBridge));
        vm.prank(owner);
        token.setPeer(SRC_EID, SRC_PEER);

        bytes32 dstAmount = _makeHandle(1);
        bytes32[] memory srcList = new bytes32[](1);
        bytes32[] memory dstList = new bytes32[](1);
        srcList[0] = _makeHandle(0);
        dstList[0] = dstAmount;

        _allow(dstAmount, address(token));

        vm.prank(address(dstBridge));
        token.onConfidentialBridgeReceived(SRC_EID, SRC_PEER, abi.encode(bob), srcList, dstList, bytes32(0));

        euint64 balance = token.balanceOf(bob);
        assertTrue(euint64.unwrap(balance) != bytes32(0), "recipient balance handle should be set");
        assertTrue(acl.isAllowed(euint64.unwrap(balance), bob), "recipient must be allowed on new balance");
        assertTrue(acl.isAllowed(euint64.unwrap(balance), address(token)), "token must be allowed on new balance");
    }
}
