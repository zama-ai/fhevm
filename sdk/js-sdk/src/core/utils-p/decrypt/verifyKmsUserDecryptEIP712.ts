import {
  ThresholdSignerError,
  UnknownSignerError,
} from '../../errors/SignersError.js';
import { createKmsEIP712Domain } from '../../kms/createKmsEIP712Domain.js';
import { kmsUserDecryptEIP712Types } from '../../kms/kmsUserDecryptEIP712Types.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type {
  KmsUserDecryptEIP712,
  KmsUserDecryptEIP712Message,
} from '../../types/kms.js';
import type { Bytes65Hex, ChecksummedAddress } from '../../types/primitives.js';
import { recoverSigners } from '../runtime/recoverSigners.js';

export type VerifyKmsUserDecryptEIP712Parameters = {
  readonly signer: ChecksummedAddress;
  readonly message: KmsUserDecryptEIP712Message;
  readonly signature: Bytes65Hex;
};

export async function verifyKmsUserDecryptEIP712(
  context: { readonly chain: FhevmChain; readonly runtime: FhevmRuntime },
  parameters: VerifyKmsUserDecryptEIP712Parameters,
): Promise<void> {
  // A 'UserDecryptRequestVerification' KmsEIP712Domain (for signed permit)
  // uses `chain.id` (NOT fhevm.gateway.id!!)
  const domain = createKmsEIP712Domain({
    chainId: context.chain.id,
    verifyingContractAddressDecryption:
      context.chain.fhevm.gateway.contracts.decryption.address,
  });

  const recoveredAddresses = await recoverSigners(context, {
    domain,
    types: kmsUserDecryptEIP712Types,
    primaryType:
      'UserDecryptRequestVerification' satisfies KmsUserDecryptEIP712['primaryType'],
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
