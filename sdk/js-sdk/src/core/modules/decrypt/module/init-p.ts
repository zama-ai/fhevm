import type { FhevmRuntime, FhevmRuntimeConfig } from '../../../types/coreFhevmRuntime.js';
import type {
  GetTkmsModuleInfoParameters,
  GetTkmsModuleInfoReturnType,
  InitTkmsModuleParameters,
  TkmsModuleInfo,
} from '../types.js';
import type { KmsLibApi, TkmsVersion } from '../../../../wasm/tkms/KmsLibApi.js';
import type { KmsAssetMetadata } from '../../../../wasm/tkms/loadKmsLib.js';
import { isBrowserLike } from '../../../base/isomorphicWorker.js';
import { isomorphicCompileVerifiedWasm, isomorphicCompileWasmFromBase64 } from '../../../base/wasm.js';
import { assertIsFhevmRuntime } from '../../../runtime/CoreFhevmRuntime-p.js';
import { wasmBaseUrl } from '../../../../wasm/wasmBaseUrl.js';
import { kmsAssetsWithVersion, loadKmsLib, loadKmsWasmBase64 } from '../../../../wasm/tkms/loadKmsLib.js';

////////////////////////////////////////////////////////////////////////////////

const ownerUidByVersion = new Map<TkmsVersion, string>();
const cachedTkmsModulePromiseByVersion = new Map<TkmsVersion, Promise<KmsLibApi>>();
const moduleInfoByVersion = new Map<TkmsVersion, TkmsModuleInfo>();

////////////////////////////////////////////////////////////////////////////////

// Pure JS file (not compiled by tsc) — provides cross-platform base URL
// for resolving WASM paths. Uses import.meta.url in ESM, __filename in CJS.

// (Node only) Path relative to src/wasm/ where wasmBaseUrl is anchored
const nodeDefaultLocateFile = (file: string): URL => {
  return new URL(`./tkms/${file}`, wasmBaseUrl);
};

type KmsAssetResolution = 'user' | 'node' | 'none';
type ResolvedKmsAsset = KmsAssetMetadata & {
  readonly url: URL | undefined;
  readonly resolution: KmsAssetResolution;
};

function _resolveKmsAsset(asset: KmsAssetMetadata, locateFile: FhevmRuntimeConfig['locateFile']): ResolvedKmsAsset {
  let url: URL | undefined;
  let resolution: KmsAssetResolution = 'none';

  if (locateFile !== undefined) {
    url = locateFile(asset.filename);
    resolution = 'user';
  } else if (!isBrowserLike()) {
    url = nodeDefaultLocateFile(asset.localRelativePath);
    resolution = 'node';
  }

  return Object.freeze({ ...asset, url, resolution });
}

type ResolvedTkmsModuleConfig = {
  readonly version: TkmsVersion;
  readonly wasm: ResolvedKmsAsset;
  readonly logger: FhevmRuntimeConfig['logger'];
};

const resolvedTkmsModuleConfigByVersion = new Map<TkmsVersion, ResolvedTkmsModuleConfig>();

/**
 * @internal
 * Returns the existing resolved config, or resolves it from the runtime config.
 */
function _getOrResolveTkmsModuleConfig(runtime: FhevmRuntime, tkmsVersion: TkmsVersion): ResolvedTkmsModuleConfig {
  let resolvedTkmsModuleConfig = resolvedTkmsModuleConfigByVersion.get(tkmsVersion);
  if (resolvedTkmsModuleConfig !== undefined) {
    return resolvedTkmsModuleConfig;
  }

  resolvedTkmsModuleConfig = _resolveTkmsModuleConfig(runtime.config, tkmsVersion);

  resolvedTkmsModuleConfigByVersion.set(tkmsVersion, resolvedTkmsModuleConfig);

  return resolvedTkmsModuleConfig;
}

function _resolveTkmsModuleConfig(parameters: FhevmRuntimeConfig, version: TkmsVersion): ResolvedTkmsModuleConfig {
  const { locateFile } = parameters;

  const assets = kmsAssetsWithVersion(version);
  const wasm = _resolveKmsAsset(assets.wasm, locateFile);

  if (locateFile !== undefined) {
    parameters.logger?.debug(`resolve kms wasm filename: ${wasm.filename} -> url: ${wasm.url}`);
  } else {
    /*
      if run in Node only, use defaultLocateFile!
    */
    if (!isBrowserLike()) {
      parameters.logger?.debug(`resolve kms wasm local path: ${wasm.localRelativePath} -> url: ${wasm.url}`);
    }
  }

  // if wasmUrl is undefined, use base64 code instead
  const cfg = {
    version,
    wasm,
    logger: parameters.logger,
  };

  parameters.logger?.debug(JSON.stringify(cfg, null, 2));

  return cfg;
}

////////////////////////////////////////////////////////////////////////////////
// initTkmsModule
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

  // Cache the TKMS initialization promise once and keep it cached even if it
  // rejects.
  //
  // Retry is not supported:
  // -----------------------
  // The underlying KMS/WASM bindings
  // keep module-level state that cannot be reset reliably after a partial
  // initialization failure; later callers should observe the original error
  // instead of retrying against half-initialized state.

  let cachedTkmsModulePromise = cachedTkmsModulePromiseByVersion.get(parameters.tkmsVersion);
  if (cachedTkmsModulePromise === undefined) {
    // resolve is sync
    const cfg = _getOrResolveTkmsModuleConfig(runtime, parameters.tkmsVersion);
    cachedTkmsModulePromise = _initTkmsModule(cfg);
    cachedTkmsModulePromiseByVersion.set(parameters.tkmsVersion, cachedTkmsModulePromise);
  }

  return cachedTkmsModulePromise;
}

async function _initTkmsModule(cfg: ResolvedTkmsModuleConfig): Promise<KmsLibApi> {
  const kmsLib = await loadKmsLib(cfg.version);

  // Compile WASM module (see matrix in types.ts)
  let wasmModule;
  if (cfg.wasm.url !== undefined) {
    cfg.logger?.debug(`compile verified wasm at: ${cfg.wasm.url}`);
    wasmModule = await isomorphicCompileVerifiedWasm(cfg.wasm.url, cfg.wasm.sha256);
  } else {
    const { tkmsWasmBase64, tkmsWasmBase64IsGzipped } = await loadKmsWasmBase64(cfg.version);
    cfg.logger?.debug(`compile wasm from embedded base64 (compression:${tkmsWasmBase64IsGzipped})`);
    wasmModule = await isomorphicCompileWasmFromBase64(tkmsWasmBase64, tkmsWasmBase64IsGzipped ? 'gzip' : undefined);
  }

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
      wasmUrl: cfg.wasm.url ? new URL(cfg.wasm.url) : undefined,
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
