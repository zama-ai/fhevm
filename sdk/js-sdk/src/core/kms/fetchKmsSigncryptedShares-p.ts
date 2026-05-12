import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShare, KmsSigncryptedSharesMetadata } from '../types/kms-p.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../types/relayer.js';
import type { SignedDelegatedDecryptionPermit, SignedSelfDecryptionPermit } from '../types/signedDecryptionPermit.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import { assertHandlesBelongToSameChainId } from '../handle/FhevmHandle.js';
import { createKmsSigncryptedShares } from '../kms/KmsSigncryptedShares-p.js';
import { readKmsSignersContext } from '../host-contracts/readKmsSignersContext-p.js';
import { assertIsSignedDecryptionPermit, assertPermitIncludesContractAddresses } from './SignedDecryptionPermit-p.js';
import { assertKmsDecryptionBitLimit } from './utils.js';
import { checkPersistAllowed } from '../host-contracts/checkPersistAllowed.js';
import { assertExtraDataMatchesKmsSingersContext } from '../host-contracts/KmsSignersContext-p.js';
import { createKmsEip712Domain } from './createKmsEip712Domain.js';

/*
    See: in KMS (eip712Domain)
    json.response[i].signature is an eip712 sig potentially on this message:

    struct UserDecryptResponseVerification {
        bytes publicKey;
        bytes32[] ctHandles;
        bytes userDecryptedShare;
        bytes extraData;
    }
}    
*/

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly chain: FhevmChain;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters =
  | {
      readonly pairs: ReadonlyArray<{
        readonly handle: Handle;
        readonly contractAddress: ChecksummedAddress;
      }>;
      readonly signedPermit: SignedSelfDecryptionPermit;
      readonly options?: RelayerUserDecryptOptions | undefined;
    }
  | {
      readonly pairs: ReadonlyArray<{
        readonly handle: Handle;
        readonly contractAddress: ChecksummedAddress;
      }>;
      readonly signedPermit: SignedDelegatedDecryptionPermit;
      readonly options?: RelayerDelegatedUserDecryptOptions | undefined;
    };

type ReturnType = KmsSigncryptedShares;

////////////////////////////////////////////////////////////////////////////////

const MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10;

////////////////////////////////////////////////////////////////////////////////
// fetchKmsSignedcryptedShares
////////////////////////////////////////////////////////////////////////////////

export async function fetchKmsSignedcryptedShares(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { signedPermit, options, pairs } = parameters;

  // Check: every requested contractAddress is listed in the permit
  assertPermitIncludesContractAddresses(
    signedPermit,
    pairs.map((p) => p.contractAddress),
  );

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerOptions = {
    auth: context.runtime.config.auth,
    ...options,
  };

  assertIsSignedDecryptionPermit(signedPermit, {});

  // The max number of contracts allowed in a permit is managed by the `SignedDecryptionPermit` directly
  const { encryptedDataOwnerAddress, signerAddress, signature } = signedPermit;

  // 1. Check: At least one handle/contract pair is required
  if (pairs.length === 0) {
    throw Error(
      `encrypted value/contract pairs must not be empty, at least one encrypted value/contract pair is required`,
    );
  }

  // 2. Check: At least one contract
  const contractAddressesLength = signedPermit.eip712.message.contractAddresses.length;
  if (contractAddressesLength === 0) {
    throw Error('contractAddresses is empty');
  }

  // 3. Check: No more that 10 contract addresses
  if (contractAddressesLength > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
    throw Error(`contractAddresses max length of ${MAX_USER_DECRYPT_CONTRACT_ADDRESSES} exceeded`);
  }

  const handleContractPairs = pairs;
  const handles = pairs.map((p) => p.handle);
  Object.freeze(handles);

  // 4. Check: All handles belong to the host chainId
  assertHandlesBelongToSameChainId(handles, BigInt(context.chain.id) as Uint64BigInt);

  // 5. Check: 2048 bits limit
  assertKmsDecryptionBitLimit(handles);

  // 6. Check: Expiration date
  signedPermit.assertNotExpired();

  // 7. Check: ACL permissions (user is signer or delegatorAddress)
  await checkPersistAllowed(context, {
    address: context.chain.fhevm.contracts.acl.address as ChecksummedAddress,
    userAddress: encryptedDataOwnerAddress,
    handleContractPairs,
  });

  // 8. Verify the EIP712 signature
  // Not required because a signedPermit is guaranteed to be verified.

  // 9. Fetch `KmsSignersContext` on-chain (cached)
  // Reject the permit early if it was signed against a different KMS context
  // (e.g. stale permit from a previous context rotation).
  //
  // Compares the `extraData` embedded in the permit's EIP-712 message with the
  // `extraData` derived from the provided context. A mismatch indicates the permit
  // was created for a different KMS context (e.g. different context ID or version)
  // and must not be used for decryption.
  //
  // TODO: The current check is a strict byte-level comparison. A permit signed
  // with the correct `kmsContextId` but a different `extraData` encoding format
  // (e.g. a version change in the serialization scheme) will be rejected even
  // though the context ID matches. Consider comparing the decoded `kmsContextId`
  // instead of the raw `extraData` bytes.
  const requestedKmsSignersContext: KmsSignersContext = await readKmsSignersContext(context, {
    address: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
  });

  assertExtraDataMatchesKmsSingersContext(
    {
      extraData: signedPermit.eip712.message.extraData,
      kmsSignersContext: requestedKmsSignersContext,
    },
    { subject: 'Invalid permit' },
  );

  // 10. Fetch `KmsSigncryptedShares` from the relayer
  // Safe casts: the discriminated union on parameters guarantees
  // that options type matches signedPermit type, but TS can't prove
  // it after destructuring (nested discriminant limitation).
  const relayerUrl = context.chain.fhevm.relayerUrl;

  let shares: readonly KmsSigncryptedShare[];

  if (signedPermit.isDelegated) {
    shares = await context.runtime.relayer.fetchDelegatedUserDecrypt(
      { relayerUrl, chainId: context.chain.id },
      {
        payload: {
          handleContractPairs,
          kmsDecryptEip712Signer: signerAddress,
          kmsDecryptEip712Message: signedPermit.eip712.message,
          kmsDecryptEip712Signature: signature,
        },
        options: relayerOptions as RelayerDelegatedUserDecryptOptions,
      },
    );
  } else {
    shares = await context.runtime.relayer.fetchUserDecrypt(
      { relayerUrl, chainId: context.chain.id },
      {
        payload: {
          handleContractPairs,
          kmsDecryptEip712Signer: signerAddress,
          kmsDecryptEip712Message: signedPermit.eip712.message,
          kmsDecryptEip712Signature: signature,
        },
        options: relayerOptions as RelayerUserDecryptOptions,
      },
    );
  }

  // 11. Build and verify the sealed validated `KmsSigncryptedShares` object
  const sharesMetadata: KmsSigncryptedSharesMetadata = {
    kmsSignersContext: requestedKmsSignersContext,
    eip712Domain: createKmsEip712Domain({
      chainId: context.chain.fhevm.gateway.id,
      verifyingContractAddressDecryption: context.chain.fhevm.gateway.contracts.decryption.address,
    }),
    eip712Signature: signature,
    eip712SignerAddress: signerAddress,
    handles,
  };

  // 12. The returned KmsSigncryptedShares is guaranteed to be fully verified:
  // uniform extraData across shares, valid extraData format, and consistency
  // with the KmsSignersContext (see KmsSigncryptedSharesImpl invariants).
  return await createKmsSigncryptedShares(context, {
    metadata: sharesMetadata,
    shares,
  });
}
