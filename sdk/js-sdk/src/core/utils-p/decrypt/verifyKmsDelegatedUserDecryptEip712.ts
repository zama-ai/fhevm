import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { KmsDelegatedUserDecryptEip712, KmsDelegatedUserDecryptEip712Message } from '../../types/kms.js';
import type { Bytes65Hex, ChecksummedAddress } from '../../types/primitives.js';
import { kmsDelegatedUserDecryptEip712Types } from '../../kms/kmsDelegatedUserDecryptEip712Types.js';
import { ThresholdSignerError, UnknownSignerError } from '../../errors/SignersError.js';
import { createKmsEip712Domain } from '../../kms/createKmsEip712Domain.js';
import { recoverSigners } from '../runtime/recoverSigners.js';

export type VerifyKmsDelegatedUserDecryptEip712Parameters = {
  readonly signer: ChecksummedAddress;
  readonly message: KmsDelegatedUserDecryptEip712Message;
  readonly signature: Bytes65Hex;
};

export async function verifyKmsDelegatedUserDecryptEip712(
  context: { readonly chain: FhevmChain; readonly runtime: FhevmRuntime },
  parameters: VerifyKmsDelegatedUserDecryptEip712Parameters,
): Promise<void> {
  // A 'DelegatedUserDecryptRequestVerification' KmsEIP712Domain (for signed permit)
  // uses `chain.id` (NOT fhevm.gateway.id!!)
  const domain = createKmsEip712Domain({
    chainId: context.chain.id,
    verifyingContractAddressDecryption: context.chain.fhevm.gateway.contracts.decryption.address,
  });

  const recoveredAddresses = await recoverSigners(context, {
    domain,
    types: kmsDelegatedUserDecryptEip712Types,
    primaryType: 'DelegatedUserDecryptRequestVerification' satisfies KmsDelegatedUserDecryptEip712['primaryType'],
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
