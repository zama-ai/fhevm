// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "encrypted-types/EncryptedTypes.sol";
import {FHE} from "../lib/FHE.sol";
import {CoprocessorConfig, Impl} from "../lib/Impl.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {aclAdd, fhevmExecutorAdd, kmsVerifierAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

contract DenyListLibraryAdapter {
    function setCoprocessorConfig(CoprocessorConfig memory config) external {
        FHE.setCoprocessor(config);
    }

    function isAccountDenied(address account) external view returns (bool) {
        return Impl.isAccountDenied(account);
    }

    function isAccountDeniedFHE(address account) external view returns (bool) {
        return FHE.isAccountDenied(account);
    }
}

contract FHEDenyListTest is HostContractsDeployerTestUtils {
    DenyListLibraryAdapter internal adapter;
    ACL internal acl;

    address internal constant OWNER = address(0xAA11);
    address internal constant PAUSER = address(0xBB22);
    address internal constant GATEWAY_SOURCE = address(0xCC33);
    uint64 internal constant GATEWAY_CHAIN_ID = 31337;

    function setUp() public {
        vm.warp(1_000_000);

        adapter = new DenyListLibraryAdapter();

        address[] memory kmsSigners = new address[](1);
        kmsSigners[0] = address(0x1111);
        address[] memory inputSigners = new address[](1);
        inputSigners[0] = address(0x2222);

        _deployFullHostStack(
            OWNER,
            PAUSER,
            GATEWAY_SOURCE,
            GATEWAY_SOURCE,
            GATEWAY_CHAIN_ID,
            kmsSigners,
            1,
            inputSigners,
            1
        );

        acl = ACL(aclAdd);

        CoprocessorConfig memory config = CoprocessorConfig({
            ACLAddress: aclAdd,
            CoprocessorAddress: fhevmExecutorAdd,
            KMSVerifierAddress: kmsVerifierAdd
        });

        adapter.setCoprocessorConfig(config);
    }

    function test_ImplIsAccountDeniedFalseByDefault(address account) public view {
        bool denied = adapter.isAccountDenied(account);
        assertFalse(denied, "account should not be deny-listed by default");
    }

    function test_ImplIsAccountDeniedReflectsACL(address account) public {
        vm.assume(account != address(0));
        vm.prank(OWNER);
        acl.blockAccount(account);

        bool denied = adapter.isAccountDenied(account);
        assertTrue(denied, "deny list should reflect ACL block");

        vm.prank(OWNER);
        acl.unblockAccount(account);
    }

    function test_FHEIsAccountDeniedSyncsWithACL(address account) public {
        vm.assume(account != address(0));
        vm.prank(OWNER);
        acl.blockAccount(account);

        bool denied = adapter.isAccountDeniedFHE(account);
        assertTrue(denied, "FHE wrapper should reflect ACL deny list");

        vm.prank(OWNER);
        acl.unblockAccount(account);
        assertFalse(adapter.isAccountDeniedFHE(account), "FHE wrapper should clear deny status after unblock");
    }
}
