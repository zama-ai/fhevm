import type { Address } from '@solana/kit';

import preview from '../profiles/preview-env/program-ids.json';
import localnet from './generated/program-ids.json';

export const SOLANA_PROGRAM_PROFILES = ['localnet', 'preview-env'] as const;
export type SolanaProgramProfile = (typeof SOLANA_PROGRAM_PROFILES)[number];

/**
 * Stable program ids for the shared preview-env host on public Solana **devnet**.
 * Local e2e keeps the generated/localnet ids.
 * Keep in sync with solana/deploy/profiles/preview-env/program-ids.json and `declare_id!`
 * behind `--features preview-env`.
 */
export const PREVIEW_ENV_ZAMA_HOST_PROGRAM_ADDRESS = preview.zama_host as Address;
export const PREVIEW_ENV_CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS = preview.confidential_token as Address;

export type SolanaProgramIds = {
  readonly zamaHost: Address;
  readonly confidentialToken: Address;
  readonly demoVault: Address;
  readonly confidentialBatcher: Address;
};

export const programIdsFor = (profile: SolanaProgramProfile): SolanaProgramIds => {
  if (profile === 'preview-env') {
    return {
      zamaHost: PREVIEW_ENV_ZAMA_HOST_PROGRAM_ADDRESS,
      confidentialToken: PREVIEW_ENV_CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      demoVault: preview.demo_vault as Address,
      confidentialBatcher: preview.confidential_batcher as Address,
    };
  }
  return {
    zamaHost: localnet.zama_host as Address,
    confidentialToken: localnet.confidential_token as Address,
    demoVault: localnet.demo_vault as Address,
    confidentialBatcher: localnet.confidential_batcher as Address,
  };
};

export const readSolanaProgramProfile = (): SolanaProgramProfile => {
  const raw = process.env.SOLANA_PROGRAM_PROFILE ?? 'localnet';
  if (raw !== 'localnet' && raw !== 'preview-env') {
    throw new Error(`SOLANA_PROGRAM_PROFILE must be localnet or preview-env, got "${raw}"`);
  }
  return raw;
};
