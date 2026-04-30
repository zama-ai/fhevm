import type {
  SignedDecryptionPermit,
  SignedDelegatedDecryptionPermit,
  SignedSelfDecryptionPermit,
} from '../types/signedDecryptionPermit.js';
import type { KmsDelegatedUserDecryptEip712, KmsUserDecryptEip712 } from '../types/kms.js';
import type {
  Bytes65Hex,
  BytesHex,
  ChecksummedAddress,
  Uint256BigInt,
  Uint8Number,
  UintNumber,
} from '../types/primitives.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { NativeSigner } from '../modules/ethereum/types.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import { verifyKmsUserDecryptEip712 } from '../utils-p/decrypt/verifyKmsUserDecryptEip712.js';
import { verifyKmsDelegatedUserDecryptEip712 } from '../utils-p/decrypt/verifyKmsDelegatedUserDecryptEip712.js';
import { assertRecordNonNullableProperty } from '../base/record.js';
import { assertRecordBytes65HexProperty } from '../base/bytes.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { addressToChecksummedAddress, assertIsAddress, assertRecordAddressProperty } from '../base/address.js';
import { assertIsKmsUserDecryptEip712, createKmsUserDecryptEip712 } from './createKmsUserDecryptEip712.js';
import {
  assertIsKmsDelegatedUserDecryptEip712,
  createKmsDelegatedUserDecryptEip712,
} from './createKmsDelegatedUserDecryptEip712.js';
import { assertRecordStringProperty } from '../base/string.js';
import { assertIsTransportKeyPair, type TransportKeyPair } from './TransportKeyPair-p.js';
import { assertKmsEIP712DeadlineValidity } from './utils.js';
import { readKmsSignersContext } from '../host-contracts/readKmsSignersContext-p.js';
import { kmsSignersContextToExtraData } from '../host-contracts/KmsSignersContext-p.js';
import { fromKmsExtraData } from './kmsExtraData.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('SignedDecryptionPermit.token');
const MAX_USER_DECRYPT_DURATION_DAYS = 365 as UintNumber;
const MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10 as Uint8Number;

////////////////////////////////////////////////////////////////////////////////

/**
 * Private implementation of {@link SignedDecryptionPermit}.
 * Immutable by design — all fields are stored as private properties
 * and exposed via readonly getters. Instances are only created through
 * SDK-internal factory functions that guarantee the signature has been verified.
 *
 * **Invariant:** Every instance of this class has a verified signature.
 * Construction is only possible through factory functions that validate the
 * signature against the on-chain verifier before returning. If the EIP-712
 * format has changed (e.g. loading a permit serialized by an older SDK version),
 * verification will fail and construction will throw.
 *
 * As a consequence, any code within the SDK that receives a
 * `SignedDecryptionPermit` (verified via `instanceof`) can safely trust
 * its contents without re-verification.
 */
abstract class SignedDecryptionPermitBaseImpl {
  readonly #eip712: KmsUserDecryptEip712 | KmsDelegatedUserDecryptEip712;
  readonly #signature: Bytes65Hex;
  readonly #signerAddress: ChecksummedAddress;

  constructor(
    privateToken: symbol,
    parameters: {
      readonly eip712: KmsUserDecryptEip712 | KmsDelegatedUserDecryptEip712;
      readonly signature: Bytes65Hex;
      readonly signerAddress: ChecksummedAddress;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#eip712 = parameters.eip712;
    this.#signature = parameters.signature;
    this.#signerAddress = parameters.signerAddress;
  }

  public get eip712(): KmsUserDecryptEip712 | KmsDelegatedUserDecryptEip712 {
    return this.#eip712;
  }

  public get signature(): Bytes65Hex {
    return this.#signature;
  }

  public get signerAddress(): ChecksummedAddress {
    return this.#signerAddress;
  }

  public abstract get encryptedDataOwnerAddress(): ChecksummedAddress;
  public abstract get isDelegated(): boolean;

  public assertNotExpired(): void {
    assertKmsEIP712DeadlineValidity(this.#eip712.message, MAX_USER_DECRYPT_DURATION_DAYS);
  }

  /**
   * Asserts that this permit was signed against the given {@link KmsSignersContext}.
   *
   * Compares the `extraData` embedded in the permit's EIP-712 message with the
   * `extraData` derived from the provided context. A mismatch indicates the permit
   * was created for a different KMS context (e.g. different context ID or version)
   * and must not be used for decryption.
   *
   * @todo The current check is a strict byte-level comparison. A permit signed
   *   with the correct `kmsContextId` but a different `extraData` encoding format
   *   (e.g. a version change in the serialization scheme) will be rejected even
   *   though the context ID matches. Consider comparing the decoded `kmsContextId`
   *   instead of the raw `extraData` bytes.
   *
   * @param kmsSignersContext - The current KMS signers context to validate against.
   * @throws If the permit's `extraData` does not match the context's `extraData`.
   */
  public assertMatchesKmsContext(kmsSignersContext: KmsSignersContext): void {
    const expectedExtraData = kmsSignersContextToExtraData(kmsSignersContext);
    if (expectedExtraData !== this.#eip712.message.extraData) {
      throw new Error(
        `Invalid permit: extraData "${this.#eip712.message.extraData}" does not match expected "${expectedExtraData}" from KmsSignersContext.`,
      );
    }
  }

  public get e2eTransportPublicKey(): BytesHex {
    return this.#eip712.message.publicKey;
  }

  public get kmsContextId(): Uint256BigInt {
    return fromKmsExtraData(this.#eip712.message.extraData).kmsContextId;
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Serializes a signed decryption permit to a plain object suitable for
 * JSON serialization. Uses the public getters — does not access private fields.
 *
 * `toJSON()` is intentionally not on the class to prevent accidental
 * serialization of sensitive data via `JSON.stringify(permit)`.
 */
export function serializeSignedDecryptionPermitToJSON(permit: SignedDecryptionPermit): {
  eip712: KmsUserDecryptEip712 | KmsDelegatedUserDecryptEip712;
  signature: string;
  signerAddress: string;
} {
  assertIsSignedDecryptionPermit(permit, {});
  return {
    eip712: permit.eip712,
    signature: permit.signature,
    signerAddress: permit.signerAddress,
  };
}

////////////////////////////////////////////////////////////////////////////////
// SignedUserDecryptionPermitImpl
////////////////////////////////////////////////////////////////////////////////

class SignedSelfDecryptionPermitImpl extends SignedDecryptionPermitBaseImpl implements SignedSelfDecryptionPermit {
  public override get eip712(): KmsUserDecryptEip712 {
    return super.eip712 as KmsUserDecryptEip712;
  }

  public override get encryptedDataOwnerAddress(): ChecksummedAddress {
    return this.signerAddress;
  }

  public override get isDelegated(): false {
    return false;
  }
}

////////////////////////////////////////////////////////////////////////////////
// SignedDelegatedDecryptionPermitImpl
////////////////////////////////////////////////////////////////////////////////

class SignedDelegatedDecryptionPermitImpl
  extends SignedDecryptionPermitBaseImpl
  implements SignedDelegatedDecryptionPermit
{
  public override get eip712(): KmsDelegatedUserDecryptEip712 {
    return super.eip712 as KmsDelegatedUserDecryptEip712;
  }

  public override get encryptedDataOwnerAddress(): ChecksummedAddress {
    return this.eip712.message.delegatorAddress;
  }

  public override get isDelegated(): true {
    return true;
  }
}

////////////////////////////////////////////////////////////////////////////////

Object.freeze(SignedDecryptionPermitBaseImpl);
Object.freeze(SignedDecryptionPermitBaseImpl.prototype);
Object.freeze(SignedSelfDecryptionPermitImpl);
Object.freeze(SignedSelfDecryptionPermitImpl.prototype);
Object.freeze(SignedDelegatedDecryptionPermitImpl);
Object.freeze(SignedDelegatedDecryptionPermitImpl.prototype);

////////////////////////////////////////////////////////////////////////////////
// isSignedDecryptionPermit
////////////////////////////////////////////////////////////////////////////////

export function isSignedDecryptionPermit(value: unknown): value is SignedDecryptionPermit {
  return value instanceof SignedDecryptionPermitBaseImpl;
}

/** Throws {@link InvalidTypeError} if value is not a valid {@link SignedDecryptionPermit}. */
export function assertIsSignedDecryptionPermit(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is SignedDecryptionPermit {
  if (!isSignedDecryptionPermit(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'SignedDecryptionPermit',
      },
      options,
    );
  }
}

/**
 * Asserts that every address in {@link contractAddresses} is listed in the
 * permit's `contractAddresses` (case-insensitive comparison).
 */
export function assertPermitIncludesContractAddresses(
  permit: SignedDecryptionPermit,
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
// createSignedDecryptionPermit
////////////////////////////////////////////////////////////////////////////////

async function _createSignedDecryptionPermit(
  context: { readonly chain: FhevmChain; readonly runtime: FhevmRuntime },
  parameters: {
    readonly signerAddress: ChecksummedAddress;
    readonly eip712: KmsUserDecryptEip712 | KmsDelegatedUserDecryptEip712;
    readonly signature: Bytes65Hex;
  },
): Promise<SignedDecryptionPermit> {
  const { signerAddress, eip712, signature } = parameters;

  if (eip712.message.contractAddresses.length === 0) {
    throw Error('contractAddresses is empty');
  }

  if (eip712.message.contractAddresses.length > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
    throw Error(`contractAddresses max length of ${MAX_USER_DECRYPT_CONTRACT_ADDRESSES} exceeded`);
  }

  if (Number(eip712.message.durationDays) > MAX_USER_DECRYPT_DURATION_DAYS) {
    throw Error(`durationDays is above max duration of ${MAX_USER_DECRYPT_DURATION_DAYS}`);
  }

  if (eip712.primaryType === 'UserDecryptRequestVerification') {
    await verifyKmsUserDecryptEip712(context, {
      signer: signerAddress,
      message: eip712.message,
      signature,
    });
    return new SignedSelfDecryptionPermitImpl(PRIVATE_TOKEN, parameters);
  } else {
    await verifyKmsDelegatedUserDecryptEip712(context, {
      signer: signerAddress,
      message: eip712.message,
      signature,
    });
    return new SignedDelegatedDecryptionPermitImpl(PRIVATE_TOKEN, parameters);
  }
}

export type SignDecryptionPermitContext = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type SignDecryptionPermitCommonParameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationDays: number;
  readonly signerAddress: string;
  readonly signer: NativeSigner;
  readonly transportKeyPair: TransportKeyPair;
};

export type SignSelfDecryptionPermitParameters = SignDecryptionPermitCommonParameters & {
  readonly delegatorAddress?: undefined;
};

export type SignDelegatedDecryptionPermitParameters = SignDecryptionPermitCommonParameters & {
  readonly delegatorAddress: string;
};

export type SignDecryptionPermitParameters =
  | SignSelfDecryptionPermitParameters
  | SignDelegatedDecryptionPermitParameters;

/**
 * Creates a signed decryption permit by constructing the EIP-712 typed data
 * and signing it with the provided signer.
 *
 * If `delegatorAddress` is provided, creates a delegated permit that allows the signer
 * to decrypt encrypted values belonging to the `delegatorAddress` account.
 * Otherwise, creates a standard permit where the signer decrypts their own values.
 *
 * The EIP-712 message includes the key pair's public key, allowing the gateway
 * to encrypt the decrypted result for this specific key pair.
 *
 * @throws If the signer, address, or key pair is invalid.
 * @throws If the signature verification fails.
 */
export async function signDecryptionPermit(
  context: SignDecryptionPermitContext,
  parameters: SignSelfDecryptionPermitParameters,
): Promise<SignedSelfDecryptionPermit>;
export async function signDecryptionPermit(
  context: SignDecryptionPermitContext,
  parameters: SignDelegatedDecryptionPermitParameters,
): Promise<SignedDelegatedDecryptionPermit>;
export async function signDecryptionPermit(
  context: SignDecryptionPermitContext,
  parameters: SignDecryptionPermitParameters,
): Promise<SignedDecryptionPermit> {
  const kmsSignersContext = await readKmsSignersContext(context, {
    address: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
  });

  const extraData: BytesHex = kmsSignersContextToExtraData(kmsSignersContext);

  const {
    contractAddresses,
    startTimestamp,
    durationDays,
    signerAddress: signerAddressArg,
    transportKeyPair: transportKeyPair,
    signer,
    delegatorAddress,
  } = parameters;

  assertIsTransportKeyPair(transportKeyPair, {});
  assertIsAddress(signerAddressArg, {});

  if (delegatorAddress !== undefined) {
    assertIsAddress(delegatorAddress, {});
    if (signerAddressArg.toLowerCase() === delegatorAddress.toLowerCase()) {
      throw new Error(
        'signerAddress and delegatorAddress must be different. ' +
          'Use a non-delegated permit to decrypt your own values.',
      );
    }
  }

  const signerAddress = addressToChecksummedAddress(signerAddressArg);

  const commonMessage = {
    verifyingContractAddressDecryption: context.chain.fhevm.gateway.contracts.decryption.address as ChecksummedAddress,
    chainId: context.chain.id,
    contractAddresses,
    durationDays,
    startTimestamp,
    extraData,
    publicKey: transportKeyPair.publicKey,
  };

  const eip712 =
    delegatorAddress !== undefined
      ? createKmsDelegatedUserDecryptEip712({
          ...commonMessage,
          delegatorAddress,
        })
      : createKmsUserDecryptEip712(commonMessage);

  const signature = await context.runtime.ethereum.signTypedData(signer, {
    account: signerAddress,
    ...eip712,
  });

  return await _createSignedDecryptionPermit(context, {
    signature,
    signerAddress,
    eip712,
  });
}

////////////////////////////////////////////////////////////////////////////////
// parseSignedDecryptionPermit
////////////////////////////////////////////////////////////////////////////////

export async function parseSignedDecryptionPermit(
  context: {
    readonly chain: FhevmChain;
    readonly runtime: FhevmRuntime;
    readonly client: NonNullable<object>;
    readonly options: { readonly batchRpcCalls: boolean };
  },
  transportKeyPair: TransportKeyPair,
  permit: unknown,
): Promise<SignedDecryptionPermit> {
  assertIsTransportKeyPair(transportKeyPair, {});

  const permitName = 'permit';
  const options = {};

  assertRecordNonNullableProperty(permit, 'eip712', permitName, options);
  assertRecordBytes65HexProperty(permit, 'signature', permitName, options);
  assertRecordAddressProperty(permit, 'signerAddress', permitName, options);

  const eip712 = permit.eip712;
  assertRecordStringProperty(eip712, 'primaryType', `${permitName}.eip712`, options);
  const primaryType = (eip712 as Record<string, unknown>).primaryType;

  if (primaryType === 'UserDecryptRequestVerification') {
    assertIsKmsUserDecryptEip712(eip712, `${permitName}.eip712`, options);
  } else if (primaryType === 'DelegatedUserDecryptRequestVerification') {
    assertIsKmsDelegatedUserDecryptEip712(eip712, `${permitName}.eip712`, options);
  } else {
    throw new Error(`Unknown permit primaryType: ${primaryType}`);
  }

  if (eip712.message.publicKey.toLowerCase() !== transportKeyPair.publicKey.toLowerCase()) {
    throw new Error(
      "The permit's publicKey does not match the E2eTransportKeyPair's publicKey. " +
        'Ensure the permit was signed with the same key pair.',
    );
  }

  return await _createSignedDecryptionPermit(context, {
    signature: permit.signature,
    eip712,
    signerAddress: addressToChecksummedAddress(permit.signerAddress),
  });
}
