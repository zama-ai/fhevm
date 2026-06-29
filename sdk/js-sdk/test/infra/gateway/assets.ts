// Serves the SDK's raw WASM/worker assets so the browser can fetch them same-origin
// (via the gateway) for the URL-based wasm-load modes (verified-blob / *-direct-url /
// auto). Same-origin keeps it COEP-compatible; embedded-base64 never needs this.
//
// The SDK requests assets by VERSIONED filename (the `filename` field in its asset
// metadata), e.g. `tfhe_bg.v1.6.1.wasm` / `tfhe-worker.v1.6.1.mjs`. On disk those live
// under the SDK's wasm dir as `<module>/v<version>/<base>.<ext>`. This maps one to the
// other (and refuses anything that doesn't match, so a URL can't escape the dir).

import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

// `<base>.v<version>.<ext>` — base is alnum/_/-, version starts with a digit. The
// strict shape doubles as path-traversal protection (no slashes or `..`).
const VERSIONED_ASSET = /^([A-Za-z0-9_-]+)\.v([0-9][0-9A-Za-z.\-]*)\.(wasm|mjs)$/;

/** Maps a versioned asset filename to its on-disk path under `assetDir`, or undefined. */
export function resolveWasmAssetPath(assetDir: string, filename: string): string | undefined {
  const match = VERSIONED_ASSET.exec(filename);
  if (match === null) {
    return undefined;
  }
  const [, base, version, ext] = match;
  const moduleDir = base!.startsWith('kms') ? 'tkms' : 'tfhe';
  return resolve(assetDir, moduleDir, `v${version!}`, `${base!}.${ext!}`);
}

export type WasmAsset = {
  readonly contentType: string;
  readonly bytes: Uint8Array;
};

/** Reads a raw WASM/worker asset by versioned filename, or undefined if absent/invalid. */
export function readWasmAsset(assetDir: string, filename: string): WasmAsset | undefined {
  const path = resolveWasmAssetPath(assetDir, filename);
  if (path === undefined) {
    return undefined;
  }
  let bytes: Uint8Array;
  try {
    bytes = readFileSync(path);
  } catch {
    return undefined;
  }
  // `.mjs` is loaded via `new Worker(url, { type: 'module' })`; `.wasm` via fetch+compile.
  const contentType = filename.endsWith('.wasm') ? 'application/wasm' : 'text/javascript';
  return { contentType, bytes };
}
