import type { AbstractEthereumUtils, CleartextAddresses, FhevmAddressesV12 } from './types/public.js';

////////////////////////////////////////////////////////////////////////////////

type FhevmAddressAllocationV12 = {
  readonly fhevmAddresses: FhevmAddressesV12;
  readonly nextStartNonce: bigint;
};

////////////////////////////////////////////////////////////////////////////////

function precomputeFhevmAddressesV12(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly from: `0x${string}`;
  readonly startNonce: bigint;
}): FhevmAddressAllocationV12 {
  // Nonce layout of `deployEmptyProxiesV12`:
  //   +0 EmptyUUPSProxyACL, +1 ACL, +2 EmptyUUPSProxy (shared impl),
  //   +3 FHEVMExecutor, +4 KMSVerifier, +5 InputVerifier, +6 HCULimit.
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

export function precomputeAddresses(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly from: `0x${string}`;
  readonly startNonce: bigint;
}): {
  fhevmAddresses: FhevmAddressesV12;
  cleartextAddresses: CleartextAddresses;
  pauserSetAddress: string;
  nextStartNonce: bigint;
} {
  const { fhevmAddresses, nextStartNonce } = precomputeFhevmAddressesV12(parameters);
  // Cleartext infra proxies follow the v12 core, then PauserSet.
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
