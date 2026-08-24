// The advisory layer, and the boundary it turns on.
//
// The wording is pinned, not just the presence of a warning: a warning that fails to say the grant
// follows the signer's delegations has told them the safe half of the truth. The boundary is pinned
// on both sides for the same reason — a week is warned about only when it is exceeded, so a permit
// of exactly a week reads the same to every consumer.

import type { SolanaPermitFields, SolanaPermitWireFields } from './index.js';
import { describe, expect, it } from 'vitest';
import {
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_PERMISSIVE_WARNING,
  PERMIT_TRANSPORT_KEY_LEN,
  PERMIT_WARN_ABOVE_DURATION_SECONDS,
  decodeSolanaPermitFields,
  solanaPermitWarnings,
} from './index.js';

const identity = (fill: number): Uint8Array => new Uint8Array(PERMIT_IDENTITY_LEN).fill(fill);

const routing = (): Uint8Array => {
  const bytes = new Uint8Array(PERMIT_KMS_ROUTING_LEN);
  bytes[0] = PERMIT_KMS_ROUTING_VERSION;
  bytes.set(identity(0x33), 1);
  bytes.set(identity(0x44), 1 + PERMIT_IDENTITY_LEN);
  return bytes;
};

const BASE_WIRE: SolanaPermitWireFields = {
  userPubkey: identity(0x11),
  transportKey: new Uint8Array(PERMIT_TRANSPORT_KEY_LEN),
  allowedAclDomainKeys: [],
  startTimestamp: 1_767_229_380n,
  durationSeconds: PERMIT_WARN_ABOVE_DURATION_SECONDS,
  verifyingProgramId: identity(0x22),
  chainId: 10_037_641_751_006_774_702n,
  extraData: routing(),
};

const fieldsOf = (overrides: Partial<SolanaPermitWireFields>): SolanaPermitFields =>
  decodeSolanaPermitFields({ ...BASE_WIRE, ...overrides });

const A_WEEK = PERMIT_WARN_ABOVE_DURATION_SECONDS;
const SCOPED = [identity(0x01)];

describe('the permissive long-window warning', () => {
  it('is raised for a permissive permit that outlasts a week', () => {
    const warnings = solanaPermitWarnings(fieldsOf({ allowedAclDomainKeys: [], durationSeconds: A_WEEK + 1n }));
    expect(warnings).toEqual([{ code: 'PermissiveLongWindow', message: PERMIT_PERMISSIVE_WARNING }]);
  });

  it('says that the grant follows the signer’s delegations, not only their own handles', () => {
    expect(PERMIT_PERMISSIVE_WARNING).toMatch(/delegation/i);
    expect(PERMIT_PERMISSIVE_WARNING).toMatch(/not only/i);
  });

  it('is not raised at exactly a week: the boundary is exceeded, not reached', () => {
    expect(solanaPermitWarnings(fieldsOf({ allowedAclDomainKeys: [], durationSeconds: A_WEEK }))).toEqual([]);
  });

  it('is not raised for a short permissive permit', () => {
    expect(solanaPermitWarnings(fieldsOf({ allowedAclDomainKeys: [], durationSeconds: 3_600n }))).toEqual([]);
  });

  // A scoped permit states its domains in the text, and the signer can read what they cover; length
  // alone is not the thing worth remarking on.
  it('is not raised for a scoped permit of any length', () => {
    for (const durationSeconds of [3_600n, A_WEEK, A_WEEK + 1n, 31_536_000n]) {
      expect(
        solanaPermitWarnings(fieldsOf({ allowedAclDomainKeys: SCOPED, durationSeconds })),
        `duration ${durationSeconds}`,
      ).toEqual([]);
    }
  });

  it('raises each warning once, however wide the permit is', () => {
    const warnings = solanaPermitWarnings(fieldsOf({ allowedAclDomainKeys: [], durationSeconds: 31_536_000n }));
    expect(warnings).toHaveLength(1);
  });
});
