import type { WasmAssetLoadMode, WasmAssetTransport } from '../types/wasmAssets.js';
import { verifySha256 } from './bytes.js';
import { FetchError } from './errors/FetchError.js';
import { isDataUrlFetchSupported } from './fetch.js';
import { isBrowserLike } from './isomorphicWorker.js';

/**
 * Decodes a base64 string to bytes using the fastest available method.
 *
 * - `fetch("data:...")` — preferred, uses native base64 decoder
 * - `Buffer.from(str, "base64")` — Node.js fallback, native C++
 * - `atob` + loop — browser fallback when data URL fetch is blocked (e.g., CSP)
 */
async function _isomorphicDecodeBase64(base64: string): Promise<BufferSource> {
  if (!isBrowserLike()) {
    const { Buffer } = await import('node:buffer');
    return Buffer.from(base64, 'base64');
  }

  if (await isDataUrlFetchSupported()) {
    const res = await fetch(`data:application/octet-stream;base64,${base64}`);
    return res.arrayBuffer();
  }

  // Ugly last resort: atob returns a binary string, not bytes.
  // No batch conversion exists — we must copy char-by-char.
  const binaryStr = atob(base64);
  const bytes = new Uint8Array(binaryStr.length);
  for (let i = 0; i < binaryStr.length; i++) {
    bytes[i] = binaryStr.charCodeAt(i);
  }

  return bytes;
}

async function _isomorphicFetchWasmResponse(wasmUrl: URL): Promise<Response> {
  const res = await fetch(wasmUrl);

  if (!res.ok) {
    throw new FetchError({
      message: `Failed to fetch WASM: ${res.status} ${res.statusText}`,
      url: wasmUrl.toString(),
    });
  }

  return res;
}

async function _isomorphicReadWasmBytes(wasmUrl: URL): Promise<BufferSource> {
  if (!isBrowserLike() && wasmUrl.protocol === 'file:') {
    const { readFile } = await import('node:fs/promises');
    const { fileURLToPath } = await import('node:url');

    // Node's Buffer extends Uint8Array but TS 5.7+ considers its .buffer as
    // ArrayBufferLike (includes SharedArrayBuffer), making it incompatible with
    // BufferSource. At runtime Buffer is always backed by ArrayBuffer, so the
    // cast at WebAssembly.compile is safe and avoids copying.
    return readFile(fileURLToPath(wasmUrl));
  }

  const res = await _isomorphicFetchWasmResponse(wasmUrl);
  return res.arrayBuffer();
}

// /**
//  * Compiles a WASM module from a URL, isomorphically (browser and Node.js).
//  *
//  * - Node + `file://`: uses `readFile` (Node `fetch` doesn't support `file://`)
//  * - Browser + streaming: uses `WebAssembly.compileStreaming` (fastest path)
//  * - Fallback: `fetch` + `arrayBuffer` + `WebAssembly.compile`
//  */
// export async function isomorphicCompileWasm(wasmUrl: URL): Promise<WebAssembly.Module> {
//   const isBrowser = isBrowserLike();

//   if (isBrowser) {
//     const res = await _isomorphicFetchWasmResponse(wasmUrl);
//     if (typeof WebAssembly.compileStreaming === 'function') {
//       return await WebAssembly.compileStreaming(res);
//     }
//     return WebAssembly.compile(await res.arrayBuffer());
//   }

//   const bytes = await _isomorphicReadWasmBytes(wasmUrl);
//   // Safe cast: Node's Buffer is always backed by ArrayBuffer at runtime.
//   return WebAssembly.compile(bytes);
// }

/**
 * Fetches/reads a WASM module, verifies its SHA-256 digest, then compiles it.
 *
 * This intentionally does not use `WebAssembly.compileStreaming`: integrity
 * verification needs the full byte array before the module can be trusted.
 *
 * @param wasmUrl - URL of the WASM binary to fetch/read.
 * @param expectedSha256 - Expected SHA-256 hex digest, with or without `0x`.
 */
export async function isomorphicCompileVerifiedWasm(wasmUrl: URL, expectedSha256: string): Promise<WebAssembly.Module> {
  // trusted-direct-url === do not verify
  // precheck-direct-url

  const bytes = await _isomorphicReadWasmBytes(wasmUrl);
  await verifySha256(bytes, expectedSha256, { subject: `WASM ${wasmUrl.toString()}` });

  // Safe cast: Node's Buffer is always backed by ArrayBuffer at runtime.
  return WebAssembly.compile(bytes);
}

/**
 * Compiles a WASM module from a base64-encoded string.
 * Minimum content security policy (CSP):
 *
 *    Content-Security-Policy: script-src 'self' 'wasm-unsafe-eval'
 *
 * @param wasmAsBase64 - The WASM binary encoded as a base64 string
 * @param compressionFormat - `'gzip' | 'deflate' | 'deflate-raw'`, or
 * `undefined` when the payload is raw wasm.
 * @returns A compiled WebAssembly.Module ready to be instantiated
 */
export async function isomorphicCompileWasmFromBase64(
  wasmAsBase64: string,
  compressionFormat?: CompressionFormat,
): Promise<WebAssembly.Module> {
  // verified-blob == with verification
  // embedded-base64 (do not verifiy)
  const bytes = await _isomorphicDecodeBase64(wasmAsBase64);

  if (compressionFormat === undefined) {
    return WebAssembly.compile(bytes);
  } else {
    const stream = new Blob([bytes]).stream().pipeThrough(new DecompressionStream(compressionFormat));
    // compileStreaming requires Content-Type: application/wasm — supply it explicitly.
    return WebAssembly.compileStreaming(new Response(stream, { headers: { 'Content-Type': 'application/wasm' } }));
  }
}

/**
 * Maps a WASM asset load mode to the transport option(s) it may use.
 *
 * Explicit modes have one transport. `auto` returns `url-or-base64` because
 * the final transport is selected later: URL when a usable asset URL is
 * available, otherwise embedded base64.
 *
 * @param loadMode - The configured WASM asset loading strategy.
 * @returns The certain transport for explicit modes, or `both` for `auto`.
 */
export function toWasmAssetTransport(loadMode: WasmAssetLoadMode): WasmAssetTransport {
  if (loadMode === 'auto') {
    return 'either';
  }
  if (loadMode === 'embedded-base64') {
    return 'base64';
  }
  return 'url';
}

/**
 * Returns whether the SDK should perform a SHA-256 check for this load mode.
 *
 * `embedded-base64` does not need a runtime SHA-256 check because the bytes are
 * bundled with the SDK. `trusted-direct-url` deliberately skips SDK verification
 * and trusts the configured URL. `auto` requires a SHA-256 check only when it
 * can use the URL path; without a usable URL, it falls back to embedded base64.
 * `precheck-direct-url` performs a preflight check only and does not verify the
 * bytes later fetched by the runtime.
 *
 * @param loadMode - The configured WASM asset loading strategy.
 * @param hasUrl - Whether a usable asset URL is available for URL-backed loading.
 * @returns `true` when an SDK-side SHA-256 check is required.
 */ export function wasmLoadRequiresSha256Check(loadMode: WasmAssetLoadMode, hasUrl: boolean): boolean {
  if (loadMode === 'embedded-base64' || loadMode === 'trusted-direct-url') {
    return false;
  }
  if (loadMode === 'auto' && !hasUrl) {
    return false;
  }
  return true;
}

/**
 * Returns whether the load mode needs blob/eval worker support.
 *
 * `verified-blob`, `embedded-base64`, and `auto` create workers from SDK-owned
 * bytes/code (Blob URLs in browsers, eval workers in Node), so they require the
 * inline worker path to be available. `precheck-direct-url` and
 * `trusted-direct-url` hand a URL directly to the runtime with `new Worker(url)`,
 * so they do not require blob/eval worker support.
 *
 * @param loadMode - The configured WASM asset loading strategy.
 * @returns `true` when the load mode needs blob/eval worker support.
 */
export function wasmLoadRequiresBlob(loadMode: WasmAssetLoadMode): boolean {
  return loadMode === 'verified-blob' || loadMode === 'embedded-base64' || loadMode === 'auto';
}

/**
 * Resolves the effective WASM module compilation mode.
 *
 * Module compilation currently has two paths: precheck and compile bytes from
 * a URL, or compile SDK-embedded base64 bytes. Therefore every URL-backed load
 * mode collapses to `precheck-direct-url` for module compilation, while
 * `embedded-base64` and URL-less `auto` collapse to `embedded-base64`.
 *
 * This helper intentionally keeps the `WasmAssetLoadMode` vocabulary even
 * though only `precheck-direct-url` and `embedded-base64` are returned. WASM
 * module compilation does not use Blob/eval workers; `verified-blob` is a
 * worker-loading mode, not the closest module-compilation mode.
 *
 * @param loadMode - The configured WASM asset loading strategy.
 * @param hasUrl - Whether a usable asset URL is available for URL-backed loading.
 * @returns `precheck-direct-url` for URL-backed module compilation, otherwise
 * `embedded-base64`.
 */
export function toWasmModuleLoadMode(loadMode: WasmAssetLoadMode, hasUrl: boolean): WasmAssetLoadMode {
  if (!hasUrl || loadMode === 'embedded-base64') {
    return 'embedded-base64';
  }
  return 'precheck-direct-url';
}
