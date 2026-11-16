// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @notice Tells a cleartext mock apart from the real thing, by asking it.
 *
 * @dev Every contract in the cleartext stack carries `bool public constant IS_CLEARTEXT = true`, and the
 *      forge-only variants carry `IS_FORGE` on top of it. No
 *      production host contract has that selector, so probing for it is how code decides which kind of
 *      deployment it is talking to — without being configured, and without a chain-id table that would
 *      be wrong the moment a stack is deployed somewhere new.
 *
 * @dev A LOW-LEVEL call, deliberately. A production contract does not return `false`; it has no such
 *      function and REVERTS, which a typed call would propagate instead of answering.
 */
library LibCleartextProbe {
    /// @dev `IS_CLEARTEXT()`
    bytes4 private constant IS_CLEARTEXT_SELECTOR = 0x83d5b266;

    /// @dev `IS_FORGE()`
    bytes4 private constant IS_FORGE_SELECTOR = 0x489caceb;

    /// @notice Whether `target` is part of a cleartext stack.
    /// @dev    False for anything that reverts, returns the wrong width, or has no code at all.
    function isCleartext(address target) internal view returns (bool) {
        return _answersTrue(target, IS_CLEARTEXT_SELECTOR);
    }

    /**
     * @notice Whether `target` is one of the FORGE variants: the implementations that call cheatcodes, which
     *         only the in-process forge stack deploys.
     *
     * @dev A strictly narrower question than `isCleartext`, and the only way to ask it: the forge and the
     *      broadcast variant of a contract report the same `getVersion()`, so nothing else tells them apart.
     *      False on every stack a node can hold, including a cleartext one deployed by broadcast.
     */
    function isForge(address target) internal view returns (bool) {
        return _answersTrue(target, IS_FORGE_SELECTOR);
    }

    function _answersTrue(address target, bytes4 selector) private view returns (bool) {
        (bool ok, bytes memory ret) = target.staticcall(abi.encodeWithSelector(selector));
        return ok && ret.length == 32 && abi.decode(ret, (bool));
    }
}
