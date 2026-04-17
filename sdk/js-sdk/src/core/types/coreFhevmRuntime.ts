import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { EncryptModule, EncryptModuleFactory, WithEncryptModule } from '../modules/encrypt/types.js';
import type { DecryptModule, DecryptModuleFactory, WithDecryptModule } from '../modules/decrypt/types.js';
import type { Logger } from './logger.js';
import type { Auth } from './auth.js';

////////////////////////////////////////////////////////////////////////////////

export type FhevmRuntimeConfig = {
  readonly locateFile?: ((file: string) => URL) | undefined;
  readonly logger?: Logger | undefined;
  readonly singleThread?: boolean | undefined;
  readonly numberOfThreads?: number | undefined;
  readonly auth?: Auth | undefined;
};

// eslint-disable-next-line @typescript-eslint/naming-convention
interface FhevmRuntime_Base {
  readonly ethereum: EthereumModule;
  readonly relayer: RelayerModule;
  readonly uid: string;
  readonly config: FhevmRuntimeConfig;

  extend(factory: DecryptModuleFactory): this & { readonly decrypt: DecryptModule };

  extend(factory: EncryptModuleFactory): this & { readonly encrypt: EncryptModule };
}

////////////////////////////////////////////////////////////////////////////////

export type WithModuleMap = {
  readonly decrypt: WithDecryptModule;
  readonly encrypt: WithEncryptModule;
};

export type FhevmRuntime<Extensions extends object = object> = FhevmRuntime_Base & Extensions;

// Convenience types for generics
export type WithModule<M extends keyof WithModuleMap> = FhevmRuntime<WithModuleMap[M]>;
export type WithDecrypt = FhevmRuntime<WithDecryptModule>;
export type WithEncrypt = FhevmRuntime<WithEncryptModule>;

export type WithAll = FhevmRuntime<WithEncryptModule & WithDecryptModule>;
