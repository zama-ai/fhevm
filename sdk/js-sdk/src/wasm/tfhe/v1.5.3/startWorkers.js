/**
 * Auto-generated from scripts/wasm/tfhe/startWorkers.template.js.
 * Embedded worker base64 payload SHA-256: cf41d0b2db75ac8c018bdad46091f42673f3697cf47f50e3469a6a9ee2d93137
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
 *  - Hash is the build-time constant "6e4b745fadd500bb55b0624774053d979b2155258a51bad709d080f3e458d5d1".
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
const _workerUrlSha256 = "6e4b745fadd500bb55b0624774053d979b2155258a51bad709d080f3e458d5d1";
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
 * 1. Fetch the URL and verify its SHA-256 against "6e4b745fadd500bb55b0624774053d979b2155258a51bad709d080f3e458d5d1" — fails fast on mismatch.
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
  const workerBase64 = "YXN5bmMgZnVuY3Rpb24gX19fZ2V0VGFyZ2V0KCkgewogIC8vIFByZWZlciBwYXJlbnRQb3J0IHdoZW5ldmVyIHRoaXMgaXMgYSBub2RlOndvcmtlcl90aHJlYWRzIHdvcmtlci4KICAvLyBgYWRkRXZlbnRMaXN0ZW5lcmAgaXMgbm90IGEgYnJvd3NlciBzaWduYWw6IEJ1biBleHBvc2VzIEV2ZW50VGFyZ2V0IEFQSXMKICAvLyBpbnNpZGUgd29ya2VyX3RocmVhZHMsIGJ1dCBidW4gPj0gMS40IGRlbGl2ZXJzIHBhcmVudCBtZXNzYWdlcyBvbmx5IHRvCiAgLy8gcGFyZW50UG9ydCAoTm9kZSBwYXJpdHkpLiBMaXN0ZW5pbmcgb24gYHNlbGZgIGhhbmdzIHN0YXJ0V29ya2VycygpLgogIHRyeSB7CiAgICBjb25zdCBub2RlTW9kdWxlTmFtZSA9ICd3b3JrZXJfdGhyZWFkcyc7CiAgICBjb25zdCBub2RlTW9kdWxlSWQgPSBgbm9kZToke25vZGVNb2R1bGVOYW1lfWA7CiAgICBjb25zdCB7IHBhcmVudFBvcnQgfSA9IGF3YWl0IGltcG9ydCgvKiBAdml0ZS1pZ25vcmUgKi8gbm9kZU1vZHVsZUlkKTsKICAgIGlmIChwYXJlbnRQb3J0KSByZXR1cm4gcGFyZW50UG9ydDsKICB9IGNhdGNoIHsKICAgIC8vIEJyb3dzZXIgLyBlZGdlOiBub2RlOndvcmtlcl90aHJlYWRzIGlzIG5vdCBpbXBvcnRhYmxlLgogIH0KICByZXR1cm4gc2VsZjsKfQoKZnVuY3Rpb24gX19fd2FpdEZvck1zZ1R5cGUodGFyZ2V0LCB0eXBlKSB7CiAgcmV0dXJuIG5ldyBQcm9taXNlKChyZXNvbHZlKSA9PiB7CiAgICBpZiAodHlwZW9mIHRhcmdldC5vbiA9PT0gJ2Z1bmN0aW9uJykgewogICAgICAvLyBOb2RlOiBFdmVudEVtaXR0ZXIsIGRhdGEgcGFzc2VkIGRpcmVjdGx5CiAgICAgIHRhcmdldC5vbignbWVzc2FnZScsIGZ1bmN0aW9uIG9uTXNnKGRhdGEpIHsKICAgICAgICBpZiAoZGF0YT8udHlwZSAhPT0gdHlwZSkgcmV0dXJuOwogICAgICAgIHRhcmdldC5vZmYoJ21lc3NhZ2UnLCBvbk1zZyk7CiAgICAgICAgcmVzb2x2ZShkYXRhKTsKICAgICAgfSk7CiAgICB9IGVsc2UgewogICAgICAvLyBCcm93c2VyOiBET00gZXZlbnRzLCBkYXRhIHdyYXBwZWQgaW4gTWVzc2FnZUV2ZW50CiAgICAgIHRhcmdldC5hZGRFdmVudExpc3RlbmVyKCdtZXNzYWdlJywgZnVuY3Rpb24gb25Nc2coeyBkYXRhIH0pIHsKICAgICAgICBpZiAoZGF0YT8udHlwZSAhPT0gdHlwZSkgcmV0dXJuOwogICAgICAgIHRhcmdldC5yZW1vdmVFdmVudExpc3RlbmVyKCdtZXNzYWdlJywgb25Nc2cpOwogICAgICAgIHJlc29sdmUoZGF0YSk7CiAgICAgIH0pOwogICAgfQogIH0pOwp9CgpfX19nZXRUYXJnZXQoKS50aGVuKCh0YXJnZXQpID0+CiAgX19fd2FpdEZvck1zZ1R5cGUodGFyZ2V0LCAnd2FzbV9iaW5kZ2VuX3dvcmtlcl9pbml0JykudGhlbigKICAgIGFzeW5jICh7IGluaXQsIHJlY2VpdmVyIH0pID0+IHsKICAgICAgY29uc3QgcGtnID0gYXdhaXQgUHJvbWlzZS5yZXNvbHZlKCkudGhlbihmdW5jdGlvbiAoKSB7CiAgICAgICAgcmV0dXJuIHRmaGU7CiAgICAgIH0pOwogICAgICBhd2FpdCBwa2cuZGVmYXVsdChpbml0KTsKICAgICAgdGFyZ2V0LnBvc3RNZXNzYWdlKHsgdHlwZTogJ3dhc21fYmluZGdlbl93b3JrZXJfcmVhZHknIH0pOwogICAgICBwa2cud2JnX3JheW9uX3N0YXJ0X3dvcmtlcihyZWNlaXZlcik7CiAgICB9LAogICksCik7CgovKioKICogQHBhcmFtIHtudW1iZXJ9IHJlY2VpdmVyCiAqLwpmdW5jdGlvbiB3YmdfcmF5b25fc3RhcnRfd29ya2VyKHJlY2VpdmVyKSB7CiAgd2FzbS53YmdfcmF5b25fc3RhcnRfd29ya2VyKHJlY2VpdmVyKTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gSW50ZXJuYWwgd2FzbWJpbmRnZW4gdG9vbHMKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vCi8vIEltcG9ydHM6Ci8vIF9fd2JnX2dldF9pbXBvcnRzCi8vCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBfX3diZ19nZXRfaW1wb3J0cyhtZW1vcnkpIHsKICAgIGNvbnN0IGltcG9ydDAgPSB7CiAgICAgICAgX19wcm90b19fOiBudWxsLAogICAgICAgIF9fd2JnX0JpZ0ludF83ZWExZTc0OWNlNmI5MmZkOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIHJldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICAgICAgY29uc3QgcmV0ID0gQmlnSW50KGFyZzApOwogICAgICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICAgICAgfSwgYXJndW1lbnRzKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX0JpZ0ludF9iN2JiY2NkZmYyNThjOWYyOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBCaWdJbnQoYXJnMCk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19FcnJvcl84YzRlNDNmZTc0NTU5ZDczOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBFcnJvcihnZXRTdHJpbmdGcm9tV2FzbTAoYXJnMCwgYXJnMSkpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9iaWdpbnRfZ2V0X2FzX2k2NF84ZmNmNGNlN2YxY2E3MmEyOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCB2ID0gYXJnMTsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mICh2KSA9PT0gJ2JpZ2ludCcgPyB2IDogdW5kZWZpbmVkOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRCaWdJbnQ2NChhcmcwICsgOCAqIDEsIGlzTGlrZU5vbmUocmV0KSA/IEJpZ0ludCgwKSA6IHJldCwgdHJ1ZSk7CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMCwgIWlzTGlrZU5vbmUocmV0KSwgdHJ1ZSk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2JpdF9hbmRfNDM2MmIxMTc2OTUwZDQyYTogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMCAmIGFyZzE7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2JpdF9vcl83MWEwMmQzOTc5NmVhMTNkOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwIHwgYXJnMTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fZGVidWdfc3RyaW5nXzBiYzg0ODJjNmUzNTA4YWU6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGRlYnVnU3RyaW5nKGFyZzEpOwogICAgICAgICAgICBjb25zdCBwdHIxID0gcGFzc1N0cmluZ1RvV2FzbTAocmV0LCB3YXNtLl9fd2JpbmRnZW5fbWFsbG9jLCB3YXNtLl9fd2JpbmRnZW5fcmVhbGxvYyk7CiAgICAgICAgICAgIGNvbnN0IGxlbjEgPSBXQVNNX1ZFQ1RPUl9MRU47CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMSwgbGVuMSwgdHJ1ZSk7CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMCwgcHRyMSwgdHJ1ZSk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2lzX2Z1bmN0aW9uXzAwOTVhNzNiOGIxNTZmNzY6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiAoYXJnMCkgPT09ICdmdW5jdGlvbic7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX2lzX29iamVjdF81YWU4ZTU4ODBmMmMxZmJkOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCB2YWwgPSBhcmcwOwogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgKHZhbCkgPT09ICdvYmplY3QnICYmIHZhbCAhPT0gbnVsbDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5faXNfc3RyaW5nX2NkNDQ0NTE2ZWRjNWIxODA6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiAoYXJnMCkgPT09ICdzdHJpbmcnOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9pc191bmRlZmluZWRfOWU0ZDkyNTM0YzQyZDc3ODogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMCA9PT0gdW5kZWZpbmVkOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9qc3ZhbF9lcV8xMTg4ODM5MGIwMTg2MjcwOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwID09PSBhcmcxOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfX193YmluZGdlbl9sdF9iYjU5Y2MzZDIzNTI2ZTBkOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwIDwgYXJnMTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fbWVtb3J5X2JkMWZiY2YyMWZiZWYzYzg6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gd2FzbS5tZW1vcnk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX21vZHVsZV9mNmI4MDUyZDc5YzFjYzE2OiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHdhc21Nb2R1bGU7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX25lZ182YjRkMzU2ZGZmNDlkY2M2OiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSAtYXJnMDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fc2hsXzhkNjRkMDY3NjFmOWVhNGU6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAgPDwgYXJnMTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fc2hyX2VmOGUwN2NjZTcwOWViNTQ6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAgPj4gYXJnMTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX19fd2JpbmRnZW5fc3RyaW5nX2dldF83MmZiNjk2MjAyYzU2NzI5OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBjb25zdCBvYmogPSBhcmcxOwogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgKG9iaikgPT09ICdzdHJpbmcnID8gb2JqIDogdW5kZWZpbmVkOwogICAgICAgICAgICB2YXIgcHRyMSA9IGlzTGlrZU5vbmUocmV0KSA/IDAgOiBwYXNzU3RyaW5nVG9XYXNtMChyZXQsIHdhc20uX193YmluZGdlbl9tYWxsb2MsIHdhc20uX193YmluZGdlbl9yZWFsbG9jKTsKICAgICAgICAgICAgdmFyIGxlbjEgPSBXQVNNX1ZFQ1RPUl9MRU47CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMSwgbGVuMSwgdHJ1ZSk7CiAgICAgICAgICAgIGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGFyZzAgKyA0ICogMCwgcHRyMSwgdHJ1ZSk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19fX3diaW5kZ2VuX3Rocm93X2JlMjg5ZDUwMzRlZDI3MWI6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIHRocm93IG5ldyBFcnJvcihnZXRTdHJpbmdGcm9tV2FzbTAoYXJnMCwgYXJnMSkpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfY2FsbF8zODllZmUyODQzNWE5Mzg4OiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIHJldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5jYWxsKGFyZzEpOwogICAgICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICAgICAgfSwgYXJndW1lbnRzKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX2NhbGxfNDcwOGUwYzEzYmRjOGU5NTogZnVuY3Rpb24gKCkgewogICAgICAgICAgICByZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24gKGFyZzAsIGFyZzEsIGFyZzIpIHsKICAgICAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAuY2FsbChhcmcxLCBhcmcyKTsKICAgICAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgICAgIH0sIGFyZ3VtZW50cyk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19jcnlwdG9fODZmMjYzMWU5MWI1MTUxMTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC5jcnlwdG87CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19lcnJvcl83NTM0YjhlOWEzNmYxYWI0OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICBsZXQgZGVmZXJyZWQwXzA7CiAgICAgICAgICAgIGxldCBkZWZlcnJlZDBfMTsKICAgICAgICAgICAgdHJ5IHsKICAgICAgICAgICAgICAgIGRlZmVycmVkMF8wID0gYXJnMDsKICAgICAgICAgICAgICAgIGRlZmVycmVkMF8xID0gYXJnMTsKICAgICAgICAgICAgICAgIGNvbnNvbGUuZXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGFyZzAsIGFyZzEpKTsKICAgICAgICAgICAgfQogICAgICAgICAgICBmaW5hbGx5IHsKICAgICAgICAgICAgICAgIHdhc20uX193YmluZGdlbl9mcmVlKGRlZmVycmVkMF8wLCBkZWZlcnJlZDBfMSwgMSk7CiAgICAgICAgICAgIH0KICAgICAgICB9LAogICAgICAgIF9fd2JnX2dldFJhbmRvbVZhbHVlc19iM2YxNWZjYmZhYmIwZjhiOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIHJldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICAgICAgYXJnMC5nZXRSYW5kb21WYWx1ZXMoYXJnMSk7CiAgICAgICAgICAgIH0sIGFyZ3VtZW50cyk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19nZXRUaW1lXzFlM2NkMTM5MWM1YzM5OTU6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAuZ2V0VGltZSgpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfaW5zdGFuY2VvZl9XaW5kb3dfZWQ0OWIyZGI4ZGY5MDM1OTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgbGV0IHJlc3VsdDsKICAgICAgICAgICAgdHJ5IHsKICAgICAgICAgICAgICAgIHJlc3VsdCA9IGFyZzAgaW5zdGFuY2VvZiBXaW5kb3c7CiAgICAgICAgICAgIH0KICAgICAgICAgICAgY2F0Y2ggKF8pIHsKICAgICAgICAgICAgICAgIHJlc3VsdCA9IGZhbHNlOwogICAgICAgICAgICB9CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHJlc3VsdDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX2xlbmd0aF8zMmVkOWEyNzlhY2QwNTRjOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLmxlbmd0aDsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX21zQ3J5cHRvX2Q1NjJiYmU4M2UwZDRiOTE6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAubXNDcnlwdG87CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19uZXdfMF83M2FmYzM1ZWI1NDRlNTM5OiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IG5ldyBEYXRlKCk7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19uZXdfOGE2ZjIzOGE2ZWNlODZlYTogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBuZXcgRXJyb3IoKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX25ld19ub19hcmdzXzFjN2M4NDJmMDhkMDBlYmI6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IG5ldyBGdW5jdGlvbihnZXRTdHJpbmdGcm9tV2FzbTAoYXJnMCwgYXJnMSkpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfbmV3X3dpdGhfbGVuZ3RoX2EyYzM5Y2JlODhmZDhmZjE6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IG5ldyBVaW50OEFycmF5KGFyZzAgPj4+IDApOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193Ymdfbm9kZV9lMWYyNGY4OWE3MzM2YzJlOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLm5vZGU7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19wcm9jZXNzXzM5NzVmZDZjNzJmNTIwYWE6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzAucHJvY2VzczsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3Byb3RvdHlwZXNldGNhbGxfYmRjZGNjNTg0MmU0ZDc3ZDogZnVuY3Rpb24gKGFyZzAsIGFyZzEsIGFyZzIpIHsKICAgICAgICAgICAgVWludDhBcnJheS5wcm90b3R5cGUuc2V0LmNhbGwoZ2V0QXJyYXlVOEZyb21XYXNtMChhcmcwLCBhcmcxKSwgYXJnMik7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19yYW5kb21GaWxsU3luY19mOGMxNTNiNzlmMjg1ODE3OiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIHJldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICAgICAgYXJnMC5yYW5kb21GaWxsU3luYyhhcmcxKTsKICAgICAgICAgICAgfSwgYXJndW1lbnRzKTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3JlcXVpcmVfYjc0ZjQ3ZmMyZDAyMmZkNjogZnVuY3Rpb24gKCkgewogICAgICAgICAgICByZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24gKCkgewogICAgICAgICAgICAgICAgY29uc3QgcmV0ID0gbW9kdWxlLnJlcXVpcmU7CiAgICAgICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgICAgICB9LCBhcmd1bWVudHMpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3RhY2tfMGVkNzVkNjg1NzViMGYzYzogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMS5zdGFjazsKICAgICAgICAgICAgY29uc3QgcHRyMSA9IHBhc3NTdHJpbmdUb1dhc20wKHJldCwgd2FzbS5fX3diaW5kZ2VuX21hbGxvYywgd2FzbS5fX3diaW5kZ2VuX3JlYWxsb2MpOwogICAgICAgICAgICBjb25zdCBsZW4xID0gV0FTTV9WRUNUT1JfTEVOOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDEsIGxlbjEsIHRydWUpOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDAsIHB0cjEsIHRydWUpOwogICAgICAgIH0sCiAgICAgICAgX193Ymdfc3RhcnRXb3JrZXJzXzJjYTExNzYxZTA4ZmY1ZDU6IGZ1bmN0aW9uIChhcmcwLCBhcmcxLCBhcmcyKSB7CiAgICAgICAgICAgIGhhbmRsZUVycm9yKGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgICAgIHRocm93IG5ldyBFcnJvcignc3RhcnRXb3JrZXJzIG5vdCBzdXBwb3J0ZWQgZnJvbSBhIHdvcmtlciB0aHJlYWQnKTsKICAgICAgICAgICAgfSk7CiAgICAgICAgICAgIC8vIGNvbnN0IHJldCA9IHN0YXJ0V29ya2VycyhhcmcwLCBhcmcxLCB3YmdfcmF5b25fUG9vbEJ1aWxkZXIuX193cmFwKGFyZzIpKTsKICAgICAgICAgICAgLy8gcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3N0YXRpY19hY2Nlc3Nvcl9HTE9CQUxfMTI4MzcxNjdhZDkzNTExNjogZnVuY3Rpb24gKCkgewogICAgICAgICAgICBjb25zdCByZXQgPSB0eXBlb2YgZ2xvYmFsID09PSAndW5kZWZpbmVkJyA/IG51bGwgOiBnbG9iYWw7CiAgICAgICAgICAgIHJldHVybiBpc0xpa2VOb25lKHJldCkgPyAwIDogYWRkVG9FeHRlcm5yZWZUYWJsZTAocmV0KTsKICAgICAgICB9LAogICAgICAgIF9fd2JnX3N0YXRpY19hY2Nlc3Nvcl9HTE9CQUxfVEhJU19lNjI4ZTg5YWIzYjFjOTVmOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiBnbG9iYWxUaGlzID09PSAndW5kZWZpbmVkJyA/IG51bGwgOiBnbG9iYWxUaGlzOwogICAgICAgICAgICByZXR1cm4gaXNMaWtlTm9uZShyZXQpID8gMCA6IGFkZFRvRXh0ZXJucmVmVGFibGUwKHJldCk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdGF0aWNfYWNjZXNzb3JfU0VMRl9hNjIxZDNkZmJiNjBkMGNlOiBmdW5jdGlvbiAoKSB7CiAgICAgICAgICAgIGNvbnN0IHJldCA9IHR5cGVvZiBzZWxmID09PSAndW5kZWZpbmVkJyA/IG51bGwgOiBzZWxmOwogICAgICAgICAgICByZXR1cm4gaXNMaWtlTm9uZShyZXQpID8gMCA6IGFkZFRvRXh0ZXJucmVmVGFibGUwKHJldCk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdGF0aWNfYWNjZXNzb3JfV0lORE9XX2Y4NzI3ZjBjZjg4OGUwYmQ6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gdHlwZW9mIHdpbmRvdyA9PT0gJ3VuZGVmaW5lZCcgPyBudWxsIDogd2luZG93OwogICAgICAgICAgICByZXR1cm4gaXNMaWtlTm9uZShyZXQpID8gMCA6IGFkZFRvRXh0ZXJucmVmVGFibGUwKHJldCk7CiAgICAgICAgfSwKICAgICAgICBfX3diZ19zdWJhcnJheV9hOTZlMWZlZjE3ZWQyM2NiOiBmdW5jdGlvbiAoYXJnMCwgYXJnMSwgYXJnMikgewogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwLnN1YmFycmF5KGFyZzEgPj4+IDAsIGFyZzIgPj4+IDApOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfdG9TdHJpbmdfMDI5YWMyNDQyMWZkN2EyNDogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC50b1N0cmluZygpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmdfdG9TdHJpbmdfNTZkOTQ2ZGFmZjgzODY3YjogZnVuY3Rpb24gKGFyZzAsIGFyZzEsIGFyZzIpIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMS50b1N0cmluZyhhcmcyKTsKICAgICAgICAgICAgY29uc3QgcHRyMSA9IHBhc3NTdHJpbmdUb1dhc20wKHJldCwgd2FzbS5fX3diaW5kZ2VuX21hbGxvYywgd2FzbS5fX3diaW5kZ2VuX3JlYWxsb2MpOwogICAgICAgICAgICBjb25zdCBsZW4xID0gV0FTTV9WRUNUT1JfTEVOOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDEsIGxlbjEsIHRydWUpOwogICAgICAgICAgICBnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihhcmcwICsgNCAqIDAsIHB0cjEsIHRydWUpOwogICAgICAgIH0sCiAgICAgICAgX193YmdfdmVyc2lvbnNfNGUzMTIyNmY1ZThkYzkwOTogZnVuY3Rpb24gKGFyZzApIHsKICAgICAgICAgICAgY29uc3QgcmV0ID0gYXJnMC52ZXJzaW9uczsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDAxOiBmdW5jdGlvbiAoYXJnMCkgewogICAgICAgICAgICAvLyBDYXN0IGludHJpbnNpYyBmb3IgYEY2NCAtPiBFeHRlcm5yZWZgLgogICAgICAgICAgICBjb25zdCByZXQgPSBhcmcwOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDI6IGZ1bmN0aW9uIChhcmcwLCBhcmcxKSB7CiAgICAgICAgICAgIC8vIENhc3QgaW50cmluc2ljIGZvciBgSTEyOCAtPiBFeHRlcm5yZWZgLgogICAgICAgICAgICBjb25zdCByZXQgPSAoQmlnSW50LmFzVWludE4oNjQsIGFyZzApIHwgKGFyZzEgPDwgQmlnSW50KDY0KSkpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDM6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIC8vIENhc3QgaW50cmluc2ljIGZvciBgSTY0IC0+IEV4dGVybnJlZmAuCiAgICAgICAgICAgIGNvbnN0IHJldCA9IGFyZzA7CiAgICAgICAgICAgIHJldHVybiByZXQ7CiAgICAgICAgfSwKICAgICAgICBfX3diaW5kZ2VuX2Nhc3RfMDAwMDAwMDAwMDAwMDAwNDogZnVuY3Rpb24gKGFyZzAsIGFyZzEpIHsKICAgICAgICAgICAgLy8gQ2FzdCBpbnRyaW5zaWMgZm9yIGBSZWYoU2xpY2UoVTgpKSAtPiBOYW1lZEV4dGVybnJlZigiVWludDhBcnJheSIpYC4KICAgICAgICAgICAgY29uc3QgcmV0ID0gZ2V0QXJyYXlVOEZyb21XYXNtMChhcmcwLCBhcmcxKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDA1OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICAvLyBDYXN0IGludHJpbnNpYyBmb3IgYFJlZihTdHJpbmcpIC0+IEV4dGVybnJlZmAuCiAgICAgICAgICAgIGNvbnN0IHJldCA9IGdldFN0cmluZ0Zyb21XYXNtMChhcmcwLCBhcmcxKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDA2OiBmdW5jdGlvbiAoYXJnMCwgYXJnMSkgewogICAgICAgICAgICAvLyBDYXN0IGludHJpbnNpYyBmb3IgYFUxMjggLT4gRXh0ZXJucmVmYC4KICAgICAgICAgICAgY29uc3QgcmV0ID0gKEJpZ0ludC5hc1VpbnROKDY0LCBhcmcwKSB8IChCaWdJbnQuYXNVaW50Tig2NCwgYXJnMSkgPDwgQmlnSW50KDY0KSkpOwogICAgICAgICAgICByZXR1cm4gcmV0OwogICAgICAgIH0sCiAgICAgICAgX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDc6IGZ1bmN0aW9uIChhcmcwKSB7CiAgICAgICAgICAgIC8vIENhc3QgaW50cmluc2ljIGZvciBgVTY0IC0+IEV4dGVybnJlZmAuCiAgICAgICAgICAgIGNvbnN0IHJldCA9IEJpZ0ludC5hc1VpbnROKDY0LCBhcmcwKTsKICAgICAgICAgICAgcmV0dXJuIHJldDsKICAgICAgICB9LAogICAgICAgIF9fd2JpbmRnZW5faW5pdF9leHRlcm5yZWZfdGFibGU6IGZ1bmN0aW9uICgpIHsKICAgICAgICAgICAgY29uc3QgdGFibGUgPSB3YXNtLl9fd2JpbmRnZW5fZXh0ZXJucmVmczsKICAgICAgICAgICAgY29uc3Qgb2Zmc2V0ID0gdGFibGUuZ3Jvdyg0KTsKICAgICAgICAgICAgdGFibGUuc2V0KDAsIHVuZGVmaW5lZCk7CiAgICAgICAgICAgIHRhYmxlLnNldChvZmZzZXQgKyAwLCB1bmRlZmluZWQpOwogICAgICAgICAgICB0YWJsZS5zZXQob2Zmc2V0ICsgMSwgbnVsbCk7CiAgICAgICAgICAgIHRhYmxlLnNldChvZmZzZXQgKyAyLCB0cnVlKTsKICAgICAgICAgICAgdGFibGUuc2V0KG9mZnNldCArIDMsIGZhbHNlKTsKICAgICAgICB9LAogICAgICAgIG1lbW9yeTogbWVtb3J5IHx8IG5ldyBXZWJBc3NlbWJseS5NZW1vcnkoeyBpbml0aWFsOiAyMSwgbWF4aW11bTogMTYzODQsIHNoYXJlZDogdHJ1ZSB9KSwKICAgIH07CiAgICByZXR1cm4gewogICAgICAgIF9fcHJvdG9fXzogbnVsbCwKICAgICAgICAiLi90ZmhlX2JnLmpzIjogaW1wb3J0MCwKICAgIH07Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGFkZFRvRXh0ZXJucmVmVGFibGUwCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBhZGRUb0V4dGVybnJlZlRhYmxlMChvYmopIHsKICAgIGNvbnN0IGlkeCA9IHdhc20uX19leHRlcm5yZWZfdGFibGVfYWxsb2MoKTsKICAgIHdhc20uX193YmluZGdlbl9leHRlcm5yZWZzLnNldChpZHgsIG9iaik7CiAgICByZXR1cm4gaWR4Owp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBkZWJ1Z1N0cmluZwovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gZGVidWdTdHJpbmcodmFsKSB7CiAgICAvLyBwcmltaXRpdmUgdHlwZXMKICAgIGNvbnN0IHR5cGUgPSB0eXBlb2YgdmFsOwogICAgaWYgKHR5cGUgPT0gJ251bWJlcicgfHwgdHlwZSA9PSAnYm9vbGVhbicgfHwgdmFsID09IG51bGwpIHsKICAgICAgICByZXR1cm4gYCR7dmFsfWA7CiAgICB9CiAgICBpZiAodHlwZSA9PSAnc3RyaW5nJykgewogICAgICAgIHJldHVybiBgIiR7dmFsfSJgOwogICAgfQogICAgaWYgKHR5cGUgPT0gJ3N5bWJvbCcpIHsKICAgICAgICBjb25zdCBkZXNjcmlwdGlvbiA9IHZhbC5kZXNjcmlwdGlvbjsKICAgICAgICBpZiAoZGVzY3JpcHRpb24gPT0gbnVsbCkgewogICAgICAgICAgICByZXR1cm4gJ1N5bWJvbCc7CiAgICAgICAgfQogICAgICAgIGVsc2UgewogICAgICAgICAgICByZXR1cm4gYFN5bWJvbCgke2Rlc2NyaXB0aW9ufSlgOwogICAgICAgIH0KICAgIH0KICAgIGlmICh0eXBlID09ICdmdW5jdGlvbicpIHsKICAgICAgICBjb25zdCBuYW1lID0gdmFsLm5hbWU7CiAgICAgICAgaWYgKHR5cGVvZiBuYW1lID09ICdzdHJpbmcnICYmIG5hbWUubGVuZ3RoID4gMCkgewogICAgICAgICAgICByZXR1cm4gYEZ1bmN0aW9uKCR7bmFtZX0pYDsKICAgICAgICB9CiAgICAgICAgZWxzZSB7CiAgICAgICAgICAgIHJldHVybiAnRnVuY3Rpb24nOwogICAgICAgIH0KICAgIH0KICAgIC8vIG9iamVjdHMKICAgIGlmIChBcnJheS5pc0FycmF5KHZhbCkpIHsKICAgICAgICBjb25zdCBsZW5ndGggPSB2YWwubGVuZ3RoOwogICAgICAgIGxldCBkZWJ1ZyA9ICdbJzsKICAgICAgICBpZiAobGVuZ3RoID4gMCkgewogICAgICAgICAgICBkZWJ1ZyArPSBkZWJ1Z1N0cmluZyh2YWxbMF0pOwogICAgICAgIH0KICAgICAgICBmb3IgKGxldCBpID0gMTsgaSA8IGxlbmd0aDsgaSsrKSB7CiAgICAgICAgICAgIGRlYnVnICs9ICcsICcgKyBkZWJ1Z1N0cmluZyh2YWxbaV0pOwogICAgICAgIH0KICAgICAgICBkZWJ1ZyArPSAnXSc7CiAgICAgICAgcmV0dXJuIGRlYnVnOwogICAgfQogICAgLy8gVGVzdCBmb3IgYnVpbHQtaW4KICAgIGNvbnN0IGJ1aWx0SW5NYXRjaGVzID0gL1xbb2JqZWN0IChbXlxdXSspXF0vLmV4ZWModG9TdHJpbmcuY2FsbCh2YWwpKTsKICAgIGxldCBjbGFzc05hbWU7CiAgICBpZiAoYnVpbHRJbk1hdGNoZXMgJiYgYnVpbHRJbk1hdGNoZXMubGVuZ3RoID4gMSkgewogICAgICAgIGNsYXNzTmFtZSA9IGJ1aWx0SW5NYXRjaGVzWzFdOwogICAgfQogICAgZWxzZSB7CiAgICAgICAgLy8gRmFpbGVkIHRvIG1hdGNoIHRoZSBzdGFuZGFyZCAnW29iamVjdCBDbGFzc05hbWVdJwogICAgICAgIHJldHVybiB0b1N0cmluZy5jYWxsKHZhbCk7CiAgICB9CiAgICBpZiAoY2xhc3NOYW1lID09ICdPYmplY3QnKSB7CiAgICAgICAgLy8gd2UncmUgYSB1c2VyIGRlZmluZWQgY2xhc3Mgb3IgT2JqZWN0CiAgICAgICAgLy8gSlNPTi5zdHJpbmdpZnkgYXZvaWRzIHByb2JsZW1zIHdpdGggY3ljbGVzLCBhbmQgaXMgZ2VuZXJhbGx5IG11Y2gKICAgICAgICAvLyBlYXNpZXIgdGhhbiBsb29waW5nIHRocm91Z2ggb3duUHJvcGVydGllcyBvZiBgdmFsYC4KICAgICAgICB0cnkgewogICAgICAgICAgICByZXR1cm4gJ09iamVjdCgnICsgSlNPTi5zdHJpbmdpZnkodmFsKSArICcpJzsKICAgICAgICB9CiAgICAgICAgY2F0Y2ggKF8pIHsKICAgICAgICAgICAgcmV0dXJuICdPYmplY3QnOwogICAgICAgIH0KICAgIH0KICAgIC8vIGVycm9ycwogICAgaWYgKHZhbCBpbnN0YW5jZW9mIEVycm9yKSB7CiAgICAgICAgcmV0dXJuIGAke3ZhbC5uYW1lfTogJHt2YWwubWVzc2FnZX1cbiR7dmFsLnN0YWNrfWA7CiAgICB9CiAgICAvLyBUT0RPIHdlIGNvdWxkIHRlc3QgZm9yIG1vcmUgdGhpbmdzIGhlcmUsIGxpa2UgYFNldGBzIGFuZCBgTWFwYHMuCiAgICByZXR1cm4gY2xhc3NOYW1lOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBnZXRBcnJheVU4RnJvbVdhc20wCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBnZXRBcnJheVU4RnJvbVdhc20wKHB0ciwgbGVuKSB7CiAgICBwdHIgPSBwdHIgPj4+IDA7CiAgICByZXR1cm4gZ2V0VWludDhBcnJheU1lbW9yeTAoKS5zdWJhcnJheShwdHIgLyAxLCBwdHIgLyAxICsgbGVuKTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gY2FjaGVkRGF0YVZpZXdNZW1vcnkwCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpsZXQgY2FjaGVkRGF0YVZpZXdNZW1vcnkwID0gbnVsbDsKCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGdldERhdGFWaWV3TWVtb3J5MAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gZ2V0RGF0YVZpZXdNZW1vcnkwKCkgewogICAgaWYgKGNhY2hlZERhdGFWaWV3TWVtb3J5MCA9PT0gbnVsbCB8fCBjYWNoZWREYXRhVmlld01lbW9yeTAuYnVmZmVyICE9PSB3YXNtLm1lbW9yeS5idWZmZXIpIHsKICAgICAgICBjYWNoZWREYXRhVmlld01lbW9yeTAgPSBuZXcgRGF0YVZpZXcod2FzbS5tZW1vcnkuYnVmZmVyKTsKICAgIH0KICAgIHJldHVybiBjYWNoZWREYXRhVmlld01lbW9yeTA7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGdldFN0cmluZ0Zyb21XYXNtMAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gZ2V0U3RyaW5nRnJvbVdhc20wKHB0ciwgbGVuKSB7CiAgICBwdHIgPSBwdHIgPj4+IDA7CiAgICByZXR1cm4gZGVjb2RlVGV4dChwdHIsIGxlbik7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpsZXQgY2FjaGVkVWludDhBcnJheU1lbW9yeTAgPSBudWxsOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gZ2V0VWludDhBcnJheU1lbW9yeTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGdldFVpbnQ4QXJyYXlNZW1vcnkwKCkgewogICAgaWYgKGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwID09PSBudWxsIHx8IGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwLmJ1ZmZlciAhPT0gd2FzbS5tZW1vcnkuYnVmZmVyKSB7CiAgICAgICAgY2FjaGVkVWludDhBcnJheU1lbW9yeTAgPSBuZXcgVWludDhBcnJheSh3YXNtLm1lbW9yeS5idWZmZXIpOwogICAgfQogICAgcmV0dXJuIGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBoYW5kbGVFcnJvcgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKZnVuY3Rpb24gaGFuZGxlRXJyb3IoZiwgYXJncykgewogICAgdHJ5IHsKICAgICAgICByZXR1cm4gZi5hcHBseSh0aGlzLCBhcmdzKTsKICAgIH0KICAgIGNhdGNoIChlKSB7CiAgICAgICAgY29uc3QgaWR4ID0gYWRkVG9FeHRlcm5yZWZUYWJsZTAoZSk7CiAgICAgICAgd2FzbS5fX3diaW5kZ2VuX2V4bl9zdG9yZShpZHgpOwogICAgfQp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBpc0xpa2VOb25lCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBpc0xpa2VOb25lKHgpIHsKICAgIHJldHVybiB4ID09PSB1bmRlZmluZWQgfHwgeCA9PT0gbnVsbDsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gcGFzc1N0cmluZ1RvV2FzbTAKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIHBhc3NTdHJpbmdUb1dhc20wKGFyZywgbWFsbG9jLCByZWFsbG9jKSB7CiAgICBpZiAocmVhbGxvYyA9PT0gdW5kZWZpbmVkKSB7CiAgICAgICAgY29uc3QgYnVmID0gY2FjaGVkVGV4dEVuY29kZXIuZW5jb2RlKGFyZyk7CiAgICAgICAgY29uc3QgcHRyID0gbWFsbG9jKGJ1Zi5sZW5ndGgsIDEpID4+PiAwOwogICAgICAgIGdldFVpbnQ4QXJyYXlNZW1vcnkwKCkuc3ViYXJyYXkocHRyLCBwdHIgKyBidWYubGVuZ3RoKS5zZXQoYnVmKTsKICAgICAgICBXQVNNX1ZFQ1RPUl9MRU4gPSBidWYubGVuZ3RoOwogICAgICAgIHJldHVybiBwdHI7CiAgICB9CiAgICBsZXQgbGVuID0gYXJnLmxlbmd0aDsKICAgIGxldCBwdHIgPSBtYWxsb2MobGVuLCAxKSA+Pj4gMDsKICAgIGNvbnN0IG1lbSA9IGdldFVpbnQ4QXJyYXlNZW1vcnkwKCk7CiAgICBsZXQgb2Zmc2V0ID0gMDsKICAgIGZvciAoOyBvZmZzZXQgPCBsZW47IG9mZnNldCsrKSB7CiAgICAgICAgY29uc3QgY29kZSA9IGFyZy5jaGFyQ29kZUF0KG9mZnNldCk7CiAgICAgICAgaWYgKGNvZGUgPiAweDdGKQogICAgICAgICAgICBicmVhazsKICAgICAgICBtZW1bcHRyICsgb2Zmc2V0XSA9IGNvZGU7CiAgICB9CiAgICBpZiAob2Zmc2V0ICE9PSBsZW4pIHsKICAgICAgICBpZiAob2Zmc2V0ICE9PSAwKSB7CiAgICAgICAgICAgIGFyZyA9IGFyZy5zbGljZShvZmZzZXQpOwogICAgICAgIH0KICAgICAgICBwdHIgPSByZWFsbG9jKHB0ciwgbGVuLCBsZW4gPSBvZmZzZXQgKyBhcmcubGVuZ3RoICogMywgMSkgPj4+IDA7CiAgICAgICAgY29uc3QgdmlldyA9IGdldFVpbnQ4QXJyYXlNZW1vcnkwKCkuc3ViYXJyYXkocHRyICsgb2Zmc2V0LCBwdHIgKyBsZW4pOwogICAgICAgIGNvbnN0IHJldCA9IGNhY2hlZFRleHRFbmNvZGVyLmVuY29kZUludG8oYXJnLCB2aWV3KTsKICAgICAgICBvZmZzZXQgKz0gcmV0LndyaXR0ZW47CiAgICAgICAgcHRyID0gcmVhbGxvYyhwdHIsIGxlbiwgb2Zmc2V0LCAxKSA+Pj4gMDsKICAgIH0KICAgIFdBU01fVkVDVE9SX0xFTiA9IG9mZnNldDsKICAgIHJldHVybiBwdHI7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGNhY2hlZFRleHREZWNvZGVyCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpsZXQgY2FjaGVkVGV4dERlY29kZXIgPSAodHlwZW9mIFRleHREZWNvZGVyICE9PSAndW5kZWZpbmVkJyA/IG5ldyBUZXh0RGVjb2RlcigndXRmLTgnLCB7IGlnbm9yZUJPTTogdHJ1ZSwgZmF0YWw6IHRydWUgfSkgOiB1bmRlZmluZWQpOwoKaWYgKGNhY2hlZFRleHREZWNvZGVyKQogICAgY2FjaGVkVGV4dERlY29kZXIuZGVjb2RlKCk7CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBNQVhfU0FGQVJJX0RFQ09ERV9CWVRFUwovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKY29uc3QgTUFYX1NBRkFSSV9ERUNPREVfQllURVMgPSAyMTQ2NDM1MDcyOwoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gbnVtQnl0ZXNEZWNvZGVkCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpsZXQgbnVtQnl0ZXNEZWNvZGVkID0gMDsKCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vIGRlY29kZVRleHQKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCmZ1bmN0aW9uIGRlY29kZVRleHQocHRyLCBsZW4pIHsKICAgIG51bUJ5dGVzRGVjb2RlZCArPSBsZW47CiAgICBpZiAobnVtQnl0ZXNEZWNvZGVkID49IE1BWF9TQUZBUklfREVDT0RFX0JZVEVTKSB7CiAgICAgICAgY2FjaGVkVGV4dERlY29kZXIgPSBuZXcgVGV4dERlY29kZXIoJ3V0Zi04JywgeyBpZ25vcmVCT006IHRydWUsIGZhdGFsOiB0cnVlIH0pOwogICAgICAgIGNhY2hlZFRleHREZWNvZGVyLmRlY29kZSgpOwogICAgICAgIG51bUJ5dGVzRGVjb2RlZCA9IGxlbjsKICAgIH0KICAgIHJldHVybiBjYWNoZWRUZXh0RGVjb2Rlci5kZWNvZGUoZ2V0VWludDhBcnJheU1lbW9yeTAoKS5zbGljZShwdHIsIHB0ciArIGxlbikpOwp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBjYWNoZWRUZXh0RW5jb2RlcgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKY29uc3QgY2FjaGVkVGV4dEVuY29kZXIgPSAodHlwZW9mIFRleHRFbmNvZGVyICE9PSAndW5kZWZpbmVkJyA/IG5ldyBUZXh0RW5jb2RlcigpIDogdW5kZWZpbmVkKTsKCmlmIChjYWNoZWRUZXh0RW5jb2RlcikgewogICAgY2FjaGVkVGV4dEVuY29kZXIuZW5jb2RlSW50byA9IGZ1bmN0aW9uIChhcmcsIHZpZXcpIHsKICAgICAgICBjb25zdCBidWYgPSBjYWNoZWRUZXh0RW5jb2Rlci5lbmNvZGUoYXJnKTsKICAgICAgICB2aWV3LnNldChidWYpOwogICAgICAgIHJldHVybiB7CiAgICAgICAgICAgIHJlYWQ6IGFyZy5sZW5ndGgsCiAgICAgICAgICAgIHdyaXR0ZW46IGJ1Zi5sZW5ndGgKICAgICAgICB9OwogICAgfTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gV0FTTV9WRUNUT1JfTEVOIGlzIGEgbW9kdWxlLWxldmVsIHZhcmlhYmxlIHRoYXQgc3RvcmVzIHRoZSBieXRlIGxlbmd0aCBvZgovLyB0aGUgZGF0YSBqdXN0IHdyaXR0ZW4gaW50byBXQVNNIG1lbW9yeS4gSXQgYWN0cyBhcyBhbiBvdXQtcGFyYW1ldGVyLgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IFdBU01fVkVDVE9SX0xFTiA9IDA7CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBXQVNNIG1vZHVsZSBzdGF0ZQovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKbGV0IHdhc21Nb2R1bGUsIHdhc207CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBJbml0OgovLyBfX3diZ19maW5hbGl6ZV9pbml0Ci8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgpmdW5jdGlvbiBfX3diZ19maW5hbGl6ZV9pbml0KGluc3RhbmNlLCBtb2R1bGUsIHRocmVhZF9zdGFja19zaXplKSB7CiAgICB3YXNtID0gaW5zdGFuY2UuZXhwb3J0czsKICAgIHdhc21Nb2R1bGUgPSBtb2R1bGU7CiAgICBjYWNoZWREYXRhVmlld01lbW9yeTAgPSBudWxsOwogICAgY2FjaGVkVWludDhBcnJheU1lbW9yeTAgPSBudWxsOwogICAgaWYgKHR5cGVvZiB0aHJlYWRfc3RhY2tfc2l6ZSAhPT0gJ3VuZGVmaW5lZCcgJiYgKHR5cGVvZiB0aHJlYWRfc3RhY2tfc2l6ZSAhPT0gJ251bWJlcicgfHwgdGhyZWFkX3N0YWNrX3NpemUgPT09IDAgfHwgdGhyZWFkX3N0YWNrX3NpemUgJSA2NTUzNiAhPT0gMCkpIHsKICAgICAgICB0aHJvdyAnaW52YWxpZCBzdGFjayBzaXplJzsKICAgIH0KICAgIHdhc20uX193YmluZGdlbl9zdGFydCh0aHJlYWRfc3RhY2tfc2l6ZSk7CiAgICByZXR1cm4gd2FzbTsKfQoKLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KLy8gSW5pdDoKLy8gX193YmdfbG9hZAovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwoKYXN5bmMgZnVuY3Rpb24gX193YmdfbG9hZChtb2R1bGUsIGltcG9ydHMpIHsKICAgIGlmICh0eXBlb2YgUmVzcG9uc2UgPT09ICdmdW5jdGlvbicgJiYgbW9kdWxlIGluc3RhbmNlb2YgUmVzcG9uc2UpIHsKICAgICAgICBpZiAodHlwZW9mIFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlU3RyZWFtaW5nID09PSAnZnVuY3Rpb24nKSB7CiAgICAgICAgICAgIHRyeSB7CiAgICAgICAgICAgICAgICByZXR1cm4gYXdhaXQgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGVTdHJlYW1pbmcobW9kdWxlLCBpbXBvcnRzKTsKICAgICAgICAgICAgfQogICAgICAgICAgICBjYXRjaCAoZSkgewogICAgICAgICAgICAgICAgY29uc3QgdmFsaWRSZXNwb25zZSA9IG1vZHVsZS5vayAmJiBleHBlY3RlZFJlc3BvbnNlVHlwZShtb2R1bGUudHlwZSk7CiAgICAgICAgICAgICAgICBpZiAodmFsaWRSZXNwb25zZSAmJiBtb2R1bGUuaGVhZGVycy5nZXQoJ0NvbnRlbnQtVHlwZScpICE9PSAnYXBwbGljYXRpb24vd2FzbScpIHsKICAgICAgICAgICAgICAgICAgICBjb25zb2xlLndhcm4oImBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZVN0cmVhbWluZ2AgZmFpbGVkIGJlY2F1c2UgeW91ciBzZXJ2ZXIgZG9lcyBub3Qgc2VydmUgV2FzbSB3aXRoIGBhcHBsaWNhdGlvbi93YXNtYCBNSU1FIHR5cGUuIEZhbGxpbmcgYmFjayB0byBgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGVgIHdoaWNoIGlzIHNsb3dlci4gT3JpZ2luYWwgZXJyb3I6XG4iLCBlKTsKICAgICAgICAgICAgICAgIH0KICAgICAgICAgICAgICAgIGVsc2UgewogICAgICAgICAgICAgICAgICAgIHRocm93IGU7CiAgICAgICAgICAgICAgICB9CiAgICAgICAgICAgIH0KICAgICAgICB9CiAgICAgICAgY29uc3QgYnl0ZXMgPSBhd2FpdCBtb2R1bGUuYXJyYXlCdWZmZXIoKTsKICAgICAgICByZXR1cm4gYXdhaXQgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGUoYnl0ZXMsIGltcG9ydHMpOwogICAgfQogICAgZWxzZSB7CiAgICAgICAgY29uc3QgaW5zdGFuY2UgPSBhd2FpdCBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZShtb2R1bGUsIGltcG9ydHMpOwogICAgICAgIGlmIChpbnN0YW5jZSBpbnN0YW5jZW9mIFdlYkFzc2VtYmx5Lkluc3RhbmNlKSB7CiAgICAgICAgICAgIHJldHVybiB7IGluc3RhbmNlLCBtb2R1bGUgfTsKICAgICAgICB9CiAgICAgICAgZWxzZSB7CiAgICAgICAgICAgIHJldHVybiBpbnN0YW5jZTsKICAgICAgICB9CiAgICB9CiAgICBmdW5jdGlvbiBleHBlY3RlZFJlc3BvbnNlVHlwZSh0eXBlKSB7CiAgICAgICAgc3dpdGNoICh0eXBlKSB7CiAgICAgICAgICAgIGNhc2UgJ2Jhc2ljJzoKICAgICAgICAgICAgY2FzZSAnY29ycyc6CiAgICAgICAgICAgIGNhc2UgJ2RlZmF1bHQnOiByZXR1cm4gdHJ1ZTsKICAgICAgICB9CiAgICAgICAgcmV0dXJuIGZhbHNlOwogICAgfQp9CgovLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLwovLyBJbml0OgovLyBfX3diZ19pbml0Ci8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCgphc3luYyBmdW5jdGlvbiBfX3diZ19pbml0KG1vZHVsZV9vcl9wYXRoLCBtZW1vcnkpIHsKICAgIGlmICh3YXNtICE9PSB1bmRlZmluZWQpCiAgICAgICAgcmV0dXJuIHdhc207CiAgICBsZXQgdGhyZWFkX3N0YWNrX3NpemU7CiAgICBpZiAobW9kdWxlX29yX3BhdGggIT09IHVuZGVmaW5lZCkgewogICAgICAgIGlmIChPYmplY3QuZ2V0UHJvdG90eXBlT2YobW9kdWxlX29yX3BhdGgpID09PSBPYmplY3QucHJvdG90eXBlKSB7CiAgICAgICAgICAgICh7IG1vZHVsZV9vcl9wYXRoLCBtZW1vcnksIHRocmVhZF9zdGFja19zaXplIH0gPSBtb2R1bGVfb3JfcGF0aCk7CiAgICAgICAgfQogICAgICAgIGVsc2UgewogICAgICAgICAgICBjb25zb2xlLndhcm4oJ3VzaW5nIGRlcHJlY2F0ZWQgcGFyYW1ldGVycyBmb3IgdGhlIGluaXRpYWxpemF0aW9uIGZ1bmN0aW9uOyBwYXNzIGEgc2luZ2xlIG9iamVjdCBpbnN0ZWFkJyk7CiAgICAgICAgfQogICAgfQogICAgLy8gICBpZiAobW9kdWxlX29yX3BhdGggPT09IHVuZGVmaW5lZCkgewogICAgLy8gICAgIG1vZHVsZV9vcl9wYXRoID0gbmV3IFVSTCgndGZoZV9iZy53YXNtJywgaW1wb3J0Lm1ldGEudXJsKTsKICAgIC8vICAgfQogICAgY29uc3QgaW1wb3J0cyA9IF9fd2JnX2dldF9pbXBvcnRzKG1lbW9yeSk7CiAgICAvLyAgIGlmICgKICAgIC8vICAgICB0eXBlb2YgbW9kdWxlX29yX3BhdGggPT09ICdzdHJpbmcnIHx8CiAgICAvLyAgICAgKHR5cGVvZiBSZXF1ZXN0ID09PSAnZnVuY3Rpb24nICYmIG1vZHVsZV9vcl9wYXRoIGluc3RhbmNlb2YgUmVxdWVzdCkgfHwKICAgIC8vICAgICAodHlwZW9mIFVSTCA9PT0gJ2Z1bmN0aW9uJyAmJiBtb2R1bGVfb3JfcGF0aCBpbnN0YW5jZW9mIFVSTCkKICAgIC8vICAgKSB7CiAgICAvLyAgICAgbW9kdWxlX29yX3BhdGggPSBmZXRjaChtb2R1bGVfb3JfcGF0aCk7CiAgICAvLyAgIH0KICAgIGNvbnN0IHsgaW5zdGFuY2UsIG1vZHVsZSB9ID0gYXdhaXQgX193YmdfbG9hZChhd2FpdCBtb2R1bGVfb3JfcGF0aCwgaW1wb3J0cyk7CiAgICByZXR1cm4gX193YmdfZmluYWxpemVfaW5pdChpbnN0YW5jZSwgbW9kdWxlLCB0aHJlYWRfc3RhY2tfc2l6ZSk7Cn0KCi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vCi8vCi8vIFRoZSAndGZoZScgZ2xvYmFsIG9iamVjdAovLyA9PT09PT09PT09PT09PT09PT09PT09PT0KLy8gRmluYWwgdGZoZSBvYmplY3QgZ2xvYmFsIGRlY2xhcmF0aW9uIGNhbGxlZCBieSAnd2FpdEZvck1zZ1R5cGUnIG9ubHkKLy8KLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8KCnZhciB0ZmhlID0gLyojX19QVVJFX18qLyBPYmplY3QuZnJlZXplKHsKICBfX3Byb3RvX186IG51bGwsCiAgZGVmYXVsdDogX193YmdfaW5pdCwKICB3YmdfcmF5b25fc3RhcnRfd29ya2VyOiB3YmdfcmF5b25fc3RhcnRfd29ya2VyLAp9KTsKCg==";
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
