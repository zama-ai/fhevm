////////////////////////////////////////////////////////////////////////////////
// encryptModule
////////////////////////////////////////////////////////////////////////////////

import type { FhevmRuntime } from "../../../types/coreFhevmRuntime.js";
import type {
  BuildWithProofPackedReturnTypeParameters,
  DeserializeGlobalFheCrsParameters,
  DeserializeGlobalFhePublicKeyParameters,
  EncryptModuleFactory,
  ParseTFHEProvenCompactCiphertextListParameters,
  SerializeGlobalFheCrsParameters,
  SerializeGlobalFhePkeParamsParameters,
  SerializeGlobalFhePublicKeyParameters,
} from "../types.js";
import {
  buildWithProofPacked,
  deserializeGlobalFheCrs,
  deserializeGlobalFhePublicKey,
  parseTFHEProvenCompactCiphertextList,
  serializeGlobalFheCrs,
  serializeGlobalFhePkeParams,
  serializeGlobalFhePublicKey,
} from "./api-p.js";
import { initTfheModule } from "./init-p.js";

export const encryptModule: EncryptModuleFactory = (runtime: FhevmRuntime) => {
  return Object.freeze({
    encrypt: Object.freeze({
      initTfheModule: () => initTfheModule(runtime),
      parseTFHEProvenCompactCiphertextList: (
        args: ParseTFHEProvenCompactCiphertextListParameters,
      ) => parseTFHEProvenCompactCiphertextList(runtime, args),
      buildWithProofPacked: (args: BuildWithProofPackedReturnTypeParameters) =>
        buildWithProofPacked(runtime, args),
      serializeGlobalFhePkeParams: (
        args: SerializeGlobalFhePkeParamsParameters,
      ) => serializeGlobalFhePkeParams(runtime, args),
      serializeGlobalFhePublicKey: (
        args: SerializeGlobalFhePublicKeyParameters,
      ) => serializeGlobalFhePublicKey(runtime, args),
      serializeGlobalFheCrs: (args: SerializeGlobalFheCrsParameters) =>
        serializeGlobalFheCrs(runtime, args),
      deserializeGlobalFhePublicKey: (
        args: DeserializeGlobalFhePublicKeyParameters,
      ) => deserializeGlobalFhePublicKey(runtime, args),
      deserializeGlobalFheCrs: (args: DeserializeGlobalFheCrsParameters) =>
        deserializeGlobalFheCrs(runtime, args),
    }),
  });
};
