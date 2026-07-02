/**
 * Auto-generated from scripts/wasm/tfhe/startWorkers.template.js.
 * Embedded worker base64 payload SHA-256: 5fca7aa73420d7cc3b7239496ea5c70bc2cb520dd2f3f500801da18b126b0c54
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
 *  - Hash is the build-time constant "2c2ef02437e5b7a41f4c5c1defa0a24dcfae737037a59a145a972bb4191571b8".
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
// Load modes
////////////////////////////////////////////////////////////////////////////////

// Environment detection (browser vs Node) is NOT done here: the SDK resolves it
// once on the main thread (via environment.ts isBrowserLike, robust to bundler
// `process` shims) and injects it through setWorkerUrlConfig({ isBrowserLike }).
// See `_isBrowserLike` below.

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
  if (_isBrowserLike) {
    if (typeof crypto === 'undefined' || !crypto.subtle || typeof crypto.subtle.digest !== 'function') {
      throw new Error('Web Crypto SHA-256 digest is not available');
    }

    const hash = await crypto.subtle.digest('SHA-256', bytes);
    return __bytesToHex(new Uint8Array(hash));
  }

  const nodeModuleName = 'crypto';
  const nodeModuleId = `node:${nodeModuleName}`;
  const { createHash } = await import(
    /* @vite-ignore */ /* webpackIgnore: true */ /* turbopackIgnore: true */ nodeModuleId
  );
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
  if (!_isBrowserLike && url.protocol === 'file:') {
    const fsModuleName = 'fs/promises';
    const fsModuleId = `node:${fsModuleName}`;
    const urlModuleName = 'url';
    const urlModuleId = `node:${urlModuleName}`;
    const { readFile } = await import(
      /* @vite-ignore */ /* webpackIgnore: true */ /* turbopackIgnore: true */ fsModuleId
    );
    const { fileURLToPath } = await import(
      /* @vite-ignore */ /* webpackIgnore: true */ /* turbopackIgnore: true */ urlModuleId
    );
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
  if (_isBrowserLike) {
    return new Worker(url, {
      type: 'module',
      name: 'wasm_bindgen_worker',
    });
  }

  const nodeModuleName = 'worker_threads';
  const nodeModuleId = `node:${nodeModuleName}`;
  const { Worker: NodeWorker } = await import(
    /* @vite-ignore */ /* webpackIgnore: true */ /* turbopackIgnore: true */ nodeModuleId
  );
  return new NodeWorker(url);
}

async function __newNodeWorkerFromJsCode(jsCode) {
  const nodeModuleName = 'worker_threads';
  const nodeModuleId = `node:${nodeModuleName}`;
  const { Worker: NodeWorker } = await import(
    /* @vite-ignore */ /* webpackIgnore: true */ /* turbopackIgnore: true */ nodeModuleId
  );
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
  if (_isBrowserLike) {
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
  if (_isBrowserLike) {
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
// Injected by the SDK (the main thread) via setWorkerUrlConfig — the single
// source of truth for browser-vs-Node, replacing local detection.
let _isBrowserLike = undefined;
const _workerUrlSha256 = "2c2ef02437e5b7a41f4c5c1defa0a24dcfae737037a59a145a972bb4191571b8";
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

  const {
    workerUrl = undefined,
    wasmAssetLoadMode = 'auto',
    logger = undefined,
    isBrowserLike = undefined,
  } = parameters;

  // Check `isBrowserLike` (required: the SDK injects the resolved runtime kind;
  // the worker bootstrap never detects it itself).
  if (typeof isBrowserLike !== 'boolean') {
    throw new TypeError('setWorkerUrlConfig: isBrowserLike (boolean) is required');
  }

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
  _isBrowserLike = isBrowserLike;
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
 * 1. Fetch the URL and verify its SHA-256 against "2c2ef02437e5b7a41f4c5c1defa0a24dcfae737037a59a145a972bb4191571b8" — fails fast on mismatch.
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
  const workerBase64 = "ZnVuY3Rpb24gX19faXNCcm93c2VyTGlrZSgpIHsKICByZXR1cm4gKAogICAgdHlwZW9mIGFkZEV2ZW50TGlzdGVuZXIgPT09ICdmdW5jdGlvbicgJiYKICAgIHR5cGVvZiByZW1vdmVFdmVudExpc3RlbmVyID09PSAnZnVuY3Rpb24nCiAgKTsKfQoKYXN5bmMgZnVuY3Rpb24gX19fZ2V0VGFyZ2V0KCkgewogIGlmIChfX19pc0Jyb3dzZXJMaWtlKCkpIHJldHVybiBzZWxmOwogIGNvbnN0IG5vZGVNb2R1bGVOYW1lID0gJ3dvcmtlcl90aHJlYWRzJzsKICBjb25zdCBub2RlTW9kdWxlSWQgPSBgbm9kZToke25vZGVNb2R1bGVOYW1lfWA7CiAgY29uc3QgeyBwYXJlbnRQb3J0IH0gPSBhd2FpdCBpbXBvcnQoLyogQHZpdGUtaWdub3JlICovIG5vZGVNb2R1bGVJZCk7CiAgcmV0dXJuIHBhcmVudFBvcnQ7Cn0KCmZ1bmN0aW9uIF9fX3dhaXRGb3JNc2dUeXBlKHRhcmdldCwgdHlwZSkgewogIHJldHVybiBuZXcgUHJvbWlzZSgocmVzb2x2ZSkgPT4gewogICAgaWYgKHR5cGVvZiB0YXJnZXQub24gPT09ICdmdW5jdGlvbicpIHsKICAgICAgLy8gTm9kZTogRXZlbnRFbWl0dGVyLCBkYXRhIHBhc3NlZCBkaXJlY3RseQogICAgICB0YXJnZXQub24oJ21lc3NhZ2UnLCBmdW5jdGlvbiBvbk1zZyhkYXRhKSB7CiAgICAgICAgaWYgKGRhdGE/LnR5cGUgIT09IHR5cGUpIHJldHVybjsKICAgICAgICB0YXJnZXQub2ZmKCdtZXNzYWdlJywgb25Nc2cpOwogICAgICAgIHJlc29sdmUoZGF0YSk7CiAgICAgIH0pOwogICAgfSBlbHNlIHsKICAgICAgLy8gQnJvd3NlcjogRE9NIGV2ZW50cywgZGF0YSB3cmFwcGVkIGluIE1lc3NhZ2VFdmVudAogICAgICB0YXJnZXQuYWRkRXZlbnRMaXN0ZW5lcignbWVzc2FnZScsIGZ1bmN0aW9uIG9uTXNnKHsgZGF0YSB9KSB7CiAgICAgICAgaWYgKGRhdGE/LnR5cGUgIT09IHR5cGUpIHJldHVybjsKICAgICAgICB0YXJnZXQucmVtb3ZlRXZlbnRMaXN0ZW5lcignbWVzc2FnZScsIG9uTXNnKTsKICAgICAgICByZXNvbHZlKGRhdGEpOwogICAgICB9KTsKICAgIH0KICB9KTsKfQoKX19fZ2V0VGFyZ2V0KCkudGhlbigodGFyZ2V0KSA9PgogIF9fX3dhaXRGb3JNc2dUeXBlKHRhcmdldCwgJ3dhc21fYmluZGdlbl93b3JrZXJfaW5pdCcpLnRoZW4oCiAgICBhc3luYyAoeyBpbml0LCByZWNlaXZlciB9KSA9PiB7CiAgICAgIGNvbnN0IHBrZyA9IGF3YWl0IFByb21pc2UucmVzb2x2ZSgpLnRoZW4oZnVuY3Rpb24gKCkgewogICAgICAgIHJldHVybiB0ZmhlOwogICAgICB9KTsKICAgICAgYXdhaXQgcGtnLmRlZmF1bHQoaW5pdCk7CiAgICAgIHRhcmdldC5wb3N0TWVzc2FnZSh7IHR5cGU6ICd3YXNtX2JpbmRnZW5fd29ya2VyX3JlYWR5JyB9KTsKICAgICAgcGtnLndiZ19yYXlvbl9zdGFydF93b3JrZXIocmVjZWl2ZXIpOwogICAgfSwKICApLAopOwoKLyoqCiAqIEBwYXJhbSB7bnVtYmVyfSByZWNlaXZlcgogKi8KZnVuY3Rpb24gd2JnX3JheW9uX3N0YXJ0X3dvcmtlcihyZWNlaXZlcikgewogIHdhc20ud2JnX3JheW9uX3N0YXJ0X3dvcmtlcihyZWNlaXZlcik7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIEludGVybmFsIHdhc21iaW5kZ2VuIHRvb2xzCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLwovLyBJbXBvcnRzOgovLyBfX3diZ19nZXRfaW1wb3J0cwovLwovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gX193YmdfZ2V0X2ltcG9ydHMobWVtb3J5KSB7CiAgICBjb25zdCBpbXBvcnQwID0gewogICAgICAgIF9fcHJvdG9fXzogbnVsbCwKICAgICAgICBfX3diZ19CaWdJbnRfNTJmZjgzOTEyOTdkYTE5NDogZnVuY3Rpb24gKCkgewogICAgICAgICAgICByZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgICAgIGNvbnN0IHJldCA9IEJpZ0ludChhcmcwKTsKICAgICAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgICAgIH0sIGFyZ3VtZW50cyk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19CaWdJbnRfYWUyMDBlOTNjYWNiZDJiMzogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gQmlnSW50KGFyZzApOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfRXJyb3JfMzYzOWE2MGVkMTVmODdlNzogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gRXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGFyZzAsIGFyZzEpKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fYmlnaW50X2dldF9hc19pNjRfM2FmNmQ0Y2E3NzE5M2E0YjogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgdiA9IGFyZzE7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiAodikgPT09ICdiaWdpbnQnID8gdiA6IHVuZGVmaW5lZDsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0QmlnSW50NjQoYXJnMCArIDggKiAxLCBpc0xpa2VOb25lKHJldCkgPyBCaWdJbnQoMCkgOiByZXQsIHRydWUpOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDAsICFpc0xpa2VOb25lKHJldCksIHRydWUpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9iaXRfYW5kX2JmZWRlY2UyYmI2NTdmNGI6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAgJiBhcmcxOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9kZWJ1Z19zdHJpbmdfMDdjYjcyY2ZjYzk1MmUyYjogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gZGVidWdTdHJpbmcoYXJnMSk7CiAgICAgICAgICAgIGNvbnN0IHB0cjEgPSBwYXNzU3RyaW5nVG9XYXNtMChyZXQsIHdhc20uX193YmluZGdlbl9tYWxsb2MsIHdhc20uX193YmluZGdlbl9yZWFsbG9jKTsKICAgICAgICAgICAgY29uc3QgbGVuMSA9IFdBU01fVkVDVE9SX0xFTjsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAxLCBsZW4xLCB0cnVlKTsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAwLCBwdHIxLCB0cnVlKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5faXNfZnVuY3Rpb25fMmYwZmQ3Y2ViODZlNjRjNTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mIChhcmcwKSA9PT0gJ2Z1bmN0aW9uJzsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5faXNfb2JqZWN0XzViMjJmZjI0MTgwNjNhOWM6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHZhbCA9IGFyZzA7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiAodmFsKSA9PT0gJ29iamVjdCcgJiYgdmFsICE9PSBudWxsOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9pc19zdHJpbmdfZWRkYzA3YTNlZmFkNTJlNjogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mIChhcmcwKSA9PT0gJ3N0cmluZyc7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2lzX3VuZGVmaW5lZF8yNDRhOTJjMzRkM2I2ZWMwOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwID09PSB1bmRlZmluZWQ7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2pzdmFsX2VxXzQwM2VhYTM2MTA1MDBhMjU6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAgPT09IGFyZzE7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2x0X2M0ODNjYzY5NGRlNjdjM2U6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAgPCBhcmcxOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9tZW1vcnlfYzIzNTZkZDFhMDg5ZGZiZDogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB3YXNtLm1lbW9yeTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fbW9kdWxlX2RmNzA0MzkzZGZkMTg1M2M6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gd2FzbU1vZHVsZTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fbmVnXzliNGQ3MTgyM2UzYmM1MTM6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IC1hcmcwOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9zaHJfZDhmODI2OGYxOGM3YTFjMzogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMCA+PiBhcmcxOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9zdHJpbmdfZ2V0Xzk2NTU5MjA3M2U1ZDg0OGM6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IG9iaiA9IGFyZzE7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiAob2JqKSA9PT0gJ3N0cmluZycgPyBvYmogOiB1bmRlZmluZWQ7CiAgICAgICAgICAgIHZhciBwdHIxID0gaXNMaWtlTm9uZShyZXQpID8gMCA6IHBhc3NTdHJpbmdUb1dhc20wKHJldCwgd2FzbS5fX3diaW5kZ2VuX21hbGxvYywgd2FzbS5fX3diaW5kZ2VuX3JlYWxsb2MpOwogICAgICAgICAgICB2YXIgbGVuMSA9IFdBU01fVkVDVE9SX0xFTjsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAxLCBsZW4xLCB0cnVlKTsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAwLCBwdHIxLCB0cnVlKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fdGhyb3dfOWM3NWQ0N2JmOWU3NzMxZTogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgdGhyb3cgbmV3IEVycm9yKGdldFN0cmluZ0Zyb21XYXNtMChhcmcwLCBhcmcxKSk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19jYWxsX2E0MWQ2NDIxYjMwYTMyYzU6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgcmV0dXJuIGhhbmRsZUVycm9yKGZ1bmN0aW9uIChhcmcwLCBhcmcxLCBhcmcyKSB7CiAgICAgICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLmNhbGwoYXJnMSwgYXJnMik7CiAgICAgICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgICAgICB9LCBhcmd1bWVudHMpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfY3J5cHRvXzM4ZGYyYmFiMTI2YjYzZGM6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAuY3J5cHRvOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfZXJyb3JfYTZmYTIwMmI1OGFhMWNkMzogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgbGV0IGRlZmVycmVkMF8wOwogICAgICAgICAgICBsZXQgZGVmZXJyZWQwXzE7CiAgICAgICAgICAgIHRyeSB7CiAgICAgICAgICAgICAgICBkZWZlcnJlZDBfMCA9IGFyZzA7CiAgICAgICAgICAgICAgICBkZWZlcnJlZDBfMSA9IGFyZzE7CiAgICAgICAgICAgICAgICBjb25zb2xlLmVycm9yKGdldFN0cmluZ0Zyb21XYXNtMChhcmcwLCBhcmcxKSk7CiAgICAgICAgICAgIH0KICAgICAgICAgICAgZmluYWxseSB7CiAgICAgICAgICAgICAgICB3YXNtLl9fd2JpbmRnZW5fZnJlZShkZWZlcnJlZDBfMCwgZGVmZXJyZWQwXzEsIDEpOwogICAgICAgICAgICB9CiAgICAgICAgfSwKICAgICAgICBfX3diZ19nZXRSYW5kb21WYWx1ZXNfYzQ0YTUwZDhjZmRhZWJlYjogZnVuY3Rpb24gKCkgewogICAgICAgICAgICByZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgICAgIGFyZzAuZ2V0UmFuZG9tVmFsdWVzKGFyZzEpOwogICAgICAgICAgICB9LCBhcmd1bWVudHMpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfaW5zdGFuY2VvZl9XaW5kb3dfNDE1M2MxODE4YTFjMGMwYjogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgbGV0IHJlc3VsdDsKICAgICAgICAgICAgdHJ5IHsKICAgICAgICAgICAgICAgIHJlc3VsdCA9IGFyZzAgaW5zdGFuY2VvZiBXaW5kb3c7CiAgICAgICAgICAgIH0KICAgICAgICAgICAgY2F0Y2ggKF8pIHsKICAgICAgICAgICAgICAgIHJlc3VsdCA9IGZhbHNlOwogICAgICAgICAgICB9CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHJlc3VsdDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX2xlbmd0aF9iYTNjMDMyNjAyZWZlMzEwOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLmxlbmd0aDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX21zQ3J5cHRvX2JkNWEwMzRhZjk2YmNiYTY6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAubXNDcnlwdG87CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19uZXdfMjI3ZDdjMDU0MTRlYjg2MTogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBuZXcgRXJyb3IoKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX25ld193aXRoX2xlbmd0aF85MDExZjVkYTc5NGJmNWQ5OiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBuZXcgVWludDhBcnJheShhcmcwID4+PiAwKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX25vZGVfODRlYTg3NTQxMTI1NGRiMTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5ub2RlOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfcHJvY2Vzc180NGM3YTE0ZTExZTlmNjllOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLnByb2Nlc3M7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19wcm90b3R5cGVzZXRjYWxsX2ZkNDA1MGU4MDZlMWQ1MTk6IGZ1bmN0aW9uIChhcmcwLCBhcmcxLCBhcmcyKSB7CiAgICAgICAgICAgIFVpbnQ4QXJyYXkucHJvdG90eXBlLnNldC5jYWxsKGdldEFycmF5VThGcm9tV2FzbTAoYXJnMCwgYXJnMSksIGFyZzIpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfcmFuZG9tRmlsbFN5bmNfNmMyNWVhYzk4NjllYjUzYzogZnVuY3Rpb24gKCkgewogICAgICAgICAgICByZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgICAgIGFyZzAucmFuZG9tRmlsbFN5bmMoYXJnMSk7CiAgICAgICAgICAgIH0sIGFyZ3VtZW50cyk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19yZXF1aXJlX2I0ZWRiZGNmM2UyYTFlZjA6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgcmV0dXJuIGhhbmRsZUVycm9yKGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgICAgIGNvbnN0IHJldCA9IG1vZHVsZS5yZXF1aXJlOwogICAgICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICAgICAgfSwgYXJndW1lbnRzKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3N0YWNrXzNiMGQ5NzRiYmYzMWU0NGY6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzEuc3RhY2s7CiAgICAgICAgICAgIGNvbnN0IHB0cjEgPSBwYXNzU3RyaW5nVG9XYXNtMChyZXQsIHdhc20uX193YmluZGdlbl9tYWxsb2MsIHdhc20uX193YmluZGdlbl9yZWFsbG9jKTsKICAgICAgICAgICAgY29uc3QgbGVuMSA9IFdBU01fVkVDVE9SX0xFTjsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAxLCBsZW4xLCB0cnVlKTsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAwLCBwdHIxLCB0cnVlKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3N0YXJ0V29ya2Vyc184YjU4MmQ1N2U5MmJkMmQ0OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSwgYXJnMikgewogICAgICAgICAgICBoYW5kbGVFcnJvcihmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgICAgICB0aHJvdyBuZXcgRXJyb3IoJ3N0YXJ0V29ya2VycyBub3Qgc3VwcG9ydGVkIGZyb20gYSB3b3JrZXIgdGhyZWFkJyk7CiAgICAgICAgICAgIH0pOwogICAgICAgICAgICAvLyBjb25zdCByZXQgPSBzdGFydFdvcmtlcnMoYXJnMCwgYXJnMSwgd2JnX3JheW9uX1Bvb2xCdWlsZGVyLl9fd3JhcChhcmcyKSk7CiAgICAgICAgICAgIC8vIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdGF0aWNfYWNjZXNzb3JfR0xPQkFMX1RISVNfMWM3ZjFiZDZjNjk0MWZkYjogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgZ2xvYmFsVGhpcyA9PT0gJ3VuZGVmaW5lZCcgPyBudWxsIDogZ2xvYmFsVGhpczsKICAgICAgICAgICAgcmV0dXJuIGlzTGlrZU5vbmUocmV0KSA/IDAgOiBhZGRUb0V4dGVybnJlZlRhYmxlMChyZXQpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3RhdGljX2FjY2Vzc29yX0dMT0JBTF9lMDM5YmM5MTRmODNlNzRlOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiBnbG9iYWwgPT09ICd1bmRlZmluZWQnID8gbnVsbCA6IGdsb2JhbDsKICAgICAgICAgICAgcmV0dXJuIGlzTGlrZU5vbmUocmV0KSA/IDAgOiBhZGRUb0V4dGVybnJlZlRhYmxlMChyZXQpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3RhdGljX2FjY2Vzc29yX1NFTEZfOGJmOGM0OGMyODQyMGFkNTogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2Ygc2VsZiA9PT0gJ3VuZGVmaW5lZCcgPyBudWxsIDogc2VsZjsKICAgICAgICAgICAgcmV0dXJuIGlzTGlrZU5vbmUocmV0KSA/IDAgOiBhZGRUb0V4dGVybnJlZlRhYmxlMChyZXQpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3RhdGljX2FjY2Vzc29yX1dJTkRPV182YWVlZTliNTE2NTJlZTBmOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiB3aW5kb3cgPT09ICd1bmRlZmluZWQnID8gbnVsbCA6IHdpbmRvdzsKICAgICAgICAgICAgcmV0dXJuIGlzTGlrZU5vbmUocmV0KSA/IDAgOiBhZGRUb0V4dGVybnJlZlRhYmxlMChyZXQpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3ViYXJyYXlfZmJlM2NlZjI5MGUxZmE0MzogZnVuY3Rpb24gKGFyZzAsIGFyZzEsIGFyZzIpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5zdWJhcnJheShhcmcxID4+PiAwLCBhcmcyID4+PiAwKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3RvU3RyaW5nXzE1NjU2YWY4ZDhlNzFmMTY6IGZ1bmN0aW9uIChhcmcwLCBhcmcxLCBhcmcyKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzEudG9TdHJpbmcoYXJnMik7CiAgICAgICAgICAgIGNvbnN0IHB0cjEgPSBwYXNzU3RyaW5nVG9XYXNtMChyZXQsIHdhc20uX193YmluZGdlbl9tYWxsb2MsIHdhc20uX193YmluZGdlbl9yZWFsbG9jKTsKICAgICAgICAgICAgY29uc3QgbGVuMSA9IFdBU01fVkVDVE9SX0xFTjsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAxLCBsZW4xLCB0cnVlKTsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoYXJnMCArIDQgKiAwLCBwdHIxLCB0cnVlKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3RvU3RyaW5nXzhkODc0NDg5YmFkN2U1YTI6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAudG9TdHJpbmcoKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3ZlcnNpb25zXzI3NmIyNzk1YjFjNmEyMTk6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAudmVyc2lvbnM7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diaW5kZ2VuX2Nhc3RfMDAwMDAwMDAwMDAwMDAwMTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgLy8gQ2FzdCBpbnRyaW5zaWMgZm9yIGBGNjQgLT4gRXh0ZXJucmVmYC4KICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDAyOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICAvLyBDYXN0IGludHJpbnNpYyBmb3IgYEk2NCAtPiBFeHRlcm5yZWZgLgogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDM6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIC8vIENhc3QgaW50cmluc2ljIGZvciBgUmVmKFNsaWNlKFU4KSkgLT4gTmFtZWRFeHRlcm5yZWYoIlVpbnQ4QXJyYXkiKWAuCiAgICAgICAgICAgIGNvbnN0IHJldCA9IGdldEFycmF5VThGcm9tV2FzbTAoYXJnMCwgYXJnMSk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diaW5kZ2VuX2Nhc3RfMDAwMDAwMDAwMDAwMDAwNDogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgLy8gQ2FzdCBpbnRyaW5zaWMgZm9yIGBSZWYoU3RyaW5nKSAtPiBFeHRlcm5yZWZgLgogICAgICAgICAgICBjb25zdCByZXQgPSBnZXRTdHJpbmdGcm9tV2FzbTAoYXJnMCwgYXJnMSk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diaW5kZ2VuX2Nhc3RfMDAwMDAwMDAwMDAwMDAwNTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgLy8gQ2FzdCBpbnRyaW5zaWMgZm9yIGBVNjQgLT4gRXh0ZXJucmVmYC4KICAgICAgICAgICAgY29uc3QgcmV0ID0gQmlnSW50LmFzVWludE4oNjQsIGFyZzApOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmluZGdlbl9pbml0X2V4dGVybnJlZl90YWJsZTogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCB0YWJsZSA9IHdhc20uX193YmluZGdlbl9leHRlcm5yZWZzOwogICAgICAgICAgICBjb25zdCBvZmZzZXQgPSB0YWJsZS5ncm93KDQpOwogICAgICAgICAgICB0YWJsZS5zZXQoMCwgdW5kZWZpbmVkKTsKICAgICAgICAgICAgdGFibGUuc2V0KG9mZnNldCArIDAsIHVuZGVmaW5lZCk7CiAgICAgICAgICAgIHRhYmxlLnNldChvZmZzZXQgKyAxLCBudWxsKTsKICAgICAgICAgICAgdGFibGUuc2V0KG9mZnNldCArIDIsIHRydWUpOwogICAgICAgICAgICB0YWJsZS5zZXQob2Zmc2V0ICsgMywgZmFsc2UpOwogICAgICAgIH0sCiAgICAgICAgbWVtb3J5OiBtZW1vcnkgfHwgbmV3IFdlYkFzc2VtYmx5Lk1lbW9yeSh7IGluaXRpYWw6IDE5LCBtYXhpbXVtOiAxNjM4NCwgc2hhcmVkOiB0cnVlIH0pLAogICAgfTsKICAgIHJldHVybiB7CiAgICAgICAgX19wcm90b19fOiBudWxsLAogICAgICAgICIuL3RmaGVfYmcuanMiOiBpbXBvcnQwLAogICAgfTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gYWRkVG9FeHRlcm5yZWZUYWJsZTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGFkZFRvRXh0ZXJucmVmVGFibGUwKG9iaikgewogICAgY29uc3QgaWR4ID0gd2FzbS5fX2V4dGVybnJlZl90YWJsZV9hbGxvYygpOwogICAgd2FzbS5fX3diaW5kZ2VuX2V4dGVybnJlZnMuc2V0KGlkeCwgb2JqKTsKICAgIHJldHVybiBpZHg7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGRlYnVnU3RyaW5nCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBkZWJ1Z1N0cmluZyh2YWwpIHsKICAgIC8vIHByaW1pdGl2ZSB0eXBlcwogICAgY29uc3QgdHlwZSA9IHR5cGVvZiB2YWw7CiAgICBpZiAodHlwZSA9PSAnbnVtYmVyJyB8fCB0eXBlID09ICdib29sZWFuJyB8fCB2YWwgPT0gbnVsbCkgewogICAgICAgIHJldHVybiBgJHt2YWx9YDsKICAgIH0KICAgIGlmICh0eXBlID09ICdzdHJpbmcnKSB7CiAgICAgICAgcmV0dXJuIGAiJHt2YWx9ImA7CiAgICB9CiAgICBpZiAodHlwZSA9PSAnc3ltYm9sJykgewogICAgICAgIGNvbnN0IGRlc2NyaXB0aW9uID0gdmFsLmRlc2NyaXB0aW9uOwogICAgICAgIGlmIChkZXNjcmlwdGlvbiA9PSBudWxsKSB7CiAgICAgICAgICAgIHJldHVybiAnU3ltYm9sJzsKICAgICAgICB9CiAgICAgICAgZWxzZSB7CiAgICAgICAgICAgIHJldHVybiBgU3ltYm9sKCR7ZGVzY3JpcHRpb259KWA7CiAgICAgICAgfQogICAgfQogICAgaWYgKHR5cGUgPT0gJ2Z1bmN0aW9uJykgewogICAgICAgIGNvbnN0IG5hbWUgPSB2YWwubmFtZTsKICAgICAgICBpZiAodHlwZW9mIG5hbWUgPT0gJ3N0cmluZycgJiYgbmFtZS5sZW5ndGggPiAwKSB7CiAgICAgICAgICAgIHJldHVybiBgRnVuY3Rpb24oJHtuYW1lfSlgOwogICAgICAgIH0KICAgICAgICBlbHNlIHsKICAgICAgICAgICAgcmV0dXJuICdGdW5jdGlvbic7CiAgICAgICAgfQogICAgfQogICAgLy8gb2JqZWN0cwogICAgaWYgKEFycmF5LmlzQXJyYXkodmFsKSkgewogICAgICAgIGNvbnN0IGxlbmd0aCA9IHZhbC5sZW5ndGg7CiAgICAgICAgbGV0IGRlYnVnID0gJ1snOwogICAgICAgIGlmIChsZW5ndGggPiAwKSB7CiAgICAgICAgICAgIGRlYnVnICs9IGRlYnVnU3RyaW5nKHZhbFswXSk7CiAgICAgICAgfQogICAgICAgIGZvciAobGV0IGkgPSAxOyBpIDwgbGVuZ3RoOyBpKyspIHsKICAgICAgICAgICAgZGVidWcgKz0gJywgJyArIGRlYnVnU3RyaW5nKHZhbFtpXSk7CiAgICAgICAgfQogICAgICAgIGRlYnVnICs9ICddJzsKICAgICAgICByZXR1cm4gZGVidWc7CiAgICB9CiAgICAvLyBUZXN0IGZvciBidWlsdC1pbgogICAgY29uc3QgYnVpbHRJbk1hdGNoZXMgPSAvXFtvYmplY3QgKFteXF1dKylcXS8uZXhlYyh0b1N0cmluZy5jYWxsKHZhbCkpOwogICAgbGV0IGNsYXNzTmFtZTsKICAgIGlmIChidWlsdEluTWF0Y2hlcyAmJiBidWlsdEluTWF0Y2hlcy5sZW5ndGggPiAxKSB7CiAgICAgICAgY2xhc3NOYW1lID0gYnVpbHRJbk1hdGNoZXNbMV07CiAgICB9CiAgICBlbHNlIHsKICAgICAgICAvLyBGYWlsZWQgdG8gbWF0Y2ggdGhlIHN0YW5kYXJkICdbb2JqZWN0IENsYXNzTmFtZV0nCiAgICAgICAgcmV0dXJuIHRvU3RyaW5nLmNhbGwodmFsKTsKICAgIH0KICAgIGlmIChjbGFzc05hbWUgPT0gJ09iamVjdCcpIHsKICAgICAgICAvLyB3ZSdyZSBhIHVzZXIgZGVmaW5lZCBjbGFzcyBvciBPYmplY3QKICAgICAgICAvLyBKU09OLnN0cmluZ2lmeSBhdm9pZHMgcHJvYmxlbXMgd2l0aCBjeWNsZXMsIGFuZCBpcyBnZW5lcmFsbHkgbXVjaAogICAgICAgIC8vIGVhc2llciB0aGFuIGxvb3BpbmcgdGhyb3VnaCBvd25Qcm9wZXJ0aWVzIG9mIGB2YWxgLgogICAgICAgIHRyeSB7CiAgICAgICAgICAgIHJldHVybiAnT2JqZWN0KCcgKyBKU09OLnN0cmluZ2lmeSh2YWwpICsgJyknOwogICAgICAgIH0KICAgICAgICBjYXRjaCAoXykgewogICAgICAgICAgICByZXR1cm4gJ09iamVjdCc7CiAgICAgICAgfQogICAgfQogICAgLy8gZXJyb3JzCiAgICBpZiAodmFsIGluc3RhbmNlb2YgRXJyb3IpIHsKICAgICAgICByZXR1cm4gYCR7dmFsLm5hbWV9OiAke3ZhbC5tZXNzYWdlfVxuJHt2YWwuc3RhY2t9YDsKICAgIH0KICAgIC8vIFRPRE8gd2UgY291bGQgdGVzdCBmb3IgbW9yZSB0aGluZ3MgaGVyZSwgbGlrZSBgU2V0YHMgYW5kIGBNYXBgcy4KICAgIHJldHVybiBjbGFzc05hbWU7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGdldEFycmF5VThGcm9tV2FzbTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGdldEFycmF5VThGcm9tV2FzbTAocHRyLCBsZW4pIHsKICAgIHB0ciA9IHB0ciA+Pj4gMDsKICAgIHJldHVybiBnZXRVaW50OEFycmF5TWVtb3J5MCgpLnN1YmFycmF5KHB0ciAvIDEsIHB0ciAvIDEgKyBsZW4pOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBjYWNoZWREYXRhVmlld01lbW9yeTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmxldCBjYWNoZWREYXRhVmlld01lbW9yeTAgPSBudWxsOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gZ2V0RGF0YVZpZXdNZW1vcnkwCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBnZXREYXRhVmlld01lbW9yeTAoKSB7CiAgICBpZiAoY2FjaGVkRGF0YVZpZXdNZW1vcnkwID09PSBudWxsIHx8IGNhY2hlZERhdGFWaWV3TWVtb3J5MC5idWZmZXIgIT09IHdhc20ubWVtb3J5LmJ1ZmZlcikgewogICAgICAgIGNhY2hlZERhdGFWaWV3TWVtb3J5MCA9IG5ldyBEYXRhVmlldyh3YXNtLm1lbW9yeS5idWZmZXIpOwogICAgfQogICAgcmV0dXJuIGNhY2hlZERhdGFWaWV3TWVtb3J5MDsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gZ2V0U3RyaW5nRnJvbVdhc20wCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBnZXRTdHJpbmdGcm9tV2FzbTAocHRyLCBsZW4pIHsKICAgIHJldHVybiBkZWNvZGVUZXh0KHB0ciA+Pj4gMCwgbGVuKTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gY2FjaGVkVWludDhBcnJheU1lbW9yeTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmxldCBjYWNoZWRVaW50OEFycmF5TWVtb3J5MCA9IG51bGw7CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBnZXRVaW50OEFycmF5TWVtb3J5MAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gZ2V0VWludDhBcnJheU1lbW9yeTAoKSB7CiAgICBpZiAoY2FjaGVkVWludDhBcnJheU1lbW9yeTAgPT09IG51bGwgfHwgY2FjaGVkVWludDhBcnJheU1lbW9yeTAuYnVmZmVyICE9PSB3YXNtLm1lbW9yeS5idWZmZXIpIHsKICAgICAgICBjYWNoZWRVaW50OEFycmF5TWVtb3J5MCA9IG5ldyBVaW50OEFycmF5KHdhc20ubWVtb3J5LmJ1ZmZlcik7CiAgICB9CiAgICByZXR1cm4gY2FjaGVkVWludDhBcnJheU1lbW9yeTA7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGhhbmRsZUVycm9yCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBoYW5kbGVFcnJvcihmLCBhcmdzKSB7CiAgICB0cnkgewogICAgICAgIHJldHVybiBmLmFwcGx5KHRoaXMsIGFyZ3MpOwogICAgfQogICAgY2F0Y2ggKGUpIHsKICAgICAgICBjb25zdCBpZHggPSBhZGRUb0V4dGVybnJlZlRhYmxlMChlKTsKICAgICAgICB3YXNtLl9fd2JpbmRnZW5fZXhuX3N0b3JlKGlkeCk7CiAgICB9Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGlzTGlrZU5vbmUKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGlzTGlrZU5vbmUoeCkgewogICAgcmV0dXJuIHggPT09IHVuZGVmaW5lZCB8fCB4ID09PSBudWxsOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBwYXNzU3RyaW5nVG9XYXNtMAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gcGFzc1N0cmluZ1RvV2FzbTAoYXJnLCBtYWxsb2MsIHJlYWxsb2MpIHsKICAgIGlmIChyZWFsbG9jID09PSB1bmRlZmluZWQpIHsKICAgICAgICBjb25zdCBidWYgPSBjYWNoZWRUZXh0RW5jb2Rlci5lbmNvZGUoYXJnKTsKICAgICAgICBjb25zdCBwdHIgPSBtYWxsb2MoYnVmLmxlbmd0aCwgMSkgPj4+IDA7CiAgICAgICAgZ2V0VWludDhBcnJheU1lbW9yeTAoKS5zdWJhcnJheShwdHIsIHB0ciArIGJ1Zi5sZW5ndGgpLnNldChidWYpOwogICAgICAgIFdBU01fVkVDVE9SX0xFTiA9IGJ1Zi5sZW5ndGg7CiAgICAgICAgcmV0dXJuIHB0cjsKICAgIH0KICAgIGxldCBsZW4gPSBhcmcubGVuZ3RoOwogICAgbGV0IHB0ciA9IG1hbGxvYyhsZW4sIDEpID4+PiAwOwogICAgY29uc3QgbWVtID0gZ2V0VWludDhBcnJheU1lbW9yeTAoKTsKICAgIGxldCBvZmZzZXQgPSAwOwogICAgZm9yICg7IG9mZnNldCA8IGxlbjsgb2Zmc2V0KyspIHsKICAgICAgICBjb25zdCBjb2RlID0gYXJnLmNoYXJDb2RlQXQob2Zmc2V0KTsKICAgICAgICBpZiAoY29kZSA+IDB4N0YpCiAgICAgICAgICAgIGJyZWFrOwogICAgICAgIG1lbVtwdHIgKyBvZmZzZXRdID0gY29kZTsKICAgIH0KICAgIGlmIChvZmZzZXQgIT09IGxlbikgewogICAgICAgIGlmIChvZmZzZXQgIT09IDApIHsKICAgICAgICAgICAgYXJnID0gYXJnLnNsaWNlKG9mZnNldCk7CiAgICAgICAgfQogICAgICAgIHB0ciA9IHJlYWxsb2MocHRyLCBsZW4sIGxlbiA9IG9mZnNldCArIGFyZy5sZW5ndGggKiAzLCAxKSA+Pj4gMDsKICAgICAgICBjb25zdCB2aWV3ID0gZ2V0VWludDhBcnJheU1lbW9yeTAoKS5zdWJhcnJheShwdHIgKyBvZmZzZXQsIHB0ciArIGxlbik7CiAgICAgICAgY29uc3QgcmV0ID0gY2FjaGVkVGV4dEVuY29kZXIuZW5jb2RlSW50byhhcmcsIHZpZXcpOwogICAgICAgIG9mZnNldCArPSByZXQud3JpdHRlbjsKICAgICAgICBwdHIgPSByZWFsbG9jKHB0ciwgbGVuLCBvZmZzZXQsIDEpID4+PiAwOwogICAgfQogICAgV0FTTV9WRUNUT1JfTEVOID0gb2Zmc2V0OwogICAgcmV0dXJuIHB0cjsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gY2FjaGVkVGV4dERlY29kZXIKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmxldCBjYWNoZWRUZXh0RGVjb2RlciA9ICh0eXBlb2YgVGV4dERlY29kZXIgIT09ICd1bmRlZmluZWQnID8gbmV3IFRleHREZWNvZGVyKCd1dGYtOCcsIHsgaWdub3JlQk9NOiB0cnVlLCBmYXRhbDogdHJ1ZSB9KSA6IHVuZGVmaW5lZCk7CgppZiAoY2FjaGVkVGV4dERlY29kZXIpCiAgICBjYWNoZWRUZXh0RGVjb2Rlci5kZWNvZGUoKTsKCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIE1BWF9TQUZBUklfREVDT0RFX0JZVEVTCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpjb25zdCBNQVhfU0FGQVJJX0RFQ09ERV9CWVRFUyA9IDIxNDY0MzUwNzI7CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBudW1CeXRlc0RlY29kZWQKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmxldCBudW1CeXRlc0RlY29kZWQgPSAwOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gZGVjb2RlVGV4dAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gZGVjb2RlVGV4dChwdHIsIGxlbikgewogICAgbnVtQnl0ZXNEZWNvZGVkICs9IGxlbjsKICAgIGlmIChudW1CeXRlc0RlY29kZWQgPj0gTUFYX1NBRkFSSV9ERUNPREVfQllURVMpIHsKICAgICAgICBjYWNoZWRUZXh0RGVjb2RlciA9IG5ldyBUZXh0RGVjb2RlcigndXRmLTgnLCB7IGlnbm9yZUJPTTogdHJ1ZSwgZmF0YWw6IHRydWUgfSk7CiAgICAgICAgY2FjaGVkVGV4dERlY29kZXIuZGVjb2RlKCk7CiAgICAgICAgbnVtQnl0ZXNEZWNvZGVkID0gbGVuOwogICAgfQogICAgcmV0dXJuIGNhY2hlZFRleHREZWNvZGVyLmRlY29kZShnZXRVaW50OEFycmF5TWVtb3J5MCgpLnNsaWNlKHB0ciwgcHRyICsgbGVuKSk7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGNhY2hlZFRleHRFbmNvZGVyCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpjb25zdCBjYWNoZWRUZXh0RW5jb2RlciA9ICh0eXBlb2YgVGV4dEVuY29kZXIgIT09ICd1bmRlZmluZWQnID8gbmV3IFRleHRFbmNvZGVyKCkgOiB1bmRlZmluZWQpOwoKaWYgKGNhY2hlZFRleHRFbmNvZGVyKSB7CiAgICBjYWNoZWRUZXh0RW5jb2Rlci5lbmNvZGVJbnRvID0gZnVuY3Rpb24gKGFyZywgdmlldykgewogICAgICAgIGNvbnN0IGJ1ZiA9IGNhY2hlZFRleHRFbmNvZGVyLmVuY29kZShhcmcpOwogICAgICAgIHZpZXcuc2V0KGJ1Zik7CiAgICAgICAgcmV0dXJuIHsKICAgICAgICAgICAgcmVhZDogYXJnLmxlbmd0aCwKICAgICAgICAgICAgd3JpdHRlbjogYnVmLmxlbmd0aAogICAgICAgIH07CiAgICB9Owp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBXQVNNX1ZFQ1RPUl9MRU4gaXMgYSBtb2R1bGUtbGV2ZWwgdmFyaWFibGUgdGhhdCBzdG9yZXMgdGhlIGJ5dGUgbGVuZ3RoIG9mCi8vIHRoZSBkYXRhIGp1c3Qgd3JpdHRlbiBpbnRvIFdBU00gbWVtb3J5LiBJdCBhY3RzIGFzIGFuIG91dC1wYXJhbWV0ZXIuCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpsZXQgV0FTTV9WRUNUT1JfTEVOID0gMDsKCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIFdBU00gbW9kdWxlIHN0YXRlCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpsZXQgd2FzbU1vZHVsZSwgd2FzbUluc3RhbmNlLCB3YXNtOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gSW5pdDoKLy8gX193YmdfZmluYWxpemVfaW5pdAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gX193YmdfZmluYWxpemVfaW5pdChpbnN0YW5jZSwgbW9kdWxlLCB0aHJlYWRfc3RhY2tfc2l6ZSkgewogICAgd2FzbUluc3RhbmNlID0gaW5zdGFuY2U7CiAgICB3YXNtID0gaW5zdGFuY2UuZXhwb3J0czsKICAgIHdhc21Nb2R1bGUgPSBtb2R1bGU7CiAgICBjYWNoZWREYXRhVmlld01lbW9yeTAgPSBudWxsOwogICAgY2FjaGVkVWludDhBcnJheU1lbW9yeTAgPSBudWxsOwogICAgaWYgKHR5cGVvZiB0aHJlYWRfc3RhY2tfc2l6ZSAhPT0gJ3VuZGVmaW5lZCcgJiYgKHR5cGVvZiB0aHJlYWRfc3RhY2tfc2l6ZSAhPT0gJ251bWJlcicgfHwgdGhyZWFkX3N0YWNrX3NpemUgPT09IDAgfHwgdGhyZWFkX3N0YWNrX3NpemUgJSA2NTUzNiAhPT0gMCkpIHsKICAgICAgICB0aHJvdyBuZXcgRXJyb3IoJ2ludmFsaWQgc3RhY2sgc2l6ZScpOwogICAgfQogICAgd2FzbS5fX3diaW5kZ2VuX3N0YXJ0KHRocmVhZF9zdGFja19zaXplKTsKICAgIHJldHVybiB3YXNtOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBJbml0OgovLyBfX3diZ19sb2FkCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgphc3luYyBmdW5jdGlvbiBfX3diZ19sb2FkKG1vZHVsZSwgaW1wb3J0cykgewogICAgaWYgKHR5cGVvZiBSZXNwb25zZSA9PT0gJ2Z1bmN0aW9uJyAmJiBtb2R1bGUgaW5zdGFuY2VvZiBSZXNwb25zZSkgewogICAgICAgIGlmICh0eXBlb2YgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGVTdHJlYW1pbmcgPT09ICdmdW5jdGlvbicpIHsKICAgICAgICAgICAgdHJ5IHsKICAgICAgICAgICAgICAgIHJldHVybiBhd2FpdCBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZVN0cmVhbWluZyhtb2R1bGUsIGltcG9ydHMpOwogICAgICAgICAgICB9CiAgICAgICAgICAgIGNhdGNoIChlKSB7CiAgICAgICAgICAgICAgICBjb25zdCB2YWxpZFJlc3BvbnNlID0gbW9kdWxlLm9rICYmIGV4cGVjdGVkUmVzcG9uc2VUeXBlKG1vZHVsZS50eXBlKTsKICAgICAgICAgICAgICAgIGlmICh2YWxpZFJlc3BvbnNlICYmIG1vZHVsZS5oZWFkZXJzLmdldCgnQ29udGVudC1UeXBlJykgIT09ICdhcHBsaWNhdGlvbi93YXNtJykgewogICAgICAgICAgICAgICAgICAgIGNvbnNvbGUud2FybigiYFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlU3RyZWFtaW5nYCBmYWlsZWQgYmVjYXVzZSB5b3VyIHNlcnZlciBkb2VzIG5vdCBzZXJ2ZSBXYXNtIHdpdGggYGFwcGxpY2F0aW9uL3dhc21gIE1JTUUgdHlwZS4gRmFsbGluZyBiYWNrIHRvIGBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZWAgd2hpY2ggaXMgc2xvd2VyLiBPcmlnaW5hbCBlcnJvcjpcbiIsIGUpOwogICAgICAgICAgICAgICAgfQogICAgICAgICAgICAgICAgZWxzZSB7CiAgICAgICAgICAgICAgICAgICAgdGhyb3cgZTsKICAgICAgICAgICAgICAgIH0KICAgICAgICAgICAgfQogICAgICAgIH0KICAgICAgICBjb25zdCBieXRlcyA9IGF3YWl0IG1vZHVsZS5hcnJheUJ1ZmZlcigpOwogICAgICAgIHJldHVybiBhd2FpdCBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZShieXRlcywgaW1wb3J0cyk7CiAgICB9CiAgICBlbHNlIHsKICAgICAgICBjb25zdCBpbnN0YW5jZSA9IGF3YWl0IFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlKG1vZHVsZSwgaW1wb3J0cyk7CiAgICAgICAgaWYgKGluc3RhbmNlIGluc3RhbmNlb2YgV2ViQXNzZW1ibHkuSW5zdGFuY2UpIHsKICAgICAgICAgICAgcmV0dXJuIHsgaW5zdGFuY2UsIG1vZHVsZSB9OwogICAgICAgIH0KICAgICAgICBlbHNlIHsKICAgICAgICAgICAgcmV0dXJuIGluc3RhbmNlOwogICAgICAgIH0KICAgIH0KICAgIGZ1bmN0aW9uIGV4cGVjdGVkUmVzcG9uc2VUeXBlKHR5cGUpIHsKICAgICAgICBzd2l0Y2ggKHR5cGUpIHsKICAgICAgICAgICAgY2FzZSAnYmFzaWMnOgogICAgICAgICAgICBjYXNlICdjb3JzJzoKICAgICAgICAgICAgY2FzZSAnZGVmYXVsdCc6IHJldHVybiB0cnVlOwogICAgICAgIH0KICAgICAgICByZXR1cm4gZmFsc2U7CiAgICB9Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIEluaXQ6Ci8vIF9fd2JnX2luaXQKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmFzeW5jIGZ1bmN0aW9uIF9fd2JnX2luaXQobW9kdWxlX29yX3BhdGgsIG1lbW9yeSkgewogICAgaWYgKHdhc20gIT09IHVuZGVmaW5lZCkKICAgICAgICByZXR1cm4gd2FzbTsKICAgIGxldCB0aHJlYWRfc3RhY2tfc2l6ZTsKICAgIGlmIChtb2R1bGVfb3JfcGF0aCAhPT0gdW5kZWZpbmVkKSB7CiAgICAgICAgaWYgKE9iamVjdC5nZXRQcm90b3R5cGVPZihtb2R1bGVfb3JfcGF0aCkgPT09IE9iamVjdC5wcm90b3R5cGUpIHsKICAgICAgICAgICAgKHsgbW9kdWxlX29yX3BhdGgsIG1lbW9yeSwgdGhyZWFkX3N0YWNrX3NpemUgfSA9IG1vZHVsZV9vcl9wYXRoKTsKICAgICAgICB9CiAgICAgICAgZWxzZSB7CiAgICAgICAgICAgIGNvbnNvbGUud2FybigndXNpbmcgZGVwcmVjYXRlZCBwYXJhbWV0ZXJzIGZvciB0aGUgaW5pdGlhbGl6YXRpb24gZnVuY3Rpb247IHBhc3MgYSBzaW5nbGUgb2JqZWN0IGluc3RlYWQnKTsKICAgICAgICB9CiAgICB9CiAgICAvLyAgIGlmIChtb2R1bGVfb3JfcGF0aCA9PT0gdW5kZWZpbmVkKSB7CiAgICAvLyAgICAgbW9kdWxlX29yX3BhdGggPSBuZXcgVVJMKCd0ZmhlX2JnLndhc20nLCBpbXBvcnQubWV0YS51cmwpOwogICAgLy8gICB9CiAgICBjb25zdCBpbXBvcnRzID0gX193YmdfZ2V0X2ltcG9ydHMobWVtb3J5KTsKICAgIC8vICAgaWYgKAogICAgLy8gICAgIHR5cGVvZiBtb2R1bGVfb3JfcGF0aCA9PT0gJ3N0cmluZycgfHwKICAgIC8vICAgICAodHlwZW9mIFJlcXVlc3QgPT09ICdmdW5jdGlvbicgJiYgbW9kdWxlX29yX3BhdGggaW5zdGFuY2VvZiBSZXF1ZXN0KSB8fAogICAgLy8gICAgICh0eXBlb2YgVVJMID09PSAnZnVuY3Rpb24nICYmIG1vZHVsZV9vcl9wYXRoIGluc3RhbmNlb2YgVVJMKQogICAgLy8gICApIHsKICAgIC8vICAgICBtb2R1bGVfb3JfcGF0aCA9IGZldGNoKG1vZHVsZV9vcl9wYXRoKTsKICAgIC8vICAgfQogICAgY29uc3QgeyBpbnN0YW5jZSwgbW9kdWxlIH0gPSBhd2FpdCBfX3diZ19sb2FkKGF3YWl0IG1vZHVsZV9vcl9wYXRoLCBpbXBvcnRzKTsKICAgIHJldHVybiBfX3diZ19maW5hbGl6ZV9pbml0KGluc3RhbmNlLCBtb2R1bGUsIHRocmVhZF9zdGFja19zaXplKTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8KLy8gVGhlICd0ZmhlJyBnbG9iYWwgb2JqZWN0Ci8vID09PT09PT09PT09PT09PT09PT09PT09PQovLyBGaW5hbCB0ZmhlIG9iamVjdCBnbG9iYWwgZGVjbGFyYXRpb24gY2FsbGVkIGJ5ICd3YWl0Rm9yTXNnVHlwZScgb25seQovLwovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKdmFyIHRmaGUgPSAvKiNfX1BVUkVfXyovIE9iamVjdC5mcmVlemUoewogIF9fcHJvdG9fXzogbnVsbCwKICBkZWZhdWx0OiBfX3diZ19pbml0LAogIHdiZ19yYXlvbl9zdGFydF93b3JrZXI6IHdiZ19yYXlvbl9zdGFydF93b3JrZXIsCn0pOwoK";
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
