import type { SignedDecryptionPermit, SignedDecryptionPermitV2 } from '../types/signedDecryptionPermit.js';
import type { KmsUserDecryptEip712V2 } from '../types/kms.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress, Uint8Number } from '../types/primitives.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsSignDecryptionPermitContext, KmsSignDecryptionPermitParameters } from './SignedDecryptionPermit-p.js';
import type { KmsExtraData } from '../types/kms-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { assertRecordNonNullableProperty } from '../base/record.js';
import { assertRecordBytes65HexProperty } from '../base/bytes.js';
import { addressToChecksummedAddress, assertIsAddress, assertRecordAddressProperty } from '../base/address.js';
import { assertRecordStringProperty } from '../base/string.js';
import { assertIsTransportKeyPair, type TransportKeyPair } from './TransportKeyPair-p.js';
import { readCurrentKmsSignersContext } from '../host-contracts/readKmsSignersContext-p.js';
import { kmsSignersContextToExtraData } from '../host-contracts/KmsSignersContext-p.js';
import { assertIsKmsUserDecryptEip712V2, createKmsUserDecryptEip712V2 } from './createKmsUserDecryptEip712V2.js';
import { verifyKmsUserDecryptEip712V2 } from '../utils-p/decrypt/verifyKmsUserDecryptEip712V2.js';
import { assert } from '../base/errors/InternalError.js';
import { EXTRA_DATA_V2 } from './kmsExtraData-p.js';
import { assertIsUintNumber } from '../base/uint.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('SignedDecryptionPermitV2.token');
const MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10 as Uint8Number;

////////////////////////////////////////////////////////////////////////////////
// SignedDecryptionPermitV2Impl
////////////////////////////////////////////////////////////////////////////////

class SignedDecryptionPermitV2Impl implements SignedDecryptionPermitV2 {
  readonly #eip712: KmsUserDecryptEip712V2;
  readonly #signature: Bytes65Hex;
  readonly #signerAddress: ChecksummedAddress;
  readonly #delegatorAddress: ChecksummedAddress | undefined;

  constructor(
    privateToken: symbol,
    parameters: {
      readonly eip712: KmsUserDecryptEip712V2;
      readonly signature: Bytes65Hex;
      readonly signerAddress: ChecksummedAddress;
      readonly delegatorAddress?: ChecksummedAddress | undefined;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#eip712 = parameters.eip712;
    this.#signature = parameters.signature;
    this.#signerAddress = parameters.signerAddress;
    this.#delegatorAddress = parameters.delegatorAddress;
  }

  public get eip712(): KmsUserDecryptEip712V2 {
    return this.#eip712;
  }

  public get signature(): Bytes65Hex {
    return this.#signature;
  }

  public get signerAddress(): ChecksummedAddress {
    return this.#signerAddress;
  }

  public readonly version = 2 as const;

  public get encryptedDataOwnerAddress(): ChecksummedAddress {
    return this.#delegatorAddress ?? this.#signerAddress;
  }

  public get isDelegated(): boolean {
    return this.#delegatorAddress !== undefined;
  }

  public get transportPublicKey(): BytesHex {
    return this.#eip712.message.publicKey;
  }

  public assertNotExpired(): void {
    const { startTimestamp, durationSeconds } = this.#eip712.message;
    _assertKmsEip712V2DeadlineValidity({ startTimestamp, durationSeconds });
  }
}

////////////////////////////////////////////////////////////////////////////////

Object.freeze(SignedDecryptionPermitV2Impl);
Object.freeze(SignedDecryptionPermitV2Impl.prototype);

////////////////////////////////////////////////////////////////////////////////

export function isSignedDecryptionPermitV2(value: unknown): value is SignedDecryptionPermitV2 {
  return value instanceof SignedDecryptionPermitV2Impl;
}

////////////////////////////////////////////////////////////////////////////////
// signDecryptionPermitV2
////////////////////////////////////////////////////////////////////////////////

export async function signDecryptionPermitV2(
  context: KmsSignDecryptionPermitContext,
  parameters: KmsSignDecryptionPermitParameters,
): Promise<SignedDecryptionPermit> {
  const { signerAddress: signerAddressArg, signer, delegatorAddress: delegatorAddressArg } = parameters;

  assertIsAddress(signerAddressArg, {});
  const signerAddress = addressToChecksummedAddress(signerAddressArg);

  // Delegation is post-sign metadata (it is not part of the signed V2 message);
  // checksum it here for the resulting SignedDecryptionPermitV2 instance.
  let delegatorAddress: ChecksummedAddress | undefined;
  if (delegatorAddressArg !== undefined) {
    assertIsAddress(delegatorAddressArg, {});
    delegatorAddress = addressToChecksummedAddress(delegatorAddressArg);
  }

  // All message construction (KMS context read + extraData version assert, duration
  // and permissive-mode validation, startTimestamp rounding) lives in the unsigned
  // builder, so the locally-signed and externally-signed paths stay identical.
  const eip712 = await createUnsignedDecryptionPermitEip712V2(context, parameters);

  const signature = await context.runtime.ethereum.signTypedData(signer, {
    account: signerAddress,
    ...eip712,
  });

  return await _createSignedDecryptionPermitV2(context, {
    signature,
    signerAddress,
    eip712,
    delegatorAddress,
  });
}

////////////////////////////////////////////////////////////////////////////////
// parseSignedDecryptionPermitV2
////////////////////////////////////////////////////////////////////////////////

export async function parseSignedDecryptionPermitV2(
  context: KmsSignDecryptionPermitContext,
  parameters: {
    readonly transportKeyPair: TransportKeyPair;
    readonly permit: unknown;
    readonly fhevmContext: FhevmClientFrozenContext;
  },
): Promise<SignedDecryptionPermitV2> {
  const { transportKeyPair, permit } = parameters;
  assertIsTransportKeyPair(transportKeyPair, {});

  const permitName = 'permit-v2';
  const options = {};

  assertRecordNonNullableProperty(permit, 'eip712', permitName, options);
  assertRecordBytes65HexProperty(permit, 'signature', permitName, options);
  assertRecordAddressProperty(permit, 'signerAddress', permitName, options);

  const eip712 = permit.eip712;
  assertRecordStringProperty(eip712, 'primaryType', `${permitName}.eip712`, options);
  const primaryType = (eip712 as Record<string, unknown>).primaryType;

  if (primaryType !== 'UserDecryptRequestVerification') {
    throw new Error(`Expected primaryType 'UserDecryptRequestVerification' for V2 permit, got: ${primaryType}`);
  }

  assertIsKmsUserDecryptEip712V2(eip712, `${permitName}.eip712`, options);

  const signerAddress = addressToChecksummedAddress(permit.signerAddress);

  if (eip712.message.publicKey.toLowerCase() !== transportKeyPair.publicKey.toLowerCase()) {
    throw new Error(
      "The permit's publicKey does not match the TransportKeyPair's publicKey. " +
        'Ensure the permit was signed with the same key pair.',
    );
  }

  return await _createSignedDecryptionPermitV2(context, { signature: permit.signature, eip712, signerAddress });
}

////////////////////////////////////////////////////////////////////////////////
// createUnsignedDecryptionPermitEip712V2
////////////////////////////////////////////////////////////////////////////////

export async function createUnsignedDecryptionPermitEip712V2(
  context: KmsSignDecryptionPermitContext,
  // Unlike the V1 unsigned builder, only `signer` is omitted: the V2 (unified)
  // permit embeds `userAddress` in the signed message, so the signer's address
  // is required to build it (the V1 message never carries the signer address).
  parameters: Omit<KmsSignDecryptionPermitParameters, 'signer'>,
): Promise<KmsUserDecryptEip712V2> {
  const {
    contractAddresses,
    startTimestamp,
    durationSeconds: durationSecondsParam,
    signerAddress: signerAddressArg,
    transportKeyPair,
    delegatorAddress: delegatorAddressArg,
    fhevmContext,
  } = parameters;

  assertIsUintNumber(durationSecondsParam, { subject: 'durationSeconds' });

  if (durationSecondsParam <= 0) {
    throw new RangeError(`durationSeconds must be positive, got ${durationSecondsParam}`);
  }

  const MAX_DURATION_SECONDS = 604_800n; // 7 days in seconds
  const durationSeconds = BigInt(durationSecondsParam);

  // RFC-016: warn when permissive mode (allowedContracts=[]) is combined with a long-lived permit —
  // a stolen signature can decrypt anything the user owns until it expires or is invalidated.
  if (contractAddresses.length === 0 && durationSeconds > MAX_DURATION_SECONDS) {
    const msg = `permissive mode (allowedContracts=[]) with durationSeconds ${durationSeconds} exceeds the recommended maximum of ${MAX_DURATION_SECONDS}s — consider using a shorter window or app-bounded allowedContracts`;
    context.runtime.config.logger?.warn?.(msg);
  }

  // Validate before returning so an externally-signed permit can never be built
  // over out-of-range data (mirrors the check in `_createSignedDecryptionPermitV2`).
  if (contractAddresses.length > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
    throw Error(`allowedContracts max length of ${MAX_USER_DECRYPT_CONTRACT_ADDRESSES} exceeded`);
  }

  assertIsTransportKeyPair(transportKeyPair, {});
  assertIsAddress(signerAddressArg, {});
  const signerAddress = addressToChecksummedAddress(signerAddressArg);

  // Delegation is not part of the signed V2 message (it is post-sign metadata on
  // the SignedDecryptionPermitV2), so it does not affect this eip712 — but a
  // provided delegator is validated early to fail fast, mirroring V1.
  if (delegatorAddressArg !== undefined) {
    assertIsAddress(delegatorAddressArg, {});
  }

  const kmsSignersContext = await readCurrentKmsSignersContext(context, {
    kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
    fhevmContext,
  });

  const kmsContextExtraData: KmsExtraData = kmsSignersContextToExtraData(kmsSignersContext);

  // A unified (v2) permit requires protocol v14+ extraData (context id + epoch id).
  assert(
    kmsContextExtraData.ge(EXTRA_DATA_V2),
    `createUnsignedDecryptionPermitEip712V2 error: Invalid extraData version extraData=${kmsContextExtraData.bytesHex}`,
  );

  // RFC-016: round startTimestamp down to the nearest minute. This absorbs small clock skew
  // and prevents a future-dated startTimestamp from bypassing signature invalidation
  // (a future start would survive an invalidateDecryptionSignaturesBefore(block.timestamp) call).
  const roundedStartTimestamp = Math.floor(startTimestamp / 60) * 60;

  const commonMessage = {
    verifyingContractAddressDecryption: context.chain.fhevm.gateway.contracts.decryption.address as ChecksummedAddress,
    chainId: context.chain.id,
    userAddress: signerAddress, // identity asserting authorization
    allowedContracts: contractAddresses, // [] = permissive, [...] = specific
    durationSeconds,
    startTimestamp: roundedStartTimestamp,
    extraData: kmsContextExtraData,
    publicKey: transportKeyPair.publicKey,
  };

  const eip712 = createKmsUserDecryptEip712V2(commonMessage);

  // no need to validate as it has already been validated

  return eip712;
}

////////////////////////////////////////////////////////////////////////////////
// _createSignedDecryptionPermitV2
////////////////////////////////////////////////////////////////////////////////

async function _createSignedDecryptionPermitV2(
  context: { readonly chain: FhevmChain; readonly runtime: FhevmRuntime },
  parameters: {
    readonly signerAddress: ChecksummedAddress;
    readonly eip712: KmsUserDecryptEip712V2;
    readonly signature: Bytes65Hex;
    readonly delegatorAddress?: ChecksummedAddress | undefined;
  },
): Promise<SignedDecryptionPermitV2> {
  const { signerAddress, eip712, signature } = parameters;

  // Enforced here (the choke point shared by signDecryptionPermitV2 and
  // parseSignedDecryptionPermitV2). The message field is a validated uint256
  // string, so `BigInt` is safe; a uint can only be non-positive when it is 0.
  if (BigInt(eip712.message.durationSeconds) <= 0n) {
    throw new RangeError(`durationSeconds must be positive, got ${eip712.message.durationSeconds}`);
  }

  if (eip712.message.allowedContracts.length > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
    throw Error(`allowedContracts max length of ${MAX_USER_DECRYPT_CONTRACT_ADDRESSES} exceeded`);
  }

  await verifyKmsUserDecryptEip712V2(context, { signer: signerAddress, message: eip712.message, signature });

  return new SignedDecryptionPermitV2Impl(PRIVATE_TOKEN, parameters);
}

////////////////////////////////////////////////////////////////////////////////
// _assertKmsEip712V2DeadlineValidity
////////////////////////////////////////////////////////////////////////////////

function _assertKmsEip712V2DeadlineValidity({
  startTimestamp,
  durationSeconds,
}: {
  startTimestamp: bigint | number | string;
  durationSeconds: bigint | number | string;
}): void {
  const durationSecBigInt = BigInt(durationSeconds);
  if (durationSecBigInt === 0n) {
    throw Error('durationSeconds is zero');
  }

  const startTimestampBigInt = BigInt(startTimestamp);

  const currentTimestamp = BigInt(Math.floor(Date.now() / 1000));
  if (startTimestampBigInt > currentTimestamp) {
    throw Error('startTimestamp is set in the future');
  }

  if (startTimestampBigInt + durationSecBigInt < currentTimestamp) {
    throw Error('request has expired');
  }
}
