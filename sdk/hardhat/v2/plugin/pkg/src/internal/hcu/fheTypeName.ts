import { HardhatFhevmError } from '../../error';
import { FhevmType } from '../fheType';
import type { FheTypeName } from '../vendored/priceTypes';

/**
 * The protocol's own type names, as used by the vendored HCU price table.
 *
 * Distinct from the plugin's `FhevmTypeName` (`"euint32"`): this is the `FheType` spelling
 * (`"Uint32"`) the vendored pricing data is keyed by.
 */
// Re-exported from the price types: the vendored table decides which type names exist, so there is
// exactly one definition rather than two that can drift.
export type { FheTypeName } from '../vendored/priceTypes';

/** `eaddress` is priced as `Uint160`, which is how an address is represented on-chain. */
const FHEVM_TYPE_TO_FHE_TYPE_NAME: Readonly<Record<FhevmType, FheTypeName | undefined>> = {
  [FhevmType.ebool]: 'Bool',
  [FhevmType.euint4]: undefined,
  [FhevmType.euint8]: 'Uint8',
  [FhevmType.euint16]: 'Uint16',
  [FhevmType.euint32]: 'Uint32',
  [FhevmType.euint64]: 'Uint64',
  [FhevmType.euint128]: 'Uint128',
  [FhevmType.eaddress]: 'Uint160',
  [FhevmType.euint256]: 'Uint256',
};

function isFhevmType(value: number): value is FhevmType {
  return value in FhevmType;
}

export function getFheTypeName(fhevmType: FhevmType): FheTypeName {
  const name = FHEVM_TYPE_TO_FHE_TYPE_NAME[fhevmType];
  if (name === undefined) {
    throw new HardhatFhevmError(`No HCU price data for FHE type '${String(fhevmType)}'.`);
  }
  return name;
}

/** For a raw type byte off an event arg, which has not been checked against `FhevmType` yet. */
export function getFheTypeNameFromByte(typeByte: number): FheTypeName {
  if (!isFhevmType(typeByte)) {
    throw new HardhatFhevmError(`Unknown FHE type byte '${typeByte}'.`);
  }
  return getFheTypeName(typeByte);
}
