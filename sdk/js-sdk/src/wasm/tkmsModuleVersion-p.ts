import type { TkmsVersion } from './tkms/loadKmsLib.js';
import { DEFAULT_TKMS_VERSION } from './tkms/loadKmsLib.js';

////////////////////////////////////////////////////////////////////////////////

type TkmsModuleVersionConfig = {
  readonly moduleVersions?:
    | 'auto'
    | {
        readonly kms?: TkmsVersion | 'auto' | undefined;
      }
    | undefined;
};

export function resolveTkmsModuleVersion(config: TkmsModuleVersionConfig): TkmsVersion {
  const moduleVersions = config.moduleVersions;
  if (moduleVersions === undefined || moduleVersions === 'auto') {
    return resolveAutoTkmsModuleVersion(config);
  }

  return moduleVersions.kms === undefined || moduleVersions.kms === 'auto'
    ? resolveAutoTkmsModuleVersion(config)
    : moduleVersions.kms;
}

////////////////////////////////////////////////////////////////////////////////

function resolveAutoTkmsModuleVersion(_config: TkmsModuleVersionConfig): TkmsVersion {
  return DEFAULT_TKMS_VERSION;
}
