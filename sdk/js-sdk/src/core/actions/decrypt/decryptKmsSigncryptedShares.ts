import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShares } from '../../types/kms.js';
import type { TransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import type { TypedValue } from '../../types/primitives.js';
import { decryptKmsSigncryptedShares as decryptKmsSigncryptedShares_ } from '../../kms/decryptKmsSigncryptedShares-p.js';
import { clearValueToTypedValue } from '../../handle/ClearValue.js';
import { asFhevmWithTkmsVersion } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////
// decryptKmsSigncryptedShares (with privateKey)
////////////////////////////////////////////////////////////////////////////////

export type DecryptKmsSigncryptedSharesParameters = {
  readonly kmsSigncryptedShares: KmsSigncryptedShares;
  readonly transportKeyPair: TransportKeyPair;
};

export type DecryptKmsSigncryptedSharesReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptKmsSigncryptedShares(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptKmsSignedcryptedSharesParameters,
): Promise<DecryptKmsSignedcryptedSharesReturnType> {
  const f = asFhevmWithTkmsVersion(fhevm);

  const clearValues = await decryptKmsSignedcryptedShares_(f, parameters);

  const originToken = Symbol('decryptKmsSigncryptedShares');
  return clearValues.map((cv) => clearValueToTypedValue(cv, originToken));
}
