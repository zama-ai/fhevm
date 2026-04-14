import type { HostContractVersion } from '../../types/hostContract.js';
import type { InputVerifierContractData } from '../../types/coprocessor.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import type { CoprocessorSignersContext } from '../../types/coprocessorSignersContext.js';
import { eip712Domain, type Eip712DomainReturnType } from '../../host-contracts/eip712Domain-p.js';
import { assertIsHostContractVersionOf } from '../../host-contracts/HostContractVersion-p.js';
import { readCoprocessorSignersContext } from '../../host-contracts/readCoprocessorSignersContext-p.js';
import { getVersion } from '../../host-contracts/HostContractVersion-p.js';
import { executeWithBatching } from '../../base/promise.js';
import { assertIsCoprocessorEip712Domain } from '../../coprocessor/assertIsCoprocessorEip712Domain.js';
import { createInputVerifierContractData } from '../../host-contracts/InputVerifierContractData-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ReadInputVerifierContractDataParameters = {
  readonly address: ChecksummedAddress;
};

export type ReadInputVerifierContractDataReturnType = InputVerifierContractData;

////////////////////////////////////////////////////////////////////////////////

export async function readInputVerifierContractData(
  fhevm: Fhevm,
  parameters: ReadInputVerifierContractDataParameters,
): Promise<ReadInputVerifierContractDataReturnType> {
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
    () => getVersion(fhevm, parameters),
    () => eip712Domain(fhevm, parameters),
    () => readCoprocessorSignersContext(fhevm, parameters),
  ];

  const res = await executeWithBatching<unknown>(rpcCalls, fhevm.options.batchRpcCalls);

  const contractVersion = res[0] as HostContractVersion;
  const eip712DomainRes = res[1] as Eip712DomainReturnType;
  const coprocessorSignersContext = res[2] as CoprocessorSignersContext;

  assertIsHostContractVersionOf(contractVersion, 'InputVerifier');

  try {
    assertIsCoprocessorEip712Domain(eip712DomainRes, 'InputVerifier.eip712Domain()', {});
  } catch (e) {
    throw new Error(`Invalid InputVerifier EIP-712 domain.`, { cause: e });
  }

  if (eip712DomainRes.verifyingContract.toLowerCase() === inputVerifierContractAddress.toLowerCase()) {
    throw new Error(`Invalid InputVerifier EIP-712 domain. Unexpected verifyingContract.`);
  }

  const data = createInputVerifierContractData(new WeakRef(fhevm.runtime), {
    version: contractVersion,
    address: inputVerifierContractAddress,
    eip712Domain: eip712DomainRes,
    coprocessorSignerThreshold: coprocessorSignersContext.threshold,
    coprocessorSigners: coprocessorSignersContext.signers,
  });

  return data;
}
