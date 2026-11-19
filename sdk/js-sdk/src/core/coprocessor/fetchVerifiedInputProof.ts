import type { RelayerInputProofOptions } from '../types/relayer.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { InputProof, VerifiedInputProof } from '../types/inputProof.js';
import type { ZkProof } from '../types/zkProof-p.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { CacheUsage } from '../caching/cacheUsage-p.js';
import { InputProofError } from '../errors/InputProofError.js';
import { assertHandleArrayEquals } from '../handle/FhevmHandle.js';
import { createInputProofFromInputHandles } from './InputProof-p.js';
import { verifyInputProof } from './verifyInputProof.js';
import { assertIsZkProof } from './ZkProof-p.js';
import { retryOnStaleCache } from '../caching/retryOnStaleCache-p.js';
import { recordCacheUsage } from '../caching/cacheUsage-p.js';
import { readVersionedCoprocessorSignersContext } from '../host-contracts/readCoprocessorSignersContext-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = FhevmBase<FhevmChain>;

type Parameters = {
  readonly zkProof: ZkProof;
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerInputProofOptions | undefined;
};

type ReturnType = VerifiedInputProof;

////////////////////////////////////////////////////////////////////////////////

/**
 * Sends the ZK proof to the relayer, then checks the coprocessor signatures
 * against the current signer set. A changed signer set is refreshed and the
 * same response checked again: the proof is not rebuilt, and not sent again
 * (the relayer answers the same proof with the same signatures).
 */
export async function fetchVerifiedInputProof(context: Context, parameters: Parameters): Promise<ReturnType> {
  const inputProof = await _fetchInputProof(context, parameters);
  return retryOnStaleCache(context, parameters.fhevmContext, (cacheUsage) =>
    _verifyInputProofWithCurrentSigners(context, { inputProof, fhevmContext: parameters.fhevmContext }, cacheUsage),
  );
}

////////////////////////////////////////////////////////////////////////////////

/** The input proof the relayer returns for `zkProof`, not checked yet. */
async function _fetchInputProof(context: Context, parameters: Parameters): Promise<InputProof> {
  const { zkProof, options, fhevmContext } = parameters;

  assertIsZkProof(zkProof, {});

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerOptions: RelayerInputProofOptions = {
    auth: context.runtime.config.auth,
    ...options,
  };

  // 1. extract FhevmHandles from the given ZK proof
  const inputHandles: readonly InputHandle[] = zkProof.getInputHandles();

  if (inputHandles.length === 0) {
    throw new InputProofError({
      message: `Input proof must contain at least one external handle`,
    });
  }

  // 2. Request coprocessor signatures from the relayer for the given ZK proof
  const { handles: coprocessorSignedHandles, coprocessorEip712Signatures: coprocessorSignatures } =
    await context.runtime.relayer.fetchCoprocessorSignatures(context, {
      payload: {
        zkProof,
      },
      options: relayerOptions,
      fhevmContext,
    });

  // 3. Check that the handles and the one in the fetch result
  // Note: this check is theoretically unnecessary
  // We prefer to perform this test since we do not trust the relayer
  // The purpose is to check if the relayer is possibly malicious
  assertHandleArrayEquals(inputHandles, coprocessorSignedHandles);

  // 4. The input proof, checked by the caller
  return createInputProofFromInputHandles({
    coprocessorEip712Signatures: coprocessorSignatures,
    inputHandles,
    extraData: zkProof.getExtraData(),
    signedHandleAccess: {
      userAddress: zkProof.userAddress,
      contractAddress: zkProof.contractAddress,
    },
  });
}

/**
 * The request for `retryOnStaleCache`: reads the current coprocessor signers,
 * records their version, and checks the input proof against them.
 */
async function _verifyInputProofWithCurrentSigners(
  context: Context,
  parameters: { readonly inputProof: InputProof; readonly fhevmContext: FhevmClientFrozenContext },
  cacheUsage: CacheUsage,
): Promise<ReturnType> {
  const coprocessorSigners = await readVersionedCoprocessorSignersContext(context, {
    address: context.chain.fhevm.contracts.inputVerifier.address as ChecksummedAddress,
    fhevmContext: parameters.fhevmContext,
  });
  recordCacheUsage(cacheUsage, 'coprocessorSigners', coprocessorSigners.version);
  return verifyInputProof(context, { inputProof: parameters.inputProof, coprocessorSigners: coprocessorSigners.value });
}
