////////////////////////////////////////////////////////////////////////////////
// resolveFhevmConfig
////////////////////////////////////////////////////////////////////////////////

import {
  addressToChecksummedAddress,
  assertIsAddress,
} from '../../base/address.js';
import { executeWithBatching } from '../../base/promise.js';
import type {
  FhevmExecutorContractData,
  InputVerifierContractData,
} from '../../types/coprocessor.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { KmsVerifierContractData } from '../../types/kms.js';
import type {
  ChecksummedAddress,
  Uint64BigInt,
} from '../../types/primitives.js';
import { getFHEVMExecutorAddress } from './getFHEVMExecutorAddress.js';
import { readFhevmExecutorContractData } from './readFhevmExecutorContractData.js';
import { readInputVerifierContractData } from './readInputVerifierContractData.js';
import { readKmsVerifierContractData } from './readKmsVerifierContractData.js';
import { resolveChainId } from './resolveChainId.js';

type OptionalChainContract = {
  readonly address?: string;
};

export type ResolveFhevmConfigParameters = {
  readonly id?: number | bigint | undefined;
  readonly fhevm: {
    readonly contracts: {
      readonly acl?: OptionalChainContract | undefined;
      readonly fhevmExecutor?: OptionalChainContract | undefined;
      readonly hcuLimit?: OptionalChainContract | undefined;
      readonly inputVerifier?: OptionalChainContract | undefined;
      readonly kmsVerifier: { readonly address: string };
    };
    readonly gateway?:
      | {
          readonly id?: number | bigint | undefined;
          readonly contracts?:
            | {
                readonly decryption?: OptionalChainContract | undefined;
                readonly inputVerification?: OptionalChainContract | undefined;
              }
            | undefined;
        }
      | undefined;
  };
};

export type ResolveFhevmConfigReturnType = {
  readonly id: Uint64BigInt;
  readonly acl: ChecksummedAddress;
  readonly fhevmExecutor: FhevmExecutorContractData;
  readonly inputVerifier: InputVerifierContractData;
  readonly kmsVerifier: KmsVerifierContractData;
};

export async function resolveFhevmConfig(
  fhevm: Fhevm,
  parameters: ResolveFhevmConfigParameters,
): Promise<ResolveFhevmConfigReturnType> {
  // Input is loose
  const kmsVerifierAddress = parameters.fhevm.contracts.kmsVerifier.address;
  assertIsAddress(kmsVerifierAddress, {});

  const id: Uint64BigInt = await resolveChainId(fhevm, parameters);
  const fhevmExecutorData = await _resolveFhevmExecutor(
    fhevm,
    parameters.fhevm.contracts,
  );

  _assertOptionalAddressMatch(
    parameters.fhevm.contracts.inputVerifier?.address,
    fhevmExecutorData.inputVerifierContractAddress,
    'InputVerifier',
  );
  _assertOptionalAddressMatch(
    parameters.fhevm.contracts.hcuLimit?.address,
    fhevmExecutorData.hcuLimitContractAddress,
    'HCULimit',
  );

  const rpcCalls = [
    () =>
      readInputVerifierContractData(fhevm, {
        address: fhevmExecutorData.inputVerifierContractAddress,
      }),
    () =>
      readKmsVerifierContractData(fhevm, {
        address: addressToChecksummedAddress(kmsVerifierAddress),
      }),
  ];

  const res = await executeWithBatching<unknown>(
    rpcCalls,
    fhevm.options.batchRpcCalls,
  );

  const inputVerifierData = res[0] as InputVerifierContractData;
  const kmsVerifierData = res[1] as KmsVerifierContractData;

  _assertOptionalAddressMatch(
    parameters.fhevm.gateway?.contracts?.decryption?.address,
    kmsVerifierData.eip712Domain.verifyingContract,
    'verifyingContractAddressDecryption',
  );

  _assertOptionalAddressMatch(
    parameters.fhevm.gateway?.contracts?.inputVerification?.address,
    inputVerifierData.eip712Domain.verifyingContract,
    'verifyingContractAddressInputVerification',
  );

  _assertOptionalNumericMatch(
    inputVerifierData.eip712Domain.chainId,
    kmsVerifierData.eip712Domain.chainId,
    'gatewayChainId',
  );

  _assertOptionalNumericMatch(
    parameters.fhevm.gateway?.id,
    inputVerifierData.eip712Domain.chainId,
    'gatewayChainId',
  );

  const returnValue: ResolveFhevmConfigReturnType = {
    id,
    acl: fhevmExecutorData.aclContractAddress,
    fhevmExecutor: fhevmExecutorData,
    inputVerifier: inputVerifierData,
    kmsVerifier: kmsVerifierData,
  };

  return Object.freeze(returnValue);
}

////////////////////////////////////////////////////////////////////////////////
// Private Helpers
////////////////////////////////////////////////////////////////////////////////

async function _resolveFhevmExecutor(
  fhevm: Fhevm,
  parameters: {
    readonly acl?: OptionalChainContract | undefined;
    readonly fhevmExecutor?: OptionalChainContract | undefined;
  },
): Promise<FhevmExecutorContractData> {
  const acl = parameters.acl?.address;
  const fhevmExecutor = parameters.fhevmExecutor?.address;

  if (acl !== undefined) {
    assertIsAddress(acl, {});
  }
  if (fhevmExecutor !== undefined) {
    assertIsAddress(fhevmExecutor, {});
  }

  let address;

  if (acl !== undefined) {
    const aclFhevmExecutor = await getFHEVMExecutorAddress(fhevm, {
      address: addressToChecksummedAddress(acl),
    });
    if (fhevmExecutor !== undefined) {
      if (aclFhevmExecutor !== fhevmExecutor) {
        throw new Error(
          `FHEVMExecutor address mismatch: ACL reports ${aclFhevmExecutor}, but ${fhevmExecutor} was provided`,
        );
      }
    }
    address = aclFhevmExecutor;
  } else {
    if (fhevmExecutor === undefined) {
      throw new Error(
        'Cannot resolve: either acl or fhevmExecutor address must be provided',
      );
    }
    address = addressToChecksummedAddress(fhevmExecutor);
  }

  return await readFhevmExecutorContractData(fhevm, { address });
}

function _assertOptionalAddressMatch(
  actual: string | undefined,
  expected: string,
  label: string,
): void {
  if (actual !== undefined) {
    if (actual.toLowerCase() !== expected.toLowerCase()) {
      throw new Error(
        `${label} address mismatch: expected ${expected}, but ${actual} was provided`,
      );
    }
  }
}

function _assertOptionalNumericMatch(
  actual: number | bigint | undefined,
  expected: bigint,
  label: string,
): void {
  if (actual !== undefined) {
    if (BigInt(actual) !== expected) {
      throw new Error(
        `${label} mismatch: expected ${expected}, but ${actual} was provided`,
      );
    }
  }
}
