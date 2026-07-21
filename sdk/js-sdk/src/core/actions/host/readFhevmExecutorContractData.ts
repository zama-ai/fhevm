import type { HostContractVersion } from '../../types/hostContract.js';
import type { FhevmExecutorContractData } from '../../types/coprocessor.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { ChecksummedAddress, Uint8Number } from '../../types/primitives.js';
import { assertIsAddress, assertIsChecksummedAddress } from '../../base/address.js';
import { executeWithBatching } from '../../base/promise.js';
import { isUint8 } from '../../base/uint.js';
import { createFhevmExecutorContractData } from '../../host-contracts/FhevmExecutorContractData-p.js';
import { getAclAddress } from '../../host-contracts/getAclAddress-p.js';
import { getHandleVersion } from '../../host-contracts/getHandleVersion-p.js';
import { getHcuLimitAddress } from '../../host-contracts/getHcuLimitAddress-p.js';
import { assertIsHostContractVersionOf } from '../../host-contracts/HostContractVersion-p.js';
import { getHostContractVersion } from '../../host-contracts/HostContractVersion-p.js';
import { getInputVerifierAddress } from '../../host-contracts/getInputVerifierAddress-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

export type ReadFhevmExecutorContractDataParameters = {
  readonly address: ChecksummedAddress;
};
export type ReadFhevmExecutorContractDataReturnType = FhevmExecutorContractData;

export async function readFhevmExecutorContractData(
  fhevm: Fhevm,
  parameters: ReadFhevmExecutorContractDataParameters,
): Promise<ReadFhevmExecutorContractDataReturnType> {
  const fhevmExecutorAddress = parameters.address;
  assertIsAddress(fhevmExecutorAddress, {});

  // no context needed
  await initPublicAction(fhevm);

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
    () => getHostContractVersion(fhevm, parameters),
    () => getAclAddress(fhevm, { fhevmExecutorAddress }),
    () => getHcuLimitAddress(fhevm, { fhevmExecutorAddress }),
    () => getInputVerifierAddress(fhevm, { fhevmExecutorAddress }),
    () => getHandleVersion(fhevm, { fhevmExecutorAddress }),
  ];

  const res = await executeWithBatching<unknown>(rpcCalls, fhevm.options.batchRpcCalls);

  const contractVersion = res[0] as HostContractVersion;
  const aclContractAddress = res[1];
  const hcuLimitContractAddress = res[2];
  const inputVerifierContractAddress = res[3];
  const handleVersion = res[4];

  assertIsHostContractVersionOf(contractVersion, 'FHEVMExecutor');

  if (!isUint8(handleVersion)) {
    throw new Error(`Invalid handle version.`);
  }

  assertIsChecksummedAddress(aclContractAddress, {});
  assertIsChecksummedAddress(hcuLimitContractAddress, {});
  assertIsChecksummedAddress(inputVerifierContractAddress, {});

  const data = createFhevmExecutorContractData(new WeakRef(fhevm.runtime), {
    version: contractVersion,
    address: fhevmExecutorAddress,
    aclContractAddress,
    inputVerifierContractAddress,
    hcuLimitContractAddress,
    handleVersion: Number(handleVersion) as Uint8Number,
  });

  return data;
}
