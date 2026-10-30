import type { ChecksummedAddress, UintNumber } from './primitives.js';

export type HostContractName =
  | 'ACL'
  | 'FHEVMExecutor'
  | 'HCULimit'
  | 'InputVerifier'
  | 'KMSVerifier'
  | 'ProtocolConfig'
  // Non-canonical host chains run a ProtocolConfigReplica at the ProtocolConfig address.
  | 'ProtocolConfigReplica';

export type HostContractVersion<hostContractName extends HostContractName = HostContractName> = {
  readonly contractName: hostContractName;
  readonly major: UintNumber;
  readonly minor: UintNumber;
  readonly patch: UintNumber;
};

export type HostContractData = {
  readonly address: ChecksummedAddress;
};
