import { readFileSync } from 'node:fs';
import { describe, expect, it, vi } from 'vitest';
import {
  AccountRole,
  type Address,
  address,
  appendTransactionMessageInstruction,
  blockhash,
  createTransactionMessage,
  generateKeyPairSigner,
  getAddressDecoder,
  lamports,
  type MaybeEncodedAccount,
  pipe,
  type ProgramDerivedAddressBump,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from '@solana/kit';

import {
  buildRevokePermitsInstruction,
  fetchSolanaPermitInvalidation,
  solanaPermitInvalidationWatermark,
} from './revokePermits.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';

function addr(fill: number): Address {
  return getAddressDecoder().decode(new Uint8Array(32).fill(fill));
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
const WATERMARK_ADDRESS = '37au5bsVuGt2JKKPQLE8hAdkcXf2LjfnmRKBvxt5Yzep';
const REVOKE_PERMITS_DATA = '3319597d7d5ac882';
const SYSTEM_PROGRAM = '11111111111111111111111111111111';

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

  it("signs through Kit when the user's wallet signer pays for the transaction", async () => {
    const wallet = await generateKeyPairSigner();
    const instruction = await buildRevokePermitsInstruction({
      user: wallet,
      programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
    });
    const message = pipe(
      createTransactionMessage({ version: 1 }),
      (message) => setTransactionMessageFeePayerSigner(wallet, message),
      (message) =>
        setTransactionMessageLifetimeUsingBlockhash(
          { blockhash: blockhash('11111111111111111111111111111111'), lastValidBlockHeight: 0n },
          message,
        ),
      (message) => appendTransactionMessageInstruction(instruction, message),
    );
    const signed = await signTransactionMessageWithSigners(message);
    expect(Object.keys(signed.signatures)).toEqual([wallet.address]);
    expect(signed.signatures[wallet.address]).not.toBeNull();
  });
});

const fixture = JSON.parse(
  readFileSync(
    new URL('../../../../../solana/test-fixtures/permit/permit_invalidation_account_v1.json', import.meta.url),
    'utf8',
  ),
);

it('reads the account fixture produced by the Rust host', async () => {
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
  expect(getAccountInfo).toHaveBeenCalledWith(
    address(fixture.address.address_base58),
    expect.objectContaining({ commitment: 'finalized' }),
  );
});

describe('solanaPermitInvalidationWatermark', () => {
  const fixtureUser = address(fixture.fields.find((field: { name: string }) => field.name === 'user').value_base58);
  const programAddress = address(fixture.program.id_base58);
  const recordAddress = address(fixture.address.address_base58);
  const account: MaybeEncodedAccount = {
    exists: true,
    address: recordAddress,
    programAddress,
    data: new Uint8Array(Buffer.from(fixture.account.data_hex, 'hex')),
    executable: false,
    lamports: lamports(1n),
    space: BigInt(fixture.account.data_len),
  };
  const bump = fixture.address.bump as ProgramDerivedAddressBump;

  it('throws on a record naming another user', () => {
    expect(() => solanaPermitInvalidationWatermark(account, [recordAddress, bump], addr(0x45), programAddress)).toThrow(
      'Invalid permit invalidation account',
    );
  });

  it('throws on a record storing a bump that is not the one of its address', () => {
    expect(() =>
      solanaPermitInvalidationWatermark(
        account,
        [recordAddress, (bump - 1) as ProgramDerivedAddressBump],
        fixtureUser,
        programAddress,
      ),
    ).toThrow('Invalid permit invalidation account');
  });
});
