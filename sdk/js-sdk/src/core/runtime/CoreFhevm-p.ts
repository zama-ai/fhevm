import type { ErrorMetadataParams } from "../base/errors/ErrorBase.js";
import { InvalidTypeError } from "../base/errors/InvalidTypeError.js";
import { verifyTrustedValue } from "../base/trustedValue.js";
import { uid } from "../base/uid.js";
import type {
  EthereumModule,
  TrustedClient,
} from "../modules/ethereum/types.js";
import type { FhevmChain } from "../types/fhevmChain.js";
import type {
  Fhevm,
  FhevmOptions,
  NativeClient,
  OptionalNativeClient,
} from "../types/coreFhevmClient.js";
import type { FhevmRuntime } from "../types/coreFhevmRuntime.js";
import { createTrustedClient } from "../modules/ethereum/createTrustedClient.js";

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol("CoreFhevmHostClient.token");

////////////////////////////////////////////////////////////////////////////////

export type FhevmClientConfig = {
  readonly chain: FhevmChain;
  readonly options?: FhevmOptions;
};

////////////////////////////////////////////////////////////////////////////////
// CoreFhevmImpl
////////////////////////////////////////////////////////////////////////////////

class CoreFhevmImpl<
  chain extends FhevmChain | undefined,
  runtime extends FhevmRuntime,
  client extends OptionalNativeClient,
> implements Fhevm<chain, FhevmRuntime, client>
{
  // Private fields (truly inaccessible from outside)
  readonly #uid: string;
  readonly #runtime: runtime;
  readonly #trustedClient: TrustedClient<client> | undefined;
  readonly #chain: chain | undefined;
  readonly #options: FhevmOptions;

  // Declared for TypeScript — defined at runtime via Object.defineProperties
  declare readonly uid: string;
  declare readonly chain: chain;
  declare readonly options: FhevmOptions;
  declare readonly trustedClient: client extends NativeClient
    ? TrustedClient<client>
    : undefined;
  declare readonly client: client;
  declare readonly runtime: runtime;
  declare readonly ethereum: EthereumModule;
  declare readonly verify: (token: symbol) => void;

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
      throw new Error("Unauthorized");
    }

    this.#runtime = parameters.runtime;
    this.#uid = uid();
    this.#trustedClient =
      parameters.client !== undefined
        ? createTrustedClient(parameters.client, ownerToken)
        : undefined;
    this.#chain = parameters.chain;
    this.#options = Object.freeze(parameters.options ?? {});

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
            throw new Error("Unauthorized");
          }
        },
        configurable: false,
        enumerable: false,
        writable: false,
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

export function isCoreFhevm(value: unknown): value is CoreFhevm {
  return value instanceof CoreFhevmImpl;
}

export function isCoreClientFhevm(value: unknown): value is CoreClientFhevm {
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

export function assertIsCoreFhevm(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is CoreFhevm {
  if (!isCoreFhevm(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: "CoreFhevm",
      },
      options,
    );
  }
}

export function assertIsCoreClientFhevm(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is CoreClientFhevm {
  if (!isCoreClientFhevm(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: "CoreClientFhevm",
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function getTrustedClient<
  chain extends FhevmChain | undefined,
  runtime extends FhevmRuntime,
  client extends NonNullable<object>,
>(value: Fhevm<chain, runtime, client>): TrustedClient<client> {
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
  return new CoreFhevmImpl(PRIVATE_TOKEN, ownerToken, parameters);
}

////////////////////////////////////////////////////////////////////////////////

type Actions = Record<string, (...args: never[]) => unknown>;
type ActionsFactory<T extends Fhevm> = (fhevmClient: T) => Actions;

export function extendCoreFhevm<T extends Fhevm, F extends ActionsFactory<T>>(
  client: T,
  actionsFactory: F,
): T & ReturnType<F> {
  const actions = actionsFactory(client);
  for (const [key, value] of Object.entries(actions)) {
    if (key in client) {
      // Some action groups may share the same action
      continue;
      // throw new Error(
      //   `Cannot override existing property: ${key} (id=${client.uid})`,
      // );
    }

    Object.defineProperty(client, key, {
      value,
      writable: false,
      configurable: false,
      enumerable: true,
    });
  }
  return client as T & ReturnType<F>;
}
