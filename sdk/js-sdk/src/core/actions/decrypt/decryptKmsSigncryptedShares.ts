import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShares } from '../../types/kms.js';
import type { TransportKeypair } from '../../kms/TransportKeypair-p.js';
import type { TypedValue } from '../../types/primitives.js';
import { decryptKmsSigncryptedShares as decryptKmsSigncryptedShares_ } from '../../kms/decryptKmsSigncryptedShares-p.js';
import { clearValueToTypedValue } from '../../handle/ClearValue.js';

////////////////////////////////////////////////////////////////////////////////
// decryptKmsSigncryptedShares (with privateKey)
////////////////////////////////////////////////////////////////////////////////

export type DecryptKmsSigncryptedSharesParameters = {
  readonly kmsSigncryptedShares: KmsSigncryptedShares;
  readonly transportKeypair: TransportKeypair;
};

export type DecryptKmsSigncryptedSharesReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptKmsSigncryptedShares(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptKmsSigncryptedSharesParameters,
): Promise<DecryptKmsSigncryptedSharesReturnType> {
  const clearValues = await decryptKmsSigncryptedShares_(fhevm, parameters);

  const originToken = Symbol('decryptKmsSigncryptedShares');
  return clearValues.map((cv) => clearValueToTypedValue(cv, originToken));
}
