import type { SignedDecryptionPermit, SignedDecryptionPermitV2 } from '../types/signedDecryptionPermit.js';
import type { KmsUserDecryptEip712V2 } from '../types/kms.js';
import type { BytesHex, ChecksummedAddress, Uint256BigInt, Uint8Number } from '../types/primitives.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { SignDecryptionPermitContext, SignDecryptionPermitParameters } from './SignedDecryptionPermit-p.js';
import { assertRecordNonNullableProperty } from '../base/record.js';
import { assertRecordBytesHexProperty } from '../base/bytes.js';
import { addressToChecksummedAddress, assertIsAddress, assertRecordAddressProperty } from '../base/address.js';
import { assertRecordStringProperty } from '../base/string.js';
import { assertIsTransportKeyPair, type TransportKeyPair } from './TransportKeyPair-p.js';
import { readKmsSignersContext } from '../host-contracts/readKmsSignersContext-p.js';
import { kmsSignersContextToExtraData } from '../host-contracts/KmsSignersContext-p.js';
import { EXTRA_DATA_V2, fromKmsExtraData } from './kmsExtraData.js';
import { assertIsKmsUserDecryptEip712V2, createKmsUserDecryptEip712V2 } from './createKmsUserDecryptEip712V2.js';
import { verifyErc1271UserDecrypt } from '../utils-p/decrypt/verifyErc1271UserDecrypt-p.js';
import { assert } from '../base/errors/InternalError.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('SignedDecryptionPermitV2.token');
const MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10 as Uint8Number;

////////////////////////////////////////////////////////////////////////////////

function assertKmsEip712V2DeadlineValidity({
  startTimestamp,
  durationSeconds,
}: {
  startTimestamp: bigint | number | string;
  durationSeconds: bigint | number | string;
}): void {
  if (durationSeconds === 0) {
    throw Error('durationSeconds is zero');
  }

  const durationSecBigInt = BigInt(durationSeconds);
  const startTimestampBigInt = BigInt(startTimestamp);

  const currentTimestamp = BigInt(Math.floor(Date.now() / 1000));
  if (startTimestampBigInt > currentTimestamp) {
    throw Error('startTimestamp is set in the future');
  }

  if (startTimestampBigInt + durationSecBigInt < currentTimestamp) {
    throw Error('request has expired');
  }
}

////////////////////////////////////////////////////////////////////////////////
// SignedDecryptionPermitV2Impl
////////////////////////////////////////////////////////////////////////////////

class SignedDecryptionPermitV2Impl implements SignedDecryptionPermitV2 {
  readonly #eip712: KmsUserDecryptEip712V2;
  readonly #signature: BytesHex;
  readonly #signerAddress: ChecksummedAddress;
  readonly #delegatorAddress: ChecksummedAddress | undefined;

  constructor(
    privateToken: symbol,
    parameters: {
      readonly eip712: KmsUserDecryptEip712V2;
      readonly signature: BytesHex;
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

  public get signature(): BytesHex {
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
    assertKmsEip712V2DeadlineValidity({ startTimestamp, durationSeconds });
  }

  public assertMatchesKmsContext(kmsSignersContext: KmsSignersContext): void {
    const expectedExtraData = kmsSignersContextToExtraData(kmsSignersContext);
    if (expectedExtraData !== this.#eip712.message.extraData) {
      throw new Error(
        `Invalid permit: extraData "${this.#eip712.message.extraData}" does not match expected "${expectedExtraData}" from KmsSignersContext.`,
      );
    }
  }

  public get kmsContextId(): Uint256BigInt {
    return fromKmsExtraData(this.#eip712.message.extraData).kmsContextId;
  }

  public get kmsEpochId(): Uint256BigInt {
    return fromKmsExtraData(this.#eip712.message.extraData).kmsEpochId;
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
// _createSignedDecryptionPermitV2
////////////////////////////////////////////////////////////////////////////////

async function _createSignedDecryptionPermitV2(
  context: SignDecryptionPermitContext,
  parameters: {
    readonly signerAddress: ChecksummedAddress;
    readonly eip712: KmsUserDecryptEip712V2;
    readonly signature: BytesHex;
    readonly delegatorAddress?: ChecksummedAddress | undefined;
  },
): Promise<SignedDecryptionPermitV2> {
  const { eip712, signature } = parameters;

  if (eip712.message.allowedContracts.length > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
    throw Error(`allowedContracts max length of ${MAX_USER_DECRYPT_CONTRACT_ADDRESSES} exceeded`);
  }

  // Precautionary local check against `eip712.message.userAddress`; auto-detects
  // EOA vs ERC-1271 (a normal EOA permit stays on the no-RPC fast path). The KMS
  // remains authoritative — see `verifyErc1271UserDecrypt` for the full contract.
  await verifyErc1271UserDecrypt(context, {
    userAddress: eip712.message.userAddress,
    eip712,
    signature,
  });

  return new SignedDecryptionPermitV2Impl(PRIVATE_TOKEN, parameters);
}

////////////////////////////////////////////////////////////////////////////////
// signDecryptionPermitV2
////////////////////////////////////////////////////////////////////////////////

export async function signDecryptionPermitV2(
  context: SignDecryptionPermitContext,
  parameters: SignDecryptionPermitParameters,
): Promise<SignedDecryptionPermit> {
  const kmsSignersContext = await readKmsSignersContext(context, {
    kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
  });

  const extraData: BytesHex = kmsSignersContextToExtraData(kmsSignersContext);

  // For debug purpose only:
  // -----------------------
  // In theory, it should not be possible to produce a unified eip712 (protocol v14+)
  // with an extraData coming from an old protocol v11/12/13
  assert(
    fromKmsExtraData(extraData).version >= EXTRA_DATA_V2,
    `signDecryptionPermitV2 error: Invalid extraData version extraData=${extraData}`,
  );

  const {
    contractAddresses,
    startTimestamp,
    durationSeconds: durationSecondsParam,
    signerAddress: signerAddressArg,
    transportKeyPair: transportKeyPair,
    signer,
    delegatorAddress: delegatorAddressArg,
  } = parameters;

  if (durationSecondsParam <= 0) {
    throw new RangeError(`durationSeconds must be positive, got ${durationSecondsParam}`);
  }

  const MAX_DURATION_SECONDS = 604_800n; // 7 days in seconds
  const durationSeconds = BigInt(durationSecondsParam);

  // RFC-016: warn when permissive mode (allowedContracts=[]) is combined with a long-lived permit —
  // a stolen signature can decrypt anything the user owns until it expires or is invalidated.
  if (contractAddresses.length === 0 && durationSeconds > MAX_DURATION_SECONDS) {
    const msg = `permissive mode (allowedContracts=[]) with durationSeconds ${durationSeconds} exceeds the recommended maximum of ${MAX_DURATION_SECONDS}s — consider using a shorter window or app-bounded allowedContracts`;
    const logger = context.runtime.config.logger;
    logger?.warn?.(msg);
  }

  assertIsTransportKeyPair(transportKeyPair, {});
  assertIsAddress(signerAddressArg, {});

  const signerAddress = addressToChecksummedAddress(signerAddressArg);

  let delegatorAddress: ChecksummedAddress | undefined;
  if (delegatorAddressArg !== undefined) {
    assertIsAddress(delegatorAddressArg, {});
    delegatorAddress = addressToChecksummedAddress(delegatorAddressArg);
  }

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
    extraData,
    publicKey: transportKeyPair.publicKey,
  };

  const eip712 = createKmsUserDecryptEip712V2(commonMessage);

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
  context: SignDecryptionPermitContext,
  transportKeyPair: TransportKeyPair,
  permit: unknown,
): Promise<SignedDecryptionPermitV2> {
  assertIsTransportKeyPair(transportKeyPair, {});

  const permitName = 'permit';
  const options = {};

  assertRecordNonNullableProperty(permit, 'eip712', permitName, options);

  // Accept a variable-length signature: a 65-byte EOA signature, a concatenated
  // multisig ERC-1271 blob, or the empty `0x` pre-approved-hash flow. The verify
  // step below auto-detects EOA vs ERC-1271 against `eip712.message.userAddress`.
  assertRecordBytesHexProperty(permit, 'signature', permitName, options);
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

  return await _createSignedDecryptionPermitV2(context, {
    signature: permit.signature,
    eip712,
    signerAddress,
  });
}
