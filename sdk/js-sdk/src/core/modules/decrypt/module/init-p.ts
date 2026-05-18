import type { FhevmRuntime, FhevmRuntimeConfig } from '../../../types/coreFhevmRuntime.js';
import type { GetTkmsModuleInfoReturnType, TkmsModuleInfo } from '../types.js';
import type { KmsLibApi, TkmsVersion } from '../../../../wasm/tkms/KmsLibApi.js';
import type { KmsAssetMetadata } from '../../../../wasm/tkms/loadKmsLib.js';
import { isBrowserLike } from '../../../base/isomorphicWorker.js';
import { isomorphicCompileVerifiedWasm, isomorphicCompileWasmFromBase64 } from '../../../base/wasm.js';
import { assertIsFhevmRuntime } from '../../../runtime/CoreFhevmRuntime-p.js';
import { resolveTkmsModuleVersion } from '../../../../wasm/tkmsModuleVersion-p.js';
import { wasmBaseUrl } from '../../../../wasm/wasmBaseUrl.js';
import { kmsAssetsWithVersion, loadKmsLib, loadKmsWasmBase64 } from '../../../../wasm/tkms/loadKmsLib.js';

////////////////////////////////////////////////////////////////////////////////

// Pure JS file (not compiled by tsc) — provides cross-platform base URL
// for resolving WASM paths. Uses import.meta.url in ESM, __filename in CJS.

// (Node only) Path relative to src/wasm/ where wasmBaseUrl is anchored
const nodeDefaultLocateFile = (file: string): URL => {
  return new URL(`./tkms/${file}`, wasmBaseUrl);
};

type ResolvedKmsAsset = KmsAssetMetadata & {
  readonly url: URL | undefined;
};

function _resolveKmsAsset(asset: KmsAssetMetadata, locateFile: FhevmRuntimeConfig['locateFile']): ResolvedKmsAsset {
  let url: URL | undefined;

  if (locateFile !== undefined) {
    url = locateFile(asset.filename);
  } else if (!isBrowserLike()) {
    url = nodeDefaultLocateFile(asset.localRelativePath);
  }

  return Object.freeze({ ...asset, url });
}

type ResolvedTkmsModuleConfig = {
  readonly version: TkmsVersion;
  readonly wasm: ResolvedKmsAsset;
  readonly logger: FhevmRuntimeConfig['logger'];
};

let resolvedTkmsModuleConfig: ResolvedTkmsModuleConfig | undefined = undefined;

/**
 * @internal
 * Returns the existing resolved config, or resolves it from the runtime config.
 */
function _getOrResolveTkmsModuleConfig(runtime: FhevmRuntime): ResolvedTkmsModuleConfig {
  if (resolvedTkmsModuleConfig !== undefined) return resolvedTkmsModuleConfig;

  const version = resolveTkmsModuleVersion(runtime.config);

  resolvedTkmsModuleConfig = _resolveTkmsModuleConfig(runtime.config, version);
  return resolvedTkmsModuleConfig;
}

function _resolveTkmsModuleConfig(parameters: FhevmRuntimeConfig, version: TkmsVersion): ResolvedTkmsModuleConfig {
  if (cachedTkmsModulePromise !== undefined) {
    throw new Error('Cannot configure module after initialization has started');
  }

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

type InitTkmsModuleParameters = {
  readonly module_or_path: TkmsInitInput | Promise<TkmsInitInput>;
};

let cachedTkmsModulePromise: Promise<KmsLibApi> | undefined;
let ownerUid: string | undefined = undefined;

export async function initTkmsModule(runtime: FhevmRuntime): Promise<KmsLibApi> {
  assertIsFhevmRuntime(runtime, {});

  if (ownerUid !== undefined && runtime.uid !== ownerUid) {
    throw new Error(
      `Decrypt WASM module is already owned by runtime '${ownerUid}' and cannot be shared with runtime '${runtime.uid}'`,
    );
  }

  ownerUid = runtime.uid;

  // Cache the TKMS initialization promise once and keep it cached even if it
  // rejects.
  //
  // Retry is not supported:
  // -----------------------
  // The underlying KMS/WASM bindings
  // keep module-level state that cannot be reset reliably after a partial
  // initialization failure; later callers should observe the original error
  // instead of retrying against half-initialized state.
  if (cachedTkmsModulePromise === undefined) {
    // resolve is sync
    const cfg = _getOrResolveTkmsModuleConfig(runtime);
    cachedTkmsModulePromise = _initTkmsModule(cfg);
  }

  return cachedTkmsModulePromise;
}

let moduleInfo: TkmsModuleInfo | undefined = undefined;

async function _initTkmsModule(cfg: ResolvedTkmsModuleConfig): Promise<KmsLibApi> {
  const libPromise = loadKmsLib(cfg.version);

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

  const input: InitTkmsModuleParameters = { module_or_path: wasmModule };

  const lib = await libPromise;
  await lib.default(input);

  // Note: init_panic_hook is not exposed by kms_lib
  const wasmInfo = lib.getWasmInfo();

  moduleInfo = Object.freeze({
    wasmUrl: cfg.wasm.url ? new URL(cfg.wasm.url) : undefined,
    version: wasmInfo.version,
    name: wasmInfo.name,
  });

  return lib;
}

////////////////////////////////////////////////////////////////////////////////
// getTkmsModuleInfo
////////////////////////////////////////////////////////////////////////////////

export function getTkmsModuleInfo(): GetTkmsModuleInfoReturnType {
  return moduleInfo;
}
