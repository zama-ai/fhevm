// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../lib/FHE.sol";
import {CoprocessorSetup} from "../lib/CoprocessorSetup.sol";

/**
 * @title FHECounter
 * @notice Minimal encrypted counter used as a smoke test for the mock coprocessor.
 *         Holds a single `euint64` value. `add(uint64)` trivially encrypts the
 *         given plaintext, FHE-adds it to the current value, and emits a
 *         `Counted` event with the new handle so off-chain code can locate it.
 *
 * @dev    Intentionally minimal: no encrypted-input flow (avoids the input
 *         verifier signature path), no per-user balances, no access control on
 *         `add` (anyone can bump the counter). Use only for testing.
 */
contract FHECounter {
    /// @notice Emitted on each successful `add` with the resulting handle.
    /// @dev    Off-chain readers (e.g. demo scripts) listen for this event to
    ///         locate the latest result handle to query the mock coprocessor with.
    event Counted(address indexed caller, uint64 amount, euint64 newValue);

    /// @notice Emitted on `reset` to re-initialise the counter to a fresh
    ///         `trivialEncrypt(0)` handle.
    event Reset(address indexed caller, euint64 newValue);

    /// @dev Current encrypted counter value. Initialised lazily in the
    ///      constructor via `FHE.asEuint64(0)` so the test path always sees
    ///      a well-formed handle.
    euint64 private _value;

    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
        _value = FHE.asEuint64(0);
        FHE.allowThis(_value);
    }

    /// @notice Trivially encrypts `amount` and adds it to the stored counter.
    ///         Anyone can call. ACL allowance is re-granted to `this` and to
    ///         `msg.sender` so the caller can subsequently decrypt.
    function add(uint64 amount) external {
        euint64 incr = FHE.asEuint64(amount);
        _value = FHE.add(_value, incr);
        FHE.allowThis(_value);
        FHE.allow(_value, msg.sender);
        emit Counted(msg.sender, amount, _value);
    }

    /// @notice Resets the counter to a fresh `trivialEncrypt(0)` handle.
    function reset() external {
        _value = FHE.asEuint64(0);
        FHE.allowThis(_value);
        emit Reset(msg.sender, _value);
    }

    /// @notice Returns the current encrypted value handle.
    function get() external view returns (euint64) {
        return _value;
    }
}
