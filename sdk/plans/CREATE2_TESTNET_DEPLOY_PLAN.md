# CREATE2 factory deployment — public EVM testnets

Status: **proposal, revision 2** (revised after adversarial review; see §15 for what changed and why).
Supersedes `STEPPED_DEPLOY_PLAN.md` for non-local chains.

Everything in this document is decided **before the first seal**, because every decision here is baked
into the addresses. Changing any of them afterwards moves the whole stack on every chain.

## 1. Scope

**In scope:** Sepolia and other public EVM **testnets**.

**Out of scope, deliberately:**

- **Chain 31337 (anvil / Hardhat node).** RULES.md rules 15 and 17 require the local stack to land on
  the three addresses `library-solidity/config/ZamaConfig.sol` compiles into every dApp. Those are
  `CREATE(deployer, nonce)` addresses, and you cannot grind CREATE2 salts to hit three specific
  20-byte values. Local dev keeps the existing nonce path (`FhevmDeploy.sol`, `scripts/deploy.sh`)
  unchanged. **This plan adds a second path; it replaces nothing.**
- **Mainnet.** This is the *cleartext* stack: FHE is replaced by plaintext, and the KMS/coprocessor
  signer keys derive from the published `FHEVM_MNEMONIC` at documented HD paths
  (`FhevmDeployScript.s.sol`: "deliberately not configurable here"). On a testnet that is the point —
  the js-sdk relayer must hold those keys for cleartext decryption to work. On mainnet it is total
  compromise. `seal` refuses any chain id outside a testnet allow-list. **See §11 R1: that allow-list
  binds our tooling and nobody else's.**

## 2. Why CREATE2 instead of the nonce sequence

The nonce sequence exists only to satisfy rule 15. Off 31337 it is self-inflicted fragility, and it
produces one failure that cannot be recovered from:

> A transaction included in a block consumes its nonce **whether it succeeds or reverts** — measured
> on anvil, not assumed: `status 0x0`, `contractAddress null`, nonce 0 → 1. A reverted CREATE at
> nonce *n* leaves `CREATE(deployer, n)` **permanently unfillable**, while every later address stays
> correct and every later nonce check still passes. The failure is silent until something calls the
> missing contract.

`CREATE2(factory, salt, keccak(initcode))` removes the nonce from the derivation:

| Property | nonce-CREATE | CREATE2 |
| --- | --- | --- |
| Failed create | address burned forever | **retryable at the same address** |
| Foreign tx from the deployer | shifts every later address | harmless |
| Deployer | dedicated EOA at nonce 0, used for nothing else | any sender, including a multisig |
| Same addresses across chains | no | yes, if deployer + salt + initcode match |
| Resume state | a journal of nonces and tx hashes | `getCode(addr) != ""` — a chain query |

The last row is the prize: **resume needs no local state.** Most of `STEPPED_DEPLOY_PLAN.md` existed
to protect an unrecoverable sequence.

**The honest cost:** CREATE2 makes address computation *harder*, not easier — see §5.3. The nonce path
is pure arithmetic in one script; this one needs three compiler passes.

## 3. The factory

Canonical deterministic-deployment proxy at **`0x4e59b44847b379578588920cA78FbF26c0B4956C`**
(forge-std exposes it as `CREATE2_FACTORY`). Calldata is `32-byte salt ++ initcode`. Foundry routes
`new Contract{salt: s}(args)` inside `vm.startBroadcast()` through it, and
`vm.computeCreate2Address(salt, initCodeHash, factory)` predicts the address.

**Nothing may parse the factory's return data.** Foundry computes the address locally; the TS layer
uses `computeCreate2Address` + `eth_getCode`. This removes a question rather than answering it.

**Per-chain preflight, hard gate:** `keccak(getCode(0x4e59…))` must equal the runtime hash pinned in
the manifest. Pin it once by reading it off mainnet or Sepolia; do not transcribe it from memory or
from a blog post. A different contract squatting that address on some testnet is the one realistic
way a "fatal mismatch" (§8) actually fires.

If a chain lacks the factory, the fallback is the standard presigned deployment — with two conditions
this plan will not hide: the funding goes to the factory's one-time EOA
`0x3fAB184622Dc19b6109349B94811493BF2a45362`, not to our deployer; and that transaction is
**pre-EIP-155 legacy**, which some chains reject outright. On such a chain the canonical factory can
never exist at that address, and this path is unavailable. Check before adding a chain to the
allow-list.

## 4. Frontrunning — audited, and safe

Anyone may call the factory with our salt and initcode. That is harmless **iff** construction captures
nothing from the caller. Note the criterion is stronger than it looks: the factory forwards to
CREATE2, so `msg.sender` during every constructor — including the ERC-1967 constructor's `initialize`
delegatecall — is *the factory*. Any contract expecting `owner = msg.sender` would be broken in the
honest run too, not merely frontrunnable.

Audited across **every contract type this deploy creates**, not just the two empty proxies:

| Contract | Construction-time caller capture |
| --- | --- |
| `EmptyUUPSProxyACL`, `EmptyUUPSProxy` | none — constructor is `_disableInitializers()`; owner arrives as an initcode argument or not at all |
| `ERC1967Proxy` | none — initialization is constructor-atomic, so there is no uninitialized-proxy window |
| `PauserSet` | none — **no constructor at all**; `addPauser` is `onlyACLOwner` at call time |
| `ACLOwner` | none — `constructor(initialOwner, acl_)`, both baked into initcode |
| the nine implementations | none — every constructor is `_disableInitializers()` only, which also closes implementation-initialization frontrunning |

No `msg.sender`, `tx.origin`, or block-context capture anywhere.

**This property is load-bearing and must survive every upstream sync**, so the seal tooling checks it
mechanically — scan constructors and initcode-reachable initializers for `msg.sender` / `tx.origin` —
rather than relying on a prose review going stale.

## 5. Address derivation

### 5.1 The graph resolves

Apparent circularity — addresses depend on init-code hash depends on baked-in addresses — does not
close into a cycle:

```
1. EmptyUUPSProxyACL impl    initcode references no host address
2. ACL proxy                 ERC1967Proxy(impl₁, initialize(DEPLOYER))        → aclAdd
3. EmptyUUPSProxy shared     inherits ACLOwnable → bakes aclAdd only          → needs (2)
4. 8 remaining proxies       ERC1967Proxy(impl₃, initialize())                → identical initcode
5. PauserSet                 inherits ACLOwnable → bakes aclAdd
6. ACLOwner                  constructor(DEPLOYER, aclAdd) — a leaf
7. 9 implementations         bake every host address; their own addresses referenced by nothing
```

Only `aclAdd` feeds back, and it is computable from inputs alone.

**Why B and C in §6 cannot be optimised away:** it is tempting to initialize the ACL proxy directly
with the `ACLOwner` address and skip the transfer/accept pair. That is a genuine cycle —
`aclAdd` ← ACL initcode ← ACLOwner address ← ACLOwner initcode ← `aclAdd`. The two-step handover is
structural, not ceremony.

### 5.2 The ACL proxy is initialized with the DEPLOYER

Not the admin. `PauserSet.addPauser` is `onlyACLOwner`, which resolves to `Ownable2StepUpgradeable(aclAdd).owner()`
([ACLOwnable.sol](../pkg/src/contracts/shared/ACLOwnable.sol)) — so step A must be sent by whoever the
ACL proxy was initialized with. Baking `admin` there would require the admin key live mid-run, which
is impossible if the admin is a multisig. `FhevmDeployScript.s.sol:203` already does this correctly.

**Consequence for cross-chain identity:** addresses are a function of the **deployer**, not the admin.
Same deployer ⇒ same addresses on every testnet. The admin enters only at step E and affects nothing.

### 5.3 Address computation is a three-build pipeline

`EmptyUUPSProxy` and `PauserSet` bake `aclAdd` as a compiled-in immediate, and this path forbids
bytecode patching. So the init-code hashes needed to compute their addresses only exist *after* a
build against a config that already contains the real `aclAdd`. A single `forge script` cannot
recompile mid-run:

```
build 1   any addresses.sol → hash EmptyUUPSProxyACL + ERC1967Proxy
          → compute impl₁ and aclAdd; write addresses.sol with real aclAdd, placeholders elsewhere
build 2   recompile → hash EmptyUUPSProxy + PauserSet (they import only aclAdd, so placeholder
          siblings are harmless) → compute impl₃, the eight proxies, pauserSetAdd, ACLOwner
          → rewrite addresses.sol complete
build 3   compile the implementations against the final set
          → ASSERT the build-2 init-code hashes are unchanged → seal
```

The build-3 assertion is the safety net: if adding the real addresses moved any pass-2 init-code hash,
the computed addresses are wrong and the seal must fail rather than proceed.

Driven from `scripts/deploy-testnet.sh` using `vm.getCode(...)` + `vm.computeCreate2Address(...)` per
pass. **This is the largest piece of work the CREATE2 path adds over the nonce path.**

### 5.4 Salts

```
salt = keccak256(abi.encode("fhevm.cleartext", MAJOR_MINOR, deploymentId, role))
```

`role` is the address name (`ACL_ADDRESS`, …). `deploymentId` is an operator-chosen string letting a
second stack be stood up on the same chain without collision. The eight proxies share one init-code
hash and are distinguished purely by salt.

## 6. Ordering

CREATE2 removes *address* fragility, not *logical* dependencies.

**Creates — order-free except two hard edges.** Our `ERC1967Proxy` wraps OpenZeppelin's, whose
constructor calls `upgradeToAndCall` → `_setImplementation`, which reverts
`ERC1967InvalidImplementation` when the implementation has no code. So:

```
impl₁ (EmptyUUPSProxyACL)  MUST be mined before  the ACL proxy
impl₃ (EmptyUUPSProxy)     MUST be mined before  the eight remaining proxies
PauserSet, ACLOwner, the nine implementations    genuinely order-free
```

**Calls — each gated on the previous being mined and observed:**

```
A  PauserSet.addPauser(ACLOwner)        requires ACL.owner() == deployer
A′ PauserSet.addPauser(operator)        optional, same requirement — see §6.1
B  ACL.transferOwnership(ACLOwner)      requires ACL.owner() == deployer
C  ACLOwner.acceptACLOwnership()        requires ACL.pendingOwner() == ACLOwner
D  ACLOwner.upgrade(ops)                atomic: all nine proxies materialize or none
E  ACLOwner.transferOwnership(admin)    see §7
```

`ACL` is `Ownable2Step`, so ownership actually moves at **C**, not at B. A and A′ therefore need only
precede C. (`FhevmDeployScript.s.sol`'s comment claims the deployer loses `addPauser` after the
transfer — it does not; the two-step accept is what moves it. Harmless today, but worth correcting so
nobody reorders on a false model.)

### 6.1 The operator pauser

`FhevmDeployScript.s.sol:247` optionally registers a human pauser from `PAUSER_ADDRESS_0`. This plan
keeps it as optional step A′. It need not block a run: it stays reachable forever afterwards via
`ACLOwner.execute(pauserSetAdd, addPauser(...))` — by the deployer before E, by the admin after. If
configured, it gets a predicate and a terminal condition like everything else.

## 7. Ownership must be closed out

`FhevmDeployScript` constructs `new ACLOwner(deployer, aclAdd)` and **never transfers it**. The
deployer must own `ACLOwner` during the run (C and D are `onlyOwner`), so a handover is structurally
required and is missing today. `ACLOwner.execute` is an unrestricted call as `ACL.owner()` — root over
the whole stack.

Step **E** is therefore required, not optional:

- `--admin <address>` is a mandatory parameter of `seal`. No default.
- `ACLOwner` is `Ownable2Step`, so E only *offers*. The admin must send `acceptOwnership()` — a
  transaction **not from the deployer**. The runner waits for and verifies it.
- Terminal conditions before "complete": `ACL.owner() == ACLOwner`, `ACL.pendingOwner() == 0`,
  `ACLOwner.owner() == admin`, `ACLOwner.pendingOwner() == 0`, `PauserSet.isPauser(ACLOwner)`,
  plus `PauserSet.isPauser(operator)` if A′ was configured.

A dangling `pendingOwner` on either contract is a latent takeover and blocks completion.

## 8. Idempotency — predicates in Solidity, evaluated at a pinned block

Every action has a chain-state predicate. Resume is: evaluate all predicates, do what is not yet true.

**The predicates must live inside `FhevmDeployCreate2.s.sol`, not only in the TS wrapper.**
`forge script` simulates the whole run before broadcasting anything, so an ungated `new C{salt: s}`
at an occupied address fails simulation and kills the run before a single transaction exists — as
does a repeated `addPauser` (`AccountAlreadyPauser`). Every create and every call is wrapped:

```solidity
if (predicted.code.length == 0) { new C{salt: s}(args); }
```

| Action | Already-done predicate | Precondition if not done |
| --- | --- | --- |
| any create | `getCode(predicted) != ""` | — (retryable) |
| A / A′ | `PauserSet.isPauser(x)` | `ACL.owner() == deployer` |
| B | `ACL.pendingOwner() == aclOwner \|\| ACL.owner() == aclOwner` | `ACL.owner() == deployer` |
| C | `ACL.owner() == aclOwner` | `ACL.pendingOwner() == aclOwner` |
| D | every proxy's ERC-1967 slot `==` its sealed implementation | all nine slots empty — see below |
| E | `ACLOwner.owner() == admin \|\| ACLOwner.pendingOwner() == admin` | `ACLOwner.owner() == deployer` |

A failing precondition is **fatal**, not a retry: it means the stack is in a state this run did not
create.

**Step D is tri-state, and the third state matters.** The proxies' *code* never changes, so `getCode`
cannot verify materialization; verification reads the ERC-1967 implementation slot
(`0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc`).

```
all nine slots == sealed implementations   → done, skip
all nine slots empty                       → run
anything else (mixed, or a non-sealed impl) → FATAL, a human decides
```

Because `ACLOwner.upgrade` is atomic, re-running it against a partially-materialized stack hits
`onlyFromEmptyProxy` / `reinitializer` and the entire batch reverts — permanently. Mixed state is not
a resumable condition.

**Reads happen at a pinned block ≥ N confirmations behind head, from a single provider**, for the
whole resume pass. Public RPCs sit behind load balancers; a lagging node returns empty code for a
mined create, and a false "not deployed" is what triggers the simulation death above.

**A runtime-code-hash mismatch at a predicted address is fatal — but not because of an attacker.**
Different initcode yields a different address; that is what CREATE2 *is*. A mismatch can only mean the
sealed hash is wrong (build drift) or the contract at `0x4e59…` is not the canonical factory. Check
the build and the factory preflight, not the mempool.

## 9. The build seal

Required, and for a stronger reason than "audit trail": **the addresses are a function of the
init-code hash, so the build is the address set.**

- **Retrying a failed create needs the byte-exact initcode.** Without sealed artifacts, a retry is
  only possible if the build reproduces perfectly. `artifacts/` is resume-critical.
- **Resume's first act** — computing which addresses to probe — needs the sealed init-code hashes.

`foundry.toml` sets `bytecode_hash = "none"`, which drops the metadata hash but **not** the CBOR
trailer carrying the solc version. **Decide `cbor_metadata = false` now, before the first seal** —
flipping it later moves every address on every chain. The manifest pins the full solc version+commit
and the forge version, and the reproducibility claim is *demonstrated* at seal time by an independent
recompile matching every init-code hash, not asserted.

```
manifest.json   chainId, factory address + pinned runtime hash, deployer, admin, deploymentId,
                toolchain pins, per role: salt, initCodeHash, predicted address, expected runtime
                code hash; the bootstrap config actually used (signers, thresholds, gateway chain
                id, EIP-712 verifying contracts) with its provenance; the §11 R1 warning
addresses.sol   the generated fhevm-config-<version>/addresses.sol
artifacts/      the init code that will be deployed
```

Committed and **pushed** before any transaction. Post-deploy it gains tx hashes and block numbers as
an audit trail — not as resume state.

## 10. Bootstrap config is chosen, not inherited

On a testnet the published dev signer set is correct — the js-sdk relayer must hold those keys. What
is unacceptable is inheriting it *by accident*. `seal` records the full bootstrap config in the
manifest and requires explicit acknowledgement that the defaults are intended. `verify` then checks
the chain against the manifest, not against whatever the build happened to bake in.

## 11. Testnet operational notes

**R1 — The address set is replayable onto mainnet by anyone, and our allow-list cannot prevent it.**
The factory exists on mainnet; the manifest (salts + initcode) is public in git before any transaction;
the signer keys come from a published mnemonic. So anyone can deploy a bit-identical cleartext stack
at the **identical addresses** on mainnet, with keys everyone has. A dApp or document that identifies
this stack *by address alone*, pointed at the wrong chain, would function — cleartext operations
execute fine — with attacker-known keys.

This is the one place CREATE2's cross-chain address identity is adversarial rather than useful.
Required mitigations: the manifest and every published address list state that **an address proves
initcode, never chain or operator**; and the js-sdk must hard-check chain id (consistent with the
context-pinning discipline already used for `extraData`). A stronger option exists — a
`require(block.chainid != 1 && …)` inside the empty-proxy constructors, which makes the replay revert
at the same address — at the cost of diverging from upstream on every sync. **Decide explicitly and
record the decision; do not leave it implicit.**

**R2 — Reorgs happen on Sepolia.** Wait a modest depth before each dependent call (A–E), and re-verify
the whole stack at greater depth at the end. Depth is a `--confirmations` parameter, not a constant.

**R3 — Funding and gas, quantified at seal time, not at send time.** Deploying via the factory pays
initcode as calldata (16 gas per non-zero byte); nine implementations of up to ~24 KB runtime each add
materially per create. Step D is a single transaction carrying nine `upgradeToAndCall`s including
ProtocolConfig's KMS seeding — estimate it against the target's block gas limit. Also check
**EIP-3860**: initcode ≤ 49152 bytes per create; measure the largest implementation, since a 24 KB
runtime with constructor args approaches it. Faucet-funded deployers run dry; check the balance covers
everything before starting.

**R4 — Explorer verification.** Contracts deployed via the factory show `0x4e59…` as creator, which
Etherscan handles. Verify from the manifest, scripted, after E. The eight proxies need constructor-args
encoding and manual proxy linking; exact-match requires the §9 compiler settings byte-for-byte.

## 12. Key handling

The deployer key owns `ACLOwner` — root over the stack — until E completes. `deploy-testnet.sh`
supports forge keystore accounts (`--account`); a raw `--private-key` / `DEPLOYER_PRIVATE_KEY`
env var is accepted only for 31337. "Testnet" is not "throwaway": these stacks are what the js-sdk
integration story runs against.

## 13. Deliverables

- `pkg/forge/script/ComputeAddressesCreate2.s.sol` — one **pass** of the §5.3 pipeline, invoked three
  times by the driver; writes the same `addresses.sol` the nonce path writes
- `pkg/forge/script/FhevmDeployCreate2.s.sol` — idempotent deploy, every create and call gated by its
  §8 predicate **in Solidity**
- `internal/deploySeal.ts` + CLI — seal (incl. the §4 mechanical check and the independent recompile),
  verify, status
- `scripts/deploy-testnet.sh` — drives the three passes, gates on the factory preflight, refuses chain
  ids outside the testnet allow-list

The nonce path (`ComputeAddresses.s.sol`, `FhevmDeployScript.s.sol`, `deploy.sh`) is untouched and
remains the only path for 31337.

These deploy a stack from nothing. Upgrading a live v12 stack is the next piece of work — see §15.

## 14. Policy decisions (formerly open questions)

1. **Cross-chain identity: same deployer everywhere.** Addresses depend on the deployer, not the admin
   (§5.2), so one deployer EOA across all testnets yields one address set for all of them — a single
   entry in SDK config. The admin may vary per chain; a single admin is simply operationally simpler.
   Record both in the manifest.
2. **A redeploy always takes a fresh `deploymentId`.** "Redeploy at the same addresses" is a category
   error here: the proxies exist, CREATE2 reverts on collision, and the only way to change what lives
   at those addresses is `ACLOwner.upgrade` with reinitializers — which is the *upgrade* path
   (`updateV12ToV13` precedent), not a deploy. Same addresses ⇒ upgrade the standing stack. Anything
   else ⇒ new `deploymentId`, new addresses, old stack abandoned in place (harmless on a testnet).
3. **`deploymentId` stays off-chain.** The git manifest is the stronger provenance: every address is
   already a cryptographic commitment to the salt and the initcode, verifiable by recomputation. An
   on-chain copy only adds somewhere for the record to disagree with itself. At most, emit it in a
   deploy-time event for explorer convenience.

## 15. Next: the v12 → v13 update path

Everything above deploys a stack from nothing. The other thing this tooling has to do is **upgrade a
v12 stack that is already live**, which `pkg/ts/upgrade.ts:updateV12ToV13` does today in TypeScript.
Porting it to this path is the next piece of work, and it is smaller than §5.3 makes it look.

### 15.1 The circularity is gone, so the pipeline is two passes, not three

Deploying needs three builds because `aclAdd` feeds back into the bytecode that determines `aclAdd`.
Upgrading has no such problem: **the ten existing addresses are given**. They belong to a live stack
and cannot move. So the pass that exists to break the cycle simply is not needed:

```
pass 1   READ the ten live addresses off the v12 stack; write addresses.sol with them, plus
         placeholders for the two new proxies
         → build: EmptyUUPSProxy now bakes the REAL, existing aclAdd
         → compute impl₃ and the two new proxy addresses by CREATE2
pass 2   rewrite addresses.sol complete; build the seven changed implementations against it
         → ASSERT nothing pass 1 computed has moved → seal
```

The v13 implementations must be compiled against the live v12 addresses, which is exactly what
`addresses.sol` + `FOUNDRY_REMAPPINGS` already do. Note what this replaces: `updateV12ToV13` patches
bytecode (`buildHostAddressReplacementsV13`), and §5.3 forbids patching on this path. Compiling
against the real addresses is the same result reached honestly, and it is why the upgrade fits here
at all.

### 15.2 A hybrid address set, and what that means for salts

Only **two** of the addresses are CREATE2 on this path — `ProtocolConfig` and `KMSGeneration`, the two
proxies v13 adds — plus the implementations. Everything else is the v12 stack's existing
nonce-derived set, which this path neither computes nor controls.

Consequences worth deciding before the first upgrade seal:

- The two new proxies need salts, and `deploymentId` alone is not enough to identify them: they
  belong to *a particular v12 stack*, not to a fresh deployment. Salting them with something derived
  from the live `aclAdd` is the obvious answer and should be settled explicitly.
- §14.1's "same deployer ⇒ same addresses everywhere" does **not** hold here. Two chains running v12
  stacks at different addresses get different `EmptyUUPSProxy` bytecode (it bakes `aclAdd`), so the
  two new proxies land differently on each. That is correct, and it means an upgrade manifest is
  per-chain in a way a deploy manifest is not.

### 15.3 Predicates are versions, not slots

§8's step D reads the ERC-1967 implementation slot. That works for a first materialization because
the slot moves from the empty implementation to the sealed one. An upgrade re-points proxies that are
*already* materialized, so the slot alone cannot say whether the upgrade has run.

The reinitializer version is what can. `ACL` is at `REINITIALIZER_VERSION = 5` with `reinitializeV4`,
`HCULimit` at `4` with `reinitializeV3`, and OpenZeppelin's `reinitializer` refuses to run twice at
the same version — so "already upgraded" is a readable fact, and a re-run is refused by the contracts
themselves rather than by us. The predicate table needs a row per changed contract, and
`getVersion()` is the cheap human-readable cross-check.

`InputVerifier` is untouched: its v13 bytecode is identical and its version did not bump. A predicate
that expects every proxy to move would be wrong about it.

### 15.4 Preconditions the upgrade does not create

`updateV12ToV13` requires the live stack's ACL to already be owned by an `ACLOwner`, with `admin` as
that `ACLOwner`'s owner. A v12 stack owned by an EOA has no atomic upgrade path and must have one
installed first (`setupACLOwner`). On this path that is a hard precondition in the §8 sense — fatal,
not a retry — because it describes a stack this run did not create and cannot fix on the way past.

### 15.5 Testing it locally, against a fork

The deploy path can be rehearsed on a bare anvil because it builds its own world. The upgrade path
cannot: it needs a v12 stack to exist. So the rehearsal is a **fork of a chain where one is live**,
which gives the real addresses, the real ownership, and the real KMS context to migrate:

```sh
anvil --fork-url "$RPC_WITH_A_LIVE_V12_STACK" --port 8545
```

What that buys, and what it demands:

- The ten v12 addresses are read off the fork rather than configured, so the test exercises the
  read-then-compile pass rather than a hand-written config that could be wrong in the same way twice.
- `admin` must be impersonable. On a fork that is `anvil_impersonateAccount` / `--auto-impersonate`,
  which is the one place the rehearsal legitimately diverges from a real run — step E/F's whole point
  is that the admin's key is not ours.
- The fork must be pinned to a block, or the test is not reproducible. `--fork-block-number`.
- Re-running means restarting anvil: the upgrade's reinitializers are one-shot, so a second attempt
  against the same fork state is refused by the contracts, correctly.

A local v12 stack deployed from `sdk/host-contracts-cleartext/v12` and then upgraded is the weaker
alternative — cheaper to set up, but it tests the upgrade against a stack whose addresses this
tooling chose, which is the assumption most worth not making.

### 15.6 Deliverables

- `FhevmReadV12Addresses.s.sol` — pass 1's read: the ten live addresses off the v12 stack, written as
  `addresses.sol`. Where a deploy computes, an upgrade observes.
- `FhevmComputeCreate2Addresses.s.sol` — reused, with the ACL pass skipped (§15.1).
- `FhevmUpgradeV12ToV13.s.sol` — the two new proxies + the seven implementations, then one atomic
  `ACLOwner.upgrade`. §8 predicates throughout, keyed on reinitializer versions (§15.3).
- `FhevmVerifyV13.s.sol` — or a flag on `FhevmVerify`: the same terminal conditions, plus that the
  migrated KMS context matches what the v12 stack held.

## 16. Revision history

**Revision 3** — added §15, the v12 → v13 update path, as the next piece of work: two passes rather
than three (the address circularity does not arise when the addresses are given), a hybrid address
set in which only the two new proxies are CREATE2, predicates keyed on reinitializer versions rather
than ERC-1967 slots, and a fork-based local rehearsal.

**Revision 2** — after adversarial review. Changes, in order of severity:

- **The ACL proxy is initialized with the deployer, not the admin** (§5.2). Revision 1 said `admin`,
  which would have made step A unsendable — `addPauser` is `onlyACLOwner`. Cross-chain identity is
  now correctly attributed to the deployer.
- **Address computation is a three-build pipeline** (§5.3). Revision 1's single
  `ComputeAddressesCreate2.s.sol` was unimplementable: a forge script cannot recompile mid-run, and
  `EmptyUUPSProxy` bakes `aclAdd`.
- **Creates are not order-free** (§6). `ERC1967Proxy` reverts if its implementation has no code, so
  impl₁ and impl₃ must be mined before their proxies.
- **Predicates moved into Solidity, with preconditions and a tri-state rule for D** (§8). `forge
  script` simulates before broadcasting, so a TS-side check cannot prevent a collision from killing
  the run; and mixed materialization state is fatal, not resumable.
- **The fatal-branch diagnosis was wrong** (§8). A code-hash mismatch cannot mean "someone deployed
  different initcode at our salt" — different initcode gives a different address.
- **Mainnet replay added** (§11 R1) — the one genuinely adversarial consequence of cross-chain
  address identity, which the chain-id allow-list does not address.
- **Restored** the operator pauser (§6.1) and key handling (§12), both dropped in revision 1; the
  latter had regressed from `STEPPED_DEPLOY_PLAN.md`.
- **Factory preflight hardened** (§3): pinned runtime hash, return data explicitly unused, the
  pre-EIP-155 constraint on the fallback stated.
- **Reads pinned to a confirmed block from one provider** (§8); gas/funding and EIP-3860 quantified
  at seal time (§11 R3).
- **Open questions answered as policy** (§14).
- Corrected: A must precede **C**, not B — `Ownable2Step` moves ownership at accept (§6).
