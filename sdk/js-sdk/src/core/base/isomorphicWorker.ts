import type { MessageData, NodeMessagePort } from './environment.js';
import { getNodeWorker, supportsWebWorkerApi } from './environment.js';

export async function supportsNodeWorkerApi(): Promise<boolean> {
  return (await getNodeWorker()) !== undefined;
}

export type WorkerApi = 'web' | 'node';

let _resolvedWorkerApi: Promise<WorkerApi> | undefined;

/**
 * Resolves which worker backend the SDK will use, preferring the Web Worker
 * API whenever it is available. Memoized — the answer can't change within a
 * process lifetime.
 *
 * - Web is preferred because it works in every browser context, including the
 *   sandboxed Electron renderer (where `node:worker_threads` is unavailable
 *   even though `process.versions.node` is set).
 * - Node is the fallback for runtimes with no Web Worker global (Node, jsdom).
 *
 * Runtime support matrix:
 *
 * | Runtime                       | Web API | Node API | Selected |
 * | ----------------------------- | :-----: | :------: | :------: |
 * | Browser (window / web worker) |   yes   |    no    |   web    |
 * | Sandboxed Electron renderer   |   yes   |    no    |   web    |
 * | Node.js                       |   no    |   yes    |   node   |
 * | jsdom (Vitest)                |   no    |   yes    |   node   |
 * | Deno                          |   yes   |   yes    |   web    |
 * | Bun                           |   yes   |   yes    |  node *  |
 *
 * * Bun supports both backends, but is forced to `node`: it has first-class
 *   `worker_threads` support and we keep it on the Node path for parity with
 *   plain Node.
 *
 * Note: this reports the *intended* API based on API availability, not a
 * guarantee of success — Web Worker creation can still fail at the call site
 * (e.g. CSP blocking `worker-src blob:`). The creation site must surface that.
 */
export function resolveWorkerApi(): Promise<WorkerApi> {
  _resolvedWorkerApi ??= (async () => {
    try {
      // @ts-expect-error - Bun is a runtime global only under Bun
      const isBun = typeof Bun !== 'undefined';

      // Preference order is runtime-specific:
      // - Bun has first-class worker_threads, so prefer Node for parity with
      //   plain Node, falling back to Web only if it's somehow unavailable.
      // - Everywhere else, prefer the Web Worker API (browsers, the sandboxed
      //   Electron renderer, Deno), falling back to Node (Node.js, jsdom).
      //
      // Probes stay short-circuited on purpose: on the non-Bun path the
      // synchronous Web check runs first so a browser never triggers the
      // `node:worker_threads` dynamic import (the "sync win"). Each probe runs
      // at most once per call.
      if (isBun) {
        if (await supportsNodeWorkerApi()) {
          return 'node';
        }
        if (supportsWebWorkerApi()) {
          return 'web';
        }
      } else {
        if (supportsWebWorkerApi()) {
          return 'web';
        }
        if (await supportsNodeWorkerApi()) {
          return 'node';
        }
      }

      throw new Error('No worker backend available (neither Web Worker nor node:worker_threads).');
    } catch (e) {
      // Never cache a rejected resolution — otherwise a single (possibly
      // transient) failure would poison every future call. The underlying
      // probes are themselves memoized, so retrying stays cheap.
      _resolvedWorkerApi = undefined;
      throw e;
    }
  })();
  return _resolvedWorkerApi;
}

/*
  TODO: add support for TrustedScriptURL if needed
*/
async function createIsomorphicWorkerFromCode(jsCode: string): Promise<Worker | NodeMessagePort> {
  const workerApi = await resolveWorkerApi();

  //if (isBrowserLike()) {
  if (workerApi === 'web') {
    const blob = new Blob([jsCode], { type: 'application/javascript' });
    const blobUrl = URL.createObjectURL(blob);
    try {
      const browserWorker = new Worker(blobUrl);
      return browserWorker;
    } finally {
      URL.revokeObjectURL(blobUrl);
    }
  }

  // workerApi === 'node'. Both 'node' paths in resolveWorkerApi pre-probe via
  // supportsNodeWorkerApi, so this is already confirmed (and memoized) — the
  // guard is here to narrow the `| undefined` type and to stay correct if that
  // invariant ever changes.
  const NodeWorker = await getNodeWorker();
  if (!NodeWorker) {
    throw new Error('No worker backend available (node:worker_threads unavailable).');
  }
  return new NodeWorker(jsCode, { eval: true });
}

/**
 * Runs code in an isomorphic worker thread and returns the result.
 *
 * The `code` string is wrapped in an async IIFE that receives `data` as input.
 * Use `return` to send the result back to the main thread.
 *
 * @example
 * const module = await runCodeInIsomorphicWorker<WebAssembly.Module>(
 *   `const res = await fetch("data:application/octet-stream;base64," + data);
 *    const bytes = new Uint8Array(await res.arrayBuffer());
 *    return WebAssembly.compile(bytes);`,
 *   base64,
 * );
 *
 * @param code - JS code to execute. Receives `data` as input, must `return` the result.
 * @param input - Value sent to the worker via postMessage (must be structured-cloneable).
 * @param timeoutMs - Max execution time before the worker is killed. Default: 30s.
 */
async function runCodeInIsomorphicWorker<T>(code: string, input: unknown, timeoutMs: number = 30_000): Promise<T> {
  const workerApi = await resolveWorkerApi();

  const browserCode = `
    self.onmessage = async ({ data }) => {
      try {
        const result = await (async (data) => { ${code} })(data);
        self.postMessage({ result });
      } catch (e) {
        self.postMessage({ error: String(e) });
      }
    };
  `;

  const nodeCode = `
    const { parentPort } = require("worker_threads");
    parentPort.on("message", async (data) => {
      try {
        const result = await (async (data) => { ${code} })(data);
        parentPort.postMessage({ result });
      } catch (e) {
        parentPort.postMessage({ error: String(e) });
      }
    });
  `;

  //const workerCode = isBrowserLike() ? browserCode : nodeCode;
  const workerCode = workerApi === 'web' ? browserCode : nodeCode;
  const worker = await createIsomorphicWorkerFromCode(workerCode);

  return new Promise<T>((resolve, reject) => {
    // Guards against double-settle (e.g. "exit" firing after "message")
    let settled = false;

    // Terminates the worker, clears the timer, and prevents further settles
    const cleanup = (): void => {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      // fire and forget. Do not await the promise (only in Nodejs)
      try {
        // eslint-disable-next-line @typescript-eslint/no-floating-promises
        worker.terminate();
      } catch (_) {
        /* already dead */
      }
    };

    // Normalizes any error and rejects the promise
    const fail = (e: unknown): void => {
      cleanup();
      reject(e instanceof Error ? e : new Error(String(e)));
    };

    // Processes the worker's { result, error } envelope
    const handle = (msg: Record<string, unknown>): void => {
      cleanup();
      if (msg.error !== undefined) {
        reject(new Error(typeof msg.error === 'string' ? msg.error : JSON.stringify(msg.error)));
      } else {
        resolve(msg.result as T);
      }
    };

    // Rejects if the worker takes too long (e.g. infinite loop, hung fetch)
    // Declared after fail/handle so all references are resolved before the timer can fire
    const timer = setTimeout(() => {
      if (!settled) fail(new Error(`Worker timed out after ${String(timeoutMs)}ms`));
    }, timeoutMs);

    // Bind listeners and send input to the worker
    if ('on' in worker) {
      // Node: EventEmitter API
      worker.on('message', (data: MessageData) => {
        handle(data);
      });
      worker.on('error', (e: Error) => {
        fail(e);
      });
      worker.on('exit', (exitCode: number) => {
        if (!settled) fail(new Error(`Worker exited with code ${String(exitCode)}`));
      });
      worker.postMessage(input);
    } else {
      // Browser: DOM event API
      worker.onmessage = ({ data }: MessageEvent) => {
        handle(data as Record<string, unknown>);
      };
      worker.onerror = (e: ErrorEvent) => {
        fail(new Error(e.message));
      };
      worker.postMessage(input);
    }
  });
}

/**
 * Smoke-tests the full inline worker pipeline (Blob URL in browser, eval in Node).
 *
 * Sends `"hello"` as input, executes `data + " world!"` in a worker, and verifies
 * the result is `"hello world!"`. This validates:
 * - Worker creation (Blob URL or eval mode)
 * - postMessage input delivery
 * - Code execution inside the worker
 * - Result returned via postMessage
 *
 * Returns `false` (instead of throwing) when inline workers are blocked:
 *
 * Browser:
 * - CSP `worker-src` does not allow `blob:` — the most common failure case.
 *   @see https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/worker-src
 * - CSP `script-src` blocks `unsafe-eval` (some engines treat Blob workers as eval).
 *   @see https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/script-src
 * - Sandboxed iframe without `allow-scripts`.
 *   @see https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#sandbox
 *
 * Node.js:
 * - `--disallow-code-generation-from-strings` flag blocks `{ eval: true }` workers.
 *   @see https://nodejs.org/api/cli.html#--disallow-code-generation-from-strings
 * - Experimental permission model restricts worker creation.
 *   @see https://nodejs.org/api/permissions.html
 *
 * When this returns `false`, blob/eval-based workers are unavailable in this
 * environment (e.g. blocked by CSP `worker-src`/`script-src`, or Node's
 * code-generation restrictions), and code cannot be run in an isomorphic worker.
 */
let _blobWorkerSupportedPromise: Promise<boolean> | undefined;
export function isBlobWorkerSupported(): Promise<boolean> {
  // eslint-disable-next-line @typescript-eslint/no-unnecessary-type-arguments
  _blobWorkerSupportedPromise ??= runCodeInIsomorphicWorker<string>(`return data + " world!";`, 'hello', 5000).then(
    (res) => res === 'hello world!',
    () => false,
  );
  return _blobWorkerSupportedPromise;
}
