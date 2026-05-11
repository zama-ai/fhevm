# Browser Test Plan

## Context

This repo is an SDK for encrypting/decrypting data using 2 WASM modules (TFHE + TKMS).
The SDK always runs in multi-threaded mode — TFHE spawns Web Workers and requires
`SharedArrayBuffer`. This is the primary reason browser testing is necessary: we must
validate that WASM threading works correctly under real browser security constraints.

## Goal

Validate the SDK's encrypt/decrypt functionality across all major browsers via Playwright,
with a focus on multi-threaded WASM execution.

## Constraints

- The SDK is dual-compiled: CJS and ESM
- TFHE always runs multi-threaded (Web Workers + SharedArrayBuffer)
- The test server **must** serve pages with these headers for SharedArrayBuffer to work:
  - `Cross-Origin-Opener-Policy: same-origin`
  - `Cross-Origin-Embedder-Policy: require-corp`
- Must run on Chromium, Firefox, and WebKit (Playwright defaults)

## Scope

- TFHE and TKMS WASM module loading
- TFHE multi-threaded worker spawning and execution
- Encrypt/decrypt round-trip
- ESM bundle in browser
- Two WASM loading strategies:
  - **URL-based (primary)**: The happy path. Test thoroughly across all browsers.
  - **Base64 (fallback)**: Used when binary `.wasm` files cannot be served. Test that it works, but don't optimize for its performance — the slowness is an accepted tradeoff.

## Approach

- Use Playwright as the browser testing framework
- Vite as the dev server (handles ESM resolution + COOP/COEP headers)
- Keep Vitest for unit/Node tests; Playwright for browser integration tests
- **1 page = 1 test**: each test scenario is a standalone HTML page with its own script
- An index page lists all tests for manual browsing
- Separate npm scripts: `test:browser` (automated), `test:browser:manual` (dev server)

## Test Scenarios

Each scenario is a self-contained page that reports pass/fail with step timings.

| Page                | Description                                                                                                |
| ------------------- | ---------------------------------------------------------------------------------------------------------- |
| `smoke-base64.html` | `createFhevmClient()` + `init()` — proves WASM, workers, and FHE key download work using fallback base64   |
| `smoke-wasm.html`   | `createFhevmClient()` + `init()` — proves WASM, workers, and FHE key download work using native wasm files |
| _(future)_          | Encrypt uint16 round-trip                                                                                  |
| _(future)_          | Base64 WASM fallback loading                                                                               |
| _(future)_          | Worker termination and cleanup                                                                             |

## Phases

### V0: Smoke test (done)

Minimal browser test using the real SDK API:

1. `setFhevmRuntimeConfig()`
2. `createFhevmClient({ chain: sepolia, provider })`
3. `await client.init()` — loads WASM, spawns workers, downloads global FHE key

If this passes, the SDK works in the browser.

### V1: ESM only

- Test both WASM loading strategies: URL-based and base64 inline
- Encrypt/decrypt round-trip tests
- Validate multi-threaded TFHE execution across Chromium, Firefox, WebKit

### V2: Bundler testing

- Test CJS/ESM via bundlers (webpack, rollup, etc.)
- Validate that bundled output works across all target browsers

## Directory Structure

Browser tests live in a dedicated, self-contained directory, fully isolated from the SDK source:

```
test/browser/
├── index.html              # Links to all test pages (for manual browsing)
├── vite.config.ts          # Vite dev server config (COOP/COEP headers)
├── playwright.config.ts    # Playwright config (browsers, timeouts)
├── pages/                  # One HTML page per test scenario
│   ├── smoke-base64.html
│   └── ...
├── scripts/                # Test logic (one script per page)
│   ├── smoke-base64.ts
│   └── ...
└── specs/                  # Playwright specs (one spec per page)
    ├── smoke-base64.spec.ts
    └── ...
```

## Manual Testing

For visual debugging and performance inspection:

- Run `npm run test:browser:manual` to start the Vite dev server
- Open `http://localhost:3333/test/browser/index.html` in Safari or Chrome
- The index page lists all test scenarios as clickable links
- Each test page displays a minimal, text-only view of:
  - Each test step as it executes (WASM loading, worker init, encrypt, decrypt)
  - Pass/fail status per step
  - Timing for each step (e.g. "TFHE init: 1243ms")
  - Total elapsed time
- No framework UI — just plain text appended to the page in real time
