import type { RelayerFetchOptions } from "../../modules/relayer/types.js";
import type {
  Fhevm,
  OptionalNativeClient,
} from "../../types/coreFhevmClient.js";
import type { WithEncryptAndRelayer } from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type {
  GlobalFhePkeParams,
  GlobalFhePkeParamsBytes,
} from "../../types/globalFhePkeParams.js";
import { deserializeGlobalFhePkeParams } from "../encrypt/deserializeGlobalFhePkeParams.js";
import { fetchGlobalFhePkeParams } from "./fetchGlobalFhePkeParams.js";

export type ResolveGlobalFhePkeParamsParameters = {
  readonly globalFheParamsBytes?: GlobalFhePkeParamsBytes;
  readonly options?: RelayerFetchOptions;
  readonly ignoreCache?: boolean;
};

export type ResolveGlobalFhePkeParamsReturnType = GlobalFhePkeParams;

export async function resolveGlobalFhePkeParams(
  fhevm: Fhevm<FhevmChain, WithEncryptAndRelayer, OptionalNativeClient>,
  parameters: ResolveGlobalFhePkeParamsParameters,
): Promise<ResolveGlobalFhePkeParamsReturnType> {
  // Defensive test
  if (parameters.globalFheParamsBytes != null) {
    try {
      return await deserializeGlobalFhePkeParams(
        fhevm,
        parameters.globalFheParamsBytes,
      );
    } catch {
      // Silent catch.
      // If the provided params are invalid, fall through to fetching from the relayer.
    }
  }

  return await fetchGlobalFhePkeParams(fhevm, {
    ignoreCache: parameters.ignoreCache,
    options: parameters.options,
  });
}
