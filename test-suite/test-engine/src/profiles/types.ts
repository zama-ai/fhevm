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

export interface ProfileDefinition {
  apiVersion: typeof PROFILE_API_VERSION;
  kind: 'Profile';
  metadata: { name: string; description: string };
  spec: {
    requiresConfirmation: boolean;
    envFiles: EnvFileEntry[];
    /** Typed section of the Chain module. Optional: a profile without it cannot run chain steps. */
    chain?: ChainSectionDefinition;
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

/** Plain data only: it is serialised into the worker job and structured-cloned into each World. */
export interface ResolvedProfile {
  name: string;
  description: string;
  requiresConfirmation: boolean;
  chain?: ResolvedChainSection;
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
