// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FhevmAddresses} from "./FhevmAddressesLib.sol";

/**
 * @title FhevmCheats
 * @notice Minimal registry that holds the deployed FHEVM proxy addresses for
 *         forge script / test consumption. Planted at a well-known cheat-slot
 *         address via `vm.etch(type(FhevmCheats).runtimeCode)`.
 *
 *         This is NOT a real on-chain contract. It exists only during a single
 *         forge run: `vm.etch` writes runtime bytecode to the slot, subsequent
 *         calls read the populated storage, and everything evaporates when the
 *         run ends. Do not rely on it from a real RPC node.
 *
 *         Constructor-less by design — `vm.etch` installs runtime code without
 *         ever running a constructor, so any init-time logic would be lost.
 */
contract FhevmCheats {
    address public acl;
    address public fhevmExecutor;
    address public kmsVerifier;
    address public inputVerifier;
    address public hcuLimit;
    address public pauserSet;
    address public fheTest;

    /// @notice Populate every slot in a single call from the deploy script.
    function setAll(FhevmAddresses memory addresses, address fheTestAdd) external {
        acl = addresses.acl;
        fhevmExecutor = addresses.fhevmExecutor;
        kmsVerifier = addresses.kmsVerifier;
        inputVerifier = addresses.inputVerifier;
        hcuLimit = addresses.hcuLimit;
        pauserSet = addresses.pauserSet;
        fheTest = fheTestAdd;
    }
}
