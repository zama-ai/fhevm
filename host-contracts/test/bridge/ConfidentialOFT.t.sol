// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {TestHelperOz5} from "@layerzerolabs/test-devtools-evm-foundry/contracts/TestHelperOz5.sol";
import {Origin} from "@layerzerolabs/oapp-evm-upgradeable/contracts/oapp/OAppReceiverUpgradeable.sol";
import {euint64} from "encrypted-types/EncryptedTypes.sol";

import {DeployableERC1967Proxy, HostContractsDeployerTestUtils} from "../../fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ConfidentialBridge} from "../../contracts/bridge/ConfidentialBridge.sol";
import {ConfidentialOFT} from "../../examples/bridge/ConfidentialOFT.sol";
import {aclAdd, fhevmExecutorAdd} from "../../addresses/FHEVMHostAddresses.sol";

/**
 * @title ConfidentialOFTTest
 * @notice Tests focused on ConfidentialOFT's bridge-integration paths.
 *
 * @dev    The FHE compute / encrypted balance behavior is exercised by EncryptedERC20
 *         tests elsewhere; these tests instead cover:
 *         - send authorization (must hold ACL on the amount handle)
 *         - onConfidentialBridgeReceived authentication (bridge-only, trusted-peer-only)
 *         - setPeer governance
 *
 *         The cOFT is deployed on the destination chain (eid=DST_EID) so its
 *         `confidentialBridge` is the destination-side bridge — the contract that
 *         dispatches `onConfidentialBridgeReceived` via lzCompose.
 */
contract ConfidentialOFTTest is TestHelperOz5, HostContractsDeployerTestUtils {
    uint32 internal constant SRC_EID = 1;
    uint32 internal constant DST_EID = 2;
    uint64 internal constant DST_CHAIN_ID = 4242;

    address internal owner = makeAddr("owner");
    address internal alice = makeAddr("alice");
    address internal bob = makeAddr("bob");

    ACL internal acl;
    ConfidentialBridge internal srcBridge;
    ConfidentialBridge internal dstBridge;
    ConfidentialOFT internal oft;
    address internal fhevmExecutor;

    function setUp() public virtual override {
        super.setUp();
        setUpEndpoints(2, LibraryType.SimpleMessageLib);

        _deployACL(owner);
        _deployFHEVMExecutor(owner);
        acl = ACL(aclAdd);
        fhevmExecutor = fhevmExecutorAdd;

        uint32[] memory srcDstEids = new uint32[](1);
        uint64[] memory srcDstChainIds = new uint64[](1);
        srcDstEids[0] = DST_EID;
        srcDstChainIds[0] = DST_CHAIN_ID;
        srcBridge = _deployBridgeProxy(endpoints[SRC_EID], srcDstEids, srcDstChainIds);
        dstBridge = _deployBridgeProxy(endpoints[DST_EID], new uint32[](0), new uint64[](0));

        vm.startPrank(owner);
        srcBridge.setPeer(DST_EID, _addressToBytes32(address(dstBridge)));
        dstBridge.setPeer(SRC_EID, _addressToBytes32(address(srcBridge)));
        vm.stopPrank();

        oft = new ConfidentialOFT(address(dstBridge), owner);

        vm.deal(alice, 100 ether);
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

    function _addressToBytes32(address a) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(a)));
    }

    function _makeHandle(uint256 seed) internal view returns (bytes32 h) {
        h = keccak256(abi.encodePacked("oft-handle", seed));
        h = h & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        h = h | (bytes32(uint256(0xff)) << 80);
        h = h | (bytes32(uint256(uint64(block.chainid))) << 16);
        h = h | (bytes32(uint256(0x05)) << 8); // FheType.Uint64
    }

    ////////////////////////////////////////////////////////////////////////////////
    // setPeer
    ////////////////////////////////////////////////////////////////////////////////

    /// @dev EVM peer encoded as bytes32 for the bytes32-based peer registry.
    bytes32 private constant SRC_PEER_BEEF = bytes32(uint256(0xBEEF));

    function test_SetPeer_OnlyOwner() public {
        vm.expectRevert();
        oft.setPeer(SRC_EID, SRC_PEER_BEEF);
    }

    function test_SetPeer_StoresAndClears() public {
        assertEq(oft.peers(SRC_EID), bytes32(0));
        vm.prank(owner);
        oft.setPeer(SRC_EID, SRC_PEER_BEEF);
        assertEq(oft.peers(SRC_EID), SRC_PEER_BEEF);
        vm.prank(owner);
        oft.setPeer(SRC_EID, bytes32(0));
        assertEq(oft.peers(SRC_EID), bytes32(0));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // onConfidentialBridgeReceived authentication
    ////////////////////////////////////////////////////////////////////////////////

    function test_OnReceive_RevertsIfCallerNotBridge() public {
        bytes32[] memory empty = new bytes32[](0);
        vm.expectRevert(abi.encodeWithSelector(ConfidentialOFT.OnlyConfidentialBridge.selector, address(this)));
        oft.onConfidentialBridgeReceived(SRC_EID, SRC_PEER_BEEF, abi.encode(bob, bytes32(0)), empty, empty, bytes32(0));
    }

    function test_OnReceive_RevertsIfPeerUntrusted() public {
        bytes32[] memory empty = new bytes32[](0);
        vm.prank(address(dstBridge));
        vm.expectRevert(abi.encodeWithSelector(ConfidentialOFT.UntrustedPeer.selector, SRC_EID, SRC_PEER_BEEF));
        oft.onConfidentialBridgeReceived(SRC_EID, SRC_PEER_BEEF, abi.encode(bob, bytes32(0)), empty, empty, bytes32(0));
    }

    /// @dev When the peer is trusted, onConfidentialBridgeReceived proceeds past authentication and runs
    ///      `_mint(recipient, dstAmount)`. The FHE.add downstream requires ACL/coprocessor
    ///      state we don't seed here, so we accept that the call may revert inside FHE —
    ///      but it must revert AFTER passing our authentication checks. Inspect the
    ///      revert reason to verify the flow.
    function test_OnReceive_PassesAuthenticationWhenPeerTrusted() public {
        bytes32 dst = _makeHandle(1);
        bytes32[] memory srcList = new bytes32[](1);
        bytes32[] memory dstList = new bytes32[](1);
        srcList[0] = _makeHandle(0);
        dstList[0] = dst;

        vm.prank(owner);
        oft.setPeer(SRC_EID, SRC_PEER_BEEF);

        // Authentication should NOT revert with OnlyConfidentialBridge / UntrustedPeer.
        // If a revert happens, it must come from a later FHE.* call, not auth.
        vm.prank(address(dstBridge));
        try oft.onConfidentialBridgeReceived(SRC_EID, SRC_PEER_BEEF, abi.encode(bob, dst), srcList, dstList, bytes32(0)) {
            // Mint succeeded — pass.
        } catch (bytes memory reason) {
            // If we hit one of our auth errors, the test fails.
            bytes4 sel = bytes4(reason);
            assertTrue(
                sel != ConfidentialOFT.OnlyConfidentialBridge.selector && sel != ConfidentialOFT.UntrustedPeer.selector,
                "authentication should have passed"
            );
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // send: sender-side authorization
    ////////////////////////////////////////////////////////////////////////////////

    /// @dev Sending requires `FHE.isSenderAllowed(amount)` on the caller. Since alice
    ///      is not allowed on a fresh handle, send must revert before any LZ work.
    function test_Send_RevertsWhenSenderNotAllowedOnAmount() public {
        // The fresh handle is unknown to the ACL, so isSenderAllowed fails.
        vm.prank(owner);
        oft.setPeer(DST_EID, SRC_PEER_BEEF);

        vm.prank(alice);
        vm.expectRevert();
        oft.send{value: 1 ether}(DST_EID, euint64.wrap(_makeHandle(0)), bob, uint64(150_000));
    }
}
