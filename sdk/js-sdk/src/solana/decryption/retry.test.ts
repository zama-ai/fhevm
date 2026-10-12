// The classification, and its coverage of the codes the decryption relayer can actually send.
//
// The code list is read from where the codes are declared: the Connector's `ErrorCode`
// (`kms-connector/crates/api/src/error.rs`), which the relayer forwards, and the relayer's own
// (`relayer-http/crates/relayer-http/src/endpoint/error.rs`). A code this SDK has no action for falls
// to the unknown-code rule, which gives up; giving up on something meant as transient is a client
// that stops one attempt short of working, and nothing would report it, hence this test.

import { readFileSync } from 'node:fs';
import { describe, expect, it, vi } from 'vitest';
import {
  SOLANA_DECRYPTION_CODE_ACTIONS,
  SOLANA_DECRYPTION_DEFAULT_ATTEMPTS,
  SOLANA_DECRYPTION_RETRY_SECONDS,
  SolanaDecryptionRunError,
  classifySolanaDecryptionRejection,
  retrySolanaDecryption,
  type SolanaDecryptionOutcome,
} from './retry.js';

const REPO = new URL('../../../../../', import.meta.url);
const read = (path: string): string => readFileSync(new URL(path, REPO), 'utf8');

/** The Connector's codes, in the wire form it sends them: `ErrorCode`, snake_cased. */
function connectorCodes(): readonly string[] {
  const body = /pub enum ErrorCode \{([^}]*)\}/.exec(read('kms-connector/crates/api/src/error.rs'))?.[1];
  if (body === undefined) throw new Error('error.rs does not declare pub enum ErrorCode');
  return body
    .replace(/\/\/.*$/gm, '')
    .replace(/#\[[^\]]*\]/g, '')
    .split(',')
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0)
    .map((variant) => variant.replace(/([a-z0-9])([A-Z])/g, '$1_$2').toLowerCase());
}

/** The relayer's own codes: every `StatusCode::…, "code"` pair outside its test module. */
function relayerCodes(): readonly string[] {
  const source = read('relayer-http/crates/relayer-http/src/endpoint/error.rs').split('#[cfg(test)]')[0] ?? '';
  return [...source.matchAll(/StatusCode::[A-Z_]+,\s*"([a-z_]+)"/g)].map((match) => match[1] ?? '');
}

describe('the code table', () => {
  it('has an action for every code the relayer can send', () => {
    const codes = [...connectorCodes(), ...relayerCodes()];
    expect(connectorCodes()).toContain('acl_denied');
    expect(relayerCodes()).toContain('shutting_down');
    for (const code of codes) {
      expect(Object.keys(SOLANA_DECRYPTION_CODE_ACTIONS), `no action for ${code}`).toContain(code);
    }
  });

  it('names no code the relayer cannot send', () => {
    const codes = new Set([...connectorCodes(), ...relayerCodes()]);
    for (const code of Object.keys(SOLANA_DECRYPTION_CODE_ACTIONS)) {
      expect(codes, `${code} is not a relayer or Connector code`).toContain(code);
    }
  });
});

describe('the classification', () => {
  // A request refused identically forever: retrying costs attempts and teaches the client nothing.
  it.each(['malformed', 'user_signature_rejected', 'kms_context_destroyed', 'unprocessable', 'internal'])(
    'gives up on %s',
    (code) => {
      expect(classifySolanaDecryptionRejection({ code })).toBe('give-up');
    },
  );

  // Nothing about the request is wrong; a later state of the host or the service passes it.
  it.each(['acl_denied', 'ciphertext_not_found', 'rate_limited', 'upstream_transient', 'timeout', 'shutting_down'])(
    'retries the same bytes after %s',
    (code) => {
      expect(classifySolanaDecryptionRejection({ code })).toBe('retry-unchanged');
    },
  );

  it('gives up on a code it does not know', () => {
    expect(classifySolanaDecryptionRejection({ code: 'a_code_from_a_later_version' })).toBe('give-up');
  });
});

describe('retrySolanaDecryption', () => {
  const scripted = (script: readonly SolanaDecryptionOutcome<string>[]) => {
    let attempt = 0;
    return vi.fn(() => {
      const outcome = script[Math.min(attempt, script.length - 1)];
      attempt += 1;
      if (outcome === undefined) throw new Error('the script is empty');
      return Promise.resolve(outcome);
    });
  };
  const clock = () => ({ delay: vi.fn(() => Promise.resolve()) });
  const denied: SolanaDecryptionOutcome<string> = { ok: false, rejection: { code: 'acl_denied', message: 'no leaf' } };

  it('returns the first answer', async () => {
    const submit = scripted([denied, { ok: true, response: 'answer' }]);
    const { delay } = clock();
    await expect(retrySolanaDecryption({ submit, clock: { delay } })).resolves.toBe('answer');
    expect(submit).toHaveBeenCalledTimes(2);
    expect(delay).toHaveBeenCalledExactlyOnceWith(SOLANA_DECRYPTION_RETRY_SECONDS);
  });

  it('stops at once on a code nothing repairs', async () => {
    const submit = scripted([{ ok: false, rejection: { code: 'malformed', message: 'payload.handles: empty' } }]);
    const { delay } = clock();
    const error = await retrySolanaDecryption({ submit, clock: { delay } }).catch((caught: unknown) => caught);
    expect(error).toBeInstanceOf(SolanaDecryptionRunError);
    expect((error as SolanaDecryptionRunError).attempts).toBe(1);
    expect((error as Error).message).toContain('malformed: payload.handles: empty');
    expect(delay).not.toHaveBeenCalled();
  });

  it('stops at the budget with the last rejection, doubling the wait between attempts', async () => {
    const submit = scripted([denied]);
    const { delay } = clock();
    const error = await retrySolanaDecryption({ submit, clock: { delay } }).catch((caught: unknown) => caught);
    expect((error as SolanaDecryptionRunError).rejection).toEqual(denied.ok ? undefined : denied.rejection);
    expect((error as SolanaDecryptionRunError).attempts).toBe(SOLANA_DECRYPTION_DEFAULT_ATTEMPTS);
    expect(delay.mock.calls).toEqual([
      [SOLANA_DECRYPTION_RETRY_SECONDS],
      [SOLANA_DECRYPTION_RETRY_SECONDS * 2],
      [SOLANA_DECRYPTION_RETRY_SECONDS * 4],
    ]);
  });

  it('honors a smaller budget when one is given', async () => {
    const submit = scripted([denied]);
    const error = await retrySolanaDecryption({ submit, clock: clock(), attempts: 2 }).catch(
      (caught: unknown) => caught,
    );
    expect(submit).toHaveBeenCalledTimes(2);
    expect((error as SolanaDecryptionRunError).attempts).toBe(2);
  });

  it.each([NaN, Infinity, -Infinity, 0, -1, 1.5, Number.MAX_SAFE_INTEGER + 1])(
    'rejects an attempt budget of %s before submitting',
    async (attempts) => {
      const submit = scripted([{ ok: true, response: 'answer' }]);
      await expect(retrySolanaDecryption({ submit, clock: clock(), attempts })).rejects.toThrow(
        'attempts must be a positive safe integer',
      );
      expect(submit).not.toHaveBeenCalled();
    },
  );
});
