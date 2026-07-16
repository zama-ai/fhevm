import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShare, KmsSigncryptedSharesMetadata } from '../types/kms-p.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { RelayerUserDecryptOptions } from '../types/relayer.js';
import type { SignedDecryptionPermit, SignedDecryptionPermitV2 } from '../types/signedDecryptionPermit.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import { assertHandlesBelongToSameChainId } from '../handle/FhevmHandle.js';
import { createKmsSigncryptedShares } from './KmsSigncryptedShares-p.js';
import { readKmsSignersContextFromExtraData } from '../host-contracts/readKmsSignersContext-p.js';
import { assertIsSignedDecryptionPermit } from './SignedDecryptionPermit-p.js';
import { assertKmsDecryptionBitLimit } from './utils.js';
import { checkPersistAllowed } from '../host-contracts/checkPersistAllowed.js';
import { checkDelegation } from '../host-contracts/checkDelegation.js';
import { createKmsEip712Domain } from './createKmsEip712Domain.js';
import { resolveFhevmTkmsVersion } from '../runtime/resolveFhevmVersions-p.js';
import { EXTRA_DATA_V2, fromKmsExtraDataBytesHex } from './kmsExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly chain: FhevmChain;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly pairs: ReadonlyArray<{
    readonly handle: Handle;
    readonly contractAddress: ChecksummedAddress;
    // When omitted, defaults to userAddress (direct-access path).
    // Provide explicitly only for delegated handles where ownerAddress !== userAddress.
    readonly ownerAddress?: ChecksummedAddress | undefined;
  }>;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | undefined;
};

type ReturnType = KmsSigncryptedShares;

////////////////////////////////////////////////////////////////////////////////
// fetchKmsSigncryptedSharesV2
////////////////////////////////////////////////////////////////////////////////

export async function fetchKmsSigncryptedSharesV2(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { options, pairs } = parameters;
  const signedPermit = parameters.signedPermit as SignedDecryptionPermitV2;

  const tkmsVersion = await resolveFhevmTkmsVersion(context);

  const relayerOptions = {
    auth: context.runtime.config.auth,
    ...options,
  };

  assertIsSignedDecryptionPermit(signedPermit, {});

  /*
    it should not be possible to produce a unified eip712 
    (protocol v14+) with an extraData coming from an 
    old protocol v11/12/13.
  */
  const signedPermitVersion: number = signedPermit.version;
  if (signedPermitVersion !== 2) {
    throw Error(`fetchKmsSigncryptedSharesV2 requires a v2 permit, got v${signedPermitVersion}`);
  }
  const signedPermitExtraData = fromKmsExtraDataBytesHex(signedPermit.eip712.message.extraData);
  if (signedPermitExtraData.version < EXTRA_DATA_V2) {
    throw new Error(
      `fetchKmsSigncryptedSharesV2 error: Invalid extraData version extraData=${signedPermitExtraData.toBytesHex()}`,
    );
  }

  const { signerAddress, signature } = signedPermit;
  const userAddress = signedPermit.eip712.message.userAddress;

  // 1. Check: At least one handle/contract pair is required
  if (pairs.length === 0) {
    throw Error(
      `encrypted value/contract pairs must not be empty, at least one encrypted value/contract pair is required`,
    );
  }

  // Resolve ownerAddress for each pair: defaults to userAddress (direct-access path)
  // when not explicitly provided by the caller.
  const resolvedPairs = pairs.map((p) => ({
    handle: p.handle,
    contractAddress: p.contractAddress,
    ownerAddress: p.ownerAddress ?? userAddress,
  }));

  const handles = resolvedPairs.map((p) => p.handle);
  Object.freeze(handles);

  // 2. Check: All handles belong to the host chainId
  assertHandlesBelongToSameChainId(handles, BigInt(context.chain.id) as Uint64BigInt);

  // 3. Check: 2048 bits limit
  assertKmsDecryptionBitLimit(handles);

  // 4. Check: Expiration date
  signedPermit.assertNotExpired();

  // 5. Check: ACL permissions — split pairs by direct vs delegated access
  const directPairs = resolvedPairs.filter((p) => p.ownerAddress.toLowerCase() === userAddress.toLowerCase());
  const delegatedPairs = resolvedPairs.filter((p) => p.ownerAddress.toLowerCase() !== userAddress.toLowerCase());

  if (directPairs.length > 0) {
    await checkPersistAllowed(context, {
      aclAddress: context.chain.fhevm.contracts.acl.address as ChecksummedAddress,
      userAddress,
      handleContractPairs: directPairs,
    });
  }

  if (delegatedPairs.length > 0) {
    // Group delegated pairs by delegator (ownerAddress): a single batch may contain
    // handles from multiple delegators, each requiring a separate ACL delegation check.
    const byDelegator = new Map<ChecksummedAddress, Array<(typeof delegatedPairs)[number]>>();
    for (const pair of delegatedPairs) {
      const group = byDelegator.get(pair.ownerAddress) ?? [];
      group.push(pair);
      byDelegator.set(pair.ownerAddress, group);
    }
    for (const [delegator, groupedPairs] of byDelegator) {
      await checkDelegation(context, {
        aclAddress: context.chain.fhevm.contracts.acl.address as ChecksummedAddress,
        delegate: userAddress,
        delegator,
        handleContractPairs: groupedPairs,
      });
    }
  }

  // 6. Verify the Eip712 signature
  // Not required because a signedPermit is guaranteed to be verified.

  // 7. Fetch `KmsSignersContext` on-chain (cached)
  const requestedKmsSignersContext: KmsSignersContext = await readKmsSignersContextFromExtraData(context, {
    kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
    extraData: signedPermitExtraData,
  });

  // 8. Fetch `KmsSigncryptedShares` from the relayer (unified V2 route)
  const shares: readonly KmsSigncryptedShare[] = await context.runtime.relayer.fetchUserDecrypt(context, {
    version: 2,
    payload: {
      handleContractPairs: resolvedPairs,
      kmsDecryptEip712Signer: signerAddress,
      kmsDecryptEip712Message: signedPermit.eip712.message,
      kmsDecryptEip712Signature: signature,
    },
    options: relayerOptions,
  });

  // 9. Build and verify the sealed validated `KmsSigncryptedShares` object
  const sharesMetadata: KmsSigncryptedSharesMetadata = {
    kmsSignersContext: requestedKmsSignersContext,
    eip712Domain: createKmsEip712Domain({
      chainId: context.chain.fhevm.gateway.id,
      verifyingContractAddressDecryption: context.chain.fhevm.gateway.contracts.decryption.address,
    }),
    eip712Signature: signature,
    eip712SignerAddress: signerAddress,
    handles,
    tkmsVersion,
  };

  /*

    ----------------------------------------------------------------------------
    KMS response-signature verification.

    Each share's signature is an EIP-712 signature over the
    `UserDecryptResponseVerification` struct (see `verifyKmsSigncryptedShare` and
    gateway `Decryption.sol`). This check is already performed inside the tkms WASM
    during reconstruction (verify=true); the call below is an equivalent JS-only
    pass, kept for testing / debugging. To fully check, loop over every share.
    ----------------------------------------------------------------------------

    for (const share of shares) {
      await verifyKmsSigncryptedShare(
        context, 
        {
          metadata: sharesMetadata,
          share,
          transportPublicKey: signedPermit.transportPublicKey,
        }
      );
    }

  */

  // 10. The returned KmsSigncryptedShares is guaranteed to be fully verified:
  // uniform extraData across shares, valid extraData format, and consistency
  // with the KmsSignersContext (see KmsSigncryptedSharesImpl invariants).
  return await createKmsSigncryptedShares(context, {
    metadata: sharesMetadata,
    shares,
  });
}
