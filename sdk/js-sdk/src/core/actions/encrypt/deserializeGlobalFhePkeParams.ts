import { hexToBytesFaster } from "../../base/bytes.js";
import { createGlobalFhePkeParams } from "../../globalFheKey/GlobalFhePkeParams-p.js";
import type {
  Fhevm,
  OptionalNativeClient,
} from "../../types/coreFhevmClient.js";
import type { WithEncrypt } from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type {
  GlobalFheCrsBytes,
  GlobalFhePkeParams,
  GlobalFhePkeParamsBytes,
  GlobalFhePkeParamsBytesHex,
  GlobalFhePublicKeyBytes,
} from "../../types/globalFhePkeParams.js";

////////////////////////////////////////////////////////////////////////////////

export type DeserializeGlobalFhePkeParamsParameters = GlobalFhePkeParamsBytes;
export type DeserializeGlobalFhePkeParamsReturnType = GlobalFhePkeParams;

export async function deserializeGlobalFhePkeParams(
  fhevm: Fhevm<FhevmChain | undefined, WithEncrypt, OptionalNativeClient>,
  parameters: DeserializeGlobalFhePkeParamsParameters,
): Promise<DeserializeGlobalFhePkeParamsReturnType> {
  const publicKeyNative =
    await fhevm.runtime.encrypt.deserializeGlobalFhePublicKey({
      globalFhePublicKeyBytes: parameters.publicKeyBytes,
    });

  const crsNative = await fhevm.runtime.encrypt.deserializeGlobalFheCrs({
    globalFheCrsBytes: parameters.crsBytes,
  });

  return createGlobalFhePkeParams(new WeakRef(fhevm.runtime), {
    publicKey: publicKeyNative,
    crs: crsNative,
  });
}

////////////////////////////////////////////////////////////////////////////////

export type DeserializeGlobalFhePkeParamsFromHexParameters =
  GlobalFhePkeParamsBytesHex;
export type DeserializeGlobalFhePkeParamsFromHexReturnType = GlobalFhePkeParams;

export async function deserializeGlobalFhePkeParamsFromHex(
  fhevm: Fhevm<FhevmChain | undefined, WithEncrypt, OptionalNativeClient>,
  parameters: DeserializeGlobalFhePkeParamsFromHexParameters,
): Promise<DeserializeGlobalFhePkeParamsFromHexReturnType> {
  const publicKeyNative =
    await fhevm.runtime.encrypt.deserializeGlobalFhePublicKey({
      globalFhePublicKeyBytes: {
        id: parameters.publicKeyBytesHex.id,
        bytes: hexToBytesFaster(parameters.publicKeyBytesHex.bytesHex, {
          strict: true,
        }),
      } as GlobalFhePublicKeyBytes,
    });

  const crsNative = await fhevm.runtime.encrypt.deserializeGlobalFheCrs({
    globalFheCrsBytes: {
      id: parameters.crsBytesHex.id,
      capacity: parameters.crsBytesHex.capacity,
      bytes: hexToBytesFaster(parameters.crsBytesHex.bytesHex, {
        strict: true,
      }),
    } as GlobalFheCrsBytes,
  });

  return createGlobalFhePkeParams(new WeakRef(fhevm.runtime), {
    publicKey: publicKeyNative,
    crs: crsNative,
  });
}
