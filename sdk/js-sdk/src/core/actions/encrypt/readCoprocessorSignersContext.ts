import { assertIsChecksummedAddressArray } from "../../base/address.js";
import { executeWithBatching } from "../../base/promise.js";
import { asUint8Number, isUint8 } from "../../base/uint.js";
import { createCoprocessorSignersContext } from "../../host-contracts/CoprocessorSignersContext-p.js";
import type { CoprocessorSignersContext } from "../../types/coprocessorSignersContext.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type { ChecksummedAddress } from "../../types/primitives.js";
import { getCoprocessorSigners } from "../host/getCoprocessorSigners.js";
import { getThreshold } from "../host/getThreshold.js";

export type ReadCoprocessorSignersContextReturnType = CoprocessorSignersContext;

export async function readCoprocessorSignersContext(
  fhevm: Fhevm<FhevmChain>,
): Promise<ReadCoprocessorSignersContextReturnType> {
  const inputVerifierContractAddress = fhevm.chain.fhevm.contracts.inputVerifier
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
        address: inputVerifierContractAddress,
      }),
    () =>
      getCoprocessorSigners(fhevm, {
        address: inputVerifierContractAddress,
      }),
  ];

  const res = await executeWithBatching<unknown>(
    rpcCalls,
    fhevm.options?.batchRpcCalls,
  );

  const threshold = res[0];
  const coprocessorSigners = res[1] as unknown[];

  if (!isUint8(threshold)) {
    throw new Error(`Invalid InputVerifier coprocessor signers threshold.`);
  }

  try {
    assertIsChecksummedAddressArray(coprocessorSigners, {});
  } catch (e) {
    throw new Error(`Invalid InputVerifier coprocessor signers addresses.`, {
      cause: e,
    });
  }

  // No need to verify args, create class directly
  const data = createCoprocessorSignersContext(new WeakRef(fhevm.runtime), {
    address: inputVerifierContractAddress,
    coprocessorSigners,
    coprocessorSignerThreshold: asUint8Number(Number(threshold)),
  });

  return data;
}
