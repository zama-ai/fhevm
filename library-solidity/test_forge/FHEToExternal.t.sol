// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import "../lib/FHE.sol";

/// @dev Unit tests for FHE.toExternal: it must re-wrap an encrypted handle into its external
///      representation while preserving the underlying bytes32 handle exactly. These tests are
///      pure (no coprocessor needed) since toExternal performs no verification or ACL checks.
contract FHEToExternalTest is Test {
    bytes32 internal constant HANDLE =
        bytes32(uint256(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef));

    function test_toExternalEbool_preservesHandle() public pure {
        assertEq(externalEbool.unwrap(FHE.toExternal(ebool.wrap(HANDLE))), HANDLE);
    }

    function test_toExternalEuint8_preservesHandle() public pure {
        assertEq(externalEuint8.unwrap(FHE.toExternal(euint8.wrap(HANDLE))), HANDLE);
    }

    function test_toExternalEuint16_preservesHandle() public pure {
        assertEq(externalEuint16.unwrap(FHE.toExternal(euint16.wrap(HANDLE))), HANDLE);
    }

    function test_toExternalEuint32_preservesHandle() public pure {
        assertEq(externalEuint32.unwrap(FHE.toExternal(euint32.wrap(HANDLE))), HANDLE);
    }

    function test_toExternalEuint64_preservesHandle() public pure {
        assertEq(externalEuint64.unwrap(FHE.toExternal(euint64.wrap(HANDLE))), HANDLE);
    }

    function test_toExternalEuint128_preservesHandle() public pure {
        assertEq(externalEuint128.unwrap(FHE.toExternal(euint128.wrap(HANDLE))), HANDLE);
    }

    function test_toExternalEuint256_preservesHandle() public pure {
        assertEq(externalEuint256.unwrap(FHE.toExternal(euint256.wrap(HANDLE))), HANDLE);
    }

    function test_toExternalEaddress_preservesHandle() public pure {
        assertEq(externalEaddress.unwrap(FHE.toExternal(eaddress.wrap(HANDLE))), HANDLE);
    }

    /// @dev Any handle value round-trips through toExternal unchanged (including zero).
    function testFuzz_toExternalEuint64_preservesHandle(bytes32 handle) public pure {
        assertEq(externalEuint64.unwrap(FHE.toExternal(euint64.wrap(handle))), handle);
    }
}
