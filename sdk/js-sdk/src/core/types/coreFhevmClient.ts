//import type { TrustedClient } from "../modules/ethereum/types.js";
import type { FhevmChain } from "./fhevmChain.js";
import type { FhevmRuntime } from "./coreFhevmRuntime.js";

export type FhevmOptions = {
  readonly batchRpcCalls?: boolean;
};

export type NativeClient = NonNullable<object>;
export type OptionalNativeClient = NativeClient | undefined;
export type OptionalFhevmChain = FhevmChain | undefined;

// since runtime is almost always the default, maybe put it at the last slot ?
export type Fhevm<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
> = {
  readonly uid: string;
  readonly chain: chain; // undefined when no chain
  readonly runtime: runtime;
  readonly client: client; // undefined when no host
  readonly options?: FhevmOptions;
};
