// Type-level pins for the permit's surface.
//
// Two rules of this module are enforced by the type system before any test runs, and this file is
// where that claim is checked rather than assumed:
//
// * A u64 permit field never arrives as a `number`. Above 2^53 a `number` silently changes value,
//   and a field that changed value renders a text the wallet's signature does not cover. The
//   runtime rejects one too — belt and braces, for callers arriving from untyped JavaScript — but
//   the point of the type is that such a call never compiles in the first place.
// * Validated fields cannot be fabricated. The brand is what makes `decodeSolanaPermitFields` the
//   only door into the renderer, so a caller cannot hand the renderer fields no rule ever saw.

import type { SolanaKmsRouting, SolanaPermitFields, SolanaPermitWireFields } from './index.js';
import { expectTypeOf } from 'vitest';
import { decodeSolanaPermitFields, renderSolanaPermitText } from './index.js';

const identity = new Uint8Array(32);

const wire = {
  userPubkey: identity,
  transportKey: new Uint8Array(869),
  allowedAclDomainKeys: [],
  startTimestamp: 1_767_229_380n,
  durationSeconds: 604_800n,
  verifyingProgramId: identity,
  chainId: '10037641751006774702',
  extraData: new Uint8Array(65),
} as const satisfies SolanaPermitWireFields;

////////////////////////////////////////////////////////////////////////////////
// u64 fields take a bigint or a decimal string, and nothing else
////////////////////////////////////////////////////////////////////////////////

expectTypeOf<SolanaPermitWireFields['chainId']>().toEqualTypeOf<bigint | string>();
expectTypeOf<SolanaPermitWireFields['startTimestamp']>().toEqualTypeOf<bigint | string>();
expectTypeOf<SolanaPermitWireFields['durationSeconds']>().toEqualTypeOf<bigint | string>();

// @ts-expect-error A chain id above 2^53 loses value as a number, so no number is accepted at all.
decodeSolanaPermitFields({ ...wire, chainId: 10037641751006774702 });

// @ts-expect-error Same for the window start.
decodeSolanaPermitFields({ ...wire, startTimestamp: 1767229380 });

// @ts-expect-error And for its length.
decodeSolanaPermitFields({ ...wire, durationSeconds: 604800 });

////////////////////////////////////////////////////////////////////////////////
// The validated form has one producer
////////////////////////////////////////////////////////////////////////////////

expectTypeOf(decodeSolanaPermitFields(wire)).toEqualTypeOf<SolanaPermitFields>();

// @ts-expect-error The brand cannot be satisfied from outside the module, so validated fields
// cannot be assembled by hand — not even from values that would pass every rule.
const fabricated: SolanaPermitFields = {
  userPubkey: identity,
  transportKey: new Uint8Array(869),
  allowedAclDomainKeys: [],
  startTimestamp: 1_767_229_380n,
  durationSeconds: 604_800n,
  verifyingProgramId: identity,
  chainId: 10_037_641_751_006_774_702n,
  kmsRouting: { version: 2, kmsContextId: identity, kmsEpochId: identity },
};
void fabricated;

// @ts-expect-error The renderer takes validated fields; the wire form is not one of them.
renderSolanaPermitText(wire);

////////////////////////////////////////////////////////////////////////////////
// A second routing version has to announce itself
////////////////////////////////////////////////////////////////////////////////

// The literal, not `number`: a consumer switching on it stops compiling when a version is added,
// which is what keeps two versions from sharing one canonical text.
expectTypeOf<SolanaKmsRouting['version']>().toEqualTypeOf<2>();
