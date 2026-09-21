import { describe, expect, test } from 'bun:test';
import { readFile } from 'node:fs/promises';
import path from 'node:path';

import { REPO_ROOT } from '../../layout';
import { SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT } from '../../layout';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../../../../../solana/deploy/src/generated/zamaHost/programAddress.js';
import { BRINGUP_KMS_CONTEXT_ID } from '../../../../../solana/deploy/src/constants';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import {
  SOLANA_ENVIRONMENTS,
  programIdsFor,
  readSolanaEnvironment,
} from '../../../../../solana/deploy/src/environment';

const PROGRAMS = ['zama_host', 'confidential_token', 'demo_vault', 'confidential_batcher'] as const;

describe('solana environments', () => {
  test('bring-up KMS context id matches the tagged gateway default', () => {
    expect(`0x${Buffer.from(BRINGUP_KMS_CONTEXT_ID).toString('hex')}`).toBe(SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT);
  });

  test('localnet ids are the generated clients; preview-env ids are distinct', () => {
    const localnet = programIdsFor('localnet');
    const preview = programIdsFor('preview-env');
    expect(localnet.zamaHost).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect(localnet.confidentialToken).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    for (const key of Object.keys(localnet) as (keyof typeof localnet)[]) {
      expect(preview[key]).not.toBe(localnet[key]);
    }
  });

  test('readSolanaEnvironment defaults to localnet and rejects unknown values', () => {
    const original = process.env.SOLANA_ENVIRONMENT;
    try {
      delete process.env.SOLANA_ENVIRONMENT;
      expect(readSolanaEnvironment()).toBe('localnet');
      process.env.SOLANA_ENVIRONMENT = 'preview-env';
      expect(readSolanaEnvironment()).toBe('preview-env');
      process.env.SOLANA_ENVIRONMENT = 'mainnet';
      expect(() => readSolanaEnvironment()).toThrow('localnet, preview-env');
    } finally {
      if (original === undefined) delete process.env.SOLANA_ENVIRONMENT;
      else process.env.SOLANA_ENVIRONMENT = original;
    }
  });
});

test('every environment file names the four programs; Anchor.toml matches localnet', async () => {
  const anchor = await readFile(path.join(REPO_ROOT, 'solana/Anchor.toml'), 'utf8');
  const localnetSection = anchor.split('[programs.localnet]')[1]!.split('[')[0]!;
  for (const environment of SOLANA_ENVIRONMENTS) {
    const file = JSON.parse(await readFile(path.join(REPO_ROOT, 'solana/environments', `${environment}.json`), 'utf8'));
    expect(Object.keys(file).sort()).toEqual(['features', 'programs']);
    expect(Object.keys(file.programs).sort()).toEqual([...PROGRAMS].sort());
    if (environment === 'localnet') {
      for (const program of PROGRAMS) expect(localnetSection).toContain(`${program} = "${file.programs[program]}"`);
    }
  }
});

test('preview-env enables admin-sweep on zama-host only', async () => {
  const preview = JSON.parse(
    await readFile(path.join(REPO_ROOT, 'solana/environments/preview-env.json'), 'utf8'),
  );
  const localnet = JSON.parse(
    await readFile(path.join(REPO_ROOT, 'solana/environments/localnet.json'), 'utf8'),
  );
  expect(preview.features).toEqual({ zama_host: ['admin-sweep'] });
  expect(localnet.features).toEqual({});
});

test('the workspace pins PROGRAM_ENVIRONMENT to localnet so a shell export is inert', async () => {
  const config = await readFile(path.join(REPO_ROOT, 'solana/.cargo/config.toml'), 'utf8');
  expect(config).toContain('PROGRAM_ENVIRONMENT = { value = "localnet", force = true }');
});

test('each program takes its id from the environment file, not a literal', async () => {
  for (const program of PROGRAMS) {
    const crate = path.join(REPO_ROOT, 'solana/programs', program.replaceAll('_', '-'));
    expect(await readFile(path.join(crate, 'build.rs'), 'utf8')).toContain(`declare_program_id("${program}")`);
    const source = await readFile(path.join(crate, 'src/lib.rs'), 'utf8');
    expect(source).toContain('include!(concat!(env!("OUT_DIR"), "/program_id.rs"));');
    expect(source).not.toContain('declare_id!("');
  }
});
