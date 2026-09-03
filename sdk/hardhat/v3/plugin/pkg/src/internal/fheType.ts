// Helpers over the public `FhevmType` enum. `fhevmTypeToSdkType` is the bridge to @fhevm/sdk, whose
// encrypt actions take the type NAME; the SDK has no euint4, so that one is refused here, by name.

import { FhevmType, type FhevmTypeEuint, type FhevmTypeName } from '../types.js';

export const FhevmTypeNameMap: Readonly<Record<FhevmType, FhevmTypeName>> = Object.freeze({
  [FhevmType.ebool]: 'ebool',
  [FhevmType.euint4]: 'euint4',
  [FhevmType.euint8]: 'euint8',
  [FhevmType.euint16]: 'euint16',
  [FhevmType.euint32]: 'euint32',
  [FhevmType.euint64]: 'euint64',
  [FhevmType.euint128]: 'euint128',
  [FhevmType.eaddress]: 'eaddress',
  [FhevmType.euint256]: 'euint256',
});

export function getFhevmTypeName(fhevmType: FhevmType): FhevmTypeName {
  return FhevmTypeNameMap[fhevmType];
}

export function fhevmTypeToSdkType(fhevmType: FhevmType): string {
  if (fhevmType === FhevmType.euint4) throw new Error(`euint4 is not supported by @fhevm/sdk.`);
  return getFhevmTypeName(fhevmType);
}

export function isFhevmEuint(value: FhevmType | undefined): value is FhevmTypeEuint {
  switch (value) {
    case FhevmType.euint4:
    case FhevmType.euint8:
    case FhevmType.euint16:
    case FhevmType.euint32:
    case FhevmType.euint64:
    case FhevmType.euint128:
    case FhevmType.euint256:
      return true;
    case FhevmType.ebool:
    case FhevmType.eaddress:
    case undefined:
      return false;
  }
}
