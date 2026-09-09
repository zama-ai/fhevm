// The little the plugin needs to know about a bytes32 FHE handle before handing it to the SDK.

import { HardhatPluginError } from 'hardhat/plugins';
import { type Hex, isHex, size, toHex } from 'viem';

import { FhevmType, type FhevmTypeName } from '../types.js';
import { PLUGIN_ID } from './constants.js';
import { getFhevmTypeName } from './fheType.js';

/** The handle a Solidity FHE variable holds before anything is ever assigned to it. */
const UNINITIALIZED_HANDLE: Hex = `0x${'0'.repeat(64)}`;

export function handleKey(handle: Hex | Uint8Array): Hex {
  return typeof handle === 'string' ? handle : toHex(handle);
}

// A contract returns bytes32(0) for an FHE value that was never written — much the most common way a
// decryption fails. Left to the SDK the zero word decodes structurally ("chainId 0, expected …"),
// which reads like a network misconfiguration; caught here the error stays pointed at the cause.
export function assertHandleIsInitialized(handle: Hex): void {
  if (handle.toLowerCase() === UNINITIALIZED_HANDLE) {
    throw new HardhatPluginError(PLUGIN_ID, `Handle is not initialized`);
  }
}

/**
 * A decoded bytes32 handle. The layout is the protocol's (`FHEVMExecutor`): bytes 0-20 hash, byte 21
 * the input index or 0xff for a computed value, bytes 22-29 chain id, byte 30 FHE type, byte 31 version.
 */
export type FhevmHandleInfo = {
  readonly handleBytes32Hex: Hex;
  readonly chainId: number;
  readonly fhevmType: FhevmType;
  readonly typeName: FhevmTypeName;
  /** True when the handle came out of an FHE operation rather than a user input. */
  readonly computed: boolean;
  readonly version: number;
};

export function parseFhevmHandle(handle: Hex): FhevmHandleInfo {
  if (!isHex(handle) || size(handle) !== 32) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `Invalid FHE handle '${handle}': expected a 32-byte 0x-prefixed hex string.`,
    );
  }
  const byte = (i: number): number => Number.parseInt(handle.slice(2 + i * 2, 4 + i * 2), 16);
  const typeByte = byte(30);
  if (!(typeByte in FhevmType)) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `Invalid FHE handle '${handle}': byte 30 is not a known FHE type (got 0x${typeByte.toString(16)}).`,
    );
  }
  const fhevmType = typeByte;
  return {
    handleBytes32Hex: handle,
    chainId: Number(BigInt(`0x${handle.slice(46, 62)}`)),
    fhevmType,
    typeName: getFhevmTypeName(fhevmType),
    computed: byte(21) === 255,
    version: byte(31),
  };
}
