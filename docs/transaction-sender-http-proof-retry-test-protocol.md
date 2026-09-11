# HTTP transaction sender: proof retry reliability test protocol

Recorded: 2026-09-11. Reviewed baseline: `8833a4f06` on v0.13.4.

Status: candidate patch implemented; focused local checks
are recorded below. The broader test campaign and release gate remain pending.
Use `67762d493` as the unfixed transport baseline, not the candidate patch.
The historical evaluation used `8833a4f06`.

## Candidate implementation and campaign handoff

The selected remedy preserves proof eligibility and lets the existing operation
loop back off. It does not stop/restart the process for HTTP failures, introduce
request expiry, or depend on fresh client requests for sender-level recovery.

After the existing contract/configuration-error checks, `process_proof` uses
`is_transient_gateway_error` in `src/ops/common.rs`. A matching failure increments
the existing failure metric, logs a preservation warning, and returns `Err`
without updating the proof row. Consequently the terminal `retry_count` and its
associated database `last_error`/`last_retry_at` fields remain unchanged. Use logs,
RPC observations, and the failure counter to observe transient attempts; do not
interpret an unchanged database retry timestamp as an idle sender.

The classifier's explicit policy is:

| Error | Preserve terminal retry budget? |
| --- | --- |
| HTTP 408, 429, 500, 502, 503, 504 | Yes |
| Typed reqwest connection, timeout, request-send, or body-read error, including through an error source chain | Yes |
| Exact existing local `eth_sendRawTransactionSync timeout` custom error | Yes |
| `BackendGone` or missing batch response | Yes; BackendGone still reaches the existing shutdown gate |
| JSON-RPC code 429 | Yes |
| JSON-RPC code -32000 or -32603 with exact message `context deadline exceeded`, `service unavailable`, `temporarily unavailable`, or `too many requests` (case/outer whitespace normalized) | Yes |
| Other errors | Existing handling |

No generic substring match for “timeout” or “unavailable” is used. Contract
reverts and recognized permanent coprocessor configuration errors are handled
before this classifier. Unknown errors, authentication errors, nonce errors,
and unsuccessful receipts retain existing behavior. This is an intentional
allowlist: compare actual Gateway error codes/messages with it during the
campaign, and report uncovered infrastructure classes rather than claiming
universal outage protection.

### Review decisions: HTTP 500 and provider-specific rate limits

HTTP 500 is intentionally classified as transient. The status alone does not
establish that the proof or sender configuration is permanently invalid, so
preserving unfinished work takes precedence over exhausting its retry budget.
This differs from excluding 500 because it might represent an application
failure. The tradeoff is explicit: a persistent application defect returning
500 can keep the affected work retrying indefinitely and may impede queue
progress. Operation backoff bounds attempts; it does not guarantee fairness or
completion. Monitor repeated failures and queue age, and retain selective-error
fairness testing as a release-gate requirement. An HTTP 500 response is distinct
from an HTTP 200 response carrying an unknown JSON-RPC internal error; the
latter still follows limited-retry accounting.

The JSON-RPC allowlist is deliberately narrower than Alloy's general
provider-specific rate-limit policy. For example, JSON-RPC `-32005` is not
covered even though Alloy recognizes it as a rate-limit signal. Such errors
retain limited-retry accounting and can exhaust proof eligibility if repeated.
Do not infer support from the English meaning of an error message. Capture the
actual Gateway error codes and compare them with this policy; extend it with
explicit classification and preservation/recovery tests if the target Gateway
uses another infrastructure error class.

For comparison, the HTTP migration on
`rudy/feat/http-for-gw-listener-and-tx-sender`, reviewed at `6eb0c5cf1`
(migration commit `f3788d7b5`, Alloy 2.2.0), uses transport retries and exits the
sender when those retries exhaust. It excludes HTTP 500 and inherits Alloy's
broader rate-limit recognition. This patch keeps retry ownership in the
operation loop, retains the existing send deadline, and explicitly covers
reqwest connection/timeouts and the allowlisted Conduit deadline responses.
Both approaches aim to preserve work for classified infrastructure failures;
they do not have identical failure coverage. Neither resolves the deferred
accepted-but-unmined nonce reconciliation problem.

No database schema, dependency, CLI default, add-ciphertext retry accounting,
nonce allocation/reset logic, or submission RPC changes are included. Batch
sibling cancellation and accepted-but-unmined nonce recovery remain follow-up
work. Restarts do not revive rows already exhausted before this patch; account
for pre-existing exhausted work explicitly when preparing the campaign.

### Deployment configuration

The local stack now passes `GATEWAY_URL` (the existing HTTP variable) to the
sender and explicitly sets both batch limits to 10. The listener continues to
use `GATEWAY_WS_URL`. The Rust CLI defaults remain unchanged.

Helm gains optional `txSender.config.gatewayUrl`, accepting `value` or
`valueFrom`, so the listener's `commonConfig.gatewayUrl` can remain WebSocket.
If omitted, the sender still falls back to the shared URL for chart
compatibility; HTTP sender builds reject a WebSocket fallback at startup.
Example candidate values (use the actual endpoint or a secret reference):

```yaml
txSender:
  config:
    gatewayUrl:
      value: https://gateway.example/rpc
  extraArgs:
    - --verify-proof-resp-batch-limit=10
    - --add-ciphertexts-batch-limit=10
```

Merge these flags with the deployment's existing `extraArgs`; do not replace
unrelated flags or add duplicate batch flags. Batch 10 + 10 is a conservative
candidate configuration, not a production capacity claim. Repeat campaign runs
with actual deployment settings, including 128 proof concurrency if that is
still being considered. Preserve the deployed retry limit of 15 in those runs.

### Focused checks and remaining validation

- Four classifier unit tests cover the HTTP allowlist and exclusions, RPC
  message/code boundaries, local timeout classification, actual refused HTTP
  connections, actual request deadlines, and invalid-URL exclusion.
- `gateway_outage_preserves_verify_proof_retries` replaces the historical
  deletion assertion. It requires more observed failures than the terminal
  budget allows while the proof remains eligible.
- Four `verify_proof_transport_tests` cases start at retry count 2 with maximum
  3, exercise repeated faults, check backoff, clear the fault, and require both
  database completion and a matching contract event. They cover estimation
  HTTP 503, nonce-read HTTP 429, submission RPC deadline errors, and local send
  timeouts. Across the cases they exercise removal enabled/disabled and
  verification/rejection responses. This is sampled coverage, not the full
  cross-product in section A.
- Proxy additions return HTTP/RPC errors before forwarding and provide
  `DiscardAfter`, which never forwards an expired request after fault removal.
- Local Helm rendering checks cover shared-URL fallback, the sender-only HTTP
  override, secret references, and the unchanged listener URL. Compose parsing
  checks the explicit HTTP URL and batch arguments.

Local execution results on 2026-09-11 (original retry patch `0ffb9a3ba`,
before semantic history rewriting):

| Check | Result |
| --- | --- |
| Classifier unit tests | 4/4 passed |
| HTTP transport policy tests | 5/5 passed |
| New fault/recovery integration cases | 4/4 passed, 67.73 seconds |
| Existing proof regressions, including AWS signing, permanent configuration errors, and retry exhaustion | 32/32 passed |
| Updated connection-refusal preservation test | 1/1 passed on corrected rerun, 15.10 seconds |
| Helm renders and Compose configuration | Passed |
| Rust formatting, diff whitespace, and protocol links | Passed |

The first proof-suite run passed all 32 other cases. The updated outage test
passed its preservation assertions but failed cleanup because its inherited
two-second graceful-shutdown allowance was shorter than the existing four-second
backoff. The test allowance was raised to five seconds and the targeted rerun
passed. Runtime shutdown behavior was not changed. These results account for
46 distinct passing tests across the selected suites, not a rerun of every
transaction-sender test.

No real Gateway, client-retry, long-outage, or mixed-load campaign has been run
for this candidate.

Commands from `coprocessor/fhevm-engine` (integration tests require the existing
Anvil/PostgreSQL harness; the full proof suite also exercises AWS signing):

```sh
cargo test --release --locked -p transaction-sender --lib transient_gateway_tests
cargo test --release --locked -p transaction-sender --test verify_proof_transport_tests
cargo test --release --locked -p transaction-sender --test verify_proof_tests
cargo test --release --locked -p transaction-sender --test https_transport_tests
```

The campaign still needs the baseline negative control with the new acceptance
assertions, production-duration outages, full fault matrix, database transition
auditing, partial failures/fairness, repeated restart and mixed-operation cases,
and actual client/relayer expiry and retry behavior. Unit classification of a
fault is not equivalent to end-to-end validation of that fault.

## Review follow-up: startup errors and credential-safe diagnostics

The reviewer follow-up keeps the public `get_chain_id` API fallible: an invalid
URL scheme returns an error rather than panicking or retrying forever. All
callers propagate the result. The former `client_policy_is_bounded` assertion
was removed because comparing two duration literals did not exercise the client.
URL rejection tests now include credential markers and the public startup probe.
The binary parses its Gateway URL after clap so malformed-URL errors cannot echo
the supplied credential-bearing argument.

Both legacy provider reconnect flags remain accepted. The review corrects the
documentation: startup probing uses `graceful_shutdown_timeout` as its interval
in this branch, as it did before the transport switch; `provider_retry_interval`
does not control it. This follow-up does not change that startup timing.

`diagnostics::safe_rpc_error` emits fixed error categories, HTTP/RPC codes, and a
small allowlist of known messages. It never formats arbitrary Gateway response
bodies, RPC data, custom error strings, or local-signing error details. This
protects against bare tokens echoed by an upstream as well as full URLs.
`safe_error` locates typed RPC/reqwest failures inside error chains before
formatting them; other non-Gateway diagnostics retain their existing text.
Original error objects still drive retry classification, contract error decoding,
and BackendGone handling. Sanitization changes presentation, not those decisions.

Safe diagnostics are used for startup probes, operation failures, both proof and
ciphertext error columns, health responses, and final process-error output.
The sender's logging setup additionally disables Alloy/HTTP/TLS dependency events
and spans that independently expose URLs or response bodies, at all log levels.
The filter is global to both JSON logging and OTLP. Application logs and metrics
remain available. Shared tracing initialization retains its existing default
behavior for other services; only the sender opts into this filter. Library
consumers supplying their own tracing subscriber must apply the same filter if
they enable dependency diagnostics.

This prevents new sender diagnostic leaks. It does not rewrite previously
persisted error strings or purge historical logs in external stores. Historical
cleanup must target the actual deployment stores; URL-only substitution is not
sufficient if old response bodies contain bare tokens.

Additional focused validation:

- Unit cases cover credential-bearing HTTP errors, custom/local errors, RPC
  messages/data, malformed JSON, and anyhow contexts, retaining original types.
- Binary startup tests inspect stdout/stderr for malformed and unsupported
  credential-bearing URLs.
- `gateway_diagnostics_tests` uses real providers and PostgreSQL, captures logs
  with TRACE enabled, and injects URLs containing user/password/path/query
  markers plus an independent bare token. HTTP 400, HTTP 429, RPC errors, and
  malformed JSON exercise both limited and unlimited database-write paths. It
  also checks startup-probe and health diagnostics.
- Re-run the classifier and proof-preservation/recovery regressions to ensure
  sanitizing diagnostics does not change recovery behavior.

Local validation on 2026-09-11 passed all 18 targeted tests: 6 library tests,
7 HTTP/startup tests, 1 database/log privacy integration test, and 4 proof
preservation/recovery tests. A separate binary smoke check at TRACE with a
credential-bearing unreachable HTTP endpoint also passed without leaking markers.
Changed Rust files pass formatting checks; the workspace-wide formatting check
reports pre-existing differences in generated Gateway bindings.

Deployed JSON-log and OTLP-export inspection with synthetic credential markers
remains part of the campaign; do not use real credentials as assertion output or
fault-response fixtures.

```sh
cargo test --release --locked -p transaction-sender --lib \
  --test https_transport_tests --test gateway_diagnostics_tests \
  --test verify_proof_transport_tests
```

## Scope and acceptance contracts

On the unfixed baseline, issue 1 is proof retry-budget exhaustion during Gateway infrastructure failures.
HTTP connection errors do not produce the WebSocket `BackendGone` signal. The
sender can keep retrying while consuming the proof's terminal retry budget.
With removal enabled, exhausted rows are deleted; with removal disabled, they
remain in the database but are excluded from processing.

The original protocol evaluates a sender-level preservation fix:

> A Gateway infrastructure failure must not make unfinished proof work disappear
> or become permanently ineligible. After connectivity returns, the sender must
> complete that work automatically.

The user clarified that clients time out waiting for verification responses and
are expected to retry. This introduces a second possible acceptance contract:

> An expired individual request may be abandoned, provided a compliant client
> can obtain a correct result through supported retries after recovery, within
> an agreed latency and retry budget, without manual intervention.

These are different guarantees. Client retries can mitigate issue 1 without
fixing sender-level preservation. Report which contract was tested. The original
preservation protocol below remains appropriate for assessing a preservation
fix; the additional end-to-end protocol assesses reliance on client retries.

Neither protocol establishes issue-2 nonce safety for accepted-but-unmined
submissions. Record any such failures separately rather than overlooking them.

## Deployed readiness boundary (clarified 2026-09-11)

The deployment owner identifies the application as `@zama-fhe/sdk@3.5.1`
wrapping `@fhevm/sdk@0.13.2` (commit `07fb05fb7`), with relayer v0.13.4
and v0.13.0 as rollback. These deployment identities are supplied information;
the wrapper's application-level retry behavior has not been independently tested.
The SDK's `RelayerAsyncRequest.ts` is byte-identical between that commit and
this branch. Its GET 503 path throws immediately; it does not start a fresh
request. The default one-hour global deadline and 1,440-loop cap therefore do
not provide recovery after `readiness_check_timed_out`. Network fetch retries
and polling 202 responses are different from application resubmission after a
terminal error. Do not infer the latter from the former.

The readiness loop and testnet example settings are identical at relayer tags
v0.13.0, v0.13.2 and v0.13.4: 75 attempts separated by three-second sleeps.
There are 74 sleeps before exhaustion, approximately 222 seconds plus RPC time;
“225 seconds” is an operational approximation, not a hard wall-clock deadline.
A never-ready contract result produces the readiness timeout; contract RPC
errors can produce a different terminal error. Record both separately.

This readiness check concerns ciphertext availability for **decryption**.
It is distinct from input-proof job expiry and redispatch in section B.
The existing mock input-proof expiry test does not validate the incident's
readiness boundary. Preserve section A's unconditional sender recovery gate;
client polling does not justify deleting unfinished proofs.

### C. Incident-specific decryption acceptance matrix

Focused execution results are recorded in
[the campaign log](transaction-sender-validation-campaign-2026-09-11.md#sdk-readiness-campaign-continuation).
The actual SDK request implementation reached terminal readiness 503 at about
224 seconds against both relayer versions with mocked Gateway readiness.
Polling-default/floor and shortened final-attempt/fresh-request controls passed.
The separate real-stack public-decryption case passed on v0.13.4 with SDK
0.13.2: readiness expiry at 224.131 seconds, followed by correct plaintext
from one explicit fresh call after sender recovery at 233.200 seconds.
This does not establish automatic application-wrapper retries, full-stack
user decryption, or full-stack rollback behavior.


Run with the specified SDK version against relayer v0.13.4, then repeat on
v0.13.0. Record application wrapper version, resolved SDK package integrity,
relayer image digest and effective runtime retry configuration. Use real
ciphertext registration and decryption in the local stack; mocked readiness
checks are focused controls only. Include both public and user decryption.

| Scenario | Required observation |
| --- | --- |
| Ciphertext registration delayed, readiness restored before the final check | Original request completes correctly, with no terminal readiness error or application resubmission. Correlate sender submission, Gateway registration, readiness checks and SDK result. |
| Ciphertext remains unavailable through all 75 checks | Relayer returns 503 with `readiness_check_timed_out`; actual SDK call rejects on that response, without waiting an hour or automatically issuing another POST. Count GETs and POSTs. |
| Recovery immediately before versus after the final readiness observation | Before: existing request can succeed. After: terminal request stays failed; test an explicit fresh application call after recovery and establish its deduplication/result behavior. Do not assume input-proof deduplication rules apply to decryption. |
| SDK receives network errors, 202 and terminal 503 | Verify network retry and Retry-After polling independently; neither may be reported as retrying the terminal 503. Include absent Retry-After and a value below the SDK's one-second floor. |
| Sender-wide outage and selective proof failures during readiness | Track addCiphertext progress independently of proof responses. Require recovery without DB repair; record later-proof starvation as a separate failing gate even if decryption completes. |
| Outage exceeds the readiness window | Preserve sender work and recover it; expect the original SDK call to fail. Any claim of automatic user recovery requires observing an application-level retry policy, not an idealized retrying test client. |

Use shortened attempt intervals for deterministic boundary/race tests, and at
least one unscaled 75-attempt run per relayer version. Synchronize boundary
injection to observed check numbers, not a sleep near 225 seconds. Retain a
monotonic timeline, readiness call counts, SDK HTTP counts, job IDs, terminal
labels and final plaintext correctness. SDK poll counts alone do not measure
relayer check counts. Keep the extra KMS-share wait and its latency separate
from the readiness window.

Passing preservation and HTTP performance tests supports the sender patch;
it does not promise uninterrupted client success across an outage longer than
the relayer readiness budget. Automatic resubmission after 503 remains an
application behavior to establish, not a requirement to redesign this patch.

## Repository evidence

- [Proof retry and selection logic](../coprocessor/fhevm-engine/transaction-sender/src/ops/verify_proof.rs):
  unclassified send failures increment `retry_count`; selection requires
  `retry_count < verify_proof_resp_max_retries`.
- [Existing outage regression file](../coprocessor/fhevm-engine/transaction-sender/tests/verify_proof_tests.rs):
  at baseline `8833a4f06`, `gateway_outage_burns_verify_proof_retries` demonstrates deletion with six
  retries and 1–4 second backoff. Its reported approximately 19-second result
  must not be described as a measurement with the deployment's fifteen retries.
  The candidate replaces it with a preservation assertion, as described above.
- [Fault proxy](../coprocessor/fhevm-engine/transaction-sender/tests/support/mod.rs)
  and [failure tests](../coprocessor/fhevm-engine/transaction-sender/tests/https_failure_mode_tests.rs)
  provide a starting harness.
- [Relayer deduplication](../relayer/src/store/sql/repositories/input_proof_repo.rs):
  identical input requests conflict while the existing request is neither
  `failure` nor `timed_out`; completed requests can return cached results.
- [Relayer v2 handler](../relayer/src/http/endpoints/v2/handlers/input_proof.rs)
  dispatches fresh work only for newly inserted requests.
- [Relayer timeout processing](../relayer/src/store/sql/repositories/timeout_repo.rs)
  marks old `receipt_received` input requests `timed_out` using the configured
  timeout. This is a server-side state transition, distinct from a client
  stopping its wait. Verify other state timeout paths in the deployed flow.
- [Gateway request contract](../gateway-contracts/contracts/InputVerification.sol)
  assigns a fresh `zkProofId` to each successful `verifyProofRequest` call.
- [Gateway listener](../coprocessor/fhevm-engine/gw-listener/src/gw_listener.rs)
  inserts proof work with `ON CONFLICT(zk_proof_id) DO NOTHING`. Replaying an
  event for an existing exhausted row does not reset its retries.

These are observations of this branch, not proof that every deployed client or
API version follows this exact path. Record the actual client/API versions and
runtime timeout configuration before running end-to-end tests.

## A. Sender-level preservation protocol

### A1. Harness and observations

Use the real transaction sender, PostgreSQL, and a method-aware fault proxy in
front of Anvil with a verification contract. Keep Anvil alive while injecting
network outages so chain state survives restoration. Use a fixture that permits
checking actual verification/rejection effects, not merely unconditional mock
receipts. If consensus requires multiple coprocessors, distinguish this sender's
contribution from completion of the overall request.

For each proof, record original identity/payload, initial retry count, every
terminal retry-counter change, deletion, RPC outcome, contract completion, and
sender exit/restart. Use a test-only database audit mechanism so polling cannot
miss intermediate updates or deletion/recreation. Database disappearance is not
evidence of success; correlate it with contract completion or an explicitly
expected permanent outcome.

Define each injected error's classification before running the test. Do not
classify every JSON-RPC error as an infrastructure failure.

### A2. Negative control

Run the unfixed baseline with max retries 3, removal enabled, the existing
1–4 second backoff, and a valid unfinished proof. After startup, refuse Gateway
connections. Require the acceptance assertions to detect exhaustion/deletion.
Repeat with removal disabled and require detection of permanent ineligibility.

Run the same acceptance assertions against the candidate. Confirm that work was
selected and the fault was encountered; an idle sender must not pass.

### A3. Fault matrix

Apply faults separately to gas estimation, nonce retrieval when its cache is
empty, and submission. Include other prerequisite RPCs actually used by the
deployed signing/provider configuration. A submission-only classifier is not
sufficient when gas estimation can fail first.

| Scenario | Injection | Required result |
| --- | --- | --- |
| Gateway unavailable | Refuse connections after startup | Preserve eligibility; recover after restoration |
| Connection lost | Reset/close before forwarding | Same |
| Gateway stops answering | Hold without forwarding; discard held requests | Bounded request duration and retries; preserved work |
| HTTP overload/unavailability | 429, 502, 503, 504 separately | No terminal retry-budget consumption |
| Transient RPC failure | HTTP 200 carrying an explicitly classified transient JSON-RPC error | Same |
| Submission-only failure | Healthy reads/health probes; transient send errors | Health probes must not mask proof loss |
| Intermittent failure | Deterministic fail/fail/succeed and ten-failures/recover sequences | No exhaustion; automatic completion |

For core issue-1 isolation, failed submissions must not reach Anvil. The existing
proxy's `Blackhole` fault forwards parked requests when cleared; add a mode that
discards them instead. Its current 180-second park cap must also be accounted
for in longer tests. Otherwise recovery unintentionally introduces ambiguous
acceptance or changes the injected error class.

Use deterministic fault schedules and barriers. A timeout or reset after
upstream acceptance belongs in the additional issue-2 interaction tests.

### A4. Survival and recovery sequence

1. Seed valid proofs with retry counts 0 and `max_retries - 1`.
2. Confirm that the sender encounters the injected failure.
3. Keep the failure active beyond several times the measured unfixed exhaustion
   window for that configuration.
4. Require every unfinished proof to remain present and eligible, with no
   infrastructure-induced increase in its terminal retry count.
5. Restore connectivity without database repair or new client requests.
6. Require every proof to complete before a predeclared recovery deadline.

The near-exhaustion case detects implementations that spend one retry before
shutting down and then delete the proof on restart.

For retry-in-place, require repeated fault encounters. For shutdown/restart,
use a supervisor harness and require repeated restart cycles with no cumulative
terminal retry-budget consumption. Test startup while the Gateway is down too.

Use max retries 3 for fast regression tests, then repeat with max retries 15 and
the actual deployment backoff, request deadlines, and batch settings. Start with
a five-minute outage, extending it if needed to exceed several unfixed
exhaustion windows. Test both settings of removal-after-exhaustion.

Set recovery deadlines before examining candidate results. Base them on the
healthy drain time for the same workload plus configured polling/backoff/startup
delays and explicit scheduling allowance; report the numerical bounds used.

### A5. Queue progress, restarts, and bounded load

Repeat with more than two batches of proofs, both verification and rejection
responses, new proofs arriving during/after outages, and concurrent
add-ciphertext work. Include repeated outage/recovery cycles and sender restarts
with the same database and chain.

Require later work to progress: preserving the oldest batch forever while
starving subsequent work is not recovery. Include selective failures confined
to early items and verify the candidate's declared fairness policy.

Measure attempt timestamps, active tasks, and outstanding requests. With the
current policy and immediate errors, operation-level backoff should be roughly
1, 2, 4, 4... seconds, allowing scheduling tolerance. Declare equivalent bounds
if the candidate changes this policy. Requests within a batch can be concurrent;
do not confuse their spacing with operation-level backoff.

A candidate that skips the retry increment but returns “success, more work” can
create a tight loop. Require bounded concurrency, no task/request accumulation
over repeated cycles, and bounded supervisor restart frequency.

### A6. Permanent-error controls

Require valid work to complete and already-verified/already-rejected responses
to retain their intended cleanup behavior. Explicitly permanent configuration
errors must still become terminal. Errors designated for limited retries must
retain that behavior.

Test permanent failure -> infrastructure outage -> permanent failure. The
outage must neither consume nor erase the pre-existing terminal retry history.

### A7. Sender-level release gate

- Baseline fails the acceptance assertions; candidate passes.
- Infrastructure failures preserve unfinished work and eligibility.
- Recovery needs no manual repair or fresh client request.
- Retry, concurrency, and restart behavior remain bounded.
- Permanent-error semantics are unchanged or explicitly reviewed.
- Production-configuration and mixed-operation restart cases pass.

Passing establishes issue-1 preservation within the tested failure model. It
does not prove arbitrary-outage reliability or accepted-but-unmined nonce safety.

## B. End-to-end client-retry protocol

### B1. Why client retries change the assessment

If the intended contract permits abandoning expired requests, losing an
individual sender job need not permanently prevent the user from verifying the
input. The impact can instead be an additional timeout/retry cycle, duplicated
work/fees, and lower availability during recovery. Sender-level persistence is
then a design choice to evaluate against that end-to-end contract, not an
unconditional requirement inferred solely from the presence of a queue.

However, a client timeout alone does not imply a fresh Gateway request. In this
branch's relayer v2 path, retries while the old request remains active are
deduplicated. Once it becomes `failure` or `timed_out`, an identical request can
be inserted anew and dispatched. A fresh successful Gateway request receives a
new proof ID. Test that entire transition and the actual client retry policy.

An outage shorter than the client's timeout can still delete work early and
force the user to wait for relayer expiry despite the Gateway having recovered.
Client-side waiting expiry must not be assumed to cancel or expire the on-chain
request; late responses may still arrive.

### B2. Full-stack harness and identifiers

Use the supported client/API, relayer, Gateway listener, proof worker, sender,
database, and Gateway contracts with the required consensus topology. Inject
sender-path failures after the initial request reaches the Gateway. Also test
a broader outage affecting request submission and relayer observation.

Track one logical verification across client attempts, relayer external job IDs,
content-derived internal IDs, Gateway proof IDs, coprocessor rows, and final
client-visible results. Record client wait/retry limits, relayer state-expiry
timeouts and scheduler cadence, retry delays, and sender settings independently.
Use the same logical input for retries; generating different input would bypass
the deduplication behavior being tested.

### B3. Required scenarios

| Scenario | Evidence required |
| --- | --- |
| Sender job deleted; client retries while relayer job remains active | Observe deduplication; verify the client continues through eventual expiry rather than exhausting its own retry budget |
| Retry after relayer marks request timed out/failed | Observe fresh dispatch, new Gateway proof ID, eligible coprocessor work, and a correct client-visible result |
| Gateway recovers before client/relayer timeout | Measure additional avoidable waiting after recovery against the agreed latency bound |
| Outage spans multiple client and relayer timeout windows | Completion after recovery within supported retry policy; explicit bounded failure if the policy is exhausted |
| Old proof completes after a replacement request starts | Correct correlation; no incorrect acceptance/rejection or corruption of the replacement request |
| Old proof completes just before/after relayer expiry | Deterministic race tests for cached-result versus fresh-request behavior |
| Many clients retry together | Bounded request amplification and recovery drain time; no repeated exhaustion under renewed load |
| Relayer/sender restarts around expiry and resubmission | Persistent state/deduplication permits recovery without manual repair |

Exercise sender exhaustion with removal both enabled and disabled. A new proof
ID should not be obstructed by an old exhausted row; replaying the same ID must
not be mistaken for fresh eligible work. Verify this rather than assuming it.

Use controlled clocks/barriers for race tests where practical, then perform a
run with actual deployment timeout values. Include failures/rejections as well
as successful verification. Explicitly test the supported application's retry
behavior; an idealized client retrying forever cannot validate a finite policy.

### B4. End-to-end acceptance and reporting

Predeclare the supported outage duration, maximum client-visible recovery
latency, client retry budget, and acceptable request/fee amplification. There is
no finite retry policy that guarantees completion across an arbitrarily long
outage. Report bounded failure behavior outside the supported window.

Within that window require a correct result for each logical input, no permanent
deduplication trap, no manual repair, correct handling of late results, and load
that remains within declared limits. Record unnecessary proof computation and
Gateway calls as well as final success. Do not use `latest = pending` alone as
evidence that all logical requests completed.

Passing B without A supports deploying with an explicitly accepted client-retry
recovery contract; it must not be reported as a sender preservation fix. Passing
A and B supports both guarantees. Issue 2 requires its own nonce reconciliation
tests and remains outside either approval claim.

## Result record

For every run retain commit hashes, client/API versions, topology and config,
fault class/location/schedule, expected classification, proof/job identities,
database transition audit, RPC counts/outcomes, contract evidence, restart
history, recovery time, and pass/fail reason. Record baseline and candidate
results separately and identify any deviations from the predeclared bounds.
