import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { TransportKeyPair } from '../decrypt/index.js';
import type { Eip712Like } from '../../types/kms.js';
import { createUnsignedDecryptionPermitEip712V2 as createUnsignedDecryptionPermitEip712V2_ } from '../../kms/SignedDecryptionPermitV2-p.js';
import { SDK_PROTOCOL_API_MAJOR_VERSION, SDK_PROTOCOL_API_MINOR_VERSION } from '../../runtime/sdkProtocolApiVersion.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

export type CreateUnsignedUnifiedDecryptionPermitEip712Parameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationSeconds: number;
  readonly signerAddress: string;
  readonly transportKeyPair: TransportKeyPair;
};
export type CreateUnsignedUnifiedDecryptionPermitEip712ReturnType = Eip712Like;

export async function createUnsignedUnifiedDecryptionPermitEip712(
  fhevm: Fhevm<FhevmChain>,
  parameters: CreateUnsignedUnifiedDecryptionPermitEip712Parameters,
): Promise<CreateUnsignedUnifiedDecryptionPermitEip712ReturnType> {
  if (SDK_PROTOCOL_API_MAJOR_VERSION === 0 && SDK_PROTOCOL_API_MINOR_VERSION <= 13) {
    throw new Error(
      `Unified (V2) decryption permits are not supported: this SDK uses protocol API v0.13.x, which only supports V1 decryption permits. Creating a unified permit requires an SDK using protocol API v0.14.0 or later.`,
    );
  }
  const fhevmContext = await initPublicAction(fhevm);
  return createUnsignedDecryptionPermitEip712V2_(fhevm, { ...parameters, fhevmContext });
}
