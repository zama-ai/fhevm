import { concatBytesHex } from "../../../base/bytes.js";
import {
  abiEncodeDecryptedFhevmHandles,
  createDecryptedFhevmHandleArray,
} from "../../../handle/DecryptedFhevmHandle.js";
import { toDecryptedFheValue } from "../../../handle/FheType.js";
import { PublicDecryptionProofImpl } from "../../../kms/PublicDecryptionProof-p.js";
import type { DecryptedFheValue } from "../../../types/decryptedFheValue.js";
import type { SolidityPrimitiveTypeName } from "../../../types/fheType.js";
import type { Fhevm } from "../../../types/coreFhevmClient.js";
import type { FhevmHandle } from "../../../types/fhevmHandle.js";
import type { Bytes65Hex, BytesHex } from "../../../types/primitives.js";
import type { PublicDecryptionProof } from "../../../types/publicDecryptionProof.js";
import { verifyKmsPublicDecryptEIP712 } from "./verifyKmsPublicDecryptEIP712.js";
import type { FhevmChain } from "../../../types/fhevmChain.js";

//////////////////////////////////////////////////////////////////////////////

export type CreatePublicDecryptionProofParameters = {
  readonly orderedHandles: readonly FhevmHandle[];
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly kmsPublicDecryptEIP712Signatures: readonly Bytes65Hex[];
  readonly extraData: BytesHex;
};

export type CreatePublicDecryptionProofReturnType = PublicDecryptionProof;

//////////////////////////////////////////////////////////////////////////////

export async function createPublicDecryptionProof(
  fhevm: Fhevm<FhevmChain>,
  parameters: CreatePublicDecryptionProofParameters,
): Promise<CreatePublicDecryptionProofReturnType> {
  await verifyKmsPublicDecryptEIP712(fhevm, parameters);

  const {
    orderedHandles,
    orderedAbiEncodedClearValues,
    kmsPublicDecryptEIP712Signatures,
    extraData,
  } = parameters;

  //////////////////////////////////////////////////////////////////////////////
  // Compute the proof as numSigners + KMS signatures + extraData
  //////////////////////////////////////////////////////////////////////////////

  const packedNumSigners: BytesHex = fhevm.runtime.ethereum.encodePacked({
    types: ["uint8"],
    values: [kmsPublicDecryptEIP712Signatures.length],
  });

  const packedSignatures = fhevm.runtime.ethereum.encodePacked({
    types: Array(kmsPublicDecryptEIP712Signatures.length).fill(
      "bytes",
    ) as string[],
    values: kmsPublicDecryptEIP712Signatures,
  });

  const decryptionProof: BytesHex = concatBytesHex([
    packedNumSigners,
    packedSignatures,
    extraData,
  ]);

  //////////////////////////////////////////////////////////////////////////////
  // Deserialize ordered decrypted result
  //////////////////////////////////////////////////////////////////////////////

  const orderedAbiTypes: SolidityPrimitiveTypeName[] = orderedHandles.map(
    (h) => h.solidityPrimitiveTypeName,
  );

  const decoded = fhevm.runtime.ethereum.decode({
    types: orderedAbiTypes,
    encodedData: orderedAbiEncodedClearValues,
  });

  if (decoded.length !== orderedHandles.length) {
    throw new Error("Invalid decrypted result.");
  }

  const orderedClearValues: DecryptedFheValue[] = orderedHandles.map(
    (h, index) => toDecryptedFheValue(h.fheType, decoded[index]),
  );

  const originToken: symbol = Symbol("asasa");
  const orderedDecryptedFhevmHandles = createDecryptedFhevmHandleArray(
    orderedHandles,
    orderedClearValues,
    originToken,
  );

  const orderedAbiEncodedDecryptedFhevmHandles = abiEncodeDecryptedFhevmHandles(
    fhevm,
    {
      orderedHandles: orderedDecryptedFhevmHandles,
    },
  );

  return new PublicDecryptionProofImpl({
    decryptionProof: decryptionProof,
    orderedDecryptedHandles: orderedDecryptedFhevmHandles,
    orderedAbiEncodedClearValues:
      orderedAbiEncodedDecryptedFhevmHandles.abiEncodedClearValues,
    extraData,
  });
}
