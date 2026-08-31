/**
 * Auto-generated from scripts/wasm/tfhe/startWorkers.template.js.
 * Embedded worker base64 payload SHA-256: 1f4f4f7234cf6c3ef9d2af7a53cf219865d9b2337a9604f1a1f825c793153cc6
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
 *  - Hash is the build-time constant "a4cae88c180bc20081154feac5babf7229c65c6bb1bb9a2e47a4eed5ecfdc697".
 *  - auto silently falls back to embedded-base64 on any non-SHA-256 error.
 *  - precheck-direct-url's SHA-256 check is informational; the runtime refetches.
 *
 * Errors:
 *  - Partial worker-pool failure: successful workers are terminated before throw.
 *  - Concurrent failures: only the first error is surfaced.
 *  - __waitForMsgType has no timeout — a silent worker hangs startWorkers().
 *  - The embedded worker must listen on parentPort in node:worker_threads
 *    even when addEventListener exists (bun 1.4+ Node-parity message routing).
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
const _workerUrlSha256 = "a4cae88c180bc20081154feac5babf7229c65c6bb1bb9a2e47a4eed5ecfdc697";
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
 * 1. Fetch the URL and verify its SHA-256 against "a4cae88c180bc20081154feac5babf7229c65c6bb1bb9a2e47a4eed5ecfdc697" — fails fast on mismatch.
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
  const workerBase64 = "YXN5bmMgZnVuY3Rpb24gX19fZ2V0VGFyZ2V0KCkgewogIC8vIFByZWZlciBwYXJlbnRQb3J0IHdoZW5ldmVyIHRoaXMgaXMgYSBub2RlOndvcmtlcl90aHJlYWRzIHdvcmtlci4KICAvLyBgYWRkRXZlbnRMaXN0ZW5lcmAgaXMgbm90IGEgYnJvd3NlciBzaWduYWw6IEJ1biBleHBvc2VzIEV2ZW50VGFyZ2V0IEFQSXMKICAvLyBpbnNpZGUgd29ya2VyX3RocmVhZHMsIGJ1dCBidW4gPj0gMS40IGRlbGl2ZXJzIHBhcmVudCBtZXNzYWdlcyBvbmx5IHRvCiAgLy8gcGFyZW50UG9ydCAoTm9kZSBwYXJpdHkpLiBMaXN0ZW5pbmcgb24gYHNlbGZgIGhhbmdzIHN0YXJ0V29ya2VycygpLgogIHRyeSB7CiAgICBjb25zdCBub2RlTW9kdWxlTmFtZSA9ICd3b3JrZXJfdGhyZWFkcyc7CiAgICBjb25zdCBub2RlTW9kdWxlSWQgPSBgbm9kZToke25vZGVNb2R1bGVOYW1lfWA7CiAgICBjb25zdCB7IHBhcmVudFBvcnQgfSA9IGF3YWl0IGltcG9ydCgvKiBAdml0ZS1pZ25vcmUgKi8gbm9kZU1vZHVsZUlkKTsKICAgIGlmIChwYXJlbnRQb3J0KSByZXR1cm4gcGFyZW50UG9ydDsKICB9IGNhdGNoIHsKICAgIC8vIEJyb3dzZXIgLyBlZGdlOiBub2RlOndvcmtlcl90aHJlYWRzIGlzIG5vdCBpbXBvcnRhYmxlLgogIH0KICByZXR1cm4gc2VsZjsKfQoKZnVuY3Rpb24gX19fd2FpdEZvck1zZ1R5cGUodGFyZ2V0LCB0eXBlKSB7CiAgcmV0dXJuIG5ldyBQcm9taXNlKChyZXNvbHZlKSA9PiB7CiAgICBpZiAodHlwZW9mIHRhcmdldC5vbiA9PT0gJ2Z1bmN0aW9uJykgewogICAgICAvLyBOb2RlOiBFdmVudEVtaXR0ZXIsIGRhdGEgcGFzc2VkIGRpcmVjdGx5CiAgICAgIHRhcmdldC5vbignbWVzc2FnZScsIGZ1bmN0aW9uIG9uTXNnKGRhdGEpIHsKICAgICAgICBpZiAoZGF0YT8udHlwZSAhPT0gdHlwZSkgcmV0dXJuOwogICAgICAgIHRhcmdldC5vZmYoJ21lc3NhZ2UnLCBvbk1zZyk7CiAgICAgICAgcmVzb2x2ZShkYXRhKTsKICAgICAgfSk7CiAgICB9IGVsc2UgewogICAgICAvLyBCcm93c2VyOiBET00gZXZlbnRzLCBkYXRhIHdyYXBwZWQgaW4gTWVzc2FnZUV2ZW50CiAgICAgIHRhcmdldC5hZGRFdmVudExpc3RlbmVyKCdtZXNzYWdlJywgZnVuY3Rpb24gb25Nc2coeyBkYXRhIH0pIHsKICAgICAgICBpZiAoZGF0YT8udHlwZSAhPT0gdHlwZSkgcmV0dXJuOwogICAgICAgIHRhcmdldC5yZW1vdmVFdmVudExpc3RlbmVyKCdtZXNzYWdlJywgb25Nc2cpOwogICAgICAgIHJlc29sdmUoZGF0YSk7CiAgICAgIH0pOwogICAgfQogIH0pOwp9CgpfX19nZXRUYXJnZXQoKS50aGVuKCh0YXJnZXQpID0+CiAgX19fd2FpdEZvck1zZ1R5cGUodGFyZ2V0LCAnd2FzbV9iaW5kZ2VuX3dvcmtlcl9pbml0JykudGhlbigKICAgIGFzeW5jICh7IGluaXQsIHJlY2VpdmVyIH0pID0+IHsKICAgICAgY29uc3QgcGtnID0gYXdhaXQgUHJvbWlzZS5yZXNvbHZlKCkudGhlbihmdW5jdGlvbiAoKSB7CiAgICAgICAgcmV0dXJuIHRmaGU7CiAgICAgIH0pOwogICAgICBhd2FpdCBwa2cuZGVmYXVsdChpbml0KTsKICAgICAgdGFyZ2V0LnBvc3RNZXNzYWdlKHsgdHlwZTogJ3dhc21fYmluZGdlbl93b3JrZXJfcmVhZHknIH0pOwogICAgICBwa2cud2JnX3JheW9uX3N0YXJ0X3dvcmtlcihyZWNlaXZlcik7CiAgICB9LAogICksCik7CgovKioKICogQHBhcmFtIHtudW1iZXJ9IHJlY2VpdmVyCiAqLwpmdW5jdGlvbiB3YmdfcmF5b25fc3RhcnRfd29ya2VyKHJlY2VpdmVyKSB7CiAgd2FzbS53YmdfcmF5b25fc3RhcnRfd29ya2VyKHJlY2VpdmVyKTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gSW50ZXJuYWwgd2FzbWJpbmRnZW4gdG9vbHMKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vCi8vIEltcG9ydHM6Ci8vIF9fd2JnX2dldF9pbXBvcnRzCi8vCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBfX3diZ19nZXRfaW1wb3J0cyhtZW1vcnkpIHsKICAgIGNvbnN0IGltcG9ydDAgPSB7CiAgICAgICAgX19wcm90b19fOiBudWxsLAogICAgICAgIF9fd2JnX0JpZ0ludF81MmZmODM5MTI5N2RhMTk0OiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIHJldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICAgICAgY29uc3QgcmV0ID0gQmlnSW50KGFyZzApOwogICAgICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICAgICAgfSwgYXJndW1lbnRzKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX0JpZ0ludF9hZTIwMGU5M2NhY2JkMmIzOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBCaWdJbnQoYXJnMCk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19FcnJvcl8zNjM5YTYwZWQxNWY4N2U3OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBFcnJvcihnZXRTdHJpbmdGcm9tV2FzbTAoYXJnMCwgYXJnMSkpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9iaWdpbnRfZ2V0X2FzX2k2NF8zYWY2ZDRjYTc3MTkzYTRiOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCB2ID0gYXJnMTsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mICh2KSA9PT0gJ2JpZ2ludCcgPyB2IDogdW5kZWZpbmVkOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRCaWdJbnQ2NChhcmcwICsgOCAqIDEsIGlzTGlrZU5vbmUocmV0KSA/IEJpZ0ludCgwKSA6IHJldCwgdHJ1ZSk7CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMCwgIWlzTGlrZU5vbmUocmV0KSwgdHJ1ZSk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2JpdF9hbmRfYmZlZGVjZTJiYjY1N2Y0YjogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMCAmIGFyZzE7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2RlYnVnX3N0cmluZ18wN2NiNzJjZmNjOTUyZTJiOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBkZWJ1Z1N0cmluZyhhcmcxKTsKICAgICAgICAgICAgY29uc3QgcHRyMSA9IHBhc3NTdHJpbmdUb1dhc20wKHJldCwgd2FzbS5fX3diaW5kZ2VuX21hbGxvYywgd2FzbS5fX3diaW5kZ2VuX3JlYWxsb2MpOwogICAgICAgICAgICBjb25zdCBsZW4xID0gV0FTTV9WRUNUT1JfTEVOOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDEsIGxlbjEsIHRydWUpOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDAsIHB0cjEsIHRydWUpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9pc19mdW5jdGlvbl8yZjBmZDdjZWI4NmU2NGM1OiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgKGFyZzApID09PSAnZnVuY3Rpb24nOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9pc19vYmplY3RfNWIyMmZmMjQxODA2M2E5YzogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgdmFsID0gYXJnMDsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mICh2YWwpID09PSAnb2JqZWN0JyAmJiB2YWwgIT09IG51bGw7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2lzX3N0cmluZ19lZGRjMDdhM2VmYWQ1MmU2OiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgKGFyZzApID09PSAnc3RyaW5nJzsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5faXNfdW5kZWZpbmVkXzI0NGE5MmMzNGQzYjZlYzA6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAgPT09IHVuZGVmaW5lZDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fanN2YWxfZXFfNDAzZWFhMzYxMDUwMGEyNTogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMCA9PT0gYXJnMTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fbHRfYzQ4M2NjNjk0ZGU2N2MzZTogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMCA8IGFyZzE7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX21lbW9yeV9jMjM1NmRkMWEwODlkZmJkOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHdhc20ubWVtb3J5OwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9tb2R1bGVfZGY3MDQzOTNkZmQxODUzYzogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB3YXNtTW9kdWxlOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9uZWdfOWI0ZDcxODIzZTNiYzUxMzogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gLWFyZzA7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX3Nocl9kOGY4MjY4ZjE4YzdhMWMzOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwID4+IGFyZzE7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX3N0cmluZ19nZXRfOTY1NTkyMDczZTVkODQ4YzogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3Qgb2JqID0gYXJnMTsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mIChvYmopID09PSAnc3RyaW5nJyA/IG9iaiA6IHVuZGVmaW5lZDsKICAgICAgICAgICAgdmFyIHB0cjEgPSBpc0xpa2VOb25lKHJldCkgPyAwIDogcGFzc1N0cmluZ1RvV2FzbTAocmV0LCB3YXNtLl9fd2JpbmRnZW5fbWFsbG9jLCB3YXNtLl9fd2JpbmRnZW5fcmVhbGxvYyk7CiAgICAgICAgICAgIHZhciBsZW4xID0gV0FTTV9WRUNUT1JfTEVOOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDEsIGxlbjEsIHRydWUpOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDAsIHB0cjEsIHRydWUpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl90aHJvd185Yzc1ZDQ3YmY5ZTc3MzFlOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICB0aHJvdyBuZXcgRXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGFyZzAsIGFyZzEpKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX2NhbGxfYTQxZDY0MjFiMzBhMzJjNTogZnVuY3Rpb24gKCkgewogICAgICAgICAgICByZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24gKGFyZzAsIGFyZzEsIGFyZzIpIHsKICAgICAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAuY2FsbChhcmcxLCBhcmcyKTsKICAgICAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgICAgIH0sIGFyZ3VtZW50cyk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19jcnlwdG9fMzhkZjJiYWIxMjZiNjNkYzogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5jcnlwdG87CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19lcnJvcl9hNmZhMjAyYjU4YWExY2QzOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBsZXQgZGVmZXJyZWQwXzA7CiAgICAgICAgICAgIGxldCBkZWZlcnJlZDBfMTsKICAgICAgICAgICAgdHJ5IHsKICAgICAgICAgICAgICAgIGRlZmVycmVkMF8wID0gYXJnMDsKICAgICAgICAgICAgICAgIGRlZmVycmVkMF8xID0gYXJnMTsKICAgICAgICAgICAgICAgIGNvbnNvbGUuZXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGFyZzAsIGFyZzEpKTsKICAgICAgICAgICAgfQogICAgICAgICAgICBmaW5hbGx5IHsKICAgICAgICAgICAgICAgIHdhc20uX193YmluZGdlbl9mcmVlKGRlZmVycmVkMF8wLCBkZWZlcnJlZDBfMSwgMSk7CiAgICAgICAgICAgIH0KICAgICAgICB9LAogICAgICAgIF9fd2JnX2dldFJhbmRvbVZhbHVlc19jNDRhNTBkOGNmZGFlYmViOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIHJldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICAgICAgYXJnMC5nZXRSYW5kb21WYWx1ZXMoYXJnMSk7CiAgICAgICAgICAgIH0sIGFyZ3VtZW50cyk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19pbnN0YW5jZW9mX1dpbmRvd180MTUzYzE4MThhMWMwYzBiOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBsZXQgcmVzdWx0OwogICAgICAgICAgICB0cnkgewogICAgICAgICAgICAgICAgcmVzdWx0ID0gYXJnMCBpbnN0YW5jZW9mIFdpbmRvdzsKICAgICAgICAgICAgfQogICAgICAgICAgICBjYXRjaCAoXykgewogICAgICAgICAgICAgICAgcmVzdWx0ID0gZmFsc2U7CiAgICAgICAgICAgIH0KICAgICAgICAgICAgY29uc3QgcmV0ID0gcmVzdWx0OwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfbGVuZ3RoX2JhM2MwMzI2MDJlZmUzMTA6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAubGVuZ3RoOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfbXNDcnlwdG9fYmQ1YTAzNGFmOTZiY2JhNjogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5tc0NyeXB0bzsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX25ld18yMjdkN2MwNTQxNGViODYxOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IG5ldyBFcnJvcigpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfbmV3X3dpdGhfbGVuZ3RoXzkwMTFmNWRhNzk0YmY1ZDk6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IG5ldyBVaW50OEFycmF5KGFyZzAgPj4+IDApOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193Ymdfbm9kZV84NGVhODc1NDExMjU0ZGIxOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLm5vZGU7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19wcm9jZXNzXzQ0YzdhMTRlMTFlOWY2OWU6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAucHJvY2VzczsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3Byb3RvdHlwZXNldGNhbGxfZmQ0MDUwZTgwNmUxZDUxOTogZnVuY3Rpb24gKGFyZzAsIGFyZzEsIGFyZzIpIHsKICAgICAgICAgICAgVWludDhBcnJheS5wcm90b3R5cGUuc2V0LmNhbGwoZ2V0QXJyYXlVOEZyb21XYXNtMChhcmcwLCBhcmcxKSwgYXJnMik7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19yYW5kb21GaWxsU3luY182YzI1ZWFjOTg2OWViNTNjOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIHJldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICAgICAgYXJnMC5yYW5kb21GaWxsU3luYyhhcmcxKTsKICAgICAgICAgICAgfSwgYXJndW1lbnRzKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3JlcXVpcmVfYjRlZGJkY2YzZTJhMWVmMDogZnVuY3Rpb24gKCkgewogICAgICAgICAgICByZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24gKCkgewogICAgICAgICAgICAgICAgY29uc3QgcmV0ID0gbW9kdWxlLnJlcXVpcmU7CiAgICAgICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgICAgICB9LCBhcmd1bWVudHMpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3RhY2tfM2IwZDk3NGJiZjMxZTQ0ZjogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMS5zdGFjazsKICAgICAgICAgICAgY29uc3QgcHRyMSA9IHBhc3NTdHJpbmdUb1dhc20wKHJldCwgd2FzbS5fX3diaW5kZ2VuX21hbGxvYywgd2FzbS5fX3diaW5kZ2VuX3JlYWxsb2MpOwogICAgICAgICAgICBjb25zdCBsZW4xID0gV0FTTV9WRUNUT1JfTEVOOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDEsIGxlbjEsIHRydWUpOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDAsIHB0cjEsIHRydWUpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3RhcnRXb3JrZXJzXzhiNTgyZDU3ZTkyYmQyZDQ6IGZ1bmN0aW9uIChhcmcwLCBhcmcxLCBhcmcyKSB7CiAgICAgICAgICAgIGhhbmRsZUVycm9yKGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgICAgIHRocm93IG5ldyBFcnJvcignc3RhcnRXb3JrZXJzIG5vdCBzdXBwb3J0ZWQgZnJvbSBhIHdvcmtlciB0aHJlYWQnKTsKICAgICAgICAgICAgfSk7CiAgICAgICAgICAgIC8vIGNvbnN0IHJldCA9IHN0YXJ0V29ya2VycyhhcmcwLCBhcmcxLCB3YmdfcmF5b25fUG9vbEJ1aWxkZXIuX193cmFwKGFyZzIpKTsKICAgICAgICAgICAgLy8gcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3N0YXRpY19hY2Nlc3Nvcl9HTE9CQUxfVEhJU18xYzdmMWJkNmM2OTQxZmRiOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiBnbG9iYWxUaGlzID09PSAndW5kZWZpbmVkJyA/IG51bGwgOiBnbG9iYWxUaGlzOwogICAgICAgICAgICByZXR1cm4gaXNMaWtlTm9uZShyZXQpID8gMCA6IGFkZFRvRXh0ZXJucmVmVGFibGUwKHJldCk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdGF0aWNfYWNjZXNzb3JfR0xPQkFMX2UwMzliYzkxNGY4M2U3NGU6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mIGdsb2JhbCA9PT0gJ3VuZGVmaW5lZCcgPyBudWxsIDogZ2xvYmFsOwogICAgICAgICAgICByZXR1cm4gaXNMaWtlTm9uZShyZXQpID8gMCA6IGFkZFRvRXh0ZXJucmVmVGFibGUwKHJldCk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdGF0aWNfYWNjZXNzb3JfU0VMRl84YmY4YzQ4YzI4NDIwYWQ1OiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiBzZWxmID09PSAndW5kZWZpbmVkJyA/IG51bGwgOiBzZWxmOwogICAgICAgICAgICByZXR1cm4gaXNMaWtlTm9uZShyZXQpID8gMCA6IGFkZFRvRXh0ZXJucmVmVGFibGUwKHJldCk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdGF0aWNfYWNjZXNzb3JfV0lORE9XXzZhZWVlOWI1MTY1MmVlMGY6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mIHdpbmRvdyA9PT0gJ3VuZGVmaW5lZCcgPyBudWxsIDogd2luZG93OwogICAgICAgICAgICByZXR1cm4gaXNMaWtlTm9uZShyZXQpID8gMCA6IGFkZFRvRXh0ZXJucmVmVGFibGUwKHJldCk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdWJhcnJheV9mYmUzY2VmMjkwZTFmYTQzOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSwgYXJnMikgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLnN1YmFycmF5KGFyZzEgPj4+IDAsIGFyZzIgPj4+IDApOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfdG9TdHJpbmdfMTU2NTZhZjhkOGU3MWYxNjogZnVuY3Rpb24gKGFyZzAsIGFyZzEsIGFyZzIpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMS50b1N0cmluZyhhcmcyKTsKICAgICAgICAgICAgY29uc3QgcHRyMSA9IHBhc3NTdHJpbmdUb1dhc20wKHJldCwgd2FzbS5fX3diaW5kZ2VuX21hbGxvYywgd2FzbS5fX3diaW5kZ2VuX3JlYWxsb2MpOwogICAgICAgICAgICBjb25zdCBsZW4xID0gV0FTTV9WRUNUT1JfTEVOOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDEsIGxlbjEsIHRydWUpOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDAsIHB0cjEsIHRydWUpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfdG9TdHJpbmdfOGQ4NzQ0ODliYWQ3ZTVhMjogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC50b1N0cmluZygpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfdmVyc2lvbnNfMjc2YjI3OTViMWM2YTIxOTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC52ZXJzaW9uczsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDAxOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICAvLyBDYXN0IGludHJpbnNpYyBmb3IgYEY2NCAtPiBFeHRlcm5yZWZgLgogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDI6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIC8vIENhc3QgaW50cmluc2ljIGZvciBgSTY0IC0+IEV4dGVybnJlZmAuCiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzA7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diaW5kZ2VuX2Nhc3RfMDAwMDAwMDAwMDAwMDAwMzogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgLy8gQ2FzdCBpbnRyaW5zaWMgZm9yIGBSZWYoU2xpY2UoVTgpKSAtPiBOYW1lZEV4dGVybnJlZigiVWludDhBcnJheSIpYC4KICAgICAgICAgICAgY29uc3QgcmV0ID0gZ2V0QXJyYXlVOEZyb21XYXNtMChhcmcwLCBhcmcxKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDA0OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICAvLyBDYXN0IGludHJpbnNpYyBmb3IgYFJlZihTdHJpbmcpIC0+IEV4dGVybnJlZmAuCiAgICAgICAgICAgIGNvbnN0IHJldCA9IGdldFN0cmluZ0Zyb21XYXNtMChhcmcwLCBhcmcxKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDA1OiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICAvLyBDYXN0IGludHJpbnNpYyBmb3IgYFU2NCAtPiBFeHRlcm5yZWZgLgogICAgICAgICAgICBjb25zdCByZXQgPSBCaWdJbnQuYXNVaW50Tig2NCwgYXJnMCk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diaW5kZ2VuX2luaXRfZXh0ZXJucmVmX3RhYmxlOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHRhYmxlID0gd2FzbS5fX3diaW5kZ2VuX2V4dGVybnJlZnM7CiAgICAgICAgICAgIGNvbnN0IG9mZnNldCA9IHRhYmxlLmdyb3coNCk7CiAgICAgICAgICAgIHRhYmxlLnNldCgwLCB1bmRlZmluZWQpOwogICAgICAgICAgICB0YWJsZS5zZXQob2Zmc2V0ICsgMCwgdW5kZWZpbmVkKTsKICAgICAgICAgICAgdGFibGUuc2V0KG9mZnNldCArIDEsIG51bGwpOwogICAgICAgICAgICB0YWJsZS5zZXQob2Zmc2V0ICsgMiwgdHJ1ZSk7CiAgICAgICAgICAgIHRhYmxlLnNldChvZmZzZXQgKyAzLCBmYWxzZSk7CiAgICAgICAgfSwKICAgICAgICBtZW1vcnk6IG1lbW9yeSB8fCBuZXcgV2ViQXNzZW1ibHkuTWVtb3J5KHsgaW5pdGlhbDogMTksIG1heGltdW06IDE2Mzg0LCBzaGFyZWQ6IHRydWUgfSksCiAgICB9OwogICAgcmV0dXJuIHsKICAgICAgICBfX3Byb3RvX186IG51bGwsCiAgICAgICAgIi4vdGZoZV9iZy5qcyI6IGltcG9ydDAsCiAgICB9Owp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBhZGRUb0V4dGVybnJlZlRhYmxlMAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gYWRkVG9FeHRlcm5yZWZUYWJsZTAob2JqKSB7CiAgICBjb25zdCBpZHggPSB3YXNtLl9fZXh0ZXJucmVmX3RhYmxlX2FsbG9jKCk7CiAgICB3YXNtLl9fd2JpbmRnZW5fZXh0ZXJucmVmcy5zZXQoaWR4LCBvYmopOwogICAgcmV0dXJuIGlkeDsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gZGVidWdTdHJpbmcKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGRlYnVnU3RyaW5nKHZhbCkgewogICAgLy8gcHJpbWl0aXZlIHR5cGVzCiAgICBjb25zdCB0eXBlID0gdHlwZW9mIHZhbDsKICAgIGlmICh0eXBlID09ICdudW1iZXInIHx8IHR5cGUgPT0gJ2Jvb2xlYW4nIHx8IHZhbCA9PSBudWxsKSB7CiAgICAgICAgcmV0dXJuIGAke3ZhbH1gOwogICAgfQogICAgaWYgKHR5cGUgPT0gJ3N0cmluZycpIHsKICAgICAgICByZXR1cm4gYCIke3ZhbH0iYDsKICAgIH0KICAgIGlmICh0eXBlID09ICdzeW1ib2wnKSB7CiAgICAgICAgY29uc3QgZGVzY3JpcHRpb24gPSB2YWwuZGVzY3JpcHRpb247CiAgICAgICAgaWYgKGRlc2NyaXB0aW9uID09IG51bGwpIHsKICAgICAgICAgICAgcmV0dXJuICdTeW1ib2wnOwogICAgICAgIH0KICAgICAgICBlbHNlIHsKICAgICAgICAgICAgcmV0dXJuIGBTeW1ib2woJHtkZXNjcmlwdGlvbn0pYDsKICAgICAgICB9CiAgICB9CiAgICBpZiAodHlwZSA9PSAnZnVuY3Rpb24nKSB7CiAgICAgICAgY29uc3QgbmFtZSA9IHZhbC5uYW1lOwogICAgICAgIGlmICh0eXBlb2YgbmFtZSA9PSAnc3RyaW5nJyAmJiBuYW1lLmxlbmd0aCA+IDApIHsKICAgICAgICAgICAgcmV0dXJuIGBGdW5jdGlvbigke25hbWV9KWA7CiAgICAgICAgfQogICAgICAgIGVsc2UgewogICAgICAgICAgICByZXR1cm4gJ0Z1bmN0aW9uJzsKICAgICAgICB9CiAgICB9CiAgICAvLyBvYmplY3RzCiAgICBpZiAoQXJyYXkuaXNBcnJheSh2YWwpKSB7CiAgICAgICAgY29uc3QgbGVuZ3RoID0gdmFsLmxlbmd0aDsKICAgICAgICBsZXQgZGVidWcgPSAnWyc7CiAgICAgICAgaWYgKGxlbmd0aCA+IDApIHsKICAgICAgICAgICAgZGVidWcgKz0gZGVidWdTdHJpbmcodmFsWzBdKTsKICAgICAgICB9CiAgICAgICAgZm9yIChsZXQgaSA9IDE7IGkgPCBsZW5ndGg7IGkrKykgewogICAgICAgICAgICBkZWJ1ZyArPSAnLCAnICsgZGVidWdTdHJpbmcodmFsW2ldKTsKICAgICAgICB9CiAgICAgICAgZGVidWcgKz0gJ10nOwogICAgICAgIHJldHVybiBkZWJ1ZzsKICAgIH0KICAgIC8vIFRlc3QgZm9yIGJ1aWx0LWluCiAgICBjb25zdCBidWlsdEluTWF0Y2hlcyA9IC9cW29iamVjdCAoW15cXV0rKVxdLy5leGVjKHRvU3RyaW5nLmNhbGwodmFsKSk7CiAgICBsZXQgY2xhc3NOYW1lOwogICAgaWYgKGJ1aWx0SW5NYXRjaGVzICYmIGJ1aWx0SW5NYXRjaGVzLmxlbmd0aCA+IDEpIHsKICAgICAgICBjbGFzc05hbWUgPSBidWlsdEluTWF0Y2hlc1sxXTsKICAgIH0KICAgIGVsc2UgewogICAgICAgIC8vIEZhaWxlZCB0byBtYXRjaCB0aGUgc3RhbmRhcmQgJ1tvYmplY3QgQ2xhc3NOYW1lXScKICAgICAgICByZXR1cm4gdG9TdHJpbmcuY2FsbCh2YWwpOwogICAgfQogICAgaWYgKGNsYXNzTmFtZSA9PSAnT2JqZWN0JykgewogICAgICAgIC8vIHdlJ3JlIGEgdXNlciBkZWZpbmVkIGNsYXNzIG9yIE9iamVjdAogICAgICAgIC8vIEpTT04uc3RyaW5naWZ5IGF2b2lkcyBwcm9ibGVtcyB3aXRoIGN5Y2xlcywgYW5kIGlzIGdlbmVyYWxseSBtdWNoCiAgICAgICAgLy8gZWFzaWVyIHRoYW4gbG9vcGluZyB0aHJvdWdoIG93blByb3BlcnRpZXMgb2YgYHZhbGAuCiAgICAgICAgdHJ5IHsKICAgICAgICAgICAgcmV0dXJuICdPYmplY3QoJyArIEpTT04uc3RyaW5naWZ5KHZhbCkgKyAnKSc7CiAgICAgICAgfQogICAgICAgIGNhdGNoIChfKSB7CiAgICAgICAgICAgIHJldHVybiAnT2JqZWN0JzsKICAgICAgICB9CiAgICB9CiAgICAvLyBlcnJvcnMKICAgIGlmICh2YWwgaW5zdGFuY2VvZiBFcnJvcikgewogICAgICAgIHJldHVybiBgJHt2YWwubmFtZX06ICR7dmFsLm1lc3NhZ2V9XG4ke3ZhbC5zdGFja31gOwogICAgfQogICAgLy8gVE9ETyB3ZSBjb3VsZCB0ZXN0IGZvciBtb3JlIHRoaW5ncyBoZXJlLCBsaWtlIGBTZXRgcyBhbmQgYE1hcGBzLgogICAgcmV0dXJuIGNsYXNzTmFtZTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gZ2V0QXJyYXlVOEZyb21XYXNtMAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gZ2V0QXJyYXlVOEZyb21XYXNtMChwdHIsIGxlbikgewogICAgcHRyID0gcHRyID4+PiAwOwogICAgcmV0dXJuIGdldFVpbnQ4QXJyYXlNZW1vcnkwKCkuc3ViYXJyYXkocHRyIC8gMSwgcHRyIC8gMSArIGxlbik7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGNhY2hlZERhdGFWaWV3TWVtb3J5MAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IGNhY2hlZERhdGFWaWV3TWVtb3J5MCA9IG51bGw7CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBnZXREYXRhVmlld01lbW9yeTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGdldERhdGFWaWV3TWVtb3J5MCgpIHsKICAgIGlmIChjYWNoZWREYXRhVmlld01lbW9yeTAgPT09IG51bGwgfHwgY2FjaGVkRGF0YVZpZXdNZW1vcnkwLmJ1ZmZlciAhPT0gd2FzbS5tZW1vcnkuYnVmZmVyKSB7CiAgICAgICAgY2FjaGVkRGF0YVZpZXdNZW1vcnkwID0gbmV3IERhdGFWaWV3KHdhc20ubWVtb3J5LmJ1ZmZlcik7CiAgICB9CiAgICByZXR1cm4gY2FjaGVkRGF0YVZpZXdNZW1vcnkwOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBnZXRTdHJpbmdGcm9tV2FzbTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGdldFN0cmluZ0Zyb21XYXNtMChwdHIsIGxlbikgewogICAgcmV0dXJuIGRlY29kZVRleHQocHRyID4+PiAwLCBsZW4pOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBjYWNoZWRVaW50OEFycmF5TWVtb3J5MAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwID0gbnVsbDsKCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGdldFVpbnQ4QXJyYXlNZW1vcnkwCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBnZXRVaW50OEFycmF5TWVtb3J5MCgpIHsKICAgIGlmIChjYWNoZWRVaW50OEFycmF5TWVtb3J5MCA9PT0gbnVsbCB8fCBjYWNoZWRVaW50OEFycmF5TWVtb3J5MC5idWZmZXIgIT09IHdhc20ubWVtb3J5LmJ1ZmZlcikgewogICAgICAgIGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwID0gbmV3IFVpbnQ4QXJyYXkod2FzbS5tZW1vcnkuYnVmZmVyKTsKICAgIH0KICAgIHJldHVybiBjYWNoZWRVaW50OEFycmF5TWVtb3J5MDsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gaGFuZGxlRXJyb3IKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGhhbmRsZUVycm9yKGYsIGFyZ3MpIHsKICAgIHRyeSB7CiAgICAgICAgcmV0dXJuIGYuYXBwbHkodGhpcywgYXJncyk7CiAgICB9CiAgICBjYXRjaCAoZSkgewogICAgICAgIGNvbnN0IGlkeCA9IGFkZFRvRXh0ZXJucmVmVGFibGUwKGUpOwogICAgICAgIHdhc20uX193YmluZGdlbl9leG5fc3RvcmUoaWR4KTsKICAgIH0KfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gaXNMaWtlTm9uZQovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gaXNMaWtlTm9uZSh4KSB7CiAgICByZXR1cm4geCA9PT0gdW5kZWZpbmVkIHx8IHggPT09IG51bGw7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIHBhc3NTdHJpbmdUb1dhc20wCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBwYXNzU3RyaW5nVG9XYXNtMChhcmcsIG1hbGxvYywgcmVhbGxvYykgewogICAgaWYgKHJlYWxsb2MgPT09IHVuZGVmaW5lZCkgewogICAgICAgIGNvbnN0IGJ1ZiA9IGNhY2hlZFRleHRFbmNvZGVyLmVuY29kZShhcmcpOwogICAgICAgIGNvbnN0IHB0ciA9IG1hbGxvYyhidWYubGVuZ3RoLCAxKSA+Pj4gMDsKICAgICAgICBnZXRVaW50OEFycmF5TWVtb3J5MCgpLnN1YmFycmF5KHB0ciwgcHRyICsgYnVmLmxlbmd0aCkuc2V0KGJ1Zik7CiAgICAgICAgV0FTTV9WRUNUT1JfTEVOID0gYnVmLmxlbmd0aDsKICAgICAgICByZXR1cm4gcHRyOwogICAgfQogICAgbGV0IGxlbiA9IGFyZy5sZW5ndGg7CiAgICBsZXQgcHRyID0gbWFsbG9jKGxlbiwgMSkgPj4+IDA7CiAgICBjb25zdCBtZW0gPSBnZXRVaW50OEFycmF5TWVtb3J5MCgpOwogICAgbGV0IG9mZnNldCA9IDA7CiAgICBmb3IgKDsgb2Zmc2V0IDwgbGVuOyBvZmZzZXQrKykgewogICAgICAgIGNvbnN0IGNvZGUgPSBhcmcuY2hhckNvZGVBdChvZmZzZXQpOwogICAgICAgIGlmIChjb2RlID4gMHg3RikKICAgICAgICAgICAgYnJlYWs7CiAgICAgICAgbWVtW3B0ciArIG9mZnNldF0gPSBjb2RlOwogICAgfQogICAgaWYgKG9mZnNldCAhPT0gbGVuKSB7CiAgICAgICAgaWYgKG9mZnNldCAhPT0gMCkgewogICAgICAgICAgICBhcmcgPSBhcmcuc2xpY2Uob2Zmc2V0KTsKICAgICAgICB9CiAgICAgICAgcHRyID0gcmVhbGxvYyhwdHIsIGxlbiwgbGVuID0gb2Zmc2V0ICsgYXJnLmxlbmd0aCAqIDMsIDEpID4+PiAwOwogICAgICAgIGNvbnN0IHZpZXcgPSBnZXRVaW50OEFycmF5TWVtb3J5MCgpLnN1YmFycmF5KHB0ciArIG9mZnNldCwgcHRyICsgbGVuKTsKICAgICAgICBjb25zdCByZXQgPSBjYWNoZWRUZXh0RW5jb2Rlci5lbmNvZGVJbnRvKGFyZywgdmlldyk7CiAgICAgICAgb2Zmc2V0ICs9IHJldC53cml0dGVuOwogICAgICAgIHB0ciA9IHJlYWxsb2MocHRyLCBsZW4sIG9mZnNldCwgMSkgPj4+IDA7CiAgICB9CiAgICBXQVNNX1ZFQ1RPUl9MRU4gPSBvZmZzZXQ7CiAgICByZXR1cm4gcHRyOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBjYWNoZWRUZXh0RGVjb2RlcgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IGNhY2hlZFRleHREZWNvZGVyID0gKHR5cGVvZiBUZXh0RGVjb2RlciAhPT0gJ3VuZGVmaW5lZCcgPyBuZXcgVGV4dERlY29kZXIoJ3V0Zi04JywgeyBpZ25vcmVCT006IHRydWUsIGZhdGFsOiB0cnVlIH0pIDogdW5kZWZpbmVkKTsKCmlmIChjYWNoZWRUZXh0RGVjb2RlcikKICAgIGNhY2hlZFRleHREZWNvZGVyLmRlY29kZSgpOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gTUFYX1NBRkFSSV9ERUNPREVfQllURVMKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmNvbnN0IE1BWF9TQUZBUklfREVDT0RFX0JZVEVTID0gMjE0NjQzNTA3MjsKCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIG51bUJ5dGVzRGVjb2RlZAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IG51bUJ5dGVzRGVjb2RlZCA9IDA7CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBkZWNvZGVUZXh0Ci8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBkZWNvZGVUZXh0KHB0ciwgbGVuKSB7CiAgICBudW1CeXRlc0RlY29kZWQgKz0gbGVuOwogICAgaWYgKG51bUJ5dGVzRGVjb2RlZCA+PSBNQVhfU0FGQVJJX0RFQ09ERV9CWVRFUykgewogICAgICAgIGNhY2hlZFRleHREZWNvZGVyID0gbmV3IFRleHREZWNvZGVyKCd1dGYtOCcsIHsgaWdub3JlQk9NOiB0cnVlLCBmYXRhbDogdHJ1ZSB9KTsKICAgICAgICBjYWNoZWRUZXh0RGVjb2Rlci5kZWNvZGUoKTsKICAgICAgICBudW1CeXRlc0RlY29kZWQgPSBsZW47CiAgICB9CiAgICByZXR1cm4gY2FjaGVkVGV4dERlY29kZXIuZGVjb2RlKGdldFVpbnQ4QXJyYXlNZW1vcnkwKCkuc2xpY2UocHRyLCBwdHIgKyBsZW4pKTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gY2FjaGVkVGV4dEVuY29kZXIKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmNvbnN0IGNhY2hlZFRleHRFbmNvZGVyID0gKHR5cGVvZiBUZXh0RW5jb2RlciAhPT0gJ3VuZGVmaW5lZCcgPyBuZXcgVGV4dEVuY29kZXIoKSA6IHVuZGVmaW5lZCk7CgppZiAoY2FjaGVkVGV4dEVuY29kZXIpIHsKICAgIGNhY2hlZFRleHRFbmNvZGVyLmVuY29kZUludG8gPSBmdW5jdGlvbiAoYXJnLCB2aWV3KSB7CiAgICAgICAgY29uc3QgYnVmID0gY2FjaGVkVGV4dEVuY29kZXIuZW5jb2RlKGFyZyk7CiAgICAgICAgdmlldy5zZXQoYnVmKTsKICAgICAgICByZXR1cm4gewogICAgICAgICAgICByZWFkOiBhcmcubGVuZ3RoLAogICAgICAgICAgICB3cml0dGVuOiBidWYubGVuZ3RoCiAgICAgICAgfTsKICAgIH07Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIFdBU01fVkVDVE9SX0xFTiBpcyBhIG1vZHVsZS1sZXZlbCB2YXJpYWJsZSB0aGF0IHN0b3JlcyB0aGUgYnl0ZSBsZW5ndGggb2YKLy8gdGhlIGRhdGEganVzdCB3cml0dGVuIGludG8gV0FTTSBtZW1vcnkuIEl0IGFjdHMgYXMgYW4gb3V0LXBhcmFtZXRlci4KLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmxldCBXQVNNX1ZFQ1RPUl9MRU4gPSAwOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gV0FTTSBtb2R1bGUgc3RhdGUKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmxldCB3YXNtTW9kdWxlLCB3YXNtSW5zdGFuY2UsIHdhc207CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBJbml0OgovLyBfX3diZ19maW5hbGl6ZV9pbml0Ci8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBfX3diZ19maW5hbGl6ZV9pbml0KGluc3RhbmNlLCBtb2R1bGUsIHRocmVhZF9zdGFja19zaXplKSB7CiAgICB3YXNtSW5zdGFuY2UgPSBpbnN0YW5jZTsKICAgIHdhc20gPSBpbnN0YW5jZS5leHBvcnRzOwogICAgd2FzbU1vZHVsZSA9IG1vZHVsZTsKICAgIGNhY2hlZERhdGFWaWV3TWVtb3J5MCA9IG51bGw7CiAgICBjYWNoZWRVaW50OEFycmF5TWVtb3J5MCA9IG51bGw7CiAgICBpZiAodHlwZW9mIHRocmVhZF9zdGFja19zaXplICE9PSAndW5kZWZpbmVkJyAmJiAodHlwZW9mIHRocmVhZF9zdGFja19zaXplICE9PSAnbnVtYmVyJyB8fCB0aHJlYWRfc3RhY2tfc2l6ZSA9PT0gMCB8fCB0aHJlYWRfc3RhY2tfc2l6ZSAlIDY1NTM2ICE9PSAwKSkgewogICAgICAgIHRocm93IG5ldyBFcnJvcignaW52YWxpZCBzdGFjayBzaXplJyk7CiAgICB9CiAgICB3YXNtLl9fd2JpbmRnZW5fc3RhcnQodGhyZWFkX3N0YWNrX3NpemUpOwogICAgcmV0dXJuIHdhc207Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIEluaXQ6Ci8vIF9fd2JnX2xvYWQKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmFzeW5jIGZ1bmN0aW9uIF9fd2JnX2xvYWQobW9kdWxlLCBpbXBvcnRzKSB7CiAgICBpZiAodHlwZW9mIFJlc3BvbnNlID09PSAnZnVuY3Rpb24nICYmIG1vZHVsZSBpbnN0YW5jZW9mIFJlc3BvbnNlKSB7CiAgICAgICAgaWYgKHR5cGVvZiBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZVN0cmVhbWluZyA9PT0gJ2Z1bmN0aW9uJykgewogICAgICAgICAgICB0cnkgewogICAgICAgICAgICAgICAgcmV0dXJuIGF3YWl0IFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlU3RyZWFtaW5nKG1vZHVsZSwgaW1wb3J0cyk7CiAgICAgICAgICAgIH0KICAgICAgICAgICAgY2F0Y2ggKGUpIHsKICAgICAgICAgICAgICAgIGNvbnN0IHZhbGlkUmVzcG9uc2UgPSBtb2R1bGUub2sgJiYgZXhwZWN0ZWRSZXNwb25zZVR5cGUobW9kdWxlLnR5cGUpOwogICAgICAgICAgICAgICAgaWYgKHZhbGlkUmVzcG9uc2UgJiYgbW9kdWxlLmhlYWRlcnMuZ2V0KCdDb250ZW50LVR5cGUnKSAhPT0gJ2FwcGxpY2F0aW9uL3dhc20nKSB7CiAgICAgICAgICAgICAgICAgICAgY29uc29sZS53YXJuKCJgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGVTdHJlYW1pbmdgIGZhaWxlZCBiZWNhdXNlIHlvdXIgc2VydmVyIGRvZXMgbm90IHNlcnZlIFdhc20gd2l0aCBgYXBwbGljYXRpb24vd2FzbWAgTUlNRSB0eXBlLiBGYWxsaW5nIGJhY2sgdG8gYFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlYCB3aGljaCBpcyBzbG93ZXIuIE9yaWdpbmFsIGVycm9yOlxuIiwgZSk7CiAgICAgICAgICAgICAgICB9CiAgICAgICAgICAgICAgICBlbHNlIHsKICAgICAgICAgICAgICAgICAgICB0aHJvdyBlOwogICAgICAgICAgICAgICAgfQogICAgICAgICAgICB9CiAgICAgICAgfQogICAgICAgIGNvbnN0IGJ5dGVzID0gYXdhaXQgbW9kdWxlLmFycmF5QnVmZmVyKCk7CiAgICAgICAgcmV0dXJuIGF3YWl0IFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlKGJ5dGVzLCBpbXBvcnRzKTsKICAgIH0KICAgIGVsc2UgewogICAgICAgIGNvbnN0IGluc3RhbmNlID0gYXdhaXQgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGUobW9kdWxlLCBpbXBvcnRzKTsKICAgICAgICBpZiAoaW5zdGFuY2UgaW5zdGFuY2VvZiBXZWJBc3NlbWJseS5JbnN0YW5jZSkgewogICAgICAgICAgICByZXR1cm4geyBpbnN0YW5jZSwgbW9kdWxlIH07CiAgICAgICAgfQogICAgICAgIGVsc2UgewogICAgICAgICAgICByZXR1cm4gaW5zdGFuY2U7CiAgICAgICAgfQogICAgfQogICAgZnVuY3Rpb24gZXhwZWN0ZWRSZXNwb25zZVR5cGUodHlwZSkgewogICAgICAgIHN3aXRjaCAodHlwZSkgewogICAgICAgICAgICBjYXNlICdiYXNpYyc6CiAgICAgICAgICAgIGNhc2UgJ2NvcnMnOgogICAgICAgICAgICBjYXNlICdkZWZhdWx0JzogcmV0dXJuIHRydWU7CiAgICAgICAgfQogICAgICAgIHJldHVybiBmYWxzZTsKICAgIH0KfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gSW5pdDoKLy8gX193YmdfaW5pdAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKYXN5bmMgZnVuY3Rpb24gX193YmdfaW5pdChtb2R1bGVfb3JfcGF0aCwgbWVtb3J5KSB7CiAgICBpZiAod2FzbSAhPT0gdW5kZWZpbmVkKQogICAgICAgIHJldHVybiB3YXNtOwogICAgbGV0IHRocmVhZF9zdGFja19zaXplOwogICAgaWYgKG1vZHVsZV9vcl9wYXRoICE9PSB1bmRlZmluZWQpIHsKICAgICAgICBpZiAoT2JqZWN0LmdldFByb3RvdHlwZU9mKG1vZHVsZV9vcl9wYXRoKSA9PT0gT2JqZWN0LnByb3RvdHlwZSkgewogICAgICAgICAgICAoeyBtb2R1bGVfb3JfcGF0aCwgbWVtb3J5LCB0aHJlYWRfc3RhY2tfc2l6ZSB9ID0gbW9kdWxlX29yX3BhdGgpOwogICAgICAgIH0KICAgICAgICBlbHNlIHsKICAgICAgICAgICAgY29uc29sZS53YXJuKCd1c2luZyBkZXByZWNhdGVkIHBhcmFtZXRlcnMgZm9yIHRoZSBpbml0aWFsaXphdGlvbiBmdW5jdGlvbjsgcGFzcyBhIHNpbmdsZSBvYmplY3QgaW5zdGVhZCcpOwogICAgICAgIH0KICAgIH0KICAgIC8vICAgaWYgKG1vZHVsZV9vcl9wYXRoID09PSB1bmRlZmluZWQpIHsKICAgIC8vICAgICBtb2R1bGVfb3JfcGF0aCA9IG5ldyBVUkwoJ3RmaGVfYmcud2FzbScsIGltcG9ydC5tZXRhLnVybCk7CiAgICAvLyAgIH0KICAgIGNvbnN0IGltcG9ydHMgPSBfX3diZ19nZXRfaW1wb3J0cyhtZW1vcnkpOwogICAgLy8gICBpZiAoCiAgICAvLyAgICAgdHlwZW9mIG1vZHVsZV9vcl9wYXRoID09PSAnc3RyaW5nJyB8fAogICAgLy8gICAgICh0eXBlb2YgUmVxdWVzdCA9PT0gJ2Z1bmN0aW9uJyAmJiBtb2R1bGVfb3JfcGF0aCBpbnN0YW5jZW9mIFJlcXVlc3QpIHx8CiAgICAvLyAgICAgKHR5cGVvZiBVUkwgPT09ICdmdW5jdGlvbicgJiYgbW9kdWxlX29yX3BhdGggaW5zdGFuY2VvZiBVUkwpCiAgICAvLyAgICkgewogICAgLy8gICAgIG1vZHVsZV9vcl9wYXRoID0gZmV0Y2gobW9kdWxlX29yX3BhdGgpOwogICAgLy8gICB9CiAgICBjb25zdCB7IGluc3RhbmNlLCBtb2R1bGUgfSA9IGF3YWl0IF9fd2JnX2xvYWQoYXdhaXQgbW9kdWxlX29yX3BhdGgsIGltcG9ydHMpOwogICAgcmV0dXJuIF9fd2JnX2ZpbmFsaXplX2luaXQoaW5zdGFuY2UsIG1vZHVsZSwgdGhyZWFkX3N0YWNrX3NpemUpOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLwovLyBUaGUgJ3RmaGUnIGdsb2JhbCBvYmplY3QKLy8gPT09PT09PT09PT09PT09PT09PT09PT09Ci8vIEZpbmFsIHRmaGUgb2JqZWN0IGdsb2JhbCBkZWNsYXJhdGlvbiBjYWxsZWQgYnkgJ3dhaXRGb3JNc2dUeXBlJyBvbmx5Ci8vCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgp2YXIgdGZoZSA9IC8qI19fUFVSRV9fKi8gT2JqZWN0LmZyZWV6ZSh7CiAgX19wcm90b19fOiBudWxsLAogIGRlZmF1bHQ6IF9fd2JnX2luaXQsCiAgd2JnX3JheW9uX3N0YXJ0X3dvcmtlcjogd2JnX3JheW9uX3N0YXJ0X3dvcmtlciwKfSk7Cgo=";
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
