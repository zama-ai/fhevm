import type { EthereumModule } from "../modules/ethereum/types.js";
import type {
  RelayerModule,
  RelayerModuleFactory,
  WithRelayerModule,
} from "../modules/relayer/types.js";
import type {
  EncryptModule,
  EncryptModuleFactory,
  WithEncryptModule,
} from "../modules/encrypt/types.js";
import type {
  DecryptModule,
  DecryptModuleFactory,
  WithDecryptModule,
} from "../modules/decrypt/types.js";
import type { Logger } from "./logger.js";

export type FhevmRuntimeConfig = {
  readonly locateFile?: ((file: string) => URL) | undefined;
  readonly logger?: Logger | undefined;
  readonly singleThread?: boolean | undefined;
  readonly numberOfThreads?: number | undefined;
};

// eslint-disable-next-line @typescript-eslint/naming-convention
interface FhevmRuntime_Base {
  readonly ethereum: EthereumModule;
  readonly uid: string;
  readonly config: FhevmRuntimeConfig;

  extend(
    factory: DecryptModuleFactory,
  ): this & { readonly decrypt: DecryptModule };

  extend(
    factory: EncryptModuleFactory,
  ): this & { readonly encrypt: EncryptModule };

  extend(
    factory: RelayerModuleFactory,
  ): this & { readonly relayer: RelayerModule };
}

export type FhevmRuntime<Extensions extends object = object> =
  FhevmRuntime_Base & Extensions;

// Convenience types for CoreFhevmClient<R> generics
export type WithDecrypt = FhevmRuntime<WithDecryptModule>;
export type WithEncrypt = FhevmRuntime<WithEncryptModule>;
export type WithRelayer = FhevmRuntime<WithRelayerModule>;

export type WithEncryptAndRelayer = FhevmRuntime<
  WithEncryptModule & WithRelayerModule
>;

export type WithDecryptAndRelayer = FhevmRuntime<
  WithDecryptModule & WithRelayerModule
>;

export type WithAll = FhevmRuntime<
  WithEncryptModule & WithDecryptModule & WithRelayerModule
>;
