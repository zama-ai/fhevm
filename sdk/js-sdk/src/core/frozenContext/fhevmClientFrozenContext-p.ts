import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { PubKeyCrsVersionResolution } from '../types/coreFhevmClient.js';
import type { FhevmClientFrozenContext, fhevmClientFrozenContextBrand } from '../types/fhevmClientFrozenContext-p.js';
import type { TfheVersion, TkmsVersion } from '../types/moduleVersions.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { CANONICAL_WASM_VERSIONS } from '../runtime/WasmVersions-p.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('FhevmClientFrozenContext.token');

////////////////////////////////////////////////////////////////////////////////

/**
 * The already-resolved versions used to build a {@link FhevmClientFrozenContext}.
 *
 * `pubKeyCrsVersion` is optional: a context only needs to carry the versions a
 * given operation actually consumes. There is no `tfheVersion` / `tkmsVersion` field — this SDK release
 * ships a single TFHE and a single TKMS module, so
 * {@link FhevmClientFrozenContext.tfheVersion} / `.tkmsVersion` always report
 * {@link CANONICAL_WASM_VERSIONS} regardless of what this context resolved.
 *
 * Values passed here are assumed to already have been read/derived consistently
 * (ideally in a single batched round-trip). This type performs no I/O.
 */
export type CreateFhevmClientFrozenContextParameters = {
  readonly pubKeyCrsVersion?: PubKeyCrsVersionResolution | undefined;
};

////////////////////////////////////////////////////////////////////////////////
// FhevmClientFrozenContextImpl
////////////////////////////////////////////////////////////////////////////////

/**
 * Private implementation of {@link FhevmClientFrozenContext}. Constructed only
 * via {@link createFhevmClientFrozenContext}, frozen on construction, and
 * verified elsewhere via `instanceof` (see {@link isFhevmClientFrozenContext}).
 *
 * @internal
 */
class FhevmClientFrozenContextImpl implements FhevmClientFrozenContext {
  declare readonly [fhevmClientFrozenContextBrand]: never;

  readonly #pubKeyCrsVersion: PubKeyCrsVersionResolution | undefined;

  constructor(privateToken: symbol, parameters: CreateFhevmClientFrozenContextParameters) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Use createFhevmClientFrozenContext() instead');
    }
    this.#pubKeyCrsVersion = parameters.pubKeyCrsVersion;

    Object.freeze(this);
  }

  //////////////////////////////////////////////////////////////////////////////
  // PubKey/CRS version
  //////////////////////////////////////////////////////////////////////////////

  public get hasPubKeyCrsVersion(): boolean {
    return this.#pubKeyCrsVersion !== undefined;
  }

  /** @throws If the PubKey/CRS version was not resolved in this context. */
  public get pubKeyCrsVersion(): PubKeyCrsVersionResolution {
    if (this.#pubKeyCrsVersion === undefined) {
      throw new Error('FhevmClientFrozenContext: pubKeyCrsVersion was not resolved in this context.');
    }
    return this.#pubKeyCrsVersion;
  }

  public get tryPubKeyCrsVersion(): PubKeyCrsVersionResolution | undefined {
    return this.#pubKeyCrsVersion;
  }

  //////////////////////////////////////////////////////////////////////////////
  // TFHE / TKMS module versions
  //////////////////////////////////////////////////////////////////////////////

  /**
   * This SDK release ships a single TFHE module, so this always returns
   * {@link CANONICAL_WASM_VERSIONS}.tfhe, regardless of what this context
   * resolved.
   */
  public get tfheVersion(): TfheVersion {
    return CANONICAL_WASM_VERSIONS.tfhe;
  }

  /**
   * This SDK release ships a single TKMS module, so this always returns
   * {@link CANONICAL_WASM_VERSIONS}.kms, regardless of what this context
   * resolved.
   */
  public get tkmsVersion(): TkmsVersion {
    return CANONICAL_WASM_VERSIONS.kms;
  }

  //////////////////////////////////////////////////////////////////////////////

  public toJSON(): Record<string, unknown> {
    return {
      pubKeyCrsVersion: this.#pubKeyCrsVersion,
      tfheVersion: this.tfheVersion,
      tkmsVersion: this.tkmsVersion,
    };
  }
}

// Prevent prototype pollution and external subclassing/instantiation tricks.
Object.freeze(FhevmClientFrozenContextImpl);
Object.freeze(FhevmClientFrozenContextImpl.prototype);

////////////////////////////////////////////////////////////////////////////////

/**
 * Builds an immutable {@link FhevmClientFrozenContext} from already-resolved
 * versions. Pure — performs no I/O. Pass only the versions the caller resolved.
 */
export function createFhevmClientFrozenContext(
  parameters: CreateFhevmClientFrozenContextParameters,
): FhevmClientFrozenContext {
  return new FhevmClientFrozenContextImpl(PRIVATE_TOKEN, parameters);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Returns a fully deep, independent copy of a {@link FhevmClientFrozenContext}.
 *
 * The clone shares **no references** with the source: every version-resolution
 * object is copied (these are flat records, so a spread is a complete copy;
 * version strings are immutable). Mutating any part of one context can never affect the
 * other. The result is a new frozen context, built through the same
 * {@link createFhevmClientFrozenContext} path.
 *
 * @throws {InvalidTypeError} If `value` is not a {@link FhevmClientFrozenContext}.
 */
export function cloneFhevmClientFrozenContext(value: FhevmClientFrozenContext): FhevmClientFrozenContext {
  assertIsFhevmClientFrozenContext(value, {});

  // toJSON() exposes the full internal state (the exact constructor-parameter shape).
  const state = value.toJSON() as {
    readonly pubKeyCrsVersion?: PubKeyCrsVersionResolution | undefined;
    readonly relayerSupportsV3UserDecryptRoute: boolean;
  };

  return createFhevmClientFrozenContext({
    pubKeyCrsVersion: state.pubKeyCrsVersion === undefined ? undefined : { ...state.pubKeyCrsVersion },
  });
}

////////////////////////////////////////////////////////////////////////////////

export function isFhevmClientFrozenContext(value: unknown): value is FhevmClientFrozenContext {
  return value instanceof FhevmClientFrozenContextImpl;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsFhevmClientFrozenContext(
  value: unknown,
  options: { readonly subject?: string } & ErrorMetadataParams,
): asserts value is FhevmClientFrozenContext {
  if (!isFhevmClientFrozenContext(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'FhevmClientFrozenContext',
      },
      options,
    );
  }
}
