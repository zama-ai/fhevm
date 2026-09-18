import type { Address } from '@solana/kit';

import localnet from '../../environments/localnet.json';
import previewEnv from '../../environments/preview-env.json';

/** Deployed environments: `solana/environments/<name>.json`, also the programs' compiled ids. */
export const SOLANA_ENVIRONMENTS = ['localnet', 'preview-env'] as const;
export type SolanaEnvironment = (typeof SOLANA_ENVIRONMENTS)[number];

const ENVIRONMENTS: Record<SolanaEnvironment, { programs: Record<string, string> }> = {
  localnet,
  'preview-env': previewEnv,
};

export type SolanaProgramIds = {
  readonly zamaHost: Address;
  readonly confidentialToken: Address;
  readonly demoVault: Address;
  readonly confidentialBatcher: Address;
};

export const programIdsFor = (environment: SolanaEnvironment): SolanaProgramIds => {
  const programs = ENVIRONMENTS[environment].programs;
  return {
    zamaHost: programs.zama_host as Address,
    confidentialToken: programs.confidential_token as Address,
    demoVault: programs.demo_vault as Address,
    confidentialBatcher: programs.confidential_batcher as Address,
  };
};

export const readSolanaEnvironment = (): SolanaEnvironment => {
  const raw = process.env.SOLANA_ENVIRONMENT ?? 'localnet';
  if (!(SOLANA_ENVIRONMENTS as readonly string[]).includes(raw)) {
    throw new Error(`SOLANA_ENVIRONMENT must be one of ${SOLANA_ENVIRONMENTS.join(', ')}, got "${raw}"`);
  }
  return raw as SolanaEnvironment;
};
