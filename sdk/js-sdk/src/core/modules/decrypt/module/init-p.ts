import type { FhevmRuntime, FhevmRuntimeConfig } from '../../../types/coreFhevmRuntime.js';
import type {
  GetTkmsModuleInfoParameters,
  GetTkmsModuleInfoReturnType,
  InitTkmsModuleParameters,
  TkmsModuleInfo,
} from '../types.js';
import type { KmsLibApi, TkmsVersion } from '../../../../wasm/tkms/KmsLibApi.js';
import type { KmsAssetMetadata, KmsAssets } from '../../../../wasm/tkms/loadKmsLib.js';
import { isBrowserLike } from '../../../base/environment.js';
import { isomorphicCompileVerifiedWasm, isomorphicCompileWasmFromBase64 } from '../../../base/wasm.js';
import { assertIsFhevmRuntime } from '../../../runtime/CoreFhevmRuntime-p.js';
import { kmsAssetsWithVersion, loadKmsLib, loadKmsWasmBase64 } from '../../../../wasm/tkms/loadKmsLib.js';
import { isomorphicFileUrlExists } from '../../../base/isomorphicFs.js';

////////////////////////////////////////////////////////////////////////////////

// (Node only) Path relative to src/wasm/ where wasmBaseUrl is anchored
const nodeDefaultLocateFile = async (file: string): Promise<URL> => {
  // Pure JS file (not compiled by tsc) — provides cross-platform base URL
  // for resolving WASM paths. Uses import.meta.url in ESM, __filename in CJS.
  const { wasmBaseUrl } = await import('../../../../wasm/wasmBaseUrl.js');
  return new URL(`./tkms/${file}`, wasmBaseUrl);
};

////////////////////////////////////////////////////////////////////////////////

type KmsAssetResolution = 'user' | 'node' | 'none';

type ResolvedKmsAsset = KmsAssetMetadata & {
  readonly url: URL | undefined;
  readonly resolution: KmsAssetResolution;
};

type ResolvedKmsAssets = {
  readonly wasm: ResolvedKmsAsset;
};

////////////////////////////////////////////////////////////////////////////////

async function _resolveSingleKmsAsset(
  asset: KmsAssetMetadata,
  locateFile: FhevmRuntimeConfig['locateFile'],
): Promise<ResolvedKmsAsset> {
  let url: URL | undefined;
  let resolution: KmsAssetResolution = 'none';

  if (locateFile !== undefined) {
    const located = locateFile(asset.filename) as unknown;
    if (located === null || located === undefined) {
      url = undefined;
    } else if (located instanceof URL) {
      url = located;
    } else {
      throw new TypeError(
        `Invalid locateFile result for TKMS asset '${asset.filename}': expected URL, null, or undefined.`,
      );
    }
    resolution = 'user';
  } else if (!isBrowserLike()) {
    url = await nodeDefaultLocateFile(asset.localRelativePath);
    resolution = 'node';
  }

  return Object.freeze({ ...asset, url, resolution });
}

////////////////////////////////////////////////////////////////////////////////

async function _resolveKmsAssets(
  assets: KmsAssets,
  locateFile: FhevmRuntimeConfig['locateFile'],
): Promise<{ readonly wasm: ResolvedKmsAsset }> {
  let wasm = await _resolveSingleKmsAsset(assets.wasm, locateFile);

  // Only auto-derived ('node') file:// URLs need on-disk validation; the mode is
  // shared across all assets, so checking one is enough to know the set's mode.
  if (wasm.resolution === 'node') {
    const allExist = await isomorphicFileUrlExists(wasm.url);
    if (!allExist) {
      wasm = Object.freeze({ ...wasm, url: undefined });
    }
  }

  return { wasm };
}

////////////////////////////////////////////////////////////////////////////////

function _logResolvedKmsAssets(resolvedAssets: ResolvedKmsAssets, logger: FhevmRuntimeConfig['logger']): void {
  const { wasm } = resolvedAssets;
  if (wasm.resolution === 'user') {
    logger?.debug?.(`resolve tkms wasm filename using 'locateFile' function: ${wasm.filename} -> url: ${wasm.url}`);
  } else if (wasm.resolution === 'node') {
    if (wasm.url === undefined) {
      // Auto-derived assets were missing on disk (e.g. a bundler such as Turbopack
      // relocated the package) and were cleared by _resolveTkmsAssets -> base64.
      logger?.debug?.(
        `tkms auto-derived assets not found on disk (bundler relocation?); using embedded base64 ` +
          `(wasm: ${wasm.localRelativePath})`,
      );
    } else {
      logger?.debug?.(`resolve tkms wasm local path: ${wasm.localRelativePath} -> url: ${wasm.url}`);
    }
  } else {
    // 'none': browser zero-config (no 'locateFile', not Node) -> embedded base64.
    logger?.debug?.(`resolve tkms assets using embedded base64 (browser, no 'locateFile')`);
  }
}

////////////////////////////////////////////////////////////////////////////////

type ResolvedTkmsModuleConfig = {
  readonly version: TkmsVersion;
  readonly assets: ResolvedKmsAssets;
  readonly logger: FhevmRuntimeConfig['logger'];
};

const resolvedTkmsModuleConfigByVersion = new Map<TkmsVersion, ResolvedTkmsModuleConfig>();
const resolvingTkmsModuleConfigPromiseByVersion = new Map<TkmsVersion, Promise<ResolvedTkmsModuleConfig>>();

/**
 * @internal
 * Returns the existing resolved config for a TKMS version, or resolves it from
 * the runtime config.
 */
async function _getOrResolveTkmsModuleConfig(
  runtime: FhevmRuntime,
  tkmsVersion: TkmsVersion,
): Promise<ResolvedTkmsModuleConfig> {
  const resolvedTkmsModuleConfig = resolvedTkmsModuleConfigByVersion.get(tkmsVersion);
  if (resolvedTkmsModuleConfig !== undefined) {
    return resolvedTkmsModuleConfig;
  }

  let resolvingTkmsModuleConfigPromise = resolvingTkmsModuleConfigPromiseByVersion.get(tkmsVersion);
  if (resolvingTkmsModuleConfigPromise === undefined) {
    resolvingTkmsModuleConfigPromise = _resolveTkmsModuleConfig(runtime.config, tkmsVersion)
      .then((cfg) => {
        resolvedTkmsModuleConfigByVersion.set(tkmsVersion, cfg);
        return cfg;
      })
      .catch((error: unknown) => {
        resolvingTkmsModuleConfigPromiseByVersion.delete(tkmsVersion);
        throw error;
      });

    resolvingTkmsModuleConfigPromiseByVersion.set(tkmsVersion, resolvingTkmsModuleConfigPromise);
  }

  return resolvingTkmsModuleConfigPromise;
}

async function _resolveTkmsModuleConfig(
  parameters: FhevmRuntimeConfig,
  version: TkmsVersion,
): Promise<ResolvedTkmsModuleConfig> {
  const { locateFile } = parameters;

  const assets = kmsAssetsWithVersion(version);

  const resolvedAssets = await _resolveKmsAssets(assets, locateFile);

  _logResolvedKmsAssets(resolvedAssets, parameters.logger);

  // if wasmUrl is undefined, use base64 code instead
  const cfg = {
    version,
    assets: resolvedAssets,
    logger: parameters.logger,
  };

  //parameters.logger?.debug?.(JSON.stringify(cfg, null, 2));

  return cfg;
}

////////////////////////////////////////////////////////////////////////////////
// initTkmsModule
////////////////////////////////////////////////////////////////////////////////

const ownerUidByVersion = new Map<TkmsVersion, string>();
const cachedTkmsModulePromiseByVersion = new Map<TkmsVersion, Promise<KmsLibApi>>();
const moduleInfoByVersion = new Map<TkmsVersion, TkmsModuleInfo>();

////////////////////////////////////////////////////////////////////////////////

type TkmsInitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

type TkmsLibInitAsyncParameters = {
  readonly module_or_path: TkmsInitInput | Promise<TkmsInitInput>;
};

export async function initTkmsModule(runtime: FhevmRuntime, parameters: InitTkmsModuleParameters): Promise<KmsLibApi> {
  assertIsFhevmRuntime(runtime, {});

  const ownerUid = ownerUidByVersion.get(parameters.tkmsVersion);
  if (ownerUid !== undefined && runtime.uid !== ownerUid) {
    throw new Error(
      `Decrypt WASM module is already owned by runtime '${ownerUid}' and cannot be shared with runtime '${runtime.uid}'`,
    );
  }

  ownerUidByVersion.set(parameters.tkmsVersion, runtime.uid);

  // Cache the whole initialization promise before the first await. Several
  // clients may call initTkmsModule concurrently during startup.
  //
  // Retry is not supported

  let cachedTkmsModulePromise = cachedTkmsModulePromiseByVersion.get(parameters.tkmsVersion);
  if (cachedTkmsModulePromise === undefined) {
    cachedTkmsModulePromise = (async () => {
      // resolve is async
      const cfg = await _getOrResolveTkmsModuleConfig(runtime, parameters.tkmsVersion);
      return await _initTkmsModule(cfg);
    })();

    cachedTkmsModulePromiseByVersion.set(parameters.tkmsVersion, cachedTkmsModulePromise);
  }

  return cachedTkmsModulePromise;
}

async function _compileWasmModule(cfg: ResolvedTkmsModuleConfig): Promise<WebAssembly.Module> {
  let wasmModule;

  if (cfg.assets.wasm.url !== undefined) {
    cfg.logger?.debug?.(`compile tkms verified wasm at: ${cfg.assets.wasm.url}`);
    wasmModule = await isomorphicCompileVerifiedWasm(cfg.assets.wasm.url, cfg.assets.wasm.sha256);
  } else {
    const { tkmsWasmBase64, tkmsWasmBase64CompressionFormat } = await loadKmsWasmBase64(cfg.version);
    cfg.logger?.debug?.(
      `compile tkms wasm from embedded base64 (compression:${tkmsWasmBase64CompressionFormat ?? 'none'})`,
    );
    wasmModule = await isomorphicCompileWasmFromBase64(tkmsWasmBase64, tkmsWasmBase64CompressionFormat);
  }

  return wasmModule;
}

async function _initTkmsModule(cfg: ResolvedTkmsModuleConfig): Promise<KmsLibApi> {
  const kmsLib = await loadKmsLib(cfg.version);

  // Compile WASM module (see matrix in types.ts)
  const wasmModule = await _compileWasmModule(cfg);

  const input: TkmsLibInitAsyncParameters = { module_or_path: wasmModule };

  await kmsLib.initAsync(input);

  // Note: init_panic_hook is not exposed by kms_lib
  const wasmInfo = kmsLib.getWasmInfo();
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
      memory,
    }),
  );

  return kmsLib;
}

////////////////////////////////////////////////////////////////////////////////
// getTkmsModuleInfo
////////////////////////////////////////////////////////////////////////////////

export async function getTkmsModuleInfo(parameters: GetTkmsModuleInfoParameters): Promise<GetTkmsModuleInfoReturnType> {
  const tkmsLib = await loadKmsLib(parameters.tkmsVersion);
  const stored = moduleInfoByVersion.get(parameters.tkmsVersion);
  if (stored === undefined) {
    throw new Error(`getTkmsModuleInfo: no module info recorded for version "${parameters.tkmsVersion}"`);
  }
  const memory: { byteLength: number; pages: number } = {
    byteLength: stored.memory.byteLength,
    pages: stored.memory.pages,
  };
  const wasmInfo = tkmsLib.getWasmInfo();
  if (wasmInfo.memory !== undefined) {
    memory.byteLength = wasmInfo.memory.byteLength;
    memory.pages = wasmInfo.memory.pages;
  }
  return { ...stored, memory };
}
