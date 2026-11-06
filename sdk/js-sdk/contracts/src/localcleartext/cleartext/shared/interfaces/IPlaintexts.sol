// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title IPlaintexts
 * @notice Anything that can say what a handle is worth.
 *
 * @dev THE VALUE, AND WHETHER THERE IS ONE — deliberately its own interface. Two contracts in the
 *      cleartext stack answer it — `CleartextArithmetic`, which owns the store, and
 *      `CleartextFHEVMExecutor`, which forwards to it — and callers that only want a value should not
 *      have to name either, nor decide which. Before this, a caller wanting the executor's answer
 *      declared a one-function interface inline and called it `ICleartextFHEVMExecutor`, which promised
 *      a whole executor and delivered a lookup.
 *
 *      `hasPlaintext` is the predicate to `plaintexts`: it NEVER reverts, and it answers false for the
 *      zero word, for a handle from another chain and for one this stack never computed or was told
 *      about — every case in which `plaintexts` would refuse by name.
 *
 *      It is also the boundary that does NOT exist on a production stack: nothing there implements
 *      this, because nothing there holds a cleartext. Code written against this interface is code that
 *      only works against a cleartext stack, and its type says so.
 */
interface IPlaintexts {
    /// @notice The cleartext value recorded for `handle`. Reverts, by name, when there is none.
    function plaintexts(bytes32 handle) external view returns (uint256);

    /// @notice Whether a cleartext is recorded for `handle`. Never reverts.
    function hasPlaintext(bytes32 handle) external view returns (bool);
}
