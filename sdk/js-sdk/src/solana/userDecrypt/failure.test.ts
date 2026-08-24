// The classification, and its coverage of the labels the relayer can actually send.
//
// The label list is the relayer's, so the table is read against it: `V2ErrorLabel` in
// `relayer/src/http/endpoints/v2/types/error.rs` is where the labels are declared, and a label this
// SDK has no action for would be handled by the unknown-label fallback — which gives up. Giving up on
// something the relayer meant as transient is a client that stops one attempt short of working, and
// nothing would report it, hence this test.

import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS,
  SOLANA_USER_DECRYPT_LABEL_ACTIONS,
  classifySolanaUserDecryptRejection,
} from './index.js';

const ERROR_SOURCE = new URL('../../../../../relayer/src/http/endpoints/v2/types/error.rs', import.meta.url);

/** The labels the relayer declares, in the wire form it sends them: `V2ErrorLabel`, snake_cased. */
function relayerLabels(): readonly string[] {
  const source = readFileSync(ERROR_SOURCE, 'utf8');
  const body = /pub enum V2ErrorLabel \{([^}]*)\}/.exec(source)?.[1];
  if (body === undefined) {
    throw new Error('error.rs does not declare pub enum V2ErrorLabel');
  }
  return body
    .split(',')
    .map((entry) => entry.replace(/\/\/.*$/gm, '').trim())
    .filter((entry) => entry.length > 0)
    .map((variant) => variant.replace(/([a-z0-9])([A-Z])/g, '$1_$2').toLowerCase());
}

////////////////////////////////////////////////////////////////////////////////

describe('the label table', () => {
  it('has an action for every label the relayer declares', () => {
    const labels = relayerLabels();
    expect(labels.length).toBeGreaterThan(0);
    for (const label of labels) {
      expect(Object.keys(SOLANA_USER_DECRYPT_LABEL_ACTIONS), `no action for ${label}`).toContain(label);
    }
  });

  it('names no label the relayer does not declare', () => {
    const labels = new Set(relayerLabels());
    for (const label of Object.keys(SOLANA_USER_DECRYPT_LABEL_ACTIONS)) {
      expect(labels, `${label} is not a relayer label`).toContain(label);
    }
  });
});

describe('a refused submission', () => {
  // A request the relayer will refuse identically forever. Retrying costs attempts and teaches the
  // client nothing; the caller has to change the request.
  it.each(['validation_failed', 'malformed_json', 'missing_fields', 'host_chain_id_not_supported'])(
    'gives up on %s',
    (label) => {
      expect(classifySolanaUserDecryptRejection({ kind: 'refused', label })).toEqual({ action: 'give-up' });
    },
  );

  // Nothing about the request is wrong; the service was in a state that passes. `host_acl_failed`
  // is in this group because it is the relayer failing to READ the host ACL (an RPC hiccup), not
  // the ACL refusing the request — the relayer itself declares it retryable.
  it.each(['protocol_paused', 'internal_server_error', 'response_timed_out', 'host_acl_failed'])(
    'retries the same bytes after %s',
    (label) => {
      expect(classifySolanaUserDecryptRejection({ kind: 'refused', label })).toEqual({
        action: 'retry-unchanged',
        afterSeconds: SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS,
      });
    },
  );

  it('gives up on a label it does not know', () => {
    expect(classifySolanaUserDecryptRejection({ kind: 'refused', label: 'a_label_from_a_later_version' })).toEqual({
      action: 'give-up',
    });
  });
});

describe('an overloaded relayer', () => {
  it('is retried with the delay it asked for, not one of our own', () => {
    expect(classifySolanaUserDecryptRejection({ kind: 'overloaded', retryAfterSeconds: 17 })).toEqual({
      action: 'retry-unchanged',
      afterSeconds: 17,
    });
  });

  it('falls back to the default delay when it asks for none', () => {
    expect(classifySolanaUserDecryptRejection({ kind: 'overloaded', retryAfterSeconds: 0 })).toEqual({
      action: 'retry-unchanged',
      afterSeconds: SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS,
    });
  });
});

describe('a job that produced nothing', () => {
  // What a Connector refusal looks like from here. Its two repairable causes — a proof an append moved
  // past, and a handle an update replaced while the request was in flight — are both fixed by
  // resolving the evidence again, so they need not be told apart to be acted on.
  it('resolves the evidence again rather than resubmitting the same bytes', () => {
    expect(classifySolanaUserDecryptRejection({ kind: 'unanswered' })).toEqual({ action: 'resolve-again' });
  });
});

describe('a failed job', () => {
  it('follows the same label table as a refusal', () => {
    expect(classifySolanaUserDecryptRejection({ kind: 'failed', label: 'internal_server_error' })).toEqual({
      action: 'retry-unchanged',
      afterSeconds: SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS,
    });
    expect(classifySolanaUserDecryptRejection({ kind: 'failed', label: 'not_allowed_on_host_acl' })).toEqual({
      action: 'give-up',
    });
  });
});
