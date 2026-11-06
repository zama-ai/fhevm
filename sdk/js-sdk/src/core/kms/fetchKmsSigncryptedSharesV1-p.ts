import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsExtraData, KmsSigncryptedShare, KmsSigncryptedSharesMetadata } from '../types/kms-p.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../types/relayer.js';
import type { SignedDecryptionPermit, SignedDecryptionPermitV1 } from '../types/signedDecryptionPermit.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { assertHandlesBelongToSameChainId } from '../handle/FhevmHandle.js';
import { createKmsSigncryptedShares } from './KmsSigncryptedShares-p.js';
import { readKmsSignersContextFromPermitExtraData } from '../host-contracts/readKmsSignersContext-p.js';
import {
  assertSignedDecryptionPermitExtraDataSupported,
  isSignedDecryptionPermit,
} from './SignedDecryptionPermit-p.js';
import { isSignedDecryptionPermitV1 } from './SignedDecryptionPermitV1-p.js';
import { InvalidSignedPermitError } from '../errors/InvalidSignedPermitError.js';
import { assertKmsDecryptionBitLimit } from './utils.js';
import { checkPersistAllowed } from '../host-contracts/checkPersistAllowed.js';
import { createKmsEip712Domain } from './createKmsEip712Domain.js';
import { checkDelegation } from '../host-contracts/checkDelegation.js';

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
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
};

type ReturnType = KmsSigncryptedShares;

////////////////////////////////////////////////////////////////////////////////

const MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10;

/**
 * Asserts that every address in {@link contractAddresses} is listed in the
 * permit's `contractAddresses` (case-insensitive comparison).
 *
 * @throws {InvalidSignedPermitError} If an address is not listed in the permit.
 */
export function assertPermitV1IncludesContractAddresses(
  permit: SignedDecryptionPermitV1,
  contractAddresses: readonly string[],
): void {
  const permitAddresses = permit.eip712.message.contractAddresses;
  for (const address of contractAddresses) {
    if (!permitAddresses.some((a) => a.toLowerCase() === address.toLowerCase())) {
      throw new InvalidSignedPermitError({
        message:
          `Contract address ${address} is not listed in the permit's contractAddresses: ` +
          `sign a new permit that includes it`,
      });
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
// fetchKmsSigncryptedSharesV1
////////////////////////////////////////////////////////////////////////////////

export async function fetchKmsSigncryptedSharesV1(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { options, pairs, fhevmContext } = parameters;

  // The brand, not `version`: only the SDK builds a SignedDecryptionPermit.
  if (!isSignedDecryptionPermit(parameters.signedPermit)) {
    throw new InvalidSignedPermitError({
      message: 'Invalid permit: not a signed decryption permit created by the SDK',
    });
  }
  if (!isSignedDecryptionPermitV1(parameters.signedPermit)) {
    throw new InvalidSignedPermitError({
      message: `Unexpected permit version ${String(parameters.signedPermit.version)}, expecting version 1`,
    });
  }

  const signedPermitV1: SignedDecryptionPermitV1 = parameters.signedPermit;

  // Order: brand → version → extraData → expiry (same as canDecryptValuesFromPairs).

  // An unsupported extraData (v0, protocol v0.11) is refused by the KMS connector: fail here rather
  // than wait for a response that never comes.
  const signedPermitExtraData: KmsExtraData = assertSignedDecryptionPermitExtraDataSupported(signedPermitV1);

  signedPermitV1.assertNotExpired();

  const tkmsVersion = fhevmContext.tkmsVersion;

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

  // 6. Check: Expiration date — done up front, with the other permit checks

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
  // It is critical to fetch the exact kms signers associated with the extra data
  const requestedKmsSignersContext: KmsSignersContext = await readKmsSignersContextFromPermitExtraData(context, {
    kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
    extraData: signedPermitExtraData,
  });

  // 10. Fetch `KmsSigncryptedShares` from the relayer
  let shares: readonly KmsSigncryptedShare[];

  // The relayer forwards the eip712 and its signature to the KMS
  // The KMS does verify the user's EIP-712 signature server-side
  if (signedPermitV1.eip712.primaryType === 'DelegatedUserDecryptRequestVerification') {
    shares = await context.runtime.relayer.fetchDelegatedUserDecrypt(context, {
      version: 1,
      payload: {
        handleContractPairs,
        kmsDecryptEip712Signer: signerAddress,
        kmsDecryptEip712Message: signedPermitV1.eip712.message,
        kmsDecryptEip712Signature: signature,
      },
      fhevmContext,
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
      fhevmContext,
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
    eip712ExtraData: signedPermitV1.eip712.message.extraData,
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

  // 12. The returned KmsSigncryptedShares as sent by the relayer
  return createKmsSigncryptedShares({
    metadata: sharesMetadata,
    shares,
  });
}
