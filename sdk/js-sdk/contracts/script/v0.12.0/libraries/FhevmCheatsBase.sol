// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Vm.sol";
import {FhevmCheats} from "./FhevmCheats.sol";

/**
 * @title FhevmCheatsBase
 * @notice Mixin providing typed access to the `FhevmCheats` registry at a
 *         deterministic cheat-slot address, mirroring forge-std's
 *         `CommonBase.vm` pattern.
 *
 *         Inherit from this in any forge script or test that needs to read or
 *         populate the registry. Call `_installFhevmCheats()` exactly once per
 *         forge run before using `fhevmCheats`.
 */
abstract contract FhevmCheatsBase {
    /// @dev Cheat-slot address for the FHEVM registry.
    ///      Calculated as `address(uint160(uint256(keccak256("fhevm cheat code"))))`.
    address internal constant FHEVM_CHEATS_ADDRESS = address(uint160(uint256(keccak256("fhevm cheat code"))));

    /// @dev Typed accessor to the registry — use just like `vm` in forge-std.
    FhevmCheats internal constant fhevmCheats = FhevmCheats(FHEVM_CHEATS_ADDRESS);

    /// @dev Forge's cheat VM. Private so downstream contracts keep using their
    ///      own `vm` (e.g. `Script.vm`, `Test.vm`) without collisions.
    Vm private constant _vm = Vm(0x7109709ECfa91a80626fF3989D68f67F5b1DD12D);

    /// @notice Etch `FhevmCheats`'s runtime bytecode at `FHEVM_CHEATS_ADDRESS`
    ///         and label it for nicer traces. Idempotent — safe to call twice.
    function _installFhevmCheats() internal {
        _vm.etch(FHEVM_CHEATS_ADDRESS, type(FhevmCheats).runtimeCode);
        _vm.label(FHEVM_CHEATS_ADDRESS, "FhevmCheats");
    }
}
