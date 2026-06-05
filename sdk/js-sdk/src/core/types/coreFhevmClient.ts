import type { FhevmChain } from './fhevmChain.js';
import type { FhevmRuntime } from './coreFhevmRuntime.js';
import type { FheEncryptionKeyBytes } from './fheEncryptionKey.js';
import type {
  FhevmDecryptModuleVersions,
  FhevmEncryptModuleVersions,
  FhevmModuleVersions,
  TfheVersion,
  TkmsVersion,
} from './moduleVersions.js';

////////////////////////////////////////////////////////////////////////////////

export type FhevmBaseOptions = {
  readonly batchRpcCalls?: boolean | undefined;
};

export type FhevmEncryptOptions = FhevmBaseOptions & {
  readonly fheEncryptionKey?: FheEncryptionKeyBytes | undefined;
  readonly moduleVersions?: FhevmEncryptModuleVersions | undefined;
};

export type FhevmDecryptOptions = FhevmBaseOptions & {
  readonly moduleVersions?: FhevmDecryptModuleVersions | undefined;
};

export type FhevmOptions = FhevmBaseOptions & {
  readonly fheEncryptionKey?: FheEncryptionKeyBytes | undefined;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
};

export type ResolvedFhevmOptions = {
  readonly batchRpcCalls: boolean;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
};

////////////////////////////////////////////////////////////////////////////////

export type NativeClient = NonNullable<object>;
export type OptionalNativeClient = NativeClient | undefined;
export type OptionalFhevmChain = FhevmChain | undefined;

export type WithTfheVersion = {
  readonly tfheVersion: TfheVersion;
};

export type WithTkmsVersion = {
  readonly tkmsVersion: TkmsVersion;
};

export type FhevmExtension<
  actions extends Record<string, unknown> = Record<string, unknown>,
  runtime extends FhevmRuntime = FhevmRuntime,
> = {
  readonly actions: actions;
  readonly runtime: runtime;
  readonly init?: ((client: FhevmBase<FhevmChain>) => Promise<void>) | undefined;
};

export interface FhevmBase<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
> {
  readonly uid: string;
  readonly chain: chain; // undefined when no chain
  readonly runtime: runtime;
  readonly client: client; // undefined when no host
  readonly options: ResolvedFhevmOptions;
}

export interface Fhevm<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
> extends FhevmBase<chain, runtime, client> {
  readonly extend: <const actions extends Record<string, unknown>, extendedRuntime extends FhevmRuntime>(
    actionsFactory: (client: FhevmBase<chain, FhevmRuntime, client>) => FhevmExtension<actions, extendedRuntime>,
  ) => this & actions & { readonly runtime: extendedRuntime };
  readonly init: () => Promise<void>;
  readonly ready: Promise<void>;
}
