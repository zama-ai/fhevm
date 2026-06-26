import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';
import type { TfheVersion } from '../../wasm/tfhe/TfheApi.js';

export type { TkmsVersion, TfheVersion };

/**
 * Controls how explicit WASM module-version overrides are checked against the
 * resolved protocol compatibility table.
 *
 * This option only applies when a concrete `tfhe` or `kms` version is provided.
 * It does not affect auto-resolution.
 */
export type ModuleVersionCompatibilityCheck = 'throw' | 'warn' | 'off';

export type FhevmEncryptModuleVersions =
  | 'auto'
  | {
      readonly tfhe?: TfheVersion | undefined;
      readonly checkCompatibility?: ModuleVersionCompatibilityCheck | undefined;
    };

export type FhevmDecryptModuleVersions =
  | 'auto'
  | {
      readonly kms?: TkmsVersion | undefined;
      readonly checkCompatibility?: ModuleVersionCompatibilityCheck | undefined;
    };

export type FhevmModuleVersions =
  | 'auto'
  | {
      readonly tfhe?: TfheVersion | undefined;
      readonly kms?: TkmsVersion | undefined;
      readonly checkCompatibility?: ModuleVersionCompatibilityCheck | undefined;
    };
