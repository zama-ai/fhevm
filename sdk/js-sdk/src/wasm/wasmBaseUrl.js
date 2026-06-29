// @ts-check

/**
 * Standalone WASM URL resolver.
 *
 * This file must remain plain JavaScript with no imports and no exported helper
 * functions. It is consumed from both ESM and generated CJS builds, and bundlers
 * may evaluate or rewrite it in unusual environments. Keep all environment
 * detection self-contained in this file.
 */

/**
 * Returns true in browser or web-worker-like runtimes where `import.meta.url`
 * is the authoritative module location, even if a bundler injects Node-shaped
 * globals such as `__filename`.
 *
 * Bun is excluded because it exposes web APIs while still supporting Node-style
 * filesystem resolution.
 *
 * @returns {boolean}
 */
function __isBrowserLike() {
  return (
    // @ts-ignore
    typeof Bun === 'undefined' &&
    // @ts-ignore
    typeof process === 'undefined' &&
    typeof location !== 'undefined' &&
    typeof location.href === 'string' &&
    typeof addEventListener === 'function' &&
    typeof removeEventListener === 'function'
  );
}

/**
 * Returns true only for real Node CommonJS execution, where `__filename` and
 * `require` refer to the current file and can safely be used to build a file URL.
 *
 * This deliberately rejects browser bundler shims that define `__filename`
 * without a real Node `process.versions.node`.
 *
 * @returns {boolean}
 */
function __isNodeCjsLike() {
  return (
    !__isBrowserLike() &&
    // @ts-ignore
    typeof __filename === 'string' &&
    // @ts-ignore
    typeof require === 'function' &&
    // @ts-ignore
    typeof process !== 'undefined' &&
    // @ts-ignore
    typeof process.versions?.node === 'string'
  );
}

function __resolveWasmBaseUrl() {
  if (__isNodeCjsLike()) {
    // Keep this indirect so bundlers do not rewrite `node:url` in browser builds.
    // @ts-ignore
    const nodeRequire = require;
    const nodeUrlModule = 'node:url';
    // @ts-ignore
    return nodeRequire(nodeUrlModule).pathToFileURL(__filename).href;
  }

  return import.meta.url;
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
