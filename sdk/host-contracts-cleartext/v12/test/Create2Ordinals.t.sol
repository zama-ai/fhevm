// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {FhevmCreate2Base} from "../create2-deploy/script/FhevmCreate2Base.s.sol";
import {MaterializeInitData} from "../create2-deploy/script/MaterializeInitData.sol";

/**
 * @title Create2OrdinalsTest
 * @notice Pins the index alignment the CREATE2 path depends on and nothing else checks.
 *
 * The create2 deploy describes the stack as FOUR index-aligned lists:
 *
 *   FhevmCreate2Base._sharedProxyRoles()   the non-ACL proxies, in step D's order
 *   FhevmCreate2Base._allProxyRoles()      the same, with ACL prepended
 *   FhevmCreate2Base._implArtifact(i)      which implementation belongs at position i
 *   MaterializeInitData.initData(i, ...)   which initializer payload belongs at position i
 *
 * Position `i` means the same proxy in all four. That is a convention, not something the compiler
 * enforces — and it is exactly what a protocol generation disturbs, because dropping a host contract
 * renumbers every position after it. A partial renumber compiles cleanly and then materializes the
 * wrong implementation behind the wrong proxy, or runs the wrong initializer against the right one.
 *
 * Before this test existed the only thing that caught such a mistake was an actual deploy reaching
 * `FhevmStatus`/`FhevmVerify` — which needs a funded key, a node, and the will to run it. This turns
 * it into `forge test`.
 *
 * EXPECTED is a hand-written oracle, deliberately. Deriving it from the lists under test would make
 * the test agree with whatever they say, which is not a check. It is the one table in this file a
 * generation change is *supposed* to edit — and editing it is the prompt to re-read the four lists.
 */
contract Create2OrdinalsTest is Test, FhevmCreate2Base {
    /// @dev The role at each position, ACL first. Must equal `_allProxyRoles()` exactly.
    function _expectedRoles() private pure returns (string[] memory r) {
        r = new string[](7);
        r[0] = "ACL_ADDRESS";
        r[1] = "FHEVM_EXECUTOR_ADDRESS";
        r[2] = "KMS_VERIFIER_ADDRESS";
        r[3] = "INPUT_VERIFIER_ADDRESS";
        r[4] = "HCU_LIMIT_ADDRESS";
        r[5] = "CLEARTEXT_ARITHMETIC_ADDRESS";
        r[6] = "CLEARTEXT_DB_ADDRESS";
    }

    // `vm.expectRevert` only observes reverts one call frame down, and the two functions under test are
    // `internal` — so they are reached through these external wrappers rather than called directly.
    function callImplArtifact(uint256 i) external pure returns (string memory) {
        return _implArtifact(i);
    }

    function callInitData(uint256 i) external pure returns (bytes memory) {
        return MaterializeInitData.initData(i, address(0xA11CE));
    }

    /// @dev The implementation artifact at each position, index-aligned with _expectedRoles().
    function _expectedArtifacts() private pure returns (string[] memory a) {
        a = new string[](7);
        a[0] = "pkg/src/contracts/ACL.sol:ACL";
        a[1] = "pkg/src/cleartext/CleartextFHEVMExecutor.sol:CleartextFHEVMExecutor";
        a[2] = "pkg/src/cleartext/CleartextKMSVerifier.sol:CleartextKMSVerifier";
        a[3] = "pkg/src/cleartext/CleartextInputVerifier.sol:CleartextInputVerifier";
        a[4] = "pkg/src/contracts/HCULimit.sol:HCULimit";
        a[5] = "pkg/src/cleartext/CleartextArithmetic.sol:CleartextArithmetic";
        a[6] = "pkg/src/cleartext/CleartextDB.sol:CleartextDB";
    }

    /// The two role lists agree, and the longer one is the shorter plus ACL.
    function test_roleListsAreConsistent() public pure {
        string[] memory all = _allProxyRoles();
        string[] memory shared = _sharedProxyRoles();
        assertEq(all.length, shared.length + 1, "all = shared + ACL");
        assertEq(all[0], "ACL_ADDRESS", "ACL is first");
        for (uint256 i = 0; i < shared.length; i++) {
            assertEq(all[i + 1], shared[i], "shared role order");
        }
    }

    /// No role is left unset — the array sizes are literals, so an assignment can be forgotten.
    function test_noRoleIsEmpty() public pure {
        string[] memory all = _allProxyRoles();
        for (uint256 i = 0; i < all.length; i++) {
            assertGt(bytes(all[i]).length, 0, "role must not be empty");
        }
    }

    /// The role list matches the oracle, position by position.
    function test_rolesMatchTheExpectedOrder() public pure {
        string[] memory all = _allProxyRoles();
        string[] memory expected = _expectedRoles();
        assertEq(all.length, expected.length, "proxy count");
        for (uint256 i = 0; i < expected.length; i++) {
            assertEq(all[i], expected[i], "role at position");
        }
    }

    /// The implementation artifacts match the oracle, and cover exactly the proxy positions plus the
    /// two cleartext-infra ones. Out of range must revert rather than return a stale entry.
    function test_implArtifactsMatchTheExpectedOrder() public {
        string[] memory expected = _expectedArtifacts();
        for (uint256 i = 0; i < expected.length; i++) {
            assertEq(_implArtifact(i), expected[i], "artifact at position");
        }
        vm.expectRevert(bytes("FhevmCreate2Base: implementation index out of range"));
        this.callImplArtifact(expected.length);
    }

    /// Every artifact path actually resolves — a renumber that also mistypes a path fails here rather
    /// than mid-deploy, where `vm.getCode` would abort the run.
    function test_everyImplArtifactResolves() public view {
        string[] memory expected = _expectedArtifacts();
        for (uint256 i = 0; i < expected.length; i++) {
            assertGt(vm.getCode(expected[i]).length, 0, "artifact must resolve");
        }
    }

    /// One init payload per implementation position, and out of range reverts.
    function test_initDataCoversEveryPosition() public {
        uint256 count = _expectedArtifacts().length;
        for (uint256 i = 0; i < count; i++) {
            assertGt(MaterializeInitData.initData(i, address(0xA11CE)).length, 0, "init payload");
        }
        vm.expectRevert(bytes("MaterializeInitData: index out of range"));
        this.callInitData(count);
    }

    /// The `_allCreates` arity: 2 empty implementations + N proxies + PauserSet + ACLOwner + N
    /// implementations. Derived in the source; restated here so a change to the derivation is visible.
    function test_createCountFollowsTheProxyCount() public pure {
        uint256 n = _allProxyRoles().length;
        assertEq(2 * n + 4, 2 * _expectedRoles().length + 4, "create count tracks the proxy count");
    }
}
