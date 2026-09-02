import { describe, expect, it } from 'vitest';
import { AccountRole, address, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildRevokePermitsInstruction, solanaPermitInvalidationAddress } from './revokePermits.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../internal/generated/zamaHost/programAddress.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

function hex(bytes: Iterable<number>): string {
  return Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, '0'))
    .join('');
}

// The fixture user of the Rust cross-pin (`sdk_fixture_permit_invalidation_address_and_
// revoke_permits_bytes` in solana/runtime-tests/tests/user_decryption_delegation_mollusk.rs),
// asserted there against the host program's own derivation and codec.
const user = addr(0x44);
const WATERMARK_ADDRESS = '9mDnXemtzZPxnmXJ6ocXABXsmfXwATkWQC9basgU5q2U';
const REVOKE_PERMITS_DATA = '3319597d7d5ac882';
const SYSTEM_PROGRAM = '11111111111111111111111111111111';

describe('solanaPermitInvalidationAddress', () => {
  it('derives the canonical watermark address the host program derives', async () => {
    expect(await solanaPermitInvalidationAddress(user)).toBe(WATERMARK_ADDRESS);
  });
});

describe('buildRevokePermitsInstruction', () => {
  it('builds the exact bytes the host program decodes', async () => {
    const instruction = await buildRevokePermitsInstruction({ user });
    expect(instruction.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect(hex(instruction.data!)).toBe(REVOKE_PERMITS_DATA);
  });

  it('names the three accounts in program order with their roles', async () => {
    const instruction = await buildRevokePermitsInstruction({ user });
    expect(instruction.accounts?.map((account) => [account.address, account.role])).toEqual([
      [user, AccountRole.WRITABLE_SIGNER],
      [WATERMARK_ADDRESS, AccountRole.WRITABLE],
      [SYSTEM_PROGRAM, AccountRole.READONLY],
    ]);
  });
});
