import {
  encrypt,
  type EncryptParameters,
  type EncryptReturnType,
} from "../../actions/encrypt/encrypt.js";
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
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { WithEncryptAndRelayer } from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";

export type EncryptActions = {
  readonly encrypt: (
    parameters: EncryptParameters,
  ) => Promise<EncryptReturnType>;
  readonly fetchGlobalFhePkeParams: (
    parameters: FetchGlobalFhePkeParamsParameters,
  ) => Promise<FetchGlobalFhePkeParamsReturnType>;
  readonly fetchGlobalFhePkeParamsBytes: (
    parameters: FetchGlobalFhePkeParamsBytesParameters,
  ) => Promise<FetchGlobalFhePkeParamsBytesReturnType>;
};

export function encryptActions(
  fhevm: Fhevm<FhevmChain, WithEncryptAndRelayer>,
): EncryptActions {
  return {
    encrypt: (parameters) => encrypt(fhevm, parameters),
    fetchGlobalFhePkeParams: (parameters) =>
      fetchGlobalFhePkeParams(fhevm, parameters),
    fetchGlobalFhePkeParamsBytes: (parameters) =>
      fetchGlobalFhePkeParamsBytes(fhevm, parameters),
  };
}
