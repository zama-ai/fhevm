import { FetchError } from "./errors/FetchError.js";
import { isDataUrlFetchSupported } from "./fetch.js";
import { isBrowserLike } from "./isomorphicWorker.js";

/**
 * Decodes a base64 string to bytes using the fastest available method.
 *
 * - `fetch("data:...")` — preferred, uses native base64 decoder
 * - `Buffer.from(str, "base64")` — Node.js fallback, native C++
 * - `atob` + loop — browser fallback when data URL fetch is blocked (e.g., CSP)
 */
async function isomorphicDecodeBase64(base64: string): Promise<BufferSource> {
  if (!isBrowserLike()) {
    const { Buffer } = await import("node:buffer");
    return Buffer.from(base64, "base64");
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

/**
 * Compiles a WASM module from a URL, isomorphically (browser and Node.js).
 *
 * - Node + `file://`: uses `readFile` (Node `fetch` doesn't support `file://`)
 * - Browser + streaming: uses `WebAssembly.compileStreaming` (fastest path)
 * - Fallback: `fetch` + `arrayBuffer` + `WebAssembly.compile`
 */
export async function isomorphicCompileWasm(
  wasmUrl: URL,
): Promise<WebAssembly.Module> {
  const isBrowser = isBrowserLike();

  let bytes: BufferSource;

  if (!isBrowser && wasmUrl.protocol === "file:") {
    // Node + file url
    const { readFile } = await import("node:fs/promises");
    const { fileURLToPath } = await import("node:url");

    bytes = await readFile(fileURLToPath(wasmUrl));
  } else {
    // fetch wasm
    const res = await fetch(wasmUrl);

    if (!res.ok) {
      throw new FetchError({
        message: `Failed to fetch WASM: ${res.status} ${res.statusText}`,
        url: wasmUrl.toString(),
      });
    }

    if (isBrowser) {
      // use compile streaming
      if (typeof WebAssembly.compileStreaming === "function") {
        try {
          return await WebAssembly.compileStreaming(res);
        } catch (e) {
          throw new Error(
            "WebAssembly.compileStreaming failed. Ensure the server serves .wasm files with Content-Type: application/wasm",
            { cause: e },
          );
        }
      }
    }

    bytes = await res.arrayBuffer();
  }

  return WebAssembly.compile(bytes);
}

/**
 * Compiles a WASM module from a base64-encoded string.
 *
 * @param wasmAsBase64 - The WASM binary encoded as a base64 string
 * @returns A compiled WebAssembly.Module ready to be instantiated
 */
export async function isomorphicCompileWasmFromBase64(
  wasmAsBase64: string,
): Promise<WebAssembly.Module> {
  const bytes = await isomorphicDecodeBase64(wasmAsBase64);
  return WebAssembly.compile(bytes);
}
