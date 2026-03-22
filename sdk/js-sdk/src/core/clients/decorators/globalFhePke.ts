import {
  fetchGlobalFhePkeParams,
  type FetchGlobalFhePkeParamsParameters,
  type FetchGlobalFhePkeParamsReturnType,
} from "../../actions/key/fetchGlobalFhePkeParams.js";
import {
  fetchGlobalFhePkeParamsBytes,
  type FetchGlobalFhePkeParamsBytesParameters,
  type FetchGlobalFhePkeParamsBytesReturnType,
} from "../../actions/key/fetchGlobalFhePkeParamsBytes.js";
import {
  resolveGlobalFhePkeParams,
  type ResolveGlobalFhePkeParamsParameters,
  type ResolveGlobalFhePkeParamsReturnType,
} from "../../actions/key/resolveGlobalFhePkeParams.js";
import {
  deserializeGlobalFhePkeParamsFromHex,
  type DeserializeGlobalFhePkeParamsFromHexParameters,
  type DeserializeGlobalFhePkeParamsFromHexReturnType,
} from "../../actions/encrypt/deserializeGlobalFhePkeParams.js";
import {
  serializeGlobalFhePkeParamsToHex,
  type SerializeGlobalFhePkeParamsToHexParameters,
  type SerializeGlobalFhePkeParamsToHexReturnType,
} from "../../actions/encrypt/serializeGlobalFhePkeParams.js";
import type {
  Fhevm,
  OptionalNativeClient,
} from "../../types/coreFhevmClient.js";
import type { WithEncryptAndRelayer } from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";

export type GlobalFhePkeActions = {
  readonly fetchGlobalFhePkeParams: (
    parameters?: FetchGlobalFhePkeParamsParameters | undefined,
  ) => Promise<FetchGlobalFhePkeParamsReturnType>;
  readonly fetchGlobalFhePkeParamsBytes: (
    parameters?: FetchGlobalFhePkeParamsBytesParameters | undefined,
  ) => Promise<FetchGlobalFhePkeParamsBytesReturnType>;
  readonly deserializeGlobalFhePkeParamsFromHex: (
    parameters: DeserializeGlobalFhePkeParamsFromHexParameters,
  ) => Promise<DeserializeGlobalFhePkeParamsFromHexReturnType>;
  readonly serializeGlobalFhePkeParamsToHex: (
    parameters: SerializeGlobalFhePkeParamsToHexParameters,
  ) => Promise<SerializeGlobalFhePkeParamsToHexReturnType>;
  readonly resolveGlobalFhePkeParams: (
    parameters: ResolveGlobalFhePkeParamsParameters,
  ) => Promise<ResolveGlobalFhePkeParamsReturnType>;
};

export function globalFhePkeActions(
  fhevm: Fhevm<FhevmChain, WithEncryptAndRelayer, OptionalNativeClient>,
): GlobalFhePkeActions {
  return {
    fetchGlobalFhePkeParams: (parameters) =>
      fetchGlobalFhePkeParams(fhevm, parameters),
    fetchGlobalFhePkeParamsBytes: (parameters) =>
      fetchGlobalFhePkeParamsBytes(fhevm, parameters),
    deserializeGlobalFhePkeParamsFromHex: (parameters) =>
      deserializeGlobalFhePkeParamsFromHex(fhevm, parameters),
    serializeGlobalFhePkeParamsToHex: (parameters) =>
      serializeGlobalFhePkeParamsToHex(fhevm, parameters),
    resolveGlobalFhePkeParams: (parameters) =>
      resolveGlobalFhePkeParams(fhevm, parameters),
  };
}
