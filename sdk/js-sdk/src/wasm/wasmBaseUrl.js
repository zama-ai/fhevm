// @ts-check

/**
 * Base URL for resolving WASM file paths relative to this module.
 * Works in both ESM and CJS contexts.
 *
 * Paths must be relative to `src/wasm/`.
 *
 * @type {string}
 */
export const wasmBaseUrl =
  typeof __filename !== "undefined"
    ? /* CJS */ require("node:url").pathToFileURL(__filename).href
    : /* ESM */ import.meta.url;
