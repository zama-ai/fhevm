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

/**
 * @title  ACLOwner
 * @notice Standing owner of the FHEVM host-contract stack. Every host proxy authorizes upgrades
 *         against `ACL.owner()` (via `ACLOwnable`), and ACL authorizes its own upgrade against its
 *         owner; making this contract the ACL owner therefore grants it upgrade authority over the
 *         whole stack in a single ownership root.
 *
 * @dev Intentionally NON-upgradeable, to keep this trust root as small and auditable as possible.
 *      Scope is deliberately minimal: accept ACL ownership, batch-upgrade, and migrate ownership.
 *      Pause/unpause and other ACL-owner-only management functions are deferred — they are added
 *      later by deploying a NEW `ACLOwner` and migrating via {transferACLOwnership}, never by
 *      upgrading this contract.
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

    /// @notice Thrown when {upgrade} is called with no operations.
    error NoOps();

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
     * @notice Migration hatch: hand ACL ownership to a successor admin.
     * @dev Two-step — `newOwner` must call `ACL.acceptOwnership()` (or its own equivalent) afterwards.
     * @param newOwner The successor ACL owner (e.g. a new `ACLOwner`).
     */
    function transferACLOwnership(address newOwner) external onlyOwner {
        IOwnable2Step(acl).transferOwnership(newOwner);
    }
}
