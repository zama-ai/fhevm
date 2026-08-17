// @ts-check

/**
 * Standalone WASM URL resolver.
 *
 * This file must remain plain JavaScript with no imports and no exported helper
 * functions. It is consumed from both ESM and generated CJS builds, and bundlers
 * may evaluate or rewrite it in unusual environments. Keep all environment
 * detection self-contained in this file.
 */

function __resolveWasmBaseUrl() {
  // In the CommonJS build tsc lowers `import.meta` to `{}`, so a usable string
  // here means the ESM build (browser, worker, or Node ESM) — it's authoritative.
  const metaUrl = import.meta.url;
  if (typeof metaUrl === 'string' && metaUrl.length > 0) {
    return metaUrl;
  }

  // CommonJS build: trust `__filename` ONLY in a genuine Node process, never a
  // bundler shim. Turbopack/webpack inject `__filename` + a partial `process`
  // into client bundles, but never populate `process.versions.node`.
  // @ts-ignore
  const isRealNode = typeof process !== 'undefined' && typeof process.versions?.node === 'string';
  // @ts-ignore
  if (isRealNode && typeof __filename === 'string' && typeof require === 'function') {
    // @ts-ignore — keep `node:url` indirect so bundlers don't rewrite it
    return require('node:url').pathToFileURL(__filename).href;
  }

  return metaUrl; // undefined → caller surfaces a clear "could not resolve" error
}

/**
 * Base URL for resolving WASM file paths relative to this module.
 * Works in both ESM and CJS contexts.
 *
 * Paths must be relative to `src/wasm/`.
 *
 * @type {string}
 */
export const wasmBaseUrl = __resolveWasmBaseUrl();
