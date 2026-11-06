# RelayerHttpRequest retry — requirements and background

What any design of the HTTP-level retry in `RelayerHttpRequest` must satisfy (§1–§4), and the
context it was derived from (§5–§6). The design itself is `docs.md` in this folder. Paths are
relative to `sdk/js-sdk/`.

Rules for this file: a design choice goes in the design, never here. A requirement changes only
by an explicit decision, logged in §7 with its source. When two items conflict, the later wins and
the earlier says "superseded by". IDs are never reused: a removed item stays, marked "removed".

## 1. Goal and scenario

Retry only the branch "response not ok **and** body is a valid relayer-http error body".
A decision says whether to retry and after which delay; "retry" waits, then fetches again.
Everything else keeps today's behavior: non-JSON bodies, other non-ok JSON bodies, oversized
bodies, network failures (already retried by `fetchWithRetry`), timeouts, aborts, 2xx schema
mismatches.

**Scenario: two classes.** The current class becomes an internal single-attempt class,
`RelayerHttpAttempt` (S1–S6): one fetch → read → parse → classify, with the signal it is given.
A public `RelayerHttpRequest` (S7–S10), with the public surface of §5.2, keeps validation and owns
the request lifetime (S11–S15) and the retry loop. The abort check is a shared module-level
function. The split lets one attempt be tested alone with a synthetic signal (R4–R6).

## 2. General requirements

| ID | Requirement | Source |
|---|---|---|
| R1 | **Goal.** Retry only as in §1; "do not retry" behaves exactly as today (throw `RelayerHttpResponseError`). | plan |
| R2 | **Abort paths.** Every existing abort path keeps working (caller signal already aborted, during fetch, while reading the body, during the network-retry delay), and also during the wait between two HTTP attempts. | plan |
| R3 | **Global timeout.** `options.timeout` bounds the whole run, all attempts and waits included. | plan |
| R4 | **Testing is critical.** Every behavior (decision, loop, single attempt, each abort path, global timeout, cleanup) has focused, deterministic tests. | plan |
| R5 | **Security is critical.** No unbounded retries, waits or memory; no weakening of input validation, abort handling, body size limits or error classification. | plan |
| R6 | **Maintainability is critical.** Small single-responsibility units, each understandable and testable alone. | plan |
| R7 | **Existing code may change freely** (class names, constructor params, internal signatures, callers, error fields, tests) when it serves R4–R6. Nothing is public yet. | plan |
| R8 | **Conventions.** Short single-purpose functions, guard clauses, extracted helpers, `_`-prefixed module-private helpers, 1–3 line comments, no plan references in source comments; a bug test fails before its fix; commits are one conventional-commit title line; no push without consent. | plan |
| R9 | **Type visibility.** Each type is *real public* (what a root caller manipulates, even unnamed, e.g. the constructor params; root callers: `fetchUserDecryptNoGateway.ts`, `fetchPublicDecryptNoGateway.ts`, the SDK user), *fake public* (exported only for tests or cross-module use) or *private* (module file only). Real public types are minimal. | user |
| R10 | **No retry options for now.** No public retry option type (no `RelayerHttpRetryOptions`); defaults only. | user |
| R11 | **One decision point.** A single method of `RelayerHttpRequest` computes, from the error, whether to retry and after which delay. Supersedes the module-level function of §6. | user |
| R12 | **Design deliverable.** Names the two classes, their interface layout and every signature; no implementation. | user |
| R13 | **Overridable decision, policy out of scope.** The R11 method is overridable (now, and for tests). The policy (which errors, delays, cap) comes later: the design defines the signature and the override, not the policy. | user |
| R14 | **Staged delivery, never-retry default.** The machinery may land before the policy; until then the default decision is "no retry" for every error, so behavior stays today's (R1); retries run only through an override. | user, U1 |
| R15 | **Loop attempt ceiling.** Whatever the decision, at most 5 HTTP attempts per run (R5). It can only block a retry; the later policy sets its own cap within it. | user, U2 |
| R16 | **Body reader unchanged.** `response.text()` and the oversized-body behavior stay (R1); a bounded streaming reader (the `TODO(SECURITY)`) is a separate follow-up. | user, U3 |

## 3. Scenario rules

**Single-attempt class**
- **S1.** Internal: not exported from the package entry points.
- **S2.** Its constructor takes already-normalized, already-validated values, no raw options.
- **S3.** Owns no timer, no caller-signal listener, no cleanup; neither creates nor owns the signal.
- **S4.** Runs fetch (with network retries), body read, size check, parse, status/shape check and
  success validation, and throws exactly the error types of §5.3.
- **S5.** Classifies aborts with the shared abort check, using the `signal` and `timedOutAfterMs` it was given.
- **S6.** Keeps the body guards (`_isRelayerHttpErrorBody`, the success assertions,
  `MAX_BODY_TEXT_LENGTH`, the result-error wrapping) and the `[RelayerHttpRequest]` log prefix.

**Public class `RelayerHttpRequest`**
- **S7.** Same constructor parameters as today and every current check (operation, headers, query,
  timeout): invalid input still throws at `new`.
- **S8.** `run()` is one-shot, with today's message on a second call.
- **S9.** Creates the combined signal once, then runs the attempts inside `try { … } finally { cleanup(); }`.
- **S10.** The request init (`JSON.stringify`, `setAuth`) is built inside that `try`.

**Abort and timeout**
- **S11.** The public class owns the only `AbortController`, global timer and caller-signal
  listener; `cleanup()` runs once. No attempt creates its own timer.
- **S12.** Every attempt receives the same `signal` and `timedOutAfterMs`.
- **S13.** The wait between attempts uses the same signal; our timer → `RelayerHttpTimeoutError`,
  caller abort → the caller's reason unchanged.
- **S14.** An abort between two attempts is caught by the next attempt's `fetchWithRetry`
  (`throwIfAborted()` before fetching) and classified by the attempt.
- **S15.** The combined signal exposes the remaining time; a delay `>=` remaining rethrows the
  current `RelayerHttpResponseError` without waiting.

**Retry decision** (located by R11; per R13, S16–S17 bind every decision, default or override;
S18, S19 and the policy part of S22 bind only the later policy)
- **S16.** Called only with a `RelayerHttpResponseError` from a relayer-http error body; "no retry"
  rethrows that same error.
- **S17.** "Retry" → the delay is an integer in `[1, MAX_TIMEOUT_MS]`.
- **S18.** Stops once the maximum number of HTTP attempts is reached.
- **S19.** Inputs: status, code, attempt (§5.5, §5.6). Pure: no I/O, no timers.

**Logging**
- **S20.** One `warn` per HTTP retry, prefix `[RelayerHttpRequest]`, with url, status, code,
  `requestId`, attempt and delay.
- **S21.** When retrying stops, the existing error log and the last `RelayerHttpResponseError`
  are emitted and thrown unchanged.

**Tests**
- **S22.** Decision: table-driven over statuses and codes; attempt cap; delay range.
- **S23.** `run()` over a mocked `fetch` sequence: retryable then success (2 calls); non-retryable
  (1 call); retryable up to the cap; retryable then another failure type (type unchanged); delay
  longer than the remaining timeout (no wait).
- **S24.** Abort during the wait (fake timers): caller string reason, caller `null`, caller's own
  `TimeoutError`, our timeout → `RelayerHttpTimeoutError`.
- **S25.** Listener and timer cleanup after a run that retried (success and failure).
- **S26.** Existing tests whose status becomes retried get a response sequence matching the new call count.

## 4. Known gaps a design must address

Found in reviewing the scenario; how each is solved is a design choice.

| # | Gap |
|---|---|
| G1 | No bound on retries or delay without `options.timeout` (vs R5). |
| G2 | Retry policy undecided (`isRetryable` statuses vs kms-connector `retryable()` codes, §5.6). Retrying 403 would hang a signal test that uses a 403 error body. |
| G3 | The attempt logs an error on every retried response (vs S21). |
| G4 | The fallback `throw` after a failed wait is unspecified. |
| G5 | `instanceof RelayerHttpResponseError` also matches a caller abort reason of that type. |
| G6 | A loop that builds `new RelayerHttpAttempt` itself cannot be tested without `fetch`. |
| G7 | `JSON.stringify(payload)` per attempt: a payload mutated during a wait changes the body. |
| G8 | The attempt's one-shot guard is pointless, and its `default:` "constructor rejects" comment becomes false. |
| G9 | The generated `.d.ts` is gitignored (`sdk/js-sdk/.gitignore:2`): not a file to commit. |

## 5. Background: the code before the change

### 5.1 Files and commands

| File | Role |
|---|---|
| `src/core/modules/relayer/module/RelayerHttpRequest.ts` | The class to change. |
| `…/fetchUserDecryptNoGateway.ts`, `…/fetchPublicDecryptNoGateway.ts` | Callers: `new RelayerHttpRequest({ relayerOperation, url, payload, options, logger })`, then `await request.run()`; map the shares, or `decryptedValue`, `signatures`, `extraData`. |
| `src/core/base/fetch.ts`, `src/core/base/timeout.ts` | `fetchWithRetry`, `normalizeHeaders`; `abortableSleep`. |
| `src/core/errors/RelayerHttp{Response,Fetch,Timeout,UnexpectedResponse,Result}Error.ts` | Error types thrown. |
| `src/core/types/relayer.ts` | `RelayerCommonOptions`, `RelayerUserDecryptOptions`, `RelayerPublicDecryptOptions`, `FetchResultOf`. |

Typecheck: `npx tsc -p src/tsconfig.json --noEmit` (never without `--noEmit`; the root
`tsconfig.json` is a solution file that checks nothing). Tests: `npx vitest run --config src/vitest.config.ts <file>`.

### 5.2 Public surface (unchanged by the design)

```ts
type RelayerHttpOperation = Extract<RelayerPostOperation, 'USER_DECRYPT' | 'PUBLIC_DECRYPT'>;
type RelayerHttpRequestParams<O extends RelayerHttpOperation> = {
  relayerOperation: O;
  url: string;
  payload: Record<string, unknown>;
  options?: Omit<RelayerUserDecryptOptions, 'onProgress'> | Omit<RelayerPublicDecryptOptions, 'onProgress'> | undefined;
  logger?: Logger | undefined;
};
export class RelayerHttpRequest<O extends RelayerHttpOperation> {
  constructor(params: RelayerHttpRequestParams<O>);
  run(): Promise<FetchResultOf<O>>;
}
```

Options used: `auth`, `headers`, `query`, `signal`, `timeout`, `fetchRetries`,
`fetchRetryDelayInMilliseconds`. `onProgress` is excluded by type and never called.

### 5.3 Algorithm

Constants: `MAX_TIMEOUT_MS = 2_147_483_647` (setTimeout's largest delay; Node turns a delay above
it, below 1 or NaN into 1 ms); `MAX_BODY_TEXT_LENGTH = 10 MiB` of characters, checked after the
read, before `JSON.parse`.

**Constructor** (synchronous checks): operation is `USER_DECRYPT` or `PUBLIC_DECRYPT`, else
`InvalidPropertyError`; `normalizeHeaders` (lower-cases names, RFC 7230 tokens, rejects CR/LF/NUL
and case collisions, drops non-strings; throws); `normalizeRelayerQuery`; `options.timeout` is
`undefined` or an integer in `[1, MAX_TIMEOUT_MS]`, else `InvalidPropertyError`.

**`run()`**
1. One-shot guard (`_hasRun` set before any work).
2. Combined signal: one `AbortController`; the caller's signal is forwarded (already aborted →
   abort at once; else a `{ once: true }` listener); with a timeout, a `setTimeout` that, unless
   already aborted, records `firedTimeoutMs` and aborts with `DOMException('Timed out', 'TimeoutError')`.
   `timedOutAfterMs()` is defined only when *our* timer fired (the reason's name cannot tell it from
   a caller's `AbortSignal.timeout()`). `cleanup()` clears the timer and the listener.
3. In `try { … } finally { cleanup(); }`: build `init` with `setAuth({ method: 'POST', headers:
   { …normalized, 'content-type': 'application/json', 'zama-sdk-version', 'zama-sdk-name' },
   body: JSON.stringify(payload), signal }, auth)` (both can throw), then:
   - **fetch** through `fetchWithRetry`; on throw → abort check `''`, else logged
     `RelayerHttpFetchError` ("network error");
   - **read** `response.text()`; on throw → abort check `' while reading the response body'`, else
     `RelayerHttpFetchError` with status;
   - **size and parse**: too long → `RelayerHttpFetchError` (500-char excerpt); `JSON.parse` failure
     → `RelayerHttpFetchError`, whatever the status;
   - **status and shape**: non-ok + `_isRelayerHttpErrorBody` (non-empty `code`/`message`, UUIDv4
     `requestId`) → logged `RelayerHttpResponseError({ url, status, code, responseMessage, requestId })`
     — **the branch to retry**; other non-ok → `RelayerHttpUnexpectedResponseError`. Ok →
     `USER_DECRYPT`: envelope (`status === 'succeeded'`, UUIDv4 `requestId`), non-empty
     `result.result`, each share `payload` hex no-0x, 65-byte `signature`, required `extraData` →
     the shares; `PUBLIC_DECRYPT`: same envelope, `decryptedValue` hex no-0x, non-empty 65-byte
     `signatures`, `extraData` hex with 0x → `result`; a failed assertion → `RelayerHttpResultError`.

The abort check inspects the signal, not the caught error: our timer → logged
`RelayerHttpTimeoutError({ url, timeoutMs })`; otherwise → logged warn and `signal.reason`
unchanged (any value, even `null`).

| Cause | Thrown |
|---|---|
| our timer, any phase | `RelayerHttpTimeoutError` |
| caller abort, any phase | the caller's reason, unchanged |
| network failure, body read failure, oversized or non-JSON body | `RelayerHttpFetchError` |
| non-ok relayer-http error body / other non-ok JSON | `RelayerHttpResponseError` / `RelayerHttpUnexpectedResponseError` |
| ok, schema mismatch | `RelayerHttpResultError` |
| invalid constructor input / second `run()` | `InvalidPropertyError` or `Error` / `Error` |

### 5.4 `fetchWithRetry` and `abortableSleep`

- `fetchWithRetry`: `retries = clamp(retries ?? 3, 0, 1000)`, `delay = max(100, delay ?? 1000)`.
  Each attempt: `signal.throwIfAborted()`, then `fetch`. On a rejection: aborted signal → its
  reason; a rejection named `AbortError` → rethrown; else warn, `abortableSleep`, retry. Retries
  network failures only (every HTTP response is returned). The counter is per call.
- `abortableSleep(ms, signal)`: already aborted → the bare reason, thrown. At the time, an abort
  during the sleep rejected with a wrapper `Error('Aborted', { cause: reason })` named after the
  reason (since changed to the raw reason). Timer and listener are cleaned on both paths.

### 5.5 `RelayerHttpResponseError`

Getters `url`, `status`, `code`, `responseMessage`, `requestId`, `isRetryable`
(`status ∈ {429, 502, 503, 504}`). `RelayerHttpErrorCode` = known codes `| (string & {})`; known:
`malformed`, `not_found`, `method_not_allowed`, `unsupported_attestation_type`,
`sender_authentication_failed`, `auth_expired`, `acl_denied`, `user_signature_rejected`,
`user_decryption_expired`, `ciphertext_not_found`, `kms_context_destroyed`, `kms_context_invalid`,
`unprocessable`, `rate_limited`, `copro_consensus_failed`, `upstream_transient`, `overloaded`,
`timeout`, `internal`, `shutting_down`.

### 5.6 Server side (relayer-http and kms-connector, read-only)

relayer-http's error body (`crates/relayer-http/src/endpoint/error.rs`, `ErrorBody`) is exactly
`{ code, message, requestId }`: no `retryable` field, no `Retry-After` header; every response has
`x-request-id`.

| Code | Status | Origin | kms-connector `retryable()` |
|---|---|---|---|
| `malformed` | 400 | relayer-http (invalid request) or kms-connector | false |
| `not_found` / `method_not_allowed` | 404 / 405 | relayer-http (route, method) | — |
| `internal` | 500 | relayer-http | — |
| `shutting_down` | 503 | relayer-http (aggregation cancelled) | — |
| `timeout` | 504 | relayer-http (KMS aggregation deadline) or kms-connector | true |
| `unsupported_attestation_type` | 400 | kms-connector | false |
| `sender_authentication_failed` | 401 | kms-connector | false |
| `acl_denied` / `user_signature_rejected` | 403 | kms-connector | true |
| `ciphertext_not_found` | 404 | kms-connector | true |
| `kms_context_destroyed` | 410 | kms-connector | false |
| `kms_context_invalid` | 412 | kms-connector | true |
| `unprocessable` | 422 | kms-connector | false |
| `rate_limited` | 429 | kms-connector | true |
| `copro_consensus_failed` / `upstream_transient` | 502 | kms-connector | true |
| `overloaded` | 503 | kms-connector | true |
| `unknown` | 500 | kms-connector | true |

Below the KMS threshold, relayer-http returns the dominant kms-connector code with that code's
status (`kms-connector/crates/api/src/error.rs`, `ErrorCode::http_status`), or `upstream_transient`
/ 502 when none dominates. relayer-http already retries its KMS calls (`kms_aggregator/call.rs`,
`config.rs`): retryable errors only, in two budgets: linear waits `k·delay` for transient errors
(default 3 retries of 200 ms) and `max(initial_delay, Retry-After)·2^(k-1)` for a rate limit (default
one retry after 1 s), never past the aggregation deadline.

### 5.7 Tests touching the retried branch (at the time)

`errors.test.ts`: a 403 relayer-http error body keeps its fields; a 502 with body `null` is a
`RelayerHttpUnexpectedResponseError` (not the retried branch). `signal.test.ts`: `errorBody()`
(`acl_denied`, 403) in the listener-cleanup tests on a non-abort failure, plus abort-classification
tests for every phase and `options.timeout` validation. Tests mock `global.fetch` with `vi.fn()`
returning `new Response(…)`.

## 6. Original building blocks

A pure module-level decision
`_relayerHttpRetryDecision({ error, attempt }) → { retry: false } | { retry: true, delayMs }`
(location superseded by R11); a wait `abortableSleep(delayMs, signal)` that classifies a failure
with the abort check `' while waiting to retry'`; one timer per run, plus `remainingMs()` for S15;
logging and tests as S20–S26.

## 7. Revision log

| Date | Change | Source |
|---|---|---|
| 2026-11-01 | Two-class scenario (§1) | user |
| 2026-11-01 | R9 type visibility, minimal real public types | user |
| 2026-11-01 | R10 no retry options for now | user |
| 2026-11-01 | R11 single decision method in `RelayerHttpRequest`; supersedes the module-level decision of §6 | user |
| 2026-11-01 | R13 decision overridable; policy deferred, with S18, S19 and the policy part of S22 | user |
| 2026-11-01 | R14 never-retry default; R15 ceiling of 5 HTTP attempts; R16 body reader unchanged (rulings U1–U3) | user |
| 2026-11-03 | Requirements and plan merged into this file | user |
