// Canonical text rendering.
//
// A total, deterministic function of validated fields: it takes `SolanaPermitFields`, which cannot
// exist unvalidated, and returns the text — nothing is left to reject, so there is no failure path.
// The text is printable ASCII plus line feeds, its line order is fixed, its final line carries no
// line feed, integers are decimal without leading zeros, and the timestamp is rendered to the
// second.
//
// An empty ACL-domain list renders as one explicit line naming the permissive breadth, rather than
// as an empty enumeration block, so a human signer sees how wide the grant is.
//
// This is the second renderer of one canon; `solana/crates/zama-solana-permit/src/render.rs` is the
// first. The committed vectors are the only thing that makes them one canon rather than two texts
// that happen to look alike, which is why the vector runner is the first test of this module.
//
// The text is assembled as a list of lines joined by a single line feed. That is not a stylistic
// choice: it is what makes "the final line carries no line feed" a property of the construction
// rather than a rule someone has to remember when appending a line.

import type { SolanaPermitFields } from './types.js';
import { base58 } from '@scure/base';
import { transportKeyFingerprint } from './fingerprint.js';
import { PERMIT_KMS_ROUTING_VERSION, isPermissivePermit } from './types.js';

/** First line: names the protocol and the version of this text form. */
export const PERMIT_TEXT_HEADER = 'Zama fhevm Solana user-decrypt permit v1';

/** The one line an empty domain list renders as. */
export const PERMIT_TEXT_PERMISSIVE_DOMAINS_LINE = 'ACL domains: ALL (permissive)';

/**
 * Renders the canonical text a wallet signs.
 *
 * @param fields - Validated permit fields.
 */
export function renderSolanaPermitText(fields: SolanaPermitFields): string {
  const lines: string[] = [
    PERMIT_TEXT_HEADER,
    // Base58, Bitcoin alphabet: the encoding every Solana identity is displayed in, so the text
    // shows a signer the same string their explorer and their wallet do.
    `User: ${base58.encode(fields.userPubkey)}`,
    `Verifying program: ${base58.encode(fields.verifyingProgramId)}`,
    `Chain id: ${fields.chainId}`,
    // The key itself does not fit a wallet screen, so the text commits to a digest of it —
    // recomputed here from the full key, never taken as an input.
    `Transport key (SHAKE-256): ${base58.encode(transportKeyFingerprint(fields.transportKey))}`,
  ];

  // The routing lines belong to the routing version, which is why they are produced by the switch
  // rather than by the template around it. A future version adds an arm that owns its own lines,
  // and has to make them distinguishable from these; it cannot inherit this arm's text by omission.
  switch (fields.kmsRouting.version) {
    // Vacuously true while this is the only routing version; the switch is the exhaustiveness
    // device that stops being vacuous the moment a second version joins the union.
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
    case PERMIT_KMS_ROUTING_VERSION:
      lines.push(`KMS context: ${base58.encode(fields.kmsRouting.kmsContextId)}`);
      lines.push(`KMS epoch: ${base58.encode(fields.kmsRouting.kmsEpochId)}`);
      break;
  }

  lines.push(`Valid from: ${renderPermitTimestamp(fields.startTimestamp)} for ${fields.durationSeconds} seconds`);

  if (isPermissivePermit(fields)) {
    lines.push(PERMIT_TEXT_PERMISSIVE_DOMAINS_LINE);
  } else {
    lines.push(`ACL domains (${fields.allowedAclDomainKeys.length}):`);
    for (const key of fields.allowedAclDomainKeys) {
      lines.push(`- ${base58.encode(key)}`);
    }
  }

  return lines.join('\n');
}

const SECONDS_PER_MINUTE = 60n;
const SECONDS_PER_HOUR = 60n * SECONDS_PER_MINUTE;
const SECONDS_PER_DAY = 24n * SECONDS_PER_HOUR;

/**
 * Renders unix seconds as `YYYY-MM-DDTHH:MM:SSZ`.
 *
 * Total over every non-negative value: past the typed form's bound the year field simply grows
 * wider, so there is no input for which this fails — the bound is what keeps the width fixed, not
 * what keeps the function defined. Exported because the width and the padding are normative pins
 * of their own, testable without assembling a whole permit.
 *
 * @param unixSeconds - Seconds since the epoch.
 */
export function renderPermitTimestamp(unixSeconds: bigint): string {
  const [year, month, day] = civilFromDays(unixSeconds / SECONDS_PER_DAY);
  const secondsOfDay = unixSeconds % SECONDS_PER_DAY;
  const hour = secondsOfDay / SECONDS_PER_HOUR;
  const minute = (secondsOfDay % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
  const second = secondsOfDay % SECONDS_PER_MINUTE;

  return `${pad(year, 4)}-${pad(month, 2)}-${pad(day, 2)}T${pad(hour, 2)}:${pad(minute, 2)}:${pad(second, 2)}Z`;
}

/**
 * A decimal field of the timestamp, zero-padded to its column width.
 *
 * @param value - The field's value; non-negative.
 * @param width - The column width the value is padded to; a wider value keeps its own width.
 */
function pad(value: bigint, width: number): string {
  return value.toString().padStart(width, '0');
}

/**
 * Converts days since 1970-01-01 into a proleptic Gregorian calendar date.
 *
 * Howard Hinnant's `civil_from_days`, restricted to non-negative days — which is all the typed form
 * admits, since the validity window starts at or after the epoch. Written out rather than taken
 * from a date library because five implementations have to agree on this arithmetic, and a shared
 * 40-line algorithm is easier to agree on than five libraries' opinions about calendars, locales
 * and leap seconds. Unix time has no leap seconds, so every day here is exactly 86400 seconds long.
 *
 * The constants are the algorithm's: 146097 days per 400-year era, 719468 days from 0000-03-01 to
 * 1970-01-01, and the 153/5 pair that walks a March-first month table. Every quotient here is of
 * non-negative values, so bigint's truncating division is the floor the algorithm asks for.
 *
 * @param days - Days since the epoch; non-negative.
 */
function civilFromDays(days: bigint): readonly [year: bigint, month: bigint, day: bigint] {
  // Shift the epoch to 0000-03-01, so that a leap day lands at the end of a year and the month
  // table needs no special case for February.
  const shifted = days + 719_468n;
  const era = shifted / 146_097n;
  const dayOfEra = shifted - era * 146_097n;
  const yearOfEra = (dayOfEra - dayOfEra / 1_460n + dayOfEra / 36_524n - dayOfEra / 146_096n) / 365n;
  const dayOfYear = dayOfEra - (365n * yearOfEra + yearOfEra / 4n - yearOfEra / 100n);
  const monthPosition = (5n * dayOfYear + 2n) / 153n;
  const day = dayOfYear - (153n * monthPosition + 2n) / 5n + 1n;

  // Back from the March-first year to the calendar year.
  const month = monthPosition < 10n ? monthPosition + 3n : monthPosition - 9n;
  const year = yearOfEra + era * 400n + (month <= 2n ? 1n : 0n);

  return [year, month, day];
}
