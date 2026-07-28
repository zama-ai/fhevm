/**
 * JS-runtime environment detection and capability probes.
 *
 * Everything in this file answers one of two questions about the host
 * environment the SDK happens to be running in:
 *
 * - *Which* runtime is this? — {@link isNodeLike}, {@link isBrowserLike}
 * - *What* does it support? — {@link getNodeBuffer}
 *
 * Detection is intentionally capability-leaning rather than UA-based: edge
 * runtimes (Vercel Edge, Cloudflare Workers, Next.js edge) are neither Node nor
 * browser, so probes return the safe answer (`false` / `undefined`) and callers
 * fall back to web-standard APIs.
 */

export function isNodeLike(): boolean {
  return (
    // eslint-disable-next-line no-restricted-globals
    typeof process !== 'undefined' &&
    // eslint-disable-next-line no-restricted-globals, @typescript-eslint/no-unnecessary-condition
    typeof process.versions?.node === 'string'
  );
}

export function isBrowserLike(): boolean {
  return (
    // @ts-expect-error - Bun is a runtime global only under Bun
    typeof Bun === 'undefined' &&
    !isNodeLike() &&
    typeof location !== 'undefined' &&
    typeof location.href === 'string' &&
    typeof addEventListener === 'function' &&
    typeof removeEventListener === 'function'
  );
}

/**
 * Imports a Node built-in module by name, returning `undefined` when it isn't
 * importable (browsers, and edge runtimes: Vercel Edge, Cloudflare Workers,
 * Next.js edge). The id is built indirectly so bundlers don't statically
 * resolve/polyfill the module (a polyfill would bloat browser bundles).
 *
 * Every major bundler must be told NOT to analyze this computed dynamic import,
 * each via its own magic comment, or it errors at build/runtime:
 * - `@vite-ignore` — Vite.
 * - `webpackIgnore` — webpack.
 * - `turbopackIgnore` — Turbopack (Next.js). Without it, Turbopack can't analyze
 *   the parameter-derived specifier, can't bundle a `node:` builtin as a
 *   candidate, and replaces the call with a stub that throws
 *   `Cannot find module 'unknown'` — which silently disabled the Node
 *   `worker_threads` backend (→ single-threaded TFHE) in Next server components.
 *
 * Each comment instructs its bundler to emit a plain native `import()` that the
 * runtime resolves; in the browser that throws and is caught below (→ undefined).
 * Each capability accessor below wraps this and memoizes its result.
 */
async function _importNodeModule<mod>(name: string): Promise<mod | undefined> {
  try {
    const id = `node:${name}`;
    return (await import(/* @vite-ignore */ /* webpackIgnore: true */ /* turbopackIgnore: true */ id)) as mod;
  } catch {
    return undefined;
  }
}

type NodeBuffer = { from(str: string, encoding: string): BufferSource };

let _nodeBufferPromise: Promise<NodeBuffer | undefined> | undefined;

/**
 * Resolves Node's `Buffer` if `node:buffer` is importable, else `undefined`.
 *
 * Memoized — the answer can't change within a process lifetime. Returns the
 * `Buffer` reference (not a boolean) because `Buffer` is not a global: the
 * caller needs the actual constructor, so probing and use share one import.
 *
 * Returns `undefined` in non-Node runtimes that also lack `node:buffer`
 * (browsers, and edge runtimes: Vercel Edge, Cloudflare Workers, Next.js edge).
 */
export function getNodeBuffer(): Promise<NodeBuffer | undefined> {
  _nodeBufferPromise ??= (async () => {
    const mod = await _importNodeModule<{ Buffer?: NodeBuffer }>('buffer');
    return typeof mod?.Buffer?.from === 'function' ? mod.Buffer : undefined;
  })();
  return _nodeBufferPromise;
}

// Node's `readFile` returns a `Buffer`, which extends `Uint8Array` but whose
// `.buffer` TS 5.7+ widens to `ArrayBufferLike` (includes `SharedArrayBuffer`),
// making it incompatible with `BufferSource`. At runtime a `Buffer` is always
// backed by an `ArrayBuffer`, so typing the result as `BufferSource` here is a
// safe cast that lets callers pass the bytes to `WebAssembly.compile` directly.
type NodeFs = {
  readFile(path: string): Promise<BufferSource>;
  access(path: string): Promise<void>;
};

let _nodeFsPromise: Promise<NodeFs | undefined> | undefined;

/**
 * Resolves Node's `fs/promises` (`readFile`, `access`) if importable, else
 * `undefined` (browsers and edge runtimes). Memoized. See {@link getNodeBuffer}
 * for why this returns the module reference rather than a boolean.
 */
export function getNodeFs(): Promise<NodeFs | undefined> {
  _nodeFsPromise ??= (async () => {
    const mod = await _importNodeModule<NodeFs>('fs/promises');
    return typeof mod?.readFile === 'function' ? mod : undefined;
  })();
  return _nodeFsPromise;
}

type NodeUrl = { fileURLToPath(url: URL | string): string };

let _nodeUrlPromise: Promise<NodeUrl | undefined> | undefined;

/**
 * Resolves Node's `url` (`fileURLToPath`) if importable, else `undefined`
 * (browsers and edge runtimes). Memoized. See {@link getNodeBuffer} for why
 * this returns the module reference rather than a boolean.
 */
export function getNodeUrl(): Promise<NodeUrl | undefined> {
  _nodeUrlPromise ??= (async () => {
    const mod = await _importNodeModule<NodeUrl>('url');
    return typeof mod?.fileURLToPath === 'function' ? mod : undefined;
  })();
  return _nodeUrlPromise;
}

/**
 * Returns whether the Web Worker API is available (browsers, Deno, and other
 * web-standard runtimes). Synchronous — checks for the required globals only.
 */
export function supportsWebWorkerApi(): boolean {
  return (
    typeof Worker === 'function' &&
    typeof Blob === 'function' &&
    typeof URL !== 'undefined' &&
    typeof URL.createObjectURL === 'function'
  );
}

let _decompressionStreamSupported: boolean | undefined;

/**
 * Returns whether `DecompressionStream` is actually *usable* (plus the `Blob`
 * pipeline it feeds). Needed to inflate compressed embedded WASM.
 *
 * A `typeof` presence check is insufficient: some runtimes — notably the Next.js
 * Edge Runtime — expose `DecompressionStream` as a global **stub that throws on
 * construction**, so presence is a false positive that crashes later inside the
 * compile path. We therefore *construct* one (synchronous, cheap) and memoize the
 * result (support can't change within a process lifetime).
 *
 * Returns `false` on older browsers (Firefox <113, Safari <16.4) and edge runtimes,
 * so callers fall back (uncompressed / URL-based / JS-inflate WASM loading).
 */
export function supportsDecompressionStream(): boolean {
  if (_decompressionStreamSupported === undefined) {
    if (typeof DecompressionStream !== 'function' || typeof Blob !== 'function') {
      _decompressionStreamSupported = false;
    } else {
      try {
        // Construction is the probe — the Next.js Edge stub throws here; a real
        // implementation does not. Assigned (not bare `new`) so it reads as a value
        // probe, and discarded.
        const probe = new DecompressionStream('gzip');
        void probe;
        _decompressionStreamSupported = true;
      } catch {
        _decompressionStreamSupported = false;
      }
    }
  }
  return _decompressionStreamSupported;
}

/** Message envelope exchanged with a Node `worker_threads` worker. */
export interface MessageData {
  type: string;
  [key: string]: unknown;
}

/** Shape of a Node `worker_threads` worker instance the SDK relies on. */
export interface NodeMessagePort {
  on(event: 'error', listener: (error: Error) => void): void;
  on(event: 'exit', listener: (code: number) => void): void;
  on(event: string, listener: (data: MessageData) => void): void;
  off(event: string, listener: (data: MessageData) => void): void;
  postMessage(value: unknown): void;
  terminate(): Promise<number>;
}

type NodeWorkerConstructor = new (code: string | URL, options?: Record<string, unknown>) => NodeMessagePort;

let _nodeWorkerPromise: Promise<NodeWorkerConstructor | undefined> | undefined;

/**
 * Resolves Node's `worker_threads` `Worker` constructor if importable, else
 * `undefined` (browser, sandboxed Electron renderer, edge runtimes). Memoized.
 * See {@link getNodeBuffer} for why this returns the constructor rather than a
 * boolean.
 */
export function getNodeWorker(): Promise<NodeWorkerConstructor | undefined> {
  _nodeWorkerPromise ??= (async () => {
    const mod = await _importNodeModule<{ Worker?: NodeWorkerConstructor }>('worker_threads');
    return typeof mod?.Worker === 'function' ? mod.Worker : undefined;
  })();
  return _nodeWorkerPromise;
}
