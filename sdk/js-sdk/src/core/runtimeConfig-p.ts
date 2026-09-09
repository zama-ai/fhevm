import type { Auth } from './types/auth.js';
import type { FhevmModuleVersions } from './types/moduleVersions.js';

////////////////////////////////////////////////////////////////////////////////

export function cloneModuleVersions(moduleVersions: FhevmModuleVersions | undefined): FhevmModuleVersions | undefined {
  if (moduleVersions === undefined || moduleVersions === 'auto') {
    return moduleVersions;
  }

  return Object.freeze({ ...moduleVersions });
}

export function moduleVersionsAreEqual(
  a: FhevmModuleVersions | undefined,
  b: FhevmModuleVersions | undefined,
): boolean {
  if (a === undefined || b === undefined) {
    return a === b;
  }

  if (a === 'auto' || b === 'auto') {
    return a === b;
  }

  return a.tfhe === b.tfhe && a.kms === b.kms && a.checkCompatibility === b.checkCompatibility;
}

export function authsAreEqual(a: Auth | undefined, b: Auth | undefined): boolean {
  if (a === undefined || b === undefined) {
    return a === b;
  }

  switch (a.type) {
    case 'BearerToken':
      return b.type === 'BearerToken' && a.token === b.token;
    case 'ApiKeyHeader':
      return b.type === 'ApiKeyHeader' && a.value === b.value && a.header === b.header;
    case 'ApiKeyCookie':
      return b.type === 'ApiKeyCookie' && a.value === b.value && a.cookie === b.cookie;
  }
}
