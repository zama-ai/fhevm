/**
 * Detect browser-like environment by checking for global event listener APIs.
 */
function __isBrowserLike() {
  return (
    typeof addEventListener === "function" &&
    typeof removeEventListener === "function"
  );
}

/**
 * Create a Worker from the given URL, handling browser and Node.js differences.
 * In Node.js, data: URLs are decoded and executed via eval mode since
 * `node:worker_threads` does not support them natively.
 * @param {string | URL} url - Script URL for the worker (supports data: URLs in Node.js).
 * @returns {Promise<Worker>}
 */
async function __newIsomorphicWorker(url) {
  // Browser
  if (__isBrowserLike()) {
    //var Worker: new (scriptURL: string | URL, options?: WorkerOptions | undefined) => Worker
    return new Worker(url, {
      type: "module",
      name: "wasm_bindgen_worker",
    });
  }

  // Node.js
  const nodeModuleName = "worker_threads";
  const nodeModuleId = `node:${nodeModuleName}`;
  const { Worker: NodeWorker } = await import(/* @vite-ignore */ nodeModuleId);

  // Node's Worker doesn't support data: or blob: URLs.
  // For data: URLs, extract the code and use eval mode.
  if (typeof url === "string" && url.startsWith("data:")) {
    const base64 = url.split(",")[1];
    const code = Buffer.from(base64, "base64").toString("utf-8");
    return new NodeWorker(code, { eval: true });
  }

  return new NodeWorker(url);
}

/**
 * Create a Worker from a base64-encoded JavaScript source string.
 * In the browser, decodes to a Blob URL; in Node.js, decodes and runs via eval mode.
 * @param {string} jsCodeBase64 - Base64-encoded JavaScript source code.
 * @returns {Promise<{ worker: Worker, blobUrl: string | undefined }>}
 */
async function __newWorkerFromJsCodeBase64(jsCodeBase64) {
  // Browser
  if (__isBrowserLike()) {
    const blob = new Blob([atob(jsCodeBase64)], {
      type: "application/javascript",
    });

    const blobUrl = URL.createObjectURL(blob);

    try {
      const worker = new Worker(blobUrl, {
        type: "module",
        name: "wasm_bindgen_worker",
      });

      // Caller is responsible for revoking blobUrl after the worker is ready.
      return { worker, blobUrl };
    } catch (e) {
      URL.revokeObjectURL(blobUrl);
      throw e;
    }
  }

  // Node.js
  const nodeModuleName = "worker_threads";
  const nodeModuleId = `node:${nodeModuleName}`;
  const { Worker: NodeWorker } = await import(/* @vite-ignore */ nodeModuleId);

  const code = Buffer.from(jsCodeBase64, "base64").toString("utf-8");

  return { worker: new NodeWorker(code, { eval: true }), blobUrl: undefined };
}

/**
 * Wait for a message of the given type from a Worker.
 * Handles both Node.js EventEmitter (`on`/`off`) and browser DOM event APIs.
 * Rejects on worker errors or non-zero exit codes (Node.js).
 * @param {Worker} target - The worker to listen on.
 * @param {string} type - The `type` field of the expected message.
 * @returns {Promise<object>} Resolves with the matching message data.
 */
function __waitForMsgType(target, type) {
  return new Promise((resolve, reject) => {
    function cleanup() {
      if (typeof target.removeEventListener === "function") {
        target.removeEventListener("message", onBrowserMsg);
        target.removeEventListener("error", onBrowserError);
      } else {
        target.off("message", onNodeMsg);
        target.off("error", onNodeError);
        target.off("exit", onNodeExit);
      }
    }

    // Browser handlers
    function onBrowserMsg({ data }) {
      if (data?.type !== type) return;
      cleanup();
      resolve(data);
    }
    function onBrowserError(e) {
      cleanup();
      reject(e.error || new Error("Worker error"));
    }

    // Node handlers
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

    if (typeof target.removeEventListener === "function") {
      // Browser: DOM events
      target.addEventListener("message", onBrowserMsg);
      target.addEventListener("error", onBrowserError);
    } else {
      // Node: EventEmitter
      target.on("message", onNodeMsg);
      target.on("error", onNodeError);
      target.on("exit", onNodeExit);
    }
  });
}

////////////////////////////////////////////////////////////////////////////////
//
// Module variables
//
////////////////////////////////////////////////////////////////////////////////

/** @type {Promise<PromiseSettledResult<void>[]> | undefined} In-flight termination promise, if any. */
let _terminating;

/** @type {boolean} Whether {@link setWorkerUrlConfig} has already been called. */
let _configSet = false;

/** @type {URL | undefined} Custom URL for the worker script, if provided. */
let _workerUrl = undefined;

/** @type {{ debug: Function, error: Function } | undefined} Optional logger. */
let _logger = undefined;

/** @type {boolean} Whether {@link startWorkers} has already been called. */
let _started = false;

// Note: this is never used, but necessary to prevent a bug in Firefox
// (https://bugzilla.mozilla.org/show_bug.cgi?id=1702191) where it collects
// Web Workers that have a shared WebAssembly memory with the main thread,
// but are not explicitly rooted via a `Worker` instance.
//
// By storing them in a variable, we can keep `Worker` objects around and
// prevent them from getting GC-d.
/** @type {Worker[] | undefined} */
let _workers;

////////////////////////////////////////////////////////////////////////////////
//
// Exported functions
//
////////////////////////////////////////////////////////////////////////////////

/**
 * Return the current running TFHE worker threads.
 * @returns {Worker[] | undefined}
 */
function getTfheWorkers() {
  return _workers;
}

/**
 * Configure the worker script URL and logger. Can only be called once.
 * @param {object} [config]
 * @param {URL | undefined} [config.workerUrl] - URL to the worker script. Falls back to embedded base64 if omitted.
 * @param {{ debug: Function, error: Function } | undefined} [config.logger] - Optional logger instance.
 * @throws {Error} If called more than once.
 * @throws {TypeError} If `workerUrl` is not a `URL` instance.
 */
function setWorkerUrlConfig({
  workerUrl = undefined,
  logger = undefined,
} = {}) {
  if (_configSet) {
    throw new Error("Cannot set worker url (already set)");
  }

  if (workerUrl !== undefined) {
    if (!(workerUrl instanceof URL)) {
      throw new TypeError("workerUrl must be a URL");
    }
    _workerUrl = workerUrl;
  }

  _logger = logger;

  _configSet = true;
}

/**
 * Create a Worker from the embedded base64-encoded worker script.
 * Used as the last-resort fallback when no `workerUrl` is available
 * or when direct/blob URL creation fails.
 * - copy/paste base64 from `./tfhe-worker.v1.6.0.inline.js`
 * - run `node scripts/tfhe-worker.v1.6.0.inline.build.mjs` to generate
 * @returns {Promise<{ worker: Worker, blobUrl: string | undefined }>}
 */
async function __createWorkerFromBase64() {
  const { workerBase64 } = await import('./tfhe-worker.v1.6.0.inline.js');
  return await __newWorkerFromJsCodeBase64(workerBase64);
}

/**
 * Spawn TFHE worker threads, initialize them with the given WASM module
 * and shared memory, and finalize the thread pool via the builder.
 * Can be called only once. No reset.
 * @param {WebAssembly.Module} module - Compiled WASM module to send to each worker.
 * @param {WebAssembly.Memory} memory - Shared memory instance for cross-thread communication.
 * @param {{ numThreads(): number, receiver(): SharedArrayBuffer, build(): void }} builder - Thread pool builder from the TFHE WASM bindings.
 * @throws {Error} If termination is in progress, numThreads is 0, or any worker fails to start.
 * @returns {Promise<void>}
 */
async function startWorkers(module, memory, builder) {
  if (_started) {
    throw new Error("Already started");
  }

  _started = true;

  if (_terminating) {
    throw new Error("Cannot start workers while termination is in progress");
  }

  if (builder.numThreads() === 0) {
    throw new Error(`num_threads must be > 0.`);
  }

  const workerInit = {
    type: "wasm_bindgen_worker_init",
    init: { module_or_path: module, memory },
    // SharedArrayBuffer
    receiver: builder.receiver(),
  };

  const results = await Promise.allSettled(
    Array.from({ length: builder.numThreads() }, async (_, workerIndex) => {
      // Self-spawn into a new Worker.
      //
      // TODO: while `new URL('...', import.meta.url) becomes a semi-standard
      // way to get asset URLs relative to the module across various bundlers
      // and browser, ideally we should switch to `import.meta.resolve`
      // once it becomes a standard.
      //
      // Note: we could use `../../..` as the URL here to inline workerHelpers.js
      // into the parent entry instead of creating another split point -
      // this would be preferable from optimization perspective -
      // however, Webpack then eliminates all message handler code
      // because wasm-pack produces "sideEffects":false in package.json
      // unconditionally.
      //
      // The only way to work around that is to have side effect code
      // in an entry point such as Worker file itself.

      let blobUrl = undefined;
      let worker;

      // Worker creation fallback chain (see matrix in types.ts):
      //
      // | workerUrl | Result                                    |
      // |-----------|-------------------------------------------|
      // | defined   | Direct URL → fetch+blob → embedded base64 |
      // | undefined | Embedded base64 worker                    |

      if (_workerUrl) {
        // Step 1: Try direct URL — new Worker(url)
        try {
          worker = await __newIsomorphicWorker(_workerUrl);
          _logger?.debug(
            `[Worker #${workerIndex}] - created at url: ${_workerUrl}`,
          );
        } catch (e) {
          _logger?.error(
            `[Worker #${workerIndex}] - create failed: ${e.message}`,
            e,
          );
        }

        // Step 2: Fetch script and create blob URL — handles cross-origin restrictions
        if (worker === undefined) {
          try {
            const scriptBlob = await fetch(_workerUrl).then((r) => r.blob());
            blobUrl = URL.createObjectURL(scriptBlob);
            worker = await __newIsomorphicWorker(blobUrl);
            _logger?.debug(
              `[Worker #${workerIndex}] - created fallback blob worker at url: ${_workerUrl}`,
            );
          } catch (e) {
            if (blobUrl) {
              URL.revokeObjectURL(blobUrl);
              blobUrl = undefined;
            }

            _logger?.error(
              `[Worker #${workerIndex}] - create fallback blob worker failed: ${e.message}`,
              e,
            );
          }
        }
      }

      try {
        // Step 3: Embedded base64 worker — last resort fallback
        if (!worker) {
          try {
            const result = await __createWorkerFromBase64();
            worker = result.worker;
            blobUrl = result.blobUrl ?? blobUrl;
            _logger?.debug(
              `[Worker #${workerIndex}] - created blob worker using base64`,
            );
          } catch (e) {
            _logger?.error(
              `[Worker #${workerIndex}] - create blob worker using base64 failed`,
              e,
            );

            throw new Error(
              "All worker creation methods failed. Check CSP, COOP/COEP headers, and cross-origin policies.",
              { cause: e },
            );
          }
        }

        try {
          worker.postMessage(workerInit);
        } catch (e) {
          _logger?.error(`[Worker #${workerIndex}] - postMessage failed`, e);
          throw e;
        }

        await __waitForMsgType(worker, "wasm_bindgen_worker_ready");

        // Revoke blob URL only after the worker has loaded its script
        if (blobUrl) {
          URL.revokeObjectURL(blobUrl);
          blobUrl = undefined;
        }

        _logger?.debug(`[Worker #${workerIndex}] - ready`);

        return worker;
      } catch (err) {
        if (blobUrl) {
          URL.revokeObjectURL(blobUrl);
          blobUrl = undefined;
        }

        throw err;
      }
    }),
  );

  // Separate fulfilled workers from failures
  const workers = [];
  const errors = [];

  for (const result of results) {
    if (result.status === "fulfilled") {
      workers.push(result.value);
    } else {
      errors.push(result.reason);
    }
  }

  // If any worker failed, terminate the ones that succeeded to prevent orphaned threads
  if (errors.length > 0) {
    await Promise.allSettled(workers.map((w) => w.terminate()));
    throw errors[0];
  }

  _workers = workers;
  builder.build();
}

/**
 * Terminate all active TFHE workers and wait for cleanup to complete.
 * Safe to call concurrently — subsequent calls will await the in-progress termination.
 * Called only once. No reset.
 * @returns {Promise<unknown>}
 */
async function terminateWorkers() {
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

export { getTfheWorkers, startWorkers, terminateWorkers, setWorkerUrlConfig };
