export type SdkFamily = 'fhevm-sdk' | 'relayer-sdk';
export type SdkSource = 'workspace' | 'npm';

export type SdkSelection = {
  family: SdkFamily;
  source: SdkSource;
  requestedVersion: string;
  legacy: boolean;
};

type SelectionEnv = Readonly<Record<string, string | undefined>>;

const explicitKeys = ['E2E_SDK_FAMILY', 'E2E_SDK_SOURCE', 'E2E_SDK_VERSION'] as const;
const exactVersion = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/;

export const LEGACY_SDK_SELECTION_WARNING =
  'Legacy SDK selection is in use. Set E2E_SDK_FAMILY, E2E_SDK_SOURCE, and E2E_SDK_VERSION explicitly.';

export function selectSdk(env: SelectionEnv): SdkSelection {
  const hasExplicitSelection = explicitKeys.some((key) => Boolean(env[key]));
  if (hasExplicitSelection) {
    const family = env.E2E_SDK_FAMILY;
    const source = env.E2E_SDK_SOURCE;
    const version = env.E2E_SDK_VERSION;

    if (family !== 'fhevm-sdk' && family !== 'relayer-sdk') {
      throw new Error('E2E_SDK_FAMILY must be either "fhevm-sdk" or "relayer-sdk"');
    }
    if (source !== 'workspace' && source !== 'npm') {
      throw new Error('E2E_SDK_SOURCE must be either "workspace" or "npm"');
    }
    if (!version) {
      throw new Error('E2E_SDK_VERSION is required when explicit SDK selection is used');
    }
    if (source === 'workspace' && (family !== 'fhevm-sdk' || version !== 'workspace')) {
      throw new Error('workspace SDK selection must use fhevm-sdk with E2E_SDK_VERSION=workspace');
    }
    if (source === 'npm' && version === 'workspace') {
      throw new Error('npm SDK selection requires an exact published version');
    }
    if (source === 'npm' && !exactVersion.test(version)) {
      throw new Error('E2E_SDK_VERSION must be an exact published version when E2E_SDK_SOURCE=npm');
    }

    return { family, source, requestedVersion: version, legacy: false };
  }

  const legacyRelayerVersion = env.RELAYER_SDK_VERSION;
  if (legacyRelayerVersion) {
    if (!exactVersion.test(legacyRelayerVersion)) {
      throw new Error('RELAYER_SDK_VERSION must be an exact published version');
    }
    return {
      family: 'relayer-sdk',
      source: 'npm',
      requestedVersion: legacyRelayerVersion,
      legacy: true,
    };
  }

  return { family: 'fhevm-sdk', source: 'workspace', requestedVersion: 'workspace', legacy: true };
}
