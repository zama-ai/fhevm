// Point pins for the canonical text, its timestamp field and the transport-key fingerprint.
//
// The vector runner already proves whole texts byte for byte against the canon, which is the
// stronger check — but it cannot say *which* property broke, and it cannot reach the parts of the
// rendering no committed record contains: a zero-valued field, a chain id at the top of u64, a
// timestamp past the typed bound, and the calendar arithmetic across its whole admitted range.
//
// Nothing here reads the vector file. These inputs are built in the test, so a pin that fails is a
// statement about this renderer rather than about a fixture that may have moved.

import type { SolanaPermitFields, SolanaPermitWireFields } from './index.js';
import { shake256 } from '@noble/hashes/sha3.js';
import { base58 } from '@scure/base';
import { describe, expect, it } from 'vitest';
import {
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_MAX_DURATION_SECONDS,
  PERMIT_MAX_START_TIMESTAMP,
  PERMIT_TEXT_HEADER,
  PERMIT_TEXT_PERMISSIVE_DOMAINS_LINE,
  PERMIT_TRANSPORT_KEY_LEN,
  decodeSolanaPermitFields,
  renderPermitTimestamp,
  renderSolanaPermitText,
  transportKeyFingerprint,
} from './index.js';

////////////////////////////////////////////////////////////////////////////////
// Inputs, built here rather than read from the canon
////////////////////////////////////////////////////////////////////////////////

const identity = (fill: number): Uint8Array => new Uint8Array(PERMIT_IDENTITY_LEN).fill(fill);
const transportKeyOf = (fill: number): Uint8Array => new Uint8Array(PERMIT_TRANSPORT_KEY_LEN).fill(fill);

const routingOf = (contextFill: number, epochFill: number): Uint8Array => {
  const bytes = new Uint8Array(PERMIT_KMS_ROUTING_LEN);
  bytes[0] = PERMIT_KMS_ROUTING_VERSION;
  bytes.set(identity(contextFill), 1);
  bytes.set(identity(epochFill), 1 + PERMIT_IDENTITY_LEN);
  return bytes;
};

const BASE_WIRE: SolanaPermitWireFields = {
  userPubkey: identity(0x11),
  transportKey: transportKeyOf(0),
  allowedAclDomainKeys: [],
  startTimestamp: 1_767_229_380n,
  durationSeconds: 604_800n,
  verifyingProgramId: identity(0x22),
  chainId: 10_037_641_751_006_774_702n,
  extraData: routingOf(0x33, 0x44),
};

const fieldsOf = (overrides: Partial<SolanaPermitWireFields> = {}): SolanaPermitFields =>
  decodeSolanaPermitFields({ ...BASE_WIRE, ...overrides });

const linesOf = (fields: SolanaPermitFields): readonly string[] => renderSolanaPermitText(fields).split('\n');

/**
 * The base58 length counterexample pair, in signed order: two 32-byte keys whose base58 forms are 43
 * and 44 characters. Copied from the canon's reference record, because the point of the pair is that
 * these particular byte values straddle the width boundary.
 */
const SHORT_BASE58_KEY = base58.decode('zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz');
const LONG_BASE58_KEY = base58.decode('21111111111111111111111111111111111111111111');

////////////////////////////////////////////////////////////////////////////////

describe('the timestamp field', () => {
  // The two ends of the admitted range, and one instant that pads every component it can.
  const instants: ReadonlyArray<readonly [bigint, string]> = [
    [0n, '1970-01-01T00:00:00Z'],
    [1_767_229_380n, '2026-01-01T01:03:00Z'],
    [951_782_400n, '2000-02-29T00:00:00Z'],
    [1_709_164_800n, '2024-02-29T00:00:00Z'],
    [68_256_000n, '1972-03-01T00:00:00Z'],
    [PERMIT_MAX_START_TIMESTAMP, '9999-12-31T23:59:59Z'],
  ];

  it.each(instants)('renders %s as the instant the canon names', (unixSeconds, expected) => {
    expect(renderPermitTimestamp(unixSeconds)).toBe(expected);
  });

  it('renders to the second, zero-padded, with no fraction and no offset but Z', () => {
    for (const [unixSeconds] of instants) {
      expect(renderPermitTimestamp(unixSeconds)).toMatch(/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$/);
    }
  });

  // Totality is what lets the renderer return a text instead of a result. Strict decoding keeps
  // these values away from a real permit, so this is the only place the claim can be checked — and
  // it is also what guards the calendar arithmetic against overflowing into a throw.
  it.each([PERMIT_MAX_START_TIMESTAMP + 1n, 1_000_000_000_000n, 2n ** 63n, 2n ** 64n - 1n])(
    'stays total at %s, past everything the typed form admits',
    (unixSeconds) => {
      const rendered = renderPermitTimestamp(unixSeconds);
      expect(rendered).toMatch(/^\d{4,}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$/);
    },
  );
});

describe('the calendar the timestamp is rendered through', () => {
  /**
   * The inverse conversion, written independently of the one under test: it walks the calendar
   * forward from a date instead of decomposing a day count. Used only to check that the forward
   * direction lands where it came from.
   *
   * @param year - Calendar year.
   * @param month - Month, 1-12.
   * @param day - Day of month, 1-31.
   */
  function daysFromCivil(year: number, month: number, day: number): number {
    const shiftedYear = year - (month <= 2 ? 1 : 0);
    const era = Math.floor(shiftedYear / 400);
    const yearOfEra = shiftedYear - era * 400;
    const monthPosition = month > 2 ? month - 3 : month + 9;
    const dayOfYear = Math.floor((153 * monthPosition + 2) / 5) + day - 1;
    const dayOfEra = yearOfEra * 365 + Math.floor(yearOfEra / 4) - Math.floor(yearOfEra / 100) + dayOfYear;
    return era * 146_097 + dayOfEra - 719_468;
  }

  const SECONDS_PER_DAY = 86_400;
  const LAST_DAY = Number(PERMIT_MAX_START_TIMESTAMP / BigInt(SECONDS_PER_DAY));

  /**
   * Dense over the first century and a half — where every permit anyone will actually sign lands —
   * then strided by a prime, so the sample is not aligned to any period of the calendar, then dense
   * again over the last years of the range.
   */
  const sample = (): readonly number[] => {
    const days: number[] = [];
    for (let day = 0; day <= 64_000; day += 1) {
      days.push(day);
    }
    for (let day = 64_001; day <= LAST_DAY; day += 1_009) {
      days.push(day);
    }
    for (let day = LAST_DAY - 4_000; day <= LAST_DAY; day += 1) {
      days.push(day);
    }
    return days;
  };

  // The property a table of expected strings cannot give: it visits every month length, every leap
  // year, every century boundary and the year-9999 edge. The same sample the Rust canon walks, so a
  // disagreement about the calendar is a disagreement about the same days.
  it('renders every admitted day as a date that converts back to that day', () => {
    const days = sample();
    expect(days).toHaveLength(70_846);

    for (const day of days) {
      const rendered = renderPermitTimestamp(BigInt(day) * BigInt(SECONDS_PER_DAY));
      const [date] = rendered.split('T');
      const [year, month, dayOfMonth] = (date ?? '').split('-').map(Number);
      expect(month, `day ${day} rendered as ${rendered}`).toBeGreaterThanOrEqual(1);
      expect(month, `day ${day} rendered as ${rendered}`).toBeLessThanOrEqual(12);
      expect(dayOfMonth, `day ${day} rendered as ${rendered}`).toBeGreaterThanOrEqual(1);
      expect(dayOfMonth, `day ${day} rendered as ${rendered}`).toBeLessThanOrEqual(31);
      expect(daysFromCivil(year ?? 0, month ?? 0, dayOfMonth ?? 0), `day ${day} rendered as ${rendered}`).toBe(day);
    }
  });

  it('advances by exactly one day at a time, with no repeats, gaps or backward steps', () => {
    let previous = renderPermitTimestamp(0n).split('T')[0] ?? '';
    for (let day = 1; day <= 64_000; day += 1) {
      const current = renderPermitTimestamp(BigInt(day) * BigInt(SECONDS_PER_DAY)).split('T')[0] ?? '';
      expect(current > previous, `day ${day}: ${previous} then ${current}`).toBe(true);
      const [year, month, dayOfMonth] = current.split('-').map(Number);
      const [previousYear, previousMonth, previousDay] = previous.split('-').map(Number);
      const continuesTheMonth =
        year === previousYear && month === previousMonth && dayOfMonth === (previousDay ?? 0) + 1;
      const startsTheNextMonth = dayOfMonth === 1 && year === previousYear && month === (previousMonth ?? 0) + 1;
      const startsTheNextYear = dayOfMonth === 1 && month === 1 && year === (previousYear ?? 0) + 1;
      expect(
        continuesTheMonth || startsTheNextMonth || startsTheNextYear,
        `day ${day}: ${previous} then ${current}`,
      ).toBe(true);
      previous = current;
    }
  });
});

describe('the canonical text', () => {
  it('opens with the header and carries the fixed lines in order', () => {
    const lines = linesOf(fieldsOf());
    expect(lines[0]).toBe(PERMIT_TEXT_HEADER);
    expect(lines.slice(1, 8).map((line) => line.slice(0, line.indexOf(':') + 1))).toEqual([
      'User:',
      'Verifying program:',
      'Chain id:',
      'Transport key (SHAKE-256):',
      'KMS context:',
      'KMS epoch:',
      'Valid from:',
    ]);
  });

  it('is printable ASCII and line feeds, and its last line carries no line feed', () => {
    const text = renderSolanaPermitText(fieldsOf({ allowedAclDomainKeys: [identity(0x01)] }));
    expect(text).toMatch(/^[\x20-\x7e\n]*$/);
    expect(text.endsWith('\n')).toBe(false);
    expect(text).not.toContain('\r');
    expect(text).not.toContain('\n\n');
  });

  it('names the permissive breadth on one line instead of an empty enumeration', () => {
    const lines = linesOf(fieldsOf({ allowedAclDomainKeys: [] }));
    expect(lines.at(-1)).toBe(PERMIT_TEXT_PERMISSIVE_DOMAINS_LINE);
    expect(lines.filter((line) => line.startsWith('ACL domains ('))).toEqual([]);
    expect(lines.filter((line) => line.startsWith('- '))).toEqual([]);
  });

  it('enumerates a scoped list under its own count, in signed order', () => {
    const lines = linesOf(fieldsOf({ allowedAclDomainKeys: [identity(0x01), identity(0x02)] }));
    expect(lines.slice(-3)).toEqual([
      'ACL domains (2):',
      `- ${base58.encode(identity(0x01))}`,
      `- ${base58.encode(identity(0x02))}`,
    ]);
  });

  it('shows identities in base58, the form a signer sees everywhere else', () => {
    const lines = linesOf(fieldsOf());
    expect(lines[1]).toBe(`User: ${base58.encode(identity(0x11))}`);
    expect(lines[2]).toBe(`Verifying program: ${base58.encode(identity(0x22))}`);
    expect(lines[5]).toBe(`KMS context: ${base58.encode(identity(0x33))}`);
    expect(lines[6]).toBe(`KMS epoch: ${base58.encode(identity(0x44))}`);
  });

  // Base58 is not fixed width, and the renderer does not pad it to look like it is: a verifier that
  // measured these lines instead of reconstructing them would accept one of this pair and not the
  // other.
  it('leaves base58 identities at their natural width', () => {
    const lines = linesOf(fieldsOf({ allowedAclDomainKeys: [SHORT_BASE58_KEY, LONG_BASE58_KEY] }));
    expect(lines.slice(-2).map((line) => line.length - '- '.length)).toEqual([43, 44]);
  });

  it('writes integers in plain decimal, with zero as zero', () => {
    const lines = linesOf(fieldsOf({ chainId: 0n, startTimestamp: 0n, durationSeconds: 1n }));
    expect(lines[3]).toBe('Chain id: 0');
    // No pluralization: one unconditional word, so the line has one form in every implementation.
    expect(lines[7]).toBe('Valid from: 1970-01-01T00:00:00Z for 1 seconds');
  });

  it('writes a chain id above the JavaScript safe-integer range in full', () => {
    const chainId = 2n ** 64n - 1n;
    expect(linesOf(fieldsOf({ chainId }))[3]).toBe(`Chain id: ${chainId}`);
    expect(linesOf(fieldsOf({ durationSeconds: PERMIT_MAX_DURATION_SECONDS }))[7]).toBe(
      `Valid from: 2026-01-01T01:03:00Z for ${PERMIT_MAX_DURATION_SECONDS} seconds`,
    );
  });
});

describe('the transport-key fingerprint', () => {
  it('is the plain 32-byte SHAKE-256 digest of the key, with no domain separator', () => {
    const key = transportKeyOf(0x5a);
    expect(transportKeyFingerprint(key)).toEqual(shake256(key, { dkLen: 32 }));
  });

  it('is what the text commits to', () => {
    const transportKey = transportKeyOf(0x5a);
    expect(linesOf(fieldsOf({ transportKey }))[4]).toBe(
      `Transport key (SHAKE-256): ${base58.encode(transportKeyFingerprint(transportKey))}`,
    );
  });

  // A digest over a prefix of the key would let two different transport keys share one text, which
  // is the substitution the fingerprint exists to prevent.
  it('covers the last byte of the key', () => {
    const key = transportKeyOf(0);
    const flipped = transportKeyOf(0);
    flipped[PERMIT_TRANSPORT_KEY_LEN - 1] = 1;
    expect(transportKeyFingerprint(flipped)).not.toEqual(transportKeyFingerprint(key));
  });
});
