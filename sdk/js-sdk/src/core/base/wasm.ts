import { verifySha256 } from './bytes.js';
import { FetchError } from './errors/FetchError.js';
import { isDataUrlFetchSupported } from './fetch.js';
import { getNodeBuffer, getNodeFs, getNodeUrl, isBrowserLike, supportsDecompressionStream } from './environment.js';
import { inflateDecompress } from './inflate.js';

/**
 * Decodes a base64 string to bytes using the fastest available method.
 *
 * - `fetch("data:...")` — preferred, uses native base64 decoder
 * - `Buffer.from(str, "base64")` — Node.js fallback, native C++
 * - `atob` + loop — browser fallback when data URL fetch is blocked (e.g., CSP)
 */
async function _isomorphicDecodeBase64(base64: string): Promise<BufferSource> {
  if (!isBrowserLike()) {
    const NodeBuffer = await getNodeBuffer();
    if (NodeBuffer) {
      return NodeBuffer.from(base64, 'base64');
    }
    // Not a browser, but no node:buffer either (edge runtime). Fall through to
    // the web-standard data-URL fetch / atob paths below, which edge supports.
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
    const [fs, url] = await Promise.all([getNodeFs(), getNodeUrl()]);
    if (fs && url) {
      // The BufferSource-safe cast for Node's Buffer lives in getNodeFs's
      // return type (see environment.ts), so the bytes can flow straight into
      // WebAssembly.compile without copying.
      return fs.readFile(url.fileURLToPath(wasmUrl));
    }
    // Not a browser, but no node:fs either (edge runtime). Fall through to
    // fetch below — a file: URL will fail there, surfacing a clear error.
  }

  const res = await _isomorphicFetchWasmResponse(wasmUrl);
  return res.arrayBuffer();
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
  }

  // Preferred: stream through the platform DecompressionStream (native, zero-copy).
  if (supportsDecompressionStream()) {
    const stream = new Blob([bytes]).stream().pipeThrough(new DecompressionStream(compressionFormat));
    // compileStreaming requires Content-Type: application/wasm — supply it explicitly.
    return WebAssembly.compileStreaming(new Response(stream, { headers: { 'Content-Type': 'application/wasm' } }));
  }

  // Fallback for runtimes without a usable DecompressionStream — older browsers
  // (Firefox <113, Safari <16.4) and the Next.js Edge Runtime (whose stub throws on
  // construction). Inflate in pure JS, then compile. Keeps the small compressed
  // embedded payload working everywhere with no caller action.
  const compressed = bytes instanceof Uint8Array ? bytes : new Uint8Array(bytes as ArrayBuffer);
  return WebAssembly.compile(inflateDecompress(compressed, compressionFormat));
}
