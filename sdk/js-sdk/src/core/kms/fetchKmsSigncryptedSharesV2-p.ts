import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsExtraData, KmsSigncryptedShare, KmsSigncryptedSharesMetadata } from '../types/kms-p.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { RelayerUserDecryptOptions } from '../types/relayer.js';
import type { SignedDecryptionPermit, SignedDecryptionPermitV2 } from '../types/signedDecryptionPermit.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { UsedCacheVersions } from '../caching/usedCacheVersions-p.js';
import { assertHandlesBelongToSameChainId } from '../handle/FhevmHandle.js';
import { createKmsSigncryptedShares } from './KmsSigncryptedShares-p.js';
import { readVersionedKmsSignersContextFromPermitExtraData } from '../host-contracts/readKmsSignersContext-p.js';
import { recordUsedCacheVersion } from '../caching/usedCacheVersions-p.js';
import {
  assertSignedDecryptionPermitExtraDataSupported,
  isSignedDecryptionPermit,
} from './SignedDecryptionPermit-p.js';
import { isSignedDecryptionPermitV2 } from './SignedDecryptionPermitV2-p.js';
import { InvalidSignedPermitError } from '../errors/InvalidSignedPermitError.js';
import { assertKmsDecryptionBitLimit } from './utils.js';
import { checkPersistAllowed } from '../host-contracts/checkPersistAllowed.js';
import { checkDelegation } from '../host-contracts/checkDelegation.js';
import { createKmsEip712Domain } from './createKmsEip712Domain.js';
import { ensureRelayerFeatures } from '../relayerFeatures/ensureRelayerFeatures-p.js';

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
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly usedVersions?: UsedCacheVersions | undefined;
  readonly options?: RelayerUserDecryptOptions | undefined;
};

type ReturnType = KmsSigncryptedShares;

////////////////////////////////////////////////////////////////////////////////
// fetchKmsSigncryptedSharesV2
////////////////////////////////////////////////////////////////////////////////

export async function fetchKmsSigncryptedSharesV2(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { options, pairs, fhevmContext } = parameters;

  // Note:
  // Validate the permit first: an unusable permit fails here, without the network call of the
  // relayer-features check below.

  // The brand, not `version`: only the SDK builds a SignedDecryptionPermit.
  if (!isSignedDecryptionPermit(parameters.signedPermit)) {
    throw new InvalidSignedPermitError({
      message: 'Invalid permit: not a signed decryption permit created by the SDK',
    });
  }
  if (!isSignedDecryptionPermitV2(parameters.signedPermit)) {
    throw new InvalidSignedPermitError({
      message: `Unexpected permit version ${String(parameters.signedPermit.version)}, expecting version 2`,
    });
  }

  const signedPermitV2: SignedDecryptionPermitV2 = parameters.signedPermit;

  // Order: brand → version → extraData → expiry, then the input checks and the KMS context (same as canDecryptValuesFromPairs).
  const signedPermitExtraData: KmsExtraData = assertSignedDecryptionPermitExtraDataSupported(signedPermitV2);

  signedPermitV2.assertNotExpired();

  // The v3 / unified user-decrypt route only exists on relayers that advertise it. Guard here — not
  // only at callers — so any direct caller of this V2 path is protected too. Resolved once per client
  // (cached), using the same effective auth this function uses for the actual relayer request.
  const features = await ensureRelayerFeatures(context, {
    auth: options?.auth ?? context.runtime.config.auth,
  });
  if (!features.relayerSupportsV3UserDecryptRoute) {
    throw new Error(
      'The relayer does not support unified (V2) decryption permits. Call `canUseUnifiedDecryptionPermit` up ' +
        'front to check support, and use a legacy (V1) decryption permit against this relayer instead.',
    );
  }

  const tkmsVersion = fhevmContext.tkmsVersion;

  const relayerOptions = {
    auth: context.runtime.config.auth,
    ...options,
  };

  const { signerAddress, signature } = signedPermitV2;
  const userAddress = signedPermitV2.eip712.message.userAddress;

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

  // 4. Check: Expiration date — done up front, with the other permit checks

  // 4b. Check: the permit names the current KMS context (else
  //    InvalidSignedPermitError). After the input checks (it may read the
  //    chain), before the ACL reads.
  const { kmsSignersContext: requestedKmsSignersContext, version: kmsContextVersion } =
    await readVersionedKmsSignersContextFromPermitExtraData(context, {
      kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
      protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
      extraData: signedPermitExtraData,
    });

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

  // From here on, a failure may come from a KMS context switch: record the
  // context used (not earlier: a bad input or an ACL denial is not one).
  recordUsedCacheVersion(parameters.usedVersions, 'kmsContext', kmsContextVersion);
  // 8. Fetch `KmsSigncryptedShares` from the relayer (unified V2 route)
  const shares: readonly KmsSigncryptedShare[] = await context.runtime.relayer.fetchUserDecrypt(context, {
    version: 2,
    payload: {
      handleContractPairs: resolvedPairs,
      kmsDecryptEip712Signer: signerAddress,
      kmsDecryptEip712Message: signedPermitV2.eip712.message,
      kmsDecryptEip712Signature: signature,
    },
    options: relayerOptions,
    fhevmContext,
  });

  // 9. Build and verify the sealed validated `KmsSigncryptedShares` object
  const sharesMetadata: KmsSigncryptedSharesMetadata = {
    kmsSignersContext: requestedKmsSignersContext,
    eip712Domain: createKmsEip712Domain({
      chainId: context.chain.fhevm.gateway.id,
      verifyingContractAddressDecryption: context.chain.fhevm.gateway.contracts.decryption.address,
    }),
    eip712ExtraData: signedPermitV2.eip712.message.extraData,
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

  // 10. The returned KmsSigncryptedShares as sent by the relayer
  return createKmsSigncryptedShares({ metadata: sharesMetadata, shares });
}
