import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { EncryptModule } from '../modules/encrypt/types.js';
import type { FhevmRuntime, FhevmRuntimeConfig, WithModuleMap } from '../types/coreFhevmRuntime.js';
import type { DecryptModule } from '../modules/decrypt/types.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { uid } from '../base/uid.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('CoreFhevmClient.token');

////////////////////////////////////////////////////////////////////////////////

type ModulePlaceholder<T> = Record<string, never> | T;

function asModule<Module extends object>(placeholder: ModulePlaceholder<Module>, name: string): Module {
  if (Object.keys(placeholder).length === 0) {
    throw new Error(`Missing ${name} module`);
  }
  return placeholder as Module;
}

type Slot = { placeholder: object; factory?: unknown };

type ModuleName = keyof WithModuleMap;
type FactoryFn = (runtime: FhevmRuntime) => Record<string, object>;
type ExtendFn<T> = (factory: FactoryFn) => T;

function createExtendFn<T extends FhevmRuntime>(selfRuntime: T, moduleSlots: Map<ModuleName, Slot>): ExtendFn<T> {
  const factories = new Set<unknown>();
  return (moduleFactory: FactoryFn) => {
    // Same factory reference → idempotent no-op
    if (factories.has(moduleFactory)) {
      return selfRuntime;
    }

    // Call factory to get { moduleName: moduleFunctions }
    const module = moduleFactory(selfRuntime);

    // Extract the single key (e.g. "decrypt", "encrypt")
    const keys = Object.keys(module);
    if (keys.length !== 1 || keys[0] === undefined) {
      throw new Error('Factory must return exactly one key');
    }
    const moduleName = keys[0] as ModuleName;

    // Look up the matching placeholder slot (validates the cast above)
    const moduleSlot = moduleSlots.get(moduleName);
    if (moduleSlot === undefined) {
      throw new Error(`Unknown module: ${moduleName}`);
    }

    // Reject a different factory for the same slot
    if (moduleSlot.factory !== undefined) {
      throw new Error(`Already extended: ${moduleName}`);
    }

    // Extract the module functions object from factory result
    const moduleFunctions = module[moduleName];
    if (moduleFunctions === undefined) {
      throw new Error(`Missing ${moduleName} in factory result`);
    }

    // Fill the empty placeholder and freeze it
    Object.assign(moduleSlot.placeholder, moduleFunctions);
    Object.freeze(moduleSlot.placeholder);

    // Record factory for duplicate-slot detection
    moduleSlot.factory = moduleFactory;

    // Record factory for idempotency check
    factories.add(moduleFactory);

    return selfRuntime;
  };
}

////////////////////////////////////////////////////////////////////////////////
// CoreFhevmRuntimeImpl
//
// Class: enables instanceof checks (verifiability via assertIsFhevmRuntime)
// Frozen: Object.freeze(this) — instance is immutable after construction
// Frozen: class and prototype are frozen
// Owner token: captured in verify() closure, verified by assertIsFhevmRuntime
//
// Extension mechanism:
// - Empty placeholders (#encrypt, #decrypt, #relayer) created at construction
// - extend() fills a placeholder exactly once, then freezes it
// - Throws if a placeholder is already filled (no double-extend)
//
// Properties:
// - Tree-shakable
// - Lightweight
// - Idempotent extend
// - GC/memory friendly
// - Zero dependency
// - Immutable
// - Secure
class CoreFhevmRuntimeImpl {
  // Unique id
  readonly #uid: string;

  // Global SDK config
  readonly #config: FhevmRuntimeConfig;

  // Base modules
  readonly #ethereum: EthereumModule;
  readonly #relayer: RelayerModule;

  // Optional modules
  readonly #encrypt: ModulePlaceholder<EncryptModule> = {};
  readonly #decrypt: ModulePlaceholder<DecryptModule> = {};

  // Methods
  declare readonly verify: (token: symbol) => void;
  declare readonly hasModule: (moduleName: ModuleName) => boolean;
  declare readonly extend: FhevmRuntime['extend'];

  constructor(privateToken: symbol, ownerToken: symbol, parameters: CreateFhevmRuntimeParameters) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }

    this.#ethereum = parameters.ethereum;
    this.#relayer = parameters.relayer;

    this.#uid = uid();
    this.#config = {
      ...parameters.config,
      logger: parameters.config.logger ? { ...parameters.config.logger } : undefined,
      auth: parameters.config.auth ? { ...parameters.config.auth } : undefined,
    };
    const decrypt = this.#decrypt;
    const encrypt = this.#encrypt;

    this.verify = (token: symbol) => {
      if (token !== ownerToken) {
        throw new Error('Unauthorized');
      }
    };

    const slots = new Map<ModuleName, { placeholder: object; factory?: unknown }>([
      ['decrypt', { placeholder: decrypt }],
      ['encrypt', { placeholder: encrypt }],
    ]);

    this.extend = createExtendFn(this, slots) as unknown as FhevmRuntime['extend'];

    this.hasModule = (moduleName: ModuleName) => {
      return slots.get(moduleName)?.factory !== undefined;
    };

    Object.freeze(this);
  }

  public get uid(): string {
    return this.#uid;
  }

  public get config(): FhevmRuntimeConfig {
    return this.#config;
  }

  public get ethereum(): EthereumModule {
    return this.#ethereum;
  }

  public get relayer(): RelayerModule {
    return this.#relayer;
  }

  public get decrypt(): DecryptModule {
    return asModule(this.#decrypt, 'decrypt');
  }

  public get encrypt(): EncryptModule {
    return asModule(this.#encrypt, 'encrypt');
  }
}

////////////////////////////////////////////////////////////////////////////////

Object.freeze(CoreFhevmRuntimeImpl);
Object.freeze(CoreFhevmRuntimeImpl.prototype);

////////////////////////////////////////////////////////////////////////////////
// Public API
////////////////////////////////////////////////////////////////////////////////

export type CreateFhevmRuntimeParameters = {
  readonly ethereum: EthereumModule;
  readonly relayer: RelayerModule;
  readonly config: FhevmRuntimeConfig;
};

export function createFhevmRuntime(ownerToken: symbol, parameters: CreateFhevmRuntimeParameters): FhevmRuntime {
  return new CoreFhevmRuntimeImpl(PRIVATE_TOKEN, ownerToken, parameters);
}

////////////////////////////////////////////////////////////////////////////////

export function isFhevmRuntime(value: unknown): value is FhevmRuntime {
  return value instanceof CoreFhevmRuntimeImpl;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsFhevmRuntime(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is FhevmRuntime {
  if (!isFhevmRuntime(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'FhevmRuntime',
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function verifyFhevmRuntime(value: unknown, ownerToken: symbol): asserts value is FhevmRuntime {
  assertIsFhevmRuntime(value, {});
  (value as CoreFhevmRuntimeImpl).verify(ownerToken);
}

////////////////////////////////////////////////////////////////////////////////

export function asFhevmRuntimeWith<K extends keyof WithModuleMap>(
  fhevmRuntime: FhevmRuntime,
  moduleName: K,
): FhevmRuntime & WithModuleMap[K] {
  assertIsFhevmRuntimeWith(fhevmRuntime, moduleName, {});
  return fhevmRuntime;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsFhevmRuntimeWith<K extends keyof WithModuleMap>(
  fhevmRuntime: FhevmRuntime,
  moduleName: K,
  options: { subject?: string } & ErrorMetadataParams,
): asserts fhevmRuntime is FhevmRuntime & WithModuleMap[K] {
  assertIsFhevmRuntime(fhevmRuntime, options);
  // Getter throws via asModule() if the module is not extended
  void (fhevmRuntime as unknown as Record<string, unknown>)[moduleName];
}

////////////////////////////////////////////////////////////////////////////////

export function assertOwnedBy(parameters: { actualOwner: WeakRef<object>; expectedOwner: object; name: string }): void {
  const { actualOwner: actual, expectedOwner: expected, name } = parameters;
  const owner = actual.deref();
  if (owner === undefined) {
    throw new Error(`${name} owner has been garbage collected`);
  }
  if (owner !== expected) {
    throw new Error(`${name} has not the expected owner`);
  }
}
