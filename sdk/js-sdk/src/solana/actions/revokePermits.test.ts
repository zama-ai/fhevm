import { readFileSync } from 'node:fs';
import { describe, expect, it, vi } from 'vitest';
import { AccountRole, address, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import {
  buildRevokePermitsInstruction,
  solanaPermitInvalidationAddress,
  fetchSolanaPermitInvalidation,
} from './revokePermits.js';
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
    expect(await solanaPermitInvalidationAddress(user, ZAMA_HOST_PROGRAM_ADDRESS)).toBe(WATERMARK_ADDRESS);
  });
});

describe('buildRevokePermitsInstruction', () => {
  it('builds the exact bytes the host program decodes', async () => {
    const instruction = await buildRevokePermitsInstruction({ user, programAddress: ZAMA_HOST_PROGRAM_ADDRESS });
    expect(instruction.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect(hex(instruction.data!)).toBe(REVOKE_PERMITS_DATA);
  });

  it('names the three accounts in program order with their roles', async () => {
    const instruction = await buildRevokePermitsInstruction({ user, programAddress: ZAMA_HOST_PROGRAM_ADDRESS });
    expect(instruction.accounts?.map((account) => [account.address, account.role])).toEqual([
      [user, AccountRole.WRITABLE_SIGNER],
      [WATERMARK_ADDRESS, AccountRole.WRITABLE],
      [SYSTEM_PROGRAM, AccountRole.READONLY],
    ]);
  });
});

it('reads the account fixture produced by the Rust host', async () => {
  const fixture = JSON.parse(
    readFileSync(
      new URL('../../../../../solana/test-fixtures/permit/permit_invalidation_account_v1.json', import.meta.url),
      'utf8',
    ),
  );
  const fixtureUser = address(fixture.fields.find((field: { name: string }) => field.name === 'user').value_base58);
  const send = vi.fn().mockResolvedValue({
    value: {
      owner: fixture.account.owner_base58,
      data: [Buffer.from(fixture.account.data_hex, 'hex').toString('base64'), 'base64'],
      executable: false,
      lamports: 1n,
      space: BigInt(fixture.account.data_len),
    },
  });
  const getAccountInfo = vi.fn().mockReturnValue({ send });
  const rpc = { getAccountInfo } as unknown as Parameters<typeof fetchSolanaPermitInvalidation>[0];
  const watermark = await fetchSolanaPermitInvalidation(rpc, fixtureUser, {
    programAddress: address(fixture.program.id_base58),
  });
  expect(watermark).toBe(BigInt(fixture.produced_by.clock_unix_timestamp));
  expect(getAccountInfo).toHaveBeenCalledWith(address(fixture.address.address_base58), expect.anything());
});
