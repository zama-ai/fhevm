# Runtime & rendering compatibility

Where the `@fhevm/sdk` runs, and whether it runs **multi-threaded (MT)** or
**single-threaded (ST)**. The SDK loads two WebAssembly modules — **TFHE**
(encryption; can be multi-threaded) and **TKMS** (decryption; always lightweight /
single-threaded) — so support in any environment comes down to three capabilities:

1. **WASM compilation** — the runtime must allow building a WebAssembly module from
   bytes (`WebAssembly.compile`). If the SDK can't compile WASM, it can't run at all.
2. **Decompression** — the embedded WASM is gzip-compressed. The SDK uses the
   platform `DecompressionStream` when available, and falls back to a built-in
   pure-JS inflater otherwise (old browsers, some runtimes). So decompression is
   never a hard blocker.
3. **Threading (TFHE MT only)** — multi-threaded TFHE needs **both** a
   `SharedArrayBuffer` **and** a worker backend (Web `Worker` or
   `node:worker_threads`). If either is missing, the SDK **gracefully degrades to
   single-threaded** (ST always works; it needs no worker).

> **Single-threaded always works wherever WASM compiles.** Multi-threaded is a
> performance optimization that requires the extra threading capabilities below.

## Support matrix

| Environment | SDK runs (ST) | Multi-threaded (MT) | What MT needs here | Notes |
| --- | :---: | :---: | --- | --- |
| **Browser** (client-side) | ✅ | ✅ when cross-origin isolated | COOP/COEP headers → `SharedArrayBuffer` + Web Workers | Zero-config. Without COOP/COEP → degrades to ST. |
| **Browser — older** (Firefox <113, Safari <16.4) | ✅ | ✅ when cross-origin isolated | same as above | No `DecompressionStream` → SDK uses its pure-JS inflater. |
| **Electron** (sandboxed renderer) | ✅ | ✅ when cross-origin isolated | Web Worker (no `worker_threads` in a sandboxed renderer) | SDK auto-selects the Web Worker backend. |
| **Node.js** (scripts, backend, long-running server) | ✅ | ✅ | `node:worker_threads` + `SharedArrayBuffer` (always available) | No COOP/COEP needed — SAB is always present in Node. |
| **Bun** | ✅ | ✅ | `worker_threads` | Forced to the Node worker backend for parity with Node. |
| **Deno** | ✅ | ✅ when isolated | Web Worker + `SharedArrayBuffer` | Web-standard backend. |
| **Next.js — CSR** (client component, any server runtime) | ✅ | ✅ when cross-origin isolated | browser `SharedArrayBuffer` + Web Workers | The SDK runs in the **browser**; the server runtime only ships the shell. |
| **Next.js — SSR (Node runtime)** (server component) | ✅ | ✅ | `node:worker_threads` (COOP/COEP irrelevant server-side) | Requires bundler `node:`-import hints (see "Bundlers"). |
| **Next.js — SSR (Edge runtime)** (server component) | ❌ | ❌ | — | **Unsupported.** See "Edge" below. |
| **Next.js — Edge route + CSR** (client component on a `runtime='edge'` route) | ✅ | ✅ when cross-origin isolated | browser `SharedArrayBuffer` + Web Workers | **Supported** — the SDK runs client-side; the edge isolate never touches WASM. |
| **Vercel Edge / Cloudflare Workers** (running the SDK *in* the isolate) | ❌ | ❌ | — | **Unsupported.** See "Edge" below. |

Legend: ✅ supported · ❌ unsupported · "when cross-origin isolated" = the page is
served with `Cross-Origin-Opener-Policy: same-origin` + `Cross-Origin-Embedder-Policy:
require-corp` (so `crossOriginIsolated === true` and `SharedArrayBuffer` is available).

## The SSR vs CSR distinction (meta-frameworks)

For frameworks like Next.js, **where the SDK code executes** matters more than which
route it lives on:

- **CSR** — the SDK runs in a **client component** (its work happens in the browser
  after hydration). This is the normal, fully-supported path. The server runtime
  (Node *or* Edge) only renders the HTML shell and never compiles WASM, so even an
  **edge-rendered route works** as long as the SDK is used client-side.
- **SSR** — the SDK runs in a **server component** (its work happens during server
  render). Supported on the **Node** runtime; **not** on the **Edge** runtime.

So "edge" is not simply unsupported: **edge + CSR is supported; edge + SSR is not.**

## Why the Edge runtime can't run the SDK (server-side)

Running the SDK *inside* an edge isolate (Vercel Edge, Cloudflare Workers, Next.js
`runtime='edge'` server components) fails for three independent reasons:

1. **Dynamic WASM compilation is forbidden.** Edge isolates ban runtime code
   generation, including `WebAssembly.compile`/`instantiate` from bytes. The SDK
   compiles its modules from bytes, so it is rejected. (Next.js **dev** emulates Edge
   on Node and only *warns* — `DynamicWasmCodeGenerationWarning` — so it can appear to
   work locally while failing in production.)
2. **No `SharedArrayBuffer`.** Edge isolates do not expose SAB, so `supportsThreads`
   is `false` → MT is impossible regardless of the above.
3. **Code-size limits.** The TFHE module is several MB, which typically exceeds edge
   bundle-size caps.

**Recommendation:** on edge deployments, use the SDK from **client components (CSR)**.
The edge runtime serves the page; the browser runs the SDK.

## Bundlers

The SDK loads Node built-ins (`worker_threads`, `fs`, etc.) via dynamic `import()`
guarded by environment checks. Bundlers must be told **not** to statically analyze
those imports, each via its own magic comment — the SDK includes all three
(`@vite-ignore`, `webpackIgnore`, `turbopackIgnore`). Without the right one a bundler
can turn the import into a runtime throw, which (for example) silently disabled
server-side MT under Turbopack until fixed. No action is required by SDK consumers;
this is noted for completeness.

## Decompression fallback

The embedded WASM is gzip-compressed. The SDK probes whether a **usable**
`DecompressionStream` exists (it *constructs* one — a `typeof` check is insufficient
because some runtimes expose a stub that throws), and otherwise inflates with a
bundled, dependency-free pure-JS inflater. This keeps the small compressed payload
working on older browsers and other runtimes that lack a working `DecompressionStream`
— no consumer action required.
