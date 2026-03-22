import { assertIsChecksummedAddressArray } from "../../base/address.js";
import { executeWithBatching } from "../../base/promise.js";
import { asUint8Number, isUint8 } from "../../base/uint.js";
import { assertIsCoprocessorEIP712Domain } from "../../coprocessor/assertIsCoprocessorEIP712Domain.js";
import { createInputVerifierContractData } from "../../host-contracts/InputVerifierContractData-p.js";
import type { InputVerifierContractData } from "../../types/coprocessor.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { ChecksummedAddress } from "../../types/primitives.js";
import { eip712Domain, type Eip712DomainReturnType } from "./eip712Domain.js";
import { getCoprocessorSigners } from "./getCoprocessorSigners.js";
import { getThreshold } from "./getThreshold.js";

export type ReadInputVerifierContractDataParameters = {
  readonly address: ChecksummedAddress;
};
export type ReadInputVerifierContractDataReturnType = InputVerifierContractData;

export async function readInputVerifierContractData(
  fhevm: Fhevm,
  parameters: ReadInputVerifierContractDataParameters,
): Promise<InputVerifierContractData> {
  const inputVerifierContractAddress = parameters.address;

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
    () => eip712Domain(fhevm, parameters),
    () => getThreshold(fhevm, parameters),
    () => getCoprocessorSigners(fhevm, parameters),
  ];

  const res = await executeWithBatching<unknown>(
    rpcCalls,
    fhevm.options?.batchRpcCalls,
  );

  const eip712DomainRes = res[0] as Eip712DomainReturnType;
  const threshold = res[1];
  const coprocessorSigners = res[2] as unknown[];

  if (!isUint8(threshold)) {
    throw new Error(`Invalid InputVerifier Coprocessor signers threshold.`);
  }

  try {
    assertIsChecksummedAddressArray(coprocessorSigners, {});
  } catch (e) {
    throw new Error(`Invalid InputVerifier Coprocessor signers addresses.`, {
      cause: e,
    });
  }

  try {
    assertIsCoprocessorEIP712Domain(
      eip712DomainRes,
      "InputVerifier.eip712Domain()",
      {},
    );
  } catch (e) {
    throw new Error(`Invalid InputVerifier EIP-712 domain.`, { cause: e });
  }

  if (
    eip712DomainRes.verifyingContract.toLowerCase() ===
    inputVerifierContractAddress.toLowerCase()
  ) {
    throw new Error(
      `Invalid InputVerifier EIP-712 domain. Unexpected verifyingContract.`,
    );
  }

  const data = createInputVerifierContractData(new WeakRef(fhevm.runtime), {
    address: inputVerifierContractAddress,
    eip712Domain: eip712DomainRes,
    coprocessorSignerThreshold: asUint8Number(Number(threshold)),
    coprocessorSigners,
  });

  return data;
}
