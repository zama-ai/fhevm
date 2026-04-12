import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { verifyTrustedValue } from '../base/trustedValue.js';
import { uid } from '../base/uid.js';
import type {
  EthereumModule,
  TrustedClient,
} from '../modules/ethereum/types.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type {
  Fhevm,
  FhevmBase,
  FhevmExtension,
  FhevmOptions,
  NativeClient,
  OptionalNativeClient,
  ResolvedFhevmOptions,
} from '../types/coreFhevmClient.js';
import type {
  FhevmRuntime,
  WithModule,
  WithModuleMap,
} from '../types/coreFhevmRuntime.js';
import { createTrustedClient } from '../modules/ethereum/createTrustedClient.js';
import {
  asFhevmRuntimeWith,
  assertIsFhevmRuntime,
  assertIsFhevmRuntimeWith,
} from './CoreFhevmRuntime-p.js';
import { globalFheEncryptionKeyCache } from '../key/FheEncryptionKeyCache-p.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('CoreFhevmHostClient.token');

////////////////////////////////////////////////////////////////////////////////
// CoreFhevmImpl
////////////////////////////////////////////////////////////////////////////////

class CoreFhevmImpl<
  chain extends FhevmChain | undefined,
  runtime extends FhevmRuntime,
  client extends OptionalNativeClient,
> implements Fhevm<chain, FhevmRuntime, client> {
  // Private fields (truly inaccessible from outside)
  readonly #uid: string;
  readonly #runtime: runtime;
  readonly #trustedClient: TrustedClient<client> | undefined;
  readonly #chain: chain | undefined;
  readonly #options: ResolvedFhevmOptions;
  readonly #initFns: Set<
    (
      client: FhevmBase<
        FhevmChain | undefined,
        FhevmRuntime,
        OptionalNativeClient
      >,
    ) => Promise<void>
  >;
  #readyPromise: Promise<void> | undefined;

  // Declared for TypeScript — defined at runtime via Object.defineProperties
  declare readonly uid: string;
  declare readonly chain: chain;
  declare readonly options: ResolvedFhevmOptions;
  declare readonly trustedClient: client extends NativeClient
    ? TrustedClient<client>
    : undefined;
  declare readonly client: client;
  declare readonly runtime: runtime;
  declare readonly ethereum: EthereumModule;
  declare readonly verify: (token: symbol) => void;
  declare readonly extend: <
    const A extends Record<string, unknown>,
    RT extends FhevmRuntime,
  >(
    actionsFactory: (
      client: FhevmBase<chain, FhevmRuntime, client>,
    ) => FhevmExtension<A, RT>,
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
    this.#trustedClient =
      parameters.client !== undefined
        ? createTrustedClient(parameters.client, ownerToken)
        : undefined;
    this.#chain = parameters.chain;
    this.#options = Object.freeze(resolveOptions(parameters.options));
    this.#initFns = new Set();
    this.#readyPromise = undefined;

    // verify runtime
    (this.#runtime as unknown as { verify: (token: symbol) => void }).verify(
      ownerToken,
    );

    // Instance-level getters — configurable: false prevents shadowing/redefinition
    Object.defineProperties(this, {
      uid: {
        get: () => this.#uid,
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
          this.#trustedClient !== undefined
            ? verifyTrustedValue(this.#trustedClient, ownerToken)
            : undefined,
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
  client extends NonNullable<object> | undefined =
    | NonNullable<object>
    | undefined,
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
>(
  fhevm: FhevmBase<chain, runtime, client>,
  moduleName: module,
): Fhevm<chain, runtime & WithModule<module>, client> {
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
): Fhevm<
  chain & FhevmChain,
  runtime & WithModule<module>,
  client & NativeClient
> {
  assertIsFhevmClientWith(fhevm, moduleName);
  return fhevm;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsFhevmClientWith<
  module extends keyof WithModuleMap,
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
>(
  fhevm: FhevmBase<chain, runtime, client>,
  moduleName: module,
): asserts fhevm is Fhevm<
  chain & FhevmChain,
  runtime & WithModule<module>,
  client & NativeClient
> {
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
>(
  fhevm: FhevmBase<chain, runtime, client>,
): asserts fhevm is Fhevm<chain & FhevmChain, runtime, client & NativeClient> {
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
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = OptionalNativeClient,
> = {
  readonly chain?: chain | undefined;
  readonly client?: client | undefined;
  readonly runtime: runtime;
  readonly options?: FhevmOptions | undefined;
};

export function createCoreFhevm<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = OptionalNativeClient,
>(
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

function extendCoreFhevm<
  T extends Fhevm<FhevmChain | undefined, FhevmRuntime, OptionalNativeClient>,
>(
  client: T,
  actionsFactory: (client: T) => FhevmExtension,
  pushInitFn: (
    fn: (
      client: FhevmBase<
        FhevmChain | undefined,
        FhevmRuntime,
        OptionalNativeClient
      >,
    ) => Promise<void>,
  ) => void,
): T {
  const { actions, runtime, init } = actionsFactory(client);

  if (runtime !== client.runtime) {
    throw new Error(
      `actionsFactory must return the same runtime instance (id=${client.uid})`,
    );
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

function resolveOptions(
  options: FhevmOptions | undefined,
): ResolvedFhevmOptions {
  return {
    batchRpcCalls: options?.batchRpcCalls ?? false,
  };
}
