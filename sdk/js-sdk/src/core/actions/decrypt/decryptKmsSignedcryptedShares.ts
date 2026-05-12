import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShares } from '../../types/kms.js';
import type { TransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import type { TypedValue } from '../../types/primitives.js';
import { decryptKmsSignedcryptedShares as decryptKmsSignedcryptedShares_ } from '../../kms/decryptKmsSignedcryptedShares-p.js';
import { clearValueToTypedValue } from '../../handle/ClearValue.js';

////////////////////////////////////////////////////////////////////////////////
// decryptKmsSignedcryptedShares (with privateKey)
////////////////////////////////////////////////////////////////////////////////

export type DecryptKmsSignedcryptedSharesParameters = {
  readonly kmsSigncryptedShares: KmsSigncryptedShares;
  readonly transportKeyPair: TransportKeyPair;
};

export type DecryptKmsSignedcryptedSharesReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptKmsSignedcryptedShares(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptKmsSignedcryptedSharesParameters,
): Promise<DecryptKmsSignedcryptedSharesReturnType> {
  const clearValues = await decryptKmsSignedcryptedShares_(fhevm, parameters);

  const originToken = Symbol('decryptKmsSignedcryptedShares');
  return clearValues.map((cv) => clearValueToTypedValue(cv, originToken));
}
