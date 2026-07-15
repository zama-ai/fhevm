// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol";

/// @dev Minimal view of the UUPS upgrade entrypoint exposed by every host proxy.
interface IUUPSUpgradeable {
    function upgradeToAndCall(address newImplementation, bytes calldata data) external payable;
}

/// @dev Minimal view of the two-step ownership surface of the ACL contract.
interface IOwnable2Step {
    function acceptOwnership() external;
    function transferOwnership(address newOwner) external;
}

/// @dev Minimal view of ACL's pause surface. `pause()` is gated on the caller being a registered
///      pauser; `unpause()` is gated on the caller being the ACL owner (i.e. this contract).
interface IACLPausable {
    function pause() external;
    function unpause() external;
}

/**
 * @title  ACLOwner
 * @notice Standing owner of the FHEVM host-contract stack. Every host proxy authorizes upgrades
 *         against `ACL.owner()` (via `ACLOwnable`), and ACL authorizes its own upgrade against its
 *         owner; making this contract the ACL owner therefore grants it upgrade authority over the
 *         whole stack in a single ownership root.
 *
 * @dev Intentionally NON-upgradeable, to keep this trust root as small and auditable as possible.
 *      It exposes the common privileged operations explicitly (accept ACL ownership, batch-upgrade,
 *      pause/unpause, migrate ownership) plus a generic {execute} escape hatch so ANY other
 *      ACL-owner-gated call can be made without deploying a new `ACLOwner`. The contract itself is never
 *      upgraded; the trust root is rotated by migrating ownership via {transferACLOwnership}. For
 *      {pause} to succeed this contract must be a registered pauser in `PauserSet` (done at setup, while
 *      the pre-transfer ACL owner still holds `addPauser` authority).
 *
 *      NOTE: {execute} is a deliberately broad power — the owner can make this contract issue any call
 *      as `ACL.owner()`. The security model rests entirely on the owner key (an EOA now, a
 *      multisig/timelock later); scope it accordingly.
 *
 *      This contract is itself `Ownable2Step`; its owner (an EOA now, a multisig/timelock later)
 *      drives all privileged operations and can be rotated without touching the host stack.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ACLOwner is Ownable2Step {
    /// @notice A single proxy upgrade: point `proxy` at `implementation` and call `initData` on it.
    /// @dev `initData` is the `upgradeToAndCall` payload — `reinitializeVX()` for a live upgrade,
    ///      `initializeFromEmptyProxy(...)` for a first materialization, or empty for no re-init.
    struct Op {
        address proxy;
        address implementation;
        bytes initData;
    }

    /// @notice The ACL proxy — the stack's ownership root.
    address public immutable acl;

    /// @notice Emitted for each proxy upgraded, making the atomic batch auditable from logs.
    event HostUpgraded(address indexed proxy, address indexed implementation);

    /// @notice Emitted for each call forwarded via {execute}, keeping arbitrary owner-driven calls auditable.
    event Executed(address indexed target, bytes data);

    /// @notice Thrown when {upgrade} is called with no operations.
    error NoOps();

    /// @notice Thrown when {execute} targets an address with no deployed code.
    error TargetHasNoCode(address target);

    /**
     * @param initialOwner The address that controls this admin (drives upgrades / migration).
     * @param acl_         The ACL proxy address.
     */
    constructor(address initialOwner, address acl_) Ownable(initialOwner) {
        acl = acl_;
    }

    /**
     * @notice Completes the pending `Ownable2Step` transfer of ACL to this contract.
     * @dev One-time setup: the current ACL owner must have called `ACL.transferOwnership(this)` first.
     */
    function acceptACLOwnership() external onlyOwner {
        IOwnable2Step(acl).acceptOwnership();
    }

    /**
     * @notice Atomically upgrade every proxy in `ops`. Reverts as a whole on any failure.
     * @param ops The proxy upgrades to perform, applied in order.
     */
    function upgrade(Op[] calldata ops) external onlyOwner {
        if (ops.length == 0) {
            revert NoOps();
        }

        for (uint256 i; i < ops.length; i++) {
            IUUPSUpgradeable(ops[i].proxy).upgradeToAndCall(ops[i].implementation, ops[i].initData);
            emit HostUpgraded(ops[i].proxy, ops[i].implementation);
        }
    }

    /**
     * @notice Generic escape hatch: forward an arbitrary call to `target` with `data`, as this contract.
     * @dev Because this contract is the ACL owner, the forwarded call's `msg.sender` is `ACL.owner()`, so
     *      any `onlyACLOwner`-gated function on any host contract (e.g. `ProtocolConfig.defineNewKmsContext`,
     *      `HCULimit.setHCUPerBlock`, ...) can be invoked through here without deploying a new `ACLOwner`.
     *      A revert in `target` bubbles up unchanged. Non-payable: host owner-gated methods take no value.
     * @param target The contract to call. Must have deployed code (a low-level call to a codeless address
     *               would silently "succeed").
     * @param data   ABI-encoded calldata (selector + arguments).
     * @return result The raw return data from `target`.
     */
    function execute(address target, bytes calldata data) external onlyOwner returns (bytes memory result) {
        if (target.code.length == 0) {
            revert TargetHasNoCode(target);
        }

        bool success;
        (success, result) = target.call(data);
        if (!success) {
            // Bubble up the target's revert reason verbatim.
            assembly {
                revert(add(result, 0x20), mload(result))
            }
        }

        emit Executed(target, data);
    }

    /**
     * @notice Pause the ACL (emergency stop). Requires this contract to be a registered pauser.
     */
    function pause() external onlyOwner {
        IACLPausable(acl).pause();
    }

    /**
     * @notice Unpause the ACL. Gated by ACL on its owner, which is this contract.
     */
    function unpause() external onlyOwner {
        IACLPausable(acl).unpause();
    }

    /**
     * @notice Migration hatch: hand ACL ownership to a successor admin.
     * @dev Two-step — `newOwner` must call `ACL.acceptOwnership()` (or its own equivalent) afterwards.
     * @param newOwner The successor ACL owner (e.g. a new `ACLOwner`).
     */
    function transferACLOwnership(address newOwner) external onlyOwner {
        IOwnable2Step(acl).transferOwnership(newOwner);
    }
}
