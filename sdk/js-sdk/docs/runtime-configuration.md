# Runtime configuration

The runtime config controls _how_ the SDK loads and runs its WebAssembly
cryptography — thread count, where WASM assets come from, and which module
versions to use. Set it once, before any client is created.

```ts
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
// or: from '@fhevm/sdk/viem'

setFhevmRuntimeConfig({});
```

{% hint style="warning" %}
`setFhevmRuntimeConfig` is **per adapter** — the `@fhevm/sdk/ethers` and
`@fhevm/sdk/viem` entry points each hold their own config. Import and call it from
the **same** adapter you create your client with. If your app uses both adapters,
configure each one.
{% endhint %}

An empty object accepts every default and is the correct starting point for most
apps. Reach for the options below only when you hit a specific need — multi-thread
performance, a strict CSP, a bundler that relocates assets, or version pinning.

## Set it once

`setFhevmRuntimeConfig` writes a frozen, per-adapter singleton:

- Call it **before** creating any client or runtime.
- Calling it again with an **identical** config is a no-op (safe).
- Calling it again with a **different** config **throws**. Keep it in one place,
  at startup.

{% hint style="warning" %}
Call `setFhevmRuntimeConfig` before creating any client, and only once. Calling it again with a different configuration throws. Keep it in a single startup module.
{% endhint %}

`hasFhevmRuntimeConfig()` reports whether it has been set.

```ts
if (!hasFhevmRuntimeConfig()) {
  setFhevmRuntimeConfig({ numberOfThreads: 8 });
}
```

## Options

```ts
type FhevmRuntimeConfig = {
  readonly numberOfThreads?: number;
  readonly singleThread?: boolean;
  readonly wasmAssetLoadMode?: WasmAssetLoadMode;
  readonly locateFile?: (file: string) => URL;
  readonly moduleVersions?: FhevmModuleVersions;
  readonly logger?: Logger;
  readonly auth?: Auth;
};
```

Every field is optional.

| Option              | Default                                  | Purpose                                                        |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------- |
| `numberOfThreads`   | `navigator.hardwareConcurrency` (browser) | Worker threads for encryption. `0` forces single-threaded.   |
| `singleThread`      | `false`                                  | Force single-threaded mode; skips all worker setup.           |
| `wasmAssetLoadMode` | `'auto'`                                 | How the worker script is fetched and verified.                |
| `locateFile`        | auto (`file://` in Node, embedded in browser) | Map an asset filename to a URL you host.                 |
| `moduleVersions`    | `'auto'`                                 | Pin specific TFHE/TKMS WASM versions.                          |
| `logger`            | none                                     | `{ debug, warn, error }` hooks for SDK diagnostics.           |
| `auth`              | none                                     | Authentication context for Relayer requests.                  |

## Threading and browser headers

Multi-threaded encryption is significantly faster but requires
`SharedArrayBuffer`, which browsers only enable under two response headers:

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

{% hint style="warning" %}
Multi-threaded encryption needs these two headers, and they must be set by the server that serves your app. Without them, browsers disable `SharedArrayBuffer` and the SDK falls back to single-threaded mode.
{% endhint %}

The SDK **never throws** over threading. If the headers are missing or the
environment can't support threads, it logs a warning and transparently falls back
to single-threaded mode:

```
This browser does not support threads. Verify that your server returns correct headers:
 'Cross-Origin-Opener-Policy': 'same-origin'
 'Cross-Origin-Embedder-Policy': 'require-corp'
```

To opt out of threading entirely — for a maximally compatible build, or an
environment where the headers can't be set — force single-threaded mode:

```ts
setFhevmRuntimeConfig({ singleThread: true });
```

## Environment behavior

The SDK is isomorphic; loading differs by environment:

| Environment       | Threading                          | WASM source (default)                       |
| ----------------- | ---------------------------------- | ------------------------------------------- |
| **Browser**       | Multi-threaded with COOP/COEP, else single | Embedded base64 (no extra network requests) |
| **Node.js**       | Multi-threaded via `worker_threads` | Auto `file://` URLs, falling back to embedded |

Node auto-derives `file://` URLs for the WASM files shipped in the package. If a
bundler relocates the package and the files can't be found, it cleanly falls back
to the embedded base64 copies.

{% hint style="warning" %}
The SDK **cannot** run inside an Edge isolate (Cloudflare Workers, Vercel Edge,
Next.js `runtime='edge'` server components) — edge runtimes forbid dynamic
`WebAssembly.compile`, which the SDK relies on. Edge deployments must use the SDK
from **client components (CSR)**: the edge runtime serves the page and the browser
runs the SDK. See [Runtime compatibility](runtime-compatibility.md) for the full
environment matrix.
{% endhint %}

## WASM asset loading and CSP

WASM module loading and worker-script loading are independent. `wasmAssetLoadMode`
governs only the **worker script**:

| Mode                  | Transport | Integrity check | Notes                                                       |
| --------------------- | --------- | --------------- | ----------------------------------------------------------- |
| `'embedded-base64'`   | inline    | n/a             | Worker from SDK-embedded source. No external request.       |
| `'verified-blob'`     | URL       | SHA-256         | Fetch, verify bytes, run as a `blob:` worker. Real integrity. Requires a CSP allowing `blob:` workers. |
| `'precheck-direct-url'` | URL     | probe only      | Pre-flight fetch to fail fast on misconfig; not integrity.  |
| `'trusted-direct-url'`| URL       | none            | Hand the URL straight to the runtime.                       |
| `'auto'` (default)    | URL→inline| SHA-256 if URL  | Try `verified-blob` if a worker URL exists; else embedded.  |

The three URL modes require a resolvable worker URL — supply one via `locateFile`.
If your CSP forbids `blob:` workers, the `*-direct-url` modes use a plain
`new Worker(url)` and are exempt.

### Hosting your own WASM assets

To serve WASM/worker files from your own origin or CDN, return their URLs from
`locateFile`:

```ts
setFhevmRuntimeConfig({
  wasmAssetLoadMode: 'verified-blob',
  locateFile: (file) => new URL(`/fhevm-assets/${file}`, window.location.origin),
});
```

Return a `URL` to serve that asset from your host, or `undefined` to fall back to
the embedded base64 copy for that file.

## Pinning module versions

By default the SDK uses the latest bundled WASM versions (TFHE `1.6.2`, TKMS
`0.13.20-0`). To pin explicit versions — for reproducible builds or
compatibility with a specific protocol deployment — set `moduleVersions`:

```ts
setFhevmRuntimeConfig({
  moduleVersions: {
    tfhe: '1.6.2', // encryption WASM
    kms: '0.13.20-0', // decryption WASM
    checkCompatibility: 'throw', // 'throw' | 'warn' | 'off'
  },
});
```

Bundled versions: TFHE `'1.5.3'` or `'1.6.2'`; TKMS `'0.13.10'` or `'0.13.20-0'`.
`checkCompatibility` decides what happens when a pinned version doesn't match the
chain's protocol — throw, warn, or ignore. It has no effect under `'auto'`.

## Preloading WASM

Configuration decides _how_ modules load; the client decides _when_. Construction
does no I/O — you await `client.ready` (or `client.init()`) once to resolve
protocol versions and compile WASM before encrypting or decrypting. Await it at a
moment you control:

```ts
setFhevmRuntimeConfig({ numberOfThreads: 8 });

const client = createFhevmClient({ chain: sepolia, provider });
await client.init(); // compile WASM now
```

For the lower-level runtime init helpers (`initFhevmRuntime`,
`initFhevmEncryptRuntime`, `initFhevmDecryptRuntime`), see
[API reference](api-reference.md).

## Related

- [Clients](clients.md) — WASM sizes per client type and the lifecycle members.
- [Architecture](architecture.md) — how the runtime, modules, and WASM fit together.
- [API reference](api-reference.md) — `FhevmRuntimeConfig` and `WasmAssetLoadMode` in full.
```

