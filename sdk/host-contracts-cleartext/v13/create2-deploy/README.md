# create2-deploy

**Runs on a local anvil; never yet on a real testnet.** These files started as a drawing to make
[CREATE2_TESTNET_DEPLOY_PLAN.md](../plans/CREATE2_TESTNET_DEPLOY_PLAN.md) concrete enough to argue with,
and are now further along than that: the full flow — `compute` through step F and `verify` — has been run
end to end against a local anvil in both generations, finishing with *"OK — every terminal condition
met."* See GUIDE.md; no keystore is needed for that.

What is still true: **no real testnet run has happened.** The directory sits outside `pkg/`, nothing
imports it, and it is not part of `npm run build`. Two things are wired in now — `foundry.toml` grants
it `fs_permissions` so the manifest can be written, and `test/Create2Ordinals.t.sol` pins the index
alignment its four role tables depend on.

To check it yourself:

```sh
forge build create2-deploy/script/*.s.sol --out /tmp/draftout   # Solidity
./node_modules/.bin/tsc -p create2-deploy/tsconfig.json --noEmit  # TypeScript
node create2-deploy/deploy-testnet.ts --help
```

The coordinator is TypeScript run by **plain `node`** (≥ 22.6), which strips types at load — no tsx,
no build step, no dependencies, no `jq`. `erasableSyntaxOnly` in the tsconfig enforces that: it fails
the typecheck on anything node cannot strip (`enum`, `namespace`, parameter properties), so the
editor catches it before node does.

Read it, decide whether the shape is right, then throw it away and write the real thing under
`pkg/forge/script/`.

## Layout

| File | Plan section | What it is |
| --- | --- | --- |
| [deploy-testnet.ts](deploy-testnet.ts) | §1, §3, §11 | fresh-stack coordinator: preflight gates, 3 builds, one forge invocation per stage |
| [upgrade-testnet.ts](upgrade-testnet.ts) | upgrade plan §1–§10 | upgrades a supplied live v12 stack without moving ownership or pausers |
| [UPGRADE_RUNBOOK.md](UPGRADE_RUNBOOK.md) | — | the upgrade as terminal commands, in order, with what "good" looks like at each step |
| [utils.ts](utils.ts) | — | dependency-free helpers: process running, JSONL, path containment |
| `deploy.config.json` | — | optional, auto-discovered: the stable arguments, so they aren't retyped |
| [anvil-config.json](anvil-config.json) | — | ready-made config for the local rehearsal — see GUIDE |
| [tsconfig.json](tsconfig.json) | — | editor + `tsc --noEmit` only; nothing is ever built from it |
| [script/FhevmCreate2Base.s.sol](script/FhevmCreate2Base.s.sol) | §3, §5.4, §9 | env config, role table, salts, initcode, factory call, manifest codec |
| [script/FhevmComputeCreate2Addresses.s.sol](script/FhevmComputeCreate2Addresses.s.sol) | §5.3 | one **pass** of the three-build pipeline; `FHEVM_PASS=1\|2\|3` |
| [script/FhevmDeployCreates.s.sol](script/FhevmDeployCreates.s.sol) | §6, §8 | one CREATE2 per create, each gated on `getCode(predicted) != ""` |
| [script/FhevmRegisterPausers.s.sol](script/FhevmRegisterPausers.s.sol) | §6.1, §8 | steps A, A′ — the pausers, while the deployer still owns ACL |
| [script/FhevmOfferACLOwnership.s.sol](script/FhevmOfferACLOwnership.s.sol) | §5.1, §6, §8 | step B — offers ACL to the ACLOwner; ownership does **not** move here |
| [script/FhevmAcceptACLOwnership.s.sol](script/FhevmAcceptACLOwnership.s.sol) | §6, §8 | step C — where ownership actually moves |
| [script/FhevmMaterializeStack.s.sol](script/FhevmMaterializeStack.s.sol) | §6, §8 | step D — the empty proxies become the real stack, atomically |
| [script/FhevmOfferACLOwnerToAdmin.s.sol](script/FhevmOfferACLOwnerToAdmin.s.sol) | §7, §8 | step E — the deployer offers up root |
| [script/FhevmAcceptOwnershipAsAdmin.s.sol](script/FhevmAcceptOwnershipAsAdmin.s.sol) | §7 | step F — the admin accepts. The only script **not** sent by the deployer |
| [script/FhevmStatus.s.sol](script/FhevmStatus.s.sol) | — | what's done, what's left, and why. Read-only, never reverts |
| [script/FhevmVerify.s.sol](script/FhevmVerify.s.sol) | §7, §11 R1 | the terminal conditions; reverts non-zero if any is unmet |
| [script/FhevmUpgradeBase.s.sol](script/FhevmUpgradeBase.s.sol) | upgrade plan §5–§6 | the upgrade's 10-create and 7-op role/artifact tables |
| [script/FhevmComputeUpgradeAddresses.s.sol](script/FhevmComputeUpgradeAddresses.s.sol) | upgrade plan §4 | two-pass computation over the live v12 addresses and two new proxies |
| [script/FhevmUpgradeCreates.s.sol](script/FhevmUpgradeCreates.s.sol) | upgrade plan §5 | the ten resumable factory CREATE2 calls |
| [script/FhevmUpgradeChecks.s.sol](script/FhevmUpgradeChecks.s.sol) | — | the upgrade's non-reverting checks, shared by the gate before materialize and the verify after |
| [script/FhevmPreMaterializeCheck.s.sol](script/FhevmPreMaterializeCheck.s.sol) | — | the gate before the point of no return: build, bytecode, implementation identity, pre-state, migration freshness. Read-only |
| [script/FhevmMaterializeUpgrade.s.sol](script/FhevmMaterializeUpgrade.s.sol) | upgrade plan §6 | one admin-authorized atomic `ACLOwner.upgrade`, re-asserting the gate's load-bearing subset as `require`s |
| [script/FhevmVerifyUpgrade.s.sol](script/FhevmVerifyUpgrade.s.sol) | upgrade plan §7 | version, wiring, migration and authority terminal conditions, from a fresh recompile |
| [script/Interfaces.sol](script/Interfaces.sol) | — | minimal local views so the draft needs only `forge-std` |
| [script/MaterializeInitData.sol](script/MaterializeInitData.sol) | §10 | step D's initializer payloads, from `LocalHostBootstrap` |

## Shape of a run

```
preflight        chain id in the allow-list · factory code hash pinned · deployer funded
                 ─ all three are hard gates, before any transaction exists ─

compute          build 1 → PASS=1  → addresses.sol: real ACL, markers elsewhere
                 build 2 → PASS=2  → addresses.sol: complete;  pass2.json
                 build 3 → PASS=3  → assert nothing moved · manifest.json

   ⏸  commit and PUSH the seal        ← manual. Lose it and a half-run stack is unfinishable.

creates          every CREATE2 through 0x4e59…, each `if (predicted.code.length == 0)`
pausers          A A′ — must land before C, or `addPauser` is no longer the deployer's to call
offer-acl        B    — offers only; `ACL.owner()` is still the deployer afterwards
accept-acl       C    — ownership moves here; every `onlyACLOwner` gate flips
materialize      D    — one atomic tx; tri-state, and mixed state is not resumable
offer-admin      E    — offers only; the deployer is still root when it returns
accept-admin     F    — SENT BY THE ADMIN, the only stage that isn't the deployer's

   ⏸  …if the admin is a multisig, F polls instead of sending. Ctrl-C is safe here.

verify           §7's terminal conditions, against the chain and the manifest
```

Every stage is separately runnable (`--stage`), and every stage is idempotent. Re-running **is** the
resume path — there is no journal, because the predicates are chain queries. Interrupt any stage
(Ctrl-C, dead RPC, reverted create) and the fix is to run the same command again: what landed is
skipped, what didn't is retried **at the same address**. The one unsafe moment is re-running while
the previous attempt's transactions are still in the mempool — the coordinator refuses to start when
the sender's pending nonce is ahead of its latest.

Steps A–F additionally require `FHEVM_MIN_BLOCK` and refuse to start before it. In `all` mode the
coordinator derives it per stage as *previous stage's head + `--confirmations`*, and also waits for
that previous head to **finalize**; for a single manual stage, pass `--min-block N`.

### Upgrading a v12 stack

`upgrade-testnet.ts` has a separate four-stage flow because it preserves the live ACL, executor,
verifiers, HCU limit, cleartext contracts, `PauserSet`, and `ACLOwner`:

```text
compute → creates → precheck → rehearse → materialize → verify
```

`compute` validates the nine supplied addresses, snapshots every zero-argument getter from the v12 ABIs and
every live proxy's implementation slot, records cleartext handles, and seals the normalized KMS migration.
`creates` deploys only the two new proxies, their shared empty implementation, and seven v13
implementations. `precheck` is the read-only gate before the point of no return; `materialize` runs it
first, always, then makes the one atomic seven-op call as the existing admin. `rehearse` sits between the
two in `all`: it forks the live chain onto a local anvil, applies the exact prepared calldata as the
impersonated admin, and runs the full `verify` against the fork — nothing reaches the live chain. `verify` waits for that block
to be buried and finalized, then checks versions and wiring from a fresh recompile, compares the full
snapshot and handles, and reads the upgrade's own events back. See
[How an upgrade is audited](#how-an-upgrade-is-audited).

Use `--account` for the permissionless CREATE2 transactions and `--admin-account` for
`ACLOwner.upgrade`. If the admin is a contract or multisig, `--stage materialize` writes and prints the
exact target/value/calldata payload instead. See [GUIDE.md](GUIDE.md#v12--v13-upgrade-rehearsal) for the
local cross-generation rehearsal and the `existing` config block.

### How an upgrade is audited

An upgrade is one irreversible transaction with a long approach and a long tail. The checks are layered
so that each layer answers a question the others cannot, and so that the answer never comes from the thing
being checked:

| when | what | question it answers | independent of |
| --- | --- | --- | --- |
| `compute` | nine supplied addresses cross-read off the live stack, each from a **different** contract | are these the stack's own addresses? | the operator's typing |
| `compute` | snapshot: every v12 zero-arg getter, every live implementation slot, `--handle` values, KMS signers, admin | what was true **before**? | everything that happens later — the seal is the only witness once the upgrade has run |
| `creates` | each CREATE2 gated on `getCode`, address re-derived from the build | did each create land where the seal said? | the journal |
| `precheck` | the ten creates re-derive from a **fresh recompile**; runtime code at each equals the artifact's deployed bytecode (only the UUPS `__self` immutable may differ, and must equal the address) | is the code on chain this source's output? | the build that produced the seal |
| `precheck` | `getVersion()` and the baked wiring getters called **directly on each implementation** | is `IMPL_ACL` actually an ACL, compiled against this stack? | the hand-aligned role/artifact/initializer tables — it asks the code |
| `precheck` | live implementation slots equal the sealed ones, including the two proxies the op list must not touch | did anyone upgrade under us since the seal? | — |
| `precheck` | live `KMSVerifier` signers/threshold/context id equal the sealed migration | is the migration still a carry-over, not a replacement? | the day `compute` ran |
| `precheck` | the op list, decoded, and the `keccak256` of the exact `ACLOwner.upgrade` calldata | is what the wallet shows what this build derives? | the multisig UI |
| `rehearse` | the exact `materialize-calldata.txt`, applied on an anvil **fork** of the live chain at its head with the admin impersonated, then the full `verify` below run against the fork | does this payload, on this state, leave a stack that passes? | the live chain — a wrong answer costs nothing, and the verify is the same code that runs afterwards |
| `materialize` | the load-bearing subset of the above as `require`s, evaluated in forge's simulation | — | whether the gate was run |
| `verify` | Solidity: everything `precheck` checked, plus the seven slots, versions, wiring, `ProtocolConfig` holds the pre-upgrade KMS set and context id, ownership unchanged, nothing pending | did the intended changes happen? | the deploying machine — re-runnable by anyone with source, node and manifest |
| `verify` | survey: all v12 getters unchanged except an explicit allow-list, which must **all** have changed; `--handle` values unchanged | did anything else change? | a hand-written list of what to check |
| `verify` | events: exactly one `Upgraded` per re-pointed proxy, to the sealed implementation, all in **one** transaction; none on the untouched two; seven matching `HostUpgraded`; no ownership or pauser event since the seal | was the path between the endpoints straight? | slot values, which cannot see an implementation that ran for a block and was then replaced |

Every step of every stage is announced when it starts and reported when it ends, with its duration and
the head block, and appended to `progress.jsonl`; `--stage progress` renders that ledger offline. The
full output of each invocation, forge's included, is kept under `logs/`. Together they are the run-book's
own record of the intermediate checks, which send no transaction and would otherwise leave no trace.

#### Interruptions and reorgs

Every stage is idempotent and re-running it is the resume path; what makes that safe against a reorg is
that **nothing that decides anything is read from a block that can still be orphaned**, and **nothing
that decides anything is read from a local file**:

| moment | what is read | at which block | so that |
| --- | --- | --- | --- |
| `compute` snapshot | every getter, slot, signer, handle | one **settled** block | the seal's only witness to "before" cannot be reorged away, and all readings agree with each other |
| `compute` refusal, and the `all` skip | code at the ten sealed addresses | head | a run killed between sending and journaling still refuses to reseal; the journal is a second witness, not the first |
| `precheck`, `rehearse`, `materialize` | code at the ten sealed addresses | **settled** — waits until true | the atomic call never builds on a create that could still vanish |
| `verify` | the `HostUpgraded` block | **settled** — waits until true | a verdict is never read off a block about to be orphaned; a reorged-out materialize reports "not materialized" and `materialize` is safe to re-run |
| `materialize` after a completed materialize | the seven slots | head | `precheck` says "already materialized", `rehearse` steps aside, the script returns without sending, `verify` runs — a resumed `all` finishes rather than failing |

"Settled" means the `finalized` block when the chain serves the tag, and `--confirmations` behind the head
with `--no-finality`. The in-memory "previous stage's head plus confirmations" wait is still there; it is
no longer what the safety rests on.

Two rules make the layers worth having. **Every expectation comes from the seal, the source or a different
contract** — never from the value under test, which is why `verify` recompiles into an empty `build-check`
directory rather than reading the build that sealed the manifest. And **`precheck` reports everything,
`materialize` stops at the first thing**: the gate is for the operator deciding whether to send; the
`require`s are for the case where nobody ran the gate.

What is **not** proven, and would have to be added to prove it:

- **State keyed by arguments.** The survey reaches zero-argument getters only. ACL permissions
  (`isAllowed(handle, account)`), delegations and per-account state are unchecked unless a handle-level
  probe is added; `--handle` covers `CleartextDB.get` and nothing more. A `--probe` that records arbitrary
  `cast call`s before and requires them equal after is the natural extension.
- **Behaviour after the upgrade.** `rehearse` proves the upgraded stack passes every read-only check on a
  fork; nothing sends a transaction *through* the upgraded stack, on the fork or live. Writing a handle
  through the forked executor and reading it back is the natural extension, and the fork is where to do it.
- **The rehearsal is a fork, not the chain.** anvil replays state faithfully, but block timestamps, gas
  accounting and the mempool are its own. A payload that passes on the fork can still fail live for
  reasons of gas or ordering, which is what `materialize`'s own `require`s and the post-hoc `verify` are for.
  Verified on anvil forking anvil (`npm run test:create2-e2e`); not yet on an anvil forking a public testnet.

## Checking a stage before running it

```sh
node deploy-testnet.ts … --report                  # which STEPS ran, with the tx that did each
node deploy-testnet.ts … --stage status            # what's done, what's left, and why
node deploy-testnet.ts … --stage log               # every tx, in the order it was sent
node deploy-testnet.ts … --stage creates --dry-run # is this stage ready? sends nothing
```

Four read-only views, answering four different questions:

| | question | source |
| --- | --- | --- |
| `--report` | which **steps** ran, and which transactions did them | journal + manifest |
| `--stage log` | every transaction, in the order it was sent | journal |
| `--stage status` | what is **done** and what is **blocked** right now, and why | the chain |
| `--stage progress` | which **steps** ran, including the read-only checks, when, how long, at which block (upgrade only) | progress ledger |
| `--stage params` | the init parameters the upgrade sends, decoded: sealed after `compute`, a preview resolved from the live KMSVerifier before (upgrade only) | manifest, else the chain |
| `--dry-run` | would this stage succeed if I ran it? | the script itself |

`report` and `log` read only local files — no RPC, no keystore, no foundry — which is exactly when
you want them: the RPC is down, the key has been put away, or someone asks months later what was
deployed and when.

`--dry-run` runs the **same** forge script without `--broadcast`. Forge still simulates the whole run
against a fork at the head, so every predicate and precondition executes and reverts exactly as it
would for real — a clean run means the stage is ready, and a revert names what's missing. It is not a
parallel implementation that can drift from the one that matters. Nothing is signed (`--account` is
dropped, only `--sender` is passed) and it never waits: a dry run's job is to say whether you're
ready *now*.

`status` is the wider view, and unlike `verify` it **never reverts** — it's meant to be run when
something is wrong, so it classifies instead of stopping at the first problem. Each of the 22 creates
gets one of four verdicts:

| | meaning |
| --- | --- |
| `done` | code at the sealed address — including if someone else's transaction put it there (§4) |
| `todo` | no code; the next `--stage creates` sends it |
| `DRIFT` | this build predicts a **different** address than the manifest sealed. The build moved under the seal — check the build, not the mempool (§8) |
| `NO CODE` | `vm.getCode` returned nothing: the artifact isn't in `out/` |

Then steps A–F as `done` / `ready` / `BLOCKED`, where blocked names the **specific** unmet chain
state rather than "the previous stage didn't run" — including D's tri-state, where partial
materialization is reported as FATAL because `ACLOwner.upgrade` is atomic and re-running it reverts
permanently.

## The journal — `.out/journal.jsonl`

§9: *"Post-deploy it gains tx hashes and block numbers as an audit trail — **not as resume state**."*
That distinction is load-bearing and the code says so out loud: **nothing ever reads this file to
decide anything.** Resume is `getCode(addr) != ""` and the other chain predicates. The moment a local
log becomes an input to that decision it's a second opinion that can disagree with the chain — the
exact failure the CREATE2 path exists to avoid. It is for humans, after the fact.

One JSON object per transaction, appended across stages, distilled from forge's own `run-latest.json`
(so it invents no facts — it flattens ten of them into one stream, tagged by stage):

```
  STAGE      STATUS    BLOCK     WHAT                           ADDRESS / TX
  creates    ok        6240913   ERC1967Proxy                   0xACL01…
  A/A'       REVERTED  6240930   addPauser(address)             0xPAU02…
  D          ok        6240955   upgrade((address,address,bytes 0xOWN03…
  F          ok        6241002   admin accepted ACLOwner owners -
```

Three details that matter more than the format:

- **It is written even when a stage fails.** `broadcast()` captures forge's exit code rather than
  letting `set -e` abort, records, *then* exits. A half-finished stage is exactly what the trail is
  for; aborting before recording would drop the transactions someone needs to look at.
- **Reverts are counted and warned about per stage**, not left to scroll past. A reverted create is
  not fatal here — it doesn't burn its address (§2) — but it should never be silent.
- **Step F's line is an observation, not a receipt.** When the admin is a multisig, that transaction
  comes from a key this tooling doesn't hold, so there's no local record to distil. The block at
  which the stack stopped being the deployer's is still the single most useful line in the file, so
  it's recorded with `observed: true` and no hash rather than omitted on a technicality.

Forge's raw per-run records are kept alongside, under `.out/broadcast/` (via `FOUNDRY_BROADCAST`, so
a run leaves nothing in the package root) — when a journal line isn't enough, the full record with
calldata and gas breakdown is one directory away.

Everything a run writes lives under `--out-dir` (default `create2-deploy/.out`), and it should
be **one directory per chain** — `--out-dir .out-sepolia`, `--out-dir .out-amoy`. The *addresses*
are the same on every chain for a given deployer + deploymentId (§14.1), which is the point; what's
per-chain is the `chainId` in the manifest and everything about what was actually sent. Sharing one
directory would reseal over another chain's manifest and interleave its journal — losing the only
account of what was sent to a stack that is still standing. Preflight refuses when the manifest
already there was sealed for a different chain.

| path | what | committed? |
| --- | --- | --- |
| `journal.jsonl` | the audit trail — one line per tx | after the run |
| `broadcast/…/run-latest.json` | forge's raw records | no |
| `manifest.json` | the seal — salts, init-code hashes, addresses | **before any tx** (§9) |
| `addresses.sol` | the generated config | **before any tx** (§9) |
| `pass2.json` | compute's pass-2 → pass-3 scratch | no |
| `build/` | `forge --out` | no |

A deployment spans many invocations, often days apart. Two mechanisms keep them consistent, and they
work from opposite ends:

- **A config file removes the retyping.** `--config PATH`, or `deploy.config.json` beside the script,
  holding the stable half — what this deployment *is*. **Any flag overrides it**, so a one-off
  `--rpc-url` needs no edit. It deliberately rejects `stage`, `dryRun`, `minBlock` and `yes`: those
  are what one invocation *does*, and pinning them would make every invocation the same one. Unknown
  keys are rejected too, since a typo here selects a different address set.
- **Preflight catches drift anyway**, against the manifest — because a config file is a convention,
  not a guarantee, and the seal is the record of what the first invocation actually decided. **All
  four** sealed fields are checked, because each drifts differently:

| drifted | consequence if unchecked |
| --- | --- |
| `--rpc-url` (chain) | the next `compute` reseals over another network's record |
| `--deployment-id` | new salts, old manifest: every read-only stage reports drift on all 22 |
| `--account` (deployer) | a different address set, but only *some* of the 22 move — `creates` stops at `ACL_ADDRESS` blaming "build drift" |
| `--admin` | moves no address; silently redirects **who ends up with root** |

`--stage compute` clears only the four artifacts it produces, and **refuses to run at all** once
`journal.jsonl` is non-empty. Recomputing after transactions have been sent would move the sealed
addresses out from under a partly-deployed stack — the creates stage would then report drift, or
start building a second disjoint set beside the first. §14.2's answer is a fresh `--deployment-id`.

## The five things worth looking at

**1. The three-build pipeline is the real cost of this path** — §5.3, and the largest piece of work
CREATE2 adds over the nonce path. `EmptyUUPSProxy` and `PauserSet` bake `aclAdd` as a compiled-in
immediate, so their init-code hashes — and therefore their addresses — only exist after a build
against a config that already holds the real `aclAdd`. A `forge script` cannot recompile mid-run,
which is why the coordinator is a shell script and why one Solidity file runs three times against
three different `out/` directories. Pass 3's assertion is the safety net: if adding the real
addresses moved a hash pass 2 computed, everything downstream is wrong and the seal fails.

**2. The deployer is not in the salt, and that is correct.** The canonical factory passes the salt to
CREATE2 verbatim — it does not namespace by caller — so `msg.sender` enters the derivation nowhere.
The deployer reaches the address set through exactly one channel: it is an initcode argument to the
ACL proxy's `initialize` (§5.2), and everything downstream bakes the resulting `aclAdd`. Same
deployer ⇒ same addresses on every testnet; a different deployer ⇒ a disjoint set.

**3. Predicate and precondition are different columns.** A predicate that is already true is the
normal resume case, silent. A precondition that is false is fatal — it means the stack is in a state
this run did not create, and no retry fixes that. Step D is tri-state on top: every ERC-1967 slot
sealed → skip, every one empty → run, **anything else → a human decides**, because `ACLOwner.upgrade`
is atomic and re-running it against a partial stack reverts permanently.

**3b. The reorg gate is a required env var, not a `sleep`.** Steps A–E each read `FHEVM_MIN_BLOCK`
and refuse to start until the chain has reached it. Every one of them decides what to do by reading
state a *previous* step wrote — and those predicates decide whether a step gets **skipped**, so a
read from a block that is about to be orphaned is not a stale display value, it's a step silently
not happening. A reorged-away `addPauser` that the predicate reported as done is a stack that reaches
§7's terminal conditions with no pauser.

The coordinator waits *and* passes the number; the script refuses independently. That's the same
argument as every other gate here — a `sleep` in this shell binds this shell, not §13's TS driver or
an operator running one `--stage` by hand. `0` is a legitimate value (the first stage of a run passes
it) but there is **no default**, so skipping the wait is a decision someone made rather than a
variable someone forgot.

That it's a simulation-time check is not a weakness worth hedging about, because the two risks aren't
independent: for a transaction to land on a chain where its predicate is false, that chain must have
reorged out a block already `--confirmations` deep at read time, and one deeper by inclusion. The
inclusion-side risk is strictly dominated. **Depth is what matters, not when it's evaluated.**

**What the depth is worth is the real question**, and depth is only a proxy:

| | blocks | ≈ time at 12s slots |
| --- | --- | --- |
| `--confirmations 15` | 15 | ~3 min |
| PoS finality (2 epochs) | 64 | ~12.8 min |

A depth heuristic gets about a quarter of the way to finality. On mainnet that's usually academic;
on the **testnets this path is restricted to** it isn't — Holesky went weeks without finalizing in
February 2024. So the coordinator waits for the `finalized` tag *as well as* the depth, probes for
tag support once in preflight, and degrades loudly rather than hanging. `--no-finality` opts out.

The split is forced: a Solidity script cannot read `finalized` through `block.number`, so the
portable half (depth) is the gate, and the stronger half (finality) is the wait.

**4. The gates are in Solidity because forge simulates before it broadcasts.** An ungated CREATE2 at
an occupied address, or a repeated `addPauser`, dies in simulation — and the run ends before a single
transaction exists. A shell-side check cannot prevent that.

**4b. Ordering lives in preconditions, never in control flow — so the scripts can be small.** A
`forge script` is not a transaction: each step is its own, any run can stop between two of them, and
every stage is separately invocable. Putting six calls in one `run()` therefore guarantees nothing
about their order that putting them in six files doesn't. Once that's true, splitting is free, and
independently invocable scripts are what an orchestrator can sequence — the shell today, §13's
`internal/deploySeal.ts` tomorrow, an operator running one `--stage` by hand in between.

Every ordering constraint in the path now terminates at a precondition on chain state:

| step | needs | enforced by | new gate? |
| --- | --- | --- | --- |
| A/A′ | `ACL.owner() == deployer` | its own precondition | — |
| B | `ACL.owner() == deployer` | its own precondition | — |
| C | A ran | `PauserSet.isPauser(aclOwner)` | **yes** |
| C | B ran | `ACL.pendingOwner() == aclOwner` | no, §8 |
| C | E not yet accepted | `ACLOwner.owner() == deployer` | **yes** |
| D | C ran | `ACL.owner() == aclOwner` | no, §8 |
| E | — | `ACLOwner.owner() == deployer` | no, §8 |

Only two gates had to be invented, and both cover states nothing else forbade:

- **C's pauser check.** Skip the pausers stage, run everything to completion, and the stack looks
  finished, works for every normal operation, and has **no reachable emergency stop** —
  `ACLOwner.pause()` needs the ACLOwner registered, and a contract cannot register itself after the
  fact. It was always required; keeping A/A′ in the same file as C only made it easy not to notice.
- **C's `ACLOwner.owner()` check.** Reachable only by running stages out of order, but without it
  the failure surfaces as a raw `OwnableUnauthorizedAccount` from inside simulation.

A/A′ and B are unordered with respect to each other, and deliberately so — `transferOwnership` does
not touch `owner()`, so B leaves `addPauser` exactly as callable as it found it.

**The one gap left is E.** §8 gives it no precondition on D, so the ACLOwner can be offered to the
admin with every proxy still empty. Not unsafe — D is `onlyOwner` on the ACLOwner, so an admin
who accepts early can run D themselves — but an out-of-order run can hand over an unmaterialized
stack, and the hand-over is the point after which fixing anything costs a multisig round-trip.
`verify` catches it either way.

The draft **follows §8 and warns rather than refuses** ([`_warnIfNotMaterialized`](script/FhevmOfferACLOwnerToAdmin.s.sol)).
Promoting that to a `require` is a plan decision, not a draft one: unlike C's pauser gate it would
forbid an ordering that is arguably legitimate — offer early so the admin's multisig can schedule its
acceptance while D runs.

The question to ask of each further split, then, is not "does the orchestrator run these in order"
but "which existing precondition already forbids the wrong order, and if none does, what silent state
does that permit."

**5. Two pauses are manual on purpose.** Pushing the seal to a shared remote is the operator's call.
The admin's `acceptOwnership()` is a transaction from a key this tooling does not hold — and until it
lands, the deployer is still root over the stack.

## What is stubbed, and what would have to be true

- ~~**`MaterializeInitData`** is a stub~~ — **done.** Wired to `LocalHostBootstrap` with
  `abi.encodeCall`, the same source and the same encoding as `FhevmDeployScript._materialize`, so
  this path and the TypeScript `deploy()` produce the same stack. It is no longer standalone: it now
  imports the implementations, which is what buys the type-checked arguments.
  §10 is still open — `seal` should record the bootstrap config in the manifest, so what a
  deployment used is a fact on record rather than whatever the build baked in.
- ~~**`FACTORY_CODEHASH`** is unpinned~~ — **done.** Pinned to
  `0x2fa86add0aed31f33a762c9d88e807c475bd51d0f52bd0955754b2608f7e4989`, read off mainnet and Sepolia
  (which agree), and the same bytes anvil pre-deploys locally. It stays a *constant* rather than
  something computed, because deriving the expected value from the chain under test compares a value
  against itself and can never fail — the same argument the nonce path makes in `scripts/deploy.sh`.
- **The gas/funding estimate** in `preflight` prints a balance and checks nothing. §11 R3 wants it
  quantified against a fork — including EIP-3860 headroom, which pass 3 does check per role.
- **The manifest is address-only.** `_serializeRoleMap` writes the `address` map; the `salt` and
  `initCodeHash` maps are elided to keep one function readable. §9 needs all three, plus toolchain
  pins, the observed factory hash, the bootstrap config with its provenance, and the §11 R1 warning.
- **`cbor_metadata`** is not addressed anywhere here, and it is a **decide-before-the-first-seal**
  item (§9). `foundry.toml` already sets `bytecode_hash = "none"`, which drops the metadata hash but
  not the CBOR trailer carrying the solc version. Flipping it later moves every address on every
  chain.
- **The §4 mechanical constructor scan** (no `msg.sender` / `tx.origin` reachable from initcode) and
  the **independent recompile** at seal time are not here. Both belong in `internal/deploySeal.ts` —
  they are the TS half of the deliverables, and the plan puts them there.

## Three things this draft found that the plan does not mention

**`CREATE2_FACTORY` cannot be declared in a `Script`.** forge-std already declares it as
`internal constant` on `CommonBase`, which `Script` inherits, so a local copy is an
"Identifier already declared" error. The base inherits it instead. Worth being explicit that
inheriting the *address* is not §3's preflight: the plan's gate is on the factory's **runtime code
hash**, pinned per chain, because the realistic failure is a different contract squatting that
address on some testnet — which no constant, ours or forge-std's, can detect.

**`vm.getCode("ERC1967Proxy.sol:ERC1967Proxy")` is ambiguous in this project.** OpenZeppelin ships a
contract of the same name in a file of the same name, and forge aborts on multiple matching
artifacts. Every artifact id in `FhevmCreate2Base` is therefore path-qualified. The nonce path never
hits this because it uses `new ERC1967Proxy(...)` and lets solc resolve the import — so this is a
cost that appears only once you build initcode from artifacts instead of from types.

**`fs_permissions` in `foundry.toml` is scoped to `./internal/.deploy-config`**, deliberately, so no
other script can reach the tree. This path writes `addresses.sol`, `pass2.json` and `manifest.json`,
and needs its own entry — **nothing here can write anything until it gets one**:

```toml
fs_permissions = [
    { access = "read-write", path = "./internal/.deploy-config" },   # nonce path
    { access = "read-write", path = "./create2-deploy" },      # this path
]
```

That list is also what bounds `--out-dir`. Forge does accept absolute entries, including outside the
project root — verified, not assumed — but it is static config, so granting one per deployment
doesn't scale. The shell therefore grants a single root, resolves relative values against it, and
**rejects an out dir outside it at startup**; otherwise forge notices only midway through pass 1,
after two builds, complaining about a path the operator never typed. Giving the CREATE2 path its own
root rather than sharing `./internal/.deploy-config` also keeps either path from clobbering the
other's config.

## Running it

A real testnet, which has not been done yet:

```sh
node create2-deploy/deploy-testnet.ts \
  --rpc-url        "$SEPOLIA_RPC_URL" \
  --account        fhevm-testnet-deployer \
  --admin          0x… \
  --deployment-id  sepolia-2026-08 \
  --confirmations  3
```

A local anvil rehearsal, which has — see GUIDE.md. `--account` and `--admin` are omitted there, which
is allowed only because the node answers `anvil_nodeInfo`:

```sh
anvil --silent &
node create2-deploy/deploy-testnet.ts --config create2-deploy/anvil-config.json \
  --out-dir .out-rehearsal --no-confirm --stage all
```

**This is not the path for local dev.** RULES.md rules 15 and 17 require the local stack to land on the
three `CREATE(deployer, nonce)` addresses `ZamaConfig.sol` compiles into every dApp, and you cannot
grind CREATE2 salts to hit three specific 20-byte values. `scripts/deploy.sh` is untouched and remains
the only path that satisfies those rules. An anvil rehearsal here is for exercising the CREATE2 flow, not
for standing up a stack a dApp can use.
