import { assertIsChecksummedAddressArray } from "../../base/address.js";
import { executeWithBatching } from "../../base/promise.js";
import { asUint8Number, isUint8 } from "../../base/uint.js";
import { createKmsVerifierContractData } from "../../host-contracts/KmsVerifierContractData-p.js";
import { assertIsKmsEIP712Domain } from "../../kms/createKmsEIP712Domain.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { KmsVerifierContractData } from "../../types/kms.js";
import type { ChecksummedAddress } from "../../types/primitives.js";
import { eip712Domain, type Eip712DomainReturnType } from "./eip712Domain.js";
import { getKmsSigners } from "./getKmsSigners.js";
import { getThreshold } from "./getThreshold.js";

export type ReadKmsVerifierContractDataParameters = {
  readonly address: ChecksummedAddress;
};
export type ReadKmsVerifierContractDataReturnType = KmsVerifierContractData;

export async function readKmsVerifierContractData(
  fhevm: Fhevm,
  parameters: ReadKmsVerifierContractDataParameters,
): Promise<ReadKmsVerifierContractDataReturnType> {
  const kmsVerifierContractAddress = parameters.address;

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
    () => getKmsSigners(fhevm, parameters),
  ];

  const res = await executeWithBatching<unknown>(
    rpcCalls,
    fhevm.options?.batchRpcCalls,
  );

  const eip712DomainRes = res[0] as Eip712DomainReturnType;
  const threshold = res[1];
  const kmsSigners = res[2] as unknown[];

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

  try {
    assertIsKmsEIP712Domain(eip712DomainRes, "KMSVerifier.eip712Domain()", {});
  } catch (e) {
    throw new Error(`Invalid KMSVerifier EIP-712 domain.`, { cause: e });
  }

  if (
    eip712DomainRes.verifyingContract.toLowerCase() ===
    kmsVerifierContractAddress.toLowerCase()
  ) {
    throw new Error(
      `Invalid KMSVerifier EIP-712 domain. Unexpected verifyingContract.`,
    );
  }

  const data = createKmsVerifierContractData(new WeakRef(fhevm.runtime), {
    address: kmsVerifierContractAddress,
    eip712Domain: eip712DomainRes,
    kmsSignerThreshold: asUint8Number(Number(threshold)),
    kmsSigners,
  });

  return data;
}
