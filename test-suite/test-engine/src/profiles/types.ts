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

export interface ProfileDefinition {
  apiVersion: typeof PROFILE_API_VERSION;
  kind: 'Profile';
  metadata: { name: string; description: string };
  spec: {
    requiresConfirmation: boolean;
    envFiles: EnvFileEntry[];
    settings: Record<string, ValueSource>;
  };
}

export interface ResolvedSetting {
  value: SettingValue;
  secret: boolean;
  /** Where the value came from: `literal`, `env:NAME`, `env-file:<path>:NAME` or `default`. */
  source: string;
}

/** Plain data only: it is serialised into the worker job and structured-cloned into each World. */
export interface ResolvedProfile {
  name: string;
  description: string;
  requiresConfirmation: boolean;
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
