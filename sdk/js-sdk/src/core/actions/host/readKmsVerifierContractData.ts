import type { HostContractVersion } from '../../types/hostContract.js';
import { executeWithBatching } from '../../base/promise.js';
import { createKmsVerifierContractData } from '../../host-contracts/KmsVerifierContractData-p.js';
import { assertIsKmsEIP712Domain } from '../../kms/createKmsEIP712Domain.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { KmsVerifierContractData } from '../../types/kms.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import { eip712Domain, type Eip712DomainReturnType } from './eip712Domain.js';
import { assertIsHostContractVersionOf } from '../../host-contracts/HostContractVersion-p.js';
import { getVersion } from './getVersion.js';
import { readKmsSignersContext } from '../../host-contracts/readKmsSignersContext-p.js';
import type { KmsSignersContext } from '../../types/kmsSignersContext.js';

////////////////////////////////////////////////////////////////////////////////

export type ReadKmsVerifierContractDataParameters = {
  readonly address: ChecksummedAddress;
};

export type ReadKmsVerifierContractDataReturnType = KmsVerifierContractData;

////////////////////////////////////////////////////////////////////////////////

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
    () => getVersion(fhevm, parameters),
    () => eip712Domain(fhevm, parameters),
    () => readKmsSignersContext(fhevm, parameters),
  ];

  const res = await executeWithBatching<unknown>(
    rpcCalls,
    fhevm.options.batchRpcCalls,
  );

  const contractVersion = res[0] as HostContractVersion;
  const eip712DomainRes = res[1] as Eip712DomainReturnType;
  const kmsSignersContext = res[2] as KmsSignersContext;

  assertIsHostContractVersionOf(contractVersion, 'KMSVerifier');

  try {
    assertIsKmsEIP712Domain(eip712DomainRes, 'KMSVerifier.eip712Domain()', {});
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
    version: contractVersion,
    address: kmsVerifierContractAddress,
    eip712Domain: eip712DomainRes,
    kmsSignerThreshold: kmsSignersContext.threshold,
    kmsSigners: kmsSignersContext.signers,
  });

  return data;
}
