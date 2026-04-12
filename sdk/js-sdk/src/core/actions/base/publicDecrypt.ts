import type { RelayerPublicDecryptOptions } from '../../types/relayer.js';
import {
  assertHandlesBelongToSameChainId,
  toHandle,
} from '../../handle/FhevmHandle.js';
import { assertKmsDecryptionBitLimit } from '../../kms/utils.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type {
  BytesHex,
  ChecksummedAddress,
  Uint64BigInt,
} from '../../types/primitives.js';
import type { PublicDecryptionProof } from '../../types/publicDecryptionProof.js';
import { checkAllowedForDecryption } from './checkAllowedForDecryption.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import {
  readKmsSignersContext,
  reconcileKmsSignersContext,
} from '../../host-contracts/readKmsSignersContext-p.js';
import { kmsSignersContextToExtraData } from '../../host-contracts/KmsSignersContext-p.js';
import { createPublicDecryptionProof } from '../../kms/PublicDecryptionProof-p.js';
import type { KmsSignersContext } from '../../types/kmsSignersContext.js';

export type PublicDecryptParameters = {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type PublicDecryptReturnType = PublicDecryptionProof;

export async function publicDecrypt(
  fhevm: Fhevm<FhevmChain>,
  parameters: PublicDecryptParameters,
): Promise<PublicDecryptReturnType> {
  const { encryptedValues, options } = parameters;

  // Request side: build dynamic extraData from current context ID
  // Contrary to the userDecrypt flow, the publicDecrypt doesn't require for an
  // EIP-712 signature from the user, so the SDK can safely fetch the current
  // context ID and build the extraData transparently to the user.
  const requestedKmsSignersContext = await readKmsSignersContext(fhevm, {
    address: fhevm.chain.fhevm.contracts.kmsVerifier
      .address as ChecksummedAddress,
  });

  const requestedExtraData: BytesHex = kmsSignersContextToExtraData(
    requestedKmsSignersContext,
  );

  const orderedHandles = parameters.encryptedValues.map((ev) => toHandle(ev));

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerOptions: RelayerPublicDecryptOptions = {
    auth: fhevm.runtime.config.auth,
    ...options,
  };

  // 1. Check: At least one handle is required
  if (orderedHandles.length === 0) {
    throw Error(`handles must not be empty, at least one handle is required`);
  }

  // 2. Check: 2048 bits limit
  assertKmsDecryptionBitLimit(orderedHandles);

  // 3. Check: All handles belong to the host chainId
  assertHandlesBelongToSameChainId(
    orderedHandles,
    BigInt(fhevm.chain.id) as Uint64BigInt,
  );

  // 4. Check: ACL permissions
  await checkAllowedForDecryption(fhevm, {
    handles: encryptedValues,
    options: { checkArguments: true },
  });

  // 5. Call relayer
  const {
    orderedAbiEncodedClearValues,
    kmsPublicDecryptEIP712Signatures,
    extraData: relayerExtraData,
  } = await fhevm.runtime.relayer.fetchPublicDecrypt(
    { relayerUrl: fhevm.chain.fhevm.relayerUrl, chainId: fhevm.chain.id },
    {
      payload: {
        orderedHandles,
        extraData: requestedExtraData,
      },
      options: relayerOptions,
    },
  );

  // 6. Reconcile KMS signer context using 'loose' mode
  const reconciledKmsSignersContext: KmsSignersContext =
    await reconcileKmsSignersContext(fhevm, {
      address: fhevm.chain.fhevm.contracts.kmsVerifier
        .address as ChecksummedAddress,
      relayerExtraData,
      requestedKmsSignersContext: requestedKmsSignersContext,
      mode: 'loose',
    });

  // 7. Verify and Compute PublicDecryptionProof
  const publicDecryptionProof: PublicDecryptionProof =
    await createPublicDecryptionProof(fhevm, {
      orderedEncryptedValues: orderedHandles,
      orderedAbiEncodedClearValues,
      kmsPublicDecryptEIP712Signatures,
      kmsSignersContext: reconciledKmsSignersContext,
    });

  return publicDecryptionProof;
}
