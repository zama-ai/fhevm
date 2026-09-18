import { describe, expect, test } from 'bun:test';
import { readFile } from 'node:fs/promises';
import path from 'node:path';

import { REPO_ROOT } from '../../layout';
import { SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT } from '../../layout';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../../../../../solana/deploy/src/generated/zamaHost/programAddress.js';
import { BRINGUP_KMS_CONTEXT_ID, SOLANA_DEPLOY_PROGRAMS } from '../../../../../solana/deploy/src/constants';
import {
  DEFAULT_SOLANA_ENVIRONMENT,
  SOLANA_ENVIRONMENTS,
  deployedProgramIds,
  programIdsFor,
  readSolanaEnvironment,
} from '../../../../../solana/deploy/src/environment';

describe('solana environments', () => {
  test('bring-up KMS context id matches the tagged gateway default', () => {
    expect(`0x${Buffer.from(BRINGUP_KMS_CONTEXT_ID).toString('hex')}`).toBe(SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT);
  });

  test('the default environment ids are the ids the generated clients carry', () => {
    const ids = programIdsFor(DEFAULT_SOLANA_ENVIRONMENT);
    expect(ids.zamaHost).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect(ids.confidentialToken).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    expect(new Set(Object.values(ids)).size).toBe(SOLANA_DEPLOY_PROGRAMS.length);
    const byName = deployedProgramIds(DEFAULT_SOLANA_ENVIRONMENT);
    expect(byName.zama_host).toBe(ids.zamaHost);
    expect(byName.confidential_token).toBe(ids.confidentialToken);
    expect(byName.demo_vault).toBe(ids.demoVault);
    expect(byName.confidential_batcher).toBe(ids.confidentialBatcher);
  });

  test('readSolanaEnvironment defaults to preview-env and rejects unknown values', () => {
    const original = process.env.SOLANA_ENVIRONMENT;
    try {
      delete process.env.SOLANA_ENVIRONMENT;
      expect(readSolanaEnvironment()).toBe('preview-env');
      process.env.SOLANA_ENVIRONMENT = 'preview-env';
      expect(readSolanaEnvironment()).toBe('preview-env');
      process.env.SOLANA_ENVIRONMENT = 'localnet';
      expect(() => readSolanaEnvironment()).toThrow('must be one of preview-env, got "localnet"');
    } finally {
      if (original === undefined) delete process.env.SOLANA_ENVIRONMENT;
      else process.env.SOLANA_ENVIRONMENT = original;
    }
  });
});

test('every environment file names the four programs; Anchor.toml carries the default ids', async () => {
  const anchor = await readFile(path.join(REPO_ROOT, 'solana/Anchor.toml'), 'utf8');
  const anchorLocalnetSection = anchor.split('[programs.localnet]')[1]!.split('[')[0]!;
  for (const environment of SOLANA_ENVIRONMENTS) {
    const file = JSON.parse(await readFile(path.join(REPO_ROOT, 'solana/environments', `${environment}.json`), 'utf8'));
    expect(Object.keys(file).sort()).toEqual(['features', 'programs']);
    expect(Object.keys(file.programs).sort()).toEqual([...SOLANA_DEPLOY_PROGRAMS].sort());
    if (environment === DEFAULT_SOLANA_ENVIRONMENT) {
      for (const program of SOLANA_DEPLOY_PROGRAMS) {
        expect(anchorLocalnetSection).toContain(`${program} = "${file.programs[program]}"`);
      }
    }
  }
});

test('preview-env enables admin-sweep on zama-host only', async () => {
  const preview = JSON.parse(await readFile(path.join(REPO_ROOT, 'solana/environments/preview-env.json'), 'utf8'));
  expect(preview.features).toEqual({ zama_host: ['admin-sweep'] });
});

test('the workspace pins PROGRAM_ENVIRONMENT to the default so a shell export is inert', async () => {
  const config = await readFile(path.join(REPO_ROOT, 'solana/.cargo/config.toml'), 'utf8');
  expect(config).toContain(`PROGRAM_ENVIRONMENT = { value = "${DEFAULT_SOLANA_ENVIRONMENT}", force = true }`);
});

test('each program takes its id from the environment file, not a literal', async () => {
  for (const program of SOLANA_DEPLOY_PROGRAMS) {
    const crate = path.join(REPO_ROOT, 'solana/programs', program.replaceAll('_', '-'));
    expect(await readFile(path.join(crate, 'build.rs'), 'utf8')).toContain(`declare_program_id("${program}")`);
    const source = await readFile(path.join(crate, 'src/lib.rs'), 'utf8');
    expect(source).toContain('include!(concat!(env!("OUT_DIR"), "/program_id.rs"));');
    expect(source).not.toContain('declare_id!("');
  }
});
