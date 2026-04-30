import type { FhevmRuntime, FhevmRuntimeConfig } from '../../../types/coreFhevmRuntime.js';
import type { GetTkmsModuleInfoReturnType, TkmsModuleInfo } from '../types.js';
import init_kms_lib from '../../../../wasm/tkms/kms_lib.v0.13.10.js';
import { getWasmInfo } from '../../../../wasm/tkms/kms_lib.v0.13.10.js';
import { isBrowserLike } from '../../../base/isomorphicWorker.js';
import { isomorphicCompileWasm, isomorphicCompileWasmFromBase64 } from '../../../base/wasm.js';
import { assertIsFhevmRuntime } from '../../../runtime/CoreFhevmRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

// IMPORTANT: The import path MUST be a string literal, not a variable.
// Bundlers (Webpack, Vite, Rollup) statically analyze import() calls to
// create separate chunks for code-splitting. A variable path like
// `import(someVar)` makes the target unresolvable — the bundler either
// includes every possible file or fails entirely.
// With a literal path, the bundler creates a lazy-loaded chunk for this
// ~0.85MB base64 file, only downloaded when this function is called.
function dynamicImportWasmBase64(): Promise<{
  readonly tkmsWasmBase64: string;
}> {
  // Bundler Alert: !! KEEP THE PATH AS-IS !!
  //return import("../../../../wasm/tkms/kms_lib_bg.wasm.base64.js");
  return import('../../../../wasm/tkms/kms_lib_bg.v0.13.10.wasm.base64.js');
}

const KMS_BG_WASM_FILENAME = 'kms_lib_bg.v0.13.10.wasm';

////////////////////////////////////////////////////////////////////////////////

// Pure JS file (not compiled by tsc) — provides cross-platform base URL
// for resolving WASM paths. Uses import.meta.url in ESM, __filename in CJS.
import { wasmBaseUrl } from '../../../../wasm/wasmBaseUrl.js';

// (Node only) Path relative to src/wasm/ where wasmBaseUrl is anchored
const nodeDefaultLocateFile = (file: string): URL => {
  return new URL(`./tkms/${file}`, wasmBaseUrl);
};

type ResolvedTkmsModuleConfig = {
  readonly wasmUrl: URL | undefined;
  readonly logger: FhevmRuntimeConfig['logger'];
};

let resolvedTkmsModuleConfig: ResolvedTkmsModuleConfig | undefined = undefined;

/**
 * @internal
 * Returns the existing resolved config, or resolves it from the runtime config.
 */
function _getOrResolveTkmsModuleConfig(runtime: FhevmRuntime): ResolvedTkmsModuleConfig {
  if (resolvedTkmsModuleConfig !== undefined) return resolvedTkmsModuleConfig;

  resolvedTkmsModuleConfig = _resolveTkmsModuleConfig(runtime.config);
  return resolvedTkmsModuleConfig;
}

function _resolveTkmsModuleConfig(parameters: FhevmRuntimeConfig): ResolvedTkmsModuleConfig {
  if (cachedTkmsModulePromise !== undefined) {
    throw new Error('Cannot configure module after initialization has started');
  }

  const { locateFile } = parameters;

  let wasmUrl: URL | undefined;

  if (locateFile !== undefined) {
    wasmUrl = locateFile(KMS_BG_WASM_FILENAME);
  } else {
    /*
      if run in Node only, use defaultLocateFile!
    */
    if (!isBrowserLike()) {
      wasmUrl = nodeDefaultLocateFile(KMS_BG_WASM_FILENAME);
    }
  }

  // if wasmUrl is undefined, use base64 code instead
  const cfg = {
    wasmUrl,
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

let cachedTkmsModulePromise: Promise<void> | undefined;
let ownerUid: string | undefined = undefined;

export async function initTkmsModule(runtime: FhevmRuntime): Promise<void> {
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

async function _initTkmsModule(cfg: ResolvedTkmsModuleConfig): Promise<void> {
  // Compile WASM module (see matrix in types.ts)
  let wasmModule;
  if (cfg.wasmUrl !== undefined) {
    cfg.logger?.debug(`compile wasm at: ${cfg.wasmUrl}`);
    wasmModule = await isomorphicCompileWasm(cfg.wasmUrl);
  } else {
    cfg.logger?.debug(`compile wasm from embedded base64`);
    const { tkmsWasmBase64 } = await dynamicImportWasmBase64();
    wasmModule = await isomorphicCompileWasmFromBase64(tkmsWasmBase64);
  }

  const input: InitTkmsModuleParameters = { module_or_path: wasmModule };

  await init_kms_lib(input);

  // Note: init_panic_hook is not exposed by kms_lib

  const wasmInfo = getWasmInfo();

  moduleInfo = Object.freeze({
    wasmUrl: cfg.wasmUrl ? new URL(cfg.wasmUrl) : undefined,
    version: wasmInfo.version,
    name: wasmInfo.name,
  });
}

////////////////////////////////////////////////////////////////////////////////
// getTkmsModuleInfo
////////////////////////////////////////////////////////////////////////////////

export function getTkmsModuleInfo(): GetTkmsModuleInfoReturnType {
  return moduleInfo;
}
