// Argument parsing shared by the fhevm tasks: each failure names the argument and what it expected.

import { HardhatPluginError } from 'hardhat/plugins';
import { type Address, type Hex, getAddress, isAddress, isHex, size } from 'viem';

import { PLUGIN_ID } from '../internal/constants.js';
import { tryParseFhevmType } from '../internal/fheType.js';
import type { FhevmType } from '../types.js';

export const FHEVM_TASK_SCOPE = 'fhevm';

export function parseTypeArg(value: string): FhevmType {
  const fhevmType = tryParseFhevmType(value);
  if (fhevmType === undefined) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `Unknown FHEVM primitive type name '${value}' (expected ebool, euint8, …, eaddress).`,
    );
  }
  return fhevmType;
}

export function parseHandleArg(value: string): Hex {
  if (!isHex(value) || size(value) !== 32) {
    throw new HardhatPluginError(PLUGIN_ID, `Invalid handle '${value}': expected a 32-byte 0x-prefixed hex string.`);
  }
  return value;
}

export function parseAddressArg(name: string, value: string): Address {
  if (!isAddress(value)) throw new HardhatPluginError(PLUGIN_ID, `Invalid ${name} '${value}': expected an address.`);
  return getAddress(value);
}

export function parseIndexArg(name: string, value: number, count: number): number {
  if (!Number.isInteger(value) || value < 0 || value >= count) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `Invalid ${name} '${String(value)}': expected an account index between 0 and ${String(count - 1)}.`,
    );
  }
  return value;
}
