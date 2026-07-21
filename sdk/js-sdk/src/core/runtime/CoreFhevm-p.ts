import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { EthereumModule, TrustedClient } from '../modules/ethereum/types.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type {
  Fhevm,
  FhevmBase,
  FhevmExtension,
  FhevmOptions,
  NativeClient,
  OptionalNativeClient,
  ProtocolVersionResolution,
  ResolvedFhevmOptions,
} from '../types/coreFhevmClient.js';
import type { FhevmRuntime, WithModule, WithModuleMap } from '../types/coreFhevmRuntime.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { TfheVersion } from '../../wasm/tfhe/TfheApi.js';
import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { verifyTrustedValue } from '../base/trustedValue.js';
import { uid } from '../base/uid.js';
import { createTrustedClient } from '../modules/ethereum/createTrustedClient.js';
import { asFhevmRuntimeWith, assertIsFhevmRuntime, assertIsFhevmRuntimeWith } from './CoreFhevmRuntime-p.js';
import { globalFheEncryptionKeyCache } from '../key/FheEncryptionKeyCache-p.js';
import { cloneModuleVersions } from '../runtimeConfig-p.js';
import { cloneFhevmClientFrozenContext } from '../frozenContext/FhevmClientFrozenContext-p.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('CoreFhevmHostClient.token');
const GET_FROZEN_CONTEXT = Symbol('CoreFhevmHostClient.getFrozenContext');
const SET_FROZEN_CONTEXT = Symbol('CoreFhevmHostClient.setFrozenContext');
const GET_FROZEN_CONTEXT_PROMISE = Symbol('CoreFhevmHostClient.getFrozenContextPromise');
const SET_FROZEN_CONTEXT_PROMISE = Symbol('CoreFhevmHostClient.setFrozenContextPromise');

const UNRESOLVED_FHEVM_CONTEXT_MESSAGE = 'Fhevm context has not been resolved. Await client.ready before.';

type GetFrozenContextFn = () => FhevmClientFrozenContext | undefined;
type SetFrozenContextFn = (frozenContext: FhevmClientFrozenContext) => void;
type GetFrozenContextPromiseFn = () => Promise<FhevmClientFrozenContext> | undefined;
type SetFrozenContextPromiseFn = (frozenContextPromise: Promise<FhevmClientFrozenContext> | undefined) => void;

////////////////////////////////////////////////////////////////////////////////
// CoreFhevmImpl
////////////////////////////////////////////////////////////////////////////////

class CoreFhevmImpl<
  chain extends FhevmChain,
  runtime extends FhevmRuntime,
  client extends NativeClient,
> implements Fhevm<chain, FhevmRuntime, client> {
  // Private fields (truly inaccessible from outside)
  readonly #uid: string;
  #frozenContext: FhevmClientFrozenContext | undefined;
  #frozenContextPromise: Promise<FhevmClientFrozenContext> | undefined;
  readonly #runtime: runtime;
  readonly #trustedClient: TrustedClient<client> | undefined;
  readonly #chain: chain | undefined;
  readonly #options: ResolvedFhevmOptions;
  readonly #initFns: Set<(client: FhevmBase<FhevmChain>) => Promise<void>>;
  #readyPromise: Promise<void> | undefined;

  // Declared for TypeScript — defined at runtime via Object.defineProperties
  declare readonly uid: string;
  declare readonly chain: chain;
  declare readonly protocolVersion: ProtocolVersionResolution;
  declare readonly tfheVersion: TfheVersion;
  declare readonly tkmsVersion: TkmsVersion;
  declare readonly options: ResolvedFhevmOptions;
  declare readonly trustedClient: client extends NativeClient ? TrustedClient<client> : undefined;
  declare readonly client: client;
  declare readonly runtime: runtime;
  declare readonly ethereum: EthereumModule;
  declare readonly verify: (token: symbol) => void;
  declare readonly extend: <const A extends Record<string, unknown>, RT extends FhevmRuntime>(
    actionsFactory: (client: FhevmBase<chain, FhevmRuntime, client>) => FhevmExtension<A, RT>,
  ) => this & A & { readonly runtime: RT };
  declare readonly init: () => Promise<void>;
  declare readonly ready: Promise<void>;

  constructor(
    privateToken: symbol,
    ownerToken: symbol,
    parameters: {
      readonly chain?: chain | undefined;
      readonly runtime: runtime;
      readonly client?: client | undefined;
      readonly options?: FhevmOptions | undefined;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }

    this.#runtime = parameters.runtime;
    this.#uid = uid();
    this.#frozenContext = undefined;
    this.#frozenContextPromise = undefined;
    this.#trustedClient =
      parameters.client !== undefined ? createTrustedClient(parameters.client, ownerToken) : undefined;
    this.#chain = parameters.chain;
    this.#options = resolveOptions(parameters.options);
    this.#initFns = new Set();
    this.#readyPromise = undefined;

    // verify runtime
    (this.#runtime as unknown as { verify: (token: symbol) => void }).verify(ownerToken);

    // Instance-level getters — configurable: false prevents shadowing/redefinition
    Object.defineProperties(this, {
      uid: {
        get: () => this.#uid,
        configurable: false,
        enumerable: true,
      },
      protocolVersion: {
        get: () => {
          if (this.#frozenContext === undefined) {
            throw new Error(UNRESOLVED_FHEVM_CONTEXT_MESSAGE);
          }
          return this.#frozenContext.protocolVersion;
        },
        configurable: false,
        enumerable: true,
      },
      tfheVersion: {
        get: () => {
          if (this.#frozenContext === undefined) {
            throw new Error(UNRESOLVED_FHEVM_CONTEXT_MESSAGE);
          }
          return this.#frozenContext.tfheVersion;
        },
        configurable: false,
        enumerable: true,
      },
      tkmsVersion: {
        get: () => {
          if (this.#frozenContext === undefined) {
            throw new Error(UNRESOLVED_FHEVM_CONTEXT_MESSAGE);
          }
          return this.#frozenContext.tkmsVersion;
        },
        configurable: false,
        enumerable: true,
      },
      chain: {
        get: () => this.#chain,
        configurable: false,
        enumerable: true,
      },
      options: {
        get: () => this.#options,
        configurable: false,
        enumerable: true,
      },
      trustedClient: {
        get: () => this.#trustedClient,
        configurable: false,
        enumerable: true,
      },
      client: {
        get: () =>
          this.#trustedClient !== undefined ? verifyTrustedValue(this.#trustedClient, ownerToken) : undefined,
        configurable: false,
        enumerable: true,
      },
      runtime: {
        get: () => this.#runtime,
        configurable: false,
        enumerable: true,
      },
      ethereum: {
        get: () => this.#runtime.ethereum,
        configurable: false,
        enumerable: true,
      },
      verify: {
        value: (token: symbol): void => {
          if (token !== ownerToken) {
            throw new Error('Unauthorized');
          }
        },
        configurable: false,
        enumerable: false,
        writable: false,
      },
      // [GET_PROTOCOL_VERSION]: {
      //   value: (): ProtocolVersionResolution | undefined => this.#protocolVersion,
      //   configurable: false,
      //   enumerable: false,
      //   writable: false,
      // },
      // [GET_TFHE_VERSION]: {
      //   value: (): TfheVersion | undefined => this.#tfheVersion,
      //   configurable: false,
      //   enumerable: false,
      //   writable: false,
      // },
      // [GET_TKMS_VERSION]: {
      //   value: (): TkmsVersion | undefined => this.#tkmsVersion,
      //   configurable: false,
      //   enumerable: false,
      //   writable: false,
      // },
      // [SET_PROTOCOL_VERSION]: {
      //   value: (protocolVersion: ProtocolVersionResolution): void => {
      //     if (
      //       this.#protocolVersion !== undefined &&
      //       !_sameProtocolVersionResolution(this.#protocolVersion, protocolVersion)
      //     ) {
      //       throw new Error(
      //         `Protocol version already resolved as ${_formatProtocolVersionResolution(
      //           this.#protocolVersion,
      //         )}; cannot set ${_formatProtocolVersionResolution(protocolVersion)}.`,
      //       );
      //     }
      //     this.#protocolVersion = protocolVersion;
      //   },
      //   configurable: false,
      //   enumerable: false,
      //   writable: false,
      // },
      // [SET_TFHE_VERSION]: {
      //   value: (tfheVersion: TfheVersion): void => {
      //     if (this.#tfheVersion !== undefined && this.#tfheVersion !== tfheVersion) {
      //       throw new Error(`TFHE version already resolved as ${this.#tfheVersion}; cannot set ${tfheVersion}.`);
      //     }
      //     this.#tfheVersion = tfheVersion;
      //   },
      //   configurable: false,
      //   enumerable: false,
      //   writable: false,
      // },
      // [SET_TKMS_VERSION]: {
      //   value: (tkmsVersion: TkmsVersion): void => {
      //     if (this.#tkmsVersion !== undefined && this.#tkmsVersion !== tkmsVersion) {
      //       throw new Error(`TKMS version already resolved as ${this.#tkmsVersion}; cannot set ${tkmsVersion}.`);
      //     }
      //     this.#tkmsVersion = tkmsVersion;
      //   },
      //   configurable: false,
      //   enumerable: false,
      //   writable: false,
      // },
      [GET_FROZEN_CONTEXT]: {
        value: (): FhevmClientFrozenContext | undefined => this.#frozenContext,
        configurable: false,
        enumerable: false,
        writable: false,
      },
      [SET_FROZEN_CONTEXT]: {
        // Plain overwrite (no conflict guard): the frozen context is resolved once
        // at init and later replaced wholesale by an atomic refresh.
        value: (frozenContext: FhevmClientFrozenContext): void => {
          this.#frozenContext = frozenContext;
        },
        configurable: false,
        enumerable: false,
        writable: false,
      },
      [GET_FROZEN_CONTEXT_PROMISE]: {
        value: (): Promise<FhevmClientFrozenContext> | undefined => this.#frozenContextPromise,
        configurable: false,
        enumerable: false,
        writable: false,
      },
      [SET_FROZEN_CONTEXT_PROMISE]: {
        // Transient: holds the single in-flight resolution so concurrent init fns
        // dedupe onto one promise; cleared once it settles (the resolved context
        // is kept as data in #frozenContext).
        value: (frozenContextPromise: Promise<FhevmClientFrozenContext> | undefined): void => {
          this.#frozenContextPromise = frozenContextPromise;
        },
        configurable: false,
        enumerable: false,
        writable: false,
      },
      extend: {
        value: (actionsFactory: (client: typeof this) => FhevmExtension) =>
          extendCoreFhevm(this, actionsFactory, (fn) => {
            this.#initFns.add(fn);
            this.#readyPromise = undefined;
          }),
        configurable: false,
        enumerable: false,
        writable: false,
      },
      init: {
        value: (): Promise<void> => {
          this.#readyPromise ??= Promise.all(
            [...this.#initFns].map((fn) => fn(this)),
            // eslint-disable-next-line @typescript-eslint/no-empty-function
          ).then(() => {}); // trick to cast to Promise<void>
          return this.#readyPromise;
        },
        configurable: false,
        enumerable: false,
        writable: false,
      },
      ready: {
        get: (): Promise<void> => this.init(),
        configurable: false,
        enumerable: true,
      },
    });
  }
}

////////////////////////////////////////////////////////////////////////////////

Object.freeze(CoreFhevmImpl);
Object.freeze(CoreFhevmImpl.prototype);

////////////////////////////////////////////////////////////////////////////////

type CoreClientFhevm<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends NonNullable<object> = NonNullable<object>,
> = Fhevm<chain, runtime, client> & {
  readonly trustedClient: TrustedClient<client>;
};

type CoreFhevm<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends NonNullable<object> | undefined = NonNullable<object> | undefined,
> = Fhevm<chain, runtime, client> &
  (client extends NonNullable<object>
    ? { readonly trustedClient: TrustedClient<client> }
    : { readonly trustedClient: undefined });

////////////////////////////////////////////////////////////////////////////////

function isCoreFhevm(value: unknown): value is CoreFhevm {
  return value instanceof CoreFhevmImpl;
}

function isCoreClientFhevm(value: unknown): value is CoreClientFhevm {
  if (!isCoreFhevm(value)) {
    return false;
  }
  return value.trustedClient !== undefined;
}

////////////////////////////////////////////////////////////////////////////////

export function asCoreFhevm(value: unknown): CoreFhevm {
  assertIsCoreFhevm(value, {});
  return value;
}

export function asCoreClientFhevm(value: unknown): CoreClientFhevm {
  assertIsCoreClientFhevm(value, {});
  return value;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Narrows a Fhevm instance by asserting at runtime that:
 * - the fhevm instance is valid
 * - the specified runtime module is active
 *
 * @throws if any of the two checks fail
 */
export function asFhevmWith<
  module extends keyof WithModuleMap,
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
>(fhevm: FhevmBase<chain, runtime, client>, moduleName: module): Fhevm<chain, runtime & WithModule<module>, client> {
  const f = asCoreFhevm(fhevm);
  asFhevmRuntimeWith(f.runtime, moduleName);
  return fhevm as Fhevm<chain, runtime & WithModule<module>, client>;
}

/**
 * Narrows a Fhevm instance by asserting at runtime that:
 * - the fhevm instance is valid
 * - a native client is present (not `undefined`)
 * - a chain is configured (not `undefined`)
 * - the specified runtime module is active
 *
 * @throws if any of the four checks fail
 */
export function asFhevmClientWith<
  module extends keyof WithModuleMap,
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
>(
  fhevm: FhevmBase<chain, runtime, client>,
  moduleName: module,
): Fhevm<chain & FhevmChain, runtime & WithModule<module>, client & NativeClient> {
  assertIsFhevmClientWith(fhevm, moduleName);
  return fhevm;
}

////////////////////////////////////////////////////////////////////////////////

// export function asFhevmWithProtocolVersion<
//   chain extends FhevmChain | undefined = FhevmChain | undefined,
//   runtime extends FhevmRuntime = FhevmRuntime,
//   client extends OptionalNativeClient = NativeClient,
// >(
//   fhevm: FhevmBase<chain, runtime, client>,
// ): Fhevm<chain & FhevmChain, runtime, client & NativeClient> & WithProtocolVersion {
//   assertIsFhevmBaseClient(fhevm);
//   const f = fhevm as Fhevm<chain & FhevmChain, runtime, client & NativeClient> & WithProtocolVersion;
//   if (getResolvedProtocolVersion(f) === undefined) {
//     throw new Error(UNRESOLVED_PROTOCOL_VERSION_MESSAGE);
//   }
//   return f;
// }

// export function asFhevmWithTfheVersion<
//   chain extends FhevmChain | undefined = FhevmChain | undefined,
//   runtime extends FhevmRuntime = FhevmRuntime,
//   client extends OptionalNativeClient = NativeClient,
// >(
//   fhevm: FhevmBase<chain, runtime, client>,
// ): Fhevm<chain & FhevmChain, runtime & WithModule<'encrypt'>, client & NativeClient> & WithTfheVersion {
//   const f = asFhevmClientWith(fhevm, 'encrypt') as Fhevm<
//     chain & FhevmChain,
//     runtime & WithModule<'encrypt'>,
//     client & NativeClient
//   > &
//     WithTfheVersion;
//   if (getFrozenContext(f) === undefined) {
//     throw new Error(UNRESOLVED_FHEVM_CONTEXT_MESSAGE);
//   }
//   return f;
// }

// export function asFhevmWithTkmsVersion<
//   chain extends FhevmChain | undefined = FhevmChain | undefined,
//   runtime extends FhevmRuntime = FhevmRuntime,
//   client extends OptionalNativeClient = NativeClient,
// >(
//   fhevm: FhevmBase<chain, runtime, client>,
// ): Fhevm<chain & FhevmChain, runtime & WithModule<'decrypt'>, client & NativeClient> & WithTkmsVersion {
//   const f = asFhevmClientWith(fhevm, 'decrypt') as Fhevm<
//     chain & FhevmChain,
//     runtime & WithModule<'decrypt'>,
//     client & NativeClient
//   > &
//     WithTkmsVersion;
//   if (getFrozenContext(f) === undefined) {
//     throw new Error(UNRESOLVED_FHEVM_CONTEXT_MESSAGE);
//   }
//   return f;
// }

////////////////////////////////////////////////////////////////////////////////

export function assertIsFhevmClientWith<
  module extends keyof WithModuleMap,
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
>(
  fhevm: FhevmBase<chain, runtime, client>,
  moduleName: module,
): asserts fhevm is Fhevm<chain & FhevmChain, runtime & WithModule<module>, client & NativeClient> {
  const f = asCoreClientFhevm(fhevm);
  if (f.chain === undefined) {
    throw new Error('Fhevm client chain is undefined');
  }
  assertIsFhevmRuntimeWith(f.runtime, moduleName, {});
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsFhevmBaseClient<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
>(fhevm: unknown): asserts fhevm is Fhevm<chain & FhevmChain, runtime, client & NativeClient> {
  const f = asCoreClientFhevm(fhevm);
  if (f.chain === undefined) {
    throw new Error('Fhevm client chain is undefined');
  }
  assertIsFhevmRuntime(f.runtime, {});
}

////////////////////////////////////////////////////////////////////////////////

function assertIsCoreFhevm(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is CoreFhevm {
  if (!isCoreFhevm(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'CoreFhevm',
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

function assertIsCoreClientFhevm(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is CoreClientFhevm {
  if (!isCoreClientFhevm(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'CoreClientFhevm',
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function getTrustedClient<client extends NonNullable<object>>(value: {
  readonly client: client;
}): TrustedClient<client> {
  assertIsCoreClientFhevm(value, {});
  return value.trustedClient as TrustedClient<client>;
}

////////////////////////////////////////////////////////////////////////////////

export type CreateCoreFhevmParameters<
  chain extends FhevmChain,
  runtime extends FhevmRuntime,
  client extends NativeClient,
> = {
  readonly chain?: chain | undefined;
  readonly client?: client | undefined;
  readonly runtime: runtime;
  readonly options?: FhevmOptions | undefined;
};

export function createCoreFhevm<chain extends FhevmChain, runtime extends FhevmRuntime, client extends NativeClient>(
  ownerToken: symbol,
  parameters: CreateCoreFhevmParameters<chain, runtime, client>,
): Fhevm<chain, runtime, client> {
  // Pre-populate the global FheEncryptionKey cache if the caller provided one.
  // Avoids a 50MB fetch later when encrypt is first called.
  // No-op if an entry already exists for this relayerUrl (first write wins).
  const fheEncryptionKey = parameters.options?.fheEncryptionKey;
  if (fheEncryptionKey !== undefined) {
    const relayerUrl = parameters.chain?.fhevm.relayerUrl;
    globalFheEncryptionKeyCache.setBytes(
      parameters.runtime,
      relayerUrl ?? fheEncryptionKey.metadata.relayerUrl,
      fheEncryptionKey,
    );
  }

  return new CoreFhevmImpl(PRIVATE_TOKEN, ownerToken, parameters);
}

////////////////////////////////////////////////////////////////////////////////

function extendCoreFhevm<T extends Fhevm<FhevmChain | undefined, FhevmRuntime, OptionalNativeClient>>(
  client: T,
  actionsFactory: (client: T) => FhevmExtension,
  pushInitFn: (fn: (client: FhevmBase<FhevmChain>) => Promise<void>) => void,
): T {
  const { actions, runtime, init } = actionsFactory(client);

  if (runtime !== client.runtime) {
    throw new Error(`actionsFactory must return the same runtime instance (id=${client.uid})`);
  }

  if (init !== undefined) {
    pushInitFn(init);
  }

  for (const [key, value] of Object.entries(actions)) {
    if (key in client) {
      continue;
    }

    Object.defineProperty(client, key, {
      value,
      writable: false,
      configurable: false,
      enumerable: true,
    });
  }
  return client;
}

////////////////////////////////////////////////////////////////////////////////

function resolveOptions(options: FhevmOptions | undefined): ResolvedFhevmOptions {
  return Object.freeze<ResolvedFhevmOptions>({
    batchRpcCalls: options?.batchRpcCalls ?? false,
    moduleVersions: cloneModuleVersions(options?.moduleVersions),
  });
}

/**
 * Standard prologue that **every public API action must run first**, before it
 * touches the runtime or reads any resolved version.
 *
 * The three steps make an action safe to call at any time, in any order, on a
 * shared client:
 *
 * 1. **Lazy, idempotent init** — `await fhevm.ready`. Init is lazy and shared:
 *    the first public call triggers it and every later `ready` awaits the same
 *    in-flight promise, so this is safe regardless of prior `init()`/action calls.
 * 2. **Resolve the frozen context** — the on-chain-resolved version snapshot
 *    (protocol / PubKey-CRS / TFHE / TKMS / host-contract versions) is resolved
 *    exactly once during init (via `ensureFrozenContext`) and stored on the
 *    client; this reads it back. Being client-cached, it survives repeated calls
 *    across different actions.
 * 3. **Return a deep clone** — the caller gets an independent
 *    {@link cloneFhevmClientFrozenContext | deep copy}, not the live instance, so
 *    the action runs against a stable version view for its whole async duration,
 *    immune to any future in-flight replacement of the client's stored context.
 *
 * @param fhevm - Any SDK client; validated/narrowed via {@link asCoreFhevm}.
 * @returns A deep-cloned {@link FhevmClientFrozenContext} — never `undefined`.
 *   Pass it to the action's internal implementation as its `fhevmContext`.
 * @throws If `fhevm` is not a valid SDK client (from {@link asCoreFhevm}).
 * @throws If no frozen context is stored after `ready` — an internal invariant
 *   violation: a client built via `createFhevm*Client()` always resolves and
 *   stores its context during init, so this means the client was constructed
 *   outside the SDK or init did not complete.
 */
export async function initPublicAction(fhevm: unknown): Promise<FhevmClientFrozenContext> {
  const f = asCoreFhevm(fhevm);

  // 1. Lazy, idempotent init — safe regardless of prior calls / call order.
  await f.ready;

  // 2. Read the frozen context resolved and stored during init.
  const fhevmClientContext = getFrozenContext(f);
  if (fhevmClientContext === undefined) {
    throw new Error(
      'initPublicAction: no resolved FhevmClientFrozenContext after "ready". A client created via ' +
        'createFhevm*Client() always resolves and stores its context during init, so this is an ' +
        'internal invariant violation (client not built through the SDK, or init did not complete).',
    );
  }

  // 3. Return a deep, independent snapshot the action can safely rely on.
  return cloneFhevmClientFrozenContext(fhevmClientContext);
}

////////////////////////////////////////////////////////////////////////////////

export function setFrozenContext(fhevm: FhevmBase, frozenContext: FhevmClientFrozenContext): void {
  const f = asCoreFhevm(fhevm) as CoreFhevm & { readonly [SET_FROZEN_CONTEXT]: SetFrozenContextFn };
  f[SET_FROZEN_CONTEXT](frozenContext);
}

export function getFrozenContext(fhevm: unknown): FhevmClientFrozenContext | undefined {
  const f = asCoreFhevm(fhevm) as CoreFhevm & { readonly [GET_FROZEN_CONTEXT]: GetFrozenContextFn };
  return f[GET_FROZEN_CONTEXT]();
}

export function setFrozenContextPromise(
  fhevm: FhevmBase,
  frozenContextPromise: Promise<FhevmClientFrozenContext> | undefined,
): void {
  const f = asCoreFhevm(fhevm) as CoreFhevm & { readonly [SET_FROZEN_CONTEXT_PROMISE]: SetFrozenContextPromiseFn };
  f[SET_FROZEN_CONTEXT_PROMISE](frozenContextPromise);
}

export function getFrozenContextPromise(fhevm: unknown): Promise<FhevmClientFrozenContext> | undefined {
  const f = asCoreFhevm(fhevm) as CoreFhevm & { readonly [GET_FROZEN_CONTEXT_PROMISE]: GetFrozenContextPromiseFn };
  return f[GET_FROZEN_CONTEXT_PROMISE]();
}

////////////////////////////////////////////////////////////////////////////////
