// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

interface IFHEVMExecutorFork {
    function upgradeToAndCall(address newImplementation, bytes calldata data) external payable;
    function getVersion() external view returns (string memory);
    function getACLAddress() external view returns (address);
    function trivialEncrypt(uint256 pt, uint8 toType) external returns (bytes32 result);
    function fheDiv(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);
    function fheRem(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);
}

interface IACLFork {
    function isAllowed(bytes32 handle, address account) external view returns (bool);
    function owner() external view returns (address);
}

error DivisionByZero();

contract FHEVMExecutorForkProbe {
    uint8 internal constant UINT8_TYPE = 2;
    bytes32 internal constant TRUNCATES_TO_ZERO_UINT8 = bytes32(uint256(1 << 8));

    function encryptAndCheckACL(address executor) external returns (bytes32 handle, bool allowed, address acl) {
        IFHEVMExecutorFork e = IFHEVMExecutorFork(executor);
        acl = e.getACLAddress();
        handle = e.trivialEncrypt(1, UINT8_TYPE);
        allowed = IACLFork(acl).isAllowed(handle, address(this));
    }

    function divWithScalarThatTruncatesToZero(address executor) external returns (bytes32 result) {
        IFHEVMExecutorFork e = IFHEVMExecutorFork(executor);
        bytes32 lhs = e.trivialEncrypt(1, UINT8_TYPE);
        return e.fheDiv(lhs, TRUNCATES_TO_ZERO_UINT8, 0x01);
    }

    function remWithScalarThatTruncatesToZero(address executor) external returns (bytes32 result) {
        IFHEVMExecutorFork e = IFHEVMExecutorFork(executor);
        bytes32 lhs = e.trivialEncrypt(1, UINT8_TYPE);
        return e.fheRem(lhs, TRUNCATES_TO_ZERO_UINT8, 0x01);
    }
}

contract FHEVMExecutorUpgradeForkTest is Test {
    string internal constant MAINNET_RPC_FALLBACK = "https://ethereum-rpc.publicnode.com";
    string internal constant SEPOLIA_RPC_FALLBACK = "https://ethereum-sepolia-rpc.publicnode.com";
    bytes internal constant REINITIALIZE_V2 = hex"c4115874";

    address internal constant TESTNET_PROXY = 0x92C920834Ec8941d2C77D188936E1f7A6f49c127;
    address internal constant TESTNET_EXPECTED_ACL = 0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D;
    address internal constant TESTNET_NEW_IMPL = 0x088cA2203d53900c57a1e6Ac737730d2508f56C9;

    address internal constant MAINNET_PROXY = 0xD82385dADa1ae3E969447f20A3164F6213100e75;
    address internal constant MAINNET_EXPECTED_ACL = 0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6;
    address internal constant MAINNET_NEW_IMPL = 0xde3624dA8d9c45B57674cA0AcAC40630682211bb;

    function test_TestnetProposalFixesScalarTruncationBug() public {
        _assertUpgradeFix(
            _rpcUrl("SEPOLIA_RPC_URL", SEPOLIA_RPC_FALLBACK),
            TESTNET_PROXY,
            TESTNET_EXPECTED_ACL,
            TESTNET_NEW_IMPL
        );
    }

    function test_MainnetProposalFixesScalarTruncationBug() public {
        _assertUpgradeFix(
            _rpcUrl("MAINNET_RPC_URL", MAINNET_RPC_FALLBACK),
            MAINNET_PROXY,
            MAINNET_EXPECTED_ACL,
            MAINNET_NEW_IMPL
        );
    }

    function _assertUpgradeFix(string memory rpcUrl, address proxy, address expectedAcl, address newImpl) internal {
        vm.createSelectFork(rpcUrl);
        FHEVMExecutorForkProbe probe = new FHEVMExecutorForkProbe();

        IFHEVMExecutorFork executor = IFHEVMExecutorFork(proxy);
        address acl = executor.getACLAddress();
        assertEq(acl, expectedAcl, "unexpected ACL wiring before upgrade");

        (bytes32 handle, bool allowed, address probedAcl) = probe.encryptAndCheckACL(proxy);
        assertEq(probedAcl, expectedAcl, "probe saw unexpected ACL");
        assertTrue(handle != bytes32(0), "trivialEncrypt returned zero handle");
        assertTrue(allowed, "trivialEncrypt did not grant transient ACL allowance");

        bytes32 divBefore = probe.divWithScalarThatTruncatesToZero(proxy);
        bytes32 remBefore = probe.remWithScalarThatTruncatesToZero(proxy);
        assertTrue(divBefore != bytes32(0), "pre-upgrade div unexpectedly failed");
        assertTrue(remBefore != bytes32(0), "pre-upgrade rem unexpectedly failed");

        address dao = IACLFork(expectedAcl).owner();
        vm.deal(dao, 1 ether);
        vm.prank(dao);
        executor.upgradeToAndCall(newImpl, REINITIALIZE_V2);

        assertEq(executor.getVersion(), "FHEVMExecutor v0.2.0", "unexpected version after upgrade");
        assertEq(executor.getACLAddress(), expectedAcl, "ACL wiring changed after upgrade");

        vm.expectRevert(DivisionByZero.selector);
        probe.divWithScalarThatTruncatesToZero(proxy);

        vm.expectRevert(DivisionByZero.selector);
        probe.remWithScalarThatTruncatesToZero(proxy);
    }

    function _rpcUrl(string memory envKey, string memory fallbackUrl) internal view returns (string memory) {
        try vm.envString(envKey) returns (string memory envUrl) {
            return envUrl;
        } catch {
            return fallbackUrl;
        }
    }
}
