import type { ChecksummedAddress, Uint8Number } from "../types/primitives.js";
import type { kmsBrand } from "../types/kms.js";
import { addressToChecksummedAddress } from "../base/address.js";
import {
  DuplicateSignerError,
  ThresholdSignerError,
  UnknownSignerError,
} from "../errors/SignersError.js";
import type { KmsSignersContext } from "../types/kmsSignersContext.js";
import { InvalidTypeError } from "../base/errors/InvalidTypeError.js";
import type { ErrorMetadataParams } from "../base/errors/ErrorBase.js";
import type { FhevmRuntime } from "../types/coreFhevmRuntime.js";
import { assertOwnedBy } from "../runtime/CoreFhevmRuntime-p.js";

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol("KmsSignersContext.token");
const VERIFY_FUNC = Symbol("KmsSignersContext.verify");

////////////////////////////////////////////////////////////////////////////////
// KmsVerifierContractData (private implementation)
////////////////////////////////////////////////////////////////////////////////

class KmsSignersContextImpl implements KmsSignersContext {
  declare readonly [kmsBrand]: never;

  readonly #owner: WeakRef<FhevmRuntime>;
  readonly #address: ChecksummedAddress;
  readonly #kmsSigners: ChecksummedAddress[];
  readonly #kmsSignersSet: Set<string>;
  readonly #kmsSignerThreshold: Uint8Number;

  constructor(
    privateToken: symbol,
    owner: WeakRef<FhevmRuntime>,
    parameters: {
      readonly address: ChecksummedAddress;
      readonly kmsSigners: ChecksummedAddress[];
      readonly kmsSignerThreshold: Uint8Number;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error("Unauthorized");
    }
    this.#owner = owner;
    this.#address = parameters.address;
    this.#kmsSigners = [...parameters.kmsSigners];
    this.#kmsSignerThreshold = parameters.kmsSignerThreshold;
    this.#kmsSignersSet = new Set(
      this.#kmsSigners.map((addr) => addr.toLowerCase()),
    );

    Object.freeze(this.#kmsSigners);
    Object.freeze(this);
  }

  public get address(): ChecksummedAddress {
    return this.#address;
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
      throw new Error("Invalid KmsSignersContext instance");
    }
    assertOwnedBy({
      actualOwner: instance.#owner,
      expectedOwner: owner,
      name: "KmsSignersContext",
    });
  }

  public toJson(): Record<string, unknown> {
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
    readonly kmsSigners: ChecksummedAddress[];
    readonly kmsSignerThreshold: Uint8Number;
  },
): KmsSignersContext {
  const { address, kmsSigners, kmsSignerThreshold } = parameters;

  return new KmsSignersContextImpl(PRIVATE_TOKEN, owner, {
    address: addressToChecksummedAddress(address),
    kmsSignerThreshold: Number(kmsSignerThreshold) as Uint8Number,
    kmsSigners: kmsSigners.map(addressToChecksummedAddress),
  });
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Verifies that the given `KmsSignersContext` instance is owned
 * by the given runtime. Throws if not.
 */
export function assertKmsSignersContextOwnedBy(
  data: KmsSignersContext,
  owner: FhevmRuntime,
): void {
  KmsSignersContextImpl[VERIFY_FUNC](data, owner);
}

////////////////////////////////////////////////////////////////////////////////

export function assertKmsSignerThreshold(
  kmsSignersContext: KmsSignersContext,
  recoveredAddresses: readonly string[],
): void {
  const type = "kms";
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

export function isKmsSignersContext(
  value: unknown,
): value is KmsSignersContext {
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
        expectedType: "KmsSignersContext",
      },
      options,
    );
  }
}
