import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { HostContractName, HostContractVersion } from '../types/hostContract.js';
import type {
  FhevmProtocolContext,
  ProtocolVersionResolution,
  PubKeyCrsVersionResolution,
} from '../types/coreFhevmClient.js';
import type { TfheVersion, TkmsVersion } from '../types/moduleVersions.js';
import type { FhevmClientFrozenContext, fhevmClientFrozenContextBrand } from '../types/fhevmClientFrozenContext-p.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('FhevmClientFrozenContext.token');

////////////////////////////////////////////////////////////////////////////////

/**
 * The already-resolved versions used to build a {@link FhevmClientFrozenContext}.
 *
 * Every field is optional: a context only needs to carry the versions a given
 * operation actually consumes — an encrypt path resolves `tfheVersion` (and the
 * `protocol`/`ACL` versions it derives from) but never `tkmsVersion` or the
 * KMSVerifier version; a public-decrypt path is the opposite. Resolving only the
 * needed subset keeps the number of on-chain `getVersion()` reads minimal.
 *
 * Values passed here are assumed to already have been read/derived consistently
 * (ideally in a single batched round-trip). This type performs no I/O.
 */
export type CreateFhevmClientFrozenContextParameters = {
  readonly hostContractVersions?: Partial<Record<HostContractName, HostContractVersion>> | undefined;
  readonly protocolVersion?: ProtocolVersionResolution | undefined;
  readonly pubKeyCrsVersion?: PubKeyCrsVersionResolution | undefined;
  readonly tfheVersion?: TfheVersion | undefined;
  readonly tkmsVersion?: TkmsVersion | undefined;
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

  readonly #hostContractVersions: Readonly<Partial<Record<HostContractName, HostContractVersion>>>;
  readonly #protocolVersion: ProtocolVersionResolution | undefined;
  readonly #pubKeyCrsVersion: PubKeyCrsVersionResolution | undefined;
  readonly #tfheVersion: TfheVersion | undefined;
  readonly #tkmsVersion: TkmsVersion | undefined;

  constructor(privateToken: symbol, parameters: CreateFhevmClientFrozenContextParameters) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Use createFhevmClientFrozenContext() instead');
    }
    // Defensive copy so a later mutation of the caller's object can't alter the snapshot.
    this.#hostContractVersions = Object.freeze({ ...(parameters.hostContractVersions ?? {}) });
    this.#protocolVersion = parameters.protocolVersion;
    this.#pubKeyCrsVersion = parameters.pubKeyCrsVersion;
    this.#tfheVersion = parameters.tfheVersion;
    this.#tkmsVersion = parameters.tkmsVersion;

    Object.freeze(this);
  }

  //////////////////////////////////////////////////////////////////////////////
  // Host contract versions (raw on-chain getVersion() results)
  //////////////////////////////////////////////////////////////////////////////

  public hasHostContractVersion(name: HostContractName): boolean {
    return this.#hostContractVersions[name] !== undefined;
  }

  /** @throws If `name`'s version was not resolved in this context. */
  public hostContractVersion<name extends HostContractName>(name: name): HostContractVersion<name> {
    const version = this.#hostContractVersions[name];
    if (version === undefined) {
      throw new Error(`FhevmClientFrozenContext: host contract version '${name}' was not resolved in this context.`);
    }
    return version as HostContractVersion<name>;
  }

  public tryHostContractVersion<name extends HostContractName>(name: name): HostContractVersion<name> | undefined {
    return this.#hostContractVersions[name] as HostContractVersion<name> | undefined;
  }

  //////////////////////////////////////////////////////////////////////////////
  // Protocol version
  //////////////////////////////////////////////////////////////////////////////

  public get hasProtocolVersion(): boolean {
    return this.#protocolVersion !== undefined;
  }

  /** @throws If the protocol version was not resolved in this context. */
  public get protocolVersion(): ProtocolVersionResolution {
    if (this.#protocolVersion === undefined) {
      throw new Error('FhevmClientFrozenContext: protocolVersion was not resolved in this context.');
    }
    return this.#protocolVersion;
  }

  public get tryProtocolVersion(): ProtocolVersionResolution | undefined {
    return this.#protocolVersion;
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
  // Protocol context (protocol + PubKey/CRS bundle)
  //////////////////////////////////////////////////////////////////////////////

  public get hasProtocolContext(): boolean {
    return this.#protocolVersion !== undefined && this.#pubKeyCrsVersion !== undefined;
  }

  /**
   * The {@link FhevmProtocolContext} bundle (protocol + PubKey/CRS versions),
   * as consumed by the WASM version resolver.
   *
   * @throws If either the protocol or PubKey/CRS version was not resolved.
   */
  public get protocolContext(): FhevmProtocolContext {
    return Object.freeze({
      protocolVersion: this.protocolVersion,
      pubKeyCrsVersion: this.pubKeyCrsVersion,
    });
  }

  //////////////////////////////////////////////////////////////////////////////
  // TFHE / TKMS module versions
  //////////////////////////////////////////////////////////////////////////////

  public get hasTfheVersion(): boolean {
    return this.#tfheVersion !== undefined;
  }

  /** @throws If the TFHE version was not resolved in this context. */
  public get tfheVersion(): TfheVersion {
    if (this.#tfheVersion === undefined) {
      throw new Error('FhevmClientFrozenContext: tfheVersion was not resolved in this context.');
    }
    return this.#tfheVersion;
  }

  public get tryTfheVersion(): TfheVersion | undefined {
    return this.#tfheVersion;
  }

  public get hasTkmsVersion(): boolean {
    return this.#tkmsVersion !== undefined;
  }

  /** @throws If the TKMS version was not resolved in this context. */
  public get tkmsVersion(): TkmsVersion {
    if (this.#tkmsVersion === undefined) {
      throw new Error('FhevmClientFrozenContext: tkmsVersion was not resolved in this context.');
    }
    return this.#tkmsVersion;
  }

  public get tryTkmsVersion(): TkmsVersion | undefined {
    return this.#tkmsVersion;
  }

  //////////////////////////////////////////////////////////////////////////////

  public toJSON(): Record<string, unknown> {
    return {
      hostContractVersions: this.#hostContractVersions,
      protocolVersion: this.#protocolVersion,
      pubKeyCrsVersion: this.#pubKeyCrsVersion,
      tfheVersion: this.#tfheVersion,
      tkmsVersion: this.#tkmsVersion,
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
 * The clone shares **no references** with the source: the host-contract-versions
 * record and every {@link HostContractVersion} / version-resolution object inside
 * it are copied (these are flat records, so a spread is a complete copy; version
 * strings are immutable). Mutating any part of one context can never affect the
 * other. The result is a new frozen context, built through the same
 * {@link createFhevmClientFrozenContext} path.
 *
 * @throws {InvalidTypeError} If `value` is not a {@link FhevmClientFrozenContext}.
 */
export function cloneFhevmClientFrozenContext(value: FhevmClientFrozenContext): FhevmClientFrozenContext {
  assertIsFhevmClientFrozenContext(value, {});

  // toJSON() exposes the full internal state (the exact constructor-parameter shape).
  const state = value.toJSON() as {
    readonly hostContractVersions?: Partial<Record<HostContractName, HostContractVersion>> | undefined;
    readonly protocolVersion?: ProtocolVersionResolution | undefined;
    readonly pubKeyCrsVersion?: PubKeyCrsVersionResolution | undefined;
    readonly tfheVersion?: TfheVersion | undefined;
    readonly tkmsVersion?: TkmsVersion | undefined;
  };

  const hostContractVersions: Partial<Record<HostContractName, HostContractVersion>> = {};
  const entries = Object.entries(state.hostContractVersions ?? {}) as Array<
    [HostContractName, HostContractVersion | undefined]
  >;
  for (const [name, version] of entries) {
    if (version !== undefined) {
      hostContractVersions[name] = { ...version };
    }
  }

  return createFhevmClientFrozenContext({
    hostContractVersions,
    protocolVersion: state.protocolVersion === undefined ? undefined : { ...state.protocolVersion },
    pubKeyCrsVersion: state.pubKeyCrsVersion === undefined ? undefined : { ...state.pubKeyCrsVersion },
    tfheVersion: state.tfheVersion,
    tkmsVersion: state.tkmsVersion,
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
