import type { BytesHex, ChecksummedAddress, Uint256BigInt, Uint8Number } from '../types/primitives.js';
import type { kmsBrand } from '../types/kms.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { addressToChecksummedAddress } from '../base/address.js';
import { DuplicateSignerError, ThresholdSignerError, UnknownSignerError } from '../errors/SignersError.js';
import { assertOwnedBy } from '../runtime/CoreFhevmRuntime-p.js';
import { assertIsKmsExtraData, toKmsExtraData } from '../kms/kmsExtraData.js';
import { assertIsNonEmptyString, ensure0x } from '../base/string.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('KmsSignersContext.token');
const VERIFY_FUNC = Symbol('KmsSignersContext.verify');

////////////////////////////////////////////////////////////////////////////////
// KmsVerifierContractData (private implementation)
////////////////////////////////////////////////////////////////////////////////

class KmsSignersContextImpl implements KmsSignersContext {
  declare readonly [kmsBrand]: never;

  readonly #owner: WeakRef<FhevmRuntime>;
  readonly #address: ChecksummedAddress;
  readonly #kmsContextId: Uint256BigInt;
  readonly #kmsSigners: ChecksummedAddress[];
  readonly #kmsSignersSet: Set<string>;
  readonly #kmsSignerThreshold: Uint8Number;

  constructor(
    privateToken: symbol,
    owner: WeakRef<FhevmRuntime>,
    parameters: {
      readonly address: ChecksummedAddress;
      readonly kmsContextId: Uint256BigInt;
      readonly kmsSigners: ChecksummedAddress[];
      readonly kmsSignerThreshold: Uint8Number;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#owner = owner;
    this.#address = parameters.address;
    this.#kmsContextId = parameters.kmsContextId;
    this.#kmsSigners = [...parameters.kmsSigners];
    this.#kmsSignerThreshold = parameters.kmsSignerThreshold;
    this.#kmsSignersSet = new Set(this.#kmsSigners.map((addr) => addr.toLowerCase()));

    Object.freeze(this.#kmsSigners);
    Object.freeze(this);
  }

  public get address(): ChecksummedAddress {
    return this.#address;
  }

  public get id(): Uint256BigInt {
    return this.#kmsContextId;
  }

  public get signers(): ChecksummedAddress[] {
    return this.#kmsSigners;
  }

  public get threshold(): Uint8Number {
    return this.#kmsSignerThreshold;
  }

  public has(signer: string): boolean {
    return this.#kmsSignersSet.has(signer);
  }

  public static [VERIFY_FUNC](instance: unknown, owner: FhevmRuntime): void {
    if (!(instance instanceof KmsSignersContextImpl)) {
      throw new Error('Invalid KmsSignersContext instance');
    }
    assertOwnedBy({
      actualOwner: instance.#owner,
      expectedOwner: owner,
      name: 'KmsSignersContext',
    });
  }

  public toJSON(): Record<string, unknown> {
    return {
      address: this.#address,
      signers: this.#kmsSigners,
      threshold: this.#kmsSignerThreshold,
    };
  }
}

// Prevent prototype pollution and constructor access
Object.freeze(KmsSignersContextImpl.prototype);
Object.freeze(KmsSignersContextImpl);

////////////////////////////////////////////////////////////////////////////////

export function createKmsSignersContext(
  owner: WeakRef<FhevmRuntime>,
  parameters: {
    readonly address: ChecksummedAddress;
    readonly kmsContextId: Uint256BigInt;
    readonly kmsSigners: readonly ChecksummedAddress[];
    readonly kmsSignerThreshold: Uint8Number;
  },
): KmsSignersContext {
  const { address, kmsContextId, kmsSigners, kmsSignerThreshold } = parameters;
  return new KmsSignersContextImpl(PRIVATE_TOKEN, owner, {
    address: addressToChecksummedAddress(address),
    kmsContextId,
    kmsSignerThreshold: Number(kmsSignerThreshold) as Uint8Number,
    kmsSigners: kmsSigners.map(addressToChecksummedAddress),
  });
}

////////////////////////////////////////////////////////////////////////////////

export function kmsSignersContextToExtraData(kmsSignersContext: KmsSignersContext): BytesHex {
  assertIsKmsSignersContext(kmsSignersContext, {});
  if (kmsSignersContext.id === 0n) {
    return '0x00' as BytesHex;
  }
  return toKmsExtraData({
    version: 1 as Uint8Number,
    kmsContextId: kmsSignersContext.id,
  });
}

/**
 * Asserts that the given `extraData` is a valid KMS extra data string and
 * matches the `extraData` derived from the provided {@link KmsSignersContext}.
 *
 * @throws If `extraData` is not a non-empty string, not a valid KMS extra data,
 *   or does not match the expected value from the context.
 */
export function assertExtraDataMatchesKmsSingersContext(
  parameters: {
    readonly extraData: unknown;
    readonly kmsSignersContext: KmsSignersContext;
  },
  options: { subject?: string } & ErrorMetadataParams,
): void {
  const { extraData, kmsSignersContext } = parameters;
  assertIsNonEmptyString(extraData);

  const sanitizedExtraData = ensure0x(extraData);
  assertIsKmsExtraData(sanitizedExtraData, options);

  const expectedExtraData = kmsSignersContextToExtraData(kmsSignersContext);

  if (sanitizedExtraData !== expectedExtraData) {
    throw new Error(`extraData "${extraData}" does not match KmsSignersContext extraData "${expectedExtraData}".`);
  }
}

export function extraDataMatchesKmsSingersContext(parameters: {
  readonly extraData: unknown;
  readonly kmsSignersContext: KmsSignersContext;
}): boolean {
  try {
    assertExtraDataMatchesKmsSingersContext(parameters, {});
    return true;
  } catch {
    return false;
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Verifies that the given `KmsSignersContext` instance is owned
 * by the given runtime. Throws if not.
 */
export function assertKmsSignersContextOwnedBy(data: KmsSignersContext, owner: FhevmRuntime): void {
  KmsSignersContextImpl[VERIFY_FUNC](data, owner);
}

////////////////////////////////////////////////////////////////////////////////

export function assertKmsSignerThreshold(
  kmsSignersContext: KmsSignersContext,
  recoveredAddresses: readonly string[],
): void {
  const type = 'kms';
  const addressMap = new Set<string>();
  recoveredAddresses.forEach((address) => {
    if (addressMap.has(address.toLowerCase())) {
      throw new DuplicateSignerError({
        duplicateAddress: address,
        type,
      });
    }
    addressMap.add(address.toLowerCase());
  });

  for (const address of recoveredAddresses) {
    if (!kmsSignersContext.has(address.toLowerCase())) {
      throw new UnknownSignerError({
        unknownAddress: address,
        type,
      });
    }
  }

  if (recoveredAddresses.length < kmsSignersContext.threshold) {
    throw new ThresholdSignerError({
      type,
    });
  }
}

////////////////////////////////////////////////////////////////////////////////

export function isKmsSignersContext(value: unknown): value is KmsSignersContext {
  return value instanceof KmsSignersContextImpl;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsKmsSignersContext(
  value: unknown,
  options: { readonly subject?: string } & ErrorMetadataParams,
): asserts value is KmsSignersContext {
  if (!isKmsSignersContext(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'KmsSignersContext',
      },
      options,
    );
  }
}
