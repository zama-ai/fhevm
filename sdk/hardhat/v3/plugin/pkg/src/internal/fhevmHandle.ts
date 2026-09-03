// The little the plugin needs to know about a bytes32 FHE handle before handing it to the SDK.

import { HardhatPluginError } from 'hardhat/plugins';
import { type Hex, toHex } from 'viem';

import { PLUGIN_ID } from './constants.js';

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
