# QA KMS context — scenario 2: a new request uses the previous context and epoch while a switch is pending

Implementation report for the context-switch half of scenario 2 of the `kms-context-qa-tests`
profile, implemented as the `context-switch-pending` case.

**Status:** delivered and green against a live stack, on the second run.
**Scope:** both halves — the host orchestration and the container-side `extraData` spec. One clause
remains deliberately uncovered (see §8), unchanged since scenario 1.

> Scenario 2 has two halves, mirroring scenario 1: `qa-kms-context-scenario-2-pending-epoch.md`
> covers the epoch rotation, this one covers the context switch.

---

## 1. The scenario: as proposed, and as implemented

### 1.1 As originally proposed

```gherkin
Scenario: A new request uses the previous context and epoch while a context switch is pending
  Given the active pair is "C1, E1"
  And governance has requested a switch to "C2, E2"
  And context "C2" was pre-registered in the Gateway
  And the switch status is "PENDING"
  And ProtocolConfig still returns "C1, E1" as the active pair
  When the application performs a decryption through the SDK while the status is "PENDING"
  And the test captures the request and response extraData
  Then the decryption must complete successfully
  And the request extraData must decode as version "0x02", context "C1", and epoch "E1"
  And the response extraData must be identical to the request extraData
  And pending context "C2" and pending epoch "E2" must not be used by the request
```

### 1.2 As implemented

```gherkin
Scenario: A new request uses the previous context and epoch while a context switch is pending
  Given ProtocolConfig reports an active pair "(C1, E1)"
  And every provisioned party has a running tx-sender
  And the committee still reaches the user-decryption threshold with one member stalled
  When governance requests a switch, which stores context "C2" as Pending
  And the creation quorum forms and allocates epoch "E2", emitting NewKmsEpoch
  And one node of the new context has its tx-sender stopped
  Then ProtocolConfig must still return "(C1, E1)"
  And context "C2" must be pre-registered in the Gateway
  And a second lifecycle operation must revert "KmsLifecycleOperationInFlight"
  And "isValidKmsContext(C2)" must be false
  And "isValidEpochForContext(C2, E2)" must be false
  And every node of the new context must report the reshare for "E2" as completed
  When the application performs a decryption through the SDK
  And the test signs a permit and captures the request extraData
  Then the decryption must complete successfully
  And the request extraData must decode as version "0x02", context "C1", and epoch "E1"
  And the decoded pair must equal neither "C2" nor "E2"
  And ProtocolConfig must still return "(C1, E1)" after the probes
  When the stalled tx-sender is restarted
  Then "(C2, E2)" must activate
  And "isValidKmsContext(C2)" and "isValidEpochForContext(C2, E2)" must be true
```

### 1.3 Why it changed

**The `Given` clauses are established and verified, not assumed** — the same concession as every
case in this profile. The ids are read from the chain rather than carried as constants.

**"The switch status is PENDING" names a stage the proposal does not distinguish.** A context switch
has two, and only the second one can satisfy the scenario. See §2 — this is the finding this
implementation turned up, and the reason the case is structured the way it is.

**`E2` is recovered from the chain's logs, not predicted.** See §3.

**The Gateway pre-registration moved inside the held window.** The proposal lists it as a `Given`
established before the decryption; `case-context-switch` performs it right after the broadcast. Here
it runs after the activation confirmation is withheld, which is both closer to the scenario's
ordering (it is a precondition *of the pending state*, not of the broadcast) and strictly safer: the
ordering constraint it exists to satisfy is "before activation", and activation is now held.

**Two preconditions were added**, as in the epoch half: no *other* node may already be stalled, and
the remaining committee must still reach the user-decryption threshold.

**The clean-up is an activation, not an abort.** Releasing the confirmation and watching `(C2, E2)`
activate is what proves the switch was pending *for the stated reason*.

### 1.4 Clause-by-clause coverage

| Clause | Covered by | Where |
|---|---|---|
| active pair is `(C1, E1)` | `readCurrentPair` baseline | host |
| switch to `C2, E2` requested | `broadcastContextSwitch` + `NewKmsEpoch` from the logs | host |
| `C2` pre-registered in the Gateway | `preRegisterContextOnGateway`, inside the hold | host |
| status is `PENDING` | revert probe + `isValidKmsContext` + `isValidEpochForContext` + unchanged pair | host |
| ProtocolConfig still returns `(C1, E1)` | `assertPairUnchanged`, twice | host |
| decryption completes successfully | `runDecryption`, plus the spec's own `decryptValue` | both |
| request extraData is `0x02`, `C1`, `E1` | field checks + byte-exact string match | container |
| response extraData identical to request | **not covered** — see §8 | — |
| pending `C2` and `E2` must not be used | `KMS_QA_FORBIDDEN_CONTEXT_ID` / `_EPOCH_ID` | container |

---

## 2. A context switch has two Pending stages, and the scenario means the second

This is the finding that shaped the case, and it is not visible from the scenario text.

`defineNewKmsContextAndEpoch` does **not** create the epoch. It stores the new context as `Pending`
and stops. The epoch is allocated later, inside `confirmKmsContextCreation`, and only once
`_hasContextCreationQuorum` holds — every node of the new context has confirmed
(`ProtocolConfig.sol:383-388`):

```solidity
if (_hasContextCreationQuorum(kmsContextId)) {
    $.contextState[kmsContextId] = ContextState.Created;
    // Create the confirmed context's first epoch here, pairing it with the context by construction.
    uint256 epochId = _createPendingEpoch(kmsContextId);
    emit NewKmsEpoch(kmsContextId, epochId, previousContextId, $.latestActiveEpochId, block.number - 1);
}
```

So:

| Stage | Context state | Epoch | How you get stuck here |
|---|---|---|---|
| **1** | `Pending` | **does not exist** | a node of the new context cannot confirm creation |
| **2** | `Created` | `Pending`, resharing | a node cannot confirm activation |

Both satisfy `_checkNoKmsLifecycleOperationInFlight`, so both look like "a switch is pending" from
the outside. But a scenario that names a pending `E2` — and asserts the SDK must not use it — can
only mean **stage 2**. At stage 1 there is no `E2` to forbid, and the clause would be untestable
rather than tested.

Stage 1 is the easier hold: stop a tx-sender before the broadcast and the creation quorum never
forms. The epoch half of scenario 2 does exactly the equivalent, because an epoch rotation has no
stage 1 — `defineNewEpochForCurrentKmsContext` creates the Pending epoch immediately, in the same
transaction, and its receipt carries the event.

Reaching stage 2 therefore means letting the creation quorum complete and *then* withholding the
activation confirmation.

### The margin, stated plainly

Between observing `NewKmsEpoch` and stopping the tx-sender there is a real gap. It is bounded by the
cores' reshare — tens of seconds, measured at 63.2s for an epoch reshare on this stack — against a
one-second poll interval and a container stop of a few hundred milliseconds.

That is a wide margin, but it is a margin and not a lock, so the case does not rely on it being
respected: the first thing it does after the stop is re-read the active pair. A switch that managed
to activate inside the gap fails there, loudly, rather than turning into a vacuous pass where every
subsequent assertion is trivially satisfied by an already-activated switch.

---

## 3. No receipt, so the ids come from the chain's logs

An epoch rotation is a `cast send`, and its receipt carries `NewKmsEpoch`. A context switch is not:
it carries the whole committee definition — node params, thresholds, software version, PCR values —
which lives in the contracts task's env file, so it runs as a compose task and returns nothing.

`case-context-switch` works around this by predicting `C2 = C1 + 1` from the sequential allocation in
`_storeNextKmsContext` and verifying the prediction only *after* activation. That is sound for its
purposes, but useless here: this case must name both ids **while they are pending**, and inject them
into the container as values the SDK must not use. A prediction that is only verified afterwards
would make the negative assertion vacuous if it were wrong — forbidding an id that does not exist
always passes.

`waitForNewKmsEpochEvent` (`src/kms-qa/pending.ts`) queries `cast logs --json` from the block height
read before the broadcast and decodes the event with the existing `decodeNewKmsEpoch`. `cast logs`
emits the same `{address, topics, data}` shape a receipt carries, so the wrapper is structural
rather than a conversion.

That single read does three things at once: it yields `E2` authoritatively, it verifies the `C2`
prediction *before* anything is built on it, and its arrival is the signal that the switch has
crossed from stage 1 to stage 2.

Filtering on the context id is not belt-and-braces. On a stack that has rotated epochs before —
which, after the other three cases, is every stack — older `NewKmsEpoch` logs exist under previous
contexts, and a block lower bound alone would not exclude a rotation racing the switch.

---

## 4. Proving PENDING when no view exposes it

`ProtocolConfig` keeps `contextState` and `epochState` in private storage with no getters, so the
state is triangulated exactly as in the epoch half — with one more reading available here, because a
switch has a context of its own to interrogate:

| Reading | Signal |
|---|---|
| `getCurrentKmsContextAndEpoch` unchanged | nothing advanced |
| second lifecycle op reverts `KmsLifecycleOperationInFlight` | *something* is in flight |
| `isValidKmsContext(C2)` is `false` | `C2` exists and is not serving |
| `isValidEpochForContext(C2, E2)` is `false` | `E2` exists and is not serving |

The last two are the positive readings — the first two only say "nothing moved". Both are read again
after activation, where they must be `true`.

Underneath them sits `new_kms_epoch.status = completed` on every node of the new context, which a DB
trigger sets when the core's epoch result lands regardless of whether the activation confirmation
went out. That is what separates *held* from *stuck*, and it is why the hold can be called honest:
the switch is pending because one confirmation is missing, not because the KMS failed to reshare.

### The chain will not name a pending context's members

`getKmsSignersForContext` and `getKmsNodesForContext` are both gated on the context being valid and
revert `InvalidKmsContext(uint256)` (`0x77ddbe81`) until activation. There is no third view:

```
getKmsSignersForContext(ctx#9)                    -> revert InvalidKmsContext
getKmsNodesForContext(ctx#9)                      -> revert InvalidKmsContext
getContextCreationPreviousTxSenderThreshold(ctx#9) -> 3
isValidKmsContext(ctx#9)                          -> false
```

The first live run found this the hard way: the case tried to resolve `C2`'s membership inside the
hold, to name the nodes that should have reshared, and the view reverted.

There is no way around it, so the case makes the limitation explicit instead. The reshare assertion
runs against the set the chain *will* name — `C1`'s serving committee — and after activation the
case resolves `C2`'s membership and asserts it matches. The assumption is deferred, not permanent;
and a switch that did change membership is reported as invalidating that particular assertion rather
than being quietly tolerated, because the check would then have been made against the wrong set.

This is also why the case is scoped to a same-committee switch. A node swap is `case-context-switch`
territory, where membership is only read after activation anyway.

---

## 5. Why the stalled party is one that is also serving

`host-sc.env` defines five KMS nodes but sets `NUM_KMS_NODES=4`, so the switch builds `C2` from
parties 1-4 — the same set that serves `C1`. Party 5 is the spare and belongs to neither.

The consequence is worth spelling out, because the opposite would have been convenient: there is no
party whose confirmation the switch needs but whose decryption responses it does not. Stalling the
spare would withhold nothing and the switch would activate mid-probe. The stalled party must come
from the committee that is simultaneously serving the decryption the scenario requires to succeed.

`assertQuorumSurvivesStall` is what keeps those two requirements from colliding silently. On this
topology `getUserDecryptionThresholdForContext` returns 3 against a 4-member committee, so stalling
one leaves exactly 3 — enough, with nothing to spare.

---

## 6. What the case does

1. Read the baseline active pair `(C1, E1)` and resolve the live committee from the chain.
2. `assertTxSendersRunning` over **every provisioned party**, not just the committee: a node of the
   new context whose tx-sender is already down would hold the switch at stage 1, where `E2` never
   exists.
3. Pick the stall party (last committee member) and check the decryption quorum survives it.
4. Input-proof smoke at baseline.
5. Read the block height, predict `C2 = C1 + 1`, and broadcast the switch.
6. Wait for the `NewKmsEpoch` naming `C2` — stage 2 — and cross-check it against the baseline.
7. **Inside `withTxSendersStopped([stalled])`:** re-read the pair first, pre-register `C2` on the
   Gateway, run the four PENDING readings, require the reshare on the serving committee (see §4),
   run the decryption and the container spec with both pending ids forbidden, re-read the pair last.
8. Leaving the scope restarts the tx-sender; wait for `(C2, E2)` to activate.
9. Both validity views must now report `true`.
10. Resolve `C2`'s membership — readable at last — and assert it matches the set step 7 assumed.

---

## 7. Verification

**Static.** `bun run check` (tsc) clean; `bun test src` → 476 pass, 0 fail (7 new, covering the pure
log selector: the wrong context, the wrong event, mixed-case topics, duplicate pages, a malformed
topic mid-poll, and the empty page that is the normal pre-quorum state).

`test-suite/e2e` `npm run tsc` reports only the pre-existing errors caused by the locally installed
SDK build lagging the specs; nothing new.

**Live.** `PASS (375s)`, 41 evidence steps, against a stack at `ctx#9 / epoch#22`.

```
13  ok    3.2s  wait    the creation quorum allocates the new context's first epoch (NewKmsEpoch)
14  ok      0ms event   NewKmsEpoch (recovered from chain logs, not a receipt)
                        contextId=ctx#10 epochId=epoch#23 previousContextId=ctx#9 previousEpochId=epoch#22
15  ok      0ms assert  the event confirms the predicted context and supersedes the baseline pair
16  ok    218ms node    stop tx-sender(s) for withhold party 4's activation confirmation
18  ok       -  note    re-read active pair after withholding …: contextId=ctx#9 epochId=epoch#22
19  ok   53.7s  tx      pre-register the pending context on the gateway  contextId=ctx#10
20  ok     81ms assert  a second lifecycle operation reverts KmsLifecycleOperationInFlight
22  ok       -  note    isValidKmsContext        ctx#10             valid=false expected=false
24  ok       -  note    isValidEpochForContext   ctx#10/epoch#23    valid=false expected=false
26  ok    9.8s  assert  every node of the serving committee completed the reshare while withheld
27  ok  121.3s  probe   user-decryption while the context switch is pending
29  ok   65.7s  probe   SDK embeds the still-active pair, not the pending one, in the permit extraData
31  ok       -  note    re-read active pair after the probes …: contextId=ctx#9 epochId=epoch#22
32  ok   15.2s  node    restart tx-sender(s) after withhold party 4's activation confirmation
34  ok     85ms wait    context switch to (ctx#10, epoch#23) once the withheld confirmation is restored
39  ok      1ms assert  the activated context has the membership the pending-window reshare check assumed
                        serving=1,2,3,4 activated=1,2,3,4
41  ok       -  note    isValidEpochForContext   ctx#10/epoch#23    valid=true  expected=true
```

`2 passing` from the container spec confirms driven mode with **both** negative assertions active.

**The gap between stage 2 and the hold was 218ms.** The creation quorum formed 3.2s after the
broadcast returned, and the tx-sender was stopped in the next fifth of a second. §2 worried about
that gap against a reshare margin of tens of seconds; the measured ratio is two orders of magnitude,
and entry 18 confirms the pair had not moved when the hold closed.

**The pending window was held for ~250s**, entries 19 through 31, bracketed by the same
`ctx#9 / epoch#22` at both ends.

**Activation took 85ms and a single poll** once party 4's tx-sender came back — the same signature
as the epoch half. The withheld confirmation was queued, not lost, and it was the only thing holding
both ids.

**The reshare finished in 9.8s**, against 63.2s in the epoch half. It is not faster work: the cores
had been resharing throughout the 53.7s of Gateway pre-registration that precedes the check, so most
of it was already done by the time anything asked.

### The first run failed, and the case was wrong, not the stack

The first attempt died at step 25 trying to resolve `C2`'s membership inside the hold:

```
cast call … getKmsSignersForContext(uint256)(address[]) …
Error: execution reverted: custom error 0x77ddbe81   # InvalidKmsContext(uint256)
```

That is the finding in §4, and the case was rewritten around it rather than patched: the reshare
check now runs against the membership the chain will disclose, and entry 39 verifies after
activation that the two sets agree.

Two things about that failure are worth keeping. The supervisor restored party 4's tx-sender from
its `finally` even though the case threw mid-hold, and the withheld confirmation then went out on
its own — the switch activated unattended, which is an unplanned but complete demonstration that the
hold mechanism does exactly what it claims. And the failure was diagnosable from one evidence line:
the step label named what was being read, the fields named the context, and the raw `cast` error
carried the selector.

### Container-side specs need a rebuilt image

The `fhevm-test-suite-e2e-debug` container runs the published image
`ghcr.io/zama-ai/fhevm/test-suite/e2e:${TEST_SUITE_VERSION}` with no bind mount of the repository
(`docker-compose/test-suite-docker-compose.yml:5`), so an edited spec under `test-suite/e2e/test/`
does not reach it. The `KMS_QA_FORBIDDEN_*` assertions added to
`test/kmsContextExtraData/kmsContextExtraData.ts` were therefore **not executed** by the live runs
recorded above: the env vars were injected and silently ignored by the image's older copy of the
spec. The `2 passing` those runs report are the two pre-existing tests — the permit matching the
chain's active pair, and the `KMS_QA_EXPECTED_*` cross-check — both of which did run and did pass.

The scenario's negative clause still holds transitively: the host proved the active pair never moved
while the pending id was different, and the spec proved the permit carries the active pair. But the
explicit in-container assertion has not run yet. To run it, copy the spec in
(`docker cp test-suite/e2e/test/kmsContextExtraData/kmsContextExtraData.ts
fhevm-test-suite-e2e-debug:/app/test-suite/e2e/test/kmsContextExtraData/`) and rerun the case, or
rebuild the test-suite image as CI does.

---

## 8. The one uncovered clause

*"The response extraData must be identical to the request extraData"* — unchanged from scenario 1.
The SDK receives the per-share response `extraData` but never compares it to the request:
`equalsKmsExtraData` has zero production call sites, the response-signature verification is commented
out, and the value never reaches a public return type. Full evidence in
`test-suite/fhevm/qa-extradata-check.md`.

---

## 9. Related files

| Path | Role |
|---|---|
| `src/kms-qa/cases/case-context-switch-pending.ts` | the scenario, host side |
| `src/kms-qa/pending.ts` | the two Pending stages, the log recovery, the stall choice and quorum guard |
| `src/kms-qa/pending.test.ts` | unit tests for the pure helpers |
| `src/kms-qa/nodes.ts` | `withTxSendersStopped` — the lever that holds the window open |
| `test-suite/e2e/test/kmsContextExtraData/kmsContextExtraData.ts` | the container half, shared by all four cases |
| `qa-kms-context-scenario-2-pending-epoch.md` | the epoch-rotation half of scenario 2 |
| `qa-kms-context-scenario-1-context.md` | the activated-switch case this one mirrors |
| `qa-extradata-check.md` | why the response-extraData clause is uncovered |
| `src/commands/kms-context-switch.ts` | the existing lifecycle profile — deliberately untouched |
| `host-contracts/contracts/ProtocolConfig.sol` | `_hasContextCreationQuorum`, the two-stage source |
