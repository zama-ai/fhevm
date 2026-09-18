# QA KMS context — scenario 8: every KMS share echoes the request's extraData

Implementation report for the eighth scenario of the `kms-context-qa-tests` profile, implemented as
the `extradata-echo` case.

**Status:** delivered and green. The first live run **failed**, on a real difference; §5 records what it
found, the decision taken, and why the adjusted assertion is not a relaxation.
**Scope:** the **response** half of the KMS-context extraData story, at the relayer/KMS layer.

> This closes a clause that **five** previously proposed scenarios asked for and none could assert.
> §1 explains what changed and why the answer was "no" for so long.

---

## 1. The scenario: as proposed, and as implemented

### 1.1 As originally proposed

```gherkin
Scenario: SDKs v0, v1, and v2 decrypt with the genesis context and epoch
  Given the active pair is genesis context "C1" and genesis epoch "E1"
  And all committee nodes have material for "C1, E1"
  When an SDK v0 sends a decryption request with empty extraData
  And an SDK v1 sends a decryption request with extraData "0x01 || C1"
  And an SDK v2 sends a decryption request with extraData "0x02 || C1 || E1"
  Then all three decryptions must complete successfully
  And each response must preserve exactly the extraData from its request
```

### 1.2 As implemented

```gherkin
Scenario: Every KMS share echoes the request's extraData, byte for byte
  Given ProtocolConfig reports an active pair "C, E"
  And the relayer's latest user_decrypt_req id is recorded as a baseline

  When a user decryption is sent with extraData "0x00"
  And another with extraData "0x01 || C"
  And another with extraData "0x02 || C || E"

  Then all three must reach the "succeeded" state
  And the relayer must have recorded shares for all three versions
  And for every share, user_decrypt_share.extra_data must equal
    user_decrypt_req.req->>'extra_data' BYTE FOR BYTE
  And a v1 request must not come back as 65 bytes
  And a v0 request may come back EMPTY — and only empty
```

> The last line is the one clause that came from measurement rather than from the proposal. It was
> added after the first run failed on it; §5 explains why it is a narrowing of the tolerance, not a
> widening of the pass condition.

### 1.3 Why it changed

**"Each response must preserve exactly the extraData" became assertable — by changing the observer,
not the claim.** The clause is identical; what changed is where it is read. See §2.

**"The genesis context and epoch" became "the active pair".** Nothing in the code path distinguishes
genesis from any other context: v1 skips the epoch check entirely and v0 skips both. Pinning genesis
would demand a pristine stack and buy no coverage, while reading the active pair makes the case
runnable at any point in a stack's life.

**"Empty extraData" became `0x00`.** Truly empty does not survive the relayer. Its validator accepts
exactly three shapes:

```rust
match extra_data {
    "0x00" => Ok(()),
    s if s.len() == 68  && s.starts_with("0x01") => ...,
    s if s.len() == 132 && s.starts_with("0x02") => ...,
    _ => Err(INVALID_EXTRA_DATA_FORMAT),
}
```

An empty string falls to `_` and is rejected with 400. The **contract** accepts it
(`if (extraData.length == 0) return currentKmsContextId;`), but no request reaches the contract that
way through the relayer. So "v0" in practice means `0x00`.

**"All three decryptions must complete successfully" became a precondition, not a finding.** Each of
the three already has a passing test in
`test-suite/e2e/test/unifiedUserDecryption/unifiedUserDecryption.ts` — v0 at :552, v1 at :583, v2 at
:618. Re-proving them would be a fourth copy; this case needs them to succeed so that shares exist to
inspect, and says so in its failure message if one does not.

**Two clauses were added, and they are the point.** See §3: without the v0 and v1 rows, the case
cannot tell an echo from a regenerated value.

---

## 2. Why this clause was uncoverable for five scenarios, and is not any more

*"The response extraData must be identical to the request extraData"* appears in the scenarios behind
`epoch-rotation`, `context-switch`, `epoch-rotation-pending`, `context-switch-pending`, and the one
this case comes from. Every report records it as deliberately uncovered.

The reason was never a shortcut — it is a property of the SDK. It receives the per-share response
`extraData` and never compares or exposes it: `equalsKmsExtraData` has zero production call sites,
the response-signature verification and the cross-share consistency check are both commented out, and
the value reaches no public return type. Full evidence in `qa-extradata-check.md`.

That document listed two ways out, and warned that the second — re-enabling the SDK's inert
verification — is a product decision that should not be flipped from a QA ticket.

**The way out taken here is neither.** The relayer already stores both sides:

| Table | Column | Holds |
|---|---|---|
| `user_decrypt_req` | `req->>'extra_data'` | what was sent |
| `user_decrypt_share` | `extra_data` | what one share answered |

joined on `gw_reference_id`, one row per share. Reading them needs no bypass client, no SDK change,
and observes the production path end to end — SDK → relayer → gateway → KMS → relayer.

It is also **stronger than the clause asks**. The scenario says "the response"; the table stores each
share separately, so the case asserts that *every* node echoed. A single node regenerating the field
is caught even when the other three are correct.

Two request shapes reach the table — the legacy `/v2` route stores
`contract_addresses`/`ct_handle_contract_pairs`, the unified `/v3` route stores
`handles`/`allowed_contracts` — but `extra_data` sits at the top level of both, so the read is
route-agnostic.

**Scope, stated plainly:** this proves the relayer received shares carrying the request's extraData.
It says nothing about the SDK, which still does not verify the echo. That half remains a product
decision, and `qa-extradata-check.md` still governs it.

---

## 3. Why three versions, and why v2 alone would prove nothing

The failure guarded against is not a dropped field. It is the field being **regenerated** downstream
from the responder's own view of the active context, instead of passed through.

| Version | Bytes | Role |
|---|---|---|
| `0x02 \|\| C \|\| E` | 65 | **control** — a regenerated value would be byte-identical to what was sent. Echo and regeneration are indistinguishable here; it proves only that the path works. |
| `0x01 \|\| C` | 33 | **discriminates** — carries no epoch. Anything rebuilt from the active pair comes back as 65 bytes. |
| `0x00` | 1 | **discriminates hardest** — any regeneration at all is visible. |

Evidence that the weak case is genuinely weak: at the time this case was written, the relayer's
database held 240 shares, **all of them v2**, all echoing correctly. That history distinguishes
nothing. The v0 and v1 rows this case creates are the first of their kind.

---

## 4. Not forcing the test to match the implementation

The comparison is **byte-exact on the stored strings**, never on decoded pairs. A decoded comparison
would accept a v1 request answered with a v2 payload naming the same context — precisely the
regeneration this case exists to detect. Comparing `(context, epoch)` would let a broken system pass.

Two guards keep it from passing vacuously:

- **no rows** → fails, naming the two possible causes. An echo assertion over zero shares would
  otherwise report success for a probe that never reached the KMS;
- **a missing version** → fails, listing which. Without v0 and v1 the case cannot discriminate, and a
  run that silently produced only v2 would be reporting the weak case as the whole case.

If a share comes back different, that is the finding. The failure message prints both sides with
their byte lengths — because the *shape* of the divergence is what identifies the layer at fault —
plus the SQL to inspect it directly.

**Attribution limit, deliberately not overstated:** the rows record what the relayer received. A
mismatch proves the echo broke; it does not prove *where* — KMS core, connector, or the relayer's own
storage are all candidates. The case says so rather than blaming a component it cannot observe.

---

## 5. The first run failed, on a real difference

The case was written asserting a byte-exact echo for all three versions. The first live run reported
`EXIT=1`, 4 of 12 shares diverging — all four shares of the v0 request:

| Share | Sent | Received | Bytes sent | Bytes received |
|---|---|---|---|---|
| 0 | `0x00` | `0x` | 1 | 0 |
| 1 | `0x00` | `0x` | 1 | 0 |
| 2 | `0x00` | `0x` | 1 | 0 |
| 3 | `0x00` | `0x` | 1 | 0 |

v1 (33 bytes) and v2 (65 bytes) echoed byte for byte.

### What it is not

**It is not the regeneration this case exists to detect.** A field rebuilt from the responder's view
of the active pair would come back as 65 bytes. The decisive evidence is v1: it returned **33 bytes,
with no epoch** — something a regenerated value could not produce. There is no regeneration.

### What it is

Deliberate normalization of the legacy marker. The kms-connector converts `0x00` to empty before the
KMS core sees it, and says so in a unit test named for the behaviour:

```rust
// kms-connector/crates/kms-worker/src/core/event_processor/decryption.rs
#[test]
fn kms_decryption_extra_data_normalizes_legacy_zero_marker() {
    assert_eq!(kms_decryption_extra_data(&Bytes::from_static(&[0x00])), Vec::<u8>::new());
}
```

`0x00` and empty both mean "no context". The normalization preserves meaning; it does not preserve
bytes.

### The decision, and why it is not a relaxation

The finding was put to the requirement owner rather than resolved in the test. The decision: for the
v0 marker, one byte in and zero bytes out is acceptable — the concern was regeneration, and that was
answered.

The assertion was adjusted, and **narrowed** in the process. `isAcceptableEcho` admits exactly one
substitution — `0x00` → `0x` — and nothing else:

| Request | Response | Verdict |
|---|---|---|
| `0x00` | `0x` | accepted — the measured normalization |
| `0x00` | `0x00` | accepted — should the normalization ever be removed |
| `0x00` | 33 or 65 bytes | **rejected** — regeneration |
| v1 | v2 | **rejected** — regeneration |
| v1 or v2 | `0x` | **rejected** — the exception does not generalize |

Widening the rule to *"any length is fine for v0"* would have surrendered precisely the
discrimination v0 was chosen to provide. The predicate is pure and carries seven unit tests, so the
boundary of the exception is itself under test rather than resting on a comment.

---

## 6. What the case does

1. Read the active pair from `ProtocolConfig`.
2. Read `max(user_decrypt_req.id)` from the relayer as a baseline — bounding the later read to this
   case's own requests by id rather than by timestamp, so no clock skew and no overlap with a
   concurrent suite.
3. Drive the container spec: three decryptions, one per version, each required to reach `succeeded`.
4. Read every request/share pair recorded after the baseline.
5. Assert rows exist and all three versions are present.
6. Assert every share's `extra_data` equals its request's, byte for byte.

## 7. How to run

```bash
cd test-suite/fhevm
KMS_QA_CASES=extradata-echo ./fhevm-cli test kms-context-qa-tests

# the container half alone (drives the decryptions; asserts no echo)
./fhevm-cli test kms-context-extradata-echo
```

**Non-disruptive.** Three ordinary decryptions and two reads: no lifecycle transaction, no container
stopped, no context or epoch advanced. `mutatesLifecycle: false`, no topology requirements,
rerunnable against the same stack.

Requires the relayer's database container (`fhevm-relayer-db`) — overridable via `RELAYER_DB_*`.

## 8. Verification

**Static.** `bun run check` clean; `bun test src` → 483 pass (up from 477: seven new tests pin the
boundary of the v0 exception).

**Live.** `PASS (64s)`, after the adjustment described in §5.

```
note  relayer rows recorded by the probe  requests=3 shares=12 versions=0x00,0x01,0x02
note  echo confirmed for one request  requestId=64  extraData=0x0207…  (65 bytes)  shares=4
note  echo confirmed for one request  requestId=65  extraData=0x0107…  (33 bytes)  shares=4
note  echo confirmed for one request  requestId=66  extraData=0x00     (1 bytes)   shares=4
```

Three requests, twelve shares, all three versions present. The middle row is the one that matters:
**33 bytes back for 33 bytes sent, with no epoch added** — the proof that nothing downstream rebuilds
the field from its own state.

## 9. Related files

| Path | Role |
|---|---|
| `src/kms-qa/cases/case-extradata-echo.ts` | the case and its assertions |
| `src/kms-qa/relayer-db.ts` | the relayer-database reads and `isAcceptableEcho` — new, not a change to `kms-connector-db.ts` |
| `src/kms-qa/relayer-db.test.ts` | seven unit tests pinning the boundary of the v0 exception |
| `kms-connector/.../event_processor/decryption.rs` | `kms_decryption_extra_data_normalizes_legacy_zero_marker` |
| `test-suite/e2e/test/kmsContextExtraData/extraDataEcho.ts` | the container half: three decryptions |
| `qa-extradata-check.md` | why the SDK cannot express this clause, and still cannot |
| `test-suite/e2e/test/unifiedUserDecryption/unifiedUserDecryption.ts` | the three passing tests this case takes as preconditions |
