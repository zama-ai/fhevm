# Relayer http request module tests

This folder holds the unit tests of the relayer module. Most of them cover `RelayerHttpRequest`,
the client for relayer-http's synchronous routes (`POST /v4/user-decrypt`, `/v4/public-decrypt`).
A few cover the shared relayer helpers and the older `RelayerAsyncRequest`.

Run them all from `sdk/js-sdk`:

```sh
npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/test/
```

Each file also starts with the command that runs it alone.

---

## 1. The code under test

One call to `RelayerHttpRequest.run()` goes through four pieces:

```
RelayerHttpRequest.run()            entry point: one-shot guard, signal, timeout, cleanup, retry policy
  │  combine caller signal + timeout ──► relayerHttpSignal
  ▼
runRelayerHttpRetryLoop             runs attempts, waits between them, stops on success or a final error
  ▼
RelayerHttpAttempt.run()            one HTTP call: fetch → read body → parse → check status → validate
```

Most tests drive `run()` and observe only what a caller sees: the returned value or the thrown
error, the log lines, the number of `fetch` calls, and the timers and listeners left behind.
Some files test one inner piece directly (`attempt`, `retryLoop`, `combinedSignal`).

## 2. Two kinds of suites

The `RelayerHttpRequest` tests come in two kinds.

**Implementation-aware tests** (`attempt`, `combinedSignal`, `errors`, `leaks`, `phases`,
`publicDecrypt`, `reentrancy`, `retry`, `retryLoop`, `signal`, `userDecrypt`). They were written
with the code in view. They test inner functions directly and reach every branch.

**Black-box tests** (`blackbox`, `blackbox.abortWins`). They were written from the design and the
public API only, without reading the implementation, by independent agents. They check the
contract a caller relies on, so they stay valid if the code is rewritten. `blackbox.abortWins`
covers the later design change "an abort before settlement wins".

Some guard tests were added to the two black-box files afterwards, during a review that did read
the code (hung fetch, throwing listeners, build throws, hostile abort reasons). They test public
behaviour, but they are not black-box by origin.

## 3. How the tests were generated

The tests were written by AI agents (Claude Fable and Opus, and GPT-Astra through Codex), under
the owner's supervision, in three waves.

**Wave 1: implementation-aware suite.** The first tests were written alongside the code, with the
implementation in view (commit `7728db414`). They aim at branch coverage: every phase of a run,
every exit path, the retry guards, reentrancy and leaks.

**Wave 2: black-box campaign.** To get tests that do not simply mirror the code, a campaign was
run in which the implementation stayed sealed. Three writer agents from different model families
each wrote a suite using only the design, an approved public API contract and the relayer-http
server source, which defines the wire formats. They typechecked against stubs whose bodies only
throw. A coordinator ran their tests against the real code through an "oracle" that returned
pass or fail and assertion messages only, never source lines or stack traces. Three reviewers
judged each suite against the design, and an auditor checked the agents' tool logs to confirm
none of them read the implementation. A failing test was treated as a design-versus-code mismatch
to report, not as a test to adjust. The `blackbox` and `blackbox.abortWins` files come from this
campaign; `abortWins` follows the owner's later ruling that an abort before settlement wins.

**Wave 3: review-driven tests.** A later security, leak and AbortSignal review (two independent
agents, then a merge of their findings) listed the remaining bugs and missing tests. Each bug got
a test written first, which failed against the code, and then the fix: the body excerpt that kept
the whole body alive, timers and listeners held by a hung `fetch`, a throwing abort reason name,
a `NaN` reason, `signal: null`, and an inaccurate log line. The other gaps were closed with guard
tests that pass today and protect behaviour already correct.

## 4. Test families

### 4.1 Responses and errors

**Goal:** a response is turned into exactly one result or one typed error, and each error keeps
the information a caller needs.

- A valid success body returns the result: the share array for user decrypt
  (`userDecrypt`), the result object for public decrypt (`publicDecrypt`).
- relayer-http's own error body (`code`, `message`, `requestId`) becomes a
  `RelayerHttpResponseError`, not logged by the attempt.
- Any other non-2xx body (a proxy page, JSON `null`) becomes a logged
  `RelayerHttpUnexpectedResponseError`.
- A network failure, an unreadable body, a non-JSON body or a body over the 10 MB limit becomes a
  `RelayerHttpFetchError`. A body exactly at the limit is still parsed.
- A 200 body with the wrong shape becomes a `RelayerHttpResultError`.

**Examples:**

| Alice…                  | Then…                                               | Alice should get…                                       |
| ----------------------- | --------------------------------------------------- | ------------------------------------------------------- |
| asks to decrypt a value | the relayer answers 200 with a valid body           | the decrypted result                                    |
| asks to decrypt a value | the relayer answers 403 `{ code: "acl_denied", … }` | a `RelayerHttpResponseError` with `code = "acl_denied"` |
| asks to decrypt a value | a proxy answers 502 with an HTML page               | a `RelayerHttpUnexpectedResponseError`                  |
| asks to decrypt a value | the relayer answers 200 with a 20 MB body           | a `RelayerHttpFetchError`, and the body is never parsed |

**Files:** `errors`, `attempt`, `userDecrypt`, `publicDecrypt`, and `blackbox` › security ›
"relayer-http error body recognition" and "body size limit".

### 4.2 Abort and timeout

**Goal:** the caller can always stop a request, and the caller can always tell who stopped it.

- **Who aborted.** Our own timeout gives a `RelayerHttpTimeoutError`. A caller abort gives the
  caller's reason back unchanged, whatever it is: a string, `null`, `NaN`, an `Error`, a
  `DOMException`, or even a `RelayerHttpResponseError`. The caller's own `AbortSignal.timeout()`
  is never mistaken for ours.
- **Every phase.** An abort is checked in each phase of a run: before the start, during `fetch`,
  during the body read, during the wait between attempts, during the network-retry delay, and in
  the gap between a wait and the next `fetch`.
- **Abort wins.** An abort that lands before `run()` settles wins over the result and over the
  error, even when the response already arrived. An abort after settlement changes nothing.
- **Options.** `options.timeout` is validated (integer from 1 to `MAX_TIMEOUT_MS`), and
  `options.signal: null` means no signal.
- **The combined signal.** `createRelayerHttpSignal` merges the caller's signal and our timer
  into one internal signal, keeps the first abort reason, never aborts the caller's own signal,
  and reports the remaining time.

**Examples:** Alice starts a request with `timeout: 5000` and her own signal; the relayer is slow.

| Alice…                                                         | Then…      | Alice should get…                                       |
| -------------------------------------------------------------- | ---------- | ------------------------------------------------------- |
| clicks "Cancel" after 1 s with reason `"user left"`            | —          | exactly `"user left"`                                   |
| waits                                                          | 5 s pass   | a `RelayerHttpTimeoutError`                             |
| cancels just after the answer arrived, before `run()` returned | —          | `"user left"`, not the result                           |
| cancels after `run()` returned                                 | —          | the result she already has, unchanged                   |
| uses her own `AbortSignal.timeout(1000)`                       | 1 s passes | her own `TimeoutError`, not a `RelayerHttpTimeoutError` |

**Files:** `combinedSignal`, `signal`, `phases`, `blackbox` › abort, `blackbox.abortWins` › abort.

### 4.3 Retry policy

**Goal:** retries happen only when the policy says so, and never past the limits.

- The base class never retries a relayer-http error. A subclass overrides `_retryDecision` to
  return a delay, or `undefined` to stop.
- The decision only ever sees a `RelayerHttpResponseError`. Network errors are retried earlier,
  inside `fetchWithRetry`, with their own delay.
- Invalid delays (`NaN`, fractions, zero, values above `MAX_TIMEOUT_MS`) stop the loop instead
  of retrying ("fail closed").
- A wait that would end after the timeout is not started.
- A hard ceiling of 5 attempts holds, whatever the decision says.
- All attempts share one signal and one deadline.
- A decision or log hook that throws stops the run with that error.

**Examples:** Alice uses a subclass that retries "overloaded" errors after 1 s.

| Alice…                               | Then…                                          | Alice should get…                                          |
| ------------------------------------ | ---------------------------------------------- | ---------------------------------------------------------- |
| sends a request                      | the relayer answers 503 "overloaded", then 200 | the result, after exactly 2 calls                          |
| sends a request                      | the relayer keeps answering 503                | the last `RelayerHttpResponseError`, after at most 5 calls |
| sends a request with `timeout: 1500` | the relayer answers 503 after 1 s              | the error at once: a 1 s wait would end past her timeout   |
| uses the base class (no subclass)    | the relayer answers 503                        | the error after 1 call: no retry                           |

**Files:** `retry`, `retryLoop`, `blackbox` › retry.

### 4.4 Resource cleanup (leaks)

**Goal:** a finished request leaves nothing behind: no pending timer, no listener on the caller's
signal, and no large object kept alive by the errors it throws.

This matters because callers often share one long-lived signal across many requests. Every exit
is checked: success, each error type, each abort, a build throw (unserializable payload, invalid
auth header), and a throwing hook. Many runs on one shared signal must stay balanced, one listener
added and the same one removed per run.

**`removeEventListener` on every exit.** The listener on the caller's signal is added with
`{ once: true }`, but `once` only removes it if the signal actually aborts. On a success or a
plain error the caller's signal never fires, so the SDK must call `removeEventListener` itself,
exactly once and with the same handler. The tests spy on both calls and compare the handlers.

**Errors keep copies, not references.** A thrown error may be stored for a long time (an error
tracker, a retry wrapper, an app log), so it must not hold the request's large objects. Errors
carry only strings and numbers: url, status, content type, and a 500-character body excerpt.
They never hold the `Response`, the request init (with the auth headers), the payload or the
signals. The excerpt is built with `_bodyExcerpt()`, which makes a real copy: a plain
`bodyText.slice(0, 500)` can be stored by the JS engine as a pointer into the whole body, so a
small excerpt would keep up to 10 MB alive. This copy is checked by measurement, not by a unit
test, because whether a string is a slice is internal to the engine.

Two cases go further:

- **A `fetch` that ignores the abort and never settles.** The timer and the caller listener are
  still released as soon as the signal aborts. `run()` itself stays pending: that is by design,
  because native `fetch` never behaves this way.
- **Throwing listeners.** A third-party listener that throws on the caller signal or on the
  combined signal does not block forwarding or cleanup.

**Examples:**

| Alice…                                                      | Then…                                 | Alice should see…                                                       |
| ----------------------------------------------------------- | ------------------------------------- | ----------------------------------------------------------------------- |
| sends many requests with one page-wide signal               | they succeed, fail or get cancelled   | no SDK listener left on her signal, no SDK timer pending                |
| sends a request with `timeout: 60000`                       | it succeeds after 1 s                 | no timer left running for the other 59 s                                |
| sends a request through a broken `fetch` that never answers | our timeout fires                     | the timer and listener freed at once (`run()` stays pending, by design) |
| sends a request that succeeds, with her own signal          | her signal never aborts               | the SDK's listener removed anyway, with the same handler it added       |
| stores every error in her error tracker                     | a proxy answers with 10 MB HTML pages | each stored error holds a 500-character copy, not the 10 MB page        |

**Files:** `leaks`, `signal` › "external signal cleanup", `combinedSignal` › cleanup and
"release on abort", `blackbox` › leaks, `blackbox.abortWins` › leaks.

### 4.5 Races and orderings

**Goal:** the outcome is the same whatever order two events happen in.

These tests place an abort or a timer at an exact point between two steps of the code, using
gated promises, hooks and fake timers:

- caller abort and our timer at the same instant;
- an abort right after `run()` returns, in the same tick;
- an abort between the response arriving and `run()` settling, for a success and for an error;
- a microtask-by-microtask sweep after the response, as a smoke check.

**Examples:** Alice sets `timeout: 500` and also cancels her own signal at exactly 500 ms.

| First to fire at 500 ms | Then…                            | Alice should get…                           |
| ----------------------- | -------------------------------- | ------------------------------------------- |
| Alice's cancel          | our timer fires right after      | her own reason                              |
| our timer               | Alice's cancel lands right after | a `RelayerHttpTimeoutError`                 |
| either                  | the response arrives right after | the abort, never the result and never a mix |

**Files:** `blackbox` › races, `blackbox.abortWins` › races, `phases` › "caller abort vs our timer
at the same instant".

### 4.6 Reentrancy and lifetime

**Goal:** user callbacks cannot corrupt a request, and an instance runs only once.

- `run()` works once per instance; a second call is rejected, even from inside a hook.
- Hooks (`_retryDecision`, `logger.warn`, `logger.error`, `fetch` itself) may abort the caller's
  signal or call `run()` again. The request stays consistent.
- Two instances sharing one caller signal: one abort stops both; a finished instance no longer
  reacts to that signal.

**Examples:** Alice and Bob start two requests that share Alice's signal.

| Who…                                         | Then… | Should get…                                          |
| -------------------------------------------- | ----- | ---------------------------------------------------- |
| Bob's request finishes, then Alice cancels   | —     | Alice: her reason. Bob: his result, unchanged        |
| both are still running, Alice cancels        | —     | both: Alice's reason                                 |
| Alice calls `run()` twice on the same object | —     | the second call: an error asking for a new instance  |
| Alice's retry hook cancels her own signal    | —     | Alice: her reason, and no second call to the relayer |

**Files:** `reentrancy`, `blackbox` › reentrancy, `blackbox.abortWins` › reentrancy.

### 4.7 Security

**Goal:** no secret leaks out, and nothing unsafe reaches the network.

- The constructor rejects invalid input (operation, timeout).
- The request is built once: headers, JSON body and auth. An invalid header value (CR, LF, NUL)
  or an unserializable payload fails before any `fetch`.
- No auth token or header value appears in a log line or an error.
- Only relayer-http's exact error shape is trusted as a relayer error, and bodies are capped in
  size.

**Examples:**

| Alice…                                     | Then…                                                               | Alice should get…                                     |
| ------------------------------------------ | ------------------------------------------------------------------- | ----------------------------------------------------- |
| sends a request with an API key            | the relayer answers with an error                                   | an error and log lines that never contain her key     |
| uses an auth token containing a line break | —                                                                   | an error at once, and nothing sent to the relayer     |
| sends a payload containing a `BigInt`      | —                                                                   | an error at once, and nothing sent to the relayer     |
| sends a request                            | something answers 503 with a look-alike error body (no `requestId`) | a `RelayerHttpUnexpectedResponseError`, never retried |

**Files:** `blackbox` › security, plus `attempt` › "request shape".

### 4.8 Shared relayer helpers

These files test the helpers used by both relayer clients.

- **`relayerUrl`:** base URL validation (http or https only, no user info, query or fragment,
  HTTPS when auth is set), URL building, and percent-encoding of `options.query`.
- **`readErrorMessage`:** extracts `message` and `label` from a relayer or edge (Cloudflare,
  Kong) error body, and never throws.
- **`fetchFheEncryptionKeySource`:** the `v2/keyurl` call: query handling, 401/403 messages and
  the accepted response shapes.
- **`RelayerAsyncRequest.errors` / `.query`:** the older async-job client: auth and edge errors
  are surfaced with their message, and `options.query` is appended.

**Examples:** Alice sits behind a proxy that strips custom headers.

| Alice…                                             | Then…                                                                    | Alice should get…                                              |
| -------------------------------------------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------- |
| passes her API key in `options.query`              | —                                                                        | the key added to the URL, correctly encoded                    |
| configures `ftp://relayer.example` as the base URL | —                                                                        | an error before any request                                    |
| sends a request                                    | the edge (Cloudflare, Kong) answers 403 `{ message: "invalid api key" }` | "invalid api key" in the error, not a generic "HTTP error 403" |
| sends a request                                    | the edge answers 403 with a plain-text body                              | the generic message: nothing to extract                        |

## 5. File index

| File                                    | Family                                 | Kind                 |
| --------------------------------------- | -------------------------------------- | -------------------- |
| `RelayerHttpRequest.attempt`            | Responses and errors, request shape    | Implementation-aware |
| `RelayerHttpRequest.blackbox`           | All families                           | Black-box            |
| `RelayerHttpRequest.blackbox.abortWins` | Abort wins, leaks, races               | Black-box            |
| `RelayerHttpRequest.combinedSignal`     | Abort and timeout, cleanup             | Implementation-aware |
| `RelayerHttpRequest.errors`             | Responses and errors                   | Implementation-aware |
| `RelayerHttpRequest.leaks`              | Resource cleanup                       | Implementation-aware |
| `RelayerHttpRequest.phases`             | Abort in every phase                   | Implementation-aware |
| `RelayerHttpRequest.publicDecrypt`      | Responses (public decrypt)             | Implementation-aware |
| `RelayerHttpRequest.reentrancy`         | Reentrancy and lifetime                | Implementation-aware |
| `RelayerHttpRequest.retry`              | Retry policy through `run()`           | Implementation-aware |
| `RelayerHttpRequest.retryLoop`          | Retry loop and wait, directly          | Implementation-aware |
| `RelayerHttpRequest.signal`             | Abort classification, options, cleanup | Implementation-aware |
| `RelayerHttpRequest.userDecrypt`        | Responses (user decrypt)               | Implementation-aware |
| `relayerUrl`                            | Shared helpers                         | —                    |
| `readErrorMessage`                      | Shared helpers                         | —                    |
| `fetchFheEncryptionKeySource`           | Shared helpers                         | —                    |
| `RelayerAsyncRequest.errors`            | Async client                           | —                    |
| `RelayerAsyncRequest.query`             | Async client                           | —                    |

## 6. How the tests are written

- **Fake timers.** Time is driven with `vi.advanceTimersByTimeAsync`, so promise jobs run between
  timer callbacks as they do in production. `vi.advanceTimersByTimeAsync(0)` flushes promise jobs
  without moving time; tests use it to prove a run settled by itself, not because a timer fired.
- **Retry overrides.** Tests that need retries use a small local subclass that overrides
  `_retryDecision` with a script of delays.
- **Fetch stubs.** `fetch` is replaced per test: immediate responses, a fetch that rejects only on
  abort (like the real one), a stalled body stream, or a gated promise the test releases.
- **Throwing listeners.** Node reports a throwing abort listener as an `uncaughtException`. Tests
  that need one trap it, assert on it, then restore the runner's handlers.
- **Bug tests vs guard tests.** A test written for a found bug fails before the fix. A guard test
  pins behaviour that is already correct, so it passes and catches a later regression.
