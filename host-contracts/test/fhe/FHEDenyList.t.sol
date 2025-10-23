// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {FHE} from "../../lib/FHE.sol";
import {Impl, CoprocessorConfig} from "../../lib/Impl.sol";
import {ACL} from "../../contracts/ACL.sol";

contract MockACLForFHE is ACL {
    /// @dev Tests deploy the implementation directly (no proxy) because we only need deny-list toggling;
    ///      transfer ownership here so calls to `blockAccount`/`unblockAccount` succeed without going
    ///      through the upgradeable proxy flow used in production.
    constructor() {
        _transferOwnership(msg.sender);
    }
}

contract FHEDenyListTest is Test {
    MockACLForFHE internal mockACL;

    function setUp() public {
        mockACL = new MockACLForFHE();
        CoprocessorConfig memory config = CoprocessorConfig({
            ACLAddress: address(mockACL),
            CoprocessorAddress: address(0),
            DecryptionOracleAddress: address(0),
            KMSVerifierAddress: address(0)
        });
        FHE.setCoprocessor(config);
    }

    function test_ImplIsAccountDeniedFalseByDefault(address account) public view {
        assertFalse(Impl.isAccountDenied(account));
    }

    function test_ImplIsAccountDeniedReflectsACL(address account) public {
        mockACL.blockAccount(account);
        assertTrue(Impl.isAccountDenied(account));
    }

    function test_FHEIsAccountDeniedSyncsWithACL(address account) public {
        mockACL.blockAccount(account);
        assertTrue(FHE.isAccountDenied(account));

        mockACL.unblockAccount(account);
        assertFalse(FHE.isAccountDenied(account));
    }
}
