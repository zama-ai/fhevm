# RelayerHttpRequest HTTP-level retry — merged design (r5)

Requirements: `requirements-relayer-http-request.md` in this folder — R1–R16 in its §2, S1–S26 in
its §3 (S18, S19 and the policy part of S22 deferred by R13), the known gaps G1–G9 in its §4.
Paths relative to `sdk/js-sdk/`. Signatures only, no implementation (R12).

Provenance: consensus of five design rounds by three independent agents, then a merge with the
owner's later ruling "an abort before settlement always wins" (§1, §4.4, §5, §6, §7, §9–§11).
The three points left open (U1–U3) were ruled by the owner and recorded as R14–R16. This document
is reconciled with the code: where they differed, the code won. It also covers the fixes made
after the consensus (an already-aborted caller signal, the raw abort reason from `abortableSleep`,
blank error fields, empty result values).

## 1. Summary

| Item | Choice |
|---|---|
| Public class | `RelayerHttpRequest<O>` — constructor checks, one-shot `run()`, lifetime (combined signal, timer, caller listener, cleanup), init built once, loop wiring, **the exit check (abort wins)**, **the overridable decision method**. |
| Internal class | `RelayerHttpAttempt<O>` — one fetch (network retries) → read → size check → parse → status/shape → validate. Stateless, owns nothing. |
| Shared modules | `relayerHttpSignal.ts` (combined signal, abort classifier, `MAX_TIMEOUT_MS`); `relayerHttpRetryLoop.ts` (loop, guards, wait, logs). Neither knows fetch or a class. |
| Decision (R11, R13) | `protected _retryDecision(error: RelayerHttpResponseError, attempt: number): number \| undefined`; overridden by subclassing; called through `this`. |
| Default decision (R14) | **never retry** (`undefined`). |
| Loop-enforced contract | S16 (abort + type filter), S17 (delay range, fail closed), S15 (fit in remaining time), R15 ceiling `MAX_HTTP_ATTEMPTS_CEILING = 5`. Guards veto retries, never create them. |
| Abort wins | If the combined signal is aborted before `run()` settles, `run()` rejects with the abort, whatever `_buildInit` or the loop produced (§6). |
| Policy | none (R13). No policy constants, no option type (R10). |
| Body reader (R16) | unchanged (`response.text()`); bounded streaming reader is a separate follow-up. |

## 2. Files

All in `src/core/modules/relayer/module/`. Import graph (acyclic): `RelayerHttpRequest.ts` →
{`RelayerHttpAttempt.ts`, `relayerHttpRetryLoop.ts`, `relayerHttpSignal.ts`, `relayerUrl.ts`};
`RelayerHttpAttempt.ts` → {`relayerHttpSignal.ts`, `relayerUrl.ts`, `guards/RelayerUserDecryptSucceeded.ts`};
`relayerHttpRetryLoop.ts` → `relayerHttpSignal.ts`. Every file starts with an `import`
(leading-comment check).

| File | Contents |
|---|---|
| `RelayerHttpRequest.ts` | `RelayerHttpRequest`, `RelayerHttpRequestParams`, `_isAbortOutcome`, `_assertRelayerHttpOperation`, `_validateTimeoutMs` |
| `RelayerHttpAttempt.ts` | `RelayerHttpOperation`, `RelayerHttpAttempt`, `RelayerHttpAttemptParams`, `MAX_BODY_TEXT_LENGTH`, `BODY_EXCERPT_LENGTH`, `_bodyExcerpt`, body guards |
| `relayerHttpSignal.ts` | `MAX_TIMEOUT_MS`, `RelayerHttpSignal`, `createRelayerHttpSignal`, `throwIfRelayerHttpAborted`, `_addAbortListener`, `_remainingMs` |
| `relayerHttpRetryLoop.ts` | `RelayerHttpRetryLoopParams`, `runRelayerHttpRetryLoop`, `waitBeforeHttpRetry`, `MAX_HTTP_ATTEMPTS_CEILING`, `_retryDelay`, `_isValidRetryDelay`, `_fitsRemaining`, `_logRetry`, `_logResponseError` |
| `relayerUrl.ts` (used) | `normalizeRelayerQuery` (constructor), `appendRelayerQuery` (attempt) |
| `test/RelayerHttpRequest.*.test.ts` | the tests (§8) |
| `fetch*NoGateway.ts`, `base/fetch.ts`, `base/timeout.ts`, `errors/*`, `types/relayer.ts`, entry points | callers and shared code, outside this design |
| `src/_types/**` | generated, gitignored; not touched |

Naming: `_`-prefix marks module-private helpers (R8); cross-module fake-public exports are
unprefixed and carry a `/** @internal */` doc marker (marker only, no build change).

## 3. Type visibility (R9)

No `RelayerHttp*` symbol is exported from a package entry point (`module/index.ts` exports only
the `relayerModule` factory, which exposes the `fetch*` functions).

| Symbol | Class | Reason |
|---|---|---|
| `RelayerHttpRequest<O>` | real public | Created and run by both root callers. Has one `protected` member (T1). |
| `RelayerHttpRequestParams<O>` (unexported alias) | real public | Built by the root callers without naming it. |
| `RelayerHttpOperation` (exported from `RelayerHttpAttempt.ts`) | real public | Literal supplied by root callers; in the lower file to avoid a cycle. |
| `FetchResultOf`, `Relayer*Options`, `Logger`, `Auth`, error classes | real public (existing) | untouched |
| `RelayerHttpAttempt<O>`, `RelayerHttpAttemptParams<O>` | fake public | Cross-module (owner) + `test/RelayerHttpRequest.attempt.test.ts`; never from an entry point (S1). |
| `RelayerHttpSignal`, `createRelayerHttpSignal`, `throwIfRelayerHttpAborted`, `MAX_TIMEOUT_MS` | fake public | Cross-module (owner, attempt, loop) + `test/RelayerHttpRequest.combinedSignal.test.ts`. |
| `RelayerHttpRetryLoopParams<T>`, `runRelayerHttpRetryLoop`, `waitBeforeHttpRetry` | fake public | Built/called by the owner and by `test/RelayerHttpRequest.retryLoop.test.ts` (loop and wait tested alone). |
| `_isAbortOutcome`, `_retryDelay`, `_isValidRetryDelay`, `_fitsRemaining`, `_logRetry`, `_logResponseError`, `_assertRelayerHttpOperation`, `_validateTimeoutMs`, `_addAbortListener`, `_remainingMs`, `_bodyExcerpt`, body guards | private | Module-file helpers, tested through their module's unit. |
| `MAX_HTTP_ATTEMPTS_CEILING`, `MAX_BODY_TEXT_LENGTH`, `BODY_EXCERPT_LENGTH` | private | Tests assert values, not names. |

Real public types added: **zero**. The decision's signature uses only existing types
(`RelayerHttpResponseError`, `number`, `undefined`); no decision type is exported.

## 4. Interfaces (R12)

No bodies. Existing types referenced by name.

### 4.1 `relayerHttpSignal.ts`

```ts
// setTimeout's largest delay (2^31 - 1 ms); bounds options.timeout and any retry delay (S17).
// Node turns a larger delay into 1 ms.
export const MAX_TIMEOUT_MS = 2_147_483_647;

/** @internal The run's combined signal; created and cleaned up only by RelayerHttpRequest (S11). */
export type RelayerHttpSignal = {
  readonly signal: AbortSignal;                        // the only internal signal of the run
  readonly timedOutAfterMs: () => number | undefined;  // defined only when OUR timer fired
  readonly remainingMs: () => number | undefined;      // monotonic (performance.now); undefined without timeout; >= 0
  readonly cleanup: () => void;                        // clears timer + caller listener; idempotent
};

// One AbortController. Forwards the caller's abort (or copies its prior abort). Notes performance.now()
// and arms our timer. Our timer aborts with DOMException('Timed out', 'TimeoutError') and is a no-op if
// the signal is already aborted, so the first abort wins. When the caller listener or our timer aborts
// the signal, it releases the timer and the caller listener at once (same idempotent function as
// cleanup), so a fetch that never settles cannot keep them alive. A caller signal already aborted
// makes our signal dead from the start: no listener is added and no timer is armed.
export function createRelayerHttpSignal(callerSignal: AbortSignal | undefined, timeoutMs: number | undefined): RelayerHttpSignal;

// Returns if not aborted. Our timer → warn "[RelayerHttpRequest] Request to {url} timed out{during}."
// + RelayerHttpTimeoutError({ url, timeoutMs }). Else warn "[RelayerHttpRequest] Request aborted on
// {url}{during}." + signal.reason unchanged (any value, even null).
export function throwIfRelayerHttpAborted(args: {
  readonly signal: AbortSignal;
  readonly timedOutAfterMs: () => number | undefined;
  readonly url: string;
  readonly logger: Logger | undefined;
  readonly during: string;   // see the phase suffixes in §7
}): void;

// One-shot 'abort' listener ({ once: true }); returns the closure that removes it.
function _addAbortListener(signal: AbortSignal, listener: () => void): () => void;

// max(0, timeoutMs - (performance.now() - startedAtMs)), or undefined without a timer.
function _remainingMs(timeoutMs: number | undefined, startedAtMs: number): number | undefined;
```

### 4.2 `RelayerHttpAttempt.ts`

```ts
// The relayer-http routes this client targets.
export type RelayerHttpOperation = Extract<RelayerPostOperation, 'USER_DECRYPT' | 'PUBLIC_DECRYPT'>;

// Largest response body parsed, in UTF-16 code units (not bytes); checked after reading, before JSON.parse (R16).
const MAX_BODY_TEXT_LENGTH = 10 * 1024 * 1024;

// Largest body excerpt kept in an error, in characters.
const BODY_EXCERPT_LENGTH = 500;

/** @internal Already-validated values from the owner; no raw options, payload or auth (S2). */
export type RelayerHttpAttemptParams<O extends RelayerHttpOperation> = {
  readonly relayerOperation: O;
  readonly url: string;                                          // query-less: errors and logs
  readonly query: Readonly<Record<string, string>> | undefined;  // appended inside the fetch try
  readonly init: Readonly<Omit<RequestInit, 'signal'>>;         // built once by the owner; string body
  readonly signal: AbortSignal;                                  // attached after spreading init: one signal by construction
  readonly timedOutAfterMs: () => number | undefined;
  readonly fetchRetries: number | undefined;
  readonly fetchRetryDelayInMilliseconds: number | undefined;
  readonly logger: Logger | undefined;
};

/** @internal One HTTP attempt. No timer, listener, cleanup or one-shot guard (S3). */
export class RelayerHttpAttempt<O extends RelayerHttpOperation> {
  private readonly _params: RelayerHttpAttemptParams<O>;

  // Stores the params; no validation, no resources.
  constructor(params: RelayerHttpAttemptParams<O>);

  // fetch → read → size check → parse → status/shape → validate. Throws the types listed in §4.2.1.
  // The relayer-http error branch is thrown unlogged (the loop logs retry or stop).
  // No abort check between the read and the parse, by design: run()'s exit check replaces the outcome
  // anyway; the cost is a wasted parse and, for a non-relayer body, one extra error line before the warn.
  public run(): Promise<FetchResultOf<O>>;

  // fetchWithRetry({ url: appendRelayerQuery(url, query), init: { ...init, signal }, retries, retryDelayMs, logger });
  // undefined retries/delay fall back to fetchWithRetry's defaults (3 retries, 1000 ms); it clamps retries
  // to [0, 1000] and the delay to >= 100 ms. It retries any fetch rejection except an abort (signal aborted,
  // or a rejection named 'AbortError'); it never retries on a status, and the body read is outside it.
  // On throw: _throwIfAborted(''), else logged RelayerHttpFetchError (network error).
  private _fetch(): Promise<Response>;

  // response.text() (unchanged, R16; carries a TODO(SECURITY) for the unbounded read).
  // On throw: _throwIfAborted(' while reading the response body'), else logged RelayerHttpFetchError.
  private _readBody(response: Response): Promise<string>;

  // MAX_BODY_TEXT_LENGTH check, then JSON.parse; each failure → logged RelayerHttpFetchError.
  // A non-JSON body is a RelayerHttpFetchError whatever the status (e.g. a proxy's 502 HTML page).
  private _parseBody(response: Response, bodyText: string): unknown;

  // Non-ok: throws RelayerHttpResponseError for a relayer-http body (unlogged), else RelayerHttpUnexpectedResponseError (logged).
  private _throwNonOk(response: Response, bodyText: string, bodyJson: unknown): never;

  // Ok: per-operation success assertion → result, or throws _resultError. `default:` = `relayerOperation satisfies never`
  // (compile-time exhaustiveness) plus a defensive runtime throw.
  private _validateResult(bodyJson: unknown, bodyText: string): FetchResultOf<O>;

  // Logs a schema mismatch and wraps the guard's error in RelayerHttpResultError (bodyJson = raw-text excerpt).
  private _resultError(cause: unknown, bodyText: string): RelayerHttpResultError;

  // throwIfRelayerHttpAborted with this attempt's signal, timedOutAfterMs, url, logger (S5).
  private _throwIfAborted(during: string): void;
}

// The first BODY_EXCERPT_LENGTH characters as a real copy, so an error never keeps the whole body alive.
function _bodyExcerpt(bodyText: string): string;

// relayer-http's error body: non-blank `code` and `message` strings (isNonBlankString: a whitespace-only
// string is rejected; the value itself is not trimmed) and a UUIDv4 `requestId`. Codes are an open set.
function _isRelayerHttpErrorBody(value: unknown): value is { code: string; message: string; requestId: string };
// Success envelope: status 'succeeded' and a UUIDv4 requestId.
function _assertIsRelayerHttpSucceeded(value: unknown, name: string): asserts value is { status: 'succeeded'; requestId: string };
// Envelope + non-empty result.result; per share: payload (hex, no 0x, at least 1 byte), 65-byte signature
// (hex, no 0x), required extraData (hex, 0x).
function _assertIsRelayerHttpUserDecryptResult(value: unknown, name: string): asserts value is { result: { result: FetchUserDecryptResult } };
// Envelope + decryptedValue (hex, no 0x, at least 1 byte), non-empty 65-byte signatures (hex, no 0x),
// extraData (hex, 0x). An empty payload or decryptedValue is rejected (minByteLength: 1) so a malformed body
// fails as a RelayerHttpResultError, not later at signature verification or share deserialization.
// extraData may be '0x': it is the KMS-signed form of v0 extra data. Extra fields are not rejected;
// no signature or ABI check here.
function _assertIsRelayerHttpPublicDecryptResult(value: unknown, name: string): asserts value is { result: FetchPublicDecryptResult };
```

#### 4.2.1 Error model of one attempt

| Thrown | When | Logged by the attempt |
|---|---|---|
| `RelayerHttpFetchError` | network failure after `fetchWithRetry`'s own retries (also a `url` that already holds `?`: the `appendRelayerQuery` assert runs inside the fetch `try`); unreadable body; body over `MAX_BODY_TEXT_LENGTH`; non-JSON body (any status) | yes (`error`) |
| `RelayerHttpResponseError` | non-ok status with relayer-http's `{ code, message, requestId }` body | **no** (the loop logs) |
| `RelayerHttpUnexpectedResponseError` | non-ok status with any other JSON body (proxy, gateway, load balancer) | yes (`error`) |
| `RelayerHttpResultError` | ok status, result fails its schema assertion | yes (`error`) |
| `RelayerHttpTimeoutError` / caller's reason | the signal was aborted when `fetch`, the network-retry delay or `response.text()` threw | `warn` |

Errors keep the url, status, content type, message and a body excerpt of at most 500
characters, plus the underlying `cause` error where there is one (`RelayerHttpFetchError`,
`RelayerHttpResultError`). They never keep the `Response`, the init, the payload or a signal.

Follow-up (R16, not this change): a bounded streaming reader plugs in behind `_readBody` only;
no other signature changes.

### 4.3 `relayerHttpRetryLoop.ts`

```ts
// R15 safety ceiling, not a policy cap: the loop never starts attempt 6, whatever `decide` says.
// A later policy's cap must be <= this value, or raise it deliberately.
const MAX_HTTP_ATTEMPTS_CEILING = 5;

/** @internal Everything the loop needs; no fetch, no class. */
export type RelayerHttpRetryLoopParams<T> = {
  readonly runAttempt: () => Promise<T>;                                                     // one fresh attempt per call
  readonly decide: (error: RelayerHttpResponseError, attempt: number) => number | undefined; // the R11 method, bound by the owner
  readonly signal: AbortSignal;
  readonly timedOutAfterMs: () => number | undefined;
  readonly remainingMs: () => number | undefined;
  readonly url: string;
  readonly logger: Logger | undefined;
};

/**
 * @internal Runs attempts until one returns or a throw is final. The catch covers only runAttempt.
 * Steps: 1. run one attempt; 2. signal.aborted or not a RelayerHttpResponseError → rethrow unchanged;
 * 3. delay = _retryDelay(...); undefined → _logResponseError + rethrow the same instance (S21);
 * else _logRetry, waitBeforeHttpRetry, next attempt.
 */
export function runRelayerHttpRetryLoop<T>(params: RelayerHttpRetryLoopParams<T>): Promise<T>;

// Guards 3, 4, 6, 7 of §5 in order: attempt >= MAX_HTTP_ATTEMPTS_CEILING → undefined (decide not called);
// delay = decide(error, attempt), typed unknown (a throw propagates); !_isValidRetryDelay → undefined;
// !_fitsRemaining(delay, remainingMs()) → undefined; else the delay.
function _retryDelay<T>(params: RelayerHttpRetryLoopParams<T>, error: RelayerHttpResponseError, attempt: number): number | undefined;

/**
 * @internal abortableSleep(delayMs, signal); on rejection throwIfRelayerHttpAborted(' while waiting to retry'),
 * then rethrow the caught value unchanged if the signal is not aborted (never swallowed, never re-wrapped).
 * abortableSleep rejects with the raw abort reason; the wait still classifies from the signal, so our
 * own timeout becomes RelayerHttpTimeoutError. Exported for direct S13/S24 tests.
 */
export function waitBeforeHttpRetry(args: {
  readonly delayMs: number;
  readonly signal: AbortSignal;
  readonly timedOutAfterMs: () => number | undefined;
  readonly url: string;
  readonly logger: Logger | undefined;
}): Promise<void>;

// S17: a number that is an integer in [1, MAX_TIMEOUT_MS] (also rejects a wrong runtime type). Else fail closed.
function _isValidRetryDelay(value: unknown): value is number;

// S15: true when no timeout is set or delayMs < remainingMs.
function _fitsRemaining(delayMs: number, remainingMs: number | undefined): boolean;

// S20, one warn per accepted wait (text in §7). No "/max": the cap is policy.
function _logRetry<T>(params: RelayerHttpRetryLoopParams<T>, error: RelayerHttpResponseError, attempt: number, delayMs: number): void;

// S21: the "[RelayerHttpRequest] relayer-http error on …" error line (cause undefined), once at stop.
function _logResponseError<T>(params: RelayerHttpRetryLoopParams<T>, error: RelayerHttpResponseError): void;
```

### 4.4 `RelayerHttpRequest.ts`

```ts
import type { RelayerHttpOperation } from './RelayerHttpAttempt.js';

// What the two root callers pass to `new` (real public, unexported alias). onProgress is ignored.
type RelayerHttpRequestParams<O extends RelayerHttpOperation> = {
  relayerOperation: O;
  url: string;
  payload: Record<string, unknown>;
  options?: Omit<RelayerUserDecryptOptions, 'onProgress'> | Omit<RelayerPublicDecryptOptions, 'onProgress'> | undefined;
  logger?: Logger | undefined;
};

export class RelayerHttpRequest<O extends RelayerHttpOperation> {
  private readonly _relayerOperation: O;
  private readonly _url: string;
  private readonly _payload: Record<string, unknown>;
  private readonly _auth: Auth | undefined;
  private readonly _headers: Readonly<Record<string, string>>;
  private readonly _query: Readonly<Record<string, string>> | undefined;
  private readonly _signal: AbortSignal | undefined;
  private readonly _timeoutMs: number | undefined;
  private readonly _fetchRetries: number | undefined;
  private readonly _fetchRetryDelayInMilliseconds: number | undefined;
  private readonly _logger: Logger | undefined;
  private _hasRun: boolean;

  // Checks: _assertRelayerHttpOperation, normalizeHeaders, normalizeRelayerQuery, _validateTimeoutMs;
  // throws at `new` (S7). Stores the normalized values. `options.signal: null` is stored as undefined.
  constructor(params: RelayerHttpRequestParams<O>);

  // 1. One-shot (S8): a second call throws Error('RelayerHttpRequest.run() was already called on this
  //    instance. Create a new instance per request.').
  //    Then, if the caller signal is already aborted: throwIfRelayerHttpAborted on it with
  //    during ' before the request was sent' — no timer, no listener, no init, no fetch.
  // 2. combined = createRelayerHttpSignal(signal, timeoutMs), once.
  // 3. try { 3.1 init = _buildInit(); 3.2 result = await runRelayerHttpRetryLoop({
  //      runAttempt: () => this._createAttempt(init, combined).run(), decide: (e, a) => this._retryDecision(e, a),
  //      signal, timedOutAfterMs, remainingMs, url, logger });
  // 4.   _throwIfAbortedAtExit(combined, during); return result }
  // 5. catch (error) { if (!_isAbortOutcome(error, signal)) _throwIfAbortedAtExit(combined, during); throw error }
  //    finally { combined.cleanup() } (S9–S11).
  // `during` is ' before the request was sent' until the init is built, then ' after the response was received'.
  // The loop is not raced against the signal: a fetch that ignores the abort and never settles leaves run()
  // pending (the timer and listener are still released, §4.1).
  public run(): Promise<FetchResultOf<O>>;

  /**
   * R11/R13: the single retry decision, called after each relayer-http error response.
   * `attempt` is the 1-based number of HTTP attempts already made.
   * Returns the delay in ms before the next attempt (integer in [1, MAX_TIMEOUT_MS]) or undefined to stop.
   * Override in a subclass. Default: never retry (R14).
   */
  protected _retryDecision(_error: RelayerHttpResponseError, _attempt: number): number | undefined;

  // The exit check: throwIfRelayerHttpAborted on the combined signal with the given `during`.
  private _throwIfAbortedAtExit(combined: RelayerHttpSignal, during: string): void;

  // { method: 'POST', headers: normalized headers + content-type + zama-sdk-version + zama-sdk-name,
  //   body: JSON.stringify(payload) once }, then setAuth. Inside run()'s try (S10): JSON.stringify
  //   (e.g. a BigInt) and setAuth (an invalid header value) can throw, and cleanup must still run.
  //   The generated headers override same-named custom headers; setAuth runs last.
  private _buildInit(): Readonly<Omit<RequestInit, 'signal'>>;

  // One fresh RelayerHttpAttempt from the stored values, the shared init and the combined signal (S2, S12).
  private _createAttempt(init: Readonly<Omit<RequestInit, 'signal'>>, combined: RelayerHttpSignal): RelayerHttpAttempt<O>;
}

// True when the signal is aborted and `error` already is its classified abort:
// Object.is(error, signal.reason) (a NaN reason matches itself) or error instanceof RelayerHttpTimeoutError.
function _isAbortOutcome(error: unknown, signal: AbortSignal): boolean;

// Throws InvalidPropertyError (subject 'RelayerHttpRequestParams', property 'relayerOperation')
// unless value is 'USER_DECRYPT' | 'PUBLIC_DECRYPT'.
function _assertRelayerHttpOperation(value: unknown): asserts value is RelayerHttpOperation;

// undefined, or an integer in [1, MAX_TIMEOUT_MS], else InvalidPropertyError (subject 'options', property 'timeout').
function _validateTimeoutMs(timeout: number | undefined): number | undefined;
```

### 4.5 Test override (test-local)

```ts
// Built by a test-local factory from a finite script/spy; inherits the constructor unchanged.
class ScriptedRequest<O extends RelayerHttpOperation> extends RelayerHttpRequest<O> {
  protected override _retryDecision(error: RelayerHttpResponseError, attempt: number): number | undefined;
}
```

## 5. Decision: mechanism, default, contract (R11, R13, R14, R15)

**Mechanism.** `_retryDecision` is a `protected` instance method; the loop receives
`(e, a) => this._retryDecision(e, a)`, so virtual dispatch makes a subclass override the one
used. No constructor parameter (would add a real public function type and look like a retry
option, R9/R10), no static (not overridable per instance), no mutable hook (global state).

**Default (R14): never retry (`return undefined`).** One attempt; the same
`RelayerHttpResponseError` instance is thrown; one error line (R1). Zero retries, zero waits
(R5). It encodes no policy (R13); the policy task replaces this body only. Neither the default
nor the loop reads `RelayerHttpResponseError.isRetryable` (true for 429, 502, 503, 504).

**Contract enforced by the loop on every decision (default or override):**

| Order | Guard | Rule | On violation / stop |
|---|---|---|---|
| 1 | `signal.aborted` | S16 | rethrow the caught value; `decide` not called. `run()`'s exit check then replaces it with the abort unless it already is the abort (§6). |
| 2 | not a `RelayerHttpResponseError` | R1, S16 | rethrow unchanged; `decide` not called |
| 3 | `attempt >= MAX_HTTP_ATTEMPTS_CEILING` | R15 | stop; `decide` not called |
| 4 | `decide` throws | — | the thrown value propagates; `cleanup` runs in `run()`; not an attempt failure. If the signal is aborted by then, the abort wins (§6). |
| 5 | `undefined` | S16 | stop |
| 6 | `!_isValidRetryDelay(value)` | S17 | fail closed: stop (no coercion, no wait, caller-facing error type unchanged) |
| 7 | `!_fitsRemaining(delay, remainingMs())` | S15, R3 | stop, no wait; the `RelayerHttpResponseError`, unless the signal is aborted before settlement (§6) |

Guards 1–2 are one `if` in `runRelayerHttpRetryLoop`; guards 3–7 run inside `_retryDelay`, where
guards 5 and 6 are one check (`undefined` fails `_isValidRetryDelay`).

"Stop" = `_logResponseError` once, then rethrow the same instance (S21). If the signal is
aborted before `run()` settles, the exit check then turns the stop into the abort (§6). Only
`_retryDecision` can produce a retry and its delay; every guard only vetoes (R11).

## 6. Abort and timeout

### 6.1 Errors a run can reject with

| Error | When |
|---|---|
| `RelayerHttpFetchError` | network failure (after `fetchWithRetry`'s own retries), unreadable body, body over `MAX_BODY_TEXT_LENGTH`, non-JSON body, a `url` that already holds a `?` (`appendRelayerQuery` assert, inside the fetch try) |
| `RelayerHttpResponseError` | non-ok with relayer-http's `{ code, message, requestId }` body; the only type the decision sees |
| `RelayerHttpUnexpectedResponseError` | non-ok JSON that is not relayer-http's error body (proxy, gateway, load balancer) |
| `RelayerHttpResultError` | ok status, wrong result shape |
| `RelayerHttpTimeoutError({ url, timeoutMs })` | our timer aborted the run |
| the caller's reason, unchanged | the caller aborted (any value, even `null` or `NaN`) |
| what `decide` or `_buildInit` threw | unless the abort wins (a logger hook never throws out, §7.1) |
| `Error('RelayerHttpRequest.run() was already called…')` | second `run()` on the same instance (S8) |

Constructor errors (`InvalidPropertyError` for the operation or `options.timeout`; `Error` from
`normalizeHeaders` or `normalizeRelayerQuery`) are thrown at `new`, never by `run()`.

### 6.2 Rule: an abort before settlement wins

If the combined signal (caller signal or our timer) is aborted before `run()` settles, `run()`
rejects with the abort, whatever `_buildInit` or the loop produced. Caller abort → the caller's reason;
our timer → `RelayerHttpTimeoutError`. If both, the first abort of the combined signal wins.
"Before settlement" means before the exit check; an abort after `run()` settled changes nothing.

Why: it matches the platform (`fetch()` and `response.text()` reject with the abort reason);
a caller who aborted no longer wants the outcome; it removes timing-dependent outcomes; and
R3 holds even when a `fetch` that ignores the signal settles after the deadline.

Mechanism: one exit check in `run()` after the loop settles (steps 4 and 5, §4.4), through
`throwIfRelayerHttpAborted`. An outcome that already is the abort (`_isAbortOutcome`: the
reason itself, or a `RelayerHttpTimeoutError`) passes through: same instance, no second warn.
The per-step checks stay: they end the run early and give the phase-specific warn text.

Accepted trade-off: the abort also replaces an error raised in the same window (a throwing
`_retryDecision` or `_buildInit`). The abort's warn line is still written.

Logger hooks cannot break this rule: every logger call is guarded (§7.1). `throwIfRelayerHttpAborted`
calls `warn` before it throws; if that hook throws, its error is swallowed and the abort (the
caller's reason or `RelayerHttpTimeoutError`) is still thrown, in the per-step checks and in the
exit check alike.

### 6.3 Where an abort is detected

| When | Detected by | Thrown |
|---|---|---|
| caller already aborted | `run()` right after the one-shot guard, `during` = ` before the request was sent`; nothing is armed or built | caller's reason |
| fetch / network-retry delay / body read | attempt `_throwIfAborted`, classifying from the signal (the network-retry sleep rejects with the raw reason) | our timer → `RelayerHttpTimeoutError`; caller → reason |
| wait between HTTP attempts | `waitBeforeHttpRetry` → `throwIfRelayerHttpAborted` | same classification (S13) |
| between wait end and next fetch | next attempt's `fetchWithRetry` `throwIfAborted()` (S14) | same |
| delay `>=` remaining | guard 7 | current `RelayerHttpResponseError`, no wait (S15), unless the signal is aborted before settlement |
| `_buildInit` throws on a signal aborted after `run()` started | exit check, `during` = ` before the request was sent` | the abort |
| abort after the response arrived, before settlement (result or error in hand) | exit check, `during` = ` after the response was received` | the abort |
| abort from a hook while no response arrived (e.g. a logger `error` hook during a network failure) | exit check, same suffix: it is set as soon as the init is built, so it names the phase, not proof of a response | the abort |
| abort after `run()` settled | — | nothing changes |

Our timer is a `setTimeout` callback. With real timers it cannot run between guard 7 (or any
stop) and the exit check, where only microtasks run, so a slow synchronous `decide` ends with
the response error, not the timeout. A synchronous caller abort (from `decide`, a logger hook or
a listener) is seen at once. A test hook that advances fake timers past the deadline fires our
timer synchronously, and the exit check then sees it.

Ownership (S11): one `AbortController`, one timer, one caller listener, all from
`createRelayerHttpSignal` called once in `run()`; one `cleanup()` in `run()`'s `finally`. The
timer and listener are also released as soon as the caller listener or our timer aborts the
signal; a caller signal already aborted gets neither (and `run()` exits before creating them). Attempts get `signal` +
`timedOutAfterMs` only (S3, S12). `abortableSleep`'s own short timer and listener are not
global resources and are cleaned by it. `remainingMs` is monotonic (`performance.now()` at
arming); vitest 4.1.9 fakes `performance` by default with `useFakeTimers`. Reachability (T4):
because S15 admits a wait only when `delay < remaining`, our timer fires during the wait only
if time elapses between the S15 check and the start of the sleep; the tests create that gap (§8).

## 7. Logging

| Event | Level | Where | Text |
|---|---|---|---|
| accepted retry wait | `warn` | `_logRetry` | `[RelayerHttpRequest] relayer-http error on ${url} (status=…, code=…, requestId=…): ${responseMessage}. Retrying in ${delayMs} ms (attempt ${attempt}).` |
| stop (undefined, ceiling, invalid delay, no fit) | `error` | `_logResponseError` | `[RelayerHttpRequest] relayer-http error on ${url} (status=…, code=…, requestId=…): ${responseMessage}`, `cause` undefined |
| our timer | `warn` | `throwIfRelayerHttpAborted` | `[RelayerHttpRequest] Request to ${url} timed out${during}.` |
| caller abort | `warn` | `throwIfRelayerHttpAborted` | `[RelayerHttpRequest] Request aborted on ${url}${during}.` |
| network failure | `error` | attempt `_fetch` | `[RelayerHttpRequest] Fetch failed on ${url}.` |
| body read failure | `error` | attempt `_readBody` | `[RelayerHttpRequest] Failed to read the response body on ${url}.` |
| body too large | `error` | attempt `_parseBody` | `[RelayerHttpRequest] Response body on ${url} is too large to parse (${n} characters, status=…).` |
| body not JSON | `error` | attempt `_parseBody` | `[RelayerHttpRequest] Response body on ${url} is not valid JSON (status=…).` |
| non-relayer-http non-ok | `error` | attempt `_throwNonOk` | `[RelayerHttpRequest] Unexpected non-relayer-http error response on ${url} (status=…).` |
| schema mismatch | `error` | attempt `_resultError` | `[RelayerHttpRequest] Response body on ${url} does not match the expected schema.` |
| network retry inside an attempt | `warn` | `fetchWithRetry` | `[fetchWithRetry] Fetch to ${_urlForLog(url)} failed (attempt i/n): …. Retrying.` |

Phase suffixes (`during`): `''` (fetch), ` while reading the response body`,
` while waiting to retry`, ` before the request was sent`, ` after the response was received`.

The retry warn means "a wait was accepted", not "another fetch will start". A relayer-http
error is logged once, by the loop. An outcome replaced by the abort ends with the abort's
warn line; a guard-1 rethrow of a response error is no longer an unlogged exit. An abort or
timeout writes one warn line: the exit check writes none when the outcome already is the
classified abort. No log line prints a query string: every `[RelayerHttpRequest]` line uses the
query-less url, and the `[fetchWithRetry]` line strips the query and fragment with `_urlForLog`
(`base/fetch.ts`), which falls back to `[unavailable URL]` if formatting fails.

### 7.1 Logger calls never throw

The logger is supplied by the caller and may throw (a broken transport, a full buffer, a
circular `cause`). A logging failure must never change a request's outcome. So every
`logger.warn` / `logger.error` call in the relayer-http path is wrapped at its call site in
`try { … } catch { /* empty */ }`:

- `RelayerHttpAttempt.ts`: the six `error` lines of `_fetch`, `_readBody`, `_parseBody`,
  `_throwNonOk` and `_resultError`;
- `relayerHttpRetryLoop.ts`: `_logRetry` (`warn`) and `_logResponseError` (`error`);
- `relayerHttpSignal.ts`: the two `warn` lines of `throwIfRelayerHttpAborted`;
- `base/fetch.ts`: the `[fetchWithRetry]` retry `warn`.

`RelayerHttpRequest.ts` makes no direct logger call. The wrap is inline at each call site (no
shared helper). A swallowed hook error leaves everything else unchanged:

- a retry `warn` that throws: the wait and the next attempt still happen;
- an `error` line that throws: the attempt or the loop still throws its own typed error;
- an abort `warn` that throws: the abort is still thrown (§6.2).

Tests assert the swallowing, never the propagation: `retryLoop`, `reentrancy`, `leaks`,
`attempt` ("with a throwing logger"), `combinedSignal` (`throwIfRelayerHttpAborted`) and
`base/fetch.test.ts`.

## 8. Tests (R4, S22–S26)

All in `test/`. Rules: fresh `Response` per `fetch` call (a body reads once); fake timers
wherever there is a wait (`performance` faked by default); retry paths run through
`ScriptedRequest` with finite scripts (test data, not policy).

| File | Unit | Scope |
|---|---|---|
| `RelayerHttpRequest.combinedSignal.test.ts` | `MAX_TIMEOUT_MS`, `createRelayerHttpSignal`, `throwIfRelayerHttpAborted` | pre-aborted caller; forwarding; our timer → `timedOutAfterMs`; first abort wins; `remainingMs` (incl. unaffected by `vi.setSystemTime`); `cleanup`; classification. |
| `RelayerHttpRequest.attempt.test.ts` | `RelayerHttpAttempt` with a synthetic signal | each error type, both operations; relayer-http error body unlogged; abort classification; no timer, no listener; body size limit. |
| `RelayerHttpRequest.retryLoop.test.ts` | `runRelayerHttpRetryLoop`, `waitBeforeHttpRetry` (stub `runAttempt`/`decide`, mocked `abortableSleep`) | guard order, ceiling, invalid delays, fit check, aborts and hooks around the wait. |
| `RelayerHttpRequest.retry.test.ts` | `ScriptedRequest.run()` + mocked `fetch` | default never retries (R14); override dispatch; retry sequences; timeouts during the wait. |
| `RelayerHttpRequest.reentrancy.test.ts` | `run()` | one-shot `run()`; `decide` overrides that re-enter, abort or throw; logger hooks that abort, throw, advance fake time or re-enter; a caller abort fired from inside `fetch`; two concurrent instances share no state. |
| `RelayerHttpRequest.phases.test.ts` | `run()` | abort in every phase, caller and our timer; caller vs our timer at the same instant; the caller's own `AbortSignal.timeout()`; timeout bounds (1 ms, `MAX_TIMEOUT_MS`, across attempts and waits, system-clock jump). |
| `RelayerHttpRequest.leaks.test.ts` | `run()` | no timer or listener left on any exit, incl. a hung `fetch` or body read that ignores the signal; a late caller abort after settlement; an expired caller `AbortSignal.timeout()`; many sequential and concurrent runs on one shared signal. |
| `RelayerHttpRequest.signal.test.ts` | `run()` | caller-signal cleanup, abort classification (incl. a `NaN` reason, `signal: null`, the ` before the request was sent` warn on a build throw), `options.timeout` validation. |
| `RelayerHttpRequest.errors/.userDecrypt/.publicDecrypt.test.ts` | `run()` | error responses and success bodies. |
| `RelayerHttpRequest.blackbox.test.ts` | `run()`, public API only | the caller-facing contract, written from the design; later guard tests (e.g. a reason with a throwing `name`). |
| `RelayerHttpRequest.blackbox.abortWins.test.ts` | `run()`, public API only | the abort-wins rule (§6.2). |

Cases the design requires (S22–S26): default `undefined` for every (status, code) pair and
attempts 1..5 (R14 contract); S23 retry then success, stop, script exhausted, type unchanged
after a retry; S24 caller string, `null`, caller `TimeoutError` and our timer during the wait
(the logger `warn` hook advances fake time after the S15 check but not past the deadline);
S25 listener removed once and no timer left after a retried success and failure; payload,
auth and options mutated during the wait → identical bodies and headers; S8 second `run()`
message. Abort wins: an abort after the response arrived, before settlement, wins over the
result and over each error type; an abort after settlement changes nothing.

Bug-first (R8): a test for a found bug fails before its fix.

## 9. Known gaps (requirements §4, G1–G9, and later fixes)

| Gap | Resolution |
|---|---|
| No bound without `options.timeout` | Default: 0 retries (R14). Overrides: ≤ 5 attempts (R15), each delay ≤ `MAX_TIMEOUT_MS` (S17). Tighter caps are policy (T3). |
| Undecided retry policy | Out of scope (R13). Default never retries (R14); the override seam is ready. |
| Error log on every retried attempt | Attempt leaves the relayer-http branch unlogged; loop logs `warn` per retry, `error` once at stop. |
| Fallback throw in the wait | Rethrow the caught value unchanged after the abort check; tested with a mocked `abortableSleep`. |
| Caller abort reason of type `RelayerHttpResponseError` | Guard 1, and `_isAbortOutcome` matches the reason by identity; tested. |
| Abort landing after the response arrived | Exit check in `run()` (§6.2): the abort wins. |
| `fetch` that ignores the abort and never settles | `run()` stays pending by design (native `fetch` never does this); timer and listener are released at the abort. |
| Rejection value `null` from `fetch` or `response.text()` | Fixed: `ensureError` wraps `null` and `undefined` like any other non-`Error` value (`base/errors/utils.ts`), so the caller gets a `RelayerHttpFetchError`. Tested in `errors.test.ts`. |
| Throwing logger hook | Fixed: every logger call is wrapped (§7.1); a hook error is swallowed and never changes the outcome. |
| Payload serialized per attempt | `_buildInit` once in `run()`'s `try`; mutation test. |
| Error keeping the whole body alive | `_bodyExcerpt` copies at most 500 characters. |
| Caller signal already aborted | `run()` exits at once with the reason; `createRelayerHttpSignal` arms no timer for it either. Tested in `signal` and `combinedSignal`. |
| Abort reason wrapped by `abortableSleep` | Fixed: it rejects with the raw reason, like `fetch`; `fetchWithRetry` now throws the raw reason whatever the abort phase. |
| Whitespace-only error `code` or `message` | Fixed: `isNonBlankString`; such a body is a `RelayerHttpUnexpectedResponseError`, never retried. |
| Empty share `payload` or `decryptedValue` | Fixed: `minByteLength: 1`; a `RelayerHttpResultError` instead of a late signature or deserialization failure. |
| Redirects | Not handled here: every fetch uses the default `redirect: 'follow'`. Open question (`'error'` or `'manual'`); a custom auth header could follow a cross-origin redirect. |
| Attempt one-shot guard + stale comment | Guard dropped (stateless); `default:` = `satisfies never` + defensive throw. |
| Loop testable only through fetch | Injected `runAttempt`/`decide`; loop tested alone. |

## 10. Tensions (accepted)

| # | Items | Tension | Resolution |
|---|---|---|---|
| T1 | R13 vs "surface stays plan §2.2" / R9 | Overriding needs a `protected` member, visible in the declaration. | `protected _retryDecision`; zero new real public types. R13 wins. |
| T2 | R13 (cap is policy) vs R5 | An override could retry forever. | R15: loop ceiling of 5, veto-only; the policy's cap must be `<=` it. |
| T3 | R5 (no unbounded waits) vs S17 as the only delay bound | Up to `4 × MAX_TIMEOUT_MS` of waiting for an override without `options.timeout`. | Bounded only formally; the real bound comes with the policy (S18). |
| T4 | S15 vs S24 "our timeout during the wait" | Reachable only when time elapses between the S15 check and the sleep. | Direct wait test with stub + `run()` test with the logger-hook gap (§8). |
| T5 | R5 (memory) vs R1 (oversized bodies keep today's behaviour) | `response.text()` buffers before the size check (`TODO(SECURITY)`). | R16: unchanged here; not widened by retry (sequential attempts); bounded reader is a follow-up. |
| T6 | R1 vs S21 | Error line moves from attempt to loop. | Same text, level, count. |
| T7 | R5 vs R1 (network retries) | `fetchWithRetry`'s counter restarts per attempt: `5 × (fetchRetries + 1)` fetches for an override (integer `fetchRetries`; it is clamped, not validated). | Accepted; one attempt with the default. |
| T8 | R4 vs R14 | Retry path dormant in production until the policy. | Fully tested through `ScriptedRequest` and the stubbed loop. |
| T9 | Abort wins vs R1 (same instance) | An abort before settlement replaces the loop's error or result, including a throwing decision. | Owner ruling: the abort wins; the abort's warn line is written. A logger hook cannot interfere (§7.1). |

## 11. Traceability

| Id | Where / how | Status |
|---|---|---|
| R1 | Guards 1–2; default never retries; same instance; one error line; replaced only by an abort before settlement | satisfied (T6, T9) |
| R2 | §6; attempt paths; `waitBeforeHttpRetry`; exit check; tests §8 | satisfied |
| R3 | One timer (`createRelayerHttpSignal`); guard 7; exit check (a late-settling `fetch` cannot return after the deadline) | satisfied |
| R4 | §8: signal, attempt, loop, wait, run(), default decision, abort wins | satisfied (T4, T8) |
| R5 | Default 0 retries; R15 ceiling; S17 fail-closed; abort guard; release on abort | satisfied with accepted tensions (T2, T3, T5, T7) |
| R6 | 2 classes + 2 purpose modules, each tested alone | satisfied |
| R7 | Moves/renames; one protected member; constructor unchanged | used |
| R8 | `_` module-private only; unprefixed `@internal` exports; `import`-first files; bug-first tests | satisfied |
| R9 | §3: zero new real public types | satisfied (T1) |
| R10 | No option type; no policy constants; one private ceiling | satisfied |
| R11 | `_retryDecision` is the only producer of a retry delay; guards only veto | satisfied |
| R12 | §1, §4 | satisfied |
| R13 | `protected` instance method, subclass override via `this`; no policy | satisfied (T1) |
| R14 | Default `_retryDecision` returns `undefined`; retries only via override | satisfied (T8) |
| R15 | `MAX_HTTP_ATTEMPTS_CEILING = 5`, guard 3 | satisfied |
| R16 | `_readBody` keeps `response.text()`; follow-up noted §4.2 | satisfied |
| S1 | Attempt exported from its file only; no entry point | satisfied |
| S2 | `RelayerHttpAttemptParams` | satisfied |
| S3 | Attempt owns nothing; tested | satisfied |
| S4 | `_fetch`, `_readBody`, `_parseBody`, `_throwNonOk`, `_validateResult`; same error types | satisfied |
| S5 | `throwIfRelayerHttpAborted` with the given values | satisfied |
| S6 | Guards, limit, `_resultError` moved; prefix kept | satisfied (log emitter moved for one branch, T6) |
| S7 | Constructor params and checks | satisfied |
| S8 | `_hasRun`, same message; test | satisfied |
| S9 | `run()` §4.4 | satisfied |
| S10 | `_buildInit` inside the `try` | satisfied |
| S11 | Only `createRelayerHttpSignal`, once; one `cleanup` (plus release on abort) | satisfied |
| S12 | `_createAttempt` passes the same signal; attached per fetch | satisfied |
| S13 | `waitBeforeHttpRetry` on the combined signal | satisfied |
| S14 | Next attempt's `fetchWithRetry` `throwIfAborted()` | satisfied |
| S15 | `remainingMs` + guard 7, independent of overrides | satisfied |
| S16 | Guards 1–2, 5; same instance on stop | satisfied (contract on every decision) |
| S17 | Guard 6, fail closed | satisfied (enforced, not trusted) |
| S18 | Policy cap deferred (R13); R15 ceiling is the backstop | deferred |
| S19 | Deferred (R13); signature takes the error (status, code) and attempt, no clock | deferred |
| S20 | `_logRetry` | satisfied (no "/max") |
| S21 | `_logResponseError` once; same instance | satisfied |
| S22 | Delay-range contract: loop tests; default contract: table test; policy table: deferred | partially (policy part deferred) |
| S23 | `ScriptedRequest.run()`; "up to the cap" = finite script + ceiling loop test | satisfied |
| S24 | Direct wait tests + `run()` tests (logger-hook gap, §8) | satisfied (T4) |
| S25 | `run()` cleanup and leak tests | satisfied |
| S26 | Default never retries: existing call counts unchanged | satisfied (R14) |
