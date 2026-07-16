import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShare, KmsSigncryptedSharesMetadata } from '../types/kms-p.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../types/relayer.js';
import type { SignedDecryptionPermit, SignedDecryptionPermitV1 } from '../types/signedDecryptionPermit.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import { assertHandlesBelongToSameChainId } from '../handle/FhevmHandle.js';
import { createKmsSigncryptedShares } from './KmsSigncryptedShares-p.js';
import { readKmsSignersContextFromExtraData } from '../host-contracts/readKmsSignersContext-p.js';
import { assertIsSignedDecryptionPermit } from './SignedDecryptionPermit-p.js';
import { assertKmsDecryptionBitLimit } from './utils.js';
import { checkPersistAllowed } from '../host-contracts/checkPersistAllowed.js';
import { createKmsEip712Domain } from './createKmsEip712Domain.js';
import { checkDelegation } from '../host-contracts/checkDelegation.js';
import { resolveFhevmTkmsVersion } from '../runtime/resolveFhevmVersions-p.js';
import { fromKmsExtraDataBytesHex } from './kmsExtraData-p.js';

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
  }>;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
};

type ReturnType = KmsSigncryptedShares;

////////////////////////////////////////////////////////////////////////////////

const MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10;

/**
 * Asserts that every address in {@link contractAddresses} is listed in the
 * permit's `contractAddresses` (case-insensitive comparison).
 */
export function assertPermitV1IncludesContractAddresses(
  permit: SignedDecryptionPermitV1,
  contractAddresses: readonly string[],
): void {
  const permitAddresses = permit.eip712.message.contractAddresses;
  for (const address of contractAddresses) {
    if (!permitAddresses.some((a) => a.toLowerCase() === address.toLowerCase())) {
      throw Error(`contract address ${address} is not listed in the permit's contractAddresses`);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
// fetchKmsSigncryptedSharesV1
////////////////////////////////////////////////////////////////////////////////

export async function fetchKmsSigncryptedSharesV1(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { options, pairs } = parameters;

  if (parameters.signedPermit.version !== 1) {
    throw Error(`fetchKmsSigncryptedSharesV1 requires a v1 permit, got v${parameters.signedPermit.version}`);
  }

  const signedPermitV1: SignedDecryptionPermitV1 = parameters.signedPermit;

  // This helper must support base clients, where TKMS is not mandatory
  // and tkmsVersion may not be initialized in the CoreFhevm instance yet.
  const tkmsVersion = await resolveFhevmTkmsVersion(context);

  // Check: every requested contractAddress is listed in the permit
  assertPermitV1IncludesContractAddresses(
    signedPermitV1,
    pairs.map((p) => p.contractAddress),
  );

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerOptions = {
    auth: context.runtime.config.auth,
    ...options,
  };

  assertIsSignedDecryptionPermit(signedPermitV1, {});

  // The max number of contracts allowed in a permit is managed by the `SignedDecryptionPermit` directly
  const { encryptedDataOwnerAddress, signerAddress, signature } = signedPermitV1;

  // 1. Check: At least one handle/contract pair is required
  if (pairs.length === 0) {
    throw Error(
      `encrypted value/contract pairs must not be empty, at least one encrypted value/contract pair is required`,
    );
  }

  // 2. Check: At least one contract
  const contractAddressesLength = signedPermitV1.eip712.message.contractAddresses.length;
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
  signedPermitV1.assertNotExpired();

  // 7. Check: ACL permissions (user is signer or delegatorAddress)
  if (signedPermitV1.eip712.primaryType === 'DelegatedUserDecryptRequestVerification') {
    await checkDelegation(context, {
      aclAddress: context.chain.fhevm.contracts.acl.address as ChecksummedAddress,
      delegate: signerAddress,
      delegator: encryptedDataOwnerAddress,
      handleContractPairs,
    });
  } else {
    await checkPersistAllowed(context, {
      aclAddress: context.chain.fhevm.contracts.acl.address as ChecksummedAddress,
      userAddress: encryptedDataOwnerAddress,
      handleContractPairs,
    });
  }

  // 8. Verify the Eip712 signature
  // Not required because a signedPermit is guaranteed to be verified.

  // 9. Fetch `KmsSignersContext` on-chain (cached)
  const extraData = fromKmsExtraDataBytesHex(signedPermitV1.eip712.message.extraData);
  const requestedKmsSignersContext: KmsSignersContext = await readKmsSignersContextFromExtraData(context, {
    kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
    extraData,
  });

  // 10. Fetch `KmsSigncryptedShares` from the relayer
  let shares: readonly KmsSigncryptedShare[];

  if (signedPermitV1.eip712.primaryType === 'DelegatedUserDecryptRequestVerification') {
    shares = await context.runtime.relayer.fetchDelegatedUserDecrypt(context, {
      version: 1,
      payload: {
        handleContractPairs,
        kmsDecryptEip712Signer: signerAddress,
        kmsDecryptEip712Message: signedPermitV1.eip712.message,
        kmsDecryptEip712Signature: signature,
      },
      options: relayerOptions as RelayerDelegatedUserDecryptOptions,
    });
  } else {
    shares = await context.runtime.relayer.fetchUserDecrypt(context, {
      version: 1,
      payload: {
        handleContractPairs,
        kmsDecryptEip712Signer: signerAddress,
        kmsDecryptEip712Message: signedPermitV1.eip712.message,
        kmsDecryptEip712Signature: signature,
      },
      options: relayerOptions as RelayerUserDecryptOptions,
    });
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

  // 12. The returned KmsSigncryptedShares is guaranteed to be fully verified:
  // uniform extraData across shares, valid extraData format, and consistency
  // with the KmsSignersContext (see KmsSigncryptedSharesImpl invariants).
  return await createKmsSigncryptedShares(context, {
    metadata: sharesMetadata,
    shares,
  });
}
