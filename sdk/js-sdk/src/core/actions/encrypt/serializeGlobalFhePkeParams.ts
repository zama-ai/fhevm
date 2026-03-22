import { bytesToHexLarge } from "../../base/bytes.js";
import { assertGlobalFhePkeParamsOwnedBy } from "../../globalFheKey/GlobalFhePkeParams-p.js";
import type { WithEncryptModule } from "../../modules/encrypt/types.js";
import type {
  Fhevm,
  OptionalNativeClient,
} from "../../types/coreFhevmClient.js";
import type {
  FhevmRuntime,
  WithEncrypt,
} from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import {
  type GlobalFheCrsBytes,
  type GlobalFheCrsBytesHex,
  type GlobalFhePkeParams,
  type GlobalFhePkeParamsBytes,
  type GlobalFhePkeParamsBytesHex,
  type GlobalFhePublicKeyBytes,
  type GlobalFhePublicKeyBytesHex,
} from "../../types/globalFhePkeParams.js";

////////////////////////////////////////////////////////////////////////////////

export type SerializeGlobalFhePkeParamsParameters = GlobalFhePkeParams;
export type SerializeGlobalFhePkeParamsReturnType = GlobalFhePkeParamsBytes;

export async function serializeGlobalFhePkeParams(
  fhevmRuntime: FhevmRuntime<WithEncryptModule>,
  parameters: SerializeGlobalFhePkeParamsParameters,
): Promise<SerializeGlobalFhePkeParamsReturnType> {
  assertGlobalFhePkeParamsOwnedBy(parameters, fhevmRuntime);

  const publicKeyBytes: GlobalFhePublicKeyBytes =
    await fhevmRuntime.encrypt.serializeGlobalFhePublicKey({
      globalFhePublicKey: parameters.publicKey,
    });

  const crsBytes: GlobalFheCrsBytes =
    await fhevmRuntime.encrypt.serializeGlobalFheCrs({
      globalFheCrs: parameters.crs,
    });

  return Object.freeze({
    publicKeyBytes: publicKeyBytes,
    crsBytes: crsBytes,
  });
}

////////////////////////////////////////////////////////////////////////////////

export type SerializeGlobalFhePkeParamsToHexParameters = GlobalFhePkeParams;
export type SerializeGlobalFhePkeParamsToHexReturnType =
  GlobalFhePkeParamsBytesHex;

export async function serializeGlobalFhePkeParamsToHex(
  fhevm: Fhevm<FhevmChain | undefined, WithEncrypt, OptionalNativeClient>,
  parameters: SerializeGlobalFhePkeParamsToHexParameters,
): Promise<SerializeGlobalFhePkeParamsToHexReturnType> {
  assertGlobalFhePkeParamsOwnedBy(parameters, fhevm.runtime);

  const publicKeyBytes: GlobalFhePublicKeyBytes =
    await fhevm.runtime.encrypt.serializeGlobalFhePublicKey({
      globalFhePublicKey: parameters.publicKey,
    });

  const crsBytes: GlobalFheCrsBytes =
    await fhevm.runtime.encrypt.serializeGlobalFheCrs({
      globalFheCrs: parameters.crs,
    });

  return Object.freeze({
    publicKeyBytesHex: {
      id: publicKeyBytes.id,
      bytesHex: bytesToHexLarge(publicKeyBytes.bytes, false),
    } as GlobalFhePublicKeyBytesHex,
    crsBytesHex: {
      id: crsBytes.id,
      capacity: crsBytes.capacity,
      bytesHex: bytesToHexLarge(crsBytes.bytes, false),
    } as GlobalFheCrsBytesHex,
  });
}
