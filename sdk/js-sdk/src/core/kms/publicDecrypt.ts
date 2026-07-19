import type { RelayerPublicDecryptOptions } from '../types/relayer.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import type { PublicDecryptionProof } from '../types/publicDecryptionProof-p.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { KmsExtraData } from '../types/kms-p.js';
import { assertHandlesBelongToSameChainId } from '../handle/FhevmHandle.js';
import { assertKmsDecryptionBitLimit } from '../kms/utils.js';
import { readCurrentKmsSignersContext, reconcileKmsSignersContext } from '../host-contracts/readKmsSignersContext-p.js';
import { kmsSignersContextToExtraData } from '../host-contracts/KmsSignersContext-p.js';
import { createPublicDecryptionProof } from '../kms/PublicDecryptionProof-p.js';
import { checkAllowedForDecryption } from '../host-contracts/checkAllowedForDecryption.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly originToken: symbol;
  readonly handles: readonly Handle[];
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

type ReturnType = PublicDecryptionProof;

////////////////////////////////////////////////////////////////////////////////

export async function publicDecrypt(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { handles, options, originToken } = parameters;

  // Request side: build dynamic extraData from current context ID
  // Contrary to the userDecrypt flow, the publicDecrypt doesn't require for an
  // EIP-712 signature from the user, so the SDK can safely fetch the current
  // context ID and build the extraData transparently to the user.
  const requestedKmsSignersContext = await readCurrentKmsSignersContext(context, {
    kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
  });

  const requestedExtraData: KmsExtraData = kmsSignersContextToExtraData(requestedKmsSignersContext);

  const orderedHandles = handles;

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerOptions: RelayerPublicDecryptOptions = {
    auth: context.runtime.config.auth,
    ...options,
  };

  // 1. Check: At least one handle is required
  if (orderedHandles.length === 0) {
    throw Error(`handles must not be empty, at least one handle is required`);
  }

  // 2. Check: 2048 bits limit
  assertKmsDecryptionBitLimit(orderedHandles);

  // 3. Check: All handles belong to the host chainId
  assertHandlesBelongToSameChainId(orderedHandles, BigInt(context.chain.id) as Uint64BigInt);

  // 4. Check: ACL permissions
  await checkAllowedForDecryption(context, {
    aclAddress: context.chain.fhevm.contracts.acl.address as ChecksummedAddress,
    handles: orderedHandles,
  });

  // 5. Call relayer
  const {
    orderedAbiEncodedClearValues,
    kmsPublicDecryptEip712Signatures,
    extraData: relayerExtraDataBytesHex,
  } = await context.runtime.relayer.fetchPublicDecrypt(context, {
    payload: {
      orderedHandles,
      extraData: requestedExtraData.bytesHex,
    },
    options: relayerOptions,
  });

  // 6. Reconcile the relayer's KMS context against the one we requested.
  //    Pins on contextId (mirrors gateway Decryption.sol DecryptionContextMismatch);
  //    rejects a substituted context, epoch deliberately ignored.
  const reconciledKmsSignersContext: KmsSignersContext = await reconcileKmsSignersContext(context, {
    kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
    relayerKmsExtraDataBytesHex: relayerExtraDataBytesHex,
    requestedKmsSignersContext: requestedKmsSignersContext,
  });

  // 7. Verify and Compute PublicDecryptionProof
  const publicDecryptionProof: PublicDecryptionProof = await createPublicDecryptionProof(context, {
    originToken,
    orderedHandles: orderedHandles,
    orderedAbiEncodedClearValues,
    kmsPublicDecryptEip712Signatures,
    kmsSignersContext: reconciledKmsSignersContext,
  });

  return publicDecryptionProof;
}
