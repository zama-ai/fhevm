import type { FhevmRuntime, FhevmRuntimeConfig } from '../../../types/coreFhevmRuntime.js';
import init_tfhe_lib from '../../../../wasm/tfhe/tfhe.v1.5.3.js';
import { init_panic_hook, initThreadPool, setWorkerUrlConfig, getWasmInfo } from '../../../../wasm/tfhe/tfhe.v1.5.3.js';
import { isomorphicCompileWasm, isomorphicCompileWasmFromBase64 } from '../../../base/wasm.js';
import { isBlobWorkerSupported, isBrowserLike } from '../../../base/isomorphicWorker.js';
import { threads } from 'wasm-feature-detect';
import { assertIsFhevmRuntime } from '../../../runtime/CoreFhevmRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

// IMPORTANT: The import path MUST be a string literal, not a variable.
// Bundlers (Webpack, Vite, Rollup) statically analyze import() calls to
// create separate chunks for code-splitting. A variable path like
// `import(someVar)` makes the target unresolvable — the bundler either
// includes every possible file or fails entirely.
// With a literal path, the bundler creates a lazy-loaded chunk for this
// ~6.5MB base64 file, only downloaded when this function is called.
function dynamicImportWasmBase64(): Promise<{
  readonly tfheWasmBase64: string;
}> {
  // Bundler Alert: !! KEEP THE PATH AS-IS !!
  return import('../../../../wasm/tfhe/tfhe_bg.v1.5.3.wasm.base64.js');
}

const TFHE_WORKER_JS_FILENAME = 'tfhe-worker.v1.5.3.mjs';
const TFHE_BG_WASM_FILENAME = 'tfhe_bg.v1.5.3.wasm';

////////////////////////////////////////////////////////////////////////////////

// Pure JS file (not compiled by tsc) — provides cross-platform base URL
// for resolving WASM paths. Uses import.meta.url in ESM, __filename in CJS.
import { wasmBaseUrl } from '../../../../wasm/wasmBaseUrl.js';
import type { GetTfheModuleInfoReturnType, TfheModuleInfo } from '../types.js';

// (Node only) Path relative to src/wasm/ where wasmBaseUrl is anchored
const nodeDefaultLocateFile = (file: string): URL => {
  return new URL(`./tfhe/${file}`, wasmBaseUrl);
};

////////////////////////////////////////////////////////////////////////////////

type TfheInitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

type InitTfheModuleParameters = {
  readonly module_or_path: TfheInitInput | Promise<TfheInitInput>;
  readonly memory?: WebAssembly.Memory;
  readonly thread_stack_size?: number;
  readonly num_threads?: number;
};

////////////////////////////////////////////////////////////////////////////////
// ResolvedTfheModuleConfig
////////////////////////////////////////////////////////////////////////////////

type ResolvedTfheModuleConfig = {
  readonly workerUrl: URL | undefined;
  readonly wasmUrl: URL | undefined;
  /* if `true`, then `numberOfThreads` is 0, if `false` then `numberOfThreads` > 0 */
  readonly singleThread: boolean;
  readonly numberOfThreads: number;
  readonly logger: FhevmRuntimeConfig['logger'];
  readonly supportsThreads: boolean | undefined;
};

let resolvedTfheModuleConfig: ResolvedTfheModuleConfig | undefined = undefined;
let resolvingTfheModuleConfigPromise: Promise<ResolvedTfheModuleConfig> | undefined;

/**
 * @internal
 * Returns the existing resolved config, or resolves it from the runtime config.
 */
async function _getOrResolveTfheModuleConfig(runtime: FhevmRuntime): Promise<ResolvedTfheModuleConfig> {
  if (resolvedTfheModuleConfig !== undefined) {
    return resolvedTfheModuleConfig;
  }

  resolvingTfheModuleConfigPromise ??= _resolveTfheModuleConfig(runtime.config)
    .then((cfg) => {
      resolvedTfheModuleConfig = cfg;
      return cfg;
    })
    .catch((error: unknown) => {
      resolvingTfheModuleConfigPromise = undefined;
      throw error;
    });

  return resolvingTfheModuleConfigPromise;
}

/**
 * @internal
 * Resolves user-provided {@link FhevmRuntimeConfig} into a fully resolved config
 * (thread count, worker URL, WASM URL). Must be called before WASM initialization.
 */
async function _resolveTfheModuleConfig(parameters: FhevmRuntimeConfig): Promise<ResolvedTfheModuleConfig> {
  const { locateFile, singleThread: singleThreadConfig, numberOfThreads: numberOfThreadsConfig } = parameters;

  let singleThread = false;
  if (singleThreadConfig !== undefined) {
    singleThread = singleThreadConfig;
  }

  const canUseBlob = await isBlobWorkerSupported();

  let wasmUrl: URL | undefined;
  let workerUrl: URL | undefined;

  if (locateFile !== undefined) {
    workerUrl = locateFile(TFHE_WORKER_JS_FILENAME);
    wasmUrl = locateFile(TFHE_BG_WASM_FILENAME);
  } else {
    /*
      if run in Node only, use defaultLocateFile!
    */
    if (!isBrowserLike()) {
      workerUrl = nodeDefaultLocateFile(TFHE_WORKER_JS_FILENAME);
      wasmUrl = nodeDefaultLocateFile(TFHE_BG_WASM_FILENAME);
    } else {
      if (!canUseBlob) {
        throw new Error('Missing locate file function');
      }
    }
  }

  let numberOfThreads: number | undefined;
  let supportsThreads: boolean | undefined;

  if (!singleThread) {
    numberOfThreads = numberOfThreadsConfig ?? navigator.hardwareConcurrency; // Node 21+

    if (numberOfThreads > 0) {
      // SharedArrayBuffer requires COOP/COEP headers in browsers.
      // Fall back to single-threaded mode when unavailable.
      supportsThreads = await threads();
      if (!supportsThreads) {
        console.warn(
          'This browser does not support threads. Verify that your server returns correct headers:\n',
          "'Cross-Origin-Opener-Policy': 'same-origin'\n",
          "'Cross-Origin-Embedder-Policy': 'require-corp'",
        );
        singleThread = true;
        numberOfThreads = 0;
      }
    } else {
      singleThread = true;
      numberOfThreads = 0;
    }
  } else {
    numberOfThreads = 0;
  }

  setWorkerUrlConfig({
    workerUrl,
    logger: parameters.logger,
  });

  const cfg = {
    numberOfThreads,
    workerUrl,
    wasmUrl,
    singleThread,
    logger: parameters.logger,
    supportsThreads,
  };

  parameters.logger?.debug(JSON.stringify(cfg, null, 2));

  return cfg;
}

////////////////////////////////////////////////////////////////////////////////
// initTfheModule
////////////////////////////////////////////////////////////////////////////////

let cachedTfheModulePromise: Promise<void> | undefined;
let ownerUid: string | undefined = undefined;

/**
 * Initializes the TFHE module.
 */
export async function initTfheModule(runtime: FhevmRuntime): Promise<void> {
  assertIsFhevmRuntime(runtime, {});

  if (ownerUid !== undefined && runtime.uid !== ownerUid) {
    throw new Error(
      `Encrypt WASM module is already owned by runtime '${ownerUid}' and cannot be shared with runtime '${runtime.uid}'`,
    );
  }

  ownerUid = runtime.uid;

  // Cache the whole initialization promise before the first await. Several
  // clients may call initTfheModule concurrently during startup; if the promise
  // were assigned after resolving the config, each caller could enter
  // _initTfheModule and try to start the global TFHE worker pool independently.
  // The worker pool is process-wide and startWorkers() is intentionally
  // one-shot, so every concurrent caller must await this same promise.
  //
  // Retry is not supported:
  // -----------------------
  // TFHE/WASM initialization and worker startup mutate
  // lower-level module globals that cannot be reset reliably after a partial
  // failure. Keep even a rejected promise cached so later callers observe the
  // original initialization error instead of retrying against half-initialized
  // state and producing secondary errors such as "Already started".

  // eslint-disable-next-line @typescript-eslint/prefer-nullish-coalescing
  if (cachedTfheModulePromise === undefined) {
    cachedTfheModulePromise = (async () => {
      // resolve is async
      const cfg = await _getOrResolveTfheModuleConfig(runtime);
      await _initTfheModule(cfg);
    })();
  }

  return cachedTfheModulePromise;
}

let moduleInfo: TfheModuleInfo | undefined = undefined;

async function _initTfheModule(cfg: ResolvedTfheModuleConfig): Promise<void> {
  // Compile WASM module (see matrix in types.ts)
  let wasmModule;
  if (cfg.wasmUrl !== undefined) {
    cfg.logger?.debug(`compile wasm at: ${cfg.wasmUrl}`);
    wasmModule = await isomorphicCompileWasm(cfg.wasmUrl);
  } else {
    cfg.logger?.debug(`compile wasm from embedded base64`);
    const { tfheWasmBase64 } = await dynamicImportWasmBase64();
    wasmModule = await isomorphicCompileWasmFromBase64(tfheWasmBase64);
  }

  const input: InitTfheModuleParameters = { module_or_path: wasmModule };

  // 2. Load and instantiate the TFHE WASM binary
  await init_tfhe_lib(input);

  // 3. Route WASM panics to console.error instead of silently aborting
  init_panic_hook();

  // 4. Spawn Web Workers for parallel FHE operations (skipped when single-threaded)
  if (!cfg.singleThread) {
    cfg.logger?.debug(`initThreadPool(${cfg.numberOfThreads})`);
    await initThreadPool(cfg.numberOfThreads);
  }

  const wasmInfo = getWasmInfo();

  moduleInfo = Object.freeze({
    wasmUrl: cfg.wasmUrl ? new URL(cfg.wasmUrl) : undefined,
    version: wasmInfo.version,
    name: wasmInfo.name,
    workerUrl: cfg.workerUrl ? new URL(cfg.workerUrl) : undefined,
    numberOfThreads: cfg.singleThread ? 0 : cfg.numberOfThreads,
    threadsAvailable: cfg.supportsThreads,
  });
}

////////////////////////////////////////////////////////////////////////////////
// getTfheModuleInfo
////////////////////////////////////////////////////////////////////////////////

export function getTfheModuleInfo(): GetTfheModuleInfoReturnType {
  return moduleInfo;
}
