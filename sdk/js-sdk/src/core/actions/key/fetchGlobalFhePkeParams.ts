import type { RelayerFetchOptions } from "../../modules/relayer/types.js";
import type {
  Fhevm,
  OptionalNativeClient,
} from "../../types/coreFhevmClient.js";
import type { WithEncryptAndRelayer } from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type { GlobalFhePkeParams } from "../../types/globalFhePkeParams.js";
import { deserializeGlobalFhePkeParams } from "../encrypt/deserializeGlobalFhePkeParams.js";
import { fetchGlobalFhePkeParamsBytes } from "./fetchGlobalFhePkeParamsBytes.js";

////////////////////////////////////////////////////////////////////////////////

export type FetchGlobalFhePkeParamsParameters = {
  readonly options?: RelayerFetchOptions;
  readonly ignoreCache?: boolean | undefined;
};

export type FetchGlobalFhePkeParamsReturnType = GlobalFhePkeParams;

////////////////////////////////////////////////////////////////////////////////

export async function fetchGlobalFhePkeParams(
  fhevm: Fhevm<FhevmChain, WithEncryptAndRelayer, OptionalNativeClient>,
  parameters?: FetchGlobalFhePkeParamsParameters | undefined,
): Promise<FetchGlobalFhePkeParamsReturnType> {
  const paramsBytes = await fetchGlobalFhePkeParamsBytes(fhevm, parameters);
  return deserializeGlobalFhePkeParams(fhevm, paramsBytes);
}
