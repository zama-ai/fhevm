# QA KMS context — scenario: the SDK uses a new context and epoch

Implementation report for the context-switch scenario of the `kms-context-qa-tests` profile,
implemented as the `context-switch` case.

**Status:** delivered and green against a live stack.
**Scope:** both halves — the host orchestration and the container-side `extraData` spec.

> Sibling of [`qa-kms-context-scenario-1-epoch.md`](./qa-kms-context-scenario-1-epoch.md), which
> covers the epoch-rotation scenario. The two share the same methodology, the same toolkit and the
> same container spec; this document records only what is specific to the context switch.

---

## 1. The scenario: as proposed, and as implemented

### 1.1 As originally proposed

```gherkin
Scenario: The SDK uses a new context and epoch
  Given the previous active pair is "C1, E1"
  And a context switch to "C2, E2" has completed
  And context "C2" was registered in the Gateway before activation
  And ProtocolConfig returns "C2, E2" as the active pair
  When the application performs a decryption through the SDK
  And the test captures the request and response extraData
  Then the decryption must complete successfully
  And the request extraData must decode as version "0x02", context "C2", and epoch "E2"
  And the response extraData must be identical to the request extraData
  And context "C1" and epoch "E1" must not be used by the new request
```

### 1.2 As implemented

```gherkin
Feature: Normal decryption after a new context becomes active

  Scenario: The SDK uses the currently active context and epoch
    Given ProtocolConfig reports an active pair "(C_prev, E_prev)"
    And a context switch to the next sequential context has been broadcast
    And that context was pre-registered in the Gateway before activation
    And ProtocolConfig reports the activated pair "(C, E)", with C == the pre-registered id
    When the application performs a decryption through the SDK
    And the test captures the request extraData
    Then the decryption must complete successfully
    And the request extraData must decode as version "0x02", context "C", and epoch "E"
    And the response extraData must be identical to the request extraData
    And neither "C_prev" nor "E_prev" may appear in the request extraData
```

### 1.3 Why it changed

**The same three concessions as the epoch scenario**, for the same reasons, recorded in full in
[§2.3 of the epoch report](./qa-kms-context-scenario-1-epoch.md): the literal ids had no source and
are now read from `ProtocolConfig`; `Given … has completed` is a precondition this case establishes
and *verifies* rather than assumes; and the superseded pair is read from the chain instead of being
carried as a constant. Those were accepted as precedent for this scenario without re-litigating them.

**One concession specific to the context switch: the pending id is predicted, then verified.**

The epoch case sends `defineNewEpochForCurrentKmsContext` with `cast send` and reads `NewKmsEpoch`
straight off the receipt. That is not available here. `defineNewKmsContextAndEpoch` carries the whole
committee definition — node params, thresholds, software version, PCR values — which lives in the
contracts task's env file, so the switch is broadcast as a compose task (`host-sc-context-switch`)
and returns no receipt.

The new context id is therefore derived from the contract's sequential-allocation invariant
(`_storeNextKmsContext`, so `C = C_prev + 1`) **before** the chain reports it — which is necessary,
because the Gateway must be pre-registered with that id before activation. The prediction is never
trusted: the case asserts the id the chain actually activates equals the one pre-registered, the
same check `kms-context-switch.ts:222` makes. A mismatch means a client would present a context the
Gateway rejects.

**One clause could not be implemented**, exactly as in the epoch scenario: *"the response extraData
must be identical to the request extraData"*. The SDK neither verifies nor exposes the response
value. See [`qa-extradata-check.md`](./qa-extradata-check.md).

### 1.4 Clause-by-clause coverage

| Clause | Runs | Status |
|---|---|---|
| `Given the previous active pair is "C_prev, E_prev"` | host | done |
| `And a context switch … has completed` | host | done — broadcast **and** activation awaited |
| `And context … was registered in the Gateway before activation` | host | done — and the activated id is asserted to be that one |
| `And ProtocolConfig returns "(C, E)" as the active pair` | host | done |
| `When the application performs a decryption through the SDK` | container | done |
| `And the test captures the request … extraData` | container | done |
| `Then the decryption must complete successfully` | container | done |
| `And the request extraData must decode as version "0x02", context "C", epoch "E"` | container | done |
| `And neither "C_prev" nor "E_prev" may appear` | container | done — asserted explicitly, not merely implied |
| `And the response extraData must be identical to the request extraData` | container | **not covered — not possible through the SDK** |

Nine of the ten clauses are covered.

## 2. How it differs from the epoch scenario

| | `epoch-rotation` | `context-switch` |
|---|---|---|
| What moves | epoch only, same context | **both** context and epoch |
| Broadcast | `cast send defineNewEpochForCurrentKmsContext()` | `host-sc-context-switch` compose task |
| New id known from | the `NewKmsEpoch` receipt | predicted as `previous + 1`, then verified on activation |
| Gateway | not involved | **pre-registered before activation**, and the activated id must match |
| Negative clause | epoch `E_prev` must not appear | **neither** `C_prev` **nor** `E_prev` may appear |
| Committee | read once | read **before and after** — a switch is the operation that can change it |

## 3. What the case does

1. Read the baseline active pair `(C_prev, E_prev)`.
2. Resolve the live committee from the chain, for the pre-switch context.
3. Input-proof smoke at baseline, so a later failure is attributable to the transition.
4. Predict the pending context id as `C_prev + 1` and record it as a prediction.
5. Broadcast `defineNewKmsContextAndEpoch` via `host-sc-context-switch`, with no env override (the
   `HOST_SC_CONTEXT_ENV` swap file is only for node swaps). Note the committee comes from the
   task's env file, **not** from current chain state — on a stack that has been node-swapped this
   restores the env's committee rather than preserving the serving one.
6. Pre-register that context on the Gateway via `gateway-sc-context-switch`.
7. Wait for activation: the context must equal the predicted id **and** the epoch must advance.
8. Assert the activated context is exactly the pre-registered one, and that the epoch really moved.
9. Resolve the committee again, and record whether it changed across the switch.
10. Require `new_kms_epoch.status = completed` on every **serving** node.
11. Input-proof smoke and user-decryption under the new pair.
12. Run the container spec, injecting both the new pair and the superseded one.

## 4. The shared container spec

`test-suite/e2e/test/kmsContextExtraData/kmsContextExtraData.ts` serves **both** scenarios — the
client-side claim is identical in each: the SDK must embed the pair that is active on chain. Only
what the host half did first differs.

It was extended for this scenario with the explicit negative assertion. The profile injects
`KMS_QA_PREVIOUS_CONTEXT_ID` / `KMS_QA_PREVIOUS_EPOCH_ID`; when present the spec asserts the decoded
ids differ from them, and also that the superseded ids differ from the active ones — so a transition
that silently did not advance fails here rather than passing on a tautology. Absent, those checks are
skipped and the spec still runs standalone.

The epoch case now injects the previous epoch too, so its own negative clause is asserted explicitly
rather than merely implied by the equality checks.

## 5. How to run

```bash
cd test-suite/fhevm

# both cases, in registry order
./fhevm-cli test kms-context-qa-tests

# this scenario only
KMS_QA_CASES=context-switch ./fhevm-cli test kms-context-qa-tests
```

Requires `--scenario five-party-swap-threshold-kms` (or `KMS_QA_ALLOW_ANY_SCENARIO=1` on a topology
that satisfies the case's own requirements — a same-committee switch needs the committee only).

**Disruptive.** Advances both the context and the epoch, and does not roll them back.

## 6. Verification

Static: `tsc --noEmit` clean; `bun test src` → 453 pass, 0 fail; both cases listed by the
`KMS_QA_CASES` preflight error; the profile registers and rejects `--grep` as before.

Final run:

```
[kms-context-qa][context-switch][wait]   context switch to contextId=…638598 ok 15.2s
[kms-context-qa][context-switch][assert] the activated context is the one pre-registered
                                         activatedContextId=…638598 preRegisteredContextId=…638598
[kms-context-qa][context-switch][note]   committee across the switch before=1,2,3,4 after=1,2,3,4 changed=false
[kms-context-qa][context-switch][assert] every serving node completed the new epoch reshare ok 809ms
[kms-context-qa][context-switch][probe]  user-decryption under the new context ok 117.4s
[kms-context-qa][context-switch][probe]  SDK embeds the new (context, epoch) in the permit extraData ok 61.2s
                                         contextId=…638598 epochId=…301263
                                         previousContextId=…638597 previousEpochId=…301262

  2 passing

[kms-context-qa] PASS (407s) — 1 case(s)
```

`2 passing` confirms the container half ran in driven mode, with the negative assertions active:
neither `C_prev` nor `E_prev` appears in the permit's `extraData`.

### The first live run failed, and the cause was the stack, not the case

The switch was defined but never activated — the wait burned its full 600s budget. The evidence
pinned it precisely:

| Signal | Value |
|---|---|
| context `…638597` | created, `isLive=true`, `isValid=false` → **Pending** |
| parties 1, 2, 3, 5 | `new_kms_context=completed`, `responses=completed` |
| **party 4** | `new_kms_context=completed`, **`responses=pending`** |
| `kms-connector-4-tx-sender` | **exited 4 hours earlier** |

Party 4's worker had processed the event and stored its response, but with no tx-sender it was never
submitted. `_hasContextCreationQuorum` requires **every** node of the new context to confirm, and the
env defines all five — so the context could never leave Pending.

The node had been stopped by an earlier `kms-context-switch` run, whose node-swap step stops the
dropped party's tx-sender and, in its own words, *"left down"* (`kms-context-switch.ts:177`).
Restarting it was enough: the queued confirmation went out and the stuck context activated on its
own, confirming the diagnosis.

**Fix:** `assertTxSendersRunning` (`src/kms-qa/nodes.ts`) now runs as the case's very first step and
checks every provisioned party. The failure went from a 600s timeout saying *"did not activate, check
the logs"* to **0.5s** naming the container, explaining the quorum rule, and giving the remediation —
including the likelihood that `kms-context-switch` left the node down.

This is the second finding of the same shape: the epoch scenario's first run assumed the committee
was `1..committeeSize`, this one assumed every tx-sender was up. Both assumptions break on a stack
that has been node-swapped. Worth considering a health preflight shared by all cases.

## 7. Related files

| Path | Role |
|---|---|
| `src/kms-qa/cases/case-context-switch.ts` | this case |
| `src/kms-qa/protocol-config.ts` | `broadcastContextSwitch`, `preRegisterContextOnGateway` |
| `test-suite/e2e/test/kmsContextExtraData/kmsContextExtraData.ts` | the shared container spec |
| `qa-kms-context-scenario-1-epoch.md` | the sibling scenario, and the shared rationale |
| `qa-extradata-check.md` | why the response-extraData clause is uncovered |
| `src/commands/kms-context-switch.ts` | the existing lifecycle profile — deliberately untouched |
