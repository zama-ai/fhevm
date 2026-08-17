import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { KmsUserDecryptEip712V1, KmsUserDecryptEip712V1Message } from '../../types/kms.js';
import type { Bytes65Hex, ChecksummedAddress } from '../../types/primitives.js';
import { ThresholdSignerError, UnknownSignerError } from '../../errors/SignersError.js';
import { createKmsEip712Domain } from '../../kms/createKmsEip712Domain.js';
import { kmsUserDecryptEip712V1Types } from '../../kms/kmsUserDecryptEip712V1Types.js';
import { recoverSigners } from '../runtime/recoverSigners.js';

export type VerifyKmsUserDecryptEip712V1Parameters = {
  readonly signer: ChecksummedAddress;
  readonly message: KmsUserDecryptEip712V1Message;
  readonly signature: Bytes65Hex;
};

export async function verifyKmsUserDecryptEip712V1(
  context: { readonly chain: FhevmChain; readonly runtime: FhevmRuntime },
  parameters: VerifyKmsUserDecryptEip712V1Parameters,
): Promise<void> {
  // A 'UserDecryptRequestVerification' KmsEip712Domain (for signed permit)
  // uses `chain.id` (NOT fhevm.gateway.id!!)
  const domain = createKmsEip712Domain({
    chainId: context.chain.id,
    verifyingContractAddressDecryption: context.chain.fhevm.gateway.contracts.decryption.address,
  });

  const recoveredAddresses = await recoverSigners(context, {
    domain,
    types: kmsUserDecryptEip712V1Types,
    primaryType: 'UserDecryptRequestVerification' satisfies KmsUserDecryptEip712V1['primaryType'],
    signatures: [parameters.signature],
    message: parameters.message,
  });

  if (recoveredAddresses.length !== 1) {
    throw new ThresholdSignerError({
      type: 'kms',
    });
  }

  if (recoveredAddresses[0] !== parameters.signer) {
    throw new UnknownSignerError({
      unknownAddress: recoveredAddresses[0] as unknown as ChecksummedAddress,
      type: 'kms',
    });
  }
}
