# Stepped, ledger-backed deployment

Status: **proposal**. Nothing here is implemented yet.

## 1. Why

`scripts/deploy.sh` runs the whole host deployment as three commands, the last of which
(`FhevmDeployScript.s.sol` under `--broadcast`) sends ~26 transactions in one `forge script`
invocation. It works, and every address is checked as it lands. But it has one property that is
unacceptable for a real network:

> **If it is interrupted, there is no way to resume, and no durable record of what happened.**

The reason is structural. Every host address is `CREATE(deployer, nonce)`, and the implementation
bytecode is *compiled against* that address set before it is deployed (step 1 computes the addresses,
step 2 compiles them in). So the deployer's nonce at step 1 is load-bearing for the entire stack. An
interruption advances the nonce; re-running recomputes a different address set and deploys a second,
unrelated stack; re-running only step 3 fails on the first address assertion.

Worse, the *build itself* lives only in `internal/.deploy-config/out`, which `npm run clean` deletes.
Lose it and you cannot verify what was deployed, or finish deploying it.

Goal: a deployment that is **resumable**, **auditable**, and **verifiable by a third party**, where
every single transaction is recorded durably before and after it is sent, and confirmed to a stated
depth before the next one begins.

## 2. What "catastrophe" means here

Ordered by severity, worst first. The design is judged against these.

| # | Failure | Consequence |
| - | ------- | ----------- |
| C1 | A transaction in the address-critical prefix is lost, reverted, or replaced | Every subsequent address shifts while the compiled bytecode keeps pointing at the old ones. The stack is unrecoverable at these addresses; on 31337 every `ZamaConfig`-compiled dApp breaks. |
| C2 | The sealed build is lost | Cannot verify what was deployed, cannot resume, cannot reproduce. |
| C3 | The runner cannot tell "not sent" from "sent, outcome unknown" | Re-sending burns the nonce → C1. Not re-sending stalls forever. |
| C4 | A reorg drops an already-"confirmed" transaction | Silent divergence between the ledger and the chain. |
| C5 | Ownership ends up somewhere unintended | `ACLOwner` is the upgrade root for the whole stack. Wrong owner = permanent loss of upgrade authority, or an attacker holding it. |
| C6 | The deployer key is exposed | Total compromise during the window in which it owns ACL. |

## 3. Two phases

### Phase 0 — seal the build. No transaction until this is committed.

`foundry.toml` sets **`bytecode_hash = "none"`**, so no metadata hash is embedded and creation
bytecode is a pure function of (sources, solc 0.8.24, optimizer 800 runs, evm cancun, via_ir off).
That is what makes the seal *verifiable by recompilation* rather than merely trusted.

```
<journal>/<chainId>-<deployer>-<startNonce>/
  manifest.json      toolchain, deployer, chainId, startNonce, the address set,
                     per-contract creation-code keccak256, the full step table
  addresses.sol      the exact generated config from ComputeAddresses.s.sol
  artifacts/         the creation bytecode that will actually be deployed
  steps/             empty at seal time
```

Sealed with a git commit (and a tag) in the journal repo. `seal` refuses to run if the journal
worktree is dirty.

### Phase 1..N — one transaction per step, each recorded before and after.

## 4. The step table

26 transactions, in nonce order. Steps 0–11 are **address-critical**: their CREATE address is
compiled into other contracts' bytecode. Steps 12–25 are not — nothing references those addresses.

```
 idx  nonce  kind    what                              critical
  0   +0     create  EmptyUUPSProxyACL impl               yes
  1   +1     create  ERC1967Proxy (ACL)                   yes
  2   +2     create  EmptyUUPSProxy shared impl           yes
  3   +3     create  ERC1967Proxy (FHEVMExecutor)         yes
  4   +4     create  ERC1967Proxy (KMSVerifier)           yes
  5   +5     create  ERC1967Proxy (InputVerifier)         yes
  6   +6     create  ERC1967Proxy (HCULimit)              yes
  7   +7     create  ERC1967Proxy (ProtocolConfig)        yes
  8   +8     create  ERC1967Proxy (KMSGeneration)         yes
  9   +9     create  ERC1967Proxy (CleartextArithmetic)   yes
 10   +10    create  ERC1967Proxy (CleartextDB)           yes
 11   +11    create  PauserSet                            yes
 12   +12    create  ACLOwner                             no
 13   +13    call    PauserSet.addPauser(ACLOwner)        no
 14   +14    call    ACL.transferOwnership(ACLOwner)      no
 15   +15    call    ACLOwner.acceptACLOwnership()        no
 16..24      create  9 × implementation                   no
 25   +25    call    ACLOwner.upgrade(ops)                no
```

Notes that constrain the design:

- Under `--broadcast` a plain *call* consumes a nonce exactly as a CREATE does, so steps 13–15 are
  part of the same unbroken sequence.
- Step 25 is **atomic by construction**: it materializes all nine proxies in one transaction, so
  there is no half-materialized state to resume into.
- Ownership moves at steps 14–15. Before 14 the deployer owns ACL; after 15 the `ACLOwner` does.
  Step 13 must precede 14 because `PauserSet.addPauser` is `onlyACLOwner`.

## 5. Per-step protocol

```
1. PREFLIGHT
   a. chainId still matches the manifest
   b. on-chain nonce == this step's expected nonce      (refuse otherwise)
   c. create steps: getCode(expectedAddress) is empty
   d. SIMULATE: eth_call / estimateGas against pending state
2. write steps/NN-<name>.json  status=pending           (nonce, expectedAddr, payloadKeccak)
3. SEND
4. write txHash to disk IMMEDIATELY, before any waiting   <-- the critical window
5. await receipt; require status == 1
6. await N confirmations
7. VERIFY: contractAddress == expectedAddress
           keccak(getCode(addr)) == manifest's deployed-code hash
8. write status=confirmed, git commit
```

**Step 1d exists because a reverted transaction still consumes a nonce.** In the address-critical
prefix that is C1. Simulating first turns a would-be revert into a refusal that costs nothing.

**Step 4 is the answer to C3.** The dangerous window is between "broadcast accepted" and "hash
persisted"; writing before any waiting shrinks it to one fsync. Even if it were lost, the step is
recoverable in substance: the address is deterministic, so `getCode(expectedAddress)` proves whether
the CREATE landed and whether its code is right — the hash is convenience, not proof.

## 6. Resume

`run --resume` reads the journal and, for every step, re-derives truth from the chain rather than
trusting the file:

```
for each step:
  confirmed in ledger?  -> re-verify on chain (code hash + depth). mismatch => STOP.
  pending/sent?         -> if nonce advanced past it: find the outcome, reconcile
                           if nonce still at it:      re-simulate and send
  absent?               -> first unexecuted step; continue from here
```

Then assert the on-chain nonce equals the first unexecuted step's expected nonce before sending
anything. If it does not, the sequence has been contaminated by a foreign transaction and the run
aborts: inside the critical prefix that is unrecoverable (see §8).

## 7. `verify` — third-party, no local state

Takes only the journal and an RPC URL. Re-checks every recorded step: the transaction exists, is at
least N blocks deep, succeeded, was sent by the recorded deployer at the recorded nonce, and the code
now at each address hashes to what `manifest.json` sealed. Also re-checks the ownership end state
(§9). Intended to be runnable months later by someone who did not perform the deploy.

## 8. Unrecoverable cases, stated honestly

- **Foreign transaction from the deployer inside steps 0–11.** Every later address moves. There is no
  fix; the address set must be abandoned. On a dev chain, reset. On a real chain, redeploy from a
  fresh EOA at nonce 0 and seal a new build. The runner detects it and refuses to continue.
- **Reorg deeper than N.** Same as above if it lands in the critical prefix. N is the knob.

Mitigation is prevention: a **dedicated deployer EOA at nonce 0, used for nothing else, ever.**

## 9. Production concerns this plan must also answer

Listed because they are what separates a working script from a safe one.

1. **Key handling.** `deploy.sh` currently takes `--private-key` on the command line (shell history,
   `ps`, CI logs). Production must support a keystore or a hardware wallet. Note foundry already uses
   `--ledger` for a Ledger device, so this document calls the record directory the **journal** to
   avoid the collision.
2. **Pre-funding.** Check the deployer's balance covers all 26 transactions at the chosen fee cap
   before step 0. Running out mid-sequence stalls at the worst possible moment.
3. **Stuck transactions.** If a send is mined too slowly, the next step must NOT be sent. Recovery is
   fee-bump *at the same nonce* (replacement), never a new nonce. The runner must make this the only
   available action.
4. **Fee policy.** Explicit EIP-1559 `maxFeePerGas` / `maxPriorityFeePerGas` per step, recorded in the
   journal. No "whatever the node suggests" for a deploy this consequential.
5. **Ownership end state.** After step 25, assert: `ACL.owner() == ACLOwner`, `ACLOwner.owner() ==`
   the intended admin, `PauserSet.isPauser(ACLOwner)`. For production the `ACLOwner` owner should be
   a multisig or timelock, not an EOA — `ACLOwner.execute` is an unrestricted call as `ACL.owner()`.
6. **Separation of duties.** Sealing the build and running the deploy can be different people; the
   journal commit is the handoff artifact.
7. **Journal integrity.** Signed commits and a tag on the seal, so the record cannot be rewritten
   after the fact.
8. **Source verification.** Publish sources to the relevant explorer after step 25.

## 10. Deliverables

- `internal/steppedDeploy.ts` — step table, execution engine, resume/reconcile logic
- `internal/deployJournal.ts` — journal read/write, git commit, verification helpers
- `internal/cli/steppedDeploy.ts` — `seal` | `run` | `resume` | `verify` | `status`
- npm scripts; `scripts/deploy.sh` step 3 optionally delegates to the runner

Bytecode comes from the forge real-address build (`internal/.deploy-config/out`), preserving
`deploy.sh`'s invariant that nothing is ever patched. Written in TypeScript under `internal/` because
it needs JSON state, receipt polling and git — it cannot live in `pkg/`, whose `ts/` is library-free
by RULES.md rule 8.

Confirmations default by chain: 15 on a real network, 0 on 31337, where an auto-mining anvil only
advances blocks when a transaction is sent and 15 would hang at step 0. Overridable.
