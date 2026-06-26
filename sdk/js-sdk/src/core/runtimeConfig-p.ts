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
