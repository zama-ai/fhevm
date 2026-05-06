# Runtime configuration

Before creating any clients, you must call `setFhevmRuntimeConfig()` once to configure how the SDK runs its internal WASM modules. This page covers every configuration option, environment-specific behavior, and framework setup.

## Quick start

The simplest valid configuration:

```ts
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem'; // or "@fhevm/sdk/ethers"

setFhevmRuntimeConfig({});
```

All parameters are optional. This works in both Node.js and browsers with sensible defaults.

## Configuration options

```ts
setFhevmRuntimeConfig({
  numberOfThreads: 4,
  singleThread: false,
  locateFile: (file) => new URL(`/wasm/${file}`, window.location.origin),
  logger: { debug: console.log, error: console.error },
});
```

| Option            | Type                    | Default                 | Description                                       |
| ----------------- | ----------------------- | ----------------------- | ------------------------------------------------- |
| `numberOfThreads` | `number`                | All available cores     | How many threads WASM uses for encryption         |
| `singleThread`    | `boolean`               | `false`                 | Force single-threaded mode (disables Web Workers) |
| `locateFile`      | `(file: string) => URL` | Auto-detect (see below) | Custom resolver for WASM file locations           |
| `logger`          | `{ debug, error }`      | `undefined`             | Optional logger for SDK debug output              |

### `numberOfThreads`

Controls the number of worker threads used for TFHE encryption. More threads means faster `encrypt()` calls.

```ts
setFhevmRuntimeConfig({
  numberOfThreads: navigator.hardwareConcurrency || 4,
});
```

- **Node.js:** Controls the `node:worker_threads` pool size
- **Browser:** Controls the number of Web Workers spawned
- **Default:** Uses all available cores (`navigator.hardwareConcurrency` in browsers, all cores in Node.js)
- If set to `0` or less, falls back to single-threaded mode

Decryption (TKMS) does **not** use multi-threading — it's a much lighter operation (~600KB WASM).

### `singleThread`

Forces single-threaded mode. Use this when you can't set the COOP/COEP headers required for `SharedArrayBuffer` in browsers (see [Multi-threading in browsers](#multi-threading-in-browsers)).

```ts
setFhevmRuntimeConfig({
  singleThread: true,
});
```

When `true`, encryption runs on the main thread. It's slower but works everywhere — no headers needed, no Web Worker restrictions.

### `locateFile`

Tells the SDK where to find its WASM files. The function receives a filename and must return a `URL` pointing to that file.

```ts
setFhevmRuntimeConfig({
  locateFile: (file) => new URL(`https://cdn.example.com/wasm/${file}`),
});
```

The SDK resolves these files:

| File                     | Size   | Module            | Purpose                                         |
| ------------------------ | ------ | ----------------- | ----------------------------------------------- |
| `tfhe_bg.v1.5.3.wasm`    | ~5MB   | Encryption (TFHE) | FHE encryption WASM binary                      |
| `tfhe-worker.v1.5.3.mjs` | ~2KB   | Encryption (TFHE) | Web Worker script for multi-threaded encryption |
| `kms_lib_bg.wasm`        | ~600KB | Decryption (TKMS) | KMS decryption WASM binary                      |

**When you don't provide `locateFile`:**

- **Node.js:** The SDK resolves WASM files automatically using `__filename` (CJS) or `import.meta.url` (ESM). No configuration needed.
- **Browser:** The SDK embeds WASM as base64 strings in the JavaScript bundle. This means zero setup — but it adds ~5MB to your bundle. For production, consider serving WASM files separately (see [Serving WASM from a CDN](#serving-wasm-from-a-cdn)).

### `logger`

Optional logger for debug and error output. Useful for diagnosing WASM initialization issues.

```ts
setFhevmRuntimeConfig({
  logger: {
    debug: (message: string) => console.log('[fhevm]', message),
    error: (message: string, cause: unknown) => console.error('[fhevm]', message, cause),
  },
});
```

The logger interface:

```ts
type Logger = {
  readonly debug: (message: string) => void;
  readonly error: (message: string, cause: unknown) => void;
};
```

Pass `console` directly if you want all output:

```ts
setFhevmRuntimeConfig({
  logger: console,
});
```

## Idempotency and immutability

`setFhevmRuntimeConfig()` can only be called **once** per adapter. Calling it again with the **same** parameters is fine (idempotent). Calling with **different** parameters throws an error.

```ts
// First call — sets the config
setFhevmRuntimeConfig({ numberOfThreads: 4 });

// Same config — no-op (safe)
setFhevmRuntimeConfig({ numberOfThreads: 4 });

// Different config — throws an error
setFhevmRuntimeConfig({ numberOfThreads: 8 }); // ❌ Error
```

This prevents accidental reconfiguration in apps with multiple entry points or hot module reloading. If you're using a framework with HMR (like Next.js or Vite in dev mode), wrap the call in a guard:

```ts
let configured = false;
if (!configured) {
  try {
    setFhevmRuntimeConfig({ numberOfThreads: 4 });
    configured = true;
  } catch {
    // Already configured from a previous HMR cycle
  }
}
```

---

## Node.js setup

Node.js requires no special configuration. The SDK resolves WASM file paths automatically and uses `node:worker_threads` for multi-threading.

```ts
import { setFhevmRuntimeConfig, createFhevmClient } from '@fhevm/sdk/ethers';
import { sepolia } from '@fhevm/sdk/chains';
import { ethers } from 'ethers';

setFhevmRuntimeConfig({ numberOfThreads: 4 });

const provider = new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com');
const client = createFhevmClient({ chain: sepolia, provider });
```

**Requirements:**

- Node.js >= 22.0
- Both ESM and CommonJS are supported (the SDK ships dual builds)
- WASM base URL uses `__filename` in CJS and `import.meta.url` in ESM — handled automatically

---

## Browser setup

In browsers, the SDK works out of the box with embedded base64 WASM. For production, you'll want to configure two things: **WASM file hosting** (for smaller bundles) and **HTTP headers** (for multi-threading).

### Multi-threading in browsers

The TFHE WASM module uses `SharedArrayBuffer` for multi-threading, which browsers only enable when your server sends these headers:

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

**Without these headers**, the SDK falls back to single-threaded mode automatically. Encryption still works — it's just slower.

### Serving WASM from a CDN

By default, the SDK embeds WASM as base64 in your JavaScript bundle. This is convenient (zero config) but adds ~5MB to your bundle. For production, serve the WASM files from your static assets or a CDN:

1. Copy the WASM files to your public directory (e.g., `/public/wasm/`)
2. Point `locateFile` to that directory:

```ts
setFhevmRuntimeConfig({
  numberOfThreads: navigator.hardwareConcurrency || 4,
  locateFile: (file) => new URL(`/wasm/${file}`, window.location.origin),
});
```

This replaces the ~5MB base64-inlined WASM with a standard HTTP download on first use.

---

## Framework-specific setup

### Next.js

Add COOP/COEP headers to `next.config.js`:

```js
module.exports = {
  async headers() {
    return [
      {
        source: '/(.*)',
        headers: [
          { key: 'Cross-Origin-Opener-Policy', value: 'same-origin' },
          { key: 'Cross-Origin-Embedder-Policy', value: 'require-corp' },
        ],
      },
    ];
  },
};
```

Configure the runtime once at app startup (e.g., in a top-level layout or provider component):

```ts
'use client';

import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';

let configured = false;
if (!configured) {
  try {
    setFhevmRuntimeConfig({ numberOfThreads: 4 });
    configured = true;
  } catch {
    // Already configured (HMR in dev mode)
  }
}
```

### Vite

Add COOP/COEP headers to `vite.config.ts`:

```ts
export default defineConfig({
  server: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
});
```

Then configure the runtime in your app entry:

```ts
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';

setFhevmRuntimeConfig({
  numberOfThreads: navigator.hardwareConcurrency || 4,
});
```

### Nginx / reverse proxy

Add to your server or location block:

```
add_header Cross-Origin-Opener-Policy same-origin;
add_header Cross-Origin-Embedder-Policy require-corp;
```

### Browser extensions

Browser extensions run in a restricted environment. Key considerations:

- **Manifest V3 service workers** don't have access to `SharedArrayBuffer` — use `singleThread: true`
- **Content scripts** share the page's COOP/COEP context — multi-threading works only if the host page has the right headers
- **Extension pages** (popup, options) can use multi-threading if they have the right CSP

Recommended extension configuration:

```ts
setFhevmRuntimeConfig({
  singleThread: true, // safest for extensions
  locateFile: (file) => new URL(`wasm/${file}`, chrome.runtime.getURL('/')),
});
```

If your extension page controls its own headers and needs faster encryption, you can enable multi-threading:

```ts
setFhevmRuntimeConfig({
  numberOfThreads: 4,
  locateFile: (file) => new URL(`wasm/${file}`, chrome.runtime.getURL('/')),
});
```

---

## What loads when

WASM modules load **lazily** — not when you call `setFhevmRuntimeConfig()` or `createFhevmClient()`, but the first time you call an action that needs them:

| First call to...             | What loads                     | Size                           |
| ---------------------------- | ------------------------------ | ------------------------------ |
| `encrypt()`                  | TFHE WASM + network public key | ~5MB WASM + ~50MB key download |
| `decrypt()`                  | TKMS WASM                      | ~600KB                         |
| `publicDecrypt()`            | Nothing (HTTP only)            | —                              |
| `signDecryptionPermit()`     | Nothing                        | —                              |
| `generateTransportKeyPair()` | TKMS WASM                      | ~600KB                         |

If you want to preload WASM at app startup (for example, behind a loading spinner), call:

```ts
await client.init();
// or equivalently:
await client.ready;
```

Both return a promise that resolves when all modules for that client type are initialized. Calling either multiple times is safe (idempotent).

---

## Troubleshooting

**"FhevmRuntime config has already been set and cannot be changed"**
You called `setFhevmRuntimeConfig()` twice with different parameters. This usually happens with HMR in development. Wrap the call in a guard (see [Idempotency](#idempotency-and-immutability)).

**Encryption is slow in the browser**
Check that COOP/COEP headers are set correctly. Open the browser console — the SDK logs a warning if `SharedArrayBuffer` is not available and it falls back to single-threaded mode.

**"Missing locate file function"**
The browser can't find WASM files and the base64 fallback failed. Provide a `locateFile` function or ensure your bundler includes the SDK's embedded WASM chunks.

**Worker creation fails (CSP error)**
Your Content Security Policy blocks `blob:` URLs or inline workers. Either update your CSP to allow `worker-src blob:` or use `locateFile` to point to hosted worker scripts.
