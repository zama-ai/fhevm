/**
 * Auto-generated from scripts/wasm/tfhe/startWorkers.template.js.
 * Embedded worker base64 payload SHA-256: c6beb212e48e93e3813f82c35f96125dae236d4369aed0b3992adcb4c925cb52
 */

/**
 * Worker load mode security guarantees:
 *
 * embedded-base64    Integrity: build-time. Inherits the JS bundle's integrity.
 * verified-blob      Integrity: runtime SHA-256 of fetched bytes; executed bytes
 *                    are the verified bytes themselves.
 * precheck-direct-url  No integrity guarantee. The SDK fetches the URL once and
 *                      validates SHA-256, then the runtime fetches the URL a
 *                      second time and executes those (unverified) bytes. Use
 *                      for fail-fast on misconfigured URLs / wrong builds, not
 *                      for protection against on-path or CDN-edge tampering.
 * trusted-direct-url No integrity check. Use only when the URL is fully trusted
 *                    (e.g., same-origin static asset).
 * auto               Tries verified-blob if workerUrl is set, falls back to
 *                    embedded-base64 on any non-SHA-256 error. SHA-256 mismatch
 *                    is always fatal and never falls back.
 */

/**
 * Module invariants.
 *
 * Lifecycle (one-shot, no retry):
 *  - setWorkerUrlConfig() and startWorkers() each at most once.
 *  - A failed startWorkers() locks the module; no retry, no reconfigure.
 *  - terminateWorkers() throws while startWorkers() is in flight; idempotent after.
 *
 * Concurrency:
 *  - _started and _starting are check-and-set without intervening await.
 *  - Parallel workers dedupe to a single fetch+verify (cached promise).
 *
 * Security:
 *  - SHA-256 mismatch (Sha256MismatchError) is always fatal — never falls back.
 *  - Hash is the build-time constant "348fe6c2e77bafe0cbfb9c0a512af99b5f97de1119ceadf07cc34620dbc5690e".
 *  - auto silently falls back to embedded-base64 on any non-SHA-256 error.
 *  - precheck-direct-url's SHA-256 check is informational; the runtime refetches.
 *
 * Errors:
 *  - Partial worker-pool failure: successful workers are terminated before throw.
 *  - Concurrent failures: only the first error is surfaced.
 *  - __waitForMsgType has no timeout — a silent worker hangs startWorkers().
 *
 * Resources:
 *  - Blob URLs are revoked on both success and synchronous-constructor failure.
 *  - _verifiedWorkerUrlBytesPromise is cleared in startWorkers()'s finally.
 *  - _workers is a strong reference (owns the shared WebAssembly memory).
 *  - __waitForMsgType listeners are not removed if the worker never replies.
 *
 * Caller contract:
 *  - workerUrl: URL instance (required for verified/checked/trusted-direct-url).
 *  - logger: must expose debug(message) and error(message, cause).
 *  - wasmAssetLoadMode: one of __wasmAssetLoadModes.
 */

////////////////////////////////////////////////////////////////////////////////
// Runtime detection and load modes
////////////////////////////////////////////////////////////////////////////////

function __isBrowserLike() {
  return (
    typeof Bun === 'undefined' &&
    typeof process === 'undefined' &&
    typeof addEventListener === 'function' &&
    typeof removeEventListener === 'function'
  );
}

const __wasmAssetLoadModes = ['embedded-base64', 'verified-blob', 'precheck-direct-url', 'trusted-direct-url', 'auto'];

function __isWasmAssetLoadMode(value) {
  return __wasmAssetLoadModes.includes(value);
}

////////////////////////////////////////////////////////////////////////////////
// SHA-256 verification
////////////////////////////////////////////////////////////////////////////////

function __bytesToHex(bytes) {
  return [...bytes].map((b) => b.toString(16).padStart(2, '0')).join('');
}

/**
 * Computes the SHA-256 digest of worker bytes and returns it as lowercase hex.
 * @param {ArrayBuffer | Uint8Array} bytes Worker bytes to hash.
 * @returns {Promise<string>} Lowercase hexadecimal SHA-256 digest without a `0x` prefix.
 */
async function __sha256(bytes) {
  if (__isBrowserLike()) {
    if (typeof crypto === 'undefined' || !crypto.subtle || typeof crypto.subtle.digest !== 'function') {
      throw new Error('Web Crypto SHA-256 digest is not available');
    }

    const hash = await crypto.subtle.digest('SHA-256', bytes);
    return __bytesToHex(new Uint8Array(hash));
  }

  const nodeModuleName = 'crypto';
  const nodeModuleId = `node:${nodeModuleName}`;
  const { createHash } = await import(/* @vite-ignore */ nodeModuleId);
  return createHash('sha256').update(new Uint8Array(bytes)).digest('hex');
}

/**
 * Verifies that worker bytes match the expected SHA-256 digest.
 * @param {ArrayBuffer | Uint8Array} bytes Worker bytes to verify.
 * @param {string} expectedSha256 Expected lowercase hex digest without a `0x` prefix.
 * @param {string} url Url to verify.
 * @returns {Promise<void>} Resolves when the digest matches.
 * @throws {Error} Throws a `Sha256MismatchError` when the digest does not match.
 */
async function __verifySha256(bytes, expectedSha256, url) {
  const actualSha256 = await __sha256(bytes);

  if (actualSha256 !== expectedSha256) {
    const error = new Error(`SHA-256 mismatch: expected ${expectedSha256}, got ${actualSha256}. url=${url}`);
    error.name = 'Sha256MismatchError';
    throw error;
  }
}

function __isSha256MismatchError(error) {
  return error?.name === 'Sha256MismatchError';
}

////////////////////////////////////////////////////////////////////////////////
// Worker URL byte loading
////////////////////////////////////////////////////////////////////////////////

/**
 * Reads worker script bytes from a URL and verifies their SHA-256 digest.
 * @param {URL} url Worker script URL.
 * @param {string} expectedSha256 Expected lowercase hex digest without a `0x` prefix.
 * @returns {Promise<ArrayBuffer | Uint8Array>} Verified worker script bytes.
 */
async function __fetchAndVerifyWorkerUrlBytes(url, expectedSha256) {
  const bytes = await __readWorkerUrlBytes(url);

  await __verifySha256(bytes, expectedSha256, url);

  return bytes;
}

/**
 * Reads worker script bytes from a URL.
 * Uses the filesystem for Node `file:` URLs, otherwise falls back to `fetch`.
 * Assumes `fetch` exists for non-`file:` URLs.
 * @param {URL} url Worker script URL.
 * @returns {Promise<ArrayBuffer | Uint8Array>} Raw worker script bytes.
 */
async function __readWorkerUrlBytes(url) {
  if (!__isBrowserLike() && url.protocol === 'file:') {
    const fsModuleName = 'fs/promises';
    const fsModuleId = `node:${fsModuleName}`;
    const urlModuleName = 'url';
    const urlModuleId = `node:${urlModuleName}`;
    const { readFile } = await import(/* @vite-ignore */ fsModuleId);
    const { fileURLToPath } = await import(/* @vite-ignore */ urlModuleId);
    return await readFile(fileURLToPath(url));
  }

  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch worker: ${response.status} ${response.statusText}`);
  }

  return await response.arrayBuffer();
}

////////////////////////////////////////////////////////////////////////////////
// Worker construction primitives
////////////////////////////////////////////////////////////////////////////////

/**
 * Creates a browser module Worker from a Blob.
 * Browser-only: creates a Blob URL and returns it so the caller can revoke it after worker startup.
 * @param {Blob} blob JavaScript worker source Blob.
 * @returns {Promise<{ worker: Worker, blobUrl: string }>} Created browser worker and Blob URL to revoke.
 */
async function __newBrowserWorkerFromBlob(blob) {
  const blobUrl = URL.createObjectURL(blob);

  try {
    const worker = new Worker(blobUrl, {
      type: 'module',
      name: 'wasm_bindgen_worker',
    });

    return { worker, blobUrl };
  } catch (e) {
    URL.revokeObjectURL(blobUrl);
    throw e;
  }
}

/**
 * Creates a module Worker that loads its script directly from a URL object, isomorphically.
 * Browsers use the global Worker; Node uses worker_threads' Worker.
 * @param {URL} url Worker script URL.
 * @returns {Promise<Worker>} Created worker.
 */
async function __newIsomorphicWorkerFromUrl(url) {
  if (__isBrowserLike()) {
    return new Worker(url, {
      type: 'module',
      name: 'wasm_bindgen_worker',
    });
  }

  const nodeModuleName = 'worker_threads';
  const nodeModuleId = `node:${nodeModuleName}`;
  const { Worker: NodeWorker } = await import(/* @vite-ignore */ nodeModuleId);
  return new NodeWorker(url);
}

async function __newNodeWorkerFromJsCode(jsCode) {
  const nodeModuleName = 'worker_threads';
  const nodeModuleId = `node:${nodeModuleName}`;
  const { Worker: NodeWorker } = await import(/* @vite-ignore */ nodeModuleId);
  return { worker: new NodeWorker(jsCode, { eval: true }), blobUrl: undefined };
}

/**
 * Creates a worker from already verified JavaScript source bytes.
 * 1. Caller must provide bytes returned by the SHA-256 verification path.
 * 2. In browsers, wrap the verified bytes in a Blob URL and create a module Worker.
 * 3. In Node, decode the verified bytes as UTF-8 JavaScript and create a worker_threads eval Worker.
 * @param {ArrayBuffer | Uint8Array} verifiedJsCodeBytes SHA-256 verified JavaScript source bytes.
 * @returns {Promise<{ worker: Worker, blobUrl: string | undefined }>} Created worker and optional Blob URL to revoke.
 */
async function __newIsomorphicWorkerFromVerifiedJsCodeBytes(verifiedJsCodeBytes) {
  if (__isBrowserLike()) {
    return await __newBrowserWorkerFromBlob(
      new Blob([verifiedJsCodeBytes], {
        type: 'application/javascript',
      }),
    );
  }

  return await __newNodeWorkerFromJsCode(Buffer.from(verifiedJsCodeBytes).toString('utf-8'));
}

/**
 * Creates a worker from the SDK-embedded base64 JavaScript source.
 * 1. In browsers, decode the base64 source into a Blob URL and create a module Worker.
 * 2. In Node, decode the base64 source into UTF-8 JavaScript and create a worker_threads eval Worker.
 * @param {string} jsCodeBase64 Base64-encoded JavaScript worker source.
 * @returns {Promise<{ worker: Worker, blobUrl: string | undefined }>} Created worker and optional Blob URL to revoke.
 */
async function __newWorkerFromJsCodeBase64(jsCodeBase64) {
  if (__isBrowserLike()) {
    const blob = new Blob([atob(jsCodeBase64)], {
      type: 'application/javascript',
    });

    const blobUrl = URL.createObjectURL(blob);

    try {
      const worker = new Worker(blobUrl, {
        type: 'module',
        name: 'wasm_bindgen_worker',
      });

      return { worker, blobUrl };
    } catch (e) {
      URL.revokeObjectURL(blobUrl);
      throw e;
    }
  }

  const code = Buffer.from(jsCodeBase64, 'base64').toString('utf-8');
  return await __newNodeWorkerFromJsCode(code);
}

////////////////////////////////////////////////////////////////////////////////
// Worker message protocol
////////////////////////////////////////////////////////////////////////////////

function __waitForMsgType(target, type) {
  return new Promise((resolve, reject) => {
    function cleanup() {
      if (typeof target.removeEventListener === 'function') {
        target.removeEventListener('message', onBrowserMsg);
        target.removeEventListener('error', onBrowserError);
      } else {
        target.off('message', onNodeMsg);
        target.off('error', onNodeError);
        target.off('exit', onNodeExit);
      }
    }

    function onBrowserMsg({ data }) {
      if (data?.type !== type) return;
      cleanup();
      resolve(data);
    }

    function onBrowserError(e) {
      cleanup();
      reject(e.error || new Error('Worker error'));
    }

    function onNodeMsg(data) {
      if (data?.type !== type) return;
      cleanup();
      resolve(data);
    }

    function onNodeError(err) {
      cleanup();
      reject(err);
    }

    function onNodeExit(code) {
      cleanup();
      reject(new Error(`Worker exited with code ${code}`));
    }

    if (typeof target.removeEventListener === 'function') {
      target.addEventListener('message', onBrowserMsg);
      target.addEventListener('error', onBrowserError);
    } else {
      target.on('message', onNodeMsg);
      target.on('error', onNodeError);
      target.on('exit', onNodeExit);
    }
  });
}

////////////////////////////////////////////////////////////////////////////////
// Module state
////////////////////////////////////////////////////////////////////////////////

let _terminating;
let _configSet = false;
let _workerUrl = undefined;
let _wasmAssetLoadMode = 'auto';
const _workerUrlSha256 = "348fe6c2e77bafe0cbfb9c0a512af99b5f97de1119ceadf07cc34620dbc5690e";
let _verifiedWorkerUrlBytesPromise = undefined;
let _logger = undefined;
let _started = false;
// True only while the body of startWorkers() is executing.
// Reset by the try/finally in startWorkers, so a failed start still allows
// terminateWorkers() to be called (and become a no-op when _workers is unset).
let _starting = false;

// Keep workers strongly referenced while they own shared WebAssembly memory.
let _workers;

function getTfheWorkers() {
  return _workers;
}

////////////////////////////////////////////////////////////////////////////////
// Configuration API
////////////////////////////////////////////////////////////////////////////////

function __assertLogger(logger) {
  if (logger === undefined) {
    return;
  }

  if (typeof logger.debug !== 'function' || typeof logger.error !== 'function') {
    throw new TypeError('logger must expose debug(message) and error(message, cause) functions');
  }
}

function setWorkerUrlConfig(parameters = {}) {
  if (_configSet) {
    throw new Error('Cannot set worker URL config: config was already set');
  }

  if (_started) {
    throw new Error('Cannot set worker URL config after workers have started');
  }

  if (parameters === null || typeof parameters !== 'object') {
    throw new TypeError('setWorkerUrlConfig parameters must be an object');
  }

  const { workerUrl = undefined, wasmAssetLoadMode = 'auto', logger = undefined } = parameters;

  // Check `wasmAssetLoadMode`
  if (!__isWasmAssetLoadMode(wasmAssetLoadMode)) {
    throw new TypeError(`wasmAssetLoadMode must be one of: ${__wasmAssetLoadModes.join(', ')}`);
  }

  // Check `workerUrl`
  if (workerUrl !== undefined) {
    if (!(workerUrl instanceof URL)) {
      throw new TypeError('workerUrl must be a URL');
    }
    _workerUrl = workerUrl;
  } else {
    if (
      wasmAssetLoadMode === 'verified-blob' ||
      wasmAssetLoadMode === 'precheck-direct-url' ||
      wasmAssetLoadMode === 'trusted-direct-url'
    ) {
      throw new Error(`workerUrl is required when wasmAssetLoadMode is "${wasmAssetLoadMode}"`);
    }
  }

  // Check `logger`
  __assertLogger(logger);

  _wasmAssetLoadMode = wasmAssetLoadMode;
  _logger = logger;
  _configSet = true;
}

////////////////////////////////////////////////////////////////////////////////
// Worker source strategies
////////////////////////////////////////////////////////////////////////////////

/**
 * Returns the cached verification promise for the configured worker URL.
 * The first call reads `_workerUrl` and verifies it against `_workerUrlSha256`; later calls reuse the same promise
 * so parallel workers do not refetch or rehash the script.
 * @returns {Promise<ArrayBuffer | Uint8Array>} Verified worker script bytes.
 * @throws {Error} If no worker URL is configured.
 */
function __getVerifiedWorkerUrlBytesPromise() {
  const workerUrl = _workerUrl;

  if (workerUrl === undefined) {
    throw new Error('workerUrl is required to verify worker URL bytes');
  }

  if (_verifiedWorkerUrlBytesPromise !== undefined) {
    return _verifiedWorkerUrlBytesPromise;
  }

  _verifiedWorkerUrlBytesPromise = __fetchAndVerifyWorkerUrlBytes(workerUrl, _workerUrlSha256);
  return _verifiedWorkerUrlBytesPromise;
}

/**
 * Creates a worker from the configured URL after SHA-256 verification.
 * 1. Reuse cached verified bytes.
 * 2. Execute those exact bytes as a Blob worker in browsers.
 * 3. Execute those exact bytes as an eval worker in Node.
 * @returns {Promise<{ worker: Worker, blobUrl: string | undefined }>} Created worker and optional Blob URL to revoke.
 */
async function __createWorkerFromVerifiedWorkerUrl() {
  const verifiedWorkerBytes = await __getVerifiedWorkerUrlBytesPromise();
  return await __newIsomorphicWorkerFromVerifiedJsCodeBytes(verifiedWorkerBytes);
}

/**
 * Creates a worker by passing the configured URL directly to the runtime.
 * 1. Require a configured worker URL.
 * 2. Do not perform SDK byte verification.
 * 3. Let the browser or Node runtime load and execute the URL directly.
 * @returns {Promise<{ worker: Worker, blobUrl: undefined }>} Created worker with no Blob URL to revoke.
 */
async function __createWorkerFromTrustedDirectWorkerUrl() {
  if (_workerUrl === undefined) {
    throw new Error('workerUrl is required to create a trusted direct worker');
  }

  return { worker: await __newIsomorphicWorkerFromUrl(_workerUrl), blobUrl: undefined };
}

/**
 * Creates a worker by passing the configured URL directly to the runtime, after a pre-flight SHA-256 probe.
 *
 * IMPORTANT: this is NOT an integrity check. The SDK fetches the URL once to validate
 * the hash, then hands the URL to the runtime, which fetches it a SECOND time and
 * executes those bytes. The two fetches are independent — the executed bytes are
 * never verified. Use only for fail-fast on misconfigured URLs / build mismatches.
 *
 * For an actual integrity guarantee, use `verified-blob` (requires CSP allowing blob: workers).
 *
 * 1. Fetch the URL and verify its SHA-256 against "348fe6c2e77bafe0cbfb9c0a512af99b5f97de1119ceadf07cc34620dbc5690e" — fails fast on mismatch.
 * 2. Discard the verified bytes.
 * 3. Let the runtime fetch the same URL again and execute it (no verification on this fetch).
 * @returns {Promise<{ worker: Worker, blobUrl: undefined }>} Created worker with no Blob URL to revoke.
 */
async function __createWorkerFromCheckedDirectWorkerUrl() {
  await __getVerifiedWorkerUrlBytesPromise();
  return await __createWorkerFromTrustedDirectWorkerUrl();
}

/**
 * Creates a worker from the SDK-embedded base64 worker source.
 * 1. Read the base64-encoded JavaScript source baked into this module.
 * 2. Decode into a Blob URL and create a module Worker in browsers.
 * 3. Decode into UTF-8 JavaScript and create a worker_threads eval Worker in Node.
 * @returns {Promise<{ worker: Worker, blobUrl: string | undefined }>} Created worker and optional Blob URL to revoke.
 */
async function __createWorkerFromBase64() {
  const workerBase64 = "ZnVuY3Rpb24gX19faXNCcm93c2VyTGlrZSgpIHsKICByZXR1cm4gKAogICAgdHlwZW9mIGFkZEV2ZW50TGlzdGVuZXIgPT09ICdmdW5jdGlvbicgJiYKICAgIHR5cGVvZiByZW1vdmVFdmVudExpc3RlbmVyID09PSAnZnVuY3Rpb24nCiAgKTsKfQoKYXN5bmMgZnVuY3Rpb24gX19fZ2V0VGFyZ2V0KCkgewogIGlmIChfX19pc0Jyb3dzZXJMaWtlKCkpIHJldHVybiBzZWxmOwogIGNvbnN0IG5vZGVNb2R1bGVOYW1lID0gJ3dvcmtlcl90aHJlYWRzJzsKICBjb25zdCBub2RlTW9kdWxlSWQgPSBgbm9kZToke25vZGVNb2R1bGVOYW1lfWA7CiAgY29uc3QgeyBwYXJlbnRQb3J0IH0gPSBhd2FpdCBpbXBvcnQoLyogQHZpdGUtaWdub3JlICovIG5vZGVNb2R1bGVJZCk7CiAgcmV0dXJuIHBhcmVudFBvcnQ7Cn0KCmZ1bmN0aW9uIF9fX3dhaXRGb3JNc2dUeXBlKHRhcmdldCwgdHlwZSkgewogIHJldHVybiBuZXcgUHJvbWlzZSgocmVzb2x2ZSkgPT4gewogICAgaWYgKHR5cGVvZiB0YXJnZXQub24gPT09ICdmdW5jdGlvbicpIHsKICAgICAgLy8gTm9kZTogRXZlbnRFbWl0dGVyLCBkYXRhIHBhc3NlZCBkaXJlY3RseQogICAgICB0YXJnZXQub24oJ21lc3NhZ2UnLCBmdW5jdGlvbiBvbk1zZyhkYXRhKSB7CiAgICAgICAgaWYgKGRhdGE/LnR5cGUgIT09IHR5cGUpIHJldHVybjsKICAgICAgICB0YXJnZXQub2ZmKCdtZXNzYWdlJywgb25Nc2cpOwogICAgICAgIHJlc29sdmUoZGF0YSk7CiAgICAgIH0pOwogICAgfSBlbHNlIHsKICAgICAgLy8gQnJvd3NlcjogRE9NIGV2ZW50cywgZGF0YSB3cmFwcGVkIGluIE1lc3NhZ2VFdmVudAogICAgICB0YXJnZXQuYWRkRXZlbnRMaXN0ZW5lcignbWVzc2FnZScsIGZ1bmN0aW9uIG9uTXNnKHsgZGF0YSB9KSB7CiAgICAgICAgaWYgKGRhdGE/LnR5cGUgIT09IHR5cGUpIHJldHVybjsKICAgICAgICB0YXJnZXQucmVtb3ZlRXZlbnRMaXN0ZW5lcignbWVzc2FnZScsIG9uTXNnKTsKICAgICAgICByZXNvbHZlKGRhdGEpOwogICAgICB9KTsKICAgIH0KICB9KTsKfQoKX19fZ2V0VGFyZ2V0KCkudGhlbigodGFyZ2V0KSA9PgogIF9fX3dhaXRGb3JNc2dUeXBlKHRhcmdldCwgJ3dhc21fYmluZGdlbl93b3JrZXJfaW5pdCcpLnRoZW4oCiAgICBhc3luYyAoeyBpbml0LCByZWNlaXZlciB9KSA9PiB7CiAgICAgIGNvbnN0IHBrZyA9IGF3YWl0IFByb21pc2UucmVzb2x2ZSgpLnRoZW4oZnVuY3Rpb24gKCkgewogICAgICAgIHJldHVybiB0ZmhlOwogICAgICB9KTsKICAgICAgYXdhaXQgcGtnLmRlZmF1bHQoaW5pdCk7CiAgICAgIHRhcmdldC5wb3N0TWVzc2FnZSh7IHR5cGU6ICd3YXNtX2JpbmRnZW5fd29ya2VyX3JlYWR5JyB9KTsKICAgICAgcGtnLndiZ19yYXlvbl9zdGFydF93b3JrZXIocmVjZWl2ZXIpOwogICAgfSwKICApLAopOwoKLyoqCiAqIEBwYXJhbSB7bnVtYmVyfSByZWNlaXZlcgogKi8KZnVuY3Rpb24gd2JnX3JheW9uX3N0YXJ0X3dvcmtlcihyZWNlaXZlcikgewogIHdhc20ud2JnX3JheW9uX3N0YXJ0X3dvcmtlcihyZWNlaXZlcik7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIEludGVybmFsIHdhc21iaW5kZ2VuIHRvb2xzCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLwovLyBJbXBvcnRzOgovLyBfX3diZ19nZXRfaW1wb3J0cwovLwovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gX193YmdfZ2V0X2ltcG9ydHMobWVtb3J5KSB7CiAgICBjb25zdCBpbXBvcnQwID0gewogICAgICAgIF9fcHJvdG9fXzogbnVsbCwKICAgICAgICBfX3diZ19CaWdJbnRfNjViY2VhMjUxZTc4ODA4MzogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gQmlnSW50KGFyZzApOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfQmlnSW50X2NjYmNhNzkzZjQ1NmU1ODI6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgcmV0dXJuIGhhbmRsZUVycm9yKGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgICAgICBjb25zdCByZXQgPSBCaWdJbnQoYXJnMCk7CiAgICAgICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgICAgICB9LCBhcmd1bWVudHMpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfRXJyb3JfOTYwYzE1NWQzZDQ5ZTRjMjogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gRXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGFyZzAsIGFyZzEpKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fYmlnaW50X2dldF9hc19pNjRfM2QzYWJhNWQ2MTZjNmE1MTogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgdiA9IGFyZzE7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiAodikgPT09ICdiaWdpbnQnID8gdiA6IHVuZGVmaW5lZDsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0QmlnSW50NjQoYXJnMCArIDggKiAxLCBpc0xpa2VOb25lKHJldCkgPyBCaWdJbnQoMCkgOiByZXQsIHRydWUpOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDAsICFpc0xpa2VOb25lKHJldCksIHRydWUpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9iaXRfYW5kXzk2ZjY5NmM1NmI0OTUwZDU6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAgJiBhcmcxOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9iaXRfb3JfZDU2NGMyNzUxZmRhYzZjOTogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMCB8IGFyZzE7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2RlYnVnX3N0cmluZ19hYjRiMzRkMjNkNjc3OGJkOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBkZWJ1Z1N0cmluZyhhcmcxKTsKICAgICAgICAgICAgY29uc3QgcHRyMSA9IHBhc3NTdHJpbmdUb1dhc20wKHJldCwgd2FzbS5fX3diaW5kZ2VuX21hbGxvYywgd2FzbS5fX3diaW5kZ2VuX3JlYWxsb2MpOwogICAgICAgICAgICBjb25zdCBsZW4xID0gV0FTTV9WRUNUT1JfTEVOOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDEsIGxlbjEsIHRydWUpOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDAsIHB0cjEsIHRydWUpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9pc19mdW5jdGlvbl8zYmFhOWRiMWE5ODdmNDdkOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgKGFyZzApID09PSAnZnVuY3Rpb24nOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9pc19vYmplY3RfNjMzMjJlYzBjZDZlYTRlZjogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgdmFsID0gYXJnMDsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mICh2YWwpID09PSAnb2JqZWN0JyAmJiB2YWwgIT09IG51bGw7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2lzX3N0cmluZ182ZGYzYmY3ZWYxMTY0ZWQzOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgKGFyZzApID09PSAnc3RyaW5nJzsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5faXNfdW5kZWZpbmVkXzI5YTQzYjRkNDI5MjBhYmQ6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAgPT09IHVuZGVmaW5lZDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fanN2YWxfZXFfZDM0NjVkOGEwNzY5NzIyODogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMCA9PT0gYXJnMTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fbHRfNzhiYWIzODI2MjhmYjQ4ZjogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMCA8IGFyZzE7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX21lbW9yeV9kZmExMjA5NmY0MDBjOWJkOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHdhc20ubWVtb3J5OwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9tb2R1bGVfYjVlNmZiOTVkYmRiN2Q3ZTogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB3YXNtTW9kdWxlOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9uZWdfOGQzOWQyM2VmNjVjOWZkYjogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gLWFyZzA7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX3NobF9jN2VhNDE3MzM4N2ExNjY5OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwIDw8IGFyZzE7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX3Nocl80MzY1NTNjYmFlZjQxYTY2OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwID4+IGFyZzE7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX3N0cmluZ19nZXRfN2VkNTMyMjk5MWNhYWVjNTogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3Qgb2JqID0gYXJnMTsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mIChvYmopID09PSAnc3RyaW5nJyA/IG9iaiA6IHVuZGVmaW5lZDsKICAgICAgICAgICAgdmFyIHB0cjEgPSBpc0xpa2VOb25lKHJldCkgPyAwIDogcGFzc1N0cmluZ1RvV2FzbTAocmV0LCB3YXNtLl9fd2JpbmRnZW5fbWFsbG9jLCB3YXNtLl9fd2JpbmRnZW5fcmVhbGxvYyk7CiAgICAgICAgICAgIHZhciBsZW4xID0gV0FTTV9WRUNUT1JfTEVOOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDEsIGxlbjEsIHRydWUpOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDAsIHB0cjEsIHRydWUpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl90aHJvd182YjY0NDQ5YjliOWVkMzNjOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICB0aHJvdyBuZXcgRXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGFyZzAsIGFyZzEpKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX2NhbGxfYTI0NTkyYTZmMzQ5YTk3ZTogZnVuY3Rpb24gKCkgewogICAgICAgICAgICByZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24gKGFyZzAsIGFyZzEsIGFyZzIpIHsKICAgICAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAuY2FsbChhcmcxLCBhcmcyKTsKICAgICAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgICAgIH0sIGFyZ3VtZW50cyk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19jcnlwdG9fMzhkZjJiYWIxMjZiNjNkYzogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5jcnlwdG87CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19lcnJvcl9hNmZhMjAyYjU4YWExY2QzOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBsZXQgZGVmZXJyZWQwXzA7CiAgICAgICAgICAgIGxldCBkZWZlcnJlZDBfMTsKICAgICAgICAgICAgdHJ5IHsKICAgICAgICAgICAgICAgIGRlZmVycmVkMF8wID0gYXJnMDsKICAgICAgICAgICAgICAgIGRlZmVycmVkMF8xID0gYXJnMTsKICAgICAgICAgICAgICAgIGNvbnNvbGUuZXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGFyZzAsIGFyZzEpKTsKICAgICAgICAgICAgfQogICAgICAgICAgICBmaW5hbGx5IHsKICAgICAgICAgICAgICAgIHdhc20uX193YmluZGdlbl9mcmVlKGRlZmVycmVkMF8wLCBkZWZlcnJlZDBfMSwgMSk7CiAgICAgICAgICAgIH0KICAgICAgICB9LAogICAgICAgIF9fd2JnX2dldFJhbmRvbVZhbHVlc19jNDRhNTBkOGNmZGFlYmViOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIHJldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICAgICAgYXJnMC5nZXRSYW5kb21WYWx1ZXMoYXJnMSk7CiAgICAgICAgICAgIH0sIGFyZ3VtZW50cyk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19nZXRUaW1lX2RhN2M1NWY1MmI3MWU4YzY6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAuZ2V0VGltZSgpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfaW5zdGFuY2VvZl9XaW5kb3dfY2M2NGM4NmM4ZWY5ZTAyYjogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgbGV0IHJlc3VsdDsKICAgICAgICAgICAgdHJ5IHsKICAgICAgICAgICAgICAgIHJlc3VsdCA9IGFyZzAgaW5zdGFuY2VvZiBXaW5kb3c7CiAgICAgICAgICAgIH0KICAgICAgICAgICAgY2F0Y2ggKF8pIHsKICAgICAgICAgICAgICAgIHJlc3VsdCA9IGZhbHNlOwogICAgICAgICAgICB9CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHJlc3VsdDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX2xlbmd0aF85ZjE3NzUyMjRjZjFkODE1OiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLmxlbmd0aDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX21zQ3J5cHRvX2JkNWEwMzRhZjk2YmNiYTY6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAubXNDcnlwdG87CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19uZXdfMF80ZDY1NzIwMWNlZDE0ZGUzOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IG5ldyBEYXRlKCk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19uZXdfMjI3ZDdjMDU0MTRlYjg2MTogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBuZXcgRXJyb3IoKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX25ld193aXRoX2xlbmd0aF84Yzg1NGU0MWVhNGRhZTliOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBuZXcgVWludDhBcnJheShhcmcwID4+PiAwKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX25vZGVfODRlYTg3NTQxMTI1NGRiMTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5ub2RlOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfcHJvY2Vzc180NGM3YTE0ZTExZTlmNjllOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLnByb2Nlc3M7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19wcm90b3R5cGVzZXRjYWxsX2E2YjAyZWIwMGIwZjRjZTI6IGZ1bmN0aW9uIChhcmcwLCBhcmcxLCBhcmcyKSB7CiAgICAgICAgICAgIFVpbnQ4QXJyYXkucHJvdG90eXBlLnNldC5jYWxsKGdldEFycmF5VThGcm9tV2FzbTAoYXJnMCwgYXJnMSksIGFyZzIpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfcmFuZG9tRmlsbFN5bmNfNmMyNWVhYzk4NjllYjUzYzogZnVuY3Rpb24gKCkgewogICAgICAgICAgICByZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgICAgIGFyZzAucmFuZG9tRmlsbFN5bmMoYXJnMSk7CiAgICAgICAgICAgIH0sIGFyZ3VtZW50cyk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19yZXF1aXJlX2I0ZWRiZGNmM2UyYTFlZjA6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgcmV0dXJuIGhhbmRsZUVycm9yKGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgICAgIGNvbnN0IHJldCA9IG1vZHVsZS5yZXF1aXJlOwogICAgICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICAgICAgfSwgYXJndW1lbnRzKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3N0YWNrXzNiMGQ5NzRiYmYzMWU0NGY6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzEuc3RhY2s7CiAgICAgICAgICAgIGNvbnN0IHB0cjEgPSBwYXNzU3RyaW5nVG9XYXNtMChyZXQsIHdhc20uX193YmluZGdlbl9tYWxsb2MsIHdhc20uX193YmluZGdlbl9yZWFsbG9jKTsKICAgICAgICAgICAgY29uc3QgbGVuMSA9IFdBU01fVkVDVE9SX0xFTjsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAxLCBsZW4xLCB0cnVlKTsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAwLCBwdHIxLCB0cnVlKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3N0YXJ0V29ya2Vyc184YjU4MmQ1N2U5MmJkMmQ0OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSwgYXJnMikgewogICAgICAgICAgICBoYW5kbGVFcnJvcihmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgICAgICB0aHJvdyBuZXcgRXJyb3IoJ3N0YXJ0V29ya2VycyBub3Qgc3VwcG9ydGVkIGZyb20gYSB3b3JrZXIgdGhyZWFkJyk7CiAgICAgICAgICAgIH0pOwogICAgICAgICAgICAvLyBjb25zdCByZXQgPSBzdGFydFdvcmtlcnMoYXJnMCwgYXJnMSwgd2JnX3JheW9uX1Bvb2xCdWlsZGVyLl9fd3JhcChhcmcyKSk7CiAgICAgICAgICAgIC8vIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdGF0aWNfYWNjZXNzb3JfR0xPQkFMXzhjZmFkYzg3YTI5N2NhMDI6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mIGdsb2JhbCA9PT0gJ3VuZGVmaW5lZCcgPyBudWxsIDogZ2xvYmFsOwogICAgICAgICAgICByZXR1cm4gaXNMaWtlTm9uZShyZXQpID8gMCA6IGFkZFRvRXh0ZXJucmVmVGFibGUwKHJldCk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdGF0aWNfYWNjZXNzb3JfR0xPQkFMX1RISVNfNjAyMjU2YWU1YzhmNDJjZjogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgZ2xvYmFsVGhpcyA9PT0gJ3VuZGVmaW5lZCcgPyBudWxsIDogZ2xvYmFsVGhpczsKICAgICAgICAgICAgcmV0dXJuIGlzTGlrZU5vbmUocmV0KSA/IDAgOiBhZGRUb0V4dGVybnJlZlRhYmxlMChyZXQpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3RhdGljX2FjY2Vzc29yX1NFTEZfZTQ0NWMxYzc0ODRhZWNjMzogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2Ygc2VsZiA9PT0gJ3VuZGVmaW5lZCcgPyBudWxsIDogc2VsZjsKICAgICAgICAgICAgcmV0dXJuIGlzTGlrZU5vbmUocmV0KSA/IDAgOiBhZGRUb0V4dGVybnJlZlRhYmxlMChyZXQpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3RhdGljX2FjY2Vzc29yX1dJTkRPV19mMjBlODU3NmVmMWUwZjE3OiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiB3aW5kb3cgPT09ICd1bmRlZmluZWQnID8gbnVsbCA6IHdpbmRvdzsKICAgICAgICAgICAgcmV0dXJuIGlzTGlrZU5vbmUocmV0KSA/IDAgOiBhZGRUb0V4dGVybnJlZlRhYmxlMChyZXQpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3ViYXJyYXlfZjhjYTQ2YTI1YjFmNWUwZDogZnVuY3Rpb24gKGFyZzAsIGFyZzEsIGFyZzIpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5zdWJhcnJheShhcmcxID4+PiAwLCBhcmcyID4+PiAwKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3RvU3RyaW5nXzZkYzFhOTRlMGJkYmEzNzg6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAudG9TdHJpbmcoKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3RvU3RyaW5nX2M5NmRjNzZkNTU0N2E3MTU6IGZ1bmN0aW9uIChhcmcwLCBhcmcxLCBhcmcyKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzEudG9TdHJpbmcoYXJnMik7CiAgICAgICAgICAgIGNvbnN0IHB0cjEgPSBwYXNzU3RyaW5nVG9XYXNtMChyZXQsIHdhc20uX193YmluZGdlbl9tYWxsb2MsIHdhc20uX193YmluZGdlbl9yZWFsbG9jKTsKICAgICAgICAgICAgY29uc3QgbGVuMSA9IFdBU01fVkVDVE9SX0xFTjsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAxLCBsZW4xLCB0cnVlKTsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAwLCBwdHIxLCB0cnVlKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3ZlcnNpb25zXzI3NmIyNzk1YjFjNmEyMTk6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAudmVyc2lvbnM7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diaW5kZ2VuX2Nhc3RfMDAwMDAwMDAwMDAwMDAwMTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgLy8gQ2FzdCBpbnRyaW5zaWMgZm9yIGBGNjQgLT4gRXh0ZXJucmVmYC4KICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDAyOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICAvLyBDYXN0IGludHJpbnNpYyBmb3IgYEkxMjggLT4gRXh0ZXJucmVmYC4KICAgICAgICAgICAgY29uc3QgcmV0ID0gKEJpZ0ludC5hc1VpbnROKDY0LCBhcmcwKSB8IChhcmcxIDw8IEJpZ0ludCg2NCkpKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDAzOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICAvLyBDYXN0IGludHJpbnNpYyBmb3IgYEk2NCAtPiBFeHRlcm5yZWZgLgogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDQ6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIC8vIENhc3QgaW50cmluc2ljIGZvciBgUmVmKFNsaWNlKFU4KSkgLT4gTmFtZWRFeHRlcm5yZWYoIlVpbnQ4QXJyYXkiKWAuCiAgICAgICAgICAgIGNvbnN0IHJldCA9IGdldEFycmF5VThGcm9tV2FzbTAoYXJnMCwgYXJnMSk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diaW5kZ2VuX2Nhc3RfMDAwMDAwMDAwMDAwMDAwNTogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgLy8gQ2FzdCBpbnRyaW5zaWMgZm9yIGBSZWYoU3RyaW5nKSAtPiBFeHRlcm5yZWZgLgogICAgICAgICAgICBjb25zdCByZXQgPSBnZXRTdHJpbmdGcm9tV2FzbTAoYXJnMCwgYXJnMSk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diaW5kZ2VuX2Nhc3RfMDAwMDAwMDAwMDAwMDAwNjogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgLy8gQ2FzdCBpbnRyaW5zaWMgZm9yIGBVMTI4IC0+IEV4dGVybnJlZmAuCiAgICAgICAgICAgIGNvbnN0IHJldCA9IChCaWdJbnQuYXNVaW50Tig2NCwgYXJnMCkgfCAoQmlnSW50LmFzVWludE4oNjQsIGFyZzEpIDw8IEJpZ0ludCg2NCkpKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDA3OiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICAvLyBDYXN0IGludHJpbnNpYyBmb3IgYFU2NCAtPiBFeHRlcm5yZWZgLgogICAgICAgICAgICBjb25zdCByZXQgPSBCaWdJbnQuYXNVaW50Tig2NCwgYXJnMCk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diaW5kZ2VuX2luaXRfZXh0ZXJucmVmX3RhYmxlOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHRhYmxlID0gd2FzbS5fX3diaW5kZ2VuX2V4dGVybnJlZnM7CiAgICAgICAgICAgIGNvbnN0IG9mZnNldCA9IHRhYmxlLmdyb3coNCk7CiAgICAgICAgICAgIHRhYmxlLnNldCgwLCB1bmRlZmluZWQpOwogICAgICAgICAgICB0YWJsZS5zZXQob2Zmc2V0ICsgMCwgdW5kZWZpbmVkKTsKICAgICAgICAgICAgdGFibGUuc2V0KG9mZnNldCArIDEsIG51bGwpOwogICAgICAgICAgICB0YWJsZS5zZXQob2Zmc2V0ICsgMiwgdHJ1ZSk7CiAgICAgICAgICAgIHRhYmxlLnNldChvZmZzZXQgKyAzLCBmYWxzZSk7CiAgICAgICAgfSwKICAgICAgICBtZW1vcnk6IG1lbW9yeSB8fCBuZXcgV2ViQXNzZW1ibHkuTWVtb3J5KHsgaW5pdGlhbDogMjEsIG1heGltdW06IDE2Mzg0LCBzaGFyZWQ6IHRydWUgfSksCiAgICB9OwogICAgcmV0dXJuIHsKICAgICAgICBfX3Byb3RvX186IG51bGwsCiAgICAgICAgIi4vdGZoZV9iZy5qcyI6IGltcG9ydDAsCiAgICB9Owp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBhZGRUb0V4dGVybnJlZlRhYmxlMAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gYWRkVG9FeHRlcm5yZWZUYWJsZTAob2JqKSB7CiAgICBjb25zdCBpZHggPSB3YXNtLl9fZXh0ZXJucmVmX3RhYmxlX2FsbG9jKCk7CiAgICB3YXNtLl9fd2JpbmRnZW5fZXh0ZXJucmVmcy5zZXQoaWR4LCBvYmopOwogICAgcmV0dXJuIGlkeDsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gZGVidWdTdHJpbmcKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGRlYnVnU3RyaW5nKHZhbCkgewogICAgLy8gcHJpbWl0aXZlIHR5cGVzCiAgICBjb25zdCB0eXBlID0gdHlwZW9mIHZhbDsKICAgIGlmICh0eXBlID09ICdudW1iZXInIHx8IHR5cGUgPT0gJ2Jvb2xlYW4nIHx8IHZhbCA9PSBudWxsKSB7CiAgICAgICAgcmV0dXJuIGAke3ZhbH1gOwogICAgfQogICAgaWYgKHR5cGUgPT0gJ3N0cmluZycpIHsKICAgICAgICByZXR1cm4gYCIke3ZhbH0iYDsKICAgIH0KICAgIGlmICh0eXBlID09ICdzeW1ib2wnKSB7CiAgICAgICAgY29uc3QgZGVzY3JpcHRpb24gPSB2YWwuZGVzY3JpcHRpb247CiAgICAgICAgaWYgKGRlc2NyaXB0aW9uID09IG51bGwpIHsKICAgICAgICAgICAgcmV0dXJuICdTeW1ib2wnOwogICAgICAgIH0KICAgICAgICBlbHNlIHsKICAgICAgICAgICAgcmV0dXJuIGBTeW1ib2woJHtkZXNjcmlwdGlvbn0pYDsKICAgICAgICB9CiAgICB9CiAgICBpZiAodHlwZSA9PSAnZnVuY3Rpb24nKSB7CiAgICAgICAgY29uc3QgbmFtZSA9IHZhbC5uYW1lOwogICAgICAgIGlmICh0eXBlb2YgbmFtZSA9PSAnc3RyaW5nJyAmJiBuYW1lLmxlbmd0aCA+IDApIHsKICAgICAgICAgICAgcmV0dXJuIGBGdW5jdGlvbigke25hbWV9KWA7CiAgICAgICAgfQogICAgICAgIGVsc2UgewogICAgICAgICAgICByZXR1cm4gJ0Z1bmN0aW9uJzsKICAgICAgICB9CiAgICB9CiAgICAvLyBvYmplY3RzCiAgICBpZiAoQXJyYXkuaXNBcnJheSh2YWwpKSB7CiAgICAgICAgY29uc3QgbGVuZ3RoID0gdmFsLmxlbmd0aDsKICAgICAgICBsZXQgZGVidWcgPSAnWyc7CiAgICAgICAgaWYgKGxlbmd0aCA+IDApIHsKICAgICAgICAgICAgZGVidWcgKz0gZGVidWdTdHJpbmcodmFsWzBdKTsKICAgICAgICB9CiAgICAgICAgZm9yIChsZXQgaSA9IDE7IGkgPCBsZW5ndGg7IGkrKykgewogICAgICAgICAgICBkZWJ1ZyArPSAnLCAnICsgZGVidWdTdHJpbmcodmFsW2ldKTsKICAgICAgICB9CiAgICAgICAgZGVidWcgKz0gJ10nOwogICAgICAgIHJldHVybiBkZWJ1ZzsKICAgIH0KICAgIC8vIFRlc3QgZm9yIGJ1aWx0LWluCiAgICBjb25zdCBidWlsdEluTWF0Y2hlcyA9IC9cW29iamVjdCAoW15cXV0rKVxdLy5leGVjKHRvU3RyaW5nLmNhbGwodmFsKSk7CiAgICBsZXQgY2xhc3NOYW1lOwogICAgaWYgKGJ1aWx0SW5NYXRjaGVzICYmIGJ1aWx0SW5NYXRjaGVzLmxlbmd0aCA+IDEpIHsKICAgICAgICBjbGFzc05hbWUgPSBidWlsdEluTWF0Y2hlc1sxXTsKICAgIH0KICAgIGVsc2UgewogICAgICAgIC8vIEZhaWxlZCB0byBtYXRjaCB0aGUgc3RhbmRhcmQgJ1tvYmplY3QgQ2xhc3NOYW1lXScKICAgICAgICByZXR1cm4gdG9TdHJpbmcuY2FsbCh2YWwpOwogICAgfQogICAgaWYgKGNsYXNzTmFtZSA9PSAnT2JqZWN0JykgewogICAgICAgIC8vIHdlJ3JlIGEgdXNlciBkZWZpbmVkIGNsYXNzIG9yIE9iamVjdAogICAgICAgIC8vIEpTT04uc3RyaW5naWZ5IGF2b2lkcyBwcm9ibGVtcyB3aXRoIGN5Y2xlcywgYW5kIGlzIGdlbmVyYWxseSBtdWNoCiAgICAgICAgLy8gZWFzaWVyIHRoYW4gbG9vcGluZyB0aHJvdWdoIG93blByb3BlcnRpZXMgb2YgYHZhbGAuCiAgICAgICAgdHJ5IHsKICAgICAgICAgICAgcmV0dXJuICdPYmplY3QoJyArIEpTT04uc3RyaW5naWZ5KHZhbCkgKyAnKSc7CiAgICAgICAgfQogICAgICAgIGNhdGNoIChfKSB7CiAgICAgICAgICAgIHJldHVybiAnT2JqZWN0JzsKICAgICAgICB9CiAgICB9CiAgICAvLyBlcnJvcnMKICAgIGlmICh2YWwgaW5zdGFuY2VvZiBFcnJvcikgewogICAgICAgIHJldHVybiBgJHt2YWwubmFtZX06ICR7dmFsLm1lc3NhZ2V9XG4ke3ZhbC5zdGFja31gOwogICAgfQogICAgLy8gVE9ETyB3ZSBjb3VsZCB0ZXN0IGZvciBtb3JlIHRoaW5ncyBoZXJlLCBsaWtlIGBTZXRgcyBhbmQgYE1hcGBzLgogICAgcmV0dXJuIGNsYXNzTmFtZTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gZ2V0QXJyYXlVOEZyb21XYXNtMAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gZ2V0QXJyYXlVOEZyb21XYXNtMChwdHIsIGxlbikgewogICAgcHRyID0gcHRyID4+PiAwOwogICAgcmV0dXJuIGdldFVpbnQ4QXJyYXlNZW1vcnkwKCkuc3ViYXJyYXkocHRyIC8gMSwgcHRyIC8gMSArIGxlbik7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGNhY2hlZERhdGFWaWV3TWVtb3J5MAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IGNhY2hlZERhdGFWaWV3TWVtb3J5MCA9IG51bGw7CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBnZXREYXRhVmlld01lbW9yeTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGdldERhdGFWaWV3TWVtb3J5MCgpIHsKICAgIGlmIChjYWNoZWREYXRhVmlld01lbW9yeTAgPT09IG51bGwgfHwgY2FjaGVkRGF0YVZpZXdNZW1vcnkwLmJ1ZmZlciAhPT0gd2FzbS5tZW1vcnkuYnVmZmVyKSB7CiAgICAgICAgY2FjaGVkRGF0YVZpZXdNZW1vcnkwID0gbmV3IERhdGFWaWV3KHdhc20ubWVtb3J5LmJ1ZmZlcik7CiAgICB9CiAgICByZXR1cm4gY2FjaGVkRGF0YVZpZXdNZW1vcnkwOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBnZXRTdHJpbmdGcm9tV2FzbTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGdldFN0cmluZ0Zyb21XYXNtMChwdHIsIGxlbikgewogICAgcHRyID0gcHRyID4+PiAwOwogICAgcmV0dXJuIGRlY29kZVRleHQocHRyLCBsZW4pOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBjYWNoZWRVaW50OEFycmF5TWVtb3J5MAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwID0gbnVsbDsKCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGdldFVpbnQ4QXJyYXlNZW1vcnkwCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBnZXRVaW50OEFycmF5TWVtb3J5MCgpIHsKICAgIGlmIChjYWNoZWRVaW50OEFycmF5TWVtb3J5MCA9PT0gbnVsbCB8fCBjYWNoZWRVaW50OEFycmF5TWVtb3J5MC5idWZmZXIgIT09IHdhc20ubWVtb3J5LmJ1ZmZlcikgewogICAgICAgIGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwID0gbmV3IFVpbnQ4QXJyYXkod2FzbS5tZW1vcnkuYnVmZmVyKTsKICAgIH0KICAgIHJldHVybiBjYWNoZWRVaW50OEFycmF5TWVtb3J5MDsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gaGFuZGxlRXJyb3IKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGhhbmRsZUVycm9yKGYsIGFyZ3MpIHsKICAgIHRyeSB7CiAgICAgICAgcmV0dXJuIGYuYXBwbHkodGhpcywgYXJncyk7CiAgICB9CiAgICBjYXRjaCAoZSkgewogICAgICAgIGNvbnN0IGlkeCA9IGFkZFRvRXh0ZXJucmVmVGFibGUwKGUpOwogICAgICAgIHdhc20uX193YmluZGdlbl9leG5fc3RvcmUoaWR4KTsKICAgIH0KfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gaXNMaWtlTm9uZQovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gaXNMaWtlTm9uZSh4KSB7CiAgICByZXR1cm4geCA9PT0gdW5kZWZpbmVkIHx8IHggPT09IG51bGw7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIHBhc3NTdHJpbmdUb1dhc20wCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBwYXNzU3RyaW5nVG9XYXNtMChhcmcsIG1hbGxvYywgcmVhbGxvYykgewogICAgaWYgKHJlYWxsb2MgPT09IHVuZGVmaW5lZCkgewogICAgICAgIGNvbnN0IGJ1ZiA9IGNhY2hlZFRleHRFbmNvZGVyLmVuY29kZShhcmcpOwogICAgICAgIGNvbnN0IHB0ciA9IG1hbGxvYyhidWYubGVuZ3RoLCAxKSA+Pj4gMDsKICAgICAgICBnZXRVaW50OEFycmF5TWVtb3J5MCgpLnN1YmFycmF5KHB0ciwgcHRyICsgYnVmLmxlbmd0aCkuc2V0KGJ1Zik7CiAgICAgICAgV0FTTV9WRUNUT1JfTEVOID0gYnVmLmxlbmd0aDsKICAgICAgICByZXR1cm4gcHRyOwogICAgfQogICAgbGV0IGxlbiA9IGFyZy5sZW5ndGg7CiAgICBsZXQgcHRyID0gbWFsbG9jKGxlbiwgMSkgPj4+IDA7CiAgICBjb25zdCBtZW0gPSBnZXRVaW50OEFycmF5TWVtb3J5MCgpOwogICAgbGV0IG9mZnNldCA9IDA7CiAgICBmb3IgKDsgb2Zmc2V0IDwgbGVuOyBvZmZzZXQrKykgewogICAgICAgIGNvbnN0IGNvZGUgPSBhcmcuY2hhckNvZGVBdChvZmZzZXQpOwogICAgICAgIGlmIChjb2RlID4gMHg3RikKICAgICAgICAgICAgYnJlYWs7CiAgICAgICAgbWVtW3B0ciArIG9mZnNldF0gPSBjb2RlOwogICAgfQogICAgaWYgKG9mZnNldCAhPT0gbGVuKSB7CiAgICAgICAgaWYgKG9mZnNldCAhPT0gMCkgewogICAgICAgICAgICBhcmcgPSBhcmcuc2xpY2Uob2Zmc2V0KTsKICAgICAgICB9CiAgICAgICAgcHRyID0gcmVhbGxvYyhwdHIsIGxlbiwgbGVuID0gb2Zmc2V0ICsgYXJnLmxlbmd0aCAqIDMsIDEpID4+PiAwOwogICAgICAgIGNvbnN0IHZpZXcgPSBnZXRVaW50OEFycmF5TWVtb3J5MCgpLnN1YmFycmF5KHB0ciArIG9mZnNldCwgcHRyICsgbGVuKTsKICAgICAgICBjb25zdCByZXQgPSBjYWNoZWRUZXh0RW5jb2Rlci5lbmNvZGVJbnRvKGFyZywgdmlldyk7CiAgICAgICAgb2Zmc2V0ICs9IHJldC53cml0dGVuOwogICAgICAgIHB0ciA9IHJlYWxsb2MocHRyLCBsZW4sIG9mZnNldCwgMSkgPj4+IDA7CiAgICB9CiAgICBXQVNNX1ZFQ1RPUl9MRU4gPSBvZmZzZXQ7CiAgICByZXR1cm4gcHRyOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBjYWNoZWRUZXh0RGVjb2RlcgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IGNhY2hlZFRleHREZWNvZGVyID0gKHR5cGVvZiBUZXh0RGVjb2RlciAhPT0gJ3VuZGVmaW5lZCcgPyBuZXcgVGV4dERlY29kZXIoJ3V0Zi04JywgeyBpZ25vcmVCT006IHRydWUsIGZhdGFsOiB0cnVlIH0pIDogdW5kZWZpbmVkKTsKCmlmIChjYWNoZWRUZXh0RGVjb2RlcikKICAgIGNhY2hlZFRleHREZWNvZGVyLmRlY29kZSgpOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gTUFYX1NBRkFSSV9ERUNPREVfQllURVMKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmNvbnN0IE1BWF9TQUZBUklfREVDT0RFX0JZVEVTID0gMjE0NjQzNTA3MjsKCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIG51bUJ5dGVzRGVjb2RlZAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IG51bUJ5dGVzRGVjb2RlZCA9IDA7CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBkZWNvZGVUZXh0Ci8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBkZWNvZGVUZXh0KHB0ciwgbGVuKSB7CiAgICBudW1CeXRlc0RlY29kZWQgKz0gbGVuOwogICAgaWYgKG51bUJ5dGVzRGVjb2RlZCA+PSBNQVhfU0FGQVJJX0RFQ09ERV9CWVRFUykgewogICAgICAgIGNhY2hlZFRleHREZWNvZGVyID0gbmV3IFRleHREZWNvZGVyKCd1dGYtOCcsIHsgaWdub3JlQk9NOiB0cnVlLCBmYXRhbDogdHJ1ZSB9KTsKICAgICAgICBjYWNoZWRUZXh0RGVjb2Rlci5kZWNvZGUoKTsKICAgICAgICBudW1CeXRlc0RlY29kZWQgPSBsZW47CiAgICB9CiAgICByZXR1cm4gY2FjaGVkVGV4dERlY29kZXIuZGVjb2RlKGdldFVpbnQ4QXJyYXlNZW1vcnkwKCkuc2xpY2UocHRyLCBwdHIgKyBsZW4pKTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gY2FjaGVkVGV4dEVuY29kZXIKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmNvbnN0IGNhY2hlZFRleHRFbmNvZGVyID0gKHR5cGVvZiBUZXh0RW5jb2RlciAhPT0gJ3VuZGVmaW5lZCcgPyBuZXcgVGV4dEVuY29kZXIoKSA6IHVuZGVmaW5lZCk7CgppZiAoY2FjaGVkVGV4dEVuY29kZXIpIHsKICAgIGNhY2hlZFRleHRFbmNvZGVyLmVuY29kZUludG8gPSBmdW5jdGlvbiAoYXJnLCB2aWV3KSB7CiAgICAgICAgY29uc3QgYnVmID0gY2FjaGVkVGV4dEVuY29kZXIuZW5jb2RlKGFyZyk7CiAgICAgICAgdmlldy5zZXQoYnVmKTsKICAgICAgICByZXR1cm4gewogICAgICAgICAgICByZWFkOiBhcmcubGVuZ3RoLAogICAgICAgICAgICB3cml0dGVuOiBidWYubGVuZ3RoCiAgICAgICAgfTsKICAgIH07Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIFdBU01fVkVDVE9SX0xFTiBpcyBhIG1vZHVsZS1sZXZlbCB2YXJpYWJsZSB0aGF0IHN0b3JlcyB0aGUgYnl0ZSBsZW5ndGggb2YKLy8gdGhlIGRhdGEganVzdCB3cml0dGVuIGludG8gV0FTTSBtZW1vcnkuIEl0IGFjdHMgYXMgYW4gb3V0LXBhcmFtZXRlci4KLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmxldCBXQVNNX1ZFQ1RPUl9MRU4gPSAwOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gV0FTTSBtb2R1bGUgc3RhdGUKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmxldCB3YXNtTW9kdWxlLCB3YXNtOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gSW5pdDoKLy8gX193YmdfZmluYWxpemVfaW5pdAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gX193YmdfZmluYWxpemVfaW5pdChpbnN0YW5jZSwgbW9kdWxlLCB0aHJlYWRfc3RhY2tfc2l6ZSkgewogICAgd2FzbSA9IGluc3RhbmNlLmV4cG9ydHM7CiAgICB3YXNtTW9kdWxlID0gbW9kdWxlOwogICAgY2FjaGVkRGF0YVZpZXdNZW1vcnkwID0gbnVsbDsKICAgIGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwID0gbnVsbDsKICAgIGlmICh0eXBlb2YgdGhyZWFkX3N0YWNrX3NpemUgIT09ICd1bmRlZmluZWQnICYmICh0eXBlb2YgdGhyZWFkX3N0YWNrX3NpemUgIT09ICdudW1iZXInIHx8IHRocmVhZF9zdGFja19zaXplID09PSAwIHx8IHRocmVhZF9zdGFja19zaXplICUgNjU1MzYgIT09IDApKSB7CiAgICAgICAgdGhyb3cgbmV3IEVycm9yKCdpbnZhbGlkIHN0YWNrIHNpemUnKTsKICAgIH0KICAgIHdhc20uX193YmluZGdlbl9zdGFydCh0aHJlYWRfc3RhY2tfc2l6ZSk7CiAgICByZXR1cm4gd2FzbTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gSW5pdDoKLy8gX193YmdfbG9hZAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKYXN5bmMgZnVuY3Rpb24gX193YmdfbG9hZChtb2R1bGUsIGltcG9ydHMpIHsKICAgIGlmICh0eXBlb2YgUmVzcG9uc2UgPT09ICdmdW5jdGlvbicgJiYgbW9kdWxlIGluc3RhbmNlb2YgUmVzcG9uc2UpIHsKICAgICAgICBpZiAodHlwZW9mIFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlU3RyZWFtaW5nID09PSAnZnVuY3Rpb24nKSB7CiAgICAgICAgICAgIHRyeSB7CiAgICAgICAgICAgICAgICByZXR1cm4gYXdhaXQgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGVTdHJlYW1pbmcobW9kdWxlLCBpbXBvcnRzKTsKICAgICAgICAgICAgfQogICAgICAgICAgICBjYXRjaCAoZSkgewogICAgICAgICAgICAgICAgY29uc3QgdmFsaWRSZXNwb25zZSA9IG1vZHVsZS5vayAmJiBleHBlY3RlZFJlc3BvbnNlVHlwZShtb2R1bGUudHlwZSk7CiAgICAgICAgICAgICAgICBpZiAodmFsaWRSZXNwb25zZSAmJiBtb2R1bGUuaGVhZGVycy5nZXQoJ0NvbnRlbnQtVHlwZScpICE9PSAnYXBwbGljYXRpb24vd2FzbScpIHsKICAgICAgICAgICAgICAgICAgICBjb25zb2xlLndhcm4oImBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZVN0cmVhbWluZ2AgZmFpbGVkIGJlY2F1c2UgeW91ciBzZXJ2ZXIgZG9lcyBub3Qgc2VydmUgV2FzbSB3aXRoIGBhcHBsaWNhdGlvbi93YXNtYCBNSU1FIHR5cGUuIEZhbGxpbmcgYmFjayB0byBgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGVgIHdoaWNoIGlzIHNsb3dlci4gT3JpZ2luYWwgZXJyb3I6XG4iLCBlKTsKICAgICAgICAgICAgICAgIH0KICAgICAgICAgICAgICAgIGVsc2UgewogICAgICAgICAgICAgICAgICAgIHRocm93IGU7CiAgICAgICAgICAgICAgICB9CiAgICAgICAgICAgIH0KICAgICAgICB9CiAgICAgICAgY29uc3QgYnl0ZXMgPSBhd2FpdCBtb2R1bGUuYXJyYXlCdWZmZXIoKTsKICAgICAgICByZXR1cm4gYXdhaXQgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGUoYnl0ZXMsIGltcG9ydHMpOwogICAgfQogICAgZWxzZSB7CiAgICAgICAgY29uc3QgaW5zdGFuY2UgPSBhd2FpdCBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZShtb2R1bGUsIGltcG9ydHMpOwogICAgICAgIGlmIChpbnN0YW5jZSBpbnN0YW5jZW9mIFdlYkFzc2VtYmx5Lkluc3RhbmNlKSB7CiAgICAgICAgICAgIHJldHVybiB7IGluc3RhbmNlLCBtb2R1bGUgfTsKICAgICAgICB9CiAgICAgICAgZWxzZSB7CiAgICAgICAgICAgIHJldHVybiBpbnN0YW5jZTsKICAgICAgICB9CiAgICB9CiAgICBmdW5jdGlvbiBleHBlY3RlZFJlc3BvbnNlVHlwZSh0eXBlKSB7CiAgICAgICAgc3dpdGNoICh0eXBlKSB7CiAgICAgICAgICAgIGNhc2UgJ2Jhc2ljJzoKICAgICAgICAgICAgY2FzZSAnY29ycyc6CiAgICAgICAgICAgIGNhc2UgJ2RlZmF1bHQnOiByZXR1cm4gdHJ1ZTsKICAgICAgICB9CiAgICAgICAgcmV0dXJuIGZhbHNlOwogICAgfQp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBJbml0OgovLyBfX3diZ19pbml0Ci8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgphc3luYyBmdW5jdGlvbiBfX3diZ19pbml0KG1vZHVsZV9vcl9wYXRoLCBtZW1vcnkpIHsKICAgIGlmICh3YXNtICE9PSB1bmRlZmluZWQpCiAgICAgICAgcmV0dXJuIHdhc207CiAgICBsZXQgdGhyZWFkX3N0YWNrX3NpemU7CiAgICBpZiAobW9kdWxlX29yX3BhdGggIT09IHVuZGVmaW5lZCkgewogICAgICAgIGlmIChPYmplY3QuZ2V0UHJvdG90eXBlT2YobW9kdWxlX29yX3BhdGgpID09PSBPYmplY3QucHJvdG90eXBlKSB7CiAgICAgICAgICAgICh7IG1vZHVsZV9vcl9wYXRoLCBtZW1vcnksIHRocmVhZF9zdGFja19zaXplIH0gPSBtb2R1bGVfb3JfcGF0aCk7CiAgICAgICAgfQogICAgICAgIGVsc2UgewogICAgICAgICAgICBjb25zb2xlLndhcm4oJ3VzaW5nIGRlcHJlY2F0ZWQgcGFyYW1ldGVycyBmb3IgdGhlIGluaXRpYWxpemF0aW9uIGZ1bmN0aW9uOyBwYXNzIGEgc2luZ2xlIG9iamVjdCBpbnN0ZWFkJyk7CiAgICAgICAgfQogICAgfQogICAgLy8gICBpZiAobW9kdWxlX29yX3BhdGggPT09IHVuZGVmaW5lZCkgewogICAgLy8gICAgIG1vZHVsZV9vcl9wYXRoID0gbmV3IFVSTCgndGZoZV9iZy53YXNtJywgaW1wb3J0Lm1ldGEudXJsKTsKICAgIC8vICAgfQogICAgY29uc3QgaW1wb3J0cyA9IF9fd2JnX2dldF9pbXBvcnRzKG1lbW9yeSk7CiAgICAvLyAgIGlmICgKICAgIC8vICAgICB0eXBlb2YgbW9kdWxlX29yX3BhdGggPT09ICdzdHJpbmcnIHx8CiAgICAvLyAgICAgKHR5cGVvZiBSZXF1ZXN0ID09PSAnZnVuY3Rpb24nICYmIG1vZHVsZV9vcl9wYXRoIGluc3RhbmNlb2YgUmVxdWVzdCkgfHwKICAgIC8vICAgICAodHlwZW9mIFVSTCA9PT0gJ2Z1bmN0aW9uJyAmJiBtb2R1bGVfb3JfcGF0aCBpbnN0YW5jZW9mIFVSTCkKICAgIC8vICAgKSB7CiAgICAvLyAgICAgbW9kdWxlX29yX3BhdGggPSBmZXRjaChtb2R1bGVfb3JfcGF0aCk7CiAgICAvLyAgIH0KICAgIGNvbnN0IHsgaW5zdGFuY2UsIG1vZHVsZSB9ID0gYXdhaXQgX193YmdfbG9hZChhd2FpdCBtb2R1bGVfb3JfcGF0aCwgaW1wb3J0cyk7CiAgICByZXR1cm4gX193YmdfZmluYWxpemVfaW5pdChpbnN0YW5jZSwgbW9kdWxlLCB0aHJlYWRfc3RhY2tfc2l6ZSk7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vCi8vIFRoZSAndGZoZScgZ2xvYmFsIG9iamVjdAovLyA9PT09PT09PT09PT09PT09PT09PT09PT0KLy8gRmluYWwgdGZoZSBvYmplY3QgZ2xvYmFsIGRlY2xhcmF0aW9uIGNhbGxlZCBieSAnd2FpdEZvck1zZ1R5cGUnIG9ubHkKLy8KLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCnZhciB0ZmhlID0gLyojX19QVVJFX18qLyBPYmplY3QuZnJlZXplKHsKICBfX3Byb3RvX186IG51bGwsCiAgZGVmYXVsdDogX193YmdfaW5pdCwKICB3YmdfcmF5b25fc3RhcnRfd29ya2VyOiB3YmdfcmF5b25fc3RhcnRfd29ya2VyLAp9KTsKCg==";
  return await __newWorkerFromJsCodeBase64(workerBase64);
}

////////////////////////////////////////////////////////////////////////////////
// Worker initialization helpers
////////////////////////////////////////////////////////////////////////////////

async function __createAndInitWorker(createWorker, workerInit, workerIndex, label) {
  let blobUrl = undefined;
  let worker = undefined;

  try {
    const result = await createWorker();
    worker = result.worker;
    blobUrl = result.blobUrl;

    _logger?.debug(`[Worker #${workerIndex}] - created with ${label}`);
    worker.postMessage(workerInit);
    await __waitForMsgType(worker, 'wasm_bindgen_worker_ready');

    if (blobUrl) {
      URL.revokeObjectURL(blobUrl);
      blobUrl = undefined;
    }

    _logger?.debug(`[Worker #${workerIndex}] - ready`);
    return worker;
  } catch (e) {
    if (blobUrl) {
      URL.revokeObjectURL(blobUrl);
    }

    if (worker) {
      try {
        await worker.terminate();
      } catch {
        // Preserve the worker creation/init error that triggered cleanup.
      }
    }

    throw e;
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Mode: `embedded-base64`
 */
async function __createAndInitWorkerFromEmbeddedBase64(workerInit, workerIndex) {
  return await __createAndInitWorker(
    () => __createWorkerFromBase64(),
    workerInit,
    workerIndex,
    'embedded base64 worker',
  );
}

/**
 * Mode: `verified-blob`
 */
async function __createAndInitWorkerFromVerifiedBlob(workerInit, workerIndex) {
  return await __createAndInitWorker(
    () => __createWorkerFromVerifiedWorkerUrl(),
    workerInit,
    workerIndex,
    'verified worker URL',
  );
}

/**
 * Mode: `trusted-direct-url`
 */
async function __createAndInitWorkerFromTrustedDirectUrl(workerInit, workerIndex) {
  return await __createAndInitWorker(
    () => __createWorkerFromTrustedDirectWorkerUrl(),
    workerInit,
    workerIndex,
    'trusted direct worker URL',
  );
}

/**
 * Mode: `precheck-direct-url`
 */
async function __createAndInitWorkerFromCheckedDirectUrl(workerInit, workerIndex) {
  return await __createAndInitWorker(
    () => __createWorkerFromCheckedDirectWorkerUrl(),
    workerInit,
    workerIndex,
    'checked direct worker URL',
  );
}

/**
 * Mode: `auto`
 */
async function __createAndInitWorkerAuto(workerInit, workerIndex) {
  if (_workerUrl) {
    try {
      /**
       * Mode: `verified-blob`
       */
      return await __createAndInitWorkerFromVerifiedBlob(workerInit, workerIndex);
    } catch (e) {
      if (__isSha256MismatchError(e)) {
        throw e;
      }

      _logger?.error(`[Worker #${workerIndex}] - verified worker URL failed; falling back to embedded base64`, e);
    }
  }

  try {
    /**
     * Mode: `embedded-base64`
     */
    return await __createAndInitWorkerFromEmbeddedBase64(workerInit, workerIndex);
  } catch (e) {
    throw new Error('All worker creation methods failed. Check CSP, COOP/COEP headers, and cross-origin policies.', {
      cause: e,
    });
  }
}

////////////////////////////////////////////////////////////////////////////////
// Worker load mode dispatcher
////////////////////////////////////////////////////////////////////////////////

async function __createAndInitConfiguredWorker(workerInit, workerIndex) {
  switch (_wasmAssetLoadMode) {
    case 'embedded-base64':
      return await __createAndInitWorkerFromEmbeddedBase64(workerInit, workerIndex);

    case 'verified-blob':
      return await __createAndInitWorkerFromVerifiedBlob(workerInit, workerIndex);

    case 'precheck-direct-url':
      return await __createAndInitWorkerFromCheckedDirectUrl(workerInit, workerIndex);

    case 'trusted-direct-url':
      return await __createAndInitWorkerFromTrustedDirectUrl(workerInit, workerIndex);

    case 'auto':
      return await __createAndInitWorkerAuto(workerInit, workerIndex);

    default:
      throw new Error(`Unsupported wasmAssetLoadMode: ${_wasmAssetLoadMode}`);
  }
}

////////////////////////////////////////////////////////////////////////////////
// Worker pool lifecycle
////////////////////////////////////////////////////////////////////////////////

async function startWorkers(module, memory, builder) {
  if (_started) {
    throw new Error('Already started');
  }

  _started = true;
  _starting = true;

  try {
    if (_terminating) {
      throw new Error('Cannot start workers while termination is in progress');
    }

    if (builder.numThreads() === 0) {
      throw new Error(`num_threads must be > 0.`);
    }

    const workerInit = {
      type: 'wasm_bindgen_worker_init',
      init: { module_or_path: module, memory },
      receiver: builder.receiver(),
    };
    const results = await Promise.allSettled(
      Array.from({ length: builder.numThreads() }, async (_, workerIndex) => {
        return await __createAndInitConfiguredWorker(workerInit, workerIndex);
      }),
    );

    const workers = [];
    const errors = [];

    for (const result of results) {
      if (result.status === 'fulfilled') {
        workers.push(result.value);
      } else {
        errors.push(result.reason);
      }
    }

    if (errors.length > 0) {
      await Promise.allSettled(workers.map((w) => w.terminate()));
      throw errors[0];
    }

    _workers = workers;
    builder.build();
  } finally {
    _starting = false;
    // Drop the verified-bytes cache: each spawned worker now holds its own copy
    // of the script (via Blob URL or eval source), so the SDK no longer needs
    // to keep the bytes around. On failure the module is one-shot anyway, so
    // the cache would never be reused.
    _verifiedWorkerUrlBytesPromise = undefined;
  }
}

async function terminateWorkers() {
  if (_starting) {
    throw new Error('Cannot terminate while startWorkers() is in progress. Await the startWorkers() promise first.');
  }

  if (_terminating) {
    return _terminating;
  }

  if (!_workers) {
    return;
  }

  const workers = _workers;
  _workers = undefined;
  _terminating = Promise.allSettled(workers.map((w) => w.terminate()));

  return _terminating;
}

////////////////////////////////////////////////////////////////////////////////
// Public exports
////////////////////////////////////////////////////////////////////////////////

export { getTfheWorkers, startWorkers, terminateWorkers, setWorkerUrlConfig };
