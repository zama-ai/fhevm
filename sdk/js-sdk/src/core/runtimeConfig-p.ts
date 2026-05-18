import type { WasmModuleVersions } from './types/coreFhevmRuntime.js';

////////////////////////////////////////////////////////////////////////////////

export function cloneModuleVersions(moduleVersions: WasmModuleVersions | undefined): WasmModuleVersions | undefined {
  if (moduleVersions === undefined || moduleVersions === 'auto') {
    return moduleVersions;
  }

  return Object.freeze({ ...moduleVersions });
}
