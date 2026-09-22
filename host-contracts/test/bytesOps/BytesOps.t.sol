// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {BytesOps} from "../../contracts/shared/BytesOps.sol";

contract BytesOpsTest is Test {
    bytes internal constant DATA = hex"00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff0011223344";

    function test_SliceReturnsTheWholeArray() public pure {
        assertEq(BytesOps.slice(DATA, 0, DATA.length), DATA);
    }

    function test_SliceReturnsTheRequestedRange() public pure {
        assertEq(BytesOps.slice(DATA, 3, 4), hex"33445566");
    }

    /// @dev The signature and extraData copies both start past the first word of the source.
    function test_SliceReturnsARangeCrossingWordBoundaries() public pure {
        assertEq(BytesOps.slice(DATA, 30, 6), hex"eeff00112233");
    }

    function test_SliceReturnsEmptyBytesForAZeroSize() public pure {
        assertEq(BytesOps.slice(DATA, 5, 0), hex"");
        assertEq(BytesOps.slice(DATA, DATA.length, 0), hex"");
    }

    function test_SliceLeavesTheSourceUnchanged() public pure {
        bytes memory data = DATA;
        BytesOps.slice(data, 7, 20);
        assertEq(data, DATA);
    }

    /// @dev A size that is not a multiple of 32 must leave the rest of the last word at zero,
    ///      so that hashing or packing the result never picks up neighbouring memory.
    function test_SliceZeroesTheBytesAfterTheLastCopiedOne() public pure {
        uint256 size = 5;
        bytes memory result = BytesOps.slice(DATA, 0, size);

        bytes32 lastWord;
        assembly {
            lastWord := mload(add(result, 32))
        }

        assertEq(lastWord, bytes32(hex"0011223344") & bytes32(type(uint256).max << (8 * (32 - size))));
    }

    /// @dev Any in-range slice must match what the compiler returns for the same range.
    function test_SliceMatchesCalldataSlicing(bytes calldata data, uint256 offset, uint256 size) public pure {
        offset = bound(offset, 0, data.length);
        size = bound(size, 0, data.length - offset);

        assertEq(BytesOps.slice(data, offset, size), data[offset:offset + size]);
    }
}
