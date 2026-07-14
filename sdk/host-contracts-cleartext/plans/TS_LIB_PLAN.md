# Plan — Atomic host-contract upgrades via a standing `ACLOwner` admin

## Goal

Upgrade the FHEVM host-contract stack (ACL, FHEVMExecutor, KMSVerifier, InputVerifier, HCULimit) so
the repointing happens in **one atomic transaction** — every proxy upgraded or none. Ownership of
the stack sits permanently in a small, immutable **`ACLOwner`** admin contract that is itself
`Ownable2Step`. `ACLOwner` carries the batch `upgrade(ops[])`. Its owner (EOA now, multisig/timelock
later) drives upgrades; the host stack is never re-owned again.

## Trust split (why two phases)

1. **Deploy new implementations** — permissionless. An implementation is inert logic until a proxy
   points at it. Off-chain, deployer EOA, viem. (`buildUpgradePlan`, already implemented.)
2. **Repoint the proxies** (`upgradeToAndCall`) — owner-gated. `ACLOwner.upgrade(ops[])`, one atomic
   transaction, callable only by `ACLOwner`'s own owner.

## The non-negotiable constraint

`_authorizeUpgrade` is `onlyACLOwner` (and `onlyOwner` for ACL itself) and checks `msg.sender`.
`ACLOwner` is the ACL owner, so when it loops over the proxies `msg.sender` is `ACLOwner` and every
call authorizes. One ownership root governs the whole stack: non-ACL contracts read `ACL.owner()` via
`ACLOwnable`; ACL is owned directly.

## Init function selection (per contract)

`upgradeToAndCall(impl, initData)` — `initData` is the inner call, chosen per contract:

- **Bootstrap** (empty → real, first materialization): `initializeFromEmptyProxy(<args>)`. Guarded by
  `onlyFromEmptyProxy` (`_getInitializedVersion() == 1`).
- **Live upgrade** (real vN → vN+1): `reinitializeVX()`. Calling `initializeFromEmptyProxy` again
  reverts. If no re-init is needed, `initData = 0x` (empty).

ACL & FHEVMExecutor take no bootstrap args; KMSVerifier/InputVerifier/HCULimit do. Selection is
expressed per contract via `ContractUpgradeSpec` (see TS section).

---

## Phase 2 contract — `ACLOwner` (new Solidity, immutable)

Standing admin, **not upgradeable** (smallest possible trust root; no self-mutation path). Scope for
now is deliberately tiny: accept ACL ownership, batch-upgrade, and a migration hatch. Pause/unpause
and other ACL-owner-only management functions are **explicitly deferred** — they will be added later
by deploying a new `ACLOwner` and migrating ownership (see below), not by upgrading this one.

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Ownable, Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol"; // verify path

interface IUUPSUpgradeable {
    function upgradeToAndCall(address newImplementation, bytes calldata data) external payable;
}
interface IOwnable2Step {
    function acceptOwnership() external;
    function transferOwnership(address newOwner) external;
}

/// @notice Standing owner of the FHEVM host stack. Its own owner (EOA/multisig/timelock) drives
/// privileged operations. Extend by DEPLOYING A NEW ACLOwner and migrating via transferACLOwnership;
/// this contract is intentionally non-upgradeable.
contract ACLOwner is Ownable2Step {
    struct Op {
        address proxy;
        address implementation;
        bytes initData; // reinitializeVX() | initializeFromEmptyProxy(...) | 0x
    }

    address public immutable acl;

    event HostUpgraded(address indexed proxy, address indexed implementation);

    error NoOps();

    constructor(address initialOwner, address acl_) Ownable(initialOwner) {
        acl = acl_;
    }

    /// @notice Completes the pending Ownable2Step transfer of ACL to this contract (one-time setup).
    function acceptACLOwnership() external onlyOwner {
        IOwnable2Step(acl).acceptOwnership();
    }

    /// @notice Atomically repoint every proxy. Reverts as a whole on any failure.
    function upgrade(Op[] calldata ops) external onlyOwner {
        if (ops.length == 0) revert NoOps();
        for (uint256 i; i < ops.length; i++) {
            IUUPSUpgradeable(ops[i].proxy).upgradeToAndCall(ops[i].implementation, ops[i].initData);
            emit HostUpgraded(ops[i].proxy, ops[i].implementation);
        }
    }

    /// @notice Migration hatch: hand ACL ownership to a future ACLOwner (two-step; new owner accepts).
    function transferACLOwnership(address newOwner) external onlyOwner {
        IOwnable2Step(acl).transferOwnership(newOwner);
    }
}
```

Notes:

- No baked-in host addresses → **no bytecode placeholder patching** (unlike the impl templates).
- Add to the artifact pipeline (`internal/generateTemplates.ts`, `kind: 'non-proxy'`) → generates
  `ts/artifacts/ACLOwner.ts`; import it in `index.ts`.
- Verify the OZ import path (`@openzeppelin/contracts/access/Ownable2Step.sol`) resolves in this repo
  (foundry `libs` includes `node_modules`); the impls use `@openzeppelin/contracts-upgradeable`.

### One-time setup (deploy + hand over ACL ownership)

No precompute, no CREATE2: `ACLOwner` has an explicit `acceptACLOwnership()`, so deploy-first works
(the throwaway-batcher chicken-and-egg is gone).

1. Deploy `ACLOwner(initialOwner, acl)`.
2. Current ACL owner: `ACL.transferOwnership(aclOwnerAddress)` (sets pending owner).
3. `ACLOwner`'s owner: `ACLOwner.acceptACLOwnership()` → `ACLOwner` is now the ACL owner. Permanent.

After this, the stack is never re-owned unless you migrate (`transferACLOwnership` → new admin
accepts).

### Per-upgrade flow (repeatable, no ownership churn)

1. `buildUpgradePlan(...)` deploys the new implementations (permissionless) → `PreparedUpgrade[]`.
2. Map to `ACLOwner.Op[]` = `{ proxy, implementation, initData }`.
3. `ACLOwner`'s owner calls `ACLOwner.upgrade(ops)` — atomic. (Or, for a multisig owner, hand it the
   encoded `upgrade(ops)` calldata to execute.)

---

## TS (`ts/index.ts`)

### Already implemented (Phase 1 — keep as-is)

- `PreparedUpgrade`, `ContractUpgradeSpec`, `UpgradeConfig` (per-contract spec + `pauserSetAddress`).
- `eip712VerifierInitArgs(...)`, `hcuLimitInitArgs(...)` — type-safe bootstrap arg builders.
- `deployImplementation(...)` — deploys one impl, encodes `initData` + `upgradeCalldata`, sends nothing.
- `buildUpgradePlan(...)` — Phase 1 across the five contracts → `{ prepared }`.
- `upgrade(...)` — devnet convenience: sequential `upgradeToAndCall` from an EOA signer (non-atomic).

### To add (Phase 2 — `ACLOwner`)

```ts
// One-time: deploy the standing admin (constructor args [initialOwner, aclAddress]).
export async function deployACLOwner(parameters: {
  readonly deployer: AbstractEthereumSigner;
  readonly initialOwner: string;
  readonly aclAddress: string;
}): Promise<DeployReturnType>;

// Encode ACLOwner.upgrade(ops) from a plan — for the owner to send, or hand to a multisig.
export async function encodeACLOwnerUpgrade(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly prepared: readonly PreparedUpgrade[]; // → Op[] { proxy, implementation, initData }
}): Promise<HexString>;

// Atomic upgrade via the standing admin (owner-signed). Builds the plan, then one ACLOwner.upgrade tx.
export async function upgradeViaACLOwner(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner; // deploys impls (Phase 1)
  readonly owner: AbstractEthereumSigner; // ACLOwner's owner (sends the upgrade tx)
  readonly aclOwnerAddress: string;
  readonly precomputedAddresses: FhevmAddresses;
  readonly config: UpgradeConfig;
}): Promise<void>;
```

Checklist:

- [ ] `ACLOwner.sol` + wire into `internal/generateTemplates.ts`; import artifact in `index.ts`.
- [ ] `deployACLOwner(...)` (constructor args `[initialOwner, aclAddress]`).
- [ ] `encodeACLOwnerUpgrade(...)` (maps `PreparedUpgrade[]` → `Op[]`, encodes `upgrade(ops)`).
- [ ] `upgradeViaACLOwner(...)` — `buildUpgradePlan` → `owner.writeContract(ACLOwner.upgrade, [ops])`.
- [ ] Keep `upgrade(...)` as the devnet sequential path (no `ACLOwner`).
- [ ] (Setup helper, optional) a `handOverACLOwnership(...)` wrapping transfer + `acceptACLOwnership`.

## Signer / operator review checklist

Per `PreparedUpgrade`, verified off-chain (the chain won't):

- [ ] `implementationAddress` bytecode == CI/audited build for that contract.
- [ ] Baked-in host addresses correct (incl. `PAUSER_SET_ADDRESS` == current live PauserSet).
- [ ] Storage-layout compatible with the deployed implementation (CI `validateUpgrade` gate).
- [ ] Decoded `initData` == expected `reinitializeVX()` / `initializeFromEmptyProxy(...)`.
- [ ] `ACLOwner.upgrade` `ops[]` contains exactly the intended proxies.
- [ ] `ACLOwner` is audited (permanent trust root) and its owner is the intended EOA/multisig.

## Open decisions

1. **`ACLOwner` owner**: EOA now vs multisig/timelock. Swappable later via its own `Ownable2Step`
   without touching the host stack.
2. **Init function per contract this run**: bootstrap vs live upgrade (may be mixed, explicit each).
3. **Devnet vs production**: keep both paths (`upgrade` sequential + `upgradeViaACLOwner`)?
4. **Deferred**: pause/unpause + other ACL-admin functions — added later via a new `ACLOwner` +
   `transferACLOwnership` migration (this contract stays immutable). Note pausing is intentionally
   gated by `PauserSet` membership (fast, low-bar), separate from `ACLOwner`'s owner gate.

## Implementation order

1. `ACLOwner.sol` + artifact generation.
2. TS Phase 1 (`deployImplementation` / `buildUpgradePlan`) — **done**.
3. TS: `deployACLOwner` + `encodeACLOwnerUpgrade` + `upgradeViaACLOwner`.
4. Wire one-time setup (deploy `ACLOwner` → transfer ACL → `acceptACLOwnership`).
5. Tests: anvil — setup; bootstrap batch and a live re-upgrade batch via `ACLOwner.upgrade`; assert
   atomic revert on a bad op; assert non-owner cannot call `upgrade`; assert migration hatch works.
