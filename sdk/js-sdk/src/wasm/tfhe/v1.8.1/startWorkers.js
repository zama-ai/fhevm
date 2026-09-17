/**
 * Auto-generated from scripts/wasm/tfhe/startWorkers.template.js.
 * Embedded worker base64 payload SHA-256: f4aa3bb27cab87696db0916ebd888c382649ee9c432e31da93642591786f5ef0
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
 *  - Hash is the build-time constant "d67a2b2c52175bce3d32f0033b3e2fbc8044717728089b90945688c4270aaa6f".
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
const _workerUrlSha256 = "d67a2b2c52175bce3d32f0033b3e2fbc8044717728089b90945688c4270aaa6f";
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
 * 1. Fetch the URL and verify its SHA-256 against "d67a2b2c52175bce3d32f0033b3e2fbc8044717728089b90945688c4270aaa6f" — fails fast on mismatch.
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
  const workerBase64 = "ZnVuY3Rpb24gX19faXNCcm93c2VyTGlrZSgpIHsKICByZXR1cm4gKAogICAgdHlwZW9mIGFkZEV2ZW50TGlzdGVuZXIgPT09ICdmdW5jdGlvbicgJiYKICAgIHR5cGVvZiByZW1vdmVFdmVudExpc3RlbmVyID09PSAnZnVuY3Rpb24nCiAgKTsKfQoKYXN5bmMgZnVuY3Rpb24gX19fZ2V0VGFyZ2V0KCkgewogIGlmIChfX19pc0Jyb3dzZXJMaWtlKCkpIHJldHVybiBzZWxmOwogIGNvbnN0IG5vZGVNb2R1bGVOYW1lID0gJ3dvcmtlcl90aHJlYWRzJzsKICBjb25zdCBub2RlTW9kdWxlSWQgPSBgbm9kZToke25vZGVNb2R1bGVOYW1lfWA7CiAgY29uc3QgeyBwYXJlbnRQb3J0IH0gPSBhd2FpdCBpbXBvcnQoLyogQHZpdGUtaWdub3JlICovIG5vZGVNb2R1bGVJZCk7CiAgcmV0dXJuIHBhcmVudFBvcnQ7Cn0KCmZ1bmN0aW9uIF9fX3dhaXRGb3JNc2dUeXBlKHRhcmdldCwgdHlwZSkgewogIHJldHVybiBuZXcgUHJvbWlzZSgocmVzb2x2ZSkgPT4gewogICAgaWYgKHR5cGVvZiB0YXJnZXQub24gPT09ICdmdW5jdGlvbicpIHsKICAgICAgLy8gTm9kZTogRXZlbnRFbWl0dGVyLCBkYXRhIHBhc3NlZCBkaXJlY3RseQogICAgICB0YXJnZXQub24oJ21lc3NhZ2UnLCBmdW5jdGlvbiBvbk1zZyhkYXRhKSB7CiAgICAgICAgaWYgKGRhdGE/LnR5cGUgIT09IHR5cGUpIHJldHVybjsKICAgICAgICB0YXJnZXQub2ZmKCdtZXNzYWdlJywgb25Nc2cpOwogICAgICAgIHJlc29sdmUoZGF0YSk7CiAgICAgIH0pOwogICAgfSBlbHNlIHsKICAgICAgLy8gQnJvd3NlcjogRE9NIGV2ZW50cywgZGF0YSB3cmFwcGVkIGluIE1lc3NhZ2VFdmVudAogICAgICB0YXJnZXQuYWRkRXZlbnRMaXN0ZW5lcignbWVzc2FnZScsIGZ1bmN0aW9uIG9uTXNnKHsgZGF0YSB9KSB7CiAgICAgICAgaWYgKGRhdGE/LnR5cGUgIT09IHR5cGUpIHJldHVybjsKICAgICAgICB0YXJnZXQucmVtb3ZlRXZlbnRMaXN0ZW5lcignbWVzc2FnZScsIG9uTXNnKTsKICAgICAgICByZXNvbHZlKGRhdGEpOwogICAgICB9KTsKICAgIH0KICB9KTsKfQoKX19fZ2V0VGFyZ2V0KCkudGhlbigodGFyZ2V0KSA9PgogIF9fX3dhaXRGb3JNc2dUeXBlKHRhcmdldCwgJ3dhc21fYmluZGdlbl93b3JrZXJfaW5pdCcpLnRoZW4oCiAgICBhc3luYyAoeyBpbml0LCByZWNlaXZlciB9KSA9PiB7CiAgICAgIGNvbnN0IHBrZyA9IGF3YWl0IFByb21pc2UucmVzb2x2ZSgpLnRoZW4oZnVuY3Rpb24gKCkgewogICAgICAgIHJldHVybiB0ZmhlOwogICAgICB9KTsKICAgICAgYXdhaXQgcGtnLmRlZmF1bHQoaW5pdCk7CiAgICAgIHRhcmdldC5wb3N0TWVzc2FnZSh7IHR5cGU6ICd3YXNtX2JpbmRnZW5fd29ya2VyX3JlYWR5JyB9KTsKICAgICAgcGtnLndiZ19yYXlvbl9zdGFydF93b3JrZXIocmVjZWl2ZXIpOwogICAgfSwKICApLAopOwoKLyoqCiAqIEBwYXJhbSB7bnVtYmVyfSByZWNlaXZlcgogKi8KZnVuY3Rpb24gd2JnX3JheW9uX3N0YXJ0X3dvcmtlcihyZWNlaXZlcikgewogIHdhc20ud2JnX3JheW9uX3N0YXJ0X3dvcmtlcihyZWNlaXZlcik7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIEludGVybmFsIHdhc21iaW5kZ2VuIHRvb2xzCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLwovLyBJbXBvcnRzOgovLyBfX3diZ19nZXRfaW1wb3J0cwovLwovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gX193YmdfZ2V0X2ltcG9ydHMobWVtb3J5KSB7CiAgICBjb25zdCBpbXBvcnQwID0gewogICAgICAgIF9fcHJvdG9fXzogbnVsbCwKICAgICAgICBfX3diZ19CaWdJbnRfNjViY2VhMjUxZTc4ODA4MzogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gQmlnSW50KGFyZzApOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfQmlnSW50X2NjYmNhNzkzZjQ1NmU1ODI6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgcmV0dXJuIGhhbmRsZUVycm9yKGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgICAgICBjb25zdCByZXQgPSBCaWdJbnQoYXJnMCk7CiAgICAgICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgICAgICB9LCBhcmd1bWVudHMpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfRXJyb3JfOTYwYzE1NWQzZDQ5ZTRjMjogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gRXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGFyZzAsIGFyZzEpKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fYmlnaW50X2dldF9hc19pNjRfM2QzYWJhNWQ2MTZjNmE1MTogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgdiA9IGFyZzE7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiAodikgPT09ICdiaWdpbnQnID8gdiA6IHVuZGVmaW5lZDsKICAgICAgICAgICAgZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0QmlnSW50NjQoYXJnMCArIDggKiAxLCBpc0xpa2VOb25lKHJldCkgPyBCaWdJbnQoMCkgOiByZXQsIHRydWUpOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDAsICFpc0xpa2VOb25lKHJldCksIHRydWUpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9iaXRfYW5kXzk2ZjY5NmM1NmI0OTUwZDU6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAgJiBhcmcxOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9iaXRfb3JfZDU2NGMyNzUxZmRhYzZjOTogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMCB8IGFyZzE7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2Jvb2xlYW5fZ2V0XzZlYTE0OWYwYThkY2M1ZmY6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHYgPSBhcmcwOwogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgKHYpID09PSAnYm9vbGVhbicgPyB2IDogdW5kZWZpbmVkOwogICAgICAgICAgICByZXR1cm4gaXNMaWtlTm9uZShyZXQpID8gMHhGRkZGRkYgOiByZXQgPyAxIDogMDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fZGVidWdfc3RyaW5nX2FiNGIzNGQyM2Q2Nzc4YmQ6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGRlYnVnU3RyaW5nKGFyZzEpOwogICAgICAgICAgICBjb25zdCBwdHIxID0gcGFzc1N0cmluZ1RvV2FzbTAocmV0LCB3YXNtLl9fd2JpbmRnZW5fbWFsbG9jLCB3YXNtLl9fd2JpbmRnZW5fcmVhbGxvYyk7CiAgICAgICAgICAgIGNvbnN0IGxlbjEgPSBXQVNNX1ZFQ1RPUl9MRU47CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMSwgbGVuMSwgdHJ1ZSk7CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMCwgcHRyMSwgdHJ1ZSk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2lzX2Z1bmN0aW9uXzNiYWE5ZGIxYTk4N2Y0N2Q6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiAoYXJnMCkgPT09ICdmdW5jdGlvbic7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2lzX29iamVjdF82MzMyMmVjMGNkNmVhNGVmOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCB2YWwgPSBhcmcwOwogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgKHZhbCkgPT09ICdvYmplY3QnICYmIHZhbCAhPT0gbnVsbDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5faXNfc3RyaW5nXzZkZjNiZjdlZjExNjRlZDM6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiAoYXJnMCkgPT09ICdzdHJpbmcnOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9pc191bmRlZmluZWRfMjlhNDNiNGQ0MjkyMGFiZDogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMCA9PT0gdW5kZWZpbmVkOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9qc3ZhbF9lcV9kMzQ2NWQ4YTA3Njk3MjI4OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwID09PSBhcmcxOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9sdF83OGJhYjM4MjYyOGZiNDhmOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwIDwgYXJnMTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fbWVtb3J5X2RmYTEyMDk2ZjQwMGM5YmQ6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gd2FzbS5tZW1vcnk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX21vZHVsZV9iNWU2ZmI5NWRiZGI3ZDdlOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHdhc21Nb2R1bGU7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX25lZ184ZDM5ZDIzZWY2NWM5ZmRiOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSAtYXJnMDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fc2hsX2M3ZWE0MTczMzg3YTE2Njk6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAgPDwgYXJnMTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fc2hyXzQzNjU1M2NiYWVmNDFhNjY6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAgPj4gYXJnMTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fc3RyaW5nX2dldF83ZWQ1MzIyOTkxY2FhZWM1OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCBvYmogPSBhcmcxOwogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgKG9iaikgPT09ICdzdHJpbmcnID8gb2JqIDogdW5kZWZpbmVkOwogICAgICAgICAgICB2YXIgcHRyMSA9IGlzTGlrZU5vbmUocmV0KSA/IDAgOiBwYXNzU3RyaW5nVG9XYXNtMChyZXQsIHdhc20uX193YmluZGdlbl9tYWxsb2MsIHdhc20uX193YmluZGdlbl9yZWFsbG9jKTsKICAgICAgICAgICAgdmFyIGxlbjEgPSBXQVNNX1ZFQ1RPUl9MRU47CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMSwgbGVuMSwgdHJ1ZSk7CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMCwgcHRyMSwgdHJ1ZSk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX3Rocm93XzZiNjQ0NDliOWI5ZWQzM2M6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIHRocm93IG5ldyBFcnJvcihnZXRTdHJpbmdGcm9tV2FzbTAoYXJnMCwgYXJnMSkpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfY2FsbF9hMjQ1OTJhNmYzNDlhOTdlOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIHJldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbiAoYXJnMCwgYXJnMSwgYXJnMikgewogICAgICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5jYWxsKGFyZzEsIGFyZzIpOwogICAgICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICAgICAgfSwgYXJndW1lbnRzKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX2NyeXB0b18zOGRmMmJhYjEyNmI2M2RjOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLmNyeXB0bzsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX2Vycm9yX2E2ZmEyMDJiNThhYTFjZDM6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGxldCBkZWZlcnJlZDBfMDsKICAgICAgICAgICAgbGV0IGRlZmVycmVkMF8xOwogICAgICAgICAgICB0cnkgewogICAgICAgICAgICAgICAgZGVmZXJyZWQwXzAgPSBhcmcwOwogICAgICAgICAgICAgICAgZGVmZXJyZWQwXzEgPSBhcmcxOwogICAgICAgICAgICAgICAgY29uc29sZS5lcnJvcihnZXRTdHJpbmdGcm9tV2FzbTAoYXJnMCwgYXJnMSkpOwogICAgICAgICAgICB9CiAgICAgICAgICAgIGZpbmFsbHkgewogICAgICAgICAgICAgICAgd2FzbS5fX3diaW5kZ2VuX2ZyZWUoZGVmZXJyZWQwXzAsIGRlZmVycmVkMF8xLCAxKTsKICAgICAgICAgICAgfQogICAgICAgIH0sCiAgICAgICAgX193YmdfZ2V0UmFuZG9tVmFsdWVzX2M0NGE1MGQ4Y2ZkYWViZWI6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgcmV0dXJuIGhhbmRsZUVycm9yKGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgICAgICBhcmcwLmdldFJhbmRvbVZhbHVlcyhhcmcxKTsKICAgICAgICAgICAgfSwgYXJndW1lbnRzKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX2luc3RhbmNlb2ZfV2luZG93X2NjNjRjODZjOGVmOWUwMmI6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGxldCByZXN1bHQ7CiAgICAgICAgICAgIHRyeSB7CiAgICAgICAgICAgICAgICByZXN1bHQgPSBhcmcwIGluc3RhbmNlb2YgV2luZG93OwogICAgICAgICAgICB9CiAgICAgICAgICAgIGNhdGNoIChfKSB7CiAgICAgICAgICAgICAgICByZXN1bHQgPSBmYWxzZTsKICAgICAgICAgICAgfQogICAgICAgICAgICBjb25zdCByZXQgPSByZXN1bHQ7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19sZW5ndGhfOWYxNzc1MjI0Y2YxZDgxNTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5sZW5ndGg7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19tc0NyeXB0b19iZDVhMDM0YWY5NmJjYmE2OiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLm1zQ3J5cHRvOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfbmV3XzIyN2Q3YzA1NDE0ZWI4NjE6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gbmV3IEVycm9yKCk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19uZXdfd2l0aF9sZW5ndGhfOGM4NTRlNDFlYTRkYWU5YjogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gbmV3IFVpbnQ4QXJyYXkoYXJnMCA+Pj4gMCk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19ub2RlXzg0ZWE4NzU0MTEyNTRkYjE6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAubm9kZTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3Byb2Nlc3NfNDRjN2ExNGUxMWU5ZjY5ZTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5wcm9jZXNzOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfcHJvdG90eXBlc2V0Y2FsbF9hNmIwMmViMDBiMGY0Y2UyOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSwgYXJnMikgewogICAgICAgICAgICBVaW50OEFycmF5LnByb3RvdHlwZS5zZXQuY2FsbChnZXRBcnJheVU4RnJvbVdhc20wKGFyZzAsIGFyZzEpLCBhcmcyKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3JhbmRvbUZpbGxTeW5jXzZjMjVlYWM5ODY5ZWI1M2M6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgcmV0dXJuIGhhbmRsZUVycm9yKGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgICAgICBhcmcwLnJhbmRvbUZpbGxTeW5jKGFyZzEpOwogICAgICAgICAgICB9LCBhcmd1bWVudHMpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfcmVxdWlyZV9iNGVkYmRjZjNlMmExZWYwOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIHJldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgICAgICBjb25zdCByZXQgPSBtb2R1bGUucmVxdWlyZTsKICAgICAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgICAgIH0sIGFyZ3VtZW50cyk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdGFja18zYjBkOTc0YmJmMzFlNDRmOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcxLnN0YWNrOwogICAgICAgICAgICBjb25zdCBwdHIxID0gcGFzc1N0cmluZ1RvV2FzbTAocmV0LCB3YXNtLl9fd2JpbmRnZW5fbWFsbG9jLCB3YXNtLl9fd2JpbmRnZW5fcmVhbGxvYyk7CiAgICAgICAgICAgIGNvbnN0IGxlbjEgPSBXQVNNX1ZFQ1RPUl9MRU47CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMSwgbGVuMSwgdHJ1ZSk7CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMCwgcHRyMSwgdHJ1ZSk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdGFydFdvcmtlcnNfOGI1ODJkNTdlOTJiZDJkNDogZnVuY3Rpb24gKGFyZzAsIGFyZzEsIGFyZzIpIHsKICAgICAgICAgICAgaGFuZGxlRXJyb3IoZnVuY3Rpb24gKCkgewogICAgICAgICAgICAgICAgdGhyb3cgbmV3IEVycm9yKCdzdGFydFdvcmtlcnMgbm90IHN1cHBvcnRlZCBmcm9tIGEgd29ya2VyIHRocmVhZCcpOwogICAgICAgICAgICB9KTsKICAgICAgICAgICAgLy8gY29uc3QgcmV0ID0gc3RhcnRXb3JrZXJzKGFyZzAsIGFyZzEsIHdiZ19yYXlvbl9Qb29sQnVpbGRlci5fX3dyYXAoYXJnMikpOwogICAgICAgICAgICAvLyByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3RhdGljX2FjY2Vzc29yX0dMT0JBTF84Y2ZhZGM4N2EyOTdjYTAyOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiBnbG9iYWwgPT09ICd1bmRlZmluZWQnID8gbnVsbCA6IGdsb2JhbDsKICAgICAgICAgICAgcmV0dXJuIGlzTGlrZU5vbmUocmV0KSA/IDAgOiBhZGRUb0V4dGVybnJlZlRhYmxlMChyZXQpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3RhdGljX2FjY2Vzc29yX0dMT0JBTF9USElTXzYwMjI1NmFlNWM4ZjQyY2Y6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mIGdsb2JhbFRoaXMgPT09ICd1bmRlZmluZWQnID8gbnVsbCA6IGdsb2JhbFRoaXM7CiAgICAgICAgICAgIHJldHVybiBpc0xpa2VOb25lKHJldCkgPyAwIDogYWRkVG9FeHRlcm5yZWZUYWJsZTAocmV0KTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3N0YXRpY19hY2Nlc3Nvcl9TRUxGX2U0NDVjMWM3NDg0YWVjYzM6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mIHNlbGYgPT09ICd1bmRlZmluZWQnID8gbnVsbCA6IHNlbGY7CiAgICAgICAgICAgIHJldHVybiBpc0xpa2VOb25lKHJldCkgPyAwIDogYWRkVG9FeHRlcm5yZWZUYWJsZTAocmV0KTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3N0YXRpY19hY2Nlc3Nvcl9XSU5ET1dfZjIwZTg1NzZlZjFlMGYxNzogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2Ygd2luZG93ID09PSAndW5kZWZpbmVkJyA/IG51bGwgOiB3aW5kb3c7CiAgICAgICAgICAgIHJldHVybiBpc0xpa2VOb25lKHJldCkgPyAwIDogYWRkVG9FeHRlcm5yZWZUYWJsZTAocmV0KTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3N1YmFycmF5X2Y4Y2E0NmEyNWIxZjVlMGQ6IGZ1bmN0aW9uIChhcmcwLCBhcmcxLCBhcmcyKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAuc3ViYXJyYXkoYXJnMSA+Pj4gMCwgYXJnMiA+Pj4gMCk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ190b1N0cmluZ182ZGMxYTk0ZTBiZGJhMzc4OiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLnRvU3RyaW5nKCk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ190b1N0cmluZ19jOTZkYzc2ZDU1NDdhNzE1OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSwgYXJnMikgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcxLnRvU3RyaW5nKGFyZzIpOwogICAgICAgICAgICBjb25zdCBwdHIxID0gcGFzc1N0cmluZ1RvV2FzbTAocmV0LCB3YXNtLl9fd2JpbmRnZW5fbWFsbG9jLCB3YXNtLl9fd2JpbmRnZW5fcmVhbGxvYyk7CiAgICAgICAgICAgIGNvbnN0IGxlbjEgPSBXQVNNX1ZFQ1RPUl9MRU47CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMSwgbGVuMSwgdHJ1ZSk7CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMCwgcHRyMSwgdHJ1ZSk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ192ZXJzaW9uc18yNzZiMjc5NWIxYzZhMjE5OiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLnZlcnNpb25zOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDE6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIC8vIENhc3QgaW50cmluc2ljIGZvciBgRjY0IC0+IEV4dGVybnJlZmAuCiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzA7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diaW5kZ2VuX2Nhc3RfMDAwMDAwMDAwMDAwMDAwMjogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgLy8gQ2FzdCBpbnRyaW5zaWMgZm9yIGBJNjQgLT4gRXh0ZXJucmVmYC4KICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDAzOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICAvLyBDYXN0IGludHJpbnNpYyBmb3IgYFJlZihTbGljZShVOCkpIC0+IE5hbWVkRXh0ZXJucmVmKCJVaW50OEFycmF5IilgLgogICAgICAgICAgICBjb25zdCByZXQgPSBnZXRBcnJheVU4RnJvbVdhc20wKGFyZzAsIGFyZzEpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDQ6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIC8vIENhc3QgaW50cmluc2ljIGZvciBgUmVmKFN0cmluZykgLT4gRXh0ZXJucmVmYC4KICAgICAgICAgICAgY29uc3QgcmV0ID0gZ2V0U3RyaW5nRnJvbVdhc20wKGFyZzAsIGFyZzEpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDU6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIC8vIENhc3QgaW50cmluc2ljIGZvciBgVTY0IC0+IEV4dGVybnJlZmAuCiAgICAgICAgICAgIGNvbnN0IHJldCA9IEJpZ0ludC5hc1VpbnROKDY0LCBhcmcwKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5faW5pdF9leHRlcm5yZWZfdGFibGU6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgdGFibGUgPSB3YXNtLl9fd2JpbmRnZW5fZXh0ZXJucmVmczsKICAgICAgICAgICAgY29uc3Qgb2Zmc2V0ID0gdGFibGUuZ3Jvdyg0KTsKICAgICAgICAgICAgdGFibGUuc2V0KDAsIHVuZGVmaW5lZCk7CiAgICAgICAgICAgIHRhYmxlLnNldChvZmZzZXQgKyAwLCB1bmRlZmluZWQpOwogICAgICAgICAgICB0YWJsZS5zZXQob2Zmc2V0ICsgMSwgbnVsbCk7CiAgICAgICAgICAgIHRhYmxlLnNldChvZmZzZXQgKyAyLCB0cnVlKTsKICAgICAgICAgICAgdGFibGUuc2V0KG9mZnNldCArIDMsIGZhbHNlKTsKICAgICAgICB9LAogICAgICAgIG1lbW9yeTogbWVtb3J5IHx8IG5ldyBXZWJBc3NlbWJseS5NZW1vcnkoeyBpbml0aWFsOiAxOSwgbWF4aW11bTogMTYzODQsIHNoYXJlZDogdHJ1ZSB9KSwKICAgIH07CiAgICByZXR1cm4gewogICAgICAgIF9fcHJvdG9fXzogbnVsbCwKICAgICAgICAiLi90ZmhlX2JnLmpzIjogaW1wb3J0MCwKICAgIH07Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGFkZFRvRXh0ZXJucmVmVGFibGUwCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBhZGRUb0V4dGVybnJlZlRhYmxlMChvYmopIHsKICAgIGNvbnN0IGlkeCA9IHdhc20uX19leHRlcm5yZWZfdGFibGVfYWxsb2MoKTsKICAgIHdhc20uX193YmluZGdlbl9leHRlcm5yZWZzLnNldChpZHgsIG9iaik7CiAgICByZXR1cm4gaWR4Owp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBkZWJ1Z1N0cmluZwovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gZGVidWdTdHJpbmcodmFsKSB7CiAgICAvLyBwcmltaXRpdmUgdHlwZXMKICAgIGNvbnN0IHR5cGUgPSB0eXBlb2YgdmFsOwogICAgaWYgKHR5cGUgPT0gJ251bWJlcicgfHwgdHlwZSA9PSAnYm9vbGVhbicgfHwgdmFsID09IG51bGwpIHsKICAgICAgICByZXR1cm4gYCR7dmFsfWA7CiAgICB9CiAgICBpZiAodHlwZSA9PSAnc3RyaW5nJykgewogICAgICAgIHJldHVybiBgIiR7dmFsfSJgOwogICAgfQogICAgaWYgKHR5cGUgPT0gJ3N5bWJvbCcpIHsKICAgICAgICBjb25zdCBkZXNjcmlwdGlvbiA9IHZhbC5kZXNjcmlwdGlvbjsKICAgICAgICBpZiAoZGVzY3JpcHRpb24gPT0gbnVsbCkgewogICAgICAgICAgICByZXR1cm4gJ1N5bWJvbCc7CiAgICAgICAgfQogICAgICAgIGVsc2UgewogICAgICAgICAgICByZXR1cm4gYFN5bWJvbCgke2Rlc2NyaXB0aW9ufSlgOwogICAgICAgIH0KICAgIH0KICAgIGlmICh0eXBlID09ICdmdW5jdGlvbicpIHsKICAgICAgICBjb25zdCBuYW1lID0gdmFsLm5hbWU7CiAgICAgICAgaWYgKHR5cGVvZiBuYW1lID09ICdzdHJpbmcnICYmIG5hbWUubGVuZ3RoID4gMCkgewogICAgICAgICAgICByZXR1cm4gYEZ1bmN0aW9uKCR7bmFtZX0pYDsKICAgICAgICB9CiAgICAgICAgZWxzZSB7CiAgICAgICAgICAgIHJldHVybiAnRnVuY3Rpb24nOwogICAgICAgIH0KICAgIH0KICAgIC8vIG9iamVjdHMKICAgIGlmIChBcnJheS5pc0FycmF5KHZhbCkpIHsKICAgICAgICBjb25zdCBsZW5ndGggPSB2YWwubGVuZ3RoOwogICAgICAgIGxldCBkZWJ1ZyA9ICdbJzsKICAgICAgICBpZiAobGVuZ3RoID4gMCkgewogICAgICAgICAgICBkZWJ1ZyArPSBkZWJ1Z1N0cmluZyh2YWxbMF0pOwogICAgICAgIH0KICAgICAgICBmb3IgKGxldCBpID0gMTsgaSA8IGxlbmd0aDsgaSsrKSB7CiAgICAgICAgICAgIGRlYnVnICs9ICcsICcgKyBkZWJ1Z1N0cmluZyh2YWxbaV0pOwogICAgICAgIH0KICAgICAgICBkZWJ1ZyArPSAnXSc7CiAgICAgICAgcmV0dXJuIGRlYnVnOwogICAgfQogICAgLy8gVGVzdCBmb3IgYnVpbHQtaW4KICAgIGNvbnN0IGJ1aWx0SW5NYXRjaGVzID0gL1xbb2JqZWN0IChbXlxdXSspXF0vLmV4ZWModG9TdHJpbmcuY2FsbCh2YWwpKTsKICAgIGxldCBjbGFzc05hbWU7CiAgICBpZiAoYnVpbHRJbk1hdGNoZXMgJiYgYnVpbHRJbk1hdGNoZXMubGVuZ3RoID4gMSkgewogICAgICAgIGNsYXNzTmFtZSA9IGJ1aWx0SW5NYXRjaGVzWzFdOwogICAgfQogICAgZWxzZSB7CiAgICAgICAgLy8gRmFpbGVkIHRvIG1hdGNoIHRoZSBzdGFuZGFyZCAnW29iamVjdCBDbGFzc05hbWVdJwogICAgICAgIHJldHVybiB0b1N0cmluZy5jYWxsKHZhbCk7CiAgICB9CiAgICBpZiAoY2xhc3NOYW1lID09ICdPYmplY3QnKSB7CiAgICAgICAgLy8gd2UncmUgYSB1c2VyIGRlZmluZWQgY2xhc3Mgb3IgT2JqZWN0CiAgICAgICAgLy8gSlNPTi5zdHJpbmdpZnkgYXZvaWRzIHByb2JsZW1zIHdpdGggY3ljbGVzLCBhbmQgaXMgZ2VuZXJhbGx5IG11Y2gKICAgICAgICAvLyBlYXNpZXIgdGhhbiBsb29waW5nIHRocm91Z2ggb3duUHJvcGVydGllcyBvZiBgdmFsYC4KICAgICAgICB0cnkgewogICAgICAgICAgICByZXR1cm4gJ09iamVjdCgnICsgSlNPTi5zdHJpbmdpZnkodmFsKSArICcpJzsKICAgICAgICB9CiAgICAgICAgY2F0Y2ggKF8pIHsKICAgICAgICAgICAgcmV0dXJuICdPYmplY3QnOwogICAgICAgIH0KICAgIH0KICAgIC8vIGVycm9ycwogICAgaWYgKHZhbCBpbnN0YW5jZW9mIEVycm9yKSB7CiAgICAgICAgcmV0dXJuIGAke3ZhbC5uYW1lfTogJHt2YWwubWVzc2FnZX1cbiR7dmFsLnN0YWNrfWA7CiAgICB9CiAgICAvLyBUT0RPIHdlIGNvdWxkIHRlc3QgZm9yIG1vcmUgdGhpbmdzIGhlcmUsIGxpa2UgYFNldGBzIGFuZCBgTWFwYHMuCiAgICByZXR1cm4gY2xhc3NOYW1lOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBnZXRBcnJheVU4RnJvbVdhc20wCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBnZXRBcnJheVU4RnJvbVdhc20wKHB0ciwgbGVuKSB7CiAgICBwdHIgPSBwdHIgPj4+IDA7CiAgICByZXR1cm4gZ2V0VWludDhBcnJheU1lbW9yeTAoKS5zdWJhcnJheShwdHIgLyAxLCBwdHIgLyAxICsgbGVuKTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gY2FjaGVkRGF0YVZpZXdNZW1vcnkwCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpsZXQgY2FjaGVkRGF0YVZpZXdNZW1vcnkwID0gbnVsbDsKCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGdldERhdGFWaWV3TWVtb3J5MAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gZ2V0RGF0YVZpZXdNZW1vcnkwKCkgewogICAgaWYgKGNhY2hlZERhdGFWaWV3TWVtb3J5MCA9PT0gbnVsbCB8fCBjYWNoZWREYXRhVmlld01lbW9yeTAuYnVmZmVyICE9PSB3YXNtLm1lbW9yeS5idWZmZXIpIHsKICAgICAgICBjYWNoZWREYXRhVmlld01lbW9yeTAgPSBuZXcgRGF0YVZpZXcod2FzbS5tZW1vcnkuYnVmZmVyKTsKICAgIH0KICAgIHJldHVybiBjYWNoZWREYXRhVmlld01lbW9yeTA7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGdldFN0cmluZ0Zyb21XYXNtMAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gZ2V0U3RyaW5nRnJvbVdhc20wKHB0ciwgbGVuKSB7CiAgICBwdHIgPSBwdHIgPj4+IDA7CiAgICByZXR1cm4gZGVjb2RlVGV4dChwdHIsIGxlbik7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpsZXQgY2FjaGVkVWludDhBcnJheU1lbW9yeTAgPSBudWxsOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gZ2V0VWludDhBcnJheU1lbW9yeTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGdldFVpbnQ4QXJyYXlNZW1vcnkwKCkgewogICAgaWYgKGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwID09PSBudWxsIHx8IGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwLmJ1ZmZlciAhPT0gd2FzbS5tZW1vcnkuYnVmZmVyKSB7CiAgICAgICAgY2FjaGVkVWludDhBcnJheU1lbW9yeTAgPSBuZXcgVWludDhBcnJheSh3YXNtLm1lbW9yeS5idWZmZXIpOwogICAgfQogICAgcmV0dXJuIGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBoYW5kbGVFcnJvcgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gaGFuZGxlRXJyb3IoZiwgYXJncykgewogICAgdHJ5IHsKICAgICAgICByZXR1cm4gZi5hcHBseSh0aGlzLCBhcmdzKTsKICAgIH0KICAgIGNhdGNoIChlKSB7CiAgICAgICAgY29uc3QgaWR4ID0gYWRkVG9FeHRlcm5yZWZUYWJsZTAoZSk7CiAgICAgICAgd2FzbS5fX3diaW5kZ2VuX2V4bl9zdG9yZShpZHgpOwogICAgfQp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBpc0xpa2VOb25lCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBpc0xpa2VOb25lKHgpIHsKICAgIHJldHVybiB4ID09PSB1bmRlZmluZWQgfHwgeCA9PT0gbnVsbDsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gcGFzc1N0cmluZ1RvV2FzbTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIHBhc3NTdHJpbmdUb1dhc20wKGFyZywgbWFsbG9jLCByZWFsbG9jKSB7CiAgICBpZiAocmVhbGxvYyA9PT0gdW5kZWZpbmVkKSB7CiAgICAgICAgY29uc3QgYnVmID0gY2FjaGVkVGV4dEVuY29kZXIuZW5jb2RlKGFyZyk7CiAgICAgICAgY29uc3QgcHRyID0gbWFsbG9jKGJ1Zi5sZW5ndGgsIDEpID4+PiAwOwogICAgICAgIGdldFVpbnQ4QXJyYXlNZW1vcnkwKCkuc3ViYXJyYXkocHRyLCBwdHIgKyBidWYubGVuZ3RoKS5zZXQoYnVmKTsKICAgICAgICBXQVNNX1ZFQ1RPUl9MRU4gPSBidWYubGVuZ3RoOwogICAgICAgIHJldHVybiBwdHI7CiAgICB9CiAgICBsZXQgbGVuID0gYXJnLmxlbmd0aDsKICAgIGxldCBwdHIgPSBtYWxsb2MobGVuLCAxKSA+Pj4gMDsKICAgIGNvbnN0IG1lbSA9IGdldFVpbnQ4QXJyYXlNZW1vcnkwKCk7CiAgICBsZXQgb2Zmc2V0ID0gMDsKICAgIGZvciAoOyBvZmZzZXQgPCBsZW47IG9mZnNldCsrKSB7CiAgICAgICAgY29uc3QgY29kZSA9IGFyZy5jaGFyQ29kZUF0KG9mZnNldCk7CiAgICAgICAgaWYgKGNvZGUgPiAweDdGKQogICAgICAgICAgICBicmVhazsKICAgICAgICBtZW1bcHRyICsgb2Zmc2V0XSA9IGNvZGU7CiAgICB9CiAgICBpZiAob2Zmc2V0ICE9PSBsZW4pIHsKICAgICAgICBpZiAob2Zmc2V0ICE9PSAwKSB7CiAgICAgICAgICAgIGFyZyA9IGFyZy5zbGljZShvZmZzZXQpOwogICAgICAgIH0KICAgICAgICBwdHIgPSByZWFsbG9jKHB0ciwgbGVuLCBsZW4gPSBvZmZzZXQgKyBhcmcubGVuZ3RoICogMywgMSkgPj4+IDA7CiAgICAgICAgY29uc3QgdmlldyA9IGdldFVpbnQ4QXJyYXlNZW1vcnkwKCkuc3ViYXJyYXkocHRyICsgb2Zmc2V0LCBwdHIgKyBsZW4pOwogICAgICAgIGNvbnN0IHJldCA9IGNhY2hlZFRleHRFbmNvZGVyLmVuY29kZUludG8oYXJnLCB2aWV3KTsKICAgICAgICBvZmZzZXQgKz0gcmV0LndyaXR0ZW47CiAgICAgICAgcHRyID0gcmVhbGxvYyhwdHIsIGxlbiwgb2Zmc2V0LCAxKSA+Pj4gMDsKICAgIH0KICAgIFdBU01fVkVDVE9SX0xFTiA9IG9mZnNldDsKICAgIHJldHVybiBwdHI7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGNhY2hlZFRleHREZWNvZGVyCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpsZXQgY2FjaGVkVGV4dERlY29kZXIgPSAodHlwZW9mIFRleHREZWNvZGVyICE9PSAndW5kZWZpbmVkJyA/IG5ldyBUZXh0RGVjb2RlcigndXRmLTgnLCB7IGlnbm9yZUJPTTogdHJ1ZSwgZmF0YWw6IHRydWUgfSkgOiB1bmRlZmluZWQpOwoKaWYgKGNhY2hlZFRleHREZWNvZGVyKQogICAgY2FjaGVkVGV4dERlY29kZXIuZGVjb2RlKCk7CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBNQVhfU0FGQVJJX0RFQ09ERV9CWVRFUwovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKY29uc3QgTUFYX1NBRkFSSV9ERUNPREVfQllURVMgPSAyMTQ2NDM1MDcyOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gbnVtQnl0ZXNEZWNvZGVkCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpsZXQgbnVtQnl0ZXNEZWNvZGVkID0gMDsKCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGRlY29kZVRleHQKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGRlY29kZVRleHQocHRyLCBsZW4pIHsKICAgIG51bUJ5dGVzRGVjb2RlZCArPSBsZW47CiAgICBpZiAobnVtQnl0ZXNEZWNvZGVkID49IE1BWF9TQUZBUklfREVDT0RFX0JZVEVTKSB7CiAgICAgICAgY2FjaGVkVGV4dERlY29kZXIgPSBuZXcgVGV4dERlY29kZXIoJ3V0Zi04JywgeyBpZ25vcmVCT006IHRydWUsIGZhdGFsOiB0cnVlIH0pOwogICAgICAgIGNhY2hlZFRleHREZWNvZGVyLmRlY29kZSgpOwogICAgICAgIG51bUJ5dGVzRGVjb2RlZCA9IGxlbjsKICAgIH0KICAgIHJldHVybiBjYWNoZWRUZXh0RGVjb2Rlci5kZWNvZGUoZ2V0VWludDhBcnJheU1lbW9yeTAoKS5zbGljZShwdHIsIHB0ciArIGxlbikpOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBjYWNoZWRUZXh0RW5jb2RlcgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKY29uc3QgY2FjaGVkVGV4dEVuY29kZXIgPSAodHlwZW9mIFRleHRFbmNvZGVyICE9PSAndW5kZWZpbmVkJyA/IG5ldyBUZXh0RW5jb2RlcigpIDogdW5kZWZpbmVkKTsKCmlmIChjYWNoZWRUZXh0RW5jb2RlcikgewogICAgY2FjaGVkVGV4dEVuY29kZXIuZW5jb2RlSW50byA9IGZ1bmN0aW9uIChhcmcsIHZpZXcpIHsKICAgICAgICBjb25zdCBidWYgPSBjYWNoZWRUZXh0RW5jb2Rlci5lbmNvZGUoYXJnKTsKICAgICAgICB2aWV3LnNldChidWYpOwogICAgICAgIHJldHVybiB7CiAgICAgICAgICAgIHJlYWQ6IGFyZy5sZW5ndGgsCiAgICAgICAgICAgIHdyaXR0ZW46IGJ1Zi5sZW5ndGgKICAgICAgICB9OwogICAgfTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gV0FTTV9WRUNUT1JfTEVOIGlzIGEgbW9kdWxlLWxldmVsIHZhcmlhYmxlIHRoYXQgc3RvcmVzIHRoZSBieXRlIGxlbmd0aCBvZgovLyB0aGUgZGF0YSBqdXN0IHdyaXR0ZW4gaW50byBXQVNNIG1lbW9yeS4gSXQgYWN0cyBhcyBhbiBvdXQtcGFyYW1ldGVyLgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IFdBU01fVkVDVE9SX0xFTiA9IDA7CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBXQVNNIG1vZHVsZSBzdGF0ZQovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IHdhc21Nb2R1bGUsIHdhc207CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBJbml0OgovLyBfX3diZ19maW5hbGl6ZV9pbml0Ci8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBfX3diZ19maW5hbGl6ZV9pbml0KGluc3RhbmNlLCBtb2R1bGUsIHRocmVhZF9zdGFja19zaXplKSB7CiAgICB3YXNtID0gaW5zdGFuY2UuZXhwb3J0czsKICAgIHdhc21Nb2R1bGUgPSBtb2R1bGU7CiAgICBjYWNoZWREYXRhVmlld01lbW9yeTAgPSBudWxsOwogICAgY2FjaGVkVWludDhBcnJheU1lbW9yeTAgPSBudWxsOwogICAgaWYgKHR5cGVvZiB0aHJlYWRfc3RhY2tfc2l6ZSAhPT0gJ3VuZGVmaW5lZCcgJiYgKHR5cGVvZiB0aHJlYWRfc3RhY2tfc2l6ZSAhPT0gJ251bWJlcicgfHwgdGhyZWFkX3N0YWNrX3NpemUgPT09IDAgfHwgdGhyZWFkX3N0YWNrX3NpemUgJSA2NTUzNiAhPT0gMCkpIHsKICAgICAgICB0aHJvdyBuZXcgRXJyb3IoJ2ludmFsaWQgc3RhY2sgc2l6ZScpOwogICAgfQogICAgd2FzbS5fX3diaW5kZ2VuX3N0YXJ0KHRocmVhZF9zdGFja19zaXplKTsKICAgIHJldHVybiB3YXNtOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBJbml0OgovLyBfX3diZ19sb2FkCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgphc3luYyBmdW5jdGlvbiBfX3diZ19sb2FkKG1vZHVsZSwgaW1wb3J0cykgewogICAgaWYgKHR5cGVvZiBSZXNwb25zZSA9PT0gJ2Z1bmN0aW9uJyAmJiBtb2R1bGUgaW5zdGFuY2VvZiBSZXNwb25zZSkgewogICAgICAgIGlmICh0eXBlb2YgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGVTdHJlYW1pbmcgPT09ICdmdW5jdGlvbicpIHsKICAgICAgICAgICAgdHJ5IHsKICAgICAgICAgICAgICAgIHJldHVybiBhd2FpdCBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZVN0cmVhbWluZyhtb2R1bGUsIGltcG9ydHMpOwogICAgICAgICAgICB9CiAgICAgICAgICAgIGNhdGNoIChlKSB7CiAgICAgICAgICAgICAgICBjb25zdCB2YWxpZFJlc3BvbnNlID0gbW9kdWxlLm9rICYmIGV4cGVjdGVkUmVzcG9uc2VUeXBlKG1vZHVsZS50eXBlKTsKICAgICAgICAgICAgICAgIGlmICh2YWxpZFJlc3BvbnNlICYmIG1vZHVsZS5oZWFkZXJzLmdldCgnQ29udGVudC1UeXBlJykgIT09ICdhcHBsaWNhdGlvbi93YXNtJykgewogICAgICAgICAgICAgICAgICAgIGNvbnNvbGUud2FybigiYFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlU3RyZWFtaW5nYCBmYWlsZWQgYmVjYXVzZSB5b3VyIHNlcnZlciBkb2VzIG5vdCBzZXJ2ZSBXYXNtIHdpdGggYGFwcGxpY2F0aW9uL3dhc21gIE1JTUUgdHlwZS4gRmFsbGluZyBiYWNrIHRvIGBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZWAgd2hpY2ggaXMgc2xvd2VyLiBPcmlnaW5hbCBlcnJvcjpcbiIsIGUpOwogICAgICAgICAgICAgICAgfQogICAgICAgICAgICAgICAgZWxzZSB7CiAgICAgICAgICAgICAgICAgICAgdGhyb3cgZTsKICAgICAgICAgICAgICAgIH0KICAgICAgICAgICAgfQogICAgICAgIH0KICAgICAgICBjb25zdCBieXRlcyA9IGF3YWl0IG1vZHVsZS5hcnJheUJ1ZmZlcigpOwogICAgICAgIHJldHVybiBhd2FpdCBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZShieXRlcywgaW1wb3J0cyk7CiAgICB9CiAgICBlbHNlIHsKICAgICAgICBjb25zdCBpbnN0YW5jZSA9IGF3YWl0IFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlKG1vZHVsZSwgaW1wb3J0cyk7CiAgICAgICAgaWYgKGluc3RhbmNlIGluc3RhbmNlb2YgV2ViQXNzZW1ibHkuSW5zdGFuY2UpIHsKICAgICAgICAgICAgcmV0dXJuIHsgaW5zdGFuY2UsIG1vZHVsZSB9OwogICAgICAgIH0KICAgICAgICBlbHNlIHsKICAgICAgICAgICAgcmV0dXJuIGluc3RhbmNlOwogICAgICAgIH0KICAgIH0KICAgIGZ1bmN0aW9uIGV4cGVjdGVkUmVzcG9uc2VUeXBlKHR5cGUpIHsKICAgICAgICBzd2l0Y2ggKHR5cGUpIHsKICAgICAgICAgICAgY2FzZSAnYmFzaWMnOgogICAgICAgICAgICBjYXNlICdjb3JzJzoKICAgICAgICAgICAgY2FzZSAnZGVmYXVsdCc6IHJldHVybiB0cnVlOwogICAgICAgIH0KICAgICAgICByZXR1cm4gZmFsc2U7CiAgICB9Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIEluaXQ6Ci8vIF9fd2JnX2luaXQKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmFzeW5jIGZ1bmN0aW9uIF9fd2JnX2luaXQobW9kdWxlX29yX3BhdGgsIG1lbW9yeSkgewogICAgaWYgKHdhc20gIT09IHVuZGVmaW5lZCkKICAgICAgICByZXR1cm4gd2FzbTsKICAgIGxldCB0aHJlYWRfc3RhY2tfc2l6ZTsKICAgIGlmIChtb2R1bGVfb3JfcGF0aCAhPT0gdW5kZWZpbmVkKSB7CiAgICAgICAgaWYgKE9iamVjdC5nZXRQcm90b3R5cGVPZihtb2R1bGVfb3JfcGF0aCkgPT09IE9iamVjdC5wcm90b3R5cGUpIHsKICAgICAgICAgICAgKHsgbW9kdWxlX29yX3BhdGgsIG1lbW9yeSwgdGhyZWFkX3N0YWNrX3NpemUgfSA9IG1vZHVsZV9vcl9wYXRoKTsKICAgICAgICB9CiAgICAgICAgZWxzZSB7CiAgICAgICAgICAgIGNvbnNvbGUud2FybigndXNpbmcgZGVwcmVjYXRlZCBwYXJhbWV0ZXJzIGZvciB0aGUgaW5pdGlhbGl6YXRpb24gZnVuY3Rpb247IHBhc3MgYSBzaW5nbGUgb2JqZWN0IGluc3RlYWQnKTsKICAgICAgICB9CiAgICB9CiAgICAvLyAgIGlmIChtb2R1bGVfb3JfcGF0aCA9PT0gdW5kZWZpbmVkKSB7CiAgICAvLyAgICAgbW9kdWxlX29yX3BhdGggPSBuZXcgVVJMKCd0ZmhlX2JnLndhc20nLCBpbXBvcnQubWV0YS51cmwpOwogICAgLy8gICB9CiAgICBjb25zdCBpbXBvcnRzID0gX193YmdfZ2V0X2ltcG9ydHMobWVtb3J5KTsKICAgIC8vICAgaWYgKAogICAgLy8gICAgIHR5cGVvZiBtb2R1bGVfb3JfcGF0aCA9PT0gJ3N0cmluZycgfHwKICAgIC8vICAgICAodHlwZW9mIFJlcXVlc3QgPT09ICdmdW5jdGlvbicgJiYgbW9kdWxlX29yX3BhdGggaW5zdGFuY2VvZiBSZXF1ZXN0KSB8fAogICAgLy8gICAgICh0eXBlb2YgVVJMID09PSAnZnVuY3Rpb24nICYmIG1vZHVsZV9vcl9wYXRoIGluc3RhbmNlb2YgVVJMKQogICAgLy8gICApIHsKICAgIC8vICAgICBtb2R1bGVfb3JfcGF0aCA9IGZldGNoKG1vZHVsZV9vcl9wYXRoKTsKICAgIC8vICAgfQogICAgY29uc3QgeyBpbnN0YW5jZSwgbW9kdWxlIH0gPSBhd2FpdCBfX3diZ19sb2FkKGF3YWl0IG1vZHVsZV9vcl9wYXRoLCBpbXBvcnRzKTsKICAgIHJldHVybiBfX3diZ19maW5hbGl6ZV9pbml0KGluc3RhbmNlLCBtb2R1bGUsIHRocmVhZF9zdGFja19zaXplKTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8KLy8gVGhlICd0ZmhlJyBnbG9iYWwgb2JqZWN0Ci8vID09PT09PT09PT09PT09PT09PT09PT09PQovLyBGaW5hbCB0ZmhlIG9iamVjdCBnbG9iYWwgZGVjbGFyYXRpb24gY2FsbGVkIGJ5ICd3YWl0Rm9yTXNnVHlwZScgb25seQovLwovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKdmFyIHRmaGUgPSAvKiNfX1BVUkVfXyovIE9iamVjdC5mcmVlemUoewogIF9fcHJvdG9fXzogbnVsbCwKICBkZWZhdWx0OiBfX3diZ19pbml0LAogIHdiZ19yYXlvbl9zdGFydF93b3JrZXI6IHdiZ19yYXlvbl9zdGFydF93b3JrZXIsCn0pOwoK";
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
