import type { ChecksummedAddress, Uint8Number } from '../types/primitives.js';
import type { coprocessorBrand } from '../types/coprocessor.js';
import type { CoprocessorSignersContext, CoprocessorSignersContextJson } from '../types/coprocessorSignersContext.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { DuplicateSignerError, ThresholdSignerError, UnknownSignerError } from '../errors/SignersError.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { assertOwnedBy } from '../runtime/CoreFhevmRuntime-p.js';

const PRIVATE_TOKEN = Symbol('CoprocessorSignersContext.token');
const VERIFY_FUNC = Symbol('CoprocessorSignersContext.verify');

////////////////////////////////////////////////////////////////////////////////
// CoprocessorSignersContext (private implementation)
////////////////////////////////////////////////////////////////////////////////

class CoprocessorSignersContextImpl implements CoprocessorSignersContext {
  declare readonly [coprocessorBrand]: never;

  readonly #owner: WeakRef<FhevmRuntime>;
  readonly #address: ChecksummedAddress;
  readonly #coprocessorSigners: ChecksummedAddress[];
  readonly #coprocessorSignersSet: Set<string>;
  readonly #coprocessorSignerThreshold: Uint8Number;

  constructor(
    privateToken: symbol,
    owner: WeakRef<FhevmRuntime>,
    parameters: {
      readonly address: ChecksummedAddress;
      readonly coprocessorSigners: readonly ChecksummedAddress[];
      readonly coprocessorSignerThreshold: Uint8Number;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#owner = owner;
    this.#address = parameters.address;
    this.#coprocessorSigners = [...parameters.coprocessorSigners];
    this.#coprocessorSignerThreshold = parameters.coprocessorSignerThreshold;
    this.#coprocessorSignersSet = new Set(this.#coprocessorSigners.map((addr) => addr.toLowerCase()));

    Object.freeze(this.#coprocessorSigners);
    Object.freeze(this);
  }

  public get address(): ChecksummedAddress {
    return this.#address;
  }

  public get signers(): ChecksummedAddress[] {
    return this.#coprocessorSigners;
  }

  public get threshold(): Uint8Number {
    return this.#coprocessorSignerThreshold;
  }

  public has(signer: string): boolean {
    return this.#coprocessorSignersSet.has(signer);
  }

  public static [VERIFY_FUNC](instance: unknown, owner: FhevmRuntime): void {
    if (!(instance instanceof CoprocessorSignersContextImpl)) {
      throw new Error('Invalid CoprocessorSignersContext instance');
    }
    assertOwnedBy({
      actualOwner: instance.#owner,
      expectedOwner: owner,
      name: 'CoprocessorSignersContext',
    });
  }

  public toJSON(): CoprocessorSignersContextJson {
    return {
      address: this.#address,
      signers: this.#coprocessorSigners,
      threshold: this.#coprocessorSignerThreshold,
    };
  }
}

// Prevent prototype pollution and constructor access
Object.freeze(CoprocessorSignersContextImpl.prototype);
Object.freeze(CoprocessorSignersContextImpl);

////////////////////////////////////////////////////////////////////////////////

export function createCoprocessorSignersContext(
  owner: WeakRef<FhevmRuntime>,
  parameters: {
    readonly address: ChecksummedAddress;
    readonly coprocessorSigners: readonly ChecksummedAddress[];
    readonly coprocessorSignerThreshold: Uint8Number;
  },
): CoprocessorSignersContext {
  return new CoprocessorSignersContextImpl(PRIVATE_TOKEN, owner, parameters);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Verifies that the given `CoprocessorSignersContext` instance was created
 * by the given runtime. Throws if not.
 */
export function assertCoprocessorSignersContextOwnedBy(data: CoprocessorSignersContext, owner: FhevmRuntime): void {
  CoprocessorSignersContextImpl[VERIFY_FUNC](data, owner);
}

////////////////////////////////////////////////////////////////////////////////

export function assertCoprocessorSignerThreshold(
  coprocessorSignersContext: CoprocessorSignersContext,
  recoveredAddresses: readonly string[],
): void {
  const type = 'coprocessor';
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
    if (!coprocessorSignersContext.has(address.toLowerCase())) {
      throw new UnknownSignerError({
        unknownAddress: address,
        type,
      });
    }
  }

  if (recoveredAddresses.length < coprocessorSignersContext.threshold) {
    throw new ThresholdSignerError({
      type,
    });
  }
}

////////////////////////////////////////////////////////////////////////////////

export function isCoprocessorSignersContext(value: unknown): value is CoprocessorSignersContext {
  return value instanceof CoprocessorSignersContextImpl;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsCoprocessorSignersContext(
  value: unknown,
  options: { readonly subject?: string } & ErrorMetadataParams,
): asserts value is CoprocessorSignersContext {
  if (!isCoprocessorSignersContext(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'CoprocessorSignersContext',
      },
      options,
    );
  }
}
