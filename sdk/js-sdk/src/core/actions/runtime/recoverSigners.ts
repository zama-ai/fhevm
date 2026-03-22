import { assertIsBytes65HexArray } from "../../base/bytes.js";
import type { RecoverTypedDataAddressModuleFunction } from "../../modules/ethereum/types.js";
import type {
  Fhevm,
  OptionalNativeClient,
} from "../../types/coreFhevmClient.js";
import type { FhevmRuntime } from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";

import type { Bytes65Hex, ChecksummedAddress } from "../../types/primitives.js";

type EIP712TypesType = Record<
  string,
  ReadonlyArray<{ name: string; type: string }>
>;

export type RecoverSignersParameters<T extends EIP712TypesType> = {
  readonly domain: Parameters<
    RecoverTypedDataAddressModuleFunction["recoverTypedDataAddress"]
  >[0]["domain"];
  readonly types: T;
  readonly primaryType: string & keyof T;
  readonly signatures: readonly string[];
  readonly message: Record<string, unknown>;
};

export type RecoverSignersReturnType = ChecksummedAddress[];

export async function recoverSigners<
  T extends Record<string, ReadonlyArray<{ name: string; type: string }>>,
>(
  fhevm: Fhevm<FhevmChain | undefined, FhevmRuntime, OptionalNativeClient>,
  parameters: RecoverSignersParameters<T>,
): Promise<RecoverSignersReturnType> {
  const { domain, types, primaryType, signatures, message } = parameters;
  assertIsBytes65HexArray(signatures, { subject: "signatures" });

  const fields = types[primaryType];
  if (fields === undefined) {
    throw new Error(`Primary type "${primaryType}" not found in types`);
  }

  const recoveredAddresses = await Promise.all(
    signatures.map((signature: Bytes65Hex) =>
      fhevm.runtime.ethereum.recoverTypedDataAddress({
        signature,
        // force cast
        domain,
        primaryType,
        types: {
          [primaryType]: [...fields],
        },
        message,
      }),
    ),
  );

  return recoveredAddresses;
}
