import {
  init_panic_hook,
  initThreadPool,
  setWorkerUrlConfig,
} from "../../../../wasm/tfhe/tfhe.v1.5.3.js";
import init_tfhe_lib from "../../../../wasm/tfhe/tfhe.v1.5.3.js";
import {
  isomorphicCompileWasm,
  isomorphicCompileWasmFromBase64,
} from "../../../base/wasm.js";
import type { TfheModuleConfig } from "../types.js";
import {
  isBlobWorkerSupported,
  isBrowserLike,
} from "../../../base/isomorphicWorker.js";
import { threads } from "wasm-feature-detect";
import type { FhevmRuntime } from "../../../types/coreFhevmRuntime.js";
import { assertIsFhevmRuntime } from "../../../runtime/CoreFhevmRuntime-p.js";

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
  return import("../../../../wasm/tfhe/tfhe_bg.v1.5.3.wasm.base64.js");
}

const TFHE_WORKER_JS_FILENAME = "tfhe-worker.v1.5.3.mjs";
const TFHE_BG_WASM_FILENAME = "tfhe_bg.v1.5.3.wasm";

// Pure JS file (not compiled by tsc) — provides cross-platform base URL
// for resolving WASM paths. Uses import.meta.url in ESM, __filename in CJS.
import { wasmBaseUrl } from "../../../../wasm/wasmBaseUrl.js";

// Path relative to src/wasm/ where wasmBaseUrl is anchored
const nodeDefaultLocateFile = (file: string): URL => {
  return new URL(`./tfhe/${file}`, wasmBaseUrl);
};

////////////////////////////////////////////////////////////////////////////////

type TfheInitInput =
  | RequestInfo
  | URL
  | Response
  | BufferSource
  | WebAssembly.Module;

type InitTfheModuleParameters = {
  readonly module_or_path: TfheInitInput | Promise<TfheInitInput>;
  readonly memory?: WebAssembly.Memory;
  readonly thread_stack_size?: number;
  readonly num_threads?: number;
};

////////////////////////////////////////////////////////////////////////////////
// TfheModuleConfig
////////////////////////////////////////////////////////////////////////////////

type ResolvedTfheModuleConfig = {
  readonly workerUrl: URL | undefined;
  readonly wasmUrl: URL | undefined;
  /* if `true`, then `numberOfThreads` is 0, if `false` then `numberOfThreads` > 0 */
  readonly singleThread: boolean;
  readonly numberOfThreads: number;
  readonly logger: TfheModuleConfig["logger"];
};

let resolvedTfheModuleConfig: ResolvedTfheModuleConfig | undefined = undefined;

/**
 * @internal
 * Returns the existing resolved config, or resolves it from the runtime config.
 */
async function _getOrResolveTfheModuleConfig(
  runtime: FhevmRuntime,
): Promise<ResolvedTfheModuleConfig> {
  if (resolvedTfheModuleConfig !== undefined) return resolvedTfheModuleConfig;

  resolvedTfheModuleConfig = await _resolveTfheModuleConfig(runtime.config);
  return resolvedTfheModuleConfig;
}

/**
 * @internal
 * Resolves user-provided {@link TfheModuleConfig} into a fully resolved config
 * (thread count, worker URL, WASM URL). Must be called before WASM initialization.
 */
async function _resolveTfheModuleConfig(
  parameters: TfheModuleConfig,
): Promise<ResolvedTfheModuleConfig> {
  if (cachedTfheModulePromise !== undefined) {
    throw new Error("Cannot configure module after initialization has started");
  }

  const {
    locateFile,
    singleThread: singleThreadConfig,
    numberOfThreads: numberOfThreadsConfig,
  } = parameters;

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
        throw new Error("Missing locate file function");
      }
    }
  }

  let numberOfThreads: number | undefined;

  if (!singleThread) {
    numberOfThreads = numberOfThreadsConfig ?? navigator.hardwareConcurrency; // Node 21+

    if (numberOfThreads > 0) {
      // SharedArrayBuffer requires COOP/COEP headers in browsers.
      // Fall back to single-threaded mode when unavailable.
      const supportsThreads = await threads();
      if (!supportsThreads) {
        console.warn(
          "This browser does not support threads. Verify that your server returns correct headers:\n",
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

  if (cachedTfheModulePromise !== undefined) {
    return cachedTfheModulePromise;
  }

  // Use existing config if already set, otherwise resolve from runtime
  const cfg = await _getOrResolveTfheModuleConfig(runtime);

  cachedTfheModulePromise = _initTfheModule(cfg);

  // This is purely theoretical. Retry is not yet possible since the `_initTfheModule`
  // does not support retry (see tfhe lib internal global variables).
  cachedTfheModulePromise.catch(() => {
    // Clear cache on failure so retry is possible
    cachedTfheModulePromise = undefined;
  });

  return cachedTfheModulePromise;
}

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
}
