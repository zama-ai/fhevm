// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CleartextACL} from "./CleartextACL.sol";
import {LibFhevmHandle} from "./shared/LibFhevmHandle.sol";
import {VmSafe} from "forge-std/Vm.sol";

/**
 * @title CleartextForgeACL
 * @notice ACL variant for the in-process forge stack, carrying forge-only checks the real ACL cannot afford.
 * @dev Deployed by `pkg/forge/src/ForgeFhevmDeploy.sol` behind the ACL proxy in place of the plain `ACL`.
 *      Extends `CleartextACL`, so `IS_CLEARTEXT` and `CLEARTEXT_PROTOCOL_VERSION` come from there rather
 *      than being restated here: a `constant` cannot be overridden, so one declaration is the only option.
 *      `DeployLocalStack.s.sol` broadcasts to a node and keeps `ACL`: anything here that calls a
 *      cheatcode reverts outside forge, so this contract must never ship to a chain.
 *
 *      Each override runs its check under `pauseGasMetering`, then hands over to `super`, so a forge gas
 *      report for the stack reads the same as with the plain `ACL`. The checks run BEFORE `super` on
 *      purpose: a handle minted on another chain was never allowed here, so the base contract would
 *      reject it with `SenderNotAllowed` first and the more useful diagnostic would never fire.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract CleartextForgeACL is CleartextACL {
    VmSafe private constant vmSafe = VmSafe(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @notice Marks the Forge-only variant, mirroring forge-std's `IS_TEST`. A constant rather than a
    ///         storage variable: these run behind proxies, where an initialized state variable would
    ///         only ever be set in the implementation's own storage and read back as `false`.
    bool public constant IS_FORGE = true;

    function allow(bytes32 handle, address account) public virtual override {
        vmSafe.pauseGasMetering();
        {
            LibFhevmHandle.checkChainId(handle);
        }
        vmSafe.resumeGasMetering();
        super.allow(handle, account);
    }

    function allowForDecryption(bytes32[] memory handlesList) public virtual override {
        vmSafe.pauseGasMetering();
        {
            for (uint256 k = 0; k < handlesList.length; k++) {
                LibFhevmHandle.checkChainId(handlesList[k]);
            }
        }
        vmSafe.resumeGasMetering();
        super.allowForDecryption(handlesList);
    }

    function allowTransient(bytes32 handle, address account) public virtual override {
        vmSafe.pauseGasMetering();
        {
            LibFhevmHandle.checkChainId(handle);
        }
        vmSafe.resumeGasMetering();
        super.allowTransient(handle, account);
    }

    // Read paths. `pauseGasMetering` is not `view`, so these run the check metered: it is a shift and a
    // compare, cheap enough not to matter. `isAllowed` dispatches to `allowedTransient` and
    // `persistAllowed`, so it is covered without an override of its own.

    function allowedTransient(bytes32 handle, address account) public view virtual override returns (bool) {
        LibFhevmHandle.checkChainId(handle);
        return super.allowedTransient(handle, account);
    }

    function persistAllowed(bytes32 handle, address account) public view virtual override returns (bool) {
        LibFhevmHandle.checkChainId(handle);
        return super.persistAllowed(handle, account);
    }

    function isAllowedForDecryption(bytes32 handle) public view virtual override returns (bool) {
        LibFhevmHandle.checkChainId(handle);
        return super.isAllowedForDecryption(handle);
    }

    function isHandleDelegatedForUserDecryption(
        address delegator,
        address delegate,
        address contractAddress,
        bytes32 handle
    ) public view virtual override returns (bool) {
        LibFhevmHandle.checkChainId(handle);
        return super.isHandleDelegatedForUserDecryption(delegator, delegate, contractAddress, handle);
    }
}
