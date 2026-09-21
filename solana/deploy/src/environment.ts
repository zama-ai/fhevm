import type { Address } from '@solana/kit';

import previewEnv from '../../environments/preview-env.json';
import { SOLANA_DEPLOY_PROGRAMS } from './constants';

/**
 * Deployed environments: `solana/environments/<name>.json`, also the programs' compiled ids.
 * One id per program serves every cluster (the local test validator loads the same build at
 * genesis); a further Zama is a further file (DD-051, DD-053).
 */
export const SOLANA_ENVIRONMENTS = ['preview-env'] as const;
export type SolanaEnvironment = (typeof SOLANA_ENVIRONMENTS)[number];
export const DEFAULT_SOLANA_ENVIRONMENT: SolanaEnvironment = 'preview-env';

const ENVIRONMENTS: Record<SolanaEnvironment, { programs: Record<string, string> }> = {
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

type DeployedProgram = (typeof SOLANA_DEPLOY_PROGRAMS)[number];

/** The same ids keyed by program crate name. */
export const deployedProgramIds = (environment: SolanaEnvironment): Record<DeployedProgram, Address> => {
  const programs = ENVIRONMENTS[environment].programs;
  const ids = {} as Record<DeployedProgram, Address>;
  for (const program of SOLANA_DEPLOY_PROGRAMS) ids[program] = programs[program] as Address;
  return ids;
};

export const readSolanaEnvironment = (): SolanaEnvironment => {
  const raw = process.env.SOLANA_ENVIRONMENT ?? DEFAULT_SOLANA_ENVIRONMENT;
  if (!(SOLANA_ENVIRONMENTS as readonly string[]).includes(raw)) {
    throw new Error(`SOLANA_ENVIRONMENT must be one of ${SOLANA_ENVIRONMENTS.join(', ')}, got "${raw}"`);
  }
  return raw as SolanaEnvironment;
};
