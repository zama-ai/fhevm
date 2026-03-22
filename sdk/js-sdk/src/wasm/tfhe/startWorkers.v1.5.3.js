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
  const { Worker } = await import(nodeModuleId);

  // Node's Worker doesn't support data: or blob: URLs.
  // For data: URLs, extract the code and use eval mode.
  if (typeof url === "string" && url.startsWith("data:")) {
    const base64 = url.split(",")[1];
    const code = Buffer.from(base64, "base64").toString("utf-8");
    return new Worker(code, { eval: true });
  }

  return new Worker(url);
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
  const { Worker } = await import(nodeModuleId);

  const code = Buffer.from(jsCodeBase64, "base64").toString("utf-8");

  return { worker: new Worker(code, { eval: true }), blobUrl: undefined };
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
 * - copy/paste base64 from `./tfhe-worker.v1.5.3.inline.js`
 * - run `./inline-worker.build.mjs` to generate
 * @returns {Promise<{ worker: Worker, blobUrl: string | undefined }>}
 */
async function __createWorkerFromBase64() {
  // SHA-256: caa32a0babf2c76b7097e67b1cbae06256c3db638e31c02348c9c37988cc5f2e
  const workerBase64 =
    "ZnVuY3Rpb24gX19faXNCcm93c2VyTGlrZSgpe3JldHVybiB0eXBlb2YgYWRkRXZlbnRMaXN0ZW5lcj09ImZ1bmN0aW9uIiYmdHlwZW9mIHJlbW92ZUV2ZW50TGlzdGVuZXI9PSJmdW5jdGlvbiJ9YXN5bmMgZnVuY3Rpb24gX19fZ2V0VGFyZ2V0KCl7aWYoX19faXNCcm93c2VyTGlrZSgpKXJldHVybiBzZWxmO2NvbnN0IF89Im5vZGU6d29ya2VyX3RocmVhZHMiLHtwYXJlbnRQb3J0OmV9PWF3YWl0IGltcG9ydChfKTtyZXR1cm4gZX1mdW5jdGlvbiBfX193YWl0Rm9yTXNnVHlwZSh0LF8pe3JldHVybiBuZXcgUHJvbWlzZShlPT57dHlwZW9mIHQub249PSJmdW5jdGlvbiI/dC5vbigibWVzc2FnZSIsZnVuY3Rpb24gbihyKXtyPy50eXBlPT09XyYmKHQub2ZmKCJtZXNzYWdlIixuKSxlKHIpKX0pOnQuYWRkRXZlbnRMaXN0ZW5lcigibWVzc2FnZSIsZnVuY3Rpb24gbih7ZGF0YTpyfSl7cj8udHlwZT09PV8mJih0LnJlbW92ZUV2ZW50TGlzdGVuZXIoIm1lc3NhZ2UiLG4pLGUocikpfSl9KX1fX19nZXRUYXJnZXQoKS50aGVuKHQ9Pl9fX3dhaXRGb3JNc2dUeXBlKHQsIndhc21fYmluZGdlbl93b3JrZXJfaW5pdCIpLnRoZW4oYXN5bmMoe2luaXQ6XyxyZWNlaXZlcjplfSk9Pntjb25zdCBuPWF3YWl0IFByb21pc2UucmVzb2x2ZSgpLnRoZW4oZnVuY3Rpb24oKXtyZXR1cm4gdGZoZX0pO2F3YWl0IG4uZGVmYXVsdChfKSx0LnBvc3RNZXNzYWdlKHt0eXBlOiJ3YXNtX2JpbmRnZW5fd29ya2VyX3JlYWR5In0pLG4ud2JnX3JheW9uX3N0YXJ0X3dvcmtlcihlKX0pKTtmdW5jdGlvbiB3YmdfcmF5b25fc3RhcnRfd29ya2VyKHQpe3dhc20ud2JnX3JheW9uX3N0YXJ0X3dvcmtlcih0KX1mdW5jdGlvbiBkZWJ1Z1N0cmluZyh0KXtjb25zdCBfPXR5cGVvZiB0O2lmKF89PSJudW1iZXIifHxfPT0iYm9vbGVhbiJ8fHQ9PW51bGwpcmV0dXJuYCR7dH1gO2lmKF89PSJzdHJpbmciKXJldHVybmAiJHt0fSJgO2lmKF89PSJzeW1ib2wiKXtjb25zdCByPXQuZGVzY3JpcHRpb247cmV0dXJuIHI9PW51bGw/IlN5bWJvbCI6YFN5bWJvbCgke3J9KWB9aWYoXz09ImZ1bmN0aW9uIil7Y29uc3Qgcj10Lm5hbWU7cmV0dXJuIHR5cGVvZiByPT0ic3RyaW5nIiYmci5sZW5ndGg+MD9gRnVuY3Rpb24oJHtyfSlgOiJGdW5jdGlvbiJ9aWYoQXJyYXkuaXNBcnJheSh0KSl7Y29uc3Qgcj10Lmxlbmd0aDtsZXQgbz0iWyI7cj4wJiYobys9ZGVidWdTdHJpbmcodFswXSkpO2ZvcihsZXQgaT0xO2k8cjtpKyspbys9IiwgIitkZWJ1Z1N0cmluZyh0W2ldKTtyZXR1cm4gbys9Il0iLG99Y29uc3QgZT0vXFtvYmplY3QgKFteXF1dKylcXS8uZXhlYyh0b1N0cmluZy5jYWxsKHQpKTtsZXQgbjtpZihlJiZlLmxlbmd0aD4xKW49ZVsxXTtlbHNlIHJldHVybiB0b1N0cmluZy5jYWxsKHQpO2lmKG49PSJPYmplY3QiKXRyeXtyZXR1cm4iT2JqZWN0KCIrSlNPTi5zdHJpbmdpZnkodCkrIikifWNhdGNoe3JldHVybiJPYmplY3QifXJldHVybiB0IGluc3RhbmNlb2YgRXJyb3I/YCR7dC5uYW1lfTogJHt0Lm1lc3NhZ2V9CiR7dC5zdGFja31gOm59ZnVuY3Rpb24gZ2V0U3RyaW5nRnJvbVdhc20wKHQsXyl7cmV0dXJuIHQ9dD4+PjAsZGVjb2RlVGV4dCh0LF8pfWxldCBjYWNoZWRUZXh0RGVjb2Rlcj10eXBlb2YgVGV4dERlY29kZXI8InUiP25ldyBUZXh0RGVjb2RlcigidXRmLTgiLHtpZ25vcmVCT006ITAsZmF0YWw6ITB9KTp2b2lkIDA7Y2FjaGVkVGV4dERlY29kZXImJmNhY2hlZFRleHREZWNvZGVyLmRlY29kZSgpO2NvbnN0IE1BWF9TQUZBUklfREVDT0RFX0JZVEVTPTIxNDY0MzUwNzI7bGV0IG51bUJ5dGVzRGVjb2RlZD0wO2Z1bmN0aW9uIGRlY29kZVRleHQodCxfKXtyZXR1cm4gbnVtQnl0ZXNEZWNvZGVkKz1fLG51bUJ5dGVzRGVjb2RlZD49TUFYX1NBRkFSSV9ERUNPREVfQllURVMmJihjYWNoZWRUZXh0RGVjb2Rlcj1uZXcgVGV4dERlY29kZXIoInV0Zi04Iix7aWdub3JlQk9NOiEwLGZhdGFsOiEwfSksY2FjaGVkVGV4dERlY29kZXIuZGVjb2RlKCksbnVtQnl0ZXNEZWNvZGVkPV8pLGNhY2hlZFRleHREZWNvZGVyLmRlY29kZShnZXRVaW50OEFycmF5TWVtb3J5MCgpLnNsaWNlKHQsdCtfKSl9ZnVuY3Rpb24gYWRkVG9FeHRlcm5yZWZUYWJsZTAodCl7Y29uc3QgXz13YXNtLl9fZXh0ZXJucmVmX3RhYmxlX2FsbG9jKCk7cmV0dXJuIHdhc20uX193YmluZGdlbl9leHRlcm5yZWZzLnNldChfLHQpLF99bGV0IGNhY2hlZERhdGFWaWV3TWVtb3J5MD1udWxsO2Z1bmN0aW9uIGdldERhdGFWaWV3TWVtb3J5MCgpe3JldHVybihjYWNoZWREYXRhVmlld01lbW9yeTA9PT1udWxsfHxjYWNoZWREYXRhVmlld01lbW9yeTAuYnVmZmVyIT09d2FzbS5tZW1vcnkuYnVmZmVyKSYmKGNhY2hlZERhdGFWaWV3TWVtb3J5MD1uZXcgRGF0YVZpZXcod2FzbS5tZW1vcnkuYnVmZmVyKSksY2FjaGVkRGF0YVZpZXdNZW1vcnkwfWxldCBjYWNoZWRVaW50OEFycmF5TWVtb3J5MD1udWxsO2Z1bmN0aW9uIGdldFVpbnQ4QXJyYXlNZW1vcnkwKCl7cmV0dXJuKGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwPT09bnVsbHx8Y2FjaGVkVWludDhBcnJheU1lbW9yeTAuYnVmZmVyIT09d2FzbS5tZW1vcnkuYnVmZmVyKSYmKGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwPW5ldyBVaW50OEFycmF5KHdhc20ubWVtb3J5LmJ1ZmZlcikpLGNhY2hlZFVpbnQ4QXJyYXlNZW1vcnkwfWZ1bmN0aW9uIGdldEFycmF5VThGcm9tV2FzbTAodCxfKXtyZXR1cm4gdD10Pj4+MCxnZXRVaW50OEFycmF5TWVtb3J5MCgpLnN1YmFycmF5KHQvMSx0LzErXyl9bGV0IFdBU01fVkVDVE9SX0xFTj0wO2NvbnN0IGNhY2hlZFRleHRFbmNvZGVyPXR5cGVvZiBUZXh0RW5jb2RlcjwidSI/bmV3IFRleHRFbmNvZGVyOnZvaWQgMDtjYWNoZWRUZXh0RW5jb2RlciYmKGNhY2hlZFRleHRFbmNvZGVyLmVuY29kZUludG89ZnVuY3Rpb24odCxfKXtjb25zdCBlPWNhY2hlZFRleHRFbmNvZGVyLmVuY29kZSh0KTtyZXR1cm4gXy5zZXQoZSkse3JlYWQ6dC5sZW5ndGgsd3JpdHRlbjplLmxlbmd0aH19KTtmdW5jdGlvbiBwYXNzU3RyaW5nVG9XYXNtMCh0LF8sZSl7aWYoZT09PXZvaWQgMCl7Y29uc3QgYz1jYWNoZWRUZXh0RW5jb2Rlci5lbmNvZGUodCkscz1fKGMubGVuZ3RoLDEpPj4+MDtyZXR1cm4gZ2V0VWludDhBcnJheU1lbW9yeTAoKS5zdWJhcnJheShzLHMrYy5sZW5ndGgpLnNldChjKSxXQVNNX1ZFQ1RPUl9MRU49Yy5sZW5ndGgsc31sZXQgbj10Lmxlbmd0aCxyPV8obiwxKT4+PjA7Y29uc3Qgbz1nZXRVaW50OEFycmF5TWVtb3J5MCgpO2xldCBpPTA7Zm9yKDtpPG47aSsrKXtjb25zdCBjPXQuY2hhckNvZGVBdChpKTtpZihjPjEyNylicmVhaztvW3IraV09Y31pZihpIT09bil7aSE9PTAmJih0PXQuc2xpY2UoaSkpLHI9ZShyLG4sbj1pK3QubGVuZ3RoKjMsMSk+Pj4wO2NvbnN0IGM9Z2V0VWludDhBcnJheU1lbW9yeTAoKS5zdWJhcnJheShyK2kscituKSxzPWNhY2hlZFRleHRFbmNvZGVyLmVuY29kZUludG8odCxjKTtpKz1zLndyaXR0ZW4scj1lKHIsbixpLDEpPj4+MH1yZXR1cm4gV0FTTV9WRUNUT1JfTEVOPWkscn1mdW5jdGlvbiBoYW5kbGVFcnJvcih0LF8pe3RyeXtyZXR1cm4gdC5hcHBseSh0aGlzLF8pfWNhdGNoKGUpe2NvbnN0IG49YWRkVG9FeHRlcm5yZWZUYWJsZTAoZSk7d2FzbS5fX3diaW5kZ2VuX2V4bl9zdG9yZShuKX19ZnVuY3Rpb24gaXNMaWtlTm9uZSh0KXtyZXR1cm4gdD09bnVsbH1mdW5jdGlvbiBfX3diZ19nZXRfaW1wb3J0cyh0KXtyZXR1cm57X19wcm90b19fOm51bGwsIi4vdGZoZV9iZy5qcyI6e19fcHJvdG9fXzpudWxsLF9fd2JnX0JpZ0ludF83ZWExZTc0OWNlNmI5MmZkOmZ1bmN0aW9uKCl7cmV0dXJuIGhhbmRsZUVycm9yKGZ1bmN0aW9uKGUpe3JldHVybiBCaWdJbnQoZSl9LGFyZ3VtZW50cyl9LF9fd2JnX0JpZ0ludF9iN2JiY2NkZmYyNThjOWYyOmZ1bmN0aW9uKGUpe3JldHVybiBCaWdJbnQoZSl9LF9fd2JnX0Vycm9yXzhjNGU0M2ZlNzQ1NTlkNzM6ZnVuY3Rpb24oZSxuKXtyZXR1cm4gRXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGUsbikpfSxfX3diZ19fX3diaW5kZ2VuX2JpZ2ludF9nZXRfYXNfaTY0XzhmY2Y0Y2U3ZjFjYTcyYTI6ZnVuY3Rpb24oZSxuKXtjb25zdCByPW4sbz10eXBlb2Ygcj09ImJpZ2ludCI/cjp2b2lkIDA7Z2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0QmlnSW50NjQoZSs4LGlzTGlrZU5vbmUobyk/QmlnSW50KDApOm8sITApLGdldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGUrMCwhaXNMaWtlTm9uZShvKSwhMCl9LF9fd2JnX19fd2JpbmRnZW5fYml0X2FuZF80MzYyYjExNzY5NTBkNDJhOmZ1bmN0aW9uKGUsbil7cmV0dXJuIGUmbn0sX193YmdfX193YmluZGdlbl9iaXRfb3JfNzFhMDJkMzk3OTZlYTEzZDpmdW5jdGlvbihlLG4pe3JldHVybiBlfG59LF9fd2JnX19fd2JpbmRnZW5fZGVidWdfc3RyaW5nXzBiYzg0ODJjNmUzNTA4YWU6ZnVuY3Rpb24oZSxuKXtjb25zdCByPWRlYnVnU3RyaW5nKG4pLG89cGFzc1N0cmluZ1RvV2FzbTAocix3YXNtLl9fd2JpbmRnZW5fbWFsbG9jLHdhc20uX193YmluZGdlbl9yZWFsbG9jKSxpPVdBU01fVkVDVE9SX0xFTjtnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihlKzQsaSwhMCksZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoZSswLG8sITApfSxfX3diZ19fX3diaW5kZ2VuX2lzX2Z1bmN0aW9uXzAwOTVhNzNiOGIxNTZmNzY6ZnVuY3Rpb24oZSl7cmV0dXJuIHR5cGVvZiBlPT0iZnVuY3Rpb24ifSxfX3diZ19fX3diaW5kZ2VuX2lzX29iamVjdF81YWU4ZTU4ODBmMmMxZmJkOmZ1bmN0aW9uKGUpe2NvbnN0IG49ZTtyZXR1cm4gdHlwZW9mIG49PSJvYmplY3QiJiZuIT09bnVsbH0sX193YmdfX193YmluZGdlbl9pc19zdHJpbmdfY2Q0NDQ1MTZlZGM1YjE4MDpmdW5jdGlvbihlKXtyZXR1cm4gdHlwZW9mIGU9PSJzdHJpbmcifSxfX3diZ19fX3diaW5kZ2VuX2lzX3VuZGVmaW5lZF85ZTRkOTI1MzRjNDJkNzc4OmZ1bmN0aW9uKGUpe3JldHVybiBlPT09dm9pZCAwfSxfX3diZ19fX3diaW5kZ2VuX2pzdmFsX2VxXzExODg4MzkwYjAxODYyNzA6ZnVuY3Rpb24oZSxuKXtyZXR1cm4gZT09PW59LF9fd2JnX19fd2JpbmRnZW5fbHRfYmI1OWNjM2QyMzUyNmUwZDpmdW5jdGlvbihlLG4pe3JldHVybiBlPG59LF9fd2JnX19fd2JpbmRnZW5fbWVtb3J5X2JkMWZiY2YyMWZiZWYzYzg6ZnVuY3Rpb24oKXtyZXR1cm4gd2FzbS5tZW1vcnl9LF9fd2JnX19fd2JpbmRnZW5fbW9kdWxlX2Y2YjgwNTJkNzljMWNjMTY6ZnVuY3Rpb24oKXtyZXR1cm4gd2FzbU1vZHVsZX0sX193YmdfX193YmluZGdlbl9uZWdfNmI0ZDM1NmRmZjQ5ZGNjNjpmdW5jdGlvbihlKXtyZXR1cm4tZX0sX193YmdfX193YmluZGdlbl9zaGxfOGQ2NGQwNjc2MWY5ZWE0ZTpmdW5jdGlvbihlLG4pe3JldHVybiBlPDxufSxfX3diZ19fX3diaW5kZ2VuX3Nocl9lZjhlMDdjY2U3MDllYjU0OmZ1bmN0aW9uKGUsbil7cmV0dXJuIGU+Pm59LF9fd2JnX19fd2JpbmRnZW5fc3RyaW5nX2dldF83MmZiNjk2MjAyYzU2NzI5OmZ1bmN0aW9uKGUsbil7Y29uc3Qgcj1uLG89dHlwZW9mIHI9PSJzdHJpbmciP3I6dm9pZCAwO3ZhciBpPWlzTGlrZU5vbmUobyk/MDpwYXNzU3RyaW5nVG9XYXNtMChvLHdhc20uX193YmluZGdlbl9tYWxsb2Msd2FzbS5fX3diaW5kZ2VuX3JlYWxsb2MpLGM9V0FTTV9WRUNUT1JfTEVOO2dldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGUrNCxjLCEwKSxnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihlKzAsaSwhMCl9LF9fd2JnX19fd2JpbmRnZW5fdGhyb3dfYmUyODlkNTAzNGVkMjcxYjpmdW5jdGlvbihlLG4pe3Rocm93IG5ldyBFcnJvcihnZXRTdHJpbmdGcm9tV2FzbTAoZSxuKSl9LF9fd2JnX2NhbGxfMzg5ZWZlMjg0MzVhOTM4ODpmdW5jdGlvbigpe3JldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbihlLG4pe3JldHVybiBlLmNhbGwobil9LGFyZ3VtZW50cyl9LF9fd2JnX2NhbGxfNDcwOGUwYzEzYmRjOGU5NTpmdW5jdGlvbigpe3JldHVybiBoYW5kbGVFcnJvcihmdW5jdGlvbihlLG4scil7cmV0dXJuIGUuY2FsbChuLHIpfSxhcmd1bWVudHMpfSxfX3diZ19jcnlwdG9fODZmMjYzMWU5MWI1MTUxMTpmdW5jdGlvbihlKXtyZXR1cm4gZS5jcnlwdG99LF9fd2JnX2Vycm9yXzc1MzRiOGU5YTM2ZjFhYjQ6ZnVuY3Rpb24oZSxuKXtsZXQgcixvO3RyeXtyPWUsbz1uLGNvbnNvbGUuZXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGUsbikpfWZpbmFsbHl7d2FzbS5fX3diaW5kZ2VuX2ZyZWUocixvLDEpfX0sX193YmdfZ2V0UmFuZG9tVmFsdWVzX2IzZjE1ZmNiZmFiYjBmOGI6ZnVuY3Rpb24oKXtyZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24oZSxuKXtlLmdldFJhbmRvbVZhbHVlcyhuKX0sYXJndW1lbnRzKX0sX193YmdfZ2V0VGltZV8xZTNjZDEzOTFjNWMzOTk1OmZ1bmN0aW9uKGUpe3JldHVybiBlLmdldFRpbWUoKX0sX193YmdfaW5zdGFuY2VvZl9XaW5kb3dfZWQ0OWIyZGI4ZGY5MDM1OTpmdW5jdGlvbihlKXtsZXQgbjt0cnl7bj1lIGluc3RhbmNlb2YgV2luZG93fWNhdGNoe249ITF9cmV0dXJuIG59LF9fd2JnX2xlbmd0aF8zMmVkOWEyNzlhY2QwNTRjOmZ1bmN0aW9uKGUpe3JldHVybiBlLmxlbmd0aH0sX193YmdfbXNDcnlwdG9fZDU2MmJiZTgzZTBkNGI5MTpmdW5jdGlvbihlKXtyZXR1cm4gZS5tc0NyeXB0b30sX193YmdfbmV3XzBfNzNhZmMzNWViNTQ0ZTUzOTpmdW5jdGlvbigpe3JldHVybiBuZXcgRGF0ZX0sX193YmdfbmV3XzhhNmYyMzhhNmVjZTg2ZWE6ZnVuY3Rpb24oKXtyZXR1cm4gbmV3IEVycm9yfSxfX3diZ19uZXdfbm9fYXJnc18xYzdjODQyZjA4ZDAwZWJiOmZ1bmN0aW9uKGUsbil7cmV0dXJuIG5ldyBGdW5jdGlvbihnZXRTdHJpbmdGcm9tV2FzbTAoZSxuKSl9LF9fd2JnX25ld193aXRoX2xlbmd0aF9hMmMzOWNiZTg4ZmQ4ZmYxOmZ1bmN0aW9uKGUpe3JldHVybiBuZXcgVWludDhBcnJheShlPj4+MCl9LF9fd2JnX25vZGVfZTFmMjRmODlhNzMzNmMyZTpmdW5jdGlvbihlKXtyZXR1cm4gZS5ub2RlfSxfX3diZ19wcm9jZXNzXzM5NzVmZDZjNzJmNTIwYWE6ZnVuY3Rpb24oZSl7cmV0dXJuIGUucHJvY2Vzc30sX193YmdfcHJvdG90eXBlc2V0Y2FsbF9iZGNkY2M1ODQyZTRkNzdkOmZ1bmN0aW9uKGUsbixyKXtVaW50OEFycmF5LnByb3RvdHlwZS5zZXQuY2FsbChnZXRBcnJheVU4RnJvbVdhc20wKGUsbikscil9LF9fd2JnX3JhbmRvbUZpbGxTeW5jX2Y4YzE1M2I3OWYyODU4MTc6ZnVuY3Rpb24oKXtyZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24oZSxuKXtlLnJhbmRvbUZpbGxTeW5jKG4pfSxhcmd1bWVudHMpfSxfX3diZ19yZXF1aXJlX2I3NGY0N2ZjMmQwMjJmZDY6ZnVuY3Rpb24oKXtyZXR1cm4gaGFuZGxlRXJyb3IoZnVuY3Rpb24oKXtyZXR1cm4gbW9kdWxlLnJlcXVpcmV9LGFyZ3VtZW50cyl9LF9fd2JnX3N0YWNrXzBlZDc1ZDY4NTc1YjBmM2M6ZnVuY3Rpb24oZSxuKXtjb25zdCByPW4uc3RhY2ssbz1wYXNzU3RyaW5nVG9XYXNtMChyLHdhc20uX193YmluZGdlbl9tYWxsb2Msd2FzbS5fX3diaW5kZ2VuX3JlYWxsb2MpLGk9V0FTTV9WRUNUT1JfTEVOO2dldERhdGFWaWV3TWVtb3J5MCgpLnNldEludDMyKGUrNCxpLCEwKSxnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihlKzAsbywhMCl9LF9fd2JnX3N0YXJ0V29ya2Vyc18yY2ExMTc2MWUwOGZmNWQ1OmZ1bmN0aW9uKGUsbixyKXtoYW5kbGVFcnJvcihmdW5jdGlvbigpe3Rocm93IG5ldyBFcnJvcigic3RhcnRXb3JrZXJzIG5vdCBzdXBwb3J0ZWQgZnJvbSBhIHdvcmtlciB0aHJlYWQiKX0pfSxfX3diZ19zdGF0aWNfYWNjZXNzb3JfR0xPQkFMXzEyODM3MTY3YWQ5MzUxMTY6ZnVuY3Rpb24oKXtjb25zdCBlPXR5cGVvZiBnbG9iYWw+InUiP251bGw6Z2xvYmFsO3JldHVybiBpc0xpa2VOb25lKGUpPzA6YWRkVG9FeHRlcm5yZWZUYWJsZTAoZSl9LF9fd2JnX3N0YXRpY19hY2Nlc3Nvcl9HTE9CQUxfVEhJU19lNjI4ZTg5YWIzYjFjOTVmOmZ1bmN0aW9uKCl7Y29uc3QgZT10eXBlb2YgZ2xvYmFsVGhpcz4idSI/bnVsbDpnbG9iYWxUaGlzO3JldHVybiBpc0xpa2VOb25lKGUpPzA6YWRkVG9FeHRlcm5yZWZUYWJsZTAoZSl9LF9fd2JnX3N0YXRpY19hY2Nlc3Nvcl9TRUxGX2E2MjFkM2RmYmI2MGQwY2U6ZnVuY3Rpb24oKXtjb25zdCBlPXR5cGVvZiBzZWxmPiJ1Ij9udWxsOnNlbGY7cmV0dXJuIGlzTGlrZU5vbmUoZSk/MDphZGRUb0V4dGVybnJlZlRhYmxlMChlKX0sX193Ymdfc3RhdGljX2FjY2Vzc29yX1dJTkRPV19mODcyN2YwY2Y4ODhlMGJkOmZ1bmN0aW9uKCl7Y29uc3QgZT10eXBlb2Ygd2luZG93PiJ1Ij9udWxsOndpbmRvdztyZXR1cm4gaXNMaWtlTm9uZShlKT8wOmFkZFRvRXh0ZXJucmVmVGFibGUwKGUpfSxfX3diZ19zdWJhcnJheV9hOTZlMWZlZjE3ZWQyM2NiOmZ1bmN0aW9uKGUsbixyKXtyZXR1cm4gZS5zdWJhcnJheShuPj4+MCxyPj4+MCl9LF9fd2JnX3RvU3RyaW5nXzAyOWFjMjQ0MjFmZDdhMjQ6ZnVuY3Rpb24oZSl7cmV0dXJuIGUudG9TdHJpbmcoKX0sX193YmdfdG9TdHJpbmdfNTZkOTQ2ZGFmZjgzODY3YjpmdW5jdGlvbihlLG4scil7Y29uc3Qgbz1uLnRvU3RyaW5nKHIpLGk9cGFzc1N0cmluZ1RvV2FzbTAobyx3YXNtLl9fd2JpbmRnZW5fbWFsbG9jLHdhc20uX193YmluZGdlbl9yZWFsbG9jKSxjPVdBU01fVkVDVE9SX0xFTjtnZXREYXRhVmlld01lbW9yeTAoKS5zZXRJbnQzMihlKzQsYywhMCksZ2V0RGF0YVZpZXdNZW1vcnkwKCkuc2V0SW50MzIoZSswLGksITApfSxfX3diZ192ZXJzaW9uc180ZTMxMjI2ZjVlOGRjOTA5OmZ1bmN0aW9uKGUpe3JldHVybiBlLnZlcnNpb25zfSxfX3diaW5kZ2VuX2Nhc3RfMDAwMDAwMDAwMDAwMDAwMTpmdW5jdGlvbihlKXtyZXR1cm4gZX0sX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDI6ZnVuY3Rpb24oZSxuKXtyZXR1cm4gQmlnSW50LmFzVWludE4oNjQsZSl8bjw8QmlnSW50KDY0KX0sX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDM6ZnVuY3Rpb24oZSl7cmV0dXJuIGV9LF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDA0OmZ1bmN0aW9uKGUsbil7cmV0dXJuIGdldEFycmF5VThGcm9tV2FzbTAoZSxuKX0sX193YmluZGdlbl9jYXN0XzAwMDAwMDAwMDAwMDAwMDU6ZnVuY3Rpb24oZSxuKXtyZXR1cm4gZ2V0U3RyaW5nRnJvbVdhc20wKGUsbil9LF9fd2JpbmRnZW5fY2FzdF8wMDAwMDAwMDAwMDAwMDA2OmZ1bmN0aW9uKGUsbil7cmV0dXJuIEJpZ0ludC5hc1VpbnROKDY0LGUpfEJpZ0ludC5hc1VpbnROKDY0LG4pPDxCaWdJbnQoNjQpfSxfX3diaW5kZ2VuX2Nhc3RfMDAwMDAwMDAwMDAwMDAwNzpmdW5jdGlvbihlKXtyZXR1cm4gQmlnSW50LmFzVWludE4oNjQsZSl9LF9fd2JpbmRnZW5faW5pdF9leHRlcm5yZWZfdGFibGU6ZnVuY3Rpb24oKXtjb25zdCBlPXdhc20uX193YmluZGdlbl9leHRlcm5yZWZzLG49ZS5ncm93KDQpO2Uuc2V0KDAsdm9pZCAwKSxlLnNldChuKzAsdm9pZCAwKSxlLnNldChuKzEsbnVsbCksZS5zZXQobisyLCEwKSxlLnNldChuKzMsITEpfSxtZW1vcnk6dHx8bmV3IFdlYkFzc2VtYmx5Lk1lbW9yeSh7aW5pdGlhbDoyMSxtYXhpbXVtOjE2Mzg0LHNoYXJlZDohMH0pfX19bGV0IHdhc21Nb2R1bGUsd2FzbTtmdW5jdGlvbiBfX3diZ19maW5hbGl6ZV9pbml0KHQsXyxlKXtpZih3YXNtPXQuZXhwb3J0cyx3YXNtTW9kdWxlPV8sY2FjaGVkRGF0YVZpZXdNZW1vcnkwPW51bGwsY2FjaGVkVWludDhBcnJheU1lbW9yeTA9bnVsbCx0eXBlb2YgZTwidSImJih0eXBlb2YgZSE9Im51bWJlciJ8fGU9PT0wfHxlJTY1NTM2IT09MCkpdGhyb3ciaW52YWxpZCBzdGFjayBzaXplIjtyZXR1cm4gd2FzbS5fX3diaW5kZ2VuX3N0YXJ0KGUpLHdhc219YXN5bmMgZnVuY3Rpb24gX193YmdfbG9hZCh0LF8pe2lmKHR5cGVvZiBSZXNwb25zZT09ImZ1bmN0aW9uIiYmdCBpbnN0YW5jZW9mIFJlc3BvbnNlKXtpZih0eXBlb2YgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGVTdHJlYW1pbmc9PSJmdW5jdGlvbiIpdHJ5e3JldHVybiBhd2FpdCBXZWJBc3NlbWJseS5pbnN0YW50aWF0ZVN0cmVhbWluZyh0LF8pfWNhdGNoKHIpe2lmKHQub2smJmUodC50eXBlKSYmdC5oZWFkZXJzLmdldCgiQ29udGVudC1UeXBlIikhPT0iYXBwbGljYXRpb24vd2FzbSIpY29uc29sZS53YXJuKCJgV2ViQXNzZW1ibHkuaW5zdGFudGlhdGVTdHJlYW1pbmdgIGZhaWxlZCBiZWNhdXNlIHlvdXIgc2VydmVyIGRvZXMgbm90IHNlcnZlIFdhc20gd2l0aCBgYXBwbGljYXRpb24vd2FzbWAgTUlNRSB0eXBlLiBGYWxsaW5nIGJhY2sgdG8gYFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlYCB3aGljaCBpcyBzbG93ZXIuIE9yaWdpbmFsIGVycm9yOlxuIixyKTtlbHNlIHRocm93IHJ9Y29uc3Qgbj1hd2FpdCB0LmFycmF5QnVmZmVyKCk7cmV0dXJuIGF3YWl0IFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlKG4sXyl9ZWxzZXtjb25zdCBuPWF3YWl0IFdlYkFzc2VtYmx5Lmluc3RhbnRpYXRlKHQsXyk7cmV0dXJuIG4gaW5zdGFuY2VvZiBXZWJBc3NlbWJseS5JbnN0YW5jZT97aW5zdGFuY2U6bixtb2R1bGU6dH06bn1mdW5jdGlvbiBlKG4pe3N3aXRjaChuKXtjYXNlImJhc2ljIjpjYXNlImNvcnMiOmNhc2UiZGVmYXVsdCI6cmV0dXJuITB9cmV0dXJuITF9fWFzeW5jIGZ1bmN0aW9uIF9fd2JnX2luaXQodCxfKXtpZih3YXNtIT09dm9pZCAwKXJldHVybiB3YXNtO2xldCBlO3QhPT12b2lkIDAmJihPYmplY3QuZ2V0UHJvdG90eXBlT2YodCk9PT1PYmplY3QucHJvdG90eXBlP3ttb2R1bGVfb3JfcGF0aDp0LG1lbW9yeTpfLHRocmVhZF9zdGFja19zaXplOmV9PXQ6Y29uc29sZS53YXJuKCJ1c2luZyBkZXByZWNhdGVkIHBhcmFtZXRlcnMgZm9yIHRoZSBpbml0aWFsaXphdGlvbiBmdW5jdGlvbjsgcGFzcyBhIHNpbmdsZSBvYmplY3QgaW5zdGVhZCIpKTtjb25zdCBuPV9fd2JnX2dldF9pbXBvcnRzKF8pLHtpbnN0YW5jZTpyLG1vZHVsZTpvfT1hd2FpdCBfX3diZ19sb2FkKGF3YWl0IHQsbik7cmV0dXJuIF9fd2JnX2ZpbmFsaXplX2luaXQocixvLGUpfXZhciB0ZmhlPU9iamVjdC5mcmVlemUoe19fcHJvdG9fXzpudWxsLGRlZmF1bHQ6X193YmdfaW5pdCx3YmdfcmF5b25fc3RhcnRfd29ya2VyfSk7Cg==";
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
