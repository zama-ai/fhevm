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

/**
 * Compiles a WASM module from a URL, isomorphically (browser and Node.js).
 *
 * - Node + `file://`: uses `readFile` (Node `fetch` doesn't support `file://`)
 * - Browser + streaming: uses `WebAssembly.compileStreaming` (fastest path)
 * - Fallback: `fetch` + `arrayBuffer` + `WebAssembly.compile`
 */
export async function isomorphicCompileWasm(wasmUrl: URL): Promise<WebAssembly.Module> {
  const isBrowser = isBrowserLike();

  if (isBrowser) {
    const res = await _isomorphicFetchWasmResponse(wasmUrl);
    if (typeof WebAssembly.compileStreaming === 'function') {
      return await WebAssembly.compileStreaming(res);
    }
    return WebAssembly.compile(await res.arrayBuffer());
  }

  const bytes = await _isomorphicReadWasmBytes(wasmUrl);
  // Safe cast: Node's Buffer is always backed by ArrayBuffer at runtime.
  return WebAssembly.compile(bytes);
}

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
  const bytes = await _isomorphicDecodeBase64(wasmAsBase64);

  if (compressionFormat === undefined) {
    return WebAssembly.compile(bytes);
  } else {
    const stream = new Blob([bytes]).stream().pipeThrough(new DecompressionStream(compressionFormat));
    // compileStreaming requires Content-Type: application/wasm — supply it explicitly.
    return WebAssembly.compileStreaming(new Response(stream, { headers: { 'Content-Type': 'application/wasm' } }));
  }
}
