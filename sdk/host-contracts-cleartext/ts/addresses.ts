import type {
  AbstractEthereumUtils,
  CleartextAddresses,
  FhevmAddressesV12,
  FhevmAddressesV14,
} from './types/public.js';

////////////////////////////////////////////////////////////////////////////////

type FhevmAddressAllocationV12 = {
  readonly fhevmAddresses: FhevmAddressesV12;
  readonly nextStartNonce: bigint;
};

type FhevmAddressAllocationV14 = {
  readonly fhevmAddresses: FhevmAddressesV14;
  readonly nextStartNonce: bigint;
};

////////////////////////////////////////////////////////////////////////////////

function precomputeFhevmAddressesV12(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly from: `0x${string}`;
  readonly startNonce: bigint;
}): FhevmAddressAllocationV12 {
  return {
    fhevmAddresses: {
      aclAddress: parameters.ethUtils.getContractAddress({ from: parameters.from, nonce: parameters.startNonce + 1n }),
      fhevmExecutorAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: parameters.startNonce + 3n,
      }),
      kmsVerifierAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: parameters.startNonce + 4n,
      }),
      inputVerifierAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: parameters.startNonce + 5n,
      }),
      hcuLimitAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: parameters.startNonce + 6n,
      }),
    },
    nextStartNonce: parameters.startNonce + 7n,
  };
}

////////////////////////////////////////////////////////////////////////////////

function precomputeFhevmAddressesV14(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly from: `0x${string}`;
  readonly startNonce: bigint;
}): FhevmAddressAllocationV14 {
  const v12 = precomputeFhevmAddressesV12(parameters);
  return {
    fhevmAddresses: {
      ...v12.fhevmAddresses,
      protocolConfigAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: v12.nextStartNonce + 0n,
      }),
      kmsGenerationAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: v12.nextStartNonce + 1n,
      }),
    },
    nextStartNonce: v12.nextStartNonce + 2n,
  };
}

////////////////////////////////////////////////////////////////////////////////

export function precomputeAddresses(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly from: `0x${string}`;
  readonly startNonce: bigint;
}): {
  fhevmAddresses: FhevmAddressesV14;
  cleartextAddresses: CleartextAddresses;
  pauserSetAddress: string;
  nextStartNonce: bigint;
} {
  const { fhevmAddresses, nextStartNonce } = precomputeFhevmAddressesV14(parameters);
  // Cleartext infra proxies follow the v14 core, then PauserSet.
  const cleartextAddresses: CleartextAddresses = {
    cleartextArithmeticAddress: parameters.ethUtils.getContractAddress({
      from: parameters.from,
      nonce: nextStartNonce,
    }),
    cleartextDbAddress: parameters.ethUtils.getContractAddress({ from: parameters.from, nonce: nextStartNonce + 1n }),
  };
  return {
    fhevmAddresses,
    cleartextAddresses,
    pauserSetAddress: parameters.ethUtils.getContractAddress({ from: parameters.from, nonce: nextStartNonce + 2n }),
    nextStartNonce: nextStartNonce + 3n,
  };
}
