import type { FhevmChain } from '../types/fhevmChain.js';
import type { SignedDecryptionPermit } from '../types/signedDecryptionPermit.js';
import type { TransportKeyPair } from '../kms/TransportKeyPair-p.js';
import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import { addressToChecksummedAddress, assertIsAddress } from '../base/address.js';
import { assertHandlesBelongToSameChainId } from '../handle/FhevmHandle.js';
import {
  assertIsSignedDecryptionPermit,
  assertSignedDecryptionPermitExtraDataSupported,
} from '../kms/SignedDecryptionPermit-p.js';
import { assertPermitV1IncludesContractAddresses } from '../kms/fetchKmsSigncryptedSharesV1-p.js';
import { assertPermitMatchesKeyPair } from '../utils-p/decrypt/assertPermitMatchesKeyPair.js';
import { isUserDecryptable } from '../host-contracts/isUserDecryptable.js';
import { readKmsSignersContextFromPermitExtraData } from './readKmsSignersContext-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters =
  | {
      readonly pairs: ReadonlyArray<{ readonly handle: Handle; readonly contractAddress: ChecksummedAddress }>;
      readonly userAddress: string;
    }
  | {
      readonly pairs: ReadonlyArray<{ readonly handle: Handle; readonly contractAddress: ChecksummedAddress }>;
      readonly signedPermit: SignedDecryptionPermit;
      readonly transportKeyPair?: TransportKeyPair | undefined;
    };

type ReturnType = {
  readonly allowed: boolean;
  readonly details: ReadonlyArray<{
    readonly contractAllowed: boolean;
    readonly userAllowed: boolean;
  }>;
};

////////////////////////////////////////////////////////////////////////////////

export async function canDecryptValuesFromPairs(context: Context, parameters: Parameters): Promise<ReturnType> {
  if (parameters.pairs.length === 0) {
    throw Error('list of encrypted values cannot be empty, at least one encrypted value is required');
  }

  const handleContractPairs = parameters.pairs;
  const handles = handleContractPairs.map((pair) => pair.handle);
  const contractAddresses = handleContractPairs.map((pair) => pair.contractAddress);

  assertHandlesBelongToSameChainId(handles, BigInt(context.chain.id) as Uint64BigInt);

  let userAddress: ChecksummedAddress;

  if ('userAddress' in parameters) {
    assertIsAddress(parameters.userAddress, {});
    userAddress = addressToChecksummedAddress(parameters.userAddress);
  } else {
    const { signedPermit } = parameters;

    // Order: brand → version → extraData → expiry, then the input checks and the KMS context
    // (same as fetchKmsSigncryptedSharesV1 / V2).
    // The version is checked by assertSignedDecryptionPermitExtraDataSupported.
    assertIsSignedDecryptionPermit(signedPermit, { subject: 'signedPermit' });
    const signedPermitExtraData = assertSignedDecryptionPermitExtraDataSupported(signedPermit);

    signedPermit.assertNotExpired();

    if (signedPermit.version === 1) {
      assertPermitV1IncludesContractAddresses(signedPermit, contractAddresses);
    }

    if (parameters.transportKeyPair !== undefined) {
      assertPermitMatchesKeyPair(signedPermit, parameters.transportKeyPair);
    }

    // Only a permit for the current KMS context can decrypt. After the input
    // checks above: it may read the chain.
    await readKmsSignersContextFromPermitExtraData(context, {
      kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
      protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
      extraData: signedPermitExtraData,
    });

    userAddress = signedPermit.encryptedDataOwnerAddress;
  }

  for (const contractAddress of contractAddresses) {
    if (userAddress.toLowerCase() === contractAddress.toLowerCase()) {
      throw new Error(
        `userAddress ${userAddress} should not be equal to contractAddress when requesting user decryption!`,
      );
    }
  }

  const details = await isUserDecryptable(context, {
    aclAddress: context.chain.fhevm.contracts.acl.address as ChecksummedAddress,
    userAddress,
    handleContractPairs,
  });

  return {
    allowed: details.every((d) => d.contractAllowed && d.userAllowed),
    details,
  };
}
