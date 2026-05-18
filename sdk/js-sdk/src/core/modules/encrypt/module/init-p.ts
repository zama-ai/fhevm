import type { FhevmRuntime, FhevmRuntimeConfig } from '../../../types/coreFhevmRuntime.js';
import type { GetTfheModuleInfoReturnType, TfheModuleInfo } from '../types.js';
import type { TfheLibApi } from '../../../../wasm/tfhe/TfheApi.js';
import type { TfheAssetMetadata, TfheVersion } from '../../../../wasm/tfhe/loadTfheLib.js';
import { isomorphicCompileVerifiedWasm, isomorphicCompileWasmFromBase64 } from '../../../base/wasm.js';
import { isBlobWorkerSupported, isBrowserLike } from '../../../base/isomorphicWorker.js';
import { threads } from 'wasm-feature-detect';
import { assertIsFhevmRuntime } from '../../../runtime/CoreFhevmRuntime-p.js';
import { resolveTfheModuleVersion } from '../../../../wasm/tfheModuleVersion-p.js';
// Pure JS file (not compiled by tsc) — provides cross-platform base URL
// for resolving WASM paths. Uses import.meta.url in ESM, __filename in CJS.
import { wasmBaseUrl } from '../../../../wasm/wasmBaseUrl.js';
import { loadTfheLib, loadTfheWasmBase64, tfheAssetsWithVersion } from '../../../../wasm/tfhe/loadTfheLib.js';

////////////////////////////////////////////////////////////////////////////////

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

type ResolvedTfheAsset = TfheAssetMetadata & {
  readonly url: URL | undefined;
};

function _resolveTfheAsset(asset: TfheAssetMetadata, locateFile: FhevmRuntimeConfig['locateFile']): ResolvedTfheAsset {
  let url: URL | undefined;

  if (locateFile !== undefined) {
    url = locateFile(asset.filename);
  } else if (!isBrowserLike()) {
    url = nodeDefaultLocateFile(asset.localRelativePath);
  }

  return Object.freeze({ ...asset, url });
}

////////////////////////////////////////////////////////////////////////////////
// ResolvedTfheModuleConfig
////////////////////////////////////////////////////////////////////////////////

type ResolvedTfheModuleConfig = {
  readonly version: TfheVersion;
  readonly worker: ResolvedTfheAsset;
  readonly wasmAssetLoadMode: FhevmRuntimeConfig['wasmAssetLoadMode'];
  readonly wasm: ResolvedTfheAsset;
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

  const version = resolveTfheModuleVersion(runtime.config);

  resolvingTfheModuleConfigPromise ??= _resolveTfheModuleConfig(runtime.config, version)
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
async function _resolveTfheModuleConfig(
  parameters: FhevmRuntimeConfig,
  version: TfheVersion,
): Promise<ResolvedTfheModuleConfig> {
  const {
    locateFile,
    wasmAssetLoadMode,
    singleThread: singleThreadConfig,
    numberOfThreads: numberOfThreadsConfig,
  } = parameters;

  let singleThread = false;
  if (singleThreadConfig !== undefined) {
    singleThread = singleThreadConfig;
  }

  const canUseBlob = await isBlobWorkerSupported();

  const assets = tfheAssetsWithVersion(version);
  const wasm = _resolveTfheAsset(assets.wasm, locateFile);
  const worker = _resolveTfheAsset(assets.worker, locateFile);

  if (locateFile !== undefined) {
    parameters.logger?.debug(`resolve tfhe wasm filename: ${wasm.filename} -> url: ${wasm.url}`);
    parameters.logger?.debug(`resolve tfhe worker filename: ${worker.filename} -> url: ${worker.url}`);
  } else {
    /*
      if run in Node only, use defaultLocateFile!
    */
    if (isBrowserLike()) {
      if (!canUseBlob) {
        throw new Error('Missing locate file function');
      }
    } else {
      parameters.logger?.debug(`resolve tfhe wasm local path: ${wasm.localRelativePath} -> url: ${wasm.url}`);
      parameters.logger?.debug(`resolve tfhe worker local path: ${worker.localRelativePath} -> url: ${worker.url}`);
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

  const tfheLib = await loadTfheLib(version);
  tfheLib.setWorkerUrlConfig({
    workerUrl: worker.url,
    wasmAssetLoadMode,
    logger: parameters.logger,
  });

  const cfg = {
    version,
    numberOfThreads,
    worker,
    wasmAssetLoadMode,
    wasm,
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

let cachedTfheModulePromise: Promise<TfheLibApi> | undefined;
let ownerUid: string | undefined = undefined;

/**
 * Initializes the TFHE module and returns the loaded lib bindings.
 * Idempotent: subsequent calls return the same cached lib instance.
 */
export async function initTfheModule(runtime: FhevmRuntime): Promise<TfheLibApi> {
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
      return await _initTfheModule(cfg);
    })();
  }

  return cachedTfheModulePromise;
}

let moduleInfo: TfheModuleInfo | undefined = undefined;

async function _initTfheModule(cfg: ResolvedTfheModuleConfig): Promise<TfheLibApi> {
  const tfheLib = await loadTfheLib(cfg.version);

  // Compile WASM module (see matrix in types.ts)
  let wasmModule;
  if (cfg.wasm.url !== undefined) {
    cfg.logger?.debug(`compile verified wasm at: ${cfg.wasm.url}`);
    wasmModule = await isomorphicCompileVerifiedWasm(cfg.wasm.url, cfg.wasm.sha256);
  } else {
    const { tfheWasmBase64, tfheWasmBase64CompressionFormat } = await loadTfheWasmBase64(cfg.version);
    cfg.logger?.debug(`compile wasm from embedded base64 (compression:${tfheWasmBase64CompressionFormat ?? 'none'})`);
    wasmModule = await isomorphicCompileWasmFromBase64(tfheWasmBase64, tfheWasmBase64CompressionFormat);
  }

  const input: InitTfheModuleParameters = { module_or_path: wasmModule };

  // 2. Load and instantiate the TFHE WASM binary
  await tfheLib.default(input);

  // 3. Route WASM panics to console.error instead of silently aborting
  tfheLib.init_panic_hook();

  // 4. Spawn Web Workers for parallel FHE operations (skipped when single-threaded)
  if (!cfg.singleThread) {
    cfg.logger?.debug(`initThreadPool(${cfg.numberOfThreads})`);
    await tfheLib.initThreadPool(cfg.numberOfThreads);
  }

  const wasmInfo = tfheLib.getWasmInfo();

  moduleInfo = Object.freeze({
    wasmUrl: cfg.wasm.url ? new URL(cfg.wasm.url) : undefined,
    version: wasmInfo.version,
    name: wasmInfo.name,
    workerUrl: cfg.worker.url ? new URL(cfg.worker.url) : undefined,
    numberOfThreads: cfg.singleThread ? 0 : cfg.numberOfThreads,
    threadsAvailable: cfg.supportsThreads,
  });

  return tfheLib;
}

////////////////////////////////////////////////////////////////////////////////
// getTfheModuleInfo
////////////////////////////////////////////////////////////////////////////////

export function getTfheModuleInfo(): GetTfheModuleInfoReturnType {
  return moduleInfo;
}
