import { assertIsChecksummedAddressArray } from "../../base/address.js";
import { executeWithBatching } from "../../base/promise.js";
import { asUint8Number, isUint8 } from "../../base/uint.js";
import { createKmsSignersContext } from "../../host-contracts/KmsSignersContext-p.js";
import type { KmsSignersContext } from "../../types/kmsSignersContext.js";
import { getThreshold } from "../host/getThreshold.js";
import { getKmsSigners } from "../host/getKmsSigners.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type { ChecksummedAddress } from "../../types/primitives.js";

export type ReadKmsSignersContextReturnType = KmsSignersContext;

export async function readKmsSignersContext(
  fhevm: Fhevm<FhevmChain>,
): Promise<ReadKmsSignersContextReturnType> {
  const kmsVerifierContractAddress = fhevm.chain.fhevm.contracts.kmsVerifier
    .address as ChecksummedAddress;

  ////////////////////////////////////////////////////////////////////////////
  //
  // Important remark:
  // =================
  // Do NOTE USE `Promise.all` here!
  // You may get a server response 500 Internal Server Error
  // "Batch of more than 3 requests are not allowed on free tier, to use this
  // feature register paid account at drpc.org"
  //
  ////////////////////////////////////////////////////////////////////////////

  const rpcCalls = [
    () =>
      getThreshold(fhevm, {
        address: kmsVerifierContractAddress,
      }),
    () =>
      getKmsSigners(fhevm, {
        address: kmsVerifierContractAddress,
      }),
  ];

  const res = await executeWithBatching<unknown>(
    rpcCalls,
    fhevm.options?.batchRpcCalls,
  );

  const threshold = res[0];
  const kmsSigners = res[1] as unknown[];

  if (!isUint8(threshold)) {
    throw new Error(`Invalid KMSVerifier kms signers threshold.`);
  }

  try {
    assertIsChecksummedAddressArray(kmsSigners, {});
  } catch (e) {
    throw new Error(`Invalid KMSVerifier kms signers addresses.`, {
      cause: e,
    });
  }

  // No need to verify args, create class directly
  const data = createKmsSignersContext(new WeakRef(fhevm.runtime), {
    address: kmsVerifierContractAddress,
    kmsSigners,
    kmsSignerThreshold: asUint8Number(Number(threshold)),
  });

  return data;
}
