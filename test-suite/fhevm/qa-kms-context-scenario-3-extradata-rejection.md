# QA KMS context — scenario 3: a corrupted extraData is refused

Implementation report for the third scenario of the `kms-context-qa-tests` profile, implemented as
the `extradata-rejection` case.

**Status:** delivered and green against a live stack.
**Scope:** the wire-format validation of the KMS-context `extraData`, and the order in which it runs
relative to signature verification.

> Scenarios 1 and 2 prove the SDK puts the **right** context/epoch pair into the permit's extraData
> across every lifecycle transition. This one proves a **wrong** one does not get through. Same
> field, same versioned format, opposite direction — so the container spec lives beside its siblings
> in `test/kmsContextExtraData/`, and the host half is a case in the same profile.
>
> The check is enforced by the Relayer, but the subject is the context envelope, not the Relayer.

---

## 1. The scenario: as proposed, and as implemented

### 1.1 As originally proposed

```gherkin
Feature: Rejection of corrupted extraData by the Relayer

  Scenario: The Relayer rejects a request with a corrupted extraData version
    Given ProtocolConfig returns a valid active pair "C1, E1"
    And an instrumented SDK has built a valid decryption request
    And the original request extraData is "0x02 || C1 || E1"
    And the test harness changes only the version byte to an unsupported version
    When the altered HTTP body is sent directly to the Relayer API
    Then the Relayer must reject the request as invalid extraData
```

### 1.2 As implemented

```gherkin
Scenario: The Relayer rejects a request with a corrupted extraData version
  Given ProtocolConfig returns an active pair "C1, E1"
  And globalThis.fetch is instrumented so the SDK's request is captured, never sent
  And the real @fhevm/sdk has built and signed a unified user-decryption request
  And the captured extraData is exactly "0x02 || C1 || E1"
  When only the version byte is changed to 0x03
  And the altered envelope is POSTed to /v3/user-decrypt
  Then the Relayer must respond 400 with label "validation_failed"
  And the flagged field must be "extraData"
  And the flagged field must NOT be "signature"
  And the reported issue must name the accepted versioned formats

Scenario: The captured request is accepted when it is not corrupted
  Given the same capture, replayed verbatim
  When the untouched envelope is POSTed to /v3/user-decrypt
  Then the Relayer must respond 202

Scenario Outline: The Relayer rejects every malformed extraData shape
  Given a request signed OVER the malformed extraData, so the signature is valid
  When the envelope is POSTed to /v3/user-decrypt
  Then the Relayer must respond 400 flagging the extraData field
  Examples: unsupported version, v2 truncated, v2 with a trailing byte,
            v1 version on a v2-sized payload, untagged contextId, untagged epochId
```

### 1.3 Why it changed

**"An instrumented SDK" became a `fetch` interception, and nothing is sent.** The SDK's outgoing
request is captured and answered with a synthetic rejection rather than forwarded. Letting the
original through would run a real decryption and make the tampered replay a *duplicate* of an
already-accepted request — the relayer dedups on
`(handles, userAddress, allowedContracts, publicKey, extraData)`, so the replay would no longer be
judged on its own.

**The `Given` about the extraData is verified, not assumed.** The suite asserts the captured value is
exactly `0x02 || C1 || E1` against the pair read from `ProtocolConfig`. Without that, the tamper
could be corrupting something that was never valid, and the rejection would prove nothing.

**"Reject as invalid extraData" became a field-level assertion.** This is the substantive change, and
§2 explains why a status-code assertion cannot express the clause.

**A control was added.** Replaying the captured envelope untouched must yield `202`. Without it the
main case proves only that the relayer rejects *something*: a harness that mangled the envelope while
capturing it would produce the same `400`.

**Two scenarios were added.** The malformed-shape outline isolates the format rules from the ordering
question, and the control isolates the harness from the subject.

---

## 2. Why "the Relayer must reject it as invalid extraData" is not a status-code assertion

A request whose extraData is tampered **after signing** has two defects at once: the extraData is
malformed, and the EIP-712 signature no longer covers it. Both are rejected by the relayer as:

```
HTTP 400  { "status": "failed", "error": { "label": "validation_failed", … } }
```

Identical status, identical label. The only thing that distinguishes them is the field detail:

| Rejected for | `error.details[].field` | `issue` |
|---|---|---|
| extraData | `attestedPayload.extraData` | `Must be 0x00, or a versioned format: …` |
| signature | `signature` | `Signature is invalid` |

The extraData path is **nested**: `extra_data` lives on the inner `attested_payload`, which the v3
envelope declares `#[validate(nested)]`, so the reported path is `attestedPayload.extraData` and not
`extraData`. The suite matches on the leaf segment, which keeps the assertion readable and survives
the envelope being re-nested.

(`relayer/src/http/utils/responses.rs:182` and `src/http/endpoints/v2/types/error.rs:239`.)

So asserting `httpStatus === 400` would pass whether the relayer honored the scenario or rejected
the request for the *other* defect. The suite asserts the `extraData` field is flagged **and the
`signature` field is not**.

### What that assertion actually pins

The tampered request is only reported as an extraData failure because of the order the v3 handler
validates in (`relayer/src/http/endpoints/v3/handlers/user_decrypt.rs`):

| Line | Step |
|---|---|
| 173 | `parse_and_validate::<AttestedUserDecryptRequestJson, _>` → runs `validate_extra_data_field_decryption` |
| 184 | `"Successfully parsed and validated v3 request"` |
| 202 | signature pre-check → `invalid_signature` |

Request validation runs **before** signature verification. If those two steps were ever reordered,
the corrupted request would come back as a signature failure and this test would fail — which is the
regression this suite exists to catch. Nothing else in the repository exercises that ordering,
because every other extraData test signs over the malformed value and therefore has no second defect
to be confused with.

---

## 3. What the Relayer accepts

`validate_extra_data_field_decryption` (`relayer/src/http/utils/validations.rs:146`) is wired as a
`#[validate(custom(...))]` on the v3 envelope's `extra_data` field:

| Form | Bytes | Constraints |
|---|---|---|
| `0x00` | 1 | legacy marker |
| `0x01 \|\| contextId` | 33 | contextId's first byte must be `0x07` |
| `0x02 \|\| contextId \|\| epochId` | 65 | `0x07` and `0x08` tags respectively |

Each version has a **fixed** size; trailing bytes are rejected. New fields arrive by bumping the
version, not by appending — which is why the truncation and trailing-byte cases are separate
branches of the validator rather than incidental.

A second, stricter validator exists for input proofs (`validate_extra_data_field_input_proof`, which
accepts only `0x00`); it is out of scope here.

---

## 4. What this suite does NOT cover

**Semantic validation of the ids.** An unknown `contextId` or an inactive `epochId` is well-formed at
the wire level and is rejected further in — by the KMS Connector and the Gateway, asynchronously.
That axis already has coverage in `test/unifiedUserDecryption/unifiedUserDecryption.ts`
(`rejects extraData v2 with an inactive epochId`, `rejects extraData with an unknown contextId`).
The line between the two suites is the layer, not the value: **format at the relayer, meaning at the
KMS**.

**The response-path parser.** `parse_context_id_from_extra_data` (`relayer/src/host/extra_data.rs`)
has its own `UnsupportedVersion` / `TooShort` errors, but it runs on the extraData coming *back* from
the gateway (`src/gateway/user_decrypt_handler.rs:382`). No corrupted request can reach it.

**The relayer's own unit tests.** `validations.rs:398-462` already covers the accept/reject table in
isolation. This suite is not a second copy of that: its value is that the *deployed* relayer has the
validator wired to the route, that rejection precedes signature verification, and that the body the
real SDK produces is what gets validated.

---

## 5. Overlap with the existing suite, and why it was duplicated

Three of the malformed shapes — unsupported version, untagged contextId, untagged epochId — already
have tests in `test/unifiedUserDecryption/unifiedUserDecryption.ts`. They assert
`expect(post.httpStatus).to.equal(400)` and nothing further.

They are re-covered here at full assertion strength rather than edited in place, following this
workstream's convention of adding files rather than modifying existing ones. The duplication is
deliberate and worth its cost: the existing assertions cannot distinguish an extraData rejection from
any other `400`, so they would not catch the reordering described in §2 — the very thing this suite
is for. The two sets can diverge safely; if the existing ones are ever strengthened, the copies here
become redundant and can go.

---

## 6. How to run

```bash
cd test-suite/fhevm

# this scenario only
KMS_QA_CASES=extradata-rejection ./fhevm-cli test kms-context-qa-tests

# the whole profile, in registry order
./fhevm-cli test kms-context-qa-tests

# the container half on its own, without the profile
./fhevm-cli test kms-context-extradata-rejection
```

Standalone, against a running stack (the orchestrator cross-check skips):

```bash
cd test-suite/e2e
npx hardhat test --grep "KMS context extraData rejection" --network staging
```

**The only non-disruptive case in the profile** — `mutatesLifecycle: false`, and the only one with no
topology requirements at all. It sends no lifecycle transaction, stops no container, and advances no
on-chain state. The only side effect is one accepted decryption job (the control), which the suite does not
poll. It can be run repeatedly against the same stack.

It skips itself when the configured instance is not the `@fhevm/sdk` client, since the capture has
nothing to instrument otherwise.

---

## 7. Verification

**Static.** `test-suite/fhevm` `bun run check` clean. `test-suite/e2e` `npm run tsc` reports, for the
new files, only the pre-existing `signUnifiedDecryptionPermit` error caused by the locally installed
SDK build lagging the specs — the same error already present on `kmsContextExtraData.ts:145` before
this change.

**Live.** `PASS (55s)`, 5 evidence steps, driven through the profile.

```
[kms-context-qa][extradata-rejection][note]  active pair … contextId=ctx#11 epochId=epoch#24
[kms-context-qa][extradata-rejection][probe] a corrupted extraData is refused, and refused for
                                             the extraData rather than the signature  ok 55.3s

  ✔ agrees with the orchestrator
  ✔ refuses a tampered version byte on an SDK-built request        (153ms)
  ✔ accepts the same SDK-built request when it is not corrupted    (159ms)
  ✔ refuses every malformed shape with a field-level error         (139ms)
  4 passing
```

`4 passing` is the driven-mode signal: standalone the orchestrator cross-check reports as pending and
the suite shows `3 passing, 1 pending`.

The rejection body, verbatim from the run:

```json
{"status":"failed","requestId":"8f028573-…","error":{
  "label":"validation_failed",
  "message":"Validation failed for 1 field in the request: attestedPayload.extraData",
  "details":[{"field":"attestedPayload.extraData",
    "issue":"Must be 0x00, or a versioned format: 0x01 + 32-byte contextId (0x07-tagged first byte), or 0x02 + 32-byte contextId (0x07-tagged) + 32-byte epochId (0x08-tagged)"}]}}
```

A request whose extraData was tampered **after** signing is reported as an `extraData` failure and
never as a `signature` failure — the clause §2 exists to pin, observed rather than assumed.

The assertions run in milliseconds; the 48s is almost entirely the container's fixed startup (wasm
compile, worker pool). The whole suite sends 8 POSTs and polls nothing.

### The first run failed, and the assertion was wrong, not the relayer

The tamper was rejected exactly as designed on the first attempt — with the right label, the right
issue text, and the right field. The suite failed because it asserted the field was `extraData` while
the relayer reports the nested path. Fixed by matching the leaf segment; no change to the harness or
the scenario.

### A prerequisite the run exposed

The `fhevm-test-suite-e2e-debug` container runs the published image
`ghcr.io/zama-ai/fhevm/test-suite/e2e:${TEST_SUITE_VERSION}` with **no bind mount of the repository**
(`docker-compose/test-suite-docker-compose.yml:5`). A new or edited spec under `test-suite/e2e/test/`
therefore does not reach the container: the first run of this suite reported `0 passing (2ms)` and
`matched zero tests`, because the file did not exist there.

Local verification requires copying the files in first:

```bash
docker cp test-suite/e2e/test/relayerExtraData   fhevm-test-suite-e2e-debug:/app/test-suite/e2e/test/
```

Shipping the change requires rebuilding that image, which is what CI does. This applies to every
container-side spec in this workstream, not just this one.

---

## 8. Implementation notes

**The feature probe must be let through.** `fetchFeatures`
(`sdk/js-sdk/src/core/modules/relayer/module/fetchFeatures.ts:41`) POSTs an empty body `{}` to the
same `/v3/user-decrypt` route to detect whether the relayer supports it, and needs a real `400`/`404`
back. An interceptor keyed on the route alone would capture that probe instead of the request, and
would break capability detection at the same time. The filter is therefore on the payload carrying an
`attestedPayload.extraData` — the real request, never the probe.

**Every POST to the route is answered synthetically once interception is armed**, not just the first.
`RelayerAsyncRequest` retries, and a fall-through on the second attempt would silently send the real
request.

**The corruption helpers operate on the hex string**, never on the decoded ids. The scenario is about
the wire format; the contextId and epochId stay exactly what the SDK put there.

---

## 9. Related files

| Path | Role |
|---|---|
| `test-suite/fhevm/src/kms-qa/cases/case-extradata-rejection.ts` | the case, host side |
| `test-suite/e2e/test/kmsContextExtraData/extraDataRejection.ts` | the container half |
| `test-suite/e2e/test/kmsContextExtraData/extraDataRejectionHttp.ts` | capture, raw POST, field-level assertions, corruption helpers |
| `test-suite/e2e/test/kmsContextExtraData/kmsContextExtraData.ts` | the positive sibling: the SDK embeds the right pair |
| `test-suite/e2e/test/sdk/unified/unifiedUserDecrypt.ts` | the shared unified signer/poster, reused for the signed-over-corruption cases |
| `test-suite/e2e/test/unifiedUserDecryption/unifiedUserDecryption.ts` | the semantic (KMS-layer) extraData coverage |
| `relayer/src/http/utils/validations.rs` | `validate_extra_data_field_decryption` |
| `relayer/src/http/endpoints/v3/handlers/user_decrypt.rs` | the validation-before-signature ordering |
| `relayer/src/host/extra_data.rs` | the response-path parser, out of scope |
