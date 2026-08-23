# Plan — Split `CleartextFHEVMExecutor` (size fix + shared cleartext DB)

## Problem

`CleartextFHEVMExecutor` compiles to **25,351 B** of runtime bytecode — over EIP-170's 24,576 B cap.
Two root causes, matching the two goals:

1. **Inlined arithmetic.** `CleartextArithmetic` is a `library` whose ~40 functions are all
   `internal pure`, so the compiler **inlines every one** into the executor (~15 KB). This is the bulk
   of the overflow.
2. **Executor-local store.** The cleartext values live in `mapping(bytes32 => uint256) plaintexts`
   declared *inside* the executor, so a future second `FHEVMExecutor` (needed if the base contract
   itself outgrows 24 KB) could not share the cleartext state.

## Goals

1. Move the cleartext store into a dedicated **`CleartextDB`** contract, shared across executors.
2. Convert **`CleartextArithmetic`** from an inlined library into a deployed
   `UUPSUpgradeableEmptyProxy` contract, called externally.

Result: the arithmetic bytecode leaves the executor entirely; `CleartextFHEVMExecutor` drops to
~10–12 KB, and the stack can later run 2 executors over one shared DB.

---

## New contract: `CleartextArithmetic` (stateless logic, UUPS)

`src/cleartext/CleartextArithmetic.sol` — was `library … { internal pure }`, becomes a contract.

- `contract CleartextArithmetic is UUPSUpgradeableEmptyProxy, ACLOwnable` — **no storage** (all pure).
- Every existing function becomes `external pure` (or `public pure`). Signatures unchanged.
- `initializeFromEmptyProxy()` (no args) + `reinitializeVX()`; `_authorizeUpgrade` `onlyACLOwner`.
- Add an `ICleartextArithmetic` interface for the executor to call.
- **Move the dispatch in too.** Today the executor holds the 20-branch `_computeBinaryResult` /
  `_computeUnaryResult` `if (op == …)` ladders. Expose them on the contract instead —
  `computeBinaryOp(Operators, uint256 lhs, uint256 rhs, FheType, bytes1 scalar)`,
  `computeUnaryOp(Operators, uint256 value, FheType)` — so the executor makes **one** external call
  per op and carries none of the ladder. Maximizes the size cut.
- `FheTypeBitWidth` (tiny) can stay a library or fold into this contract.

> Decided: **upgradeable** (UUPS empty-proxy), for a stable baked address and an `onlyACLOwner`
> upgrade path consistent with the rest of the stack.

## New contract: `CleartextDB` (shared store, UUPS)

`src/cleartext/CleartextDB.sol` — the extracted `plaintexts` store.

- `contract CleartextDB is UUPSUpgradeableEmptyProxy, ACLOwnable` with ERC-7201 namespaced storage:
  - `mapping(bytes32 => uint256) plaintexts;`
  - `mapping(address => bool) isExecutor;` (write ACL — mirrors `PauserSet.isPauser`).
- **Write ACL mirrors `PauserSet`** (an authorized-address set managed by the ACL owner):
  - `get(bytes32 handle) external view returns (uint256)` — public read.
  - `set(bytes32 handle, uint256 value) external onlyExecutor` — write (analogous to `pause`'s
    `isPauser` gate).
  - `addExecutor(address) external onlyACLOwner` / `removeExecutor(address) external onlyACLOwner` /
    `isExecutor(address) external view` (the `addPauser`/`removePauser`/`isPauser` shape).
- `initializeFromEmptyProxy(address initialExecutor)` seeds the first executor (the
  `CleartextFHEVMExecutor` proxy address — known at deploy time via precompute), so no extra
  registration step is needed for the single-executor case.
- `_authorizeUpgrade` `onlyACLOwner`; bakes `ACL_ADDRESS`.

## `CleartextFHEVMExecutor` becomes thin

- Delete the `plaintexts` mapping and the inlined `_computeBinaryResult`/`_computeUnaryResult` ladders.
- Each override keeps its `super.*` symbolic-handle call, then:
  - reads operands via `ICleartextDB(cleartextDbAdd).get(handle)`,
  - computes via `ICleartextArithmetic(cleartextArithmeticAdd).computeBinaryOp(...)` / `computeUnaryOp(...)` / `fheCast` / `rand` / …,
  - writes the result via `ICleartextDB(cleartextDbAdd).set(result, value)`.
- Baked addresses now include `CLEARTEXT_ARITHMETIC_ADDRESS` and `CLEARTEXT_DB_ADDRESS`.
- `_tryReadCleartextFromProof` (pure, ~1 KB) can stay in the executor or move to the arithmetic
  contract; leave it for now.
- Expected size: **~10–12 KB** (well under the cap, with headroom for the "2 executors" split).

---

## Addresses & placeholders

- `src/addresses/FHEVMHostAddresses.sol` + `config/addresses.sol`: add
  `CLEARTEXT_ARITHMETIC_ADDRESS`/`cleartextArithmeticAdd` and `CLEARTEXT_DB_ADDRESS`/`cleartextDbAdd`
  (dummy addresses digit-only to satisfy Solidity's checksum rule — see `test/templates.test.ts`).
- `internal/generateTemplates.ts`:
  - `ADDRESS_NAMES += 'CLEARTEXT_ARITHMETIC_ADDRESS', 'CLEARTEXT_DB_ADDRESS'`.
  - `TARGET_CONTRACTS += { CleartextArithmetic, proxy }, { CleartextDB, proxy }`.
  - Update `test/templates.test.ts` `ALTERNATE_ADDRESSES` + `addressConfigSource` for the two names.
- Expected cross-refs after regen: `CleartextFHEVMExecutor` → {arithmetic, db, + existing};
  `CleartextDB` → ACL; `CleartextArithmetic` → none. Verify offsets with the offset matrix.

## Deployment / nonce layout

Two more empty proxies join the v13 batch. Append them after `KMSGeneration` so existing nonces
1–8 are unchanged; `PauserSet` shifts to the new tail:

```
… KMSGeneration(8), CleartextArithmetic(9), CleartextDB(10), PauserSet(11)
```

- `deployEmptyProxiesV13`: deploy the 2 additional ERC1967 proxies (shared `EmptyUUPSProxy` impl).
- `buildBootstrapPlanV13`: add two materialization targets —
  - `CleartextArithmetic.initializeFromEmptyProxy()` (no args),
  - `CleartextDB.initializeFromEmptyProxy(cleartextFhevmExecutorProxyAddress)`.
- The executor's v13 impl bakes the (precomputed) arithmetic + db proxy addresses; all 9 proxies are
  still materialized atomically in the single `ACLOwner.upgrade(...)`.
- Order within the batch is irrelevant (init functions don't cross-call at init time; the DB only
  records the executor address, it doesn't call it).

## TS API changes (`ts/`)

- `public.ts`: introduce a dedicated **`CleartextAddresses`** type
  (`{ cleartextArithmeticAddress; cleartextDbAddress }`) — kept separate from `FhevmAddressesV13` so
  the "real host" address set stays clean. `precomputeAddresses` returns it alongside `fhevmAddresses`
  / `pauserSetAddress`.
- `index.ts`:
  - `precomputeFhevmAddressesV13` / `precomputeAddresses`: compute the 2 cleartext addresses at their
    new nonces; PauserSet nonce and `nextStartNonce` shift by +2.
  - `buildHostAddressReplacementsV13`: add the 2 placeholders.
  - `bootstrapUpgradeConfigV13` / `buildBootstrapPlanV13`: add the 2 targets (DB init takes the
    executor address from `precomputedAddresses`).
  - **`updateV12ToV13` / `buildUpdateV12ToV13Plan`: deferred (Decision #4).** Left as-is for now;
    v13 ships fresh via `deploy`. When the cleartext-v12 test fixture lands, extend the update path to
    **upgrade the existing** `CleartextArithmetic` + `CleartextDB` (present in the v12 fixture too),
    re-point `CleartextFHEVMExecutor`, and re-measure reinitializer versions then.

## Testing

- `test/templates.test.ts`: extend fixtures for the 2 new names/contracts (already-established shape).
- `test/ts/acl-owner-upgrade.test.ts`: `deploy` now materializes 9 proxies — assert the two new
  proxies materialize, and add a functional check: `trivialEncrypt` then `fheAdd`, then read the
  result back through `CleartextDB.get(handle)` and confirm the plaintext is correct (proves the
  executor→arithmetic→DB wiring end to end).
- Assert `CleartextFHEVMExecutor` deployed bytecode is now < 24,576 B (the whole point) — can drop the
  `--code-size-limit` bump for the executor, though `CleartextArithmetic`/`CleartextKMSVerifier` may
  still need it; re-measure all sizes after the split.
- (Optional) a two-executors-share-one-DB test to validate Goal #1's forward-compatibility.

## Size expectations (to re-measure)

| Contract | Before | After (est.) |
| --- | --- | --- |
| CleartextFHEVMExecutor | 25,351 | ~10–12 KB |
| CleartextArithmetic (new deployed) | (inlined) | ~14–16 KB |
| CleartextDB (new) | — | ~2–4 KB |

## Decisions (confirmed)

1. **`CleartextArithmetic` is upgradeable** — UUPS empty-proxy (stable baked address, `onlyACLOwner`
   upgrade path).
2. **`CleartextDB` write ACL mirrors `PauserSet`** — `addExecutor`/`removeExecutor`/`isExecutor`
   (`onlyACLOwner` admin), `set` gated by `onlyExecutor`; first executor seeded at init.
3. **Introduce a dedicated `CleartextAddresses` type** — the 2 cleartext addresses are kept separate
   from `FhevmAddressesV13`.
4. **Phased — v13 fresh first, v12→v13 upgrade later.** Implement and stabilize v13 fresh (via
   `deploy`) now; the `updateV12ToV13` path is **deferred, not dropped**. A "cleartext v12" stack —
   reusing this same `CleartextArithmetic`/`CleartextDB` design — will be added **in the test folder
   only**, as a fixture to exercise `upgradeV12ToV13`. Because that v12 fixture already carries
   `CleartextArithmetic` + `CleartextDB`, the eventual upgrade **reinitializes/upgrades** those
   existing contracts (not fresh deploys), and reinitializer-version bumps are decided in that phase.
   Nothing in the v13 fresh-deploy work below changes; only the update path is postponed.

## Implementation order

1. `CleartextArithmetic.sol` → contract (`external pure` + `computeBinaryOp`/`computeUnaryOp` dispatch) + `ICleartextArithmetic`.
2. `CleartextDB.sol` + `ICleartextDB`.
3. Rewrite `CleartextFHEVMExecutor.sol` to call both externally; drop `plaintexts` + ladders.
4. Addresses: `FHEVMHostAddresses.sol`, `config/addresses.sol`, `generateTemplates.ts` (+ templates test fixtures); `forge build` + regenerate; verify the offset matrix + re-measure sizes.
5. TS: precompute (+2 nonces), replacements, bootstrap plan targets, update plan; re-export any new public types.
6. Tests: template fixtures, anvil functional round-trip, size assertion.
```
