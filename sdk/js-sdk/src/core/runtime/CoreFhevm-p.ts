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
  WithProtocolVersion,
  WithTfheVersion,
  WithTkmsVersion,
} from '../types/coreFhevmClient.js';
import type { FhevmRuntime, WithModule, WithModuleMap } from '../types/coreFhevmRuntime.js';
import type { TfheVersion } from '../../wasm/tfhe/TfheApi.js';
import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { verifyTrustedValue } from '../base/trustedValue.js';
import { uid } from '../base/uid.js';
import { createTrustedClient } from '../modules/ethereum/createTrustedClient.js';
import { asFhevmRuntimeWith, assertIsFhevmRuntime, assertIsFhevmRuntimeWith } from './CoreFhevmRuntime-p.js';
import { globalFheEncryptionKeyCache } from '../key/FheEncryptionKeyCache-p.js';
import { cloneModuleVersions } from '../runtimeConfig-p.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('CoreFhevmHostClient.token');
const GET_PROTOCOL_VERSION = Symbol('CoreFhevmHostClient.getProtocolVersion');
const GET_TFHE_VERSION = Symbol('CoreFhevmHostClient.getTfheVersion');
const GET_TKMS_VERSION = Symbol('CoreFhevmHostClient.getTkmsVersion');
const SET_PROTOCOL_VERSION = Symbol('CoreFhevmHostClient.setProtocolVersion');
const SET_TFHE_VERSION = Symbol('CoreFhevmHostClient.setTfheVersion');
const SET_TKMS_VERSION = Symbol('CoreFhevmHostClient.setTkmsVersion');

const UNRESOLVED_PROTOCOL_VERSION_MESSAGE =
  'Protocol version has not been resolved. Await client.ready before reading protocolVersion.';
const UNRESOLVED_TFHE_VERSION_MESSAGE =
  'TFHE version has not been resolved. Await client.ready before reading tfheVersion.';
const UNRESOLVED_TKMS_VERSION_MESSAGE =
  'TKMS version has not been resolved. Await client.ready before reading tkmsVersion.';

type GetProtocolVersionFn = () => ProtocolVersionResolution | undefined;
type GetTfheVersionFn = () => TfheVersion | undefined;
type GetTkmsVersionFn = () => TkmsVersion | undefined;
type SetProtocolVersionFn = (protocolVersion: ProtocolVersionResolution) => void;
type SetTfheVersionFn = (tfheVersion: TfheVersion) => void;
type SetTkmsVersionFn = (tkmsVersion: TkmsVersion) => void;

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
  #protocolVersion: ProtocolVersionResolution | undefined;
  #tfheVersion: TfheVersion | undefined;
  #tkmsVersion: TkmsVersion | undefined;
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
    this.#protocolVersion = undefined;
    this.#tfheVersion = undefined;
    this.#tkmsVersion = undefined;
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
          if (this.#protocolVersion === undefined) {
            throw new Error(UNRESOLVED_PROTOCOL_VERSION_MESSAGE);
          }
          return this.#protocolVersion;
        },
        configurable: false,
        enumerable: true,
      },
      tfheVersion: {
        get: () => {
          if (this.#tfheVersion === undefined) {
            throw new Error(UNRESOLVED_TFHE_VERSION_MESSAGE);
          }
          return this.#tfheVersion;
        },
        configurable: false,
        enumerable: true,
      },
      tkmsVersion: {
        get: () => {
          if (this.#tkmsVersion === undefined) {
            throw new Error(UNRESOLVED_TKMS_VERSION_MESSAGE);
          }
          return this.#tkmsVersion;
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
      [GET_PROTOCOL_VERSION]: {
        value: (): ProtocolVersionResolution | undefined => this.#protocolVersion,
        configurable: false,
        enumerable: false,
        writable: false,
      },
      [GET_TFHE_VERSION]: {
        value: (): TfheVersion | undefined => this.#tfheVersion,
        configurable: false,
        enumerable: false,
        writable: false,
      },
      [GET_TKMS_VERSION]: {
        value: (): TkmsVersion | undefined => this.#tkmsVersion,
        configurable: false,
        enumerable: false,
        writable: false,
      },
      [SET_PROTOCOL_VERSION]: {
        value: (protocolVersion: ProtocolVersionResolution): void => {
          if (
            this.#protocolVersion !== undefined &&
            !_sameProtocolVersionResolution(this.#protocolVersion, protocolVersion)
          ) {
            throw new Error(
              `Protocol version already resolved as ${_formatProtocolVersionResolution(
                this.#protocolVersion,
              )}; cannot set ${_formatProtocolVersionResolution(protocolVersion)}.`,
            );
          }
          this.#protocolVersion = protocolVersion;
        },
        configurable: false,
        enumerable: false,
        writable: false,
      },
      [SET_TFHE_VERSION]: {
        value: (tfheVersion: TfheVersion): void => {
          if (this.#tfheVersion !== undefined && this.#tfheVersion !== tfheVersion) {
            throw new Error(`TFHE version already resolved as ${this.#tfheVersion}; cannot set ${tfheVersion}.`);
          }
          this.#tfheVersion = tfheVersion;
        },
        configurable: false,
        enumerable: false,
        writable: false,
      },
      [SET_TKMS_VERSION]: {
        value: (tkmsVersion: TkmsVersion): void => {
          if (this.#tkmsVersion !== undefined && this.#tkmsVersion !== tkmsVersion) {
            throw new Error(`TKMS version already resolved as ${this.#tkmsVersion}; cannot set ${tkmsVersion}.`);
          }
          this.#tkmsVersion = tkmsVersion;
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

export function asFhevmWithProtocolVersion<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
>(
  fhevm: FhevmBase<chain, runtime, client>,
): Fhevm<chain & FhevmChain, runtime, client & NativeClient> & WithProtocolVersion {
  assertIsFhevmBaseClient(fhevm);
  const f = fhevm as Fhevm<chain & FhevmChain, runtime, client & NativeClient> & WithProtocolVersion;
  if (getResolvedProtocolVersion(f) === undefined) {
    throw new Error(UNRESOLVED_PROTOCOL_VERSION_MESSAGE);
  }
  return f;
}

export function asFhevmWithTfheVersion<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
>(
  fhevm: FhevmBase<chain, runtime, client>,
): Fhevm<chain & FhevmChain, runtime & WithModule<'encrypt'>, client & NativeClient> & WithTfheVersion {
  const f = asFhevmClientWith(fhevm, 'encrypt') as Fhevm<
    chain & FhevmChain,
    runtime & WithModule<'encrypt'>,
    client & NativeClient
  > &
    WithTfheVersion;
  if (getResolvedProtocolVersion(f) === undefined) {
    throw new Error(UNRESOLVED_PROTOCOL_VERSION_MESSAGE);
  }
  if (getResolvedTfheVersion(f) === undefined) {
    throw new Error(UNRESOLVED_TFHE_VERSION_MESSAGE);
  }
  return f;
}

export function asFhevmWithTkmsVersion<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
>(
  fhevm: FhevmBase<chain, runtime, client>,
): Fhevm<chain & FhevmChain, runtime & WithModule<'decrypt'>, client & NativeClient> & WithTkmsVersion {
  const f = asFhevmClientWith(fhevm, 'decrypt') as Fhevm<
    chain & FhevmChain,
    runtime & WithModule<'decrypt'>,
    client & NativeClient
  > &
    WithTkmsVersion;
  if (getResolvedProtocolVersion(f) === undefined) {
    throw new Error(UNRESOLVED_PROTOCOL_VERSION_MESSAGE);
  }
  if (getResolvedTkmsVersion(f) === undefined) {
    throw new Error(UNRESOLVED_TKMS_VERSION_MESSAGE);
  }
  return f;
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

export function createCoreFhevm<runtime extends FhevmRuntime>(
  ownerToken: symbol,
  parameters: {
    readonly chain?: undefined;
    readonly client?: undefined;
    readonly runtime: runtime;
    readonly options?: FhevmOptions | undefined;
  },
): Fhevm<undefined, runtime, undefined>;

export function createCoreFhevm<chain extends FhevmChain, runtime extends FhevmRuntime, client extends NativeClient>(
  ownerToken: symbol,
  parameters: CreateCoreFhevmParameters<chain, runtime, client>,
): Fhevm<chain, runtime, client>;

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

////////////////////////////////////////////////////////////////////////////////

function _sameProtocolVersionResolution(a: ProtocolVersionResolution, b: ProtocolVersionResolution): boolean {
  return a.version === b.version && a.comparator === b.comparator;
}

function _formatProtocolVersionResolution(protocolVersion: ProtocolVersionResolution): string {
  return `${protocolVersion.comparator}:${protocolVersion.version}`;
}

////////////////////////////////////////////////////////////////////////////////

export function setResolvedProtocolVersion(fhevm: FhevmBase, protocolVersion: ProtocolVersionResolution): void {
  const f = asCoreFhevm(fhevm) as CoreFhevm & { readonly [SET_PROTOCOL_VERSION]: SetProtocolVersionFn };
  f[SET_PROTOCOL_VERSION](protocolVersion);
}

export function setResolvedTfheVersion(
  fhevm: FhevmBase<FhevmChain | undefined, FhevmRuntime, OptionalNativeClient>,
  tfheVersion: TfheVersion,
): void {
  const f = asCoreFhevm(fhevm) as CoreFhevm & { readonly [SET_TFHE_VERSION]: SetTfheVersionFn };
  f[SET_TFHE_VERSION](tfheVersion);
}

export function setResolvedTkmsVersion(
  fhevm: FhevmBase<FhevmChain | undefined, FhevmRuntime, OptionalNativeClient>,
  tkmsVersion: TkmsVersion,
): void {
  const f = asCoreFhevm(fhevm) as CoreFhevm & { readonly [SET_TKMS_VERSION]: SetTkmsVersionFn };
  f[SET_TKMS_VERSION](tkmsVersion);
}

////////////////////////////////////////////////////////////////////////////////

export function getResolvedProtocolVersion(fhevm: unknown): ProtocolVersionResolution | undefined {
  const f = asCoreFhevm(fhevm) as CoreFhevm & { readonly [GET_PROTOCOL_VERSION]: GetProtocolVersionFn };
  return f[GET_PROTOCOL_VERSION]();
}

export function getResolvedTfheVersion(fhevm: unknown): TfheVersion | undefined {
  const f = asCoreFhevm(fhevm) as CoreFhevm & { readonly [GET_TFHE_VERSION]: GetTfheVersionFn };
  return f[GET_TFHE_VERSION]();
}

export function getResolvedTkmsVersion(fhevm: unknown): TkmsVersion | undefined {
  const f = asCoreFhevm(fhevm) as CoreFhevm & { readonly [GET_TKMS_VERSION]: GetTkmsVersionFn };
  return f[GET_TKMS_VERSION]();
}

////////////////////////////////////////////////////////////////////////////////
