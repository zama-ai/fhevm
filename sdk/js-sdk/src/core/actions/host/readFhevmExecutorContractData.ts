import type { HostContractVersion } from '../../types/hostContract.js';
import { assertIsChecksummedAddress } from '../../base/address.js';
import { executeWithBatching } from '../../base/promise.js';
import { isUint8 } from '../../base/uint.js';
import { createFhevmExecutorContractData } from '../../host-contracts/FhevmExecutorContractData-p.js';
import type { FhevmExecutorContractData } from '../../types/coprocessor.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type {
  ChecksummedAddress,
  Uint8Number,
} from '../../types/primitives.js';
import { getACLAddress } from './getACLAddress.js';
import { getHandleVersion } from './getHandleVersion.js';
import { getHCULimitAddress } from './getHCULimitAddress.js';
import { getInputVerifierAddress } from './getInputVerifierAddress.js';
import { assertIsHostContractVersionOf } from '../../host-contracts/HostContractVersion-p.js';
import { getVersion } from './getVersion.js';

export type ReadFhevmExecutorContractDataParameters = {
  readonly address: ChecksummedAddress;
};
export type ReadFhevmExecutorContractDataReturnType = FhevmExecutorContractData;

export async function readFhevmExecutorContractData(
  fhevm: Fhevm,
  parameters: ReadFhevmExecutorContractDataParameters,
): Promise<ReadFhevmExecutorContractDataReturnType> {
  const fhevmExecutorContractAddress = parameters.address;

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
    () => getVersion(fhevm, parameters),
    () => getACLAddress(fhevm, parameters),
    () => getHCULimitAddress(fhevm, parameters),
    () => getInputVerifierAddress(fhevm, parameters),
    () => getHandleVersion(fhevm, parameters),
  ];

  const res = await executeWithBatching<unknown>(
    rpcCalls,
    fhevm.options.batchRpcCalls,
  );

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
    address: fhevmExecutorContractAddress,
    aclContractAddress,
    inputVerifierContractAddress,
    hcuLimitContractAddress,
    handleVersion: Number(handleVersion) as Uint8Number,
  });

  return data;
}
