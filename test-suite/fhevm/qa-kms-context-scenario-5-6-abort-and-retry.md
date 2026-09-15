# QA KMS context — scenarios 5 and 6: aborting a switch, and retrying it

Implementation report for the fifth **and sixth** scenarios of the `kms-context-qa-tests` profile,
implemented together as the single `context-switch-abort-and-retry` case.

> **Two scenarios were proposed; one case was built.** §0 explains why, and which clauses came from
> which. Both were also altered from their proposed form — §3.3 and §4 record what changed and why.

**Status:** delivered and green against a live stack, first run.
**Scope:** the cross-layer and live-cluster behaviour of aborting a context switch held at **stage
1** — before its creation quorum — and of the retry that follows it.

> **Read §1 first.** The contract-level claims are already proven twice, in `host-contracts`. This
> case exists for what those tests cannot reach, and the scenarios were rewritten accordingly. Taking
> them at face value would credit this case with coverage it did not add.

---

## 0. Why two scenarios became one case

The two were proposed separately:

```gherkin
Scenario A: Governance aborts a context switch that does not reach quorum
Scenario B: Governance retries a switch after cancelling a context pre-registered in the Gateway
```

B's `Given` block is, clause for clause, the state A **ends in**:

| Scenario B `Given` | Where scenario A establishes it |
|---|---|
| the active pair is `C1, E1` | never moved — asserted twice, before and after the destroy |
| the switch to `C2` was aborted by `destroyKmsContext` | the `When` of A |
| `C2` remains registered in the Gateway | the Gateway canary, `valid=true` |
| ProtocolConfig still returns `C1, E1` | `assertPairUnchanged` after the destroy |
| the status indicates no pending transition | the in-flight gate, reopened |

And B's `When`/`Then` — request a new switch, register it on the Gateway, watch `C3, E3` activate —
were already A's recovery step, which existed to prove the gate had genuinely reopened rather than
merely reporting itself open.

Implementing B separately would have meant a second case spending six minutes rebuilding, from a
clean stack, precisely the state A leaves behind — to then run three steps A already runs. The seam
between them is the on-chain state, not setup code, so merging costs nothing and duplicates nothing.

**What the merge changed:** B asks for confirmations completing *"with compatible results"*.
Activation alone does not say that — it proves every signer voted, not that the work behind the votes
agreed. So the recovery step gained a per-node `new_kms_epoch.status = completed` check against the
connector DB. That assertion exists **because** B was merged in; A did not need it.

The case id reflects both: `context-switch-abort-and-retry`.

---

## 1. What was already covered, before this case existed

| Layer | Test | Stage | What it asserts |
|---|---|---|---|
| Foundry | `host-contracts/test/protocolConfig/protocolConfig.t.sol` — `test_destroyPendingContextClearsPairedEpoch` | 2 (epoch allocated) | `KmsContextDestroyed` emitted; `isValidKmsContext` false; `isValidEpochForContext` false; active pair unchanged |
| Hardhat task | `host-contracts/test/tasks/kmsContext.ts:230` — *"broadcasts the destruction of a non-current PENDING context"* | 1 (no epoch) | `aborted == true`; `abortReason == "context-destroyed"` |

Between them they cover every `Then` in the proposed scenario, **including** *"the status must
indicate that the switch was aborted"* — which is not only assertable but already asserted, through
`task:kmsContextSwitchStatus`, an event-indexing monitor that reports `aborted` and `abortReason`
(`host-contracts/tasks/kmsContext.ts:405`).

The contract itself also anticipates this exact path:

> *A switch destroyed before its creation quorum has no epoch yet, so there is nothing to clear and
> the ownership check below is false.* — `ProtocolConfig.sol:503-504`

So re-asserting those claims in an e2e suite would add nothing but a third copy and six minutes of
runtime.

## 2. What no existing test can reach

1. **Withholding real confirmations.** The unit tests obtain a Pending context because *no* node ever
   confirms — the nodes are fabricated addresses. Here a real five-party committee is running and one
   node's tx-sender is stopped on purpose, so the quorum is *withheld* rather than absent. That is
   the scenario's `Given`, actually established.
2. **The Gateway divergence.** The scenario requires `C2` to be pre-registered in the Gateway. The
   Gateway keeps its own registry and its own owner-gated `destroyKmsContext`
   (`gateway-contracts/contracts/GatewayConfig.sol:327`), entirely independent of the host's. So
   after the abort the Gateway **still holds a context the host destroyed and will never activate**.
   No unit test spans both chains; nothing has ever observed this.
3. **That the abort does not disturb service.** `C1, E1` must still complete a real user decryption
   afterwards.
4. **That the gate really reopened**, end to end — a fresh switch activating on a live cluster, not
   merely an `eth_call` being accepted.

## 3. The scenario: as proposed, and as implemented

### 3.1 As originally proposed

```gherkin
Scenario: Governance aborts a context switch that does not reach quorum
  Given the active pair is "C1, E1"
  And governance has requested a switch to "C2, E2"
  And context "C2" was pre-registered in the Gateway
  And the confirmations required to create "C2" have been withheld
  And the switch status is "PENDING"
  And ProtocolConfig still returns "C1, E1" as the active pair
  When governance executes destroyKmsContext for "C2"
  Then the status must indicate that the switch was aborted
  And ProtocolConfig must continue returning "C1, E1" as the active pair
```

### 3.2 As implemented

```gherkin
Scenario: Governance aborts a context switch that does not reach quorum
  Given ProtocolConfig reports an active pair "C1, E1"
  And every provisioned party has a running tx-sender
  And the committee still reaches the user-decryption threshold with one member stalled
  When one node of the new context has its tx-sender stopped BEFORE the broadcast
  And governance requests a switch, which stores context "C2" as Pending
  And context "C2" is pre-registered in the Gateway
  Then ProtocolConfig must still return "C1, E1"
  And a second lifecycle operation must revert "KmsLifecycleOperationInFlight"
  And "isValidKmsContext(C2)" must be false
  And NO NewKmsEpoch must have been emitted for "C2"
  When governance executes destroyKmsContext for "C2"
  Then the receipt must carry "KmsContextDestroyed(C2)"
  And ProtocolConfig must still return "C1, E1"
  And a new lifecycle operation must be allowed again
  And "isValidKmsContext(C2)" must be false
  And the GATEWAY must still report "C2" as valid
  When the stalled tx-sender is restarted
  Then a user decryption under "C1, E1" must succeed
  And a fresh context switch must activate
```

### 3.3 Why it changed

**"A switch to C2, E2" became a switch to C2 only.** At stage 1 there is no `E2`:
`_createPendingEpoch` runs inside `confirmKmsContextCreation`, only once the creation quorum holds
(`ProtocolConfig.sol:383-388`). The case proves this rather than assuming it, by requiring that no
`NewKmsEpoch` was emitted for `C2` — which is also the observation that distinguishes stage 1 from
stage 2 and therefore confirms the confirmations really were withheld.

**"The status must indicate that the switch was aborted" became "the gate reopens".** The status task
exists but has no compose service in this CLI, so the case asserts the observable meaning instead:
before the destroy a second lifecycle operation reverts `KmsLifecycleOperationInFlight`; after it,
the same `eth_call` succeeds. That is exactly what the contract documents — *"Destroyed entries are
None and do not count, so the destroy paths reopen the gate"* (`ProtocolConfig.sol:1083`). It is the
precise inverse of the reading taken a few steps earlier, by the same helper's counterpart.

**Two `Then` clauses were added** for §2's items 3 and 4, and **one observation** for item 2 — the
Gateway divergence, asserted as a canary rather than merely logged: if cross-chain cleanup is ever
added, this fails and forces a conscious decision instead of silently changing meaning.

**Two preconditions were added**, as in every case that stalls a node: no *other* node may already be
down, and the remaining committee must still reach the user-decryption threshold.

## 4. Stage 1, and the sibling relationship

`context-switch-pending` (scenario 2) deliberately targets **stage 2** — context Created, epoch
Pending — because its scenario names a pending `E2`, and it documents stage 1 as the rejected
alternative. This case *is* that alternative, and the difference is one line:

| | scenario 2 | scenario 5 |
|---|---|---|
| tx-sender stopped | **after** the creation quorum forms | **before** the broadcast |
| what is held | the activation confirmation | the creation confirmations |
| `C2` state | Created | Pending |
| `E2` | exists, Pending | **does not exist** |
| settled by | restoring the node → activation | `destroyKmsContext` → abort |

Together they cover both halves of a switch's lifecycle stall, and both directions out of it.

## 5. What the case does

1. Read the baseline pair and the live committee.
2. `assertTxSendersRunning` over every provisioned party; pick the stall party; check the decryption
   quorum survives it.
3. Input-proof smoke at baseline; read the block height.
4. **Inside `withTxSendersStopped([stalled])`** — the stop comes first, which is what makes this
   stage 1:
   1. broadcast the switch; pre-register `C2` on the Gateway;
   2. four `Given` readings: pair unchanged, gate closed, `C2` not valid, **no `NewKmsEpoch`**;
   3. `destroyKmsContext(C2)`, and check the receipt's `KmsContextDestroyed` names `C2`;
   4. four `Then` readings: pair unchanged, **gate reopened**, `C2` still not valid, and the
      **Gateway still reports `C2` valid**.
5. Leaving the scope restarts the tx-sender.
6. A real user decryption under `C1, E1` must succeed.
7. A fresh switch must activate — the gate reopening, proven rather than claimed.

## 6. How to run

```bash
cd test-suite/fhevm
KMS_QA_CASES=context-switch-abort-and-retry ./fhevm-cli test kms-context-qa-tests
```

No `down`/`up` is required first — see the disruptiveness note below.

**Disruptive but NOT single-run.** It destroys a context and, in its recovery step, advances the
context and epoch — but it assumes nothing about where it starts. The baseline pair and the committee
are read from the chain, and the ids are predicted by sequential allocation, so any starting state
works. Its two real preconditions are:

1. **every tx-sender running** — it withholds one deliberately, and a second one already down would
   leave the recovery switch unactivatable. The case checks this itself (`assertTxSendersRunning`)
   and fails in seconds with a message naming the stopped containers;
2. **no lifecycle operation in flight** — otherwise the broadcast reverts
   `KmsLifecycleOperationInFlight`.

Both are exactly the state the case leaves behind: it restarts the tx-sender it stopped, and its
recovery switch activates, so nothing is in flight. It is therefore re-runnable **back to back
against its own output**, with no `down`/`up` in between.

Verified rather than assumed: a second run was started on the stack the first one left at
`ctx#3 / epoch#2`, aborted `ctx#4` and recovered to `ctx#5 / epoch#3` — `PASS (404s)`, no re-up.

## 7. Verification

**Static.** `bun run check` clean; `bun test src` → 477 pass, 0 fail.

**Live.** `PASS (395s)`, 37 evidence steps, on a freshly re-upped stack (`ctx#1 / epoch#1`).

```
14  ok    169ms node    stop tx-sender(s) for withhold party 4's creation confirmation
16  ok       -  note    re-read active pair after broadcasting …: contextId=ctx#1 epochId=epoch#1
17  ok     26ms assert  a second lifecycle operation reverts KmsLifecycleOperationInFlight
19  ok       -  note    isValidKmsContext  ctx#2  valid=false expected=false
20  ok     18ms assert  no epoch was allocated for the pending context
                        (the creation quorum never formed)
21  ok    783ms tx      destroyKmsContext(ctx#2)
22  ok       -  note    receipt  block=830 gasUsed=73348 status=0x1 logCount=1
23  ok      0ms assert  the destroy names the pending context
25  ok       -  note    re-read active pair after destroying …: contextId=ctx#1 epochId=epoch#1
26  ok     23ms assert  a new lifecycle operation is allowed again (the in-flight gate reopened)
28  ok       -  note    isValidKmsContext  ctx#2  valid=false expected=false
30  ok       -  note    gateway isValidKmsContext  ctx#2  valid=true  expected=true
32  ok   119.1s probe   user-decryption under the original pair after the abort
37  ok    15.2s wait    recovery context switch to ctx#3 after the abort   (polls=4)
39  ok    917ms assert  every node of the recovered context completed the reshare
                        parties=1,2,3,4  -> completed, completed, completed, completed
```

Four results are worth keeping.

**Step 20 is the one that makes this stage 1.** No `NewKmsEpoch` was ever emitted for `ctx#2`, so the
creation quorum genuinely never formed — the withheld confirmation did what the scenario's `Given`
claims, rather than that being an assumption about container state.

**Steps 17 and 26 are the same call, inverted.** Before the destroy a second lifecycle operation
reverts `KmsLifecycleOperationInFlight`; 23ms of assertions later, after it, the identical `eth_call`
succeeds. That pair *is* "the status indicates the switch was aborted", expressed in the only terms
the chain offers.

**Step 30 is the measurement this case was built for.** `ctx#2` is destroyed on the host (step 28,
`false`) and simultaneously valid on the Gateway (`true`). The divergence had been reasoned about
from the source; this is the first time both registries were read with both chains live.

**Steps 37 and 39 close the loop, and are scenario B.** A fresh switch activated in 15.2s over 4
polls — the gate did not merely report itself open, a real switch went through it — and every node of
the recovered context reported its reshare `completed`, which is B's *"with compatible results"*
stated outright rather than inferred from the pointer having moved.

The reshare assertion was added by the merge and passed on its first live run (917ms, four nodes
`completed`), on a stack that started at `ctx#7 / epoch#4`, aborted `ctx#8` and recovered to
`ctx#9 / epoch#5` — the third consecutive run with no `down`/`up`.

The whole run is 395s, of which 119s is the decryption probe and ~93s the two compose tasks of the
recovery switch; every assertion itself runs in milliseconds.

## 8. Related files

| Path | Role |
|---|---|
| `src/kms-qa/cases/case-context-switch-abort.ts` | the case, covering both scenarios |
| `src/kms-qa/pending.ts` | `assertNoNewKmsEpochEvent`, `assertLifecycleGateOpen`, `assertGatewayContextValidity` |
| `qa-kms-context-scenario-2-pending-context.md` | the stage-2 sibling |
| `host-contracts/test/protocolConfig/protocolConfig.t.sol` | the Foundry coverage this case does not duplicate |
| `host-contracts/test/tasks/kmsContext.ts` | the status-task coverage of `aborted`/`abortReason` |
| `host-contracts/contracts/ProtocolConfig.sol` | `destroyKmsContext`, and the gate the destroy reopens |
| `gateway-contracts/contracts/GatewayConfig.sol` | the Gateway's independent registry and destroy |
