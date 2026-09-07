import { address } from '@solana/kit';
import { describe, expect, test } from 'bun:test';

import { SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT } from '../../layout';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../internal/generated/zamaHost/programAddress.js';
import { BRINGUP_KMS_CONTEXT_ID } from './constants';
import {
  PREVIEW_ENV_CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  PREVIEW_ENV_ZAMA_HOST_PROGRAM_ADDRESS,
  programIdsFor,
  readSolanaProgramProfile,
} from './program-profile';

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
