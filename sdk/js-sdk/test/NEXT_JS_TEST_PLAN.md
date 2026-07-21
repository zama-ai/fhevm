Context:
The SDK is meant to be bullet-proof and used in all possible JS runtimes (Browser, Edge, SSR, CSR, Mixed SSR/CSR, Nextjs, TurboPack, Electron, esbuild, vite, etc. etc.)

- All popular bundlers must be supported
- All popular browsers must be supported
- All popular mobile browsers must be supported
- must be able to test every wasm loading possibilities
- must be able to support coexisting multiple wasm modules with multiple versions in memory
- must be able to have also cleartext mode available in parallel (universal scenario)
- Basically the SDK is multi-chain so you can have a server that serves multiple chains at the same time using multiple versions of the same WASM and also an active cleartext mode in parallel
- in ST or MT mode
- With or without COOP/COEP policies.
- SSR, CSR, Mixed
- as a Node Server (long time running)
- And many other scenarios ...

We should organize those tests as follow:

- 1 platform === 1 test folder: For example Nextjs -> test/browser-next
- Each test consist to load a given WASM and use that wasm
  - kms: load kms wasm and call generate key pair
  - tfhe: load pre-existing pubkey/crs pairs (keys are located in sdk/js-sdk/test/keys)
  - you can use the coexistence test located sdk/js-sdk/test/browser/pages/smoke-coexistence.html as a good starting point
  - each test should load multiple concurrent wasm versions + 1 cleartext runtime version. The idea is to make sure multiple FHEVM runtimes can safely coexist

### Non-negotiable constraints

1. **Existing tests are frozen.** Never alter any existing test except those in
   `test/browser-next`. `test/browser` and the others run fine and are the
   _reference_. The new architecture is **additive**; if we later migrate the old
   tests onto it, that's a separate, explicit step — not part of this work.
2. **Public API is frozen.** Do not add to / change the SDK's public API to make a
   test pass. If a test genuinely needs a private function, **manage a way to
   import the private function from the SDK** — and only as a **last resort**
   (first try to express it with existing public actions/clients).
3. **The coexistence test is the model.** `test/browser/.../smoke-coexistence.ts`
   (multi-version modules + multi-chain + real proof building, incl. an expected
   forward-incompat failure) is the shape every platform's "real" test should take
   — not a minimal init smoke.

### Platform isolation (hard rule)

- Each platform folder owns its framework deps in its **own** `package.json`
  (e.g. `browser-next` owns `next`/`react`; it consumes the SDK as a packed
  tarball). Those deps **must not leak** into the shared core or any other
  platform folder — no platform pollutes the rest.
- The **shared core depends on the SDK only** (no `next`, `vite`, `playwright`,
  `electron`). It is imported by every platform but carries none of their deps.
- **SDK resolution is per-platform:** the shared core imports the SDK via a single
  specifier (`@fhevm/sdk/...`); each platform maps it to the form it tests —
  `browser` aliases it to `src`, `browser-next` resolves the packed tarball.
  Prefer the **public** surface; where the coexistence model needs internals (it
  does — see Chunk 0), use the managed private-import path from constraint #2, not
  a public-API change.

---

## Current state — what already exists

`test/browser` (Vite + Playwright; chromium/firefox/webkit) is the **reference
implementation** and already covers a large slice of the cube for the
plain-browser platform. `test/browser-next` exists but is only a minimal CSR
smoke today.

| Cube cell (plain browser)                                                         | Status | Where                                                             |
| --------------------------------------------------------------------------------- | ------ | ----------------------------------------------------------------- |
| wasm-load: url / base64 / cdn / verified-blob                                     | ✅     | `smoke-wasm`, `smoke-base64`, `smoke-cdn`, harness                |
| CSP blocks WASM compile (negative)                                                | ✅     | `smoke-csp-block` (per-page `<meta>` CSP)                         |
| coexistence: multi-version + multi-chain (module×key matrix, incl. expected fail) | ✅     | `smoke-coexistence`                                               |
| module: tfhe (encrypt w/ keys) + kms (keygen)                                     | ✅     | harness assertions                                                |
| 3 desktop browsers                                                                | ✅     | `playwright.config.ts`                                            |
| **ST mode**                                                                       | ❌     | all cells run `singleThread:false`                                |
| **without COOP/COEP**                                                             | ❌     | vite always sets headers; harness _asserts_ `crossOriginIsolated` |
| **SSR / edge / mixed / node-server**                                              | ❌     | `browser-next` is CSR-only                                        |
| **cleartext runtime in parallel**                                                 | ❌     | coexistence has no cleartext leg                                  |
| **mobile browsers**                                                               | ❌     | —                                                                 |

**Reference model (frozen — do not edit):**
`test/browser/scripts/multiWasmHarness.ts` + `smoke-coexistence.ts` (runtime setup,
concurrent multi-version init, threads/readiness assertions, `test/keys` loading,
ZK-proof build + assertions, module×key compatibility matrix). Chunk 0 writes a
_new_ core modeled on these — it does not refactor them. Note they are DOM-coupled
(`document`, `#log`, hard-assert `crossOriginIsolated`) and use SDK internals (`-p`
builders), which the new core must handle differently (no DOM; internals via the
managed private path) to run server-side and against the packed artifact.

The uncovered axes (❌ above) are the **real new work**; the chunks below target
them rather than re-proving what `test/browser` already covers.

---

## The coverage cube

"Bullet-proof" spans this space. The runner drives the meaningful slices; it does
not attempt the full cartesian product (see pruning).

```
platform   { browser, browser-mobile, browser-next, electron, vite, esbuild,
             node-server, ... }                       ← 1 folder each
render     { csr, ssr-node, ssr-edge, mixed,
             node-server (long-running, multi-chain) } ← runtime/exec context
threads    { st, mt }
headers    { coop/coep on, off }
wasm-load  { embedded-base64, trusted-direct-url, precheck-direct-url,
             verified-blob, auto }
module     { tfhe (use keys), kms (keygen), cleartext }
coexist    { single,
             multi-version (≥2 TFHE versions in one realm),
             multi-version + cleartext in parallel (the multi-chain scenario) }
```

Two requirements deserve first-class treatment, not just a checkbox:

- **Coexistence is the headline scenario**, not an edge case: the SDK is
  multi-chain, so the realistic deployment is _N_ WASM versions + a cleartext
  runtime live in one process at once. Every platform folder's "real" test is the
  coexistence one; the single-version cell is just the warm-up.
- **`node-server` long-running** is its own exec context: a persistent Node
  process that serves multiple chains, where init happens repeatedly over the
  process lifetime and runtimes must not leak or clobber across requests.

### Pruning rules (impossible / irrelevant cells)

COOP/COEP are **browser response headers**; they affect only the browser leg and
have no effect on server-side init. Threading availability differs per leg:

| Leg                  | MT possible?                         | COOP/COEP relevant? | Worker backend   |
| -------------------- | ------------------------------------ | ------------------- | ---------------- |
| CSR (browser/mobile) | only if `crossOriginIsolated`        | **yes**             | Web Workers      |
| SSR / node-server    | yes (SharedArrayBuffer always avail) | no                  | `worker_threads` |
| SSR (edge)           | **never** → always ST                | no                  | none             |

Expected outcomes the assertions encode:

```
CSR leg:        numberOfThreads > 0  ⟺  (mode==mt && crossOriginIsolated)
SSR-node leg:   numberOfThreads > 0  ⟺  (mode==mt)            // COOP/COEP ignored
SSR-edge leg:   numberOfThreads === 0  always                 // mt → degrades to st
node-server:    same as SSR-node, asserted repeatedly over the process lifetime
mixed:          assert the SSR leg AND the CSR leg independently
```

**Rule:** prune impossible cells _explicitly and log the reason_ — never skip
silently (a silent skip makes the matrix look more complete than it is).
e.g. `log("edge is always ST; skipping MT-effective assertion")`.

---

## Chunking plan

Decomposed into layers that build on each other. Each chunk is independently
mergeable, green in CI, and gives signal on its own. A thin end-to-end skeleton
lands before any widening.

```
Chunk 0  Build a new DOM-agnostic core (modeled on smoke-coexistence)  ← foundation
Chunk 1  Walking skeleton: browser-next, ONE cell                      ← prove the pipe
Chunk 2  Widen axes within browser-next (rings)                        ← one axis per PR
Chunk 3  Add other platforms on top of the core                       ← cheap once solid
```

### Chunk 0 — New DOM-agnostic core (modeled on the coexistence test)

`test/browser/.../multiWasmHarness.ts` + `smoke-coexistence.ts` are the **model**,
but they are **frozen** (constraint #1) — do **not** refactor them. Write a _new_
deps-free core (`test/shared/`) that reproduces the coexistence shape:

- `runScenario(descriptor) -> diagnostics` — given
  `{ module, wasmLoad, threads, versions, chains }`, set config + init multiple
  module versions + **use** them (tfhe: build a real proof per chain with
  `test/keys`; kms: keygen; cleartext: mock), returning a plain object
  (`numberOfThreads`, `crossOriginIsolated`, `initialized`, `error`, logs; a
  per-runtime array for coexistence). Plus `test/keys` loader, version catalog,
  the module×key compatibility expectations, and `expected(descriptor, leg)` (the
  pruning rules) with a unit test.
- **Presentation stays per-platform**: browser renders `#result`/`#log`; next
  renders the same contract from a client/server component; node-server returns
  JSON. The DOM coupling and the hard `crossOriginIsolated` assert in the existing
  harness do **not** move into the core (the core reports `crossOriginIsolated`;
  it never _requires_ it — that's how ST / no-COOP cells become expressible).
- **Internals via the managed private path (constraint #2).** The coexistence
  model needs SDK internals not in the public API — `getEthersRuntime`,
  `encryptModule`/`decryptModule`, `createZkProofBuilder` (`-p`),
  `globalFheEncryptionKeyCache` (`-p`). **Gating decision for this chunk:** how
  does the core reach these _from the packed tarball_ in `browser-next`?
  Order of preference: (a) re-express via existing public actions/clients if
  possible; (b) if not, a single managed private import path from the SDK package
  (last resort) — **without** changing the public API.
- No platform deps in the core (no next/vite/playwright/electron).

### Chunk 1 — Walking skeleton (`browser-next`, single cell)

Smallest end-to-end proof of the pipe (core + Next/Turbopack + Playwright):
`csr · st · coop-on · embedded-base64 · tfhe(use keys) · single version`. This is
deliberately _below_ the coexistence model — it only de-risks wiring. The
per-platform **definition of done is the coexistence model** (Chunk 2c), not this
smoke. `browser-next` keeps its **own** `package.json` (next/react) and consumes
the packed tarball — no next deps escape the folder.

### Chunk 2 — Widen one axis per PR (risk-ordered)

This is where the isomorphic-audit findings live, so it leads.

- **2a. threads × headers** — `st`, `mt+coop`, `mt+no-coop`. Validates graceful
  degradation (request MT without SAB → falls back to ST, must not crash).
  Driven by `run.sh --mt/--st --coop/--no-coop`.
- **2b. render** — add `ssr-node`, then `mixed`, then `ssr-edge` (behind its
  caveat). Add a `javaScriptEnabled:false` Playwright project to prove SSR legs
  render server-side.
- **2c. coexistence** — multi-version, then multi-version + cleartext (the
  multi-chain scenario). **This is the per-platform definition of done** (the
  coexistence model, constraint #3); promote it early.
- **2d. wasm-load variants** — `embedded-base64` → `*-direct-url` →
  `verified-blob` → `auto`.
- **2e. module** — exercise `kms` (keygen) and `cleartext` paths individually.

### Chunk 3 — Other platforms

Reuse Chunk 0's core; per platform only the bundler/loader/exec wiring differs:
`browser-mobile`, `vite`, `esbuild`, `electron`, `node-server` (long-running,
multi-chain). Each is a self-contained folder with its **own** `package.json`;
nothing is shared except the deps-free core and `test/keys`. The existing
`test/browser` is **left untouched** (constraint #1) — a later, separate step may
migrate it onto the core, but that is out of scope here.

---

## DRY rules (keep the matrix from exploding)

1. **Cells are data, not code.** A scenario is a row
   (`{ render, threads, coop, wasmLoad, module, versions }`); the runner iterates.
   Adding coverage = adding rows, not new test files. This makes "one axis per PR"
   trivial.
2. **Prune explicitly, log the reason** (see pruning rules above).

## Runner (`run.sh`) flags

```
--csr | --ssr | --ssr-edge | --mixed | --node-server   exec context (→ FHEVM_TEST_ROUTE)
--mt  | --st                            thread mode (→ NEXT_PUBLIC_FHEVM_TEST_THREADS)
--coop | --no-coop                      COOP/COEP headers (→ FHEVM_TEST_COOP, read by next.config)
--threads=N                             explicit thread count
--module=tfhe|kms|cleartext             wasm under test
--wasm-load=embedded-base64|...|auto    loading strategy
--coexist=single|multi|multi+cleartext  coexistence scenario
--all                                   run the pruned matrix
--rebuild --build-profile=dev|prod      forwarded to scripts/run.mjs (rebuild+pack SDK)
```

Env is set by the script and inherited by `next dev`, so both the client page
(`NEXT_PUBLIC_*`) and `next.config.mjs` (server) pick it up. Each mode needs a
fresh server (`NEXT_PUBLIC_*` is inlined at startup), so set
`reuseExistingServer: false` for the matrix.

## Document structure

This file is the **hub**: cube + pruning rules + chunking plan. Concrete,
per-platform detail lives next to each platform. Folder layout enforces isolation:

```
test/
  NEXT_JS_TEST_PLAN.md      ← this hub
  keys/                     ← shared fixtures (pubkey/CRS json)
  shared/                   ← deps-free core: runScenario, keys loader,
                              expected() rules, version catalog (SDK-only imports)
  browser/                  ← platform: plain browser (vite). own deps. PLAN.md
  browser-next/             ← platform: next/turbopack. OWN package.json (next/react),
                              packed-tarball SDK, run.sh, PLAN.md
  node-server/              ← platform: long-running node, multi-chain. own deps. PLAN.md
  ...
```

- `test/browser-next/PLAN.md` — routes, `run.sh` flags, per-cell assertions.
- Each future platform folder gets its own `PLAN.md` from the same template.
- Only `test/shared/` and `test/keys/` are cross-platform; everything else,
  including all framework deps, stays inside its platform folder.

## Status & next chunks

### ✅ Done

- **Chunk 0 — infra core** (`test/infra/`): the same-origin gateway (standalone
  HTTP server + Next `rewrites()`; `nodeAdapter`/`webAdapter`; serves `relayer`,
  `rpc`, and `config`), anvil orchestration (sequential deploys, **verified reuse**
  [chain id + **per-slot** ACL deployed] with self-heal-replace, **bounded probes**
  via `AbortSignal.timeout`), `topology.ts` (slots, **distinct chain ids**
  31337/31338 **and distinct deployers** → distinct addresses), and the launchers
  `up.sh [-d]` / `down.sh` / `run-skeleton.sh`.
- **Chunk 1 — walking skeleton** (`/gw-skeleton`): proves anvil RPC + relayer
  keys reachable same-origin under COOP/COEP. ✅
- **First real SDK cell** (`/encrypt`): public `createFhevmClient` →
  `init` (on-chain ACL → TFHE 1.6.1 → embedded-base64 WASM, gzip) →
  `generateZkProof` (real proof from the relayer-served key). Covers
  `csr · st · coop-on · embedded-base64 · tfhe(use key) · single version (v13)`,
  for **both viem and ethers** (lib via `NEXT_PUBLIC_FHEVM_TEST_LIB`). ✅
- **2a — threads × headers** (`/encrypt`): `st`, `mt+coop` (`numberOfThreads>0`,
  `crossOriginIsolated`), `mt+no-coop` (degrades to ST, no crash). ✅ — this is also
  where the **TFHE MT worker bug** under Turbopack was found & fixed (the worker
  bootstrap's `process`-shim detection; injected `isBrowserLike`).
- **2c — coexistence** (`/coexist`, **per-platform definition of done**): **v12
  (TFHE 1.5.3) + v13 (TFHE 1.6.1) + a cleartext runtime** init concurrently in one
  realm, each builds a proof (cleartext: a mock encrypt), plus the module×key
  **forward-compat expected-fail** (older 1.5.x module + newer 1.6.1 key →
  deserialize error). Green in **st and mt+coop**, both libs. ✅
- **2b — render: ssr-node, ssr-edge, mixed** (`/encrypt-ssr`, `/encrypt-edge`,
  `/encrypt-mixed`). Async server-component pages run the SDK server-side (origin
  from the request `Host` header, not `window.location`); a
  `javaScriptEnabled:false` Playwright project proves the SSR/edge legs render
  server-side. Green. **Findings:** (a) ssr-node MT uses `worker_threads`
  (COOP-independent) — fixed a Turbopack node-import break by adding
  `turbopackIgnore` to the computed `node:` import in `environment.ts`; (b) **edge
  has no Web Workers / worker_threads / SharedArrayBuffer**, so TFHE is always ST
  there (an MT request must degrade, never crash); (c) edge's `DecompressionStream`
  is a throwing stub → vendored a dependency-free JS inflater (`src/core/base/inflate.ts`)
  and a construct-and-probe `supportsDecompressionStream()`; (d) **edge-SSR can't
  dynamically compile WASM** (`DynamicWasmCodeGenerationWarning`) — genuinely
  unsupported for the SDK short of a consumer-supplied precompiled `WebAssembly.Module`;
  edge-CSR is fine (SDK runs in the browser). ✅
- **2d — wasm-load variants** (`/wasm-load`): `verified-blob` /
  `trusted-direct-url` / `precheck-direct-url` / `auto`, each in **st and mt+coop**,
  via `setFhevmRuntimeConfig({ wasmAssetLoadMode, locateFile })` pointing at the
  gateway's same-origin `/gw/asset/<file>` route. **Finding:** a URL-loaded worker
  (`new Worker(url)`) is only cross-origin-isolated (→ SharedArrayBuffer) if its own
  script response carries `Cross-Origin-Embedder-Policy: require-corp` (+ CORP
  same-origin); the gateway now serves assets with those headers. Green. ✅
- **2e — module: kms keygen + cleartext** (`/module`): a real decrypt client loads
  and RUNS the TKMS wasm via `generateTransportKeyPair`, and the
  `@fhevm/sdk/viem|ethers/cleartext` mock runtime (encrypt + transport keypair),
  each as its own cell, both libs. Green. ✅
- **`browser-next/dod.sh`**: data-driven **26-cell** matrix (encrypt, coexist,
  ssr-node/edge/mixed, wasm-load ×4 modes, module kms/cleartext) over
  viem/ethers × st/mt±coop. Extensive CLI: `--index`, `--spec`, `--lib`,
  `--mt/--st`, `--coop/--no-coop`, `--wasm-load`, `--module`, `--list`, plus
  browser selection (`--firefox`, `--webkit`/`--safari`, `--all-browsers`;
  default chromium). MT verified on chromium **and** firefox/webkit.

**Gating decision RESOLVED:** the public API (`createFhevmClient` +
`generateZkProof`) builds a real proof against the **packed tarball** via the
gateway — **no SDK internals needed**.

**Coexistence findings (real SDK constraints surfaced by 2c):**
1. **Two chains must not share contract addresses.** The SDK resolves/caches the
   protocol→TFHE version per ACL address; identical addresses on two chains collide
   (both resolved one version). Fixed by deploying each anvil from a **distinct
   deployer mnemonic** (`FIRST_ANVIL_MNEMONIC` for v12; default for v13, so its
   committed `FHEVMHostAddresses.sol` stays clean — v12's is restored post-deploy).
   Per-slot addresses are served at `/gw/<slot>/config` so pages never hardcode them.
2. **The global FheEncryptionKey cache is keyed by relayer URL** (first-write-wins).
   Two legs sharing a relayer URL clash — incl. the **cleartext mock**, which writes
   deadbeef bytes that poison a real key. Each leg uses a **unique** relayer URL
   (the expected-fail and cleartext legs get dedicated slots).

**Test purpose (scope):** these tests verify each **WASM loads and runs** by
executing one of its functions — for TFHE, `generateZkProof` (runs the module in
ST or MT); for TKMS, generating a key pair. That's the whole goal: no need to mock
the relayer's `/v2/input-proof` or exercise `encryptValue`/decryption flows.

### Env knobs already wired
`FHEVM_TEST_LIB` (viem|ethers), `FHEVM_TEST_THREADS` (mt|st), `FHEVM_TEST_COOP`
(0|1), `FHEVM_TEST_WASM_LOAD`, `FHEVM_TEST_MODULE` (kms|cleartext),
`FHEVM_TEST_KEYS_DIR`, `FHEVM_TEST_WASM_DIR` — mapped to `NEXT_PUBLIC_*` for the
client bundle by `playwright.config.ts`. Render mode is selected by route
(`/encrypt`, `/encrypt-ssr`, `/encrypt-edge`, `/encrypt-mixed`). **Slot ids +
versions are centralized in `test/infra/config.ts`** (the single source of truth;
edit it on a protocol version roll); they reach the browser as
`NEXT_PUBLIC_FHEVM_SLOT_*`, injected by `playwright.config.ts` and read via
`app/_diag/slots.js` (the app can't import `config.ts` directly — Turbopack's root
is pinned to `browser-next`).

### Next chunks (in recommended order)

> **Chunk 2 is fully done in `browser-next`** — 2a (threads × headers), 2b (render:
> ssr-node/ssr-edge/mixed), 2c (coexistence), 2d (wasm-load variants), 2e (module:
> kms + cleartext), all green; see ✅ above. Plus cross-browser (chromium/firefox/
> webkit) and the `config.ts` centralization. **Next work is Chunk 3.**

- **Chunk 3 — other platforms.** Reuse Chunk 0's platform-agnostic core
  (`test/infra/`); per platform only the bundler/loader/exec wiring differs. Each
  platform mirrors `browser-next/` (its own app + `dod.sh`), shares `test/infra/`,
  and `test/browser` stays untouched. Recommended order:
  - **`vite`** first — highest-signal contrast to Turbopack (different bundler +
    WASM-asset story: `?url`/`?init`, a `__raw_wasm` middleware), exactly where
    bundler-specific SDK bugs hide. The parent `zama-fhe/sdk` repo's `test-vite`
    app is a wiring reference.
  - then **`node-server`** (no bundler — pure Node ESM/CJS resolution +
    `worker_threads`; cheap, catches packaging bugs), **`esbuild`**, **`electron`**,
    **`browser-mobile`**.

## Setup (Step 1 — ✅ DONE & verified end-to-end)

The render axis **SSR + CSR + Mixed are all required**, so the relayer is served by
a **real HTTP server** (not Playwright `page.route`, which can't intercept the
server-side fetches SSR/Mixed make). Step 1 is implemented and the skeleton passes
in chromium (anvils + gateway + keys + RPC proxy reachable same-origin under
COOP/COEP). The "as built" subsection below is the source of truth; earlier
subsections record the reasoning.

### Two anvils → two WASM versions

- Spawn **2 anvils** via `test/scripts/fhevm-anvil.sh` (reads `PORT` + `CHAIN_ID`):
  `PORT=8544 CHAIN_ID=31337 … --foundry-profile=v12` and
  `PORT=8546 CHAIN_ID=31338 … --foundry-profile=v13`.
- **Distinct chain ids per anvil are mandatory.** Foundry's broadcast/cache is
  keyed by chain id; two concurrent deploys on the same id collide (`nonce too
  low`). 31337 / 31338 keep the caches separate (and reflect a real multi-chain
  deploy). Addresses are chain-id independent (deterministic CREATE), so both
  expose the same contract addresses.
- The WASM/key version is **resolved from the on-chain ACL version** (verified:
  ACL `[0,3,0]`→protocol `0.12`→tfhe `1.5.4`; `[0,4,0]`→`0.13`→`1.6.1`). So two
  profiles ⇒ `createFhevmClient` ×2 loads **two TFHE versions** (`1.5.4`, `1.6.1`)
  — matching `test/keys/key.1.5.4.json` / `key.1.6.1.json`. (A lighter alt — one
  anvil + per-client `moduleVersions.tfhe` override — exists, but two anvils is the
  realistic, protocol-driven coexistence path.)
- **Deploy only the FHEVM stack, not `TFHETest.sol`** (`init` reads ACL/protocol
  contracts, never the app contract). Done via the opt-in **`--skip-fhetest`** flag
  added to `localcleartext-run-tests.sh` (kept opt-in so `test:localcleartext` is
  unaffected).
- A live provider is required (init reads on-chain) → **viem** dummy chain
  (preferred), one chain def per anvil (rpcUrl + relayerUrl), modeled on
  `chains/localcleartext.ts` + `fheTest/setup-viem.ts` (public `@fhevm/sdk/viem`).

### The same-origin gateway (as built)

The browser only ever talks to the page origin. A **same-origin gateway** proxies
per slot: `rpc` → anvil, and `relayer` → an in-process mini-relayer serving keys.
This (Option B) is the most reuse-flexible choice — survives strict CSP
(`connect-src 'self'`), exotic webviews/Electron, is a fault-injection point, and
needs no CORS from anvil/relayer.

- **Gateway = standalone HTTP server + Next `rewrites()`, not a bundled route.**
  The core (`test/infra/gateway/`) is a pure `handleGateway(req) -> res` with thin
  adapters (`nodeAdapter` for an `http.Server`/Vite; `webAdapter` for a Web
  `Request`/`Response`). It runs as a server (`server.ts`/`serve.ts`); platforms
  proxy `/gw/*` to it same-origin. Next does this with `rewrites()`.
  **Why not a Next route handler:** the infra is NodeNext (`.js` import
  extensions), which Turbopack can't resolve across an external dir — so bundling
  the core into the app fails. A server + rewrite sidesteps bundling entirely.
- **Mount segment is `/gw`, not `/__gw`** — Next treats `_`-prefixed app folders
  as private (non-routed).
- **2-step relayer protocol** (verified, `fetchFheEncryptionKeySource.ts`):
  `GET /gw/<slot>/relayer/v2/keyurl` → JSON with `fheKeyInfo[0].fhePublicKey.urls`
  + `crs["2048"].urls` (**URLs**, not inline bytes); the SDK then fetches the bytes.
  Since the relayer is **ours**, `keyurl` emits **same-origin** byte URLs at the
  page origin → no response-body rewriting (the multi-wasm MinIO step disappears).
- **Page origin via `x-forwarded-host`.** Behind the rewrite, the gateway's own
  `Host` is the proxy target (`:8590`), so byte URLs must be built from
  `x-forwarded-host` (the page origin Next forwards) — else the browser would
  fetch the gateway cross-origin and COEP would block it.
- **Keys dir is env-driven** (`FHEVM_TEST_KEYS_DIR`), with an `import.meta.url`
  fallback used only by un-bundled Node consumers — so nothing relies on
  `import.meta.url` in a bundled context.

This effort is a **fresh, independent setup** (constraint #1): it duplicates code
where useful and reuses only `test/keys` and small utils — existing tests are not
touched and will migrate later.

### Step 1 deliverable — delivered & verified

Files:

```
test/scripts/localcleartext-run-tests.sh   --skip-fhetest flag (opt-in)
test/infra/
  topology.ts                  slots: ports, distinct chainIds, key files, gateway port
  anvil/anvils.ts              startAnvils/stopAnvils (marker-readiness, reuse, teardown)
  anvil/spawn-anvils.ts        CLI: bring up both anvils until Ctrl-C
  gateway/keyRelayer.ts        test/keys -> v2/keyurl wire format + bytes
  gateway/gateway.ts           pure handleGateway(req)->res (rpc proxy + relayer)
  gateway/nodeAdapter.ts       http.Server / Vite middleware (x-forwarded-host origin)
  gateway/webAdapter.ts        Web Request/Response adapter (for a future edge/Deno mount)
  gateway/server.ts            create/listen/close the gateway http.Server
  gateway/serve.ts             CLI: run the gateway standalone
test/browser-next/
  globalSetup.ts               startAnvils + start gateway server
  globalTeardown.ts            close gateway + stopAnvils
  setupState.ts                shares handles across setup/teardown
  next.config.mjs              COOP/COEP headers + /gw/* rewrite -> gateway
  playwright.config.ts         globalSetup/teardown + webServer env
  app/gw-skeleton/page.jsx     fetches keyurl + bytes + rpc for each slot
  specs/gw-skeleton.spec.ts    asserts green
```

Verified: skeleton passes in chromium — keyurl + key bytes (pub 33 KB / crs 4.5 MB)
+ RPC `eth_chainId` for both slots (`0x7a69`/`0x7a6a`), `crossOriginIsolated=true`.

Run (foundry must be on PATH):

```
cd test/browser-next
PATH="$HOME/.foundry/bin:$PATH" \
  ../../node_modules/.bin/playwright test specs/gw-skeleton.spec.ts --config playwright.config.ts
```

(`FHEVM_TEST_COOP=0` to exercise the no-COOP cell; reused anvils are left running
between runs — `startAnvils` detects and reuses a live port.)
