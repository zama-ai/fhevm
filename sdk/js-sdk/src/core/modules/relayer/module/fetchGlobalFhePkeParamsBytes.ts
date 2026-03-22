////////////////////////////////////////////////////////////////////////////////
// fetchKeyUrl
////////////////////////////////////////////////////////////////////////////////

import type {
  FetchGlobalFhePkeParamsBytesParameters,
  FetchGlobalFhePkeParamsBytesReturnType,
  RelayerClient,
} from "../types.js";
import type { RelayerKeyUrlOptions } from "../../../types/relayer.js";
import { fetchGlobalFhePkeParamsSource } from "./fetchGlobalFhePkeParamsSource.js";
import { fetchGlobalFhePkeParamsBytesWithSource as fetchGlobalFhePkeParamsBytesWithSource_ } from "../../../globalFheKey/fetchGlobalFhePkeParamsBytesWithSource.js";

////////////////////////////////////////////////////////////////////////////////
// fetchGlobalFhePkeParamsBytes
////////////////////////////////////////////////////////////////////////////////

export async function fetchGlobalFhePkeParamsBytes(
  relayerClient: RelayerClient,
  parameters: FetchGlobalFhePkeParamsBytesParameters,
): Promise<FetchGlobalFhePkeParamsBytesReturnType> {
  const { options } = parameters;
  const relayerOptions = options as RelayerKeyUrlOptions | undefined;

  // 1. Ask the relayer for the URLs where the keys are hosted
  const source = await fetchGlobalFhePkeParamsSource(relayerClient, {
    options: relayerOptions,
  });

  const init: RequestInit | undefined =
    relayerOptions?.signal !== undefined
      ? { signal: relayerOptions.signal }
      : undefined;

  // 2. Download the actual keys from those URLs
  const paramsBytes = await fetchGlobalFhePkeParamsBytesWithSource_(source, {
    retries: relayerOptions?.fetchRetries,
    retryDelayMs: relayerOptions?.fetchRetryDelayInMilliseconds,
    init,
  });

  return paramsBytes;
}
