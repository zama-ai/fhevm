import type { FhevmRuntime, FhevmRuntimeConfig } from '../../../types/coreFhevmRuntime.js';
import type {
  GetTfheModuleInfoParameters,
  GetTfheModuleInfoReturnType,
  InitTfheModuleParameters,
  TfheModuleInfo,
} from '../types.js';
import type { TfheLibApi } from '../../../../wasm/tfhe/TfheApi.js';
import type { TfheAssetMetadata, TfheAssets, TfheVersion } from '../../../../wasm/tfhe/loadTfheLib.js';
import type { WasmAssetLoadMode } from '../../../types/wasmAssets.js';
import { isomorphicCompileVerifiedWasm, isomorphicCompileWasmFromBase64 } from '../../../base/wasm.js';
import { isBlobWorkerSupported } from '../../../base/isomorphicWorker.js';
import { isBrowserLike } from '../../../base/environment.js';
import { threads } from 'wasm-feature-detect';
import { assertIsFhevmRuntime } from '../../../runtime/CoreFhevmRuntime-p.js';
import { loadTfheLib, loadTfheWasmBase64, tfheAssetsWithVersion } from '../../../../wasm/tfhe/loadTfheLib.js';
import { isomorphicFileUrlExists } from '../../../base/isomorphicFs.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * TFHE asset loading rules
 *
 * 1. Asset URL resolution
 *    - If `locateFile` is provided:
 *      - `URL` return value: that asset has a URL.
 *      - `null` / `undefined`: that asset has no URL and must use embedded base64
 *        if the selected load path allows it.
 *      - Any other value: invalid `locateFile` result.
 *      - Each asset is resolved independently; advanced callers may intentionally
 *        mix URL and embedded-base64 asset loading.
 *      - User-provided URLs are not checked for on-disk existence, even when
 *        they use the `file:` protocol. They are explicit caller input and fail
 *        later during read/fetch/SHA verification if wrong.
 *    - If `locateFile` is not provided:
 *      - Node: derive default `file://` URLs as an all-or-none set. If any
 *        auto-derived file is missing, clear all TFHE URLs and fall back to
 *        embedded base64. This preserves predictable behavior for unexpected
 *        bundler/package relocation.
 *      - Browser: no asset URL; use embedded base64 paths.
 *
 * 2. WASM module loading
 *    - If the WASM asset has a URL: load, SHA-verify, and compile from that URL.
 *    - If the WASM asset has no URL: compile from embedded base64.
 *    - `wasmAssetLoadMode` does not control WASM module loading.
 *
 * 3. Worker script loading
 *    - If single-threaded: no worker is loaded.
 *    - If threaded: `wasmAssetLoadMode` controls how the worker script is loaded.
 *    - The worker URL only makes a URL-backed worker mode possible; it does not
 *      force the worker to use that URL.
 *
 * 4. Worker mode precedence
 *    - `embedded-base64`: ignore any worker URL and use embedded worker source.
 *    - `verified-blob`: require worker URL, SHA-verify fetched bytes, execute those bytes.
 *    - `precheck-direct-url`: require worker URL, SHA precheck, then execute URL directly.
 *    - `trusted-direct-url`: require worker URL, execute URL directly without SDK verification.
 *    - `auto`: if worker URL exists, try `verified-blob`; on non-SHA failure, fall back
 *      to embedded base64. If no worker URL exists, use embedded base64.
 *
 * 5. Failure
 *    - Explicit URL worker modes with no worker URL: throw.
 *    - SHA mismatch: always throw, never fall back.
 *    - Explicit worker-mode runtime failures are surfaced by `startWorkers.js`.
 *
 * 6. Degradation:
 *    - Auto-derived Node URLs are best effort only. If either the WASM or worker
 *      file is missing on disk, both URLs are cleared and the SDK falls back to the
 *      no-URL paths.
 *    - With no WASM URL, the WASM module compiles from embedded base64.
 *    - With no worker URL, `auto` and `embedded-base64` worker modes use embedded
 *      worker source; explicit URL worker modes throw.
 *    - If threads are unsupported, unavailable, or configured with zero threads,
 *      TFHE runs single-threaded.
 *    - If `auto` mode would need a blob/eval worker but blob/eval workers are
 *      unavailable, TFHE runs single-threaded.
 *    - Single-threaded TFHE never loads a worker; WASM still loads from its resolved
 *      URL or embedded base64.
 *
 * 7. Security model:
 *    - Mixed transports are allowed for user-resolved assets when chosen explicitly.
 *    - External WASM URLs are SHA-verified before compilation.
 *    - `verified-blob` verifies and executes the exact worker bytes.
 *    - `precheck-direct-url` and `trusted-direct-url` intentionally trust the runtime
 *      worker URL fetch semantics described in `startWorkers.js`.
 *
 * Turbopack / relocated package behavior:
 *    - In Node, without `locateFile`, the SDK derives default `file://` URLs from
 *      the package's wasm base URL.
 *    - Some bundlers can relocate/package the JS in a way that makes those derived
 *      `file://` URLs point to paths that do not exist on disk.
 *    - Passing such URLs to the WASM or worker loaders would fail at runtime instead
 *      of using the embedded assets.
 *    - To avoid that, auto-derived Node URLs are checked as an all-or-none set before
 *      loading: if either the WASM or worker file is missing, both URLs are cleared.
 *    - Once cleared, WASM loads from embedded base64 and worker loading follows
 *      `wasmAssetLoadMode` with no worker URL available.
 *    - User-provided `locateFile` URLs, including `file:` URLs, are not probed here;
 *      they are explicit caller input and fail loud later if wrong.
 */

////////////////////////////////////////////////////////////////////////////////

function _requiresAssetUrl(wasmAssetLoadMode: WasmAssetLoadMode | undefined): boolean {
  return wasmAssetLoadMode !== undefined && wasmAssetLoadMode !== 'auto' && wasmAssetLoadMode !== 'embedded-base64';
}

////////////////////////////////////////////////////////////////////////////////

// (Node only) Path relative to src/wasm/ where wasmBaseUrl is anchored
const nodeDefaultLocateFile = async (file: string): Promise<URL> => {
  // Pure JS file (not compiled by tsc) — provides cross-platform base URL
  // for resolving WASM paths. Uses import.meta.url in ESM, __filename in CJS.
  const { wasmBaseUrl } = await import('../../../../wasm/wasmBaseUrl.js');
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

type ResolvedTfheAssets = {
  readonly wasm: ResolvedTfheAsset;
  readonly worker: ResolvedTfheAsset;
};

async function _resolveSingleTfheAsset(
  asset: TfheAssetMetadata,
  locateFile: FhevmRuntimeConfig['locateFile'],
): Promise<ResolvedTfheAsset> {
  let url: URL | undefined;
  let resolution: TfheAssetResolution = 'none';

  if (locateFile !== undefined) {
    const located = locateFile(asset.filename) as unknown;
    if (located === null || located === undefined) {
      url = undefined;
    } else if (located instanceof URL) {
      url = located;
    } else {
      throw new TypeError(
        `Invalid locateFile result for TFHE asset '${asset.filename}': expected URL, null, or undefined.`,
      );
    }
    resolution = 'user';
  } else if (!isBrowserLike()) {
    url = await nodeDefaultLocateFile(asset.localRelativePath);
    resolution = 'node';
  }

  return Object.freeze({ ...asset, url, resolution });
}

/**
 * Resolves TFHE asset URLs for wasm and worker scripts.
 *
 * With user `locateFile` resolution, each asset is independent: returning a URL
 * opts that asset into URL loading, while returning null/undefined opts that asset
 * into embedded base64 when its selected load path supports it.
 *
 * For `'node'` resolution (auto-derived `file://` URLs), every URL is validated
 * on disk: if any one is missing — e.g. a bundler such as Turbopack relocated the
 * package and the derived `file://ROOT/...` path no longer exists — then ALL URLs
 * are cleared so the whole set falls back to embedded base64 consistently.
 *
 * `'none'` has no URL to validate.
 */
async function _resolveTfheAssets(
  assets: TfheAssets,
  locateFile: FhevmRuntimeConfig['locateFile'],
): Promise<{ readonly wasm: ResolvedTfheAsset; readonly worker: ResolvedTfheAsset }> {
  let wasm = await _resolveSingleTfheAsset(assets.wasm, locateFile);
  let worker = await _resolveSingleTfheAsset(assets.worker, locateFile);

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
function _logResolvedTfheAssets(resolvedAssets: ResolvedTfheAssets, logger: FhevmRuntimeConfig['logger']): void {
  const { wasm, worker } = resolvedAssets;
  if (wasm.resolution === 'user') {
    logger?.debug?.(`resolve tfhe wasm filename using 'locateFile' function: ${wasm.filename} -> url: ${wasm.url}`);
    logger?.debug?.(
      `resolve tfhe worker filename using 'locateFile' function: ${worker.filename} -> url: ${worker.url}`,
    );
  } else if (wasm.resolution === 'node') {
    if (wasm.url === undefined) {
      // Auto-derived assets were missing on disk (e.g. a bundler such as Turbopack
      // relocated the package) and were cleared by _resolveTfheAssets -> base64.
      logger?.debug?.(
        `tfhe auto-derived assets not found on disk (bundler relocation?); using embedded base64 ` +
          `(wasm: ${wasm.localRelativePath}, worker: ${worker.localRelativePath})`,
      );
    } else {
      logger?.debug?.(`resolve tfhe wasm local path: ${wasm.localRelativePath} -> url: ${wasm.url}`);
      logger?.debug?.(`resolve tfhe worker local path: ${worker.localRelativePath} -> url: ${worker.url}`);
    }
  } else {
    // 'none': browser zero-config (no 'locateFile', not Node) -> embedded base64.
    logger?.debug?.(`resolve tfhe assets using embedded base64 (browser, no 'locateFile')`);
  }
}

////////////////////////////////////////////////////////////////////////////////
// ResolvedTfheModuleConfig
////////////////////////////////////////////////////////////////////////////////

type ResolvedTfheModuleConfig = {
  readonly version: TfheVersion;
  readonly assets: ResolvedTfheAssets;
  readonly wasmAssetLoadMode: FhevmRuntimeConfig['wasmAssetLoadMode'];
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
 * Only genuinely unsatisfiable configs throw here. Recoverable situations —
 * missing SAB/thread support, or `auto` mode without blob/eval worker support —
 * degrade to single-threaded later. Single-threaded TFHE needs no worker, and
 * wasm still loads from its resolved URL or embedded base64. Do not add
 * degradable cases to this function.
 */
function _assertSatisfiableTfheConfig(resolvedAssets: ResolvedTfheAssets, parameters: FhevmRuntimeConfig): void {
  const { wasmAssetLoadMode, numberOfThreads } = parameters;
  const { wasm, worker } = resolvedAssets;

  if (wasm.resolution !== worker.resolution) {
    // internal error
    throw new Error('Internal error');
  }

  // (1) Auto-derived Node URLs must remain all-or-none. User `locateFile`
  //     resolution may intentionally mix URL and embedded-base64 assets, but a
  //     partial Node file set means the package/bundler layout is inconsistent.
  if (wasm.resolution === 'node') {
    if ((wasm.url === undefined) !== (worker.url === undefined)) {
      throw new Error(
        `Inconsistent auto-derived TFHE asset URLs: Node resolution must resolve both the wasm and worker assets, or neither ` +
          `(wasm: ${wasm.url ? 'url' : 'none'}, worker: ${worker.url ? 'url' : 'none'}).`,
      );
    }
  }

  // (2) A URL-requiring worker mode (verified-blob / precheck-direct-url /
  //     trusted-direct-url) was selected, but no worker URL is available.
  //     We refuse to silently downgrade an explicit URL/verification choice to base64.
  if (worker.url === undefined && _requiresAssetUrl(wasmAssetLoadMode)) {
    throw new Error(
      `wasmAssetLoadMode '${wasmAssetLoadMode}' requires a resolvable worker URL, but none is available. ` +
        `Use 'auto' or 'embedded-base64', or provide a worker URL via 'locateFile'.`,
    );
  }

  // (3) Invalid thread count: must be a non-negative integer when provided.
  //     (Negative / NaN would otherwise be silently coerced to single-threaded,
  //     hiding the caller's mistake.)
  if (numberOfThreads !== undefined && (!Number.isInteger(numberOfThreads) || numberOfThreads < 0)) {
    throw new Error(`numberOfThreads must be a non-negative integer, received: ${String(numberOfThreads)}`);
  }
}

////////////////////////////////////////////////////////////////////////////////

type ResolvedThreadConfig = {
  readonly singleThread: boolean;
  readonly numberOfThreads: number;
  readonly supportsThreads: boolean | undefined;
};

/**
 * Resolves the effective threading config. Degrades to single-threaded when
 * multi-threading is requested but cannot run — no SharedArrayBuffer/COOP-COEP
 * support, or no way to spawn a worker. Never throws: single-threaded always works
 * (it needs no worker, and the wasm still loads via URL or embedded base64).
 */
async function _resolveThreadConfig(args: {
  readonly preferredSingleThread: boolean;
  readonly numberOfThreadsConfig: number | undefined;
  readonly wasmAssetLoadMode: WasmAssetLoadMode;
  readonly canUseBlob: boolean;
  readonly logger: FhevmRuntimeConfig['logger'];
}): Promise<ResolvedThreadConfig> {
  const { preferredSingleThread, numberOfThreadsConfig, wasmAssetLoadMode, canUseBlob, logger } = args;

  let singleThread = preferredSingleThread;
  let numberOfThreads = 0;
  let supportsThreads: boolean | undefined;

  if (!singleThread) {
    // `navigator` is absent in some edge runtimes (and Node <21); guard it so a
    // missing global degrades to single-threaded (0) instead of a ReferenceError.
    // This single-threaded fallback is also the only viable mode on edge anyway:
    // Cloudflare Workers and Vercel Edge support neither Web Workers (`new Worker`)
    // nor `node:worker_threads`, so the worker pool could not be spawned there.
    numberOfThreads = numberOfThreadsConfig ?? (typeof navigator !== 'undefined' ? navigator.hardwareConcurrency : 0);

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
      } else if (!canUseBlob && wasmAssetLoadMode === 'auto') {
        // Threads are supported, but the selected mode spawns its worker from code
        // (embedded-base64 / verified-blob / auto — blob in browser, eval in Node),
        // and blob/eval workers are unavailable here. The direct-url modes use
        // `new Worker(url)` and don't need blob support, so they're exempt.
        // Degrade to single-threaded (single-threaded TFHE needs no worker).
        logger?.warn?.(
          `Cannot spawn a '${wasmAssetLoadMode}' worker (blob/eval workers unavailable); running single-threaded.`,
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
    wasmAssetLoadMode: requestedWasmAssetLoadMode,
    singleThread: preferredSingleThread,
    numberOfThreads: numberOfThreadsConfig,
  } = parameters;

  const wasmAssetLoadMode = requestedWasmAssetLoadMode ?? 'auto';
  const canUseBlob = await isBlobWorkerSupported();

  const assets = tfheAssetsWithVersion(version);

  const resolvedAssets = await _resolveTfheAssets(assets, locateFile);

  // ── TFHE asset resolution & degradation rules ───────────────────────────────
  //
  // POLICY: user-resolved assets may intentionally mix transports (e.g. URL
  // wasm + embedded worker). Auto-derived Node assets remain all-or-none because
  // missing local files are unexpected and usually mean bundler/package relocation.
  //
  // Asset URL resolution:
  //
  //   locateFile  runtime / on-disk      resolution  asset url       policy
  //   ----------  ---------------------  ----------  --------------  --------------------------
  //   provided    any                    'user'      per asset       caller controls each asset
  //   none        Node, files present    'node'      file://…        all TFHE URLs kept
  //   none        Node, files missing *  'node'      undefined       all TFHE URLs cleared
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
  //   1. explicit URL mode but no URL is available       -> throw (unsatisfiable config)
  //   2. threads requested but unsupported (no SAB)      -> degrade to single-threaded
  //   3. auto mode cannot spawn blob/eval workers        -> degrade to single-threaded
  //
  // Explicit worker modes are not downgraded here:
  // startWorkers.js owns their worker-loading behavior and surfaces their failures.
  // Single-threaded never needs a worker; wasm still loads (URL or embedded base64).

  // Early validation: throw on impossible/contradictory configs. Recoverable
  // cases (no worker source, no thread support) degrade later — they are NOT here.
  _assertSatisfiableTfheConfig(resolvedAssets, parameters);

  _logResolvedTfheAssets(resolvedAssets, parameters.logger);

  const { singleThread, numberOfThreads, supportsThreads } = await _resolveThreadConfig({
    preferredSingleThread: preferredSingleThread ?? false,
    numberOfThreadsConfig,
    wasmAssetLoadMode,
    canUseBlob,
    logger: parameters.logger,
  });

  const tfheLib = await loadTfheLib(version);

  tfheLib.setWorkerUrlConfig({
    workerUrl: resolvedAssets.worker.url,
    wasmAssetLoadMode,
    // Single source of truth for browser-vs-Node, resolved on the main thread
    // (robust to bundler `process` shims) — the worker bootstrap no longer detects it.
    isBrowserLike: isBrowserLike(),
    logger:
      parameters.logger?.debug !== undefined && parameters.logger.error !== undefined
        ? { debug: parameters.logger.debug, error: parameters.logger.error }
        : undefined,
  });

  const cfg = {
    version,
    numberOfThreads,
    assets: resolvedAssets,
    wasmAssetLoadMode,
    singleThread,
    logger: parameters.logger,
    supportsThreads,
  };

  //parameters.logger?.debug?.(JSON.stringify(cfg, null, 2));

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

async function _compileWasmModule(cfg: ResolvedTfheModuleConfig): Promise<WebAssembly.Module> {
  let wasmModule;

  if (cfg.assets.wasm.url !== undefined) {
    cfg.logger?.debug?.(`compile verified wasm at: ${cfg.assets.wasm.url}`);
    wasmModule = await isomorphicCompileVerifiedWasm(cfg.assets.wasm.url, cfg.assets.wasm.sha256);
  } else {
    const { tfheWasmBase64, tfheWasmBase64CompressionFormat } = await loadTfheWasmBase64(cfg.version);
    cfg.logger?.debug?.(`compile wasm from embedded base64 (compression:${tfheWasmBase64CompressionFormat ?? 'none'})`);
    wasmModule = await isomorphicCompileWasmFromBase64(tfheWasmBase64, tfheWasmBase64CompressionFormat);
  }

  return wasmModule;
}

async function _initTfheModule(cfg: ResolvedTfheModuleConfig): Promise<TfheLibApi> {
  const tfheLib = await loadTfheLib(cfg.version);

  // 1. Compile WASM module
  const wasmModule = await _compileWasmModule(cfg);

  const input: TfheLibInitAsyncParameters = { module_or_path: wasmModule };

  // 2. Load and instantiate the TFHE WASM binary
  await tfheLib.initAsync(input);

  // 3. Route WASM panics to console.error instead of silently aborting
  tfheLib.init_panic_hook();

  // 4. Spawn Web Workers for parallel FHE operations (skipped when single-threaded)
  if (!cfg.singleThread) {
    cfg.logger?.debug?.(`initThreadPool(${cfg.numberOfThreads})`);
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
      wasmUrl: cfg.assets.wasm.url ? new URL(cfg.assets.wasm.url) : undefined,
      version: wasmInfo.version,
      name: wasmInfo.name,
      workerUrl: cfg.assets.worker.url ? new URL(cfg.assets.worker.url) : undefined,
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
