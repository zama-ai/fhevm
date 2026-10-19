// Public decryption over the SDK client. `publicDecrypt` keeps v2's handle-keyed result and carries
// the KMS proof, so a contract can verify the decryption on-chain
// (`contract.verify(handles, abiEncodedClearValues, decryptionProof)`); the typed variants coerce one
// value and fail by handle when the type does not match.

import { HardhatPluginError } from 'hardhat/plugins';
import { type Address, type Hex, isAddress } from 'viem';

import type { FhevmClient, PublicDecryptResults } from '../types.js';
import { PLUGIN_ID } from './constants.js';
import { assertHandleIsInitialized, handleKey } from './fhevmHandle.js';

type TypedValueLike = { readonly type: string; readonly value: unknown };
type ClearValue = bigint | boolean | Address;

export async function publicDecrypt(
  client: FhevmClient,
  handles: Array<Hex | Uint8Array>,
): Promise<PublicDecryptResults> {
  const keys = handles.map(handleKey);
  keys.forEach(assertHandleIsInitialized);

  const res = await client.decryptPublicValuesWithSignatures({ encryptedValues: handles });

  // The SDK answers positionally; the plugin's result is keyed by handle.
  const clearValues: Record<Hex, ClearValue> = {};
  keys.forEach((key, i) => {
    clearValues[key] = clearValue(res.clearValues[i]);
  });
  return {
    clearValues,
    abiEncodedClearValues: res.checkSignaturesArgs.abiEncodedCleartexts,
    decryptionProof: res.checkSignaturesArgs.decryptionProof,
  };
}

export async function publicDecryptOne(client: FhevmClient, handle: Hex): Promise<TypedValueLike> {
  assertHandleIsInitialized(handle);
  return client.decryptPublicValue({ encryptedValue: handle });
}

export function clearValue(value: TypedValueLike | undefined): ClearValue {
  if (value === undefined) throw new HardhatPluginError(PLUGIN_ID, `Missing decrypted value in the response.`);
  const v = value.value;
  if (typeof v === 'bigint' || typeof v === 'boolean') return v;
  if (typeof v === 'number') return BigInt(v);
  if (typeof v === 'string' && isAddress(v)) return v;
  throw new HardhatPluginError(PLUGIN_ID, `Unexpected decrypted value type '${typeof v}'.`);
}

export function asBoolean(value: TypedValueLike, handle: Hex): boolean {
  const v = clearValue(value);
  if (typeof v === 'boolean') return v;
  throw unexpected('ebool', handle, 'a boolean', typeof v);
}

export function asBigInt(value: TypedValueLike, handle: Hex): bigint {
  const v = clearValue(value);
  if (typeof v === 'bigint') return v;
  throw unexpected('euint', handle, 'a bigint', typeof v);
}

export function asAddress(value: TypedValueLike, handle: Hex): Address {
  const v = clearValue(value);
  if (typeof v === 'string') return v;
  throw unexpected('eaddress', handle, 'an address', typeof v);
}

function unexpected(kind: string, handle: Hex, expected: string, got: string): HardhatPluginError {
  return new HardhatPluginError(
    PLUGIN_ID,
    `Unexpected type for decrypted value of ${kind} handle '${handle}': expected ${expected}, but got '${got}' instead.`,
  );
}
