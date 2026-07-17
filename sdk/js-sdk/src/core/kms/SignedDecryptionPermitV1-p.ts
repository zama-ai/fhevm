import type { SignedDecryptionPermit, SignedDecryptionPermitV1 } from '../types/signedDecryptionPermit.js';
import type { KmsDelegatedUserDecryptEip712V1, KmsUserDecryptEip712V1 } from '../types/kms.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress, Uint8Number, UintNumber } from '../types/primitives.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsSignDecryptionPermitContext, KmsSignDecryptionPermitParameters } from './SignedDecryptionPermit-p.js';
import type { KmsExtraData } from '../types/kms-p.js';
import { verifyKmsUserDecryptEip712V1 } from '../utils-p/decrypt/verifyKmsUserDecryptEip712V1.js';
import { verifyKmsDelegatedUserDecryptEip712V1 } from '../utils-p/decrypt/verifyKmsDelegatedUserDecryptEip712V1.js';
import { assertRecordNonNullableProperty } from '../base/record.js';
import { assertRecordBytes65HexProperty } from '../base/bytes.js';
import { addressToChecksummedAddress, assertIsAddress, assertRecordAddressProperty } from '../base/address.js';
import { assertIsKmsUserDecryptEip712V1, createKmsUserDecryptEip712V1 } from './createKmsUserDecryptEip712V1.js';
import {
  assertIsKmsDelegatedUserDecryptEip712V1,
  createKmsDelegatedUserDecryptEip712V1,
} from './createKmsDelegatedUserDecryptEip712V1.js';
import { assertRecordStringProperty } from '../base/string.js';
import { assertIsTransportKeyPair, type TransportKeyPair } from './TransportKeyPair-p.js';
import { readCurrentKmsSignersContext } from '../host-contracts/readKmsSignersContext-p.js';
import { kmsSignersContextToExtraData } from '../host-contracts/KmsSignersContext-p.js';
import { isUintNumber, isUintString, MAX_UINT256, secondsToDays } from '../base/uint.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('SignedDecryptionPermitV1.token');
const MAX_USER_DECRYPT_DURATION_DAYS = 365 as UintNumber;
const MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10 as Uint8Number;

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
class SignedDecryptionPermitV1Impl implements SignedDecryptionPermitV1 {
  readonly #eip712: KmsUserDecryptEip712V1 | KmsDelegatedUserDecryptEip712V1;
  readonly #signature: Bytes65Hex;
  readonly #signerAddress: ChecksummedAddress;

  constructor(
    privateToken: symbol,
    parameters: {
      readonly eip712: KmsUserDecryptEip712V1 | KmsDelegatedUserDecryptEip712V1;
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

  public get eip712(): KmsUserDecryptEip712V1 | KmsDelegatedUserDecryptEip712V1 {
    return this.#eip712;
  }

  public get signature(): Bytes65Hex {
    return this.#signature;
  }

  public get signerAddress(): ChecksummedAddress {
    return this.#signerAddress;
  }

  public readonly version = 1 as const;

  public assertNotExpired(): void {
    _assertKmsEip712V1DeadlineValidity(this.#eip712.message, MAX_USER_DECRYPT_DURATION_DAYS);
  }

  public get transportPublicKey(): BytesHex {
    return this.#eip712.message.publicKey;
  }

  public get encryptedDataOwnerAddress(): ChecksummedAddress {
    if (this.eip712.primaryType === 'DelegatedUserDecryptRequestVerification') {
      return this.eip712.message.delegatorAddress;
    }
    return this.signerAddress;
  }

  public get isDelegated(): boolean {
    return this.eip712.primaryType === 'DelegatedUserDecryptRequestVerification';
  }
}

////////////////////////////////////////////////////////////////////////////////

Object.freeze(SignedDecryptionPermitV1Impl);
Object.freeze(SignedDecryptionPermitV1Impl.prototype);

////////////////////////////////////////////////////////////////////////////////

export function isSignedDecryptionPermitV1(value: unknown): value is SignedDecryptionPermitV1 {
  return value instanceof SignedDecryptionPermitV1Impl;
}

////////////////////////////////////////////////////////////////////////////////
// signDecryptionPermitV1
////////////////////////////////////////////////////////////////////////////////

export async function signDecryptionPermitV1(
  context: KmsSignDecryptionPermitContext,
  parameters: KmsSignDecryptionPermitParameters,
): Promise<SignedDecryptionPermit> {
  const { signerAddress: signerAddressArg, signer, delegatorAddress } = parameters;
  assertIsAddress(signerAddressArg, {});
  const signerAddress = addressToChecksummedAddress(signerAddressArg);

  if (delegatorAddress !== undefined) {
    if (signerAddress.toLowerCase() === delegatorAddress.toLowerCase()) {
      throw new Error(
        'signerAddress and delegatorAddress must be different. ' +
          'Use a non-delegated permit to decrypt your own values.',
      );
    }
  }

  const eip712 = await createUnsignedDecryptionPermitEip712V1(context, parameters);

  const signature = await context.runtime.ethereum.signTypedData(signer, {
    account: signerAddress,
    ...eip712,
  });

  return await _createSignedDecryptionPermitV1(context, {
    signature,
    signerAddress,
    eip712,
  });
}

////////////////////////////////////////////////////////////////////////////////
// createUnsignedDecryptionPermitEip712V1
////////////////////////////////////////////////////////////////////////////////

export async function createUnsignedDecryptionPermitEip712V1(
  context: KmsSignDecryptionPermitContext,
  parameters: Omit<KmsSignDecryptionPermitParameters, 'signer' | 'signerAddress'>,
): Promise<KmsDelegatedUserDecryptEip712V1 | KmsUserDecryptEip712V1> {
  const { contractAddresses, startTimestamp, durationSeconds, transportKeyPair, delegatorAddress } = parameters;

  const durationDays = secondsToDays(durationSeconds, { subject: 'durationSeconds' });

  assertIsTransportKeyPair(transportKeyPair, {});

  if (delegatorAddress !== undefined) {
    assertIsAddress(delegatorAddress, {});
  }

  const kmsSignersContext = await readCurrentKmsSignersContext(context, {
    kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
  });

  const extraData: KmsExtraData = kmsSignersContextToExtraData(kmsSignersContext);

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
      ? createKmsDelegatedUserDecryptEip712V1({
          ...commonMessage,
          delegatorAddress,
        })
      : createKmsUserDecryptEip712V1(commonMessage);

  _validateDecryptionPermitEip712V1(eip712);

  return eip712;
}

////////////////////////////////////////////////////////////////////////////////
// parseSignedDecryptionPermitV1
////////////////////////////////////////////////////////////////////////////////

export async function parseSignedDecryptionPermitV1(
  context: KmsSignDecryptionPermitContext,
  transportKeyPair: TransportKeyPair,
  permit: unknown,
): Promise<SignedDecryptionPermit> {
  assertIsTransportKeyPair(transportKeyPair, {});

  const permitName = 'permit-v1';
  const options = {};

  assertRecordNonNullableProperty(permit, 'eip712', permitName, options);
  assertRecordBytes65HexProperty(permit, 'signature', permitName, options);
  assertRecordAddressProperty(permit, 'signerAddress', permitName, options);

  const eip712 = permit.eip712;
  assertRecordStringProperty(eip712, 'primaryType', `${permitName}.eip712`, options);
  const primaryType = (eip712 as Record<string, unknown>).primaryType;

  const signerAddress = addressToChecksummedAddress(permit.signerAddress);

  if (primaryType === 'UserDecryptRequestVerification') {
    assertIsKmsUserDecryptEip712V1(eip712, `${permitName}.eip712`, options);
    if (eip712.message.publicKey.toLowerCase() !== transportKeyPair.publicKey.toLowerCase()) {
      throw new Error(
        "The permit's publicKey does not match the TransportKeyPair's publicKey. " +
          'Ensure the permit was signed with the same key pair.',
      );
    }
    return await _createSignedDecryptionPermitV1(context, { signature: permit.signature, eip712, signerAddress });
  } else if (primaryType === 'DelegatedUserDecryptRequestVerification') {
    assertIsKmsDelegatedUserDecryptEip712V1(eip712, `${permitName}.eip712`, options);
    if (eip712.message.publicKey.toLowerCase() !== transportKeyPair.publicKey.toLowerCase()) {
      throw new Error(
        "The permit's publicKey does not match the TransportKeyPair's publicKey. " +
          'Ensure the permit was signed with the same key pair.',
      );
    }
    return await _createSignedDecryptionPermitV1(context, { signature: permit.signature, eip712, signerAddress });
  } else {
    throw new Error(`Unknown permit primaryType: ${primaryType}`);
  }
}

////////////////////////////////////////////////////////////////////////////////
// _validateDecryptionPermitEip712V1
////////////////////////////////////////////////////////////////////////////////

function _validateDecryptionPermitEip712V1(eip712: KmsUserDecryptEip712V1 | KmsDelegatedUserDecryptEip712V1): void {
  if (eip712.message.contractAddresses.length === 0) {
    throw Error('contractAddresses is empty');
  }

  if (eip712.message.contractAddresses.length > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
    throw Error(`contractAddresses max length of ${MAX_USER_DECRYPT_CONTRACT_ADDRESSES} exceeded`);
  }

  const durationDays = Number(eip712.message.durationDays);
  if (!isUintNumber(durationDays)) {
    throw Error(`durationDays is not a valid unsigned integer.`);
  }
  if (durationDays < 1) {
    throw Error(`durationDays must be at least 1 day, got ${durationDays}`);
  }
  if (durationDays > MAX_USER_DECRYPT_DURATION_DAYS) {
    throw Error(`durationDays is above max duration of ${MAX_USER_DECRYPT_DURATION_DAYS}`);
  }

  if (!isUintString(eip712.message.startTimestamp, MAX_UINT256)) {
    throw Error(`startTimestamp is not a valid Uint256`);
  }
}

////////////////////////////////////////////////////////////////////////////////
// _createSignedDecryptionPermitV1
////////////////////////////////////////////////////////////////////////////////

async function _createSignedDecryptionPermitV1(
  context: { readonly chain: FhevmChain; readonly runtime: FhevmRuntime },
  parameters: {
    readonly signerAddress: ChecksummedAddress;
    readonly eip712: KmsUserDecryptEip712V1 | KmsDelegatedUserDecryptEip712V1;
    readonly signature: Bytes65Hex;
  },
): Promise<SignedDecryptionPermit> {
  const { signerAddress, eip712, signature } = parameters;

  _validateDecryptionPermitEip712V1(eip712);

  if (eip712.primaryType === 'UserDecryptRequestVerification') {
    await verifyKmsUserDecryptEip712V1(context, {
      signer: signerAddress,
      message: eip712.message,
      signature,
    });
    return new SignedDecryptionPermitV1Impl(PRIVATE_TOKEN, parameters);
  } else {
    await verifyKmsDelegatedUserDecryptEip712V1(context, {
      signer: signerAddress,
      message: eip712.message,
      signature,
    });
    return new SignedDecryptionPermitV1Impl(PRIVATE_TOKEN, parameters);
  }
}

////////////////////////////////////////////////////////////////////////////////
// _assertKmsEip712V1DeadlineValidity
////////////////////////////////////////////////////////////////////////////////

function _assertKmsEip712V1DeadlineValidity(
  {
    startTimestamp,
    durationDays,
  }: {
    startTimestamp: bigint | number | string;
    durationDays: bigint | number | string;
  },
  maxDurationDays: UintNumber,
): void {
  const durationDaysBigInt = BigInt(durationDays);
  if (durationDaysBigInt === 0n) {
    throw Error('durationDays is zero');
  }
  if (durationDaysBigInt > BigInt(maxDurationDays)) {
    throw Error(`durationDays is above max duration of ${maxDurationDays}`);
  }

  const startTimestampBigInt = BigInt(startTimestamp);

  const currentTimestamp = BigInt(Math.floor(Date.now() / 1000));
  if (startTimestampBigInt > currentTimestamp) {
    throw Error('startTimestamp is set in the future');
  }

  const durationInSeconds = durationDaysBigInt * BigInt(24 * 60 * 60);
  if (startTimestampBigInt + durationInSeconds < currentTimestamp) {
    throw Error('request has expired');
  }
}
