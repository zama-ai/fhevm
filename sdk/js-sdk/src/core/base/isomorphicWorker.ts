interface NodeMessagePort {
  on(event: "error", listener: (error: Error) => void): void;
  on(event: "exit", listener: (code: number) => void): void;
  on(event: string, listener: (data: MessageData) => void): void;
  off(event: string, listener: (data: MessageData) => void): void;
  postMessage(value: unknown): void;
  terminate(): Promise<number>;
}

interface BrowserMessagePort {
  addEventListener(
    event: string,
    listener: (event: MessageEvent<MessageData>) => void,
  ): void;
  removeEventListener(
    event: string,
    listener: (event: MessageEvent<MessageData>) => void,
  ): void;
  postMessage(value: unknown): void;
  terminate(): void;
}

export type IsomorphicMessagePort = NodeMessagePort | BrowserMessagePort;

interface MessageData {
  type: string;
  [key: string]: unknown;
}

type NodeWorkerConstructor = new (
  code: string | URL,
  options?: Record<string, unknown>,
) => NodeMessagePort;

function isNodePort(target: IsomorphicMessagePort): target is NodeMessagePort {
  return typeof (target as NodeMessagePort).on === "function";
}

export function isBrowserLike(): boolean {
  return (
    typeof addEventListener === "function" &&
    typeof removeEventListener === "function"
  );
}

/**
 * This function is called from inside a worker to get the messaging target
 * It's the communication channel between a worker thread and its parent (main) thread.
 */
export async function getIsomorphicTarget(): Promise<IsomorphicMessagePort> {
  if (isBrowserLike()) return self as unknown as BrowserMessagePort;
  const nodeWorkerModuleName = "worker_threads";
  const nodeWorkerModuleId = `node:${nodeWorkerModuleName}`;
  const { parentPort: nodeParentPort } = (await import(nodeWorkerModuleId)) as {
    parentPort: NodeMessagePort | null;
  };
  if (!nodeParentPort) {
    throw new Error("Not running inside a worker thread");
  }
  return nodeParentPort;
}

export async function createIsomorphicWorker(
  url: string,
): Promise<Worker | NodeMessagePort> {
  if (isBrowserLike()) {
    return createBrowserLikeWorker(url);
  }
  return createNodeLikeWorker(url);
}

export async function createIsomorphicWorkerFromCode(
  jsCode: string,
): Promise<Worker | NodeMessagePort> {
  if (isBrowserLike()) {
    const blob = new Blob([jsCode], { type: "application/javascript" });
    const blobUrl = URL.createObjectURL(blob);
    try {
      const browserWorker = new Worker(blobUrl);
      return browserWorker;
    } finally {
      URL.revokeObjectURL(blobUrl);
    }
  }
  const nodeWorkerModuleName = "worker_threads";
  const nodeWorkerModuleId = `node:${nodeWorkerModuleName}`;
  const { Worker: NodeWorker } = (await import(nodeWorkerModuleId)) as {
    Worker: NodeWorkerConstructor;
  };
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
export async function runCodeInIsomorphicWorker<T>(
  code: string,
  input: unknown,
  timeoutMs: number = 30_000,
): Promise<T> {
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

  const workerCode = isBrowserLike() ? browserCode : nodeCode;
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
        reject(
          new Error(
            typeof msg.error === "string"
              ? msg.error
              : JSON.stringify(msg.error),
          ),
        );
      } else {
        resolve(msg.result as T);
      }
    };

    // Rejects if the worker takes too long (e.g. infinite loop, hung fetch)
    // Declared after fail/handle so all references are resolved before the timer can fire
    const timer = setTimeout(() => {
      if (!settled)
        fail(new Error(`Worker timed out after ${String(timeoutMs)}ms`));
    }, timeoutMs);

    // Bind listeners and send input to the worker
    if ("on" in worker) {
      // Node: EventEmitter API
      worker.on("message", (data: MessageData) => {
        handle(data);
      });
      worker.on("error", (e: Error) => {
        fail(e);
      });
      worker.on("exit", (exitCode: number) => {
        if (!settled)
          fail(new Error(`Worker exited with code ${String(exitCode)}`));
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
 * When this returns `false`, fall back to URL-based workers via `createIsomorphicWorker()`.
 */
let _blobWorkerSupportedPromise: Promise<boolean> | undefined;
export function isBlobWorkerSupported(): Promise<boolean> {
  _blobWorkerSupportedPromise ??= runCodeInIsomorphicWorker<string>(
    `return data + " world!";`,
    "hello",
    5000,
  ).then(
    (res) => res === "hello world!",
    () => false,
  );
  return _blobWorkerSupportedPromise;
}

function createBrowserLikeWorker(url: string): Worker {
  return new Worker(url, {
    type: "module",
    name: "wasm_bindgen_worker",
  });
}

async function createNodeLikeWorker(url: string): Promise<NodeMessagePort> {
  const nodeWorkerModuleName = "worker_threads";
  const nodeWorkerModuleId = `node:${nodeWorkerModuleName}`;
  const { Worker: NodeWorker } = (await import(nodeWorkerModuleId)) as {
    Worker: NodeWorkerConstructor;
  };
  // Node's Worker doesn't support data: or blob: URLs.
  // For data: URLs, extract the code and use eval mode.
  if (url.startsWith("data:")) {
    const base64 = url.split(",")[1];
    if (base64 === undefined) {
      throw new Error("Invalid data url");
    }

    const nodeBufferModuleName = "buffer";
    const nodeBufferModuleId = `node:${nodeBufferModuleName}`;
    const { Buffer: NodeBuffer } = (await import(nodeBufferModuleId)) as {
      Buffer: {
        from(str: string, encoding: string): { toString(enc: string): string };
      };
    };
    //const res = await fetch("data:application/octet-stream;base64," + data);
    const code = NodeBuffer.from(base64, "base64").toString("utf-8");
    return new NodeWorker(code, { eval: true });
  }
  return new NodeWorker(url);
}

export function waitForMsgType(
  target: IsomorphicMessagePort,
  type: string,
): Promise<MessageData> {
  return new Promise((resolve) => {
    if (isNodePort(target)) {
      // Node: EventEmitter, data passed directly
      target.on("message", function onMsg(data: MessageData) {
        if (data.type !== type) return;
        target.off("message", onMsg);
        resolve(data);
      });
    } else {
      // Browser: DOM events, data wrapped in MessageEvent
      target.addEventListener(
        "message",
        function onMsg({ data }: MessageEvent<MessageData>) {
          if (data.type !== type) return;
          target.removeEventListener("message", onMsg);
          resolve(data);
        },
      );
    }
  });
}
