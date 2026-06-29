import type { TfheVersion } from './tfhe/loadTfheLib.js';
import { DEFAULT_TFHE_VERSION } from './tfhe/loadTfheLib.js';

////////////////////////////////////////////////////////////////////////////////

type TfheModuleVersionConfig = {
  readonly moduleVersions?:
    | 'auto'
    | {
        readonly tfhe?: TfheVersion | 'auto' | undefined;
      }
    | undefined;
};

export function resolveTfheModuleVersion(config: TfheModuleVersionConfig): TfheVersion {
  const moduleVersions = config.moduleVersions;
  if (moduleVersions === undefined || moduleVersions === 'auto') {
    return resolveAutoTfheModuleVersion(config);
  }

  return moduleVersions.tfhe === undefined || moduleVersions.tfhe === 'auto'
    ? resolveAutoTfheModuleVersion(config)
    : moduleVersions.tfhe;
}

////////////////////////////////////////////////////////////////////////////////

function resolveAutoTfheModuleVersion(_config: TfheModuleVersionConfig): TfheVersion {
  return DEFAULT_TFHE_VERSION;
}
