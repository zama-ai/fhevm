# CREATE2 testnet upgrade — v12 → v13

Companion to [CREATE2_TESTNET_DEPLOY_PLAN.md](CREATE2_TESTNET_DEPLOY_PLAN.md), which this reuses
wherever it can. Section references in the form §N are that document's.

## 1. What this is

`create2-deploy/upgrade-testnet.ts` — a second coordinator beside `deploy-testnet.ts`, upgrading a
**live v12 stack** to v13 through the canonical CREATE2 factory. Everything it creates goes through the
factory, like everything else in this directory.

It shares `common.ts` with the deploy: argument parsing, the config file, the out-dir identity check, the
chain and factory preflight, signer resolution, the reorg/finality waits, the journal, the seal gate, and
`broadcast()`. What it does not share is the stage list — which is the whole reason the two are separate
coordinators rather than one with a `--mode` flag.

### Two invariants that shape the design

**Ownership never changes.** The live stack's ACL is already owned by a standing `ACLOwner`, and that
`ACLOwner` is already owned by the admin. The upgrade runs entirely *through* that existing root. So the
coordinator has **no offer/accept stages at all** — not "stages that no-op", but structurally absent, so
it cannot move ownership even if invoked wrongly.

**The pausers stay the same.** No pauser stage either. `PauserSet` is untouched: the same contract, the
same membership.

Both are asserted in `verify`, not merely avoided — see §7.

## 2. Stages

Four, against the deploy's nine:

| Stage | Sends | What it does |
| --- | --- | --- |
| `compute` | no | 2 builds, 2 passes, writes and seals the upgrade manifest |
| `creates` | yes | 10 CREATE2s through the factory, each gated on `getCode(predicted) != ""` |
| `materialize` | yes | one atomic `ACLOwner.upgrade(ops)` — 2 materializations + 5 reinitializations |
| `verify` | no | versions moved as expected; **every readable value unchanged** except an allow-list; ownership and pausers provably unchanged; a pre-upgrade handle still resolves |

Plus the same read-only pseudo-stages the deploy has: `status`, `log`, `report`.

Absent by design: `pausers`, `offer-acl`, `accept-acl`, `offer-admin`, `accept-admin`.

## 3. The existing stack arrives as arguments

The upgrade takes the live v12 addresses as **inputs**, not from a previous manifest. That decoupling is
deliberate: a stack may have been deployed by the nonce path (`scripts/deploy.sh`), by an older revision
of this tooling, or by someone else entirely — and requiring a manifest this tool happened to write would
make the upgrade unusable in exactly the cases where it matters most.

Nine addresses, best supplied through the config file since nine flags is unreasonable to retype:

```json
{
  "rpcUrl": "…",
  "account": "fhevm-testnet-deployer",
  "admin": "0x…",
  "deploymentId": "cleartext-sepolia-2026-08",
  "outDir": ".out-upgrade-v13-sepolia",
  "existing": {
    "ACL_ADDRESS": "0x…",
    "FHEVM_EXECUTOR_ADDRESS": "0x…",
    "KMS_VERIFIER_ADDRESS": "0x…",
    "INPUT_VERIFIER_ADDRESS": "0x…",
    "HCU_LIMIT_ADDRESS": "0x…",
    "CLEARTEXT_ARITHMETIC_ADDRESS": "0x…",
    "CLEARTEXT_DB_ADDRESS": "0x…",
    "PAUSER_SET_ADDRESS": "0x…",
    "ACL_OWNER": "0x…"
  }
}
```

Every key also has a CLI flag (`--acl`, `--fhevm-executor`, …) so a one-off can skip the file, and so a
single address can be overridden without editing it.

### 3.1 They must be validated, because a typo is unrecoverable

This is the part that matters. The seven v13 implementations **bake these addresses into their creation
code** (§5.2), so a wrong input produces implementations whose CREATE2 addresses are wrong *and* whose
compiled-in references point at nothing. The stack would materialize cleanly and fail only in use.

So `compute` validates before it computes anything. The stack largely describes itself, and the checks
below are all it can offer:

| Supplied | Cross-checked against |
| --- | --- |
| `ACL_ADDRESS` | `executor.getACLAddress()`, `cleartextDb.getACLAddress()`, `aclOwner.acl()` |
| `FHEVM_EXECUTOR_ADDRESS` | `acl.getFHEVMExecutorAddress()`, `hcuLimit.getFHEVMExecutorAddress()` |
| `INPUT_VERIFIER_ADDRESS` | `executor.getInputVerifierAddress()` |
| `HCU_LIMIT_ADDRESS` | `executor.getHCULimitAddress()` |
| `CLEARTEXT_ARITHMETIC_ADDRESS` | `executor.getCleartextArithmeticAddress()` |
| `CLEARTEXT_DB_ADDRESS` | `arithmetic.getCleartextDBAddress()` |
| `PAUSER_SET_ADDRESS` | `acl.getPauserSetAddress()` |
| `ACL_OWNER` | `acl.owner()` — and `ACLOwner.owner()` must equal `--admin` |
| **`KMS_VERIFIER_ADDRESS`** | **nothing — see below** |

**`KMS_VERIFIER_ADDRESS` is the one address the stack cannot corroborate.** No contract in the v12 set
exposes a getter returning it: the executor reads it through a compiled-in constant, not a call. Verified
by enumerating every zero-argument address-returning getter across all fourteen ABIs.

So it gets a different kind of check, and the plan should be explicit that it is weaker:

1. code present at the address;
2. `getVersion() == "KMSVerifier v0.2.0"` — the v12 version, from the generated `LocalHostVersions`;
3. `getKmsSigners()` non-empty and `getThreshold() > 0` — it is an initialized verifier, not a bare proxy;
4. `eip712Domain()` reports name `Decryption`.

Together those establish "this is an initialized v12 KMSVerifier", which is the most that can be claimed.
It does **not** establish that it is *this stack's* KMSVerifier. The residual risk is an operator pointing
at a v12 KMSVerifier belonging to a different deployment on the same chain; the mitigation is §7's
post-upgrade signer-set comparison, which would catch it.

Every other address is pinned by at least two independent readings, and `ACL` by three.

### 3.2 `--handle` — an existing cleartext handle to prove survival

One more optional input, and the single strongest assertion the upgrade can make:

```sh
--handle 0x00987b00…7a6905          # repeatable
```

A handle already recorded in the live `CleartextDB`. `compute` reads `CleartextDB.get(handle)` and stores
the value in the manifest; `verify` reads it again afterwards and requires it identical.

Why this is the assertion that matters most: the upgrade replaces the `CleartextFHEVMExecutor` and
`CleartextArithmetic` implementations, and each of those bakes `cleartextDbAdd` into its bytecode. If the
migration patched them with anything other than the live `CleartextDB` — a placeholder marker, or the
wrong supplied address — every version string still reads correctly, `verify`'s wiring group still passes
if it only checks what was deployed, and the failure appears later as reads returning zero. A pre-upgrade
handle that still resolves to its pre-upgrade value is direct evidence that the re-pointed
implementations address the same store as before. This is the `handleBefore` check from
`test/ts/upgrade-e2e.test.ts`, applied to a real stack.

`--handle` is optional but **strongly recommended**, and its absence should be reported rather than
silently skipped. Without it `verify` can only do a fresh post-upgrade round-trip — write a value, read it
back — which proves the path works but says nothing about whether existing data survived. The two are
different guarantees and the output must not conflate them.

If the operator has no handle to hand, one can be created before the upgrade with a single
`trivialEncrypt` on the live executor; the GUIDE should say so. Note `compute` sends nothing, so creating
one is the operator's step, not the tool's.

## 4. Compute — two passes, not three

The deploy needs three builds because the ACL proxy's address feeds the shared empty implementation,
which feeds every other proxy, and that cycle has to be broken by staging (§5.3). **The upgrade needs
two**, because the ACL already exists: nothing it computes feeds anything the live stack already fixed.

| Pass | Build compiled against | Computes | Why it can |
| --- | --- | --- | --- |
| 1 | the 8 live addresses + markers for the 2 new ones | fresh shared `EmptyUUPSProxy` impl, then the `ProtocolConfig` and `KMSGeneration` proxies | the empty impl bakes only `aclAdd` (live, known); each proxy's initcode is `ERC1967Proxy(emptyImpl, initEmpty)` and references neither new address |
| 2 | the 8 live addresses + the 2 addresses pass 1 predicted | the 7 implementations | their initcode bakes the complete v13 address set, which is now fully known |

Between the passes, `compute` writes `addresses.sol` — the `fhevm-config-0.13.0/` file the payload
compiles against — containing exactly v13's ten `ADDRESS_NAMES`: the eight supplied plus the two
predicted.

Pass 2 re-derives pass 1's three addresses and **requires them unchanged**. They should not move — the
empty implementation's initcode depends only on `aclAdd` — so a difference means the rebuild changed
something it should not have, and the run must abort rather than seal a mixed set. Same argument as the
deploy's pass-3 assertion.

The sealed manifest records the supplied live addresses alongside the new ones. That is what makes the
identity check of §9 meaningful for an upgrade: a later stage re-reads it and refuses if the operator has
since changed which stack they are pointing at.

## 5. The create list — 10

| # | Create | Role |
| --- | --- | --- |
| 1 | fresh shared `EmptyUUPSProxy` implementation | `IMPL_EMPTY_UUPS_PROXY` |
| 2 | `ProtocolConfig` proxy — ERC1967 over #1 | `PROTOCOL_CONFIG_ADDRESS` |
| 3 | `KMSGeneration` proxy — ERC1967 over #1 | `KMS_GENERATION_ADDRESS` |
| 4–10 | implementations: `ProtocolConfig`, `KMSGeneration`, `ACL`, `CleartextFHEVMExecutor`, `HCULimit`, `CleartextKMSVerifier`, `CleartextArithmetic` | `IMPL_<role>` |

A fresh empty implementation rather than the live stack's: its address is not recorded in v12's address
set, so re-deriving it would be guesswork, and one more CREATE2 is cheaper than a wrong guess. It is
identical bytecode either way.

**Salts do not collide with the original deploy.** `_salt` mixes `cfg.version`, which is `"0.13"` here
against the v12 deploy's `"0.12"`, so the same role name yields a different salt. Reusing the deployment's
own `--deployment-id` is therefore correct and preferred: same stack, same id, different version
namespace.

`InputVerifier` and `CleartextDB` get no implementation: their bytecode is unchanged between the
generations, confirmed by `npm run list:upgrade-ops -- ../v12`.

## 6. Materialize — one transaction, 7 ops

Exactly what `pkg/ts/upgrade.ts` does, so the two paths cannot diverge in behaviour:

| Proxy | Implementation | Initializer |
| --- | --- | --- |
| `ProtocolConfig` | new | `initializeFromMigration(contextId, kmsNodes, thresholds)` |
| `KMSGeneration` | new | `initializeFromEmptyProxy()` |
| `ACL` | re-point | `reinitializeV4()` |
| `FHEVMExecutor` | re-point | `reinitializeV4()` |
| `HCULimit` | re-point | `reinitializeV3()` |
| `KMSVerifier` | re-point | `reinitializeV3()` |
| `CleartextArithmetic` | re-point | `reinitializeV2()` |

**The migration seed is the one thing the operator must supply that cannot be derived.** v13's
`ProtocolConfig` holds per-node metadata — tx-sender, IP, storage URL — that v12 never stored, so it is
not on chain to read. The TS path reconstructs it from package defaults when the stack was deployed with
defaults (`resolveDefaultMigration`); a testnet stack may not have been.

So: `--migration <path>` to a JSON file with `existingContextId`, `existingKmsNodes`, `existingThresholds`,
and if omitted, the same default-reconstruction the TS path uses — reading `getCurrentKmsContextId()`,
`getKmsSigners()` and `getThreshold()` off the live KMSVerifier and filling the metadata from
`LocalHostBootstrap`. The signer set read from chain must match what the defaults would produce, or the
run refuses rather than silently registering a different set. That refusal is the important half: v13
reads its KMS signers from `ProtocolConfig`, so a wrong seed *replaces* the signer set during what is
supposed to be a migration. This exact mistake was found in the TS e2e and is now asserted there.

`Create2Ordinals.t.sol` must gain the upgrade's ordinals, for the same reason the deploy's are pinned: the
op list, the implementation list and the role list are index-aligned and nothing else checks that they
agree.

## 7. Verify

Three groups. The first two mirror the deploy's; the third is new and exists because of §1's invariants.

**Versions moved as expected.** Five contracts report their v13 version, `ProtocolConfig` and
`KMSGeneration` report their initial one, and `InputVerifier` reports v0.2.0 **unchanged** — it is
deliberately absent from the op list, so a moved version there means something re-pointed it by mistake.
Read from the generated `LocalHostVersions`, never hand-written.

**Wiring.** Every address compiled into the new implementations is one actually deployed — the group that
catches a mis-addressed build, and the reason §3.1's validation matters.

**The invariants, asserted rather than assumed:**

- `ACL.owner()` is the same `ACLOwner` as before, and `ACLOwner.owner()` the same admin;
- `pendingOwner` is zero on both — no transfer even *started*;
- no `OwnershipTransferStarted` / `OwnershipTransferred` log from either contract across the whole
  upgrade;
- no `AddPauser` / `RemovePauser` / `SwapPauser` log from `PauserSet`;
- `ACL.getPauserSetAddress()` unchanged, and the `ACLOwner` still a pauser in it.

The event scans are not redundant with the value comparisons, and the reason is worth stating: `PauserSet`
exposes only `isPauser(address)` and no enumeration, so comparing values can only show that accounts
*someone thought to name* are unchanged — it cannot show nobody else was added. Log absence proves the
membership never moved, whoever is in it. The same argument applies to ownership: no event means no
transfer was even initiated. This is the design already proven in `test/ts/upgrade-e2e.test.ts`, verified
there by injecting both failures.

**Also compares the KMS signer set** before and after, from the values `compute` recorded in the manifest.
That is the mitigation for §3.1's unverifiable `KMS_VERIFIER_ADDRESS`: if the operator pointed at another
deployment's verifier, its signer set would not match what the migration seeded.

### 7.1 Everything readable must survive — the full survey

The four groups above check what someone thought to name. `test/ts/upgrade-e2e.test.ts` does better than
that, and `verify` must match it: it enumerates **every zero-argument getter** the live stack exposes —
53 of them across 9 contracts — snapshots them before the upgrade, and requires every one identical
afterwards except an explicit allow-list.

That is the check worth copying, because it is the only one that covers what nobody predicted. It already
earned its place: it caught `KMSVerifier.getKmsSigners` silently changing because the test's migration
seed disagreed with the stack it was upgrading — a bug no hand-written assertion list had noticed.

**The survey belongs in the TypeScript coordinator, not in `FhevmVerifyUpgrade.s.sol`, and that is a
capability constraint rather than a preference.** Solidity cannot enumerate an ABI; it can only call
functions someone wrote down. A Solidity survey would therefore be a hand-maintained list — exactly the
thing the e2e's approach exists to avoid. TypeScript can read the ABI JSON and iterate. So:

| Where | Checks |
| --- | --- |
| `FhevmVerifyUpgrade.s.sol` | anything needing the compiled-in address set: versions, wiring, the bootstrap values |
| `upgrade-testnet.ts` | the ABI-enumerated survey, the ownership/pauser log scans, the `--handle` value |

Both run in the `verify` stage; the coordinator runs the forge script and then does its own passes.

Details carried over from the e2e, each for a reason found the hard way:

- **Reverts are recorded as values, not skipped.** `proxiableUUID` reverts through a proxy by design
  (OpenZeppelin's `notDelegated`). Recording `<reverted>` makes "reverted before, reverts now" a survival
  and turns "worked before, reverts now" into the break it is. Skipping them would need an exclusion list.
- **The allow-list must be justified per entry, and asserted to have been used.** Only the `getVersion` of
  the five re-pointed contracts may differ, plus `HCULimit.getBlockMeter`, which returns `block.number` by
  construction. `verify` should fail if an allow-listed reading did *not* change — otherwise the list
  quietly becomes a way to ignore regressions.
- **A vacuous survey must fail.** If the ABIs cannot be read, an empty comparison passes trivially. Assert
  a minimum count.

**Which ABIs describe the live stack** is the one open question here. The survey needs the *previous*
generation's ABIs, and this is v13's tooling. Two options, and the plan does not pick:

1. `--previous-abi-dir`, defaulting to `../v12/pkg/abi` when that directory exists. Exact, but couples a
   testnet operation to a sibling checkout.
2. Use v13's own `pkg/abi`, relying on the revert-recording above to absorb getters that exist in v13 but
   not v12. Self-contained, but a getter *removed* by v13 is then invisible rather than reported.

(1) is the stronger check and (2) is the one that works from a published tarball. Whichever is chosen, the
weaker guarantee must be stated in the output rather than inferred by the reader.

## 8. Files

New Solidity, all extending `FhevmCreate2Base` so salts, initcode, CREATE2 prediction, the manifest codec
and the min-block gate are inherited rather than restated:

| File | Does |
| --- | --- |
| `script/FhevmUpgradeBase.s.sol` | the upgrade's role set, artifacts, create list, live-address loading and validation |
| `script/FhevmComputeUpgradeAddresses.s.sol` | §4's two passes; `FHEVM_PASS=1\|2`; writes `addresses.sol` and seals the manifest |
| `script/FhevmUpgradeCreates.s.sol` | §5's 10 CREATE2s, each gated on `getCode` |
| `script/FhevmMaterializeUpgrade.s.sol` | §6's single `ACLOwner.upgrade` |
| `script/FhevmVerifyUpgrade.s.sol` | §7 |

New TypeScript: `upgrade-testnet.ts` — a `Flow` descriptor, four stage functions, `main()`. Thinner than
the deploy's, except that it also owns the ABI-enumerated survey and the log scans of §7.1, which Solidity
cannot express.

Changes to existing files:

- `common.ts` — `Options` and `ConfigFile` gain the `existing` address map, the `migration` path and the
  repeatable `handle` list; `CONFIG_KEYS` gains them. Nothing else: the point of the extraction was that this file does not need to know which
  flow is running.
- `Create2Ordinals.t.sol` — the upgrade's index-aligned lists.
- `README.md`, `GUIDE.md` — §9, including the §9.1 fork walkthrough.
- `upgrade-anvil-config.json` — new, the ready-made fork/anvil rehearsal config (§9.1).
- `package.json` — no new script needed; the coordinator is run with `node`, like the deploy.

## 9. Documentation

`GUIDE.md` is currently a deploy walkthrough that never mentions upgrading. It needs:

- a section on the upgrade flow, its four stages, and why the other five are absent;
- the `existing` address block, and the fact that a wrong entry bakes into implementations — with §3.1's
  table of what is cross-checked and the explicit warning that `KMS_VERIFIER_ADDRESS` is not;
- the migration seed: what it is, why it cannot be read from a v12 stack, and when the default
  reconstruction is safe;
- a keystore-free anvil rehearsal for the upgrade, mirroring the deploy's;
- a note that the deploy's `--deployment-id` should be reused, not replaced;
- **a copy-pasteable, step-by-step fork-mode walkthrough** — see below. This is a required deliverable,
  not a nice-to-have: the fork rehearsal (§10.1) is the highest-fidelity test of the upgrade, and it is
  worth nothing if reproducing it requires reading the whole document first.

### 9.1 The fork walkthrough, specified

Numbered shell steps, no prose between them beyond one line saying what each does, and **no placeholder a
reader has to resolve** except the Sepolia RPC URL. It must run end to end by paste. Shape:

1. start `anvil --fork-url $SEPOLIA_RPC_URL --silent` in one terminal
2. (only if no v12 stack exists on the fork) deploy one with v12's `create2-deploy`, and print its nine
   addresses
3. create a cleartext handle on the live stack with one `trivialEncrypt`, and print it — this is §3.2's
   `--handle`
4. write the `existing` block into an `upgrade.fork.config.json`, given verbatim in the GUIDE
5. run `--stage compute` and read what it sealed
6. run `--stage all`
7. read the `verify` output, and what each group of it means

Steps 2 and 3 are the ones a reader will get wrong if left implicit, because they are *preconditions of
the upgrade* rather than part of it — and step 3 is the difference between proving data survived and
merely proving the stack still works. Both need the exact commands, not a description.

`anvil-config.json` gets a sibling: a ready-made `upgrade-anvil-config.json` with `confirmations: 0`,
`finality: false`, `git: false`, so the walkthrough is `--config upgrade-anvil-config.json` plus the
addresses rather than eight flags.

`README.md`'s layout table needs the five new scripts, and its "Running it" section an upgrade example.

## 10. How it gets verified

The deploy path is proven by a keystore-free anvil rehearsal. The upgrade needs a **cross-generation**
one, which is more involved and is the part most likely to surface problems:

1. v12's `create2-deploy` deploys a v12 stack on a local anvil — the "before" state;
2. read the nine addresses out of that run's manifest and hand them to v13's `upgrade-testnet.ts` **as
   arguments**, which is also what exercises §3.1's validation;
3. run all four stages;
4. `verify` must pass, including the invariants.

Worth doing a deliberate-failure pass too, given how much of today's work was only caught by real runs: a
wrong `--acl` must fail validation in `compute` rather than at materialize, and a migration seed with the
wrong signer set must be refused.

### 10.1 Forked Sepolia on anvil — the highest-fidelity rehearsal

`anvil --fork-url <sepolia>` must work, and the existing design already allows it — which is worth
recording because two decisions made for other reasons are what buy it:

- **`isAnvil` probes `anvil_nodeInfo`, not the chain id.** A fork inherits Sepolia's `11155111`, so a
  chain-id check would not recognise it as anvil and the keystore-free default would be refused. The
  method probe does.
- **The allow-list exempts anvil whatever chain id it reports.** Same reason.
- The canonical CREATE2 factory is deployed on Sepolia, so a fork has it and the §3 code-hash gate passes
  against the real thing rather than anvil's pre-deploy.

This is the best test available, for two reasons a bare anvil cannot match: it runs against **real chain
state** — real balances, real block times, a real factory — and if a v12 stack already exists on Sepolia,
the upgrade can be rehearsed against **that actual stack**, with its actual addresses passed to §3's
`existing` block. No fixture involved.

The two things a fork still needs, and neither is optional:

- `--confirmations 0 --no-finality`. A fork mines on demand like any anvil, so a reorg gate waiting for
  `head + 3` never clears.
- A fresh `--deployment-id` and `--out-dir` per rehearsal, so the salts and the seal cannot be mistaken for
  the real deployment's. §9's identity check will refuse a reused out-dir across chains, but the ids are
  the operator's to keep apart.

One caveat to document: a fork starts at the upstream head, so `blockhash(block.number - 1)` and
`block.timestamp` are real values. Nothing in this path depends on them, but `fheRand`'s seed does — so a
cleartext round-trip on a fork produces different values than on a bare anvil, and any test comparing
against fixed expected values would be wrong to. Comparing a handle's value *before and after* (§3.2) is
unaffected.

## 11. Risks

- **The two coordinators drift.** Mitigated by `common.ts` holding everything that is not a stage, and by
  `stack-order.test.ts` already asserting the deploy order agrees across all six places it is written
  down. The upgrade's op list should join that test.
- **`KMS_VERIFIER_ADDRESS` cannot be corroborated by wiring** (§3.1). Mitigated, not eliminated, by the
  signer-set comparison in §7.
- **The migration seed is operator-supplied and only partly checkable.** The signer set can be compared
  against chain; the per-node metadata cannot, because v12 never stored it. That is inherent to the
  generation change, not to this tooling — it is the same gap `updateV12ToV13` has, and the reason the
  parameter exists at all.
- **An upgrade is not idempotent the way a deploy is.** `creates` is (each create is gated on `getCode`),
  but `materialize` is not: the reinitializers are `reinitializer(n)`-guarded, so a second run reverts
  rather than no-oping. `status` must therefore report "already materialized" clearly enough that nobody
  retries into a revert and reads it as a failure.
