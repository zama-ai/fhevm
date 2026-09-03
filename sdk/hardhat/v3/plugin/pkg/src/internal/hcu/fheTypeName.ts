// The protocol's own type spelling (`Uint32`), which the vendored price table is keyed by — distinct
// from the plugin's `euint32`. `eaddress` is priced as `Uint160`, its on-chain representation.

import { HardhatPluginError } from 'hardhat/plugins';

import { FhevmType } from '../../types.js';
import { PLUGIN_ID } from '../constants.js';
import type { FheTypeName } from '../vendored/priceTypes.js';

const FHE_TYPE_NAME_BY_FHEVM_TYPE: Readonly<Record<FhevmType, FheTypeName | undefined>> = {
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

export function getFheTypeName(fhevmType: FhevmType): FheTypeName {
  const name = FHE_TYPE_NAME_BY_FHEVM_TYPE[fhevmType];
  if (name === undefined) {
    throw new HardhatPluginError(PLUGIN_ID, `No HCU price data for FHE type '${String(fhevmType)}'.`);
  }
  return name;
}

/** For a raw type byte off an event argument, not yet checked against `FhevmType`. */
export function getFheTypeNameFromByte(typeByte: number): FheTypeName {
  if (!(typeByte in FhevmType)) throw new HardhatPluginError(PLUGIN_ID, `Unknown FHE type byte '${String(typeByte)}'.`);
  return getFheTypeName(typeByte);
}
