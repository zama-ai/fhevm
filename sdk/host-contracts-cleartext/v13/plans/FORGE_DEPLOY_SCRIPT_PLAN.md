# Plan — a Forge script that deploys a cleartext stack

## Goal

Deploy a complete cleartext v13 stack from Foundry alone, given a **mnemonic**, an **RPC URL** and a
**chain**, with no Node in the loop.

## The approach: compile the real addresses in, do not patch bytecode

The TS path (`pkg/ts/deploy.ts`) ships prebuilt templates and rewrites the host addresses into the
bytecode at recorded offsets. **This script does not do that.** Instead it uses the
`fhevm-config-<version>/` injection point for its actual purpose:

1. **Precompute** every host address from the deployer and its start nonce — the addresses are
   `CREATE`-derived, and the deploy order is fixed and known.
2. **Generate** an `addresses.sol` containing those real addresses.
3. **`forge build`** against that config, so solc bakes the real addresses in directly.
4. **`forge script --broadcast`** to deploy the freshly built artifacts, in exactly the order the
   precompute assumed.

Why this is better here: no placeholders, no offset bookkeeping, no post-compile rewriting, and nothing
to prove equivalent — the deployed bytecode *is* what solc emitted. The template technique exists
because npm consumers need prebuilt artifacts; a Foundry user can compile, so they should.

**The cost, stated plainly:** the bytecode becomes deployment-specific. One build serves exactly one
(deployer, start nonce) pair, so artifacts cannot be shipped or reused, and a `forge test` that wants a
stack must generate its config first. That is the trade the template path avoids.

## Hard constraints

1. **Real deployments only — no `vm.etch` / `setCode`.** Every host address is `CREATE`-derived from
   `deployer + nonce`; etched code produces a chain no real deploy can reproduce.
2. **Materialize all proxies in a single transaction, through `ACLOwner`.** `ACLOwner.upgrade(Op[])`
   loops `upgradeToAndCall` over every proxy and reverts as a whole. Never upgrade proxy-by-proxy.
3. **The nonce run is exclusive.** Nothing else may broadcast from the deployer between the precompute
   and the last deploy, or every address is wrong. Assert, do not assume (phase C).
4. **Stock node.** Every contract fits EIP-170 (RULES.md rule 12) — no `--code-size-limit`.
5. **Never overwrite `internal/placeholders/addresses.sol`.** The generated config goes to its own
   directory and the remapping is repointed for the build. That file is the marker set for the *template*
   path and must stay untouched.

## The address table

Verified empirically: `cast compute-address --nonce N <deployer>` against a real deploy from mnemonic
index 5. With `startNonce = n`:

| Nonce | Contract               | Nonce  | Contract                   |
| ----- | ---------------------- | ------ | -------------------------- |
| n+0   | `EmptyUUPSProxyACL`    | n+7    | `ProtocolConfig` proxy     |
| n+1   | `ACL` proxy            | n+8    | `KMSGeneration` proxy      |
| n+2   | shared `EmptyUUPSProxy`| n+9    | `CleartextArithmetic` proxy|
| n+3   | `FHEVMExecutor` proxy  | n+10   | `CleartextDB` proxy        |
| n+4   | `KMSVerifier` proxy    | n+11   | `PauserSet`                |
| n+5   | `InputVerifier` proxy  | n+12   | `ACLOwner`                 |
| n+6   | `HCULimit` proxy       | n+13.. | the nine implementations   |

Only n+0…n+10 and n+11 appear in the generated config; `ACLOwner` and the implementations are not host
addresses baked into other contracts. This ordering is the contract between the generator and the
script — if they disagree, the deploy produces a stack whose contracts point at the wrong peers.

## Phases

### Phase A — precompute and generate (shell + `cast`, no compilation)

Deliberately shell-only: it needs no compiled artifacts, so there is no chicken-and-egg with a config
that does not exist yet.

```bash
DEPLOYER=$(cast wallet address --mnemonic "$MNEMONIC" --mnemonic-index "$INDEX")
START_NONCE=$(cast nonce "$DEPLOYER" --rpc-url "$RPC_URL")
# then, per row of the table:
cast compute-address --nonce $((START_NONCE + k)) "$DEPLOYER"
```

Write `test/.deploy-config/addresses.sol` (gitignored) with the ten `*_ADDRESS` constants. Addresses
from `cast` are EIP-55 checksummed, so solc accepts them as-is.

Record `DEPLOYER` and `START_NONCE` in a sidecar JSON — phase C must assert against the same values, not
re-read them.

### Phase B — build against the generated config

```bash
# repoint only for this build; restore afterwards
forge build --out test/.deploy-config/out   # with remappings pointing fhevm-config-<v>/ at the generated dir
```

Use the same mechanism as `test/templates.test.ts`'s high-entropy test: temporarily rewrite
`remappings.txt`, restore in a trap. `FOUNDRY_REMAPPINGS` does **not** work — verified: `remappings.txt`
wins for the same key.

### Phase C — deploy (`forge script --broadcast`)

```bash
forge script script/DeployCleartext.s.sol \
  --rpc-url "$RPC_URL" --chain "$CHAIN_ID" --broadcast \
  --sig 'run(string,uint32)' "$MNEMONIC" "$INDEX"
```

Inside the script:

1. `pk = vm.deriveKey(mnemonic, index)`; `deployer = vm.addr(pk)`; assert
   `vm.getNonce(deployer) == START_NONCE` from phase A — this is what catches an interleaved transaction.
2. Assert `addr.code.length == 0` for every predicted address (port `assertNoCodeAtTargets`).
3. `vm.startBroadcast(pk)` and deploy in exactly the table order, using plain `new C(...)` — the
   addresses are already compiled in, so nothing is patched. After each, assert the resulting address
   equals `vm.computeCreateAddress(deployer, START_NONCE + k)`.
4. `PauserSet`, then `ACLOwner(acl, admin)`; `ACL.transferOwnership(aclOwner)` from the deployer and
   `aclOwner.acceptACLOwnership()` from the admin — `Ownable2Step` needs both halves before `upgrade`
   will work.
5. Deploy the nine implementations (permissionless plain `CREATE`s).
6. One `aclOwner.upgrade(ops)` from the admin, `ops[i] = { proxy, implementation, initData }` in the same
   order as `toACLOwnerOps`. Then assert every proxy's ERC-1967 slot
   (`0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc`) holds the implementation just
   deployed.
7. `console2.log` the eleven addresses in the order `test/ts/utils/deployStack.ts` prints them, so the
   two paths diff by eye.

The bootstrap config (KMS nodes, thresholds, HCU limits, verifier sources) should be a struct argument or
read from JSON — not hardcoded.

### Orchestration

One wrapper — `scripts/forge-deploy.sh` — runs A, B, C with a trap that restores `remappings.txt`
whatever happens. Modelled on `scripts/anvil.sh`.

## Update-only: upgrading an existing stack

A second script, `script/UpdateCleartext.s.sol`, for the case where the proxies already exist and only
the implementations change — a rule 6 sync bringing new upstream contract code, or a cleartext fix.

**Simpler in one way, harder in another.** Simpler: the host addresses are already fixed on chain, so
nothing is predicted from a nonce — the plan's top risk, three copies of the nonce table drifting, does
not apply, and the addresses become an *input to be verified* rather than an output to be derived.
Harder: it is not purely a deployment operation. Which proxies can be upgraded is decided by the
release — a contract only carries a bumped `REINITIALIZER_VERSION` if its bytecode changed (see
`initData` discipline) — so the op list comes from reading the vendored contracts, not from a choice the
script makes.

**The stack is self-describing**, which is how the script discovers what it is updating (verified against
a live deploy):

| Query | Returns |
| --- | --- |
| `ACL.owner()` | the standing `ACLOwner` |
| `ACLOwner.owner()` | the admin allowed to call `upgrade` |
| `ACL.getFHEVMExecutorAddress()` | the executor proxy |
| `ACL.getPauserSetAddress()` | the `PauserSet` |
| `FHEVMExecutor.getACLAddress()` / `.getInputVerifierAddress()` | the ACL and input verifier |

Only the ACL address needs to be supplied; most of the rest is reachable from it. Anything not reachable
by a getter must come from a deployment JSON and then be cross-checked against whatever getter does
exist — never trusted blind.

### Phases

- **A′ — collect.** Given the ACL address and an RPC, read the live host addresses via the table above.
  Generate `addresses.sol` from the values *read from chain*, not from a nonce computation.
- **B′ — build.** Identical to phase B: repoint the remapping at the generated config, `forge build`
  to a dedicated `--out`, restore. The new implementations now bake in the live addresses.
- **C′ — upgrade.** Deploy only the implementations, then a single `ACLOwner.upgrade(ops)` from the
  admin. No proxy is created; no nonce ordering is assumed.

### Pre-flight gates

Each of these is a way to brick a live stack, so all are hard failures before any transaction:

1. **Proxies must already exist** — `addr.code.length > 0` for every host address. This is the exact
   inverse of the deploy script's `assertNoCodeAtTargets`, and confusing the two is the most likely
   copy-paste error between the scripts.
2. **Authority** — `ACL.owner() == aclOwner` and `aclOwner.owner() == msg.sender`. Fail early rather
   than reverting inside `upgrade`.
3. **Implementation self-check** — after deploying each implementation and *before* wiring it in, call
   its own address getters (`getACLAddress()`, `getInputVerifierAddress()`, …) directly on the
   implementation. They return compiled-in constants, so they answer without proxy storage. If any
   disagrees with the live address, the generated config was wrong and upgrading would point the stack at
   nothing. This is the single most valuable gate in the script.
4. **Storage layout compatibility** — diff `forge inspect <C> storageLayout` between the deployed and new
   implementation for every proxy. `ACLOwner.upgrade` is atomic against *reverts*, not against a
   successful upgrade to an incompatible layout.

### `initData` discipline

`ACLOwner.upgrade` calls `proxy.upgradeToAndCall(implementation, initData)`, so every op carries calldata
that runs *inside the proxy's storage* right after the implementation changes.

**In this codebase every op carries a call — `initData` is never empty.** Two shapes exist:

| Shape | Used when | Example from `pkg/ts` |
| --- | --- | --- |
| `initializeFromEmptyProxy(<args>)` | an empty proxy becomes real for the first time | the deploy path passes the bootstrap config per contract (`config.acl`, `config.kmsVerifier`, …) |
| `reinitializeV<n>()` — no args | a live proxy receives a new implementation | v12→v13 uses `reinitializeV4` (ACL, FHEVMExecutor), `reinitializeV3` (HCULimit, KMSVerifier), `reinitializeV2` |

The second row is the one that matters for an in-generation update, and note what it implies: even a
**stateless** implementation change gets a reinitializer call — `pkg/ts/upgrade.ts` describes
`reinitializeV2` as a "stateless bump". The purpose is not to write state but to advance OpenZeppelin's
initializer version, recording which implementation generation the proxy is on.

So the discipline for the update script is:

1. **Follow the convention: bump rather than pass empty `initData`.** Empty is *mechanically* legal —
   `upgradeToAndCall(impl, "")` just repoints the proxy — but every existing upgrade path here bumps, so
   an empty op silently opts out of the replay guard and the on-chain generation marker. The
   reinitializers are empty-bodied (`function reinitializeV4() … reinitializer(REINITIALIZER_VERSION) {}`),
   which is the tell: the call exists for bookkeeping, not initialization.
2. **A contract only gets an op if its bytecode changed.** `REINITIALIZER_VERSION` is a compile-time
   constant; a release that leaves a contract functionally untouched leaves the constant alone and the
   contract out of the op list. `InputVerifier` is the worked example — cosmetic-only changes between v12
   and v13, no bump, deliberately absent from `updateV12ToV13`. So "no fresh reinitializer" is not
   automatically a blocker; it may simply mean there is nothing to upgrade.
3. **Require it explicitly per op**, and assert the version is exactly one greater than the deployed
   implementation's. A wrong version is a revert inside the atomic `upgrade`, so it is safe but wasteful;
   a *missing* reinitializer is the silent case, and the one to guard.

Consequence worth stating plainly: by this codebase's convention an in-generation update is not a pure
bytecode swap — the contracts whose bytecode changed are expected to arrive from upstream with a bumped
`REINITIALIZER_VERSION`, so what the script can do is bounded by what the release contains. It is a
*convention* rather than a mechanical requirement: an empty-`initData` swap would work, it just forgoes
the guard.

### Validation

1. Deploy a stack with `scripts/anvil.sh`, then run the update against implementations that differ only
   by a new `reinitializeV<n+1>`. Assert every proxy's ERC-1967 slot points at the newly deployed
   implementation and that the initializer version advanced by exactly one. Note a *source-identical*
   update cannot be tested: without a fresh reinitializer the upgrade reverts on OpenZeppelin's guard,
   which is the design working, not a bug.
2. Re-run `anvil.sh`'s `verify_zama_config_addresses` wiring probes after the upgrade: addresses and
   `getACLAddress()` / `getInputVerifierAddress()` must be unchanged.
3. Corrupt one implementation's generated config, and assert gate 3 blocks the upgrade before any
   transaction is sent.

### Not covered: cross-generation migration (and why it is a separate problem)

Two things get called "update", and they are not variants of each other:

- **In-generation update** — what this script does. A v13 stack becomes a *newer* v13 stack. The set of
  contracts is unchanged; only their code changes. No proxy is created and no address moves, and every op
  is a reinitialization (`reinitializeV<n+1>`).
- **Cross-generation migration** — v12 → v13, implemented as `updateV12ToV13` in `pkg/ts/upgrade.ts`.
  The protocol itself gains contracts: `FhevmAddressesV13` is `FhevmAddressesV12` **plus**
  `protocolConfigAddress` and `kmsGenerationAddress`.

That difference inverts three of this script's assumptions:

1. **New addresses appear.** Those two v13-only contracts need fresh proxies, so nonce prediction — the
   thing the update path was free of — comes back, along with its ordering risk.
2. **The pre-flight gate flips.** This script requires code to *already exist* at every address; a
   migration requires code to be *absent* at the two new ones. Running one script's gate against the
   other's situation fails on every address.
3. **The `initData` shapes mix.** An in-generation update is reinitializations only. The real migration
   performs, in a single atomic `ACLOwner.upgrade`, **2 materializations plus 4 reinitializations** — so
   it must build both shapes from the `initData` table, and materializations additionally need the
   migration config (existing context id, signer set, thresholds) read off the live v12 `KMSVerifier`.

So a script that handled both would need every gate and every `initData` rule to branch on which mode it
is in — which is the definition of two scripts sharing a file. Keep them separate; if a Solidity version
of the migration is ever wanted, it gets its own plan.

## Validation

1. **Parity with the TS path.** Two fresh anvils, same mnemonic and start nonce: TS via
   `scripts/anvil.sh`, Solidity via this script. All eleven addresses **and** every deployed runtime
   bytecode must match byte-for-byte. This is the acceptance test, and it is also the strongest possible
   check on the template technique — two independent routes agreeing.
2. **Nonce-race refusal.** Send an unrelated transaction from the deployer between phases A and C; assert
   the phase C nonce check fires rather than deploying a wrong stack.
3. **Atomicity.** Corrupt one `Op`'s `initData` and assert the whole `upgrade` reverts with no proxy
   materialized.
4. **No placeholder anywhere.** The generated config contains only real addresses, so scan the deployed
   code for the ten markers in `internal/placeholders/addresses.sol` and expect zero hits — proof this
   path never touches the marker mechanism.
5. **`internal/placeholders/addresses.sol` unchanged** after a full run, and `remappings.txt` restored.

## Risks

- **The table is duplicated knowledge** between phase A's generator, phase C's assertions and
  `pkg/ts/addresses.ts`. Three copies will drift. Generate it from one source, or add a test that fails
  when they disagree — this is the single most likely defect.
- **`ACLOwner` at n+12 is not in the config**, so nothing baked-in depends on it. Confirm that before
  relying on it: if any contract ever hardcodes the ACL owner, the table gains a constraint.
- **Deployment-specific artifacts.** `test/.deploy-config/out` must never be confused with the committed
  `out/`, and must not be published. Keep it under `test/` and gitignored.
- **Phase A trusts a live nonce read.** On a chain with pending transactions from the deployer,
  `cast nonce` may lag; prefer the pending nonce and still assert in phase C.