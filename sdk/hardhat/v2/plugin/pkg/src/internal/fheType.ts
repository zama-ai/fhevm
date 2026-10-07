/**
 * The FHE type taxonomy.
 *
 * `@fhevm/sdk` has the same taxonomy as a string-literal union (`FheType` in
 * `core/types/fheType.d.ts`) but does not export it, so the plugin owns this until upstream does.
 * See `plans/HOST_CONTRACTS_CLEARTEXT_UPSTREAM_FIXES.md`, problem 4.
 *
 * It stays a numeric enum because it is a *value* in the plugin's public API — tests call
 * `fhevm.userDecryptEuint(FhevmType.euint32, …)`. Values are the on-chain `FheType` ids.
 * `fhevmTypeToSdkType` is the bridge to the SDK, whose `encryptValue`/`TypedValue` take the name.
 */

export enum FhevmType {
  // eslint-disable-next-line @typescript-eslint/naming-convention
  ebool = 0,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint4 = 1,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint8 = 2,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint16 = 3,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint32 = 4,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint64 = 5,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint128 = 6,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  eaddress = 7,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint256 = 8,
}

export type FhevmTypeName =
  'ebool' | 'euint4' | 'euint8' | 'euint16' | 'euint32' | 'euint64' | 'euint128' | 'eaddress' | 'euint256';

export type FhevmTypeEuint =
  | FhevmType.euint4
  | FhevmType.euint8
  | FhevmType.euint16
  | FhevmType.euint32
  | FhevmType.euint64
  | FhevmType.euint128
  | FhevmType.euint256;

export const FhevmTypeNameMap: Readonly<Record<FhevmType, FhevmTypeName>> = {
  [FhevmType.ebool]: 'ebool',
  [FhevmType.euint4]: 'euint4',
  [FhevmType.euint8]: 'euint8',
  [FhevmType.euint16]: 'euint16',
  [FhevmType.euint32]: 'euint32',
  [FhevmType.euint64]: 'euint64',
  [FhevmType.euint128]: 'euint128',
  [FhevmType.euint256]: 'euint256',
  [FhevmType.eaddress]: 'eaddress',
};
Object.freeze(FhevmTypeNameMap);

const FhevmTypeMap: Readonly<Record<FhevmTypeName, FhevmType>> = {
  ebool: FhevmType.ebool,
  euint4: FhevmType.euint4,
  euint8: FhevmType.euint8,
  euint16: FhevmType.euint16,
  euint32: FhevmType.euint32,
  euint64: FhevmType.euint64,
  euint128: FhevmType.euint128,
  eaddress: FhevmType.eaddress,
  euint256: FhevmType.euint256,
};
Object.freeze(FhevmTypeMap);

/** Parses a type name (`"euint32"`) into its enum value. Returns `undefined` if unknown. */
export function tryParseFhevmType(value: string): FhevmType | undefined {
  return Object.prototype.hasOwnProperty.call(FhevmTypeMap, value) ? FhevmTypeMap[value as FhevmTypeName] : undefined;
}

export function getFhevmTypeName(fhevmType: FhevmType): FhevmTypeName {
  const name = FhevmTypeNameMap[fhevmType];
  if ((name as unknown) === undefined) {
    throw new Error(`Unknown FhevmType value '${String(fhevmType)}'.`);
  }
  return name;
}

/**
 * The name `@fhevm/sdk` expects in `encryptValue({ value: { type, … } })`.
 *
 * `euint4` has no SDK counterpart — the SDK's `FheType` union stops at `ebool`, `euint8`…`euint256`
 * and `eaddress` — so it is rejected here rather than silently sent as an unknown string.
 */
export function fhevmTypeToSdkType(fhevmType: FhevmType): string {
  if (fhevmType === FhevmType.euint4) {
    throw new Error(`euint4 is not supported by @fhevm/sdk.`);
  }
  return getFhevmTypeName(fhevmType);
}

export function isFhevmEbool(value: FhevmType | undefined): value is FhevmType.ebool {
  return value === FhevmType.ebool;
}

export function isFhevmEaddress(value: FhevmType | undefined): value is FhevmType.eaddress {
  return value === FhevmType.eaddress;
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
