/* eslint-disable @typescript-eslint/require-await */
import type { FhevmRuntime } from "../../types/coreFhevmRuntime.js";
import type {
  BuildWithProofPackedReturnTypeParameters,
  BuildWithProofPackedReturnType,
  ParseTFHEProvenCompactCiphertextListParameters,
  ParseTFHEProvenCompactCiphertextListReturnType,
  SerializeGlobalFheCrsParameters,
  SerializeGlobalFheCrsReturnType,
  SerializeGlobalFhePkeParamsParameters,
  SerializeGlobalFhePkeParamsReturnType,
  SerializeGlobalFhePublicKeyParameters,
  SerializeGlobalFhePublicKeyReturnType,
  EncryptModuleFactory,
  DeserializeGlobalFhePublicKeyParameters,
  DeserializeGlobalFhePublicKeyReturnType,
  DeserializeGlobalFheCrsParameters,
  DeserializeGlobalFheCrsReturnType,
} from "./types.js";

////////////////////////////////////////////////////////////////////////////////
// parseTFHEProvenCompactCiphertextList
////////////////////////////////////////////////////////////////////////////////

export async function parseTFHEProvenCompactCiphertextList(
  _parameters: ParseTFHEProvenCompactCiphertextListParameters,
): Promise<ParseTFHEProvenCompactCiphertextListReturnType> {
  throw new Error("Not yet implemented");
}

////////////////////////////////////////////////////////////////////////////////
// buildWithProofPacked
////////////////////////////////////////////////////////////////////////////////

export async function buildWithProofPacked(
  _parameters: BuildWithProofPackedReturnTypeParameters,
): Promise<BuildWithProofPackedReturnType> {
  throw new Error("Not yet implemented");
}

////////////////////////////////////////////////////////////////////////////////
// serializeGlobalFhePkeParams
////////////////////////////////////////////////////////////////////////////////

export async function serializeGlobalFhePkeParams(
  _parameters: SerializeGlobalFhePkeParamsParameters,
): Promise<SerializeGlobalFhePkeParamsReturnType> {
  throw new Error("Not yet implemented");
}

////////////////////////////////////////////////////////////////////////////////
// serializeTfhePublicKey
////////////////////////////////////////////////////////////////////////////////

export async function serializeGlobalFhePublicKey(
  _parameters: SerializeGlobalFhePublicKeyParameters,
): Promise<SerializeGlobalFhePublicKeyReturnType> {
  throw new Error("Not yet implemented");
}

////////////////////////////////////////////////////////////////////////////////
// serializeTfheCrs
////////////////////////////////////////////////////////////////////////////////

export async function serializeGlobalFheCrs(
  _parameters: SerializeGlobalFheCrsParameters,
): Promise<SerializeGlobalFheCrsReturnType> {
  throw new Error("Not yet implemented");
}

////////////////////////////////////////////////////////////////////////////////
// deserializeGlobalFhePublicKey
////////////////////////////////////////////////////////////////////////////////

export async function deserializeGlobalFheCrs(
  _parameters: DeserializeGlobalFheCrsParameters,
): Promise<DeserializeGlobalFheCrsReturnType> {
  throw new Error("Not yet implemented");
}

////////////////////////////////////////////////////////////////////////////////
// deserializeGlobalFhePkeParams
////////////////////////////////////////////////////////////////////////////////

export async function deserializeGlobalFhePublicKey(
  _parameters: DeserializeGlobalFhePublicKeyParameters,
): Promise<DeserializeGlobalFhePublicKeyReturnType> {
  throw new Error("Not yet implemented");
}

////////////////////////////////////////////////////////////////////////////////
// mockTfheActions
////////////////////////////////////////////////////////////////////////////////

export const encryptActions: EncryptModuleFactory = (
  _runtime: FhevmRuntime,
) => {
  return Object.freeze({
    encrypt: Object.freeze({
      initTfheModule: () => Promise.resolve(),
      parseTFHEProvenCompactCiphertextList,
      buildWithProofPacked,
      serializeGlobalFhePkeParams,
      serializeGlobalFhePublicKey,
      serializeGlobalFheCrs,
      deserializeGlobalFhePublicKey,
      deserializeGlobalFheCrs,
    }),
  });
};
