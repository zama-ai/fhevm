import { describe, expect, it } from 'vitest';
import { AccountRole, address, createNoopSigner, type Instruction } from '@solana/kit';

import { createSolanaFheTransaction } from './fheTransaction.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from './internal/generated/zamaHost/programAddress.js';

// Pinned against the host Rust codec and derivation in transient_mollusk.rs.
const payer = createNoopSigner(address('5bV6jUfhDHCQVA1WfKBUnXUsboJgoKgkzkKcxr3joew5'));
const transientStore = address('7HVhfpvm7TiBwHw8vFNeEqkMCDTU2cWpruEweWsRziAW');
const instructions = address('Sysvar1nstructions1111111111111111111111111');

describe('createSolanaFheTransaction', () => {
  it('binds open, body accounts, and the final refund to the same canonical transient store', async () => {
    const fhe = await createSolanaFheTransaction({ payer });
    expect(fhe.accounts).toEqual({ transientStore, instructions });
    const [open, close] = fhe.wrap([]);
    expect(open!.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect([...open!.data!]).toEqual([54, 100, 76, 213, 84, 233, 196, 94]);
    expect(open!.accounts?.map((meta) => [meta.address, meta.role])).toEqual([
      [payer.address, AccountRole.WRITABLE_SIGNER],
      [transientStore, AccountRole.WRITABLE],
      [instructions, AccountRole.READONLY],
      ['11111111111111111111111111111111', AccountRole.READONLY],
    ]);
    expect(open!.accounts?.[0]).toHaveProperty('signer', payer);
    expect(close!.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect([...close!.data!]).toEqual([107, 197, 28, 166, 51, 173, 83, 189]);
    expect(close!.accounts?.map((meta) => [meta.address, meta.role])).toEqual([
      [instructions, AccountRole.READONLY],
      [transientStore, AccountRole.WRITABLE],
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
      expect(() => fhe.wrap(wrapped)).toThrow('must not open or close the transient store');
    }
    // A new transaction with the same sponsor deliberately reuses the address, not its contents.
    expect((await createSolanaFheTransaction({ payer })).accounts).toEqual(fhe.accounts);
    const other = await createSolanaFheTransaction({ payer: createNoopSigner(instructions) });
    expect(other.accounts.transientStore).not.toBe(transientStore);
  });
});
