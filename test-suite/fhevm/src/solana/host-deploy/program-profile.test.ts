import { address } from '@solana/kit';
import { describe, expect, test } from 'bun:test';
import { readFile } from 'node:fs/promises';
import path from 'node:path';

import { REPO_ROOT } from '../../layout';
import { SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT } from '../../layout';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../../../../../solana/deploy/src/generated/zamaHost/programAddress.js';
import { BRINGUP_KMS_CONTEXT_ID } from '../../../../../solana/deploy/src/constants';
import {
  PREVIEW_ENV_CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  PREVIEW_ENV_ZAMA_HOST_PROGRAM_ADDRESS,
  programIdsFor,
  readSolanaProgramProfile,
} from '../../../../../solana/deploy/src/program-profile';

describe('solana program profiles', () => {
  test('bring-up KMS context id matches the tagged gateway default', () => {
    expect(`0x${Buffer.from(BRINGUP_KMS_CONTEXT_ID).toString('hex')}`).toBe(SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT);
  });

  test('preview-env ids are stable and distinct from localnet', () => {
    expect(PREVIEW_ENV_ZAMA_HOST_PROGRAM_ADDRESS).not.toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect(PREVIEW_ENV_CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS).not.toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    expect(programIdsFor('localnet')).toEqual({
      zamaHost: ZAMA_HOST_PROGRAM_ADDRESS,
      confidentialToken: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      demoVault: address('6JJTCTipqMjhq5djroouMgi1XZ1Rtc3RMp483F8Bz8b9'),
      confidentialBatcher: address('Cr1Tyzov2Jq9AYVn5zLSLQdyd8CkZJLemHYkj6qDqFmG'),
    });
    expect(programIdsFor('preview-env')).toEqual({
      zamaHost: PREVIEW_ENV_ZAMA_HOST_PROGRAM_ADDRESS,
      confidentialToken: PREVIEW_ENV_CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      demoVault: address('gjgLTv4tB3QBP4RCnHgZvL7GZWP198HuesTA4oM9EdE'),
      confidentialBatcher: address('A2akndg4KnaLBQM3giUrFh89V3c1BUKVinoT6X1179da'),
    });
  });

  test('readSolanaProgramProfile defaults to localnet and rejects unknown values', () => {
    const original = process.env.SOLANA_PROGRAM_PROFILE;
    try {
      delete process.env.SOLANA_PROGRAM_PROFILE;
      expect(readSolanaProgramProfile()).toBe('localnet');
      process.env.SOLANA_PROGRAM_PROFILE = 'preview-env';
      expect(readSolanaProgramProfile()).toBe('preview-env');
      process.env.SOLANA_PROGRAM_PROFILE = 'mainnet';
      expect(() => readSolanaProgramProfile()).toThrow('localnet or preview-env');
    } finally {
      if (original === undefined) delete process.env.SOLANA_PROGRAM_PROFILE;
      else process.env.SOLANA_PROGRAM_PROFILE = original;
    }
  });
});

test('deployment profiles match Anchor and the Rust feature declarations', async () => {
  const anchor = await readFile(path.join(REPO_ROOT, 'solana/Anchor.toml'), 'utf8');
  for (const profile of ['localnet', 'preview-env'] as const) {
    const ids = programIdsFor(profile);
    const section = anchor.split(`[programs.${profile === 'localnet' ? 'localnet' : 'devnet'}]`)[1]!.split('[')[0]!;
    for (const [program, id] of Object.entries({
      zama_host: ids.zamaHost,
      confidential_token: ids.confidentialToken,
      demo_vault: ids.demoVault,
      confidential_batcher: ids.confidentialBatcher,
    })) {
      expect(section).toContain(`${program} = "${id}"`);
      const source = await readFile(
        path.join(REPO_ROOT, 'solana/programs', program.replaceAll('_', '-'), 'src/lib.rs'),
        'utf8',
      );
      const cfg =
        profile === 'preview-env' ? '#[cfg(feature = "preview-env")]' : '#[cfg(not(feature = "preview-env"))]';
      expect(source).toContain(`${cfg}\ndeclare_id!("${id}");`);
    }
  }
});
