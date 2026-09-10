import { describe, expect, it } from 'vitest';
import { AccountRole, address, createNoopSigner, type Instruction } from '@solana/kit';

import { createSolanaFheTransaction } from './fheTransaction.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from './internal/generated/zamaHost/programAddress.js';

// Pinned against the host Rust codec and derivation in transient_mollusk.rs.
const payer = createNoopSigner(address('5bV6jUfhDHCQVA1WfKBUnXUsboJgoKgkzkKcxr3joew5'));
const scratch = address('7HVhfpvm7TiBwHw8vFNeEqkMCDTU2cWpruEweWsRziAW');
const instructions = address('Sysvar1nstructions1111111111111111111111111');

describe('createSolanaFheTransaction', () => {
  it('binds open, body accounts, and the final refund to the same canonical scratch', async () => {
    const fhe = await createSolanaFheTransaction({ payer });
    expect(fhe.accounts).toEqual({ scratch, instructions });
    const [open, close] = fhe.wrap([]);
    expect(open!.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect([...open!.data!]).toEqual([194, 244, 203, 123, 137, 109, 255, 238]);
    expect(open!.accounts?.map((meta) => [meta.address, meta.role])).toEqual([
      [payer.address, AccountRole.WRITABLE_SIGNER],
      [scratch, AccountRole.WRITABLE],
      [instructions, AccountRole.READONLY],
      ['11111111111111111111111111111111', AccountRole.READONLY],
    ]);
    expect(open!.accounts?.[0]).toHaveProperty('signer', payer);
    expect(close!.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect([...close!.data!]).toEqual([29, 191, 137, 78, 203, 150, 199, 39]);
    expect(close!.accounts?.map((meta) => [meta.address, meta.role])).toEqual([
      [instructions, AccountRole.READONLY],
      [scratch, AccountRole.WRITABLE],
      [payer.address, AccountRole.WRITABLE],
    ]);
  });

  it('preserves arbitrary application order and rejects nested lifecycle instructions', async () => {
    const fhe = await createSolanaFheTransaction({ payer });
    const first: Instruction = {
      programAddress: address('11111111111111111111111111111111'),
      data: new Uint8Array([1]),
    };
    const second: Instruction = { ...first, data: new Uint8Array([2]) };
    for (const body of [
      [first, second],
      [second, first],
    ]) {
      const wrapped = fhe.wrap(body);
      expect(wrapped.slice(1, -1)).toEqual(body);
      expect(body).toHaveLength(2);
      expect(() => fhe.wrap(wrapped)).toThrow('must not open or close scratch');
    }
    // A new transaction with the same sponsor deliberately reuses the address, not its contents.
    expect((await createSolanaFheTransaction({ payer })).accounts).toEqual(fhe.accounts);
    const other = await createSolanaFheTransaction({ payer: createNoopSigner(instructions) });
    expect(other.accounts.scratch).not.toBe(scratch);
  });
});
