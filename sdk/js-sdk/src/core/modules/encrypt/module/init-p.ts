import type { FhevmRuntime, FhevmRuntimeConfig } from '../../../types/coreFhevmRuntime.js';
import type {
  GetTfheModuleInfoParameters,
  GetTfheModuleInfoReturnType,
  InitTfheModuleParameters,
  TfheModuleInfo,
} from '../types.js';
import type { TfheLibApi } from '../../../../wasm/tfhe/TfheApi.js';
import type { TfheAssetMetadata, TfheAssets, TfheVersion } from '../../../../wasm/tfhe/loadTfheLib.js';
import { isomorphicCompileVerifiedWasm, isomorphicCompileWasmFromBase64 } from '../../../base/wasm.js';
import { isBlobWorkerSupported, isBrowserLike } from '../../../base/isomorphicWorker.js';
import { threads } from 'wasm-feature-detect';
import { assertIsFhevmRuntime } from '../../../runtime/CoreFhevmRuntime-p.js';
// Pure JS file (not compiled by tsc) — provides cross-platform base URL
// for resolving WASM paths. Uses import.meta.url in ESM, __filename in CJS.
import { wasmBaseUrl } from '../../../../wasm/wasmBaseUrl.js';
import { loadTfheLib, loadTfheWasmBase64, tfheAssetsWithVersion } from '../../../../wasm/tfhe/loadTfheLib.js';
import { isomorphicFileUrlExists } from '../../../base/isomorphicFs.js';
import type { WasmAssetLoadMode } from '../../../types/wasmAssets.js';

////////////////////////////////////////////////////////////////////////////////

// (Node only) Path relative to src/wasm/ where wasmBaseUrl is anchored
const nodeDefaultLocateFile = (file: string): URL => {
  return new URL(`./tfhe/${file}`, wasmBaseUrl);
};

////////////////////////////////////////////////////////////////////////////////

type TfheInitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

type TfheLibInitAsyncParameters = {
  readonly module_or_path: TfheInitInput | Promise<TfheInitInput>;
  readonly memory?: WebAssembly.Memory;
  readonly thread_stack_size?: number;
  readonly num_threads?: number;
};

type TfheAssetResolution = 'user' | 'node' | 'none';
type ResolvedTfheAsset = TfheAssetMetadata & {
  readonly url: URL | undefined;
  readonly resolution: TfheAssetResolution;
};

function _resolveTfheAsset(asset: TfheAssetMetadata, locateFile: FhevmRuntimeConfig['locateFile']): ResolvedTfheAsset {
  let url: URL | undefined;
  let resolution: TfheAssetResolution = 'none';

  if (locateFile !== undefined) {
    url = locateFile(asset.filename);
    resolution = 'user';
  } else if (!isBrowserLike()) {
    url = nodeDefaultLocateFile(asset.localRelativePath);
    resolution = 'node';
  }

  return Object.freeze({ ...asset, url, resolution });
}

/**
 * Resolves all TFHE assets (wasm + worker) as a single set.
 *
 * The resolution mode is identical across assets (they share `locateFile` and
 * the same runtime), so it is decided once. For `'node'` resolution (auto-derived
 * `file://` URLs), every URL is validated on disk: if any one is missing — e.g. a
 * bundler such as Turbopack relocated the package and the derived `file://ROOT/...`
 * path no longer exists — then ALL URLs are cleared so the whole set falls back to
 * embedded base64 consistently. Mixing disk + base64 across assets is never allowed.
 *
 * `'user'` URLs are returned untouched (they fail loud later if wrong); `'none'`
 * has no URL to validate.
 */
async function _resolveTfheAssets(
  assets: TfheAssets,
  locateFile: FhevmRuntimeConfig['locateFile'],
): Promise<{ readonly wasm: ResolvedTfheAsset; readonly worker: ResolvedTfheAsset }> {
  let wasm = _resolveTfheAsset(assets.wasm, locateFile);
  let worker = _resolveTfheAsset(assets.worker, locateFile);

  // Only auto-derived ('node') file:// URLs need on-disk validation; the mode is
  // shared across all assets, so checking one is enough to know the set's mode.
  if (wasm.resolution === 'node') {
    const allExist = (
      await Promise.all([isomorphicFileUrlExists(wasm.url), isomorphicFileUrlExists(worker.url)])
    ).every(Boolean);
    if (!allExist) {
      wasm = Object.freeze({ ...wasm, url: undefined });
      worker = Object.freeze({ ...worker, url: undefined });
    }
  }

  return { wasm, worker };
}

/**
 * Emits debug traces describing how the TFHE assets were resolved (user-provided
 * `locateFile`, auto-derived on-disk path, embedded base64, or bundler-relocated
 * fallback). Pure logging — no control flow.
 */
function _logResolvedTfheAssets(
  wasm: ResolvedTfheAsset,
  worker: ResolvedTfheAsset,
  logger: FhevmRuntimeConfig['logger'],
): void {
  if (wasm.resolution === 'user') {
    logger?.debug(`resolve tfhe wasm filename using 'locateFile' function: ${wasm.filename} -> url: ${wasm.url}`);
    logger?.debug(`resolve tfhe worker filename using 'locateFile' function: ${worker.filename} -> url: ${worker.url}`);
  } else if (wasm.resolution === 'node') {
    if (wasm.url === undefined) {
      // Auto-derived assets were missing on disk (e.g. a bundler such as Turbopack
      // relocated the package) and were cleared by _resolveTfheAssets -> base64.
      logger?.debug(
        `tfhe auto-derived assets not found on disk (bundler relocation?); using embedded base64 ` +
          `(wasm: ${wasm.localRelativePath}, worker: ${worker.localRelativePath})`,
      );
    } else {
      logger?.debug(`resolve tfhe wasm local path: ${wasm.localRelativePath} -> url: ${wasm.url}`);
      logger?.debug(`resolve tfhe worker local path: ${worker.localRelativePath} -> url: ${worker.url}`);
    }
  } else {
    // 'none': browser zero-config (no 'locateFile', not Node) -> embedded base64.
    logger?.debug(`resolve tfhe assets using embedded base64 (browser, no 'locateFile')`);
  }
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

const resolvedTfheModuleConfigByVersion = new Map<TfheVersion, ResolvedTfheModuleConfig>();
const resolvingTfheModuleConfigPromiseByVersion = new Map<TfheVersion, Promise<ResolvedTfheModuleConfig>>();

/**
 * @internal
 * Returns the existing resolved config for a TFHE version, or resolves it from
 * the runtime config.
 */
async function _getOrResolveTfheModuleConfig(
  runtime: FhevmRuntime,
  tfheVersion: TfheVersion,
): Promise<ResolvedTfheModuleConfig> {
  const resolvedTfheModuleConfig = resolvedTfheModuleConfigByVersion.get(tfheVersion);
  if (resolvedTfheModuleConfig !== undefined) {
    return resolvedTfheModuleConfig;
  }

  let resolvingTfheModuleConfigPromise = resolvingTfheModuleConfigPromiseByVersion.get(tfheVersion);
  if (resolvingTfheModuleConfigPromise === undefined) {
    resolvingTfheModuleConfigPromise = _resolveTfheModuleConfig(runtime.config, tfheVersion)
      .then((cfg) => {
        resolvedTfheModuleConfigByVersion.set(tfheVersion, cfg);
        return cfg;
      })
      .catch((error: unknown) => {
        resolvingTfheModuleConfigPromiseByVersion.delete(tfheVersion);
        throw error;
      });

    resolvingTfheModuleConfigPromiseByVersion.set(tfheVersion, resolvingTfheModuleConfigPromise);
  }

  return resolvingTfheModuleConfigPromise;
}

/**
 * Fails fast on impossible / contradictory TFHE configurations, before any
 * wasm or worker work begins.
 *
 * Only genuinely unsatisfiable configs throw here. Recoverable situations — no
 * worker source, missing SAB/thread support, `!canUseBlob` — are NOT errors:
 * they degrade to single-threaded later (single-threaded TFHE needs no worker,
 * and the wasm always loads via embedded base64). Do not add degradable cases
 * to this function.
 */
function _assertSatisfiableTfheConfig(
  wasm: ResolvedTfheAsset,
  worker: ResolvedTfheAsset,
  parameters: FhevmRuntimeConfig,
): void {
  const { wasmAssetLoadMode, numberOfThreads } = parameters;

  // (1) wasm and worker must share the same transport: both resolve to a URL, or
  //     neither does. A selective `locateFile` that returns a URL for one asset
  //     but not the other would split the transport, breaking the same-transport
  //     invariant. ('node'/'none' are already consistent — cleared as a set.)
  if ((wasm.url === undefined) !== (worker.url === undefined)) {
    throw new Error(
      `Inconsistent TFHE asset URLs: 'locateFile' must resolve both the wasm and worker assets, or neither ` +
        `(wasm: ${wasm.url ? 'url' : 'none'}, worker: ${worker.url ? 'url' : 'none'}).`,
    );
  }

  // (2) A URL-requiring worker mode (verified-blob / precheck-direct-url /
  //     trusted-direct-url) was selected, but no asset URL is available (no
  //     'locateFile', and any auto-derived on-disk assets are missing/cleared).
  //     We refuse to silently downgrade an explicit URL/verification choice to base64.
  if (worker.url === undefined && _requiresAssetUrl(wasmAssetLoadMode)) {
    throw new Error(
      `wasmAssetLoadMode '${wasmAssetLoadMode}' requires a resolvable asset URL, but none is available ` +
        `(no 'locateFile', and on-disk assets are missing — e.g. relocated by a bundler). ` +
        `Use 'auto' or 'embedded-base64', or provide 'locateFile'.`,
    );
  }

  // (3) Invalid thread count: must be a non-negative integer when provided.
  //     (Negative / NaN would otherwise be silently coerced to single-threaded,
  //     hiding the caller's mistake.)
  if (numberOfThreads !== undefined && (!Number.isInteger(numberOfThreads) || numberOfThreads < 0)) {
    throw new Error(`numberOfThreads must be a non-negative integer, received: ${String(numberOfThreads)}`);
  }
}

type ResolvedThreadConfig = {
  readonly singleThread: boolean;
  readonly numberOfThreads: number;
  readonly supportsThreads: boolean | undefined;
};

/**
 * True when the worker for this load mode is spawned from code — a blob worker in
 * the browser, an eval worker in Node — and therefore needs blob/eval worker
 * support (`canUseBlob`). The direct-url modes use `new Worker(url)` and do not.
 *
 * `undefined` defaults to `'auto'`, which is a code-worker mode.
 */
function _needsBlobWorker(wasmAssetLoadMode: FhevmRuntimeConfig['wasmAssetLoadMode']): boolean {
  return wasmAssetLoadMode !== 'precheck-direct-url' && wasmAssetLoadMode !== 'trusted-direct-url';
}

function _requiresAssetUrl(wasmAssetLoadMode: FhevmRuntimeConfig['wasmAssetLoadMode']): boolean {
  return wasmAssetLoadMode !== undefined && wasmAssetLoadMode !== 'auto' && wasmAssetLoadMode !== 'embedded-base64';
}

/**
 * Resolves the effective threading config. Degrades to single-threaded when
 * multi-threading is requested but cannot run — no SharedArrayBuffer/COOP-COEP
 * support, or no way to spawn a worker. Never throws: single-threaded always works
 * (it needs no worker, and the wasm still loads via URL or embedded base64).
 */
async function _resolveThreadConfig(args: {
  readonly preferredSingleThread: boolean;
  readonly numberOfThreadsConfig: number | undefined;
  readonly wasmAssetLoadMode: FhevmRuntimeConfig['wasmAssetLoadMode'];
  readonly canUseBlob: boolean;
  readonly logger: FhevmRuntimeConfig['logger'];
}): Promise<ResolvedThreadConfig> {
  const { preferredSingleThread, numberOfThreadsConfig, wasmAssetLoadMode, canUseBlob, logger } = args;

  let singleThread = preferredSingleThread;
  let numberOfThreads = 0;
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
      } else if (!canUseBlob && _needsBlobWorker(wasmAssetLoadMode)) {
        // Threads are supported, but the selected mode spawns its worker from code
        // (embedded-base64 / verified-blob / auto — blob in browser, eval in Node),
        // and blob/eval workers are unavailable here. The direct-url modes use
        // `new Worker(url)` and don't need blob support, so they're exempt.
        // Degrade to single-threaded (single-threaded TFHE needs no worker).
        logger?.warn?.(
          `Cannot spawn a '${wasmAssetLoadMode ?? 'auto'}' worker (blob/eval workers unavailable); running single-threaded.`,
        );
        singleThread = true;
        numberOfThreads = 0;
      }
    } else {
      singleThread = true;
      numberOfThreads = 0;
    }
  }

  return { singleThread, numberOfThreads, supportsThreads };
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
    singleThread: preferredSingleThread,
    numberOfThreads: numberOfThreadsConfig,
  } = parameters;

  const canUseBlob = await isBlobWorkerSupported();

  const assets = tfheAssetsWithVersion(version);

  const { wasm, worker } = await _resolveTfheAssets(assets, locateFile);

  // ── TFHE asset resolution & degradation rules ───────────────────────────────
  //
  // INVARIANT: wasm and worker always use the SAME transport — either both load
  // from a URL, or both from embedded base64. Never mixed: a URL-loaded (and
  // SHA-verified) wasm paired with an unverified base64 worker (or vice versa)
  // would split the integrity story across the two assets.
  //
  // Asset URL — wasm + worker are resolved as a SET (all-or-nothing):
  //
  //   locateFile  runtime / on-disk      resolution  asset url       -> loaded from
  //   ----------  ---------------------  ----------  --------------  --------------------
  //   provided    any                    'user'      locateFile(f)   URL (loud on failure)
  //   none        Node, files present    'node'      file://…        URL
  //   none        Node, files missing *  'node'      undefined       embedded base64
  //   none        browser                'none'      undefined       embedded base64
  //   * e.g. a bundler (Turbopack) relocated the package; URLs cleared by _resolveTfheAssets.
  //
  // Worker load mode (wasmAssetLoadMode) requirements:
  //
  //   mode                 needs URL?  needs blob/eval worker (canUseBlob)?
  //   -------------------  ----------  ------------------------------------
  //   embedded-base64      no          yes
  //   verified-blob        yes         yes
  //   auto                 no          yes   (verified-blob if URL, else embedded)
  //   precheck-direct-url  yes         no    (new Worker(url))
  //   trusted-direct-url   yes         no    (new Worker(url))
  //
  // Failure / degradation (in order):
  //   1. mode needs a URL but none is available          -> throw (unsatisfiable explicit mode)
  //   2. threads requested but unsupported (no SAB)      -> degrade to single-threaded
  //   3. threads requested but worker unspawnable        -> degrade to single-threaded
  //        (mode needs a blob/eval worker && !canUseBlob)
  //   Single-threaded never needs a worker; wasm still loads (URL or embedded base64).

  // Early validation: throw on impossible/contradictory configs. Recoverable
  // cases (no worker source, no thread support) degrade later — they are NOT here.
  _assertSatisfiableTfheConfig(wasm, worker, parameters);

  _logResolvedTfheAssets(wasm, worker, parameters.logger);

  const { singleThread, numberOfThreads, supportsThreads } = await _resolveThreadConfig({
    preferredSingleThread: preferredSingleThread ?? false,
    numberOfThreadsConfig,
    wasmAssetLoadMode,
    canUseBlob,
    logger: parameters.logger,
  });

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

const ownerUidByVersion = new Map<TfheVersion, string>();
const cachedTfheModulePromiseByVersion = new Map<TfheVersion, Promise<TfheLibApi>>();
const moduleInfoByVersion = new Map<TfheVersion, TfheModuleInfo>();

////////////////////////////////////////////////////////////////////////////////
// Version-tagged native wrappers
////////////////////////////////////////////////////////////////////////////////

export const TFHE_VERSION_TAG: unique symbol = Symbol('TFHE.version');

export type VersionTaggedTfheNative = {
  readonly [TFHE_VERSION_TAG]: TfheVersion;
};

export function getTaggedTfheVersion(value: VersionTaggedTfheNative): TfheVersion {
  return value[TFHE_VERSION_TAG];
}

export function assertTaggedTfheVersion(value: VersionTaggedTfheNative, expectedVersion: TfheVersion): void {
  const actualVersion = getTaggedTfheVersion(value);
  if (actualVersion !== expectedVersion) {
    throw new Error(`Unexpected TFHE native wrapper version '${actualVersion}', expected '${expectedVersion}'.`);
  }
}

/**
 * Initializes the TFHE module and returns the loaded lib bindings.
 * Idempotent per TFHE version: subsequent calls return the same cached lib instance.
 */
export async function initTfheModule(runtime: FhevmRuntime, parameters: InitTfheModuleParameters): Promise<TfheLibApi> {
  assertIsFhevmRuntime(runtime, {});

  const ownerUid = ownerUidByVersion.get(parameters.tfheVersion);
  if (ownerUid !== undefined && runtime.uid !== ownerUid) {
    throw new Error(
      `Encrypt WASM module is already owned by runtime '${ownerUid}' and cannot be shared with runtime '${runtime.uid}'`,
    );
  }

  ownerUidByVersion.set(parameters.tfheVersion, runtime.uid);

  // Cache the whole initialization promise before the first await. Several
  // clients may call initTfheModule concurrently during startup; if the promise
  // were assigned after resolving the config, each caller could enter
  // _initTfheModule and try to start the global TFHE worker pool independently.
  // Each TFHE version has its own JS glue module and worker pool, and each
  // version's startWorkers() is intentionally one-shot, so concurrent callers
  // for the same version must await the same promise.
  //
  // Retry is not supported:
  // -----------------------
  // TFHE/WASM initialization and worker startup mutate
  // lower-level module globals that cannot be reset reliably after a partial
  // failure. Keep even a rejected promise cached so later callers observe the
  // original initialization error instead of retrying against half-initialized
  // state and producing secondary errors such as "Already started".

  let cachedTfheModulePromise = cachedTfheModulePromiseByVersion.get(parameters.tfheVersion);
  if (cachedTfheModulePromise === undefined) {
    cachedTfheModulePromise = (async () => {
      // resolve is async
      const cfg = await _getOrResolveTfheModuleConfig(runtime, parameters.tfheVersion);
      return await _initTfheModule(cfg);
    })();

    cachedTfheModulePromiseByVersion.set(parameters.tfheVersion, cachedTfheModulePromise);
  }

  return cachedTfheModulePromise;
}

async function compileWasmModule(cfg: ResolvedTfheModuleConfig): Promise<WebAssembly.Module> {
  let wasmModule;

  if (cfg.wasm.url !== undefined) {
    cfg.logger?.debug(`compile verified wasm at: ${cfg.wasm.url}`);
    wasmModule = await isomorphicCompileVerifiedWasm(cfg.wasm.url, cfg.wasm.sha256);
  } else {
    const { tfheWasmBase64, tfheWasmBase64CompressionFormat } = await loadTfheWasmBase64(cfg.version);
    cfg.logger?.debug(`compile wasm from embedded base64 (compression:${tfheWasmBase64CompressionFormat ?? 'none'})`);
    wasmModule = await isomorphicCompileWasmFromBase64(tfheWasmBase64, tfheWasmBase64CompressionFormat);
  }

  return wasmModule;
}

async function _initTfheModule(cfg: ResolvedTfheModuleConfig): Promise<TfheLibApi> {
  const tfheLib = await loadTfheLib(cfg.version);

  // Compile WASM module (see matrix in types.ts)
  const wasmModule = await compileWasmModule(cfg);

  const input: TfheLibInitAsyncParameters = { module_or_path: wasmModule };

  // 2. Load and instantiate the TFHE WASM binary
  await tfheLib.initAsync(input);

  // 3. Route WASM panics to console.error instead of silently aborting
  tfheLib.init_panic_hook();

  // 4. Spawn Web Workers for parallel FHE operations (skipped when single-threaded)
  if (!cfg.singleThread) {
    cfg.logger?.debug(`initThreadPool(${cfg.numberOfThreads})`);
    await tfheLib.initThreadPool(cfg.numberOfThreads);
  }

  const wasmInfo = tfheLib.getWasmInfo();
  const memory = { byteLength: 0, pages: 0 };
  if (wasmInfo.memory !== undefined) {
    memory.byteLength = wasmInfo.memory.byteLength;
    memory.pages = wasmInfo.memory.pages;
  }

  moduleInfoByVersion.set(
    cfg.version,
    Object.freeze({
      wasmUrl: cfg.wasm.url ? new URL(cfg.wasm.url) : undefined,
      version: wasmInfo.version,
      name: wasmInfo.name,
      workerUrl: cfg.worker.url ? new URL(cfg.worker.url) : undefined,
      numberOfThreads: cfg.singleThread ? 0 : cfg.numberOfThreads,
      threadsAvailable: cfg.supportsThreads,
      memory,
    }),
  );

  return tfheLib;
}

////////////////////////////////////////////////////////////////////////////////
// getTfheModuleInfo
////////////////////////////////////////////////////////////////////////////////

export async function getTfheModuleInfo(parameters: GetTfheModuleInfoParameters): Promise<GetTfheModuleInfoReturnType> {
  const tfheLib = await loadTfheLib(parameters.tfheVersion);
  const stored = moduleInfoByVersion.get(parameters.tfheVersion);
  if (stored === undefined) {
    throw new Error(`getTfheModuleInfo: no module info recorded for version "${parameters.tfheVersion}"`);
  }
  const memory: { byteLength: number; pages: number } = {
    byteLength: stored.memory.byteLength,
    pages: stored.memory.pages,
  };
  const wasmInfo = tfheLib.getWasmInfo();
  if (wasmInfo.memory !== undefined) {
    memory.byteLength = wasmInfo.memory.byteLength;
    memory.pages = wasmInfo.memory.pages;
  }
  return { ...stored, memory };
}
