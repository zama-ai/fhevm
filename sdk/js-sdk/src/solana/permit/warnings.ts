// What a signer should be told before signing, beyond what the text already says.
//
// The canonical text always states the breadth of the grant — a permissive permit says so on its own
// line, which no verifier and no wallet can hide. This module is the advisory layer on top: it names
// the one combination that is easy to reach and hard to read the consequences of, a permissive permit
// that lasts longer than a week.
//
// Warnings are returned, not logged and not thrown. A permit is not wrong for carrying one, so the
// decision to show it, block on it or ignore it belongs to the application; a `console.warn` here
// would make that decision for every consumer, and an exception would refuse a permit the protocol
// admits.

import type { SolanaPermitFields } from './types.js';
import { isPermissivePermit } from './types.js';

/** The window past which a permissive permit is worth remarking on: one week. */
export const PERMIT_WARN_ABOVE_DURATION_SECONDS = 604_800n;

/**
 * The wording of the permissive warning.
 *
 * Fixed text rather than a template, and it names delegations explicitly: the breadth a signer
 * underestimates is not the number of their own handles but that the grant follows every delegation
 * currently made to them.
 */
export const PERMIT_PERMISSIVE_WARNING =
  'This permit places no restriction on which ACL domains may be decrypted under it: for as long as ' +
  'it is valid it covers every handle the signer can access, including every delegation currently ' +
  'made to the signer — not only handles of their own. It lasts longer than a week.';

/** An advisory about a permit that is admissible as it stands. */
export type SolanaPermitWarning = {
  readonly code: 'PermissiveLongWindow';
  readonly message: typeof PERMIT_PERMISSIVE_WARNING;
};

/**
 * The warnings a validated permit deserves, in a fixed order.
 *
 * @param fields - Validated permit fields.
 */
export function solanaPermitWarnings(fields: SolanaPermitFields): readonly SolanaPermitWarning[] {
  const warnings: SolanaPermitWarning[] = [];
  // Exceeded, not reached: a permit of exactly a week reads the same to every consumer.
  if (isPermissivePermit(fields) && fields.durationSeconds > PERMIT_WARN_ABOVE_DURATION_SECONDS) {
    warnings.push({ code: 'PermissiveLongWindow', message: PERMIT_PERMISSIVE_WARNING });
  }
  return warnings;
}
