import type { ChecksummedAddress, UintNumber } from './primitives.js';

export type HostContractNameMap = {
  readonly ACL: 'ACL';
  readonly FHEVMExecutor: 'FHEVMExecutor';
  readonly HCULimit: 'HCULimit';
  readonly InputVerifier: 'InputVerifier';
  readonly KMSVerifier: 'KMSVerifier';
};

// eslint-disable-next-line @typescript-eslint/no-redundant-type-constituents
export type HostContractName = keyof HostContractNameMap & string;

export type HostContractVersionString =
  `${string} v${string}.${string}.${string}`;

export type HostContractVersion<
  hostContractName extends HostContractName = HostContractName,
> = {
  readonly version: HostContractVersionString;
  readonly contractName: hostContractName;
  readonly major: UintNumber;
  readonly minor: UintNumber;
  readonly patch: UintNumber;
};

export type HostContractData<hostContractName extends HostContractName> = {
  readonly version: HostContractVersion<hostContractName>;
  readonly address: ChecksummedAddress;
};
