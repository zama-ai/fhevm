import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { EVIDENCE_MEDIA_TYPE, Evidence } from '../src/api/evidence.js';
import { createRedactor } from '../src/secrets/redactor.js';

const fixedNow = () => new Date('2026-10-06T12:00:00.000Z');

describe('Evidence.record', () => {
  it('attaches a redacted JSON payload with the evidence media type', () => {
    const attached: { data: string; mediaType: string }[] = [];
    const evidence = new Evidence(
      (data, options) => {
        attached.push({ data, mediaType: options.mediaType });
      },
      createRedactor([{ key: 'k', value: 'super-secret-value' }]),
      fixedNow,
    );
    evidence.record('chain.tx', { hash: '0xabc', token: 'super-secret-value' }, { label: 'Mint' });
    assert.equal(attached.length, 1);
    assert.equal(attached[0]!.mediaType, EVIDENCE_MEDIA_TYPE);
    assert.deepEqual(JSON.parse(attached[0]!.data), {
      kind: 'chain.tx',
      label: 'Mint',
      data: { hash: '0xabc', token: '[REDACTED:k]' },
      timestamp: '2026-10-06T12:00:00.000Z',
    });
  });

  it('rejects invalid kinds and non-object data', () => {
    const evidence = new Evidence(() => undefined, createRedactor([]), fixedNow);
    assert.throws(() => evidence.record('Not Valid', {}), /evidence kind 'Not Valid' is invalid/);
    assert.throws(() => evidence.record('x', [] as unknown as Record<string, unknown>), /must be a JSON object/);
  });

  it('explains that record() only works inside a step or hook', () => {
    const evidence = new Evidence(
      () => {
        throw new Error(
          'Cannot attach when a step/hook is not running. Ensure your step/hook waits for the attach to finish.',
        );
      },
      createRedactor([]),
      fixedNow,
    );
    assert.throws(() => evidence.record('x', {}), /must be called \(and awaited\) inside a step or hook/);
  });
});
