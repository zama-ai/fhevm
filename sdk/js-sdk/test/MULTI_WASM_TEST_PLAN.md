# Multi-WASM Coexistence Test Plan

## Context

The SDK supports multiple WASM module versions side-by-side:

- Multiple **Encrypt** WASM modules can coexist; see [src/core/modules/encrypt/module](../src/core/modules/encrypt/module).
- Multiple **Decrypt** WASM modules can coexist; see [src/core/modules/decrypt/module](../src/core/modules/decrypt/module).

Versioned WASM libraries:

- Encrypt (TFHE): [src/wasm/tfhe](../src/wasm/tfhe)
- Decrypt (TKMS): [src/wasm/tkms](../src/wasm/tkms)

The required module version is selected at runtime via the on-chain host-contract version heuristic in
[src/core/runtime/HyperWasmSolver-p.ts](../src/core/runtime/HyperWasmSolver-p.ts):

- `ACL.getVersion() < 0.4.0` -> TFHE `1.5.3` + TKMS `0.13.10`
- `ACL.getVersion() >= 0.4.0` -> TFHE `1.6.1` + TKMS `0.13.20-0`

### Lifecycle Boundary

The SDK should treat loaded WASM modules as **JS-realm lifetime resources**.

This is intentional:

- TFHE multithreaded worker startup is one-shot per generated WASM module version.
- Terminating TFHE workers in-place is not a pause/resume lifecycle; after termination, the module should be abandoned.
- TKMS is non-multithreaded, but generated wasm-bindgen class instances can hold raw WASM pointers. In-place unload/reload would be unsafe while JS references may still exist.
- `WebAssembly.Memory` can grow and generally does not shrink.

Therefore the supported SDK scope is:

> A single JS realm may run N supported versions of a given WASM module in parallel, for the lifetime of that realm.

The SDK does **not** support unloading, restarting, replacing, or re-spawning an already loaded WASM version in the same JS realm.

## Goal

Prove that the SDK can:

- Correctly route clients to the expected TFHE/TKMS versions using `moduleVersions: 'auto'`.
- Hold two WASM versions of both encrypt and decrypt modules simultaneously in one JS realm.
- Run both versions concurrently without crashes or cross-version contamination.
- Start with bounded, explicit resource usage by setting `numberOfThreads`.

This is a coexistence and routing test. Minimal encryption/decryption operations are in scope only when needed to prove that the loaded modules are usable and isolated.

## Explicit Non-Goals

Do **not** test same-realm restart, terminate, reload, or re-spawn behavior.

Restart/terminate/reload tests are only meaningful if the runtime is isolated in a separate realm/process and that isolated runtime owns all native WASM objects. In that model, the parent process must communicate with the isolated runtime using only plain serializable data, not wasm-bindgen class instances.

Examples that are out of scope for this plan:

- `init version A -> terminate workers -> init version A again` in the same JS realm.
- `init version A -> unload WASM -> reload version A` in the same JS realm.
- Asserting that process RSS shrinks after freeing/terminating a WASM module.
- Calling `terminateWorkers()` as a normal SDK lifecycle API.

## Test Environment

### Default Topology: Two Anvils

Use two Anvil/localstack instances on different ports, each with a full host-contract deployment:

- Deployment A -> `ACL.getVersion() < 0.4.0` -> expected TFHE `1.5.3`, TKMS `0.13.10`
- Deployment B -> `ACL.getVersion() >= 0.4.0` -> expected TFHE `1.6.1`, TKMS `0.13.20-0`

Both clients are created in the same JS realm so the test exercises simultaneous WASM module coexistence.

This topology is preferred because it can reuse the existing single-deployment setup and still exercises the relevant SDK behavior: each client reads the ACL version from its configured chain and initializes the selected WASM versions in the same process.

### Optional Topology: One Anvil, Two Deployments

One Anvil with two host-contract deployments at distinct addresses is closer to production, but it requires deployment tooling that can produce two independent address sets on one chain.

If this topology is used, the helper must pass two ACL addresses and two complete chain configs. Avoid relying on a single global address set.

### Relayer URL Requirement

The FHE encryption key cache is global and keyed by `relayerUrl`. Coexistence tests that fetch encryption keys must use distinct relayer URLs/key sources unless the purpose of the test is explicitly to cover same-relayer cache behavior.

Add a separate contamination guard for the same-relayer case so the suite does not accidentally pass by reusing the first cached key.

## Test Tiers

| Tier                    | Runtime                                               | Tooling    | Purpose                                                                                                                                                  |
| ----------------------- | ----------------------------------------------------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1. Unit**             | Node.js                                               | Vitest     | Fast, deterministic tests for version routing and mismatch guards.                                                                                       |
| **2. Node Integration** | Node.js + `worker_threads`                            | Vitest     | Proves two auto-routed WASM version pairs coexist in one server-like JS realm.                                                                           |
| **3. Browser Smoke**    | Chromium Playwright, optionally broader browser smoke | Playwright | Catches browser-only same-realm module/worker coexistence, COOP/COEP, MIME, CSP, and `SharedArrayBuffer` issues without requiring multiple local chains. |

## Shared Helpers

Helpers live under `test/multi-wasm/shared/` or `test/multi-wasm/support/` and should make the lifecycle assumptions explicit.

Shared helper responsibilities:

- Start or discover the two local deployments.
- Return two full client inputs, not just ACL addresses:
  - `rpcUrl`
  - chain config
  - ACL address
  - relayer URL
  - expected TFHE version
  - expected TKMS version
- Configure the SDK once per JS realm with:
  - `moduleVersions: 'auto'`
  - explicit `numberOfThreads`, preferably `1` or `2`
  - `singleThread: false`
  - fixed `wasmAssetLoadMode`
- Create two clients in the same JS realm.

Do not write helpers that change global SDK config between test cases in the same JS realm. Use separate test processes/workers when different runtime config is required.

## Plan

### Phase 0 - Version-Routing Unit Tests

Test `hyperWasmResolveTfheModuleVersion` and `hyperWasmResolveTkmsModuleVersion` directly.

Cases:

1. ACL version below `0.4.0` resolves to TFHE `1.5.3` and TKMS `0.13.10`.
2. ACL version exactly `0.4.0` resolves to TFHE `1.6.1` and TKMS `0.13.20-0`.
3. ACL version above `0.4.0` resolves to TFHE `1.6.1` and TKMS `0.13.20-0`.
4. Explicit `moduleVersions` overrides `auto`.
5. Missing native client fails with a clear error.
6. Non-ACL host contract version fails the `ACL` assertion.
7. Malformed host-contract version strings fail cleanly.

Also cover `getVersion` cache behavior enough to ensure two distinct ACL addresses do not contaminate each other.

### Phase 1 - Single-Version Smoke

Before testing coexistence, validate one client against each deployment independently.

For each deployment:

1. Configure the SDK with `moduleVersions: 'auto'`, `singleThread: false`, and an explicit `numberOfThreads`.
2. Create one client.
3. Await `client.ready`.
4. Assert the initialized TFHE and TKMS versions match the ACL version.
5. Assert `numberOfThreads` equals the configured value.
6. Assert `threadsAvailable === true` for multithreaded TFHE.

This catches broken loader paths, missing worker assets, or incorrect routing before the two-version test adds concurrency.

### Phase 2 - Node Parallel Coexistence

Create both clients in the same Node.js process / JS realm.

1. Configure once:
   - `moduleVersions: 'auto'`
   - `singleThread: false`
   - `numberOfThreads: 1` or `2`
2. Create `clientA` for the old ACL deployment.
3. Create `clientB` for the new ACL deployment.
4. Initialize both sequentially:
   - `await clientA.ready`
   - `await clientB.ready`
5. Initialize both concurrently in a separate isolated test process:
   - `await Promise.all([clientA.ready, clientB.ready])`
6. Assert:
   - client A uses TFHE `1.5.3` and TKMS `0.13.10`
   - client B uses TFHE `1.6.1` and TKMS `0.13.20-0`
   - the TFHE versions differ
   - the TKMS versions differ
   - each TFHE module reports the configured `numberOfThreads`
   - each TFHE module reports `threadsAvailable === true`

Then run minimal work on both clients concurrently.

Suggested operations:

- `Promise.all([clientA.encryptValues(...small uint8...), clientB.encryptValues(...small uint8...)])`
- `Promise.all([clientA.generateTransportKeyPair(), clientB.generateTransportKeyPair()])`

Assertions:

- Both operations complete without worker, `SharedArrayBuffer`, or version mismatch errors.
- Running client A before client B and client B before client A both succeed.
- After using one client, the other remains usable.

Do not terminate either module as part of this test.

### Phase 3 - Cross-Version Contamination Guards

These tests prove that native wrappers and caches do not silently cross version boundaries.

Cases:

1. **TFHE key/version mismatch**
   - Obtain or construct an FHE encryption key for version A.
   - Attempt to use it in a version B operation.
   - Assert a clear mismatch/ownership error, not silent success.

2. **TFHE owner mismatch**
   - Ensure a `FheEncryptionKeyWasm` owned by one runtime/client cannot be used by another incompatible runtime/client.

3. **TKMS private-key mismatch**
   - Generate a TKMS transport/private key for version A.
   - Attempt to use it with version B.
   - Assert a clear `TkmsVersion mismatch`.

4. **Relayer cache isolation**
   - Run coexistence with distinct relayer URLs and assert each client fetches/uses its own key material.
   - Add a same-relayer test that documents expected behavior. If same-relayer reuse is invalid for mixed deployments, assert that it fails clearly rather than silently reusing incompatible key material.

5. **Module info is version-specific**
   - Query module info for both expected versions after initialization.
   - Assert each version has its own info and thread count.
   - Query an uninitialized supported version, if available in an isolated process, and assert it is `undefined`.

### Phase 4 - Lightweight Browser Module Coexistence

Add one lightweight Playwright page under `test/browser`, not `test/multi-wasm`.

Reason:

- The full two-client browser topology would require two independent chain deployments.
- Current localstack tooling cannot run the required two-chain setup.
- `test/multi-wasm` is still useful for the existing single-version matrix, but it is not the right harness for browser coexistence while only one localstack chain is available.

The browser coexistence test should initialize modules directly in one browser JS realm:

1. Configure the SDK once with:
   - explicit `numberOfThreads`, preferably `1`
   - `singleThread: false`
   - fixed `wasmAssetLoadMode`
   - local URL-backed WASM/worker assets
2. Create or obtain one runtime in the page.
3. Directly initialize:
   - TFHE `1.5.3`
   - TFHE `1.6.1`
   - TKMS `0.13.10`
   - TKMS `0.13.20-0`
4. Initialize the versions concurrently where possible.
5. Assert:
   - both TFHE module infos are present
   - both TKMS module infos are present
   - each TFHE module reports the configured `numberOfThreads`
   - each TFHE module reports `threadsAvailable === true`
   - `crossOriginIsolated === true`
   - URL-backed WASM and worker assets loaded successfully
6. Run minimal chain-free usability work, such as TKMS transport/private-key generation for both TKMS versions.

This browser smoke does not prove `moduleVersions: 'auto'` routing, because it intentionally avoids ACL calls and localstack dependencies. Auto-routing remains covered by unit tests and Node/client integration tests.

The explicit multithreaded coexistence browser test may be Chromium-only to keep it cheap and deterministic. Broader Chromium/Firefox/WebKit smoke coverage can stay focused on single-version loader behavior or single-threaded direct-module initialization.

## Resource Assertions

Use explicit resource budgets in every multithreaded coexistence test.

Required assertions:

- `numberOfThreads` is configured explicitly, not inherited from `navigator.hardwareConcurrency`.
- Each initialized TFHE version reports exactly the configured thread count.
- Browser tests confirm `crossOriginIsolated === true` when multithreading is expected.

Optional instrumentation:

- Expose per-version WASM linear memory size from generated glue:
  - `wasm.memory.buffer.byteLength`
  - pages = `byteLength / 65536`
- Track process-level `process.memoryUsage().rss`, `external`, and `arrayBuffers` before and after initialization.

Memory assertions should be high-water/resource-budget checks, not shrink-after-free checks.

## Isolated Runtime Tests

Restart, terminate, reload, and memory-reclamation tests belong in a separate isolated-runtime plan.

Only test those behaviors when:

- The SDK runtime is created inside a Worker, iframe, child process, or other isolated realm.
- The isolated runtime owns all wasm-bindgen native objects.
- The parent sends only serializable data.
- The parent receives only serializable data.
- The parent destroys the whole isolated realm/process to reclaim resources.

Those tests should not use native SDK objects across the isolation boundary.

## Implementation Order

1. Add routing unit tests for `HyperWasmSolver-p.ts`.
2. Add single-version smoke tests for both deployments.
3. Add Node two-client coexistence with explicit `numberOfThreads`.
4. Add cross-version contamination guards.
5. Add lightweight browser same-realm module coexistence under `test/browser`.
6. Optionally add memory/resource instrumentation.
7. Keep isolated restart/terminate/reload testing as a separate future track.

## Open Questions

- Should the default coexistence topology stay as two Anvils, or is one Anvil with two deployments important enough to justify deployment-tooling changes?
- What exact thread budget should CI use: `1` for cheapest coexistence coverage, or `2` to prove real parallel worker fan-out?
- Should `getTfheModuleInfo()` / `getTkmsModuleInfo()` expose `wasmMemoryBytes` for operational monitoring?
- What is the intended behavior when two mixed-version deployments share the same relayer URL?

## References

- `src/core/runtime/HyperWasmSolver-p.ts` - auto version routing.
- `src/core/modules/encrypt/module` - TFHE module init and API wrappers.
- `src/core/modules/decrypt/module` - TKMS module init and API wrappers.
- `src/wasm/tfhe/*/startWorkers.js` - generated TFHE worker lifecycle.
- `test/browser` - lightweight browser loader and direct-module coexistence smoke tests.
- `test/multi-wasm` - existing browser roundtrip matrix and asset-loading harness.
- `docs/compatibility.md` - compatibility notes and version mapping.

## Robustness Tests

These target the failure and contention surfaces of running multiple WASM instances in
one realm — concurrency, inter-WASM pollution, cache races, state drift. They extend the
browser smoke ([`test/browser/scripts/smoke-coexistence.ts`](browser/scripts/smoke-coexistence.ts)),
which already covers concurrent init of different versions, the
`(module version × served key format)` compatibility matrix, and **concurrent** execution
of that whole matrix via `Promise.all` (with deterministic ordered assertion afterward).

Note on concurrency vs. speed: `build_with_proof_packed` is a synchronous,
main-thread-blocking WASM call (rayon workers parallelize *within* one call). Running
builds concurrently is about proving correctness/safety under interleaving, not wall-clock
speedup — the existing concurrent matrix confirmed ~equal wall-clock to sequential.

### #1 - Cross-module object rejection (inter-WASM pollution) - HIGH

Goal: prove a WASM object built by module A cannot be used by module B, and that a
rejected cross-use does not corrupt either module. (Concrete, browser-level realization of
Phase 3 cases 1-2.)

Mechanism:

1. With module A (`1.6.1`): `deserializeFheEncryptionPublicKey` + `deserializeFheEncryptionCrs`
   to get real A-bound objects; assemble a `FheEncryptionKeyWasm` via `createFheEncryptionKeyWasm`.
2. Call `runtime.encrypt.buildWithProofPacked({ fheEncryptionKey: <A's key>, tfheVersion: B })`
   against module B (`1.5.3`). Assert it throws cleanly (no crash/hang).
3. Then run a valid encryption on both A and B and assert both still succeed.

Catches: missing object-provenance checks, wasm-bindgen pointer/heap confusion across
instances, corruption-on-rejection that silently poisons later operations.

Notes: uses the lower-level `buildWithProofPacked` (not the `ZkProofBuilder`), so `metaData`
must be assembled manually (the 3x20 + 32 layout in `ZkProofBuilder-p.ts` `build()`).
Distinct from the `CHAIN_DEFINITIONS` matrix, which fails at *format* deserialization
rather than *object provenance*.

### #2 - Concurrent idempotent-init storm - MEDIUM

Goal: guarantee concurrent inits of the same version collapse to one shared module and one
worker pool.

Mechanism:

1. Fire ~20 concurrent `initTfheModule({ tfheVersion })` for the same version, plus a mix
   across versions.
2. Assert all resolve; `getTfheModuleInfo` is stable (same `numberOfThreads`,
   `threadsAvailable === true`, same wasm/worker URLs).
3. Run an encryption afterward to confirm the module is healthy. Repeat for TKMS
   (`initTkmsModule`).

Catches: double-init races spawning duplicate worker pools - especially dangerous given
TFHE workers are one-shot (a racy second `startWorkers` is unrecoverable; see Lifecycle
Boundary above).

Notes: fast and cheap. An "init happened exactly once" assertion needs a spy/counter hook;
otherwise assert stability + post-init health as a proxy.

### #3 - Single-slot cache contention + post-failure recovery - HIGH

Goal: stress the cache's in-flight chaining under concurrency and prove failed
deserialization self-evicts without poisoning the slot/module.

Mechanism:

- (a) Contention: prime one slot, then fire many concurrent `build()`s against the same
  `relayerUrl` slot. Assert all succeed with consistent results.
- (b) Recovery: trigger the known failing case (`1.5.3` + `key.1.6.1`) so the slot
  self-evicts on deserialize failure, then re-prime that slot with a compatible key and
  build. Assert success.

Catches: `globalFheEncryptionKeyCache` `ensureBytes`/`ensureWasm` pending-promise chaining
races (`_pendingChained`), self-eviction poisoning, in-flight dedupe bugs.

Notes: "only one deserialize happened" is observable only with a spy/counter; otherwise
assert all-succeed + consistency as the signal. Complements the Phase 3 case 4 relayer
cache isolation guard.

### #4 - Interleaved round-robin endurance - MEDIUM

Goal: detect slow cross-module state drift / leaks that a single pass cannot see.

Mechanism: a loop alternating `1.5.3` -> `1.6.1` -> TKMS ops for N iterations, asserting
each result stays valid throughout. Optionally assert TKMS
`getTkmsModuleInfo().memory.pages` does not grow unbounded.

Catches: global state bleed between module instances over time, memory growth.

Notes: slower (many iterations); keep N modest to stay within the Playwright 300s timeout.
Per Resource Assertions above, treat memory as a high-water budget, not a shrink check.

### Suggested order

1. #1 Cross-module rejection - most direct inter-WASM pollution guard.
2. #3 Cache contention + recovery - highest-signal race coverage.
3. #2 Concurrent init storm - cheap, guards the one-shot-worker hazard.
4. #4 Round-robin endurance - endurance/leak coverage; run last / less often.
