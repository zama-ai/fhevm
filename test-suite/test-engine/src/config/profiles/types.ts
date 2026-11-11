/**
 * Contract of the Profile component: a validated profile definition (as written in
 * `profiles/<name>.yaml`) and the resolved profile handed to the planner and to scenarios.
 */

export const PROFILE_API_VERSION = 'fhevm.zama.ai/profile/v1';
/** Built-in profile used when `--profile` is not given. It offers nothing and has no file. */
export const BUILTIN_PROFILE = 'none';

export type SettingValue = string | number | boolean;

export interface ValueSource {
  value?: SettingValue;
  env?: string;
  default?: SettingValue;
  secret?: boolean;
}

export type EnvFileEntry = string | { path: string; optional?: boolean };

/** Gas limit rule for transactions sent without an explicit `gasLimit` (same as the e2e suite). */
export interface ChainGas {
  bufferPercent: number;
  limitCap: number;
}

/** `spec.chain` as written in the profile file (schema defaults applied). */
export interface ChainSectionDefinition {
  rpcUrl: ValueSource;
  chainId: ValueSource;
  mnemonic: ValueSource;
  hdPath: string;
  /** Wallet role → account index. */
  wallets: Record<string, number>;
  gas: ChainGas;
}

/** `spec.relayer` as written in the profile file: what the FHE module reaches through the relayer. */
export interface RelayerSectionDefinition {
  url: ValueSource;
  /** Optional; an empty value means no authentication. */
  apiKey?: ValueSource;
  gateway: {
    chainId: ValueSource;
    decryption: ValueSource;
    inputVerification: ValueSource;
  };
}

/** `spec.contracts` as written in the profile file: protocol contracts deployed on the host chain. */
export interface ContractsSectionDefinition {
  acl: ValueSource;
  /** The FHEVMExecutor (`CoprocessorAddress` in the Solidity coprocessor config). */
  executor: ValueSource;
  inputVerifier: ValueSource;
  kmsVerifier: ValueSource;
  protocolConfig: ValueSource;
}

export interface ProfileDefinition {
  apiVersion: typeof PROFILE_API_VERSION;
  kind: 'Profile';
  metadata: { name: string; description: string };
  spec: {
    requiresConfirmation: boolean;
    envFiles: EnvFileEntry[];
    /** Typed section of the Chain module. Optional: a profile without it cannot run chain steps. */
    chain?: ChainSectionDefinition;
    /** Typed section of the FHE module. */
    relayer?: RelayerSectionDefinition;
    /** Host-chain protocol contracts, read by the FHE module and by `test-engine compile`. */
    contracts?: ContractsSectionDefinition;
    settings: Record<string, ValueSource>;
  };
}

export interface ResolvedSetting {
  value: SettingValue;
  secret: boolean;
  /** Where the value came from: `literal`, `env:NAME`, `env-file:<path>:NAME` or `default`. */
  source: string;
}

/** A resolved value whose type was checked after resolution. */
export type ResolvedValue<T extends SettingValue> = ResolvedSetting & { value: T };

/** `spec.chain` once resolved and type-checked. */
export interface ResolvedChainSection {
  rpcUrl: ResolvedValue<string>;
  chainId: ResolvedValue<number>;
  mnemonic: ResolvedValue<string>;
  hdPath: string;
  wallets: Record<string, number>;
  gas: ChainGas;
}

/** `spec.relayer` once resolved and type-checked. */
export interface ResolvedRelayerSection {
  url: ResolvedValue<string>;
  /** An empty value means no authentication. */
  apiKey?: ResolvedValue<string>;
  gateway: {
    chainId: ResolvedValue<number>;
    decryption: ResolvedValue<string>;
    inputVerification: ResolvedValue<string>;
  };
}

/** `spec.contracts` once resolved and type-checked. */
export interface ResolvedContractsSection {
  acl: ResolvedValue<string>;
  executor: ResolvedValue<string>;
  inputVerifier: ResolvedValue<string>;
  kmsVerifier: ResolvedValue<string>;
  protocolConfig: ResolvedValue<string>;
}

/** Plain data only: it is serialised into the worker job and structured-cloned into each World. */
export interface ResolvedProfile {
  name: string;
  description: string;
  requiresConfirmation: boolean;
  chain?: ResolvedChainSection;
  relayer?: ResolvedRelayerSection;
  contracts?: ResolvedContractsSection;
  settings: Record<string, ResolvedSetting>;
}

export class ProfileError extends Error {
  constructor(
    /** Profile file path, or the profile name when there is no file. */
    readonly source: string,
    readonly problems: string[],
  ) {
    super(`${source}: ${problems.join('; ')}`);
    this.name = 'ProfileError';
  }
}
