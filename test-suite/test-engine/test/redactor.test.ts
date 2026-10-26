import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { RedactingLineBuffer, createRedactor } from '../src/secrets/redactor.js';

const SECRET = 'p@ss word/"quoted"';

describe('createRedactor', () => {
  const redactor = createRedactor([{ key: 'api-key', value: SECRET }]);

  it('redacts the raw, JSON-escaped and URL-encoded forms', () => {
    assert.equal(redactor.redact(`raw ${SECRET} end`), 'raw [REDACTED:api-key] end');
    assert.equal(redactor.redact(JSON.stringify({ value: SECRET })), '{"value":"[REDACTED:api-key]"}');
    assert.equal(
      redactor.redact(`https://x.invalid/${encodeURIComponent(SECRET)}`),
      'https://x.invalid/[REDACTED:api-key]',
    );
  });

  it('redacts nested objects, arrays and keys, without mutating the input', () => {
    const input = { a: [SECRET, { b: `x${SECRET}y` }], [SECRET]: 1, n: 5, t: true, z: null };
    const output = redactor.redactDeep(input);
    assert.deepEqual(output, {
      a: ['[REDACTED:api-key]', { b: 'x[REDACTED:api-key]y' }],
      '[REDACTED:api-key]': 1,
      n: 5,
      t: true,
      z: null,
    });
    assert.equal(input.a[0], SECRET);
  });

  it('replaces the longest secret first when secrets overlap', () => {
    const overlapping = createRedactor([
      { key: 'short', value: 'abcdefgh' },
      { key: 'long', value: 'abcdefgh-ijklmnop' },
    ]);
    assert.equal(overlapping.redact('abcdefgh-ijklmnop and abcdefgh'), '[REDACTED:long] and [REDACTED:short]');
  });

  it('is inactive without secrets and refuses secrets that are too short', () => {
    const empty = createRedactor([]);
    assert.equal(empty.active, false);
    assert.equal(empty.redact('anything'), 'anything');
    assert.throws(() => createRedactor([{ key: 'k', value: '1234567' }]), /shorter than 8 characters/);
  });
});

describe('RedactingLineBuffer', () => {
  it('redacts a secret split across chunks', () => {
    const written: string[] = [];
    const buffer = new RedactingLineBuffer(createRedactor([{ key: 'k', value: 'split-secret-value' }]), (text) =>
      written.push(text),
    );
    buffer.push('line with split-sec');
    buffer.push('ret-value inside\nnext ');
    buffer.push('line');
    buffer.flush();
    assert.equal(written.join(''), 'line with [REDACTED:k] inside\nnext line');
  });
});
