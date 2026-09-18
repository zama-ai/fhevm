import { describe, expect, test } from 'bun:test';
import { readFile } from 'node:fs/promises';
import path from 'node:path';

import { REPO_ROOT } from '../../layout';
import { SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT } from '../../layout';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../../../../../solana/deploy/src/generated/zamaHost/programAddress.js';
import { BRINGUP_KMS_CONTEXT_ID } from '../../../../../solana/deploy/src/constants';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import {
  DEFAULT_SOLANA_ENVIRONMENT,
  SOLANA_ENVIRONMENTS,
  programIdsFor,
  readSolanaEnvironment,
} from '../../../../../solana/deploy/src/environment';

const PROGRAMS = ['zama_host', 'confidential_token', 'demo_vault', 'confidential_batcher'] as const;

describe('solana environments', () => {
  test('bring-up KMS context id matches the tagged gateway default', () => {
    expect(`0x${Buffer.from(BRINGUP_KMS_CONTEXT_ID).toString('hex')}`).toBe(SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT);
  });

  test('the default environment ids are the ids the generated clients carry', () => {
    const ids = programIdsFor(DEFAULT_SOLANA_ENVIRONMENT);
    expect(ids.zamaHost).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect(ids.confidentialToken).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    expect(new Set(Object.values(ids)).size).toBe(PROGRAMS.length);
  });

  test('readSolanaEnvironment defaults to preview-env and rejects unknown values', () => {
    const original = process.env.SOLANA_ENVIRONMENT;
    try {
      delete process.env.SOLANA_ENVIRONMENT;
      expect(readSolanaEnvironment()).toBe('preview-env');
      process.env.SOLANA_ENVIRONMENT = 'preview-env';
      expect(readSolanaEnvironment()).toBe('preview-env');
      process.env.SOLANA_ENVIRONMENT = 'localnet';
      expect(() => readSolanaEnvironment()).toThrow('preview-env');
    } finally {
      if (original === undefined) delete process.env.SOLANA_ENVIRONMENT;
      else process.env.SOLANA_ENVIRONMENT = original;
    }
  });
});

test('every environment file names the four programs; Anchor.toml carries the default ids', async () => {
  const anchor = await readFile(path.join(REPO_ROOT, 'solana/Anchor.toml'), 'utf8');
  const localnetSection = anchor.split('[programs.localnet]')[1]!.split('[')[0]!;
  for (const environment of SOLANA_ENVIRONMENTS) {
    const file = JSON.parse(await readFile(path.join(REPO_ROOT, 'solana/environments', `${environment}.json`), 'utf8'));
    expect(Object.keys(file).sort()).toEqual(['features', 'programs']);
    expect(Object.keys(file.programs).sort()).toEqual([...PROGRAMS].sort());
    if (environment === DEFAULT_SOLANA_ENVIRONMENT) {
      for (const program of PROGRAMS) expect(localnetSection).toContain(`${program} = "${file.programs[program]}"`);
    }
  }
});
