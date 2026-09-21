import { describe, expect, expectTypeOf, it } from 'vitest';
import {
  AccountRole,
  address,
  appendTransactionMessageInstructions,
  createNoopSigner,
  createTransactionMessage,
  setTransactionMessageFeePayerSigner,
  type Instruction,
} from '@solana/kit';

import {
  INSTRUCTIONS_SYSVAR_ADDRESS,
  appendTransientStoreInstructions,
  prepareTransientStore,
} from './transientStore.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from './internal/generated/zamaHost/programAddress.js';

// Pinned against the host Rust codec and derivation in transient_mollusk.rs.
const payer = createNoopSigner(address('5bV6jUfhDHCQVA1WfKBUnXUsboJgoKgkzkKcxr3joew5'));
const transientStoreAddress = address('7HVhfpvm7TiBwHw8vFNeEqkMCDTU2cWpruEweWsRziAW');

describe('prepareTransientStore', () => {
  it('binds open, the journal PDA, and the final refund to the same canonical transient store', async () => {
    const transientStore = await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
    expect(transientStore.address).toBe(transientStoreAddress);
    const [open, close] = appendTransientStoreInstructions(transientStore, []);
    expect(open!.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect([...open!.data!]).toEqual([54, 100, 76, 213, 84, 233, 196, 94]);
    expect(open!.accounts?.map((meta) => [meta.address, meta.role])).toEqual([
      [payer.address, AccountRole.WRITABLE_SIGNER],
      [transientStoreAddress, AccountRole.WRITABLE],
      [INSTRUCTIONS_SYSVAR_ADDRESS, AccountRole.READONLY],
      ['11111111111111111111111111111111', AccountRole.READONLY],
    ]);
    expect(open!.accounts?.[0]).toHaveProperty('signer', payer);
    expect(close!.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect([...close!.data!]).toEqual([107, 197, 28, 166, 51, 173, 83, 189]);
    expect(close!.accounts?.map((meta) => [meta.address, meta.role])).toEqual([
      [INSTRUCTIONS_SYSVAR_ADDRESS, AccountRole.READONLY],
      [transientStoreAddress, AccountRole.WRITABLE],
      [payer.address, AccountRole.WRITABLE],
    ]);
  });

  it('keeps a custom host consistent across derivation and lifecycle validation', async () => {
    const host = payer.address;
    const transientStore = await prepareTransientStore({ payer, host });
    expect(transientStore.address).not.toBe(transientStoreAddress);
    const sandwiched = appendTransientStoreInstructions(transientStore, []);
    expect(sandwiched.map((ix) => ix.programAddress)).toEqual([host, host]);
    for (const ix of sandwiched)
      expect(ix.accounts?.some((account) => account.address === transientStore.address)).toBe(true);
    expect(() => appendTransientStoreInstructions(transientStore, sandwiched)).toThrow(
      'must not open or close the transient store',
    );
  });

  it('preserves arbitrary application order and rejects nested lifecycle instructions', async () => {
    const transientStore = await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
    const first: Instruction = {
      programAddress: address('11111111111111111111111111111111'),
      data: new Uint8Array([1]),
    };
    const second: Instruction = { ...first, data: new Uint8Array([2]) };
    for (const body of [
      [first, second],
      [second, first],
    ]) {
      const sandwiched = appendTransientStoreInstructions(transientStore, body);
      expect(sandwiched.slice(1, -1)).toEqual(body);
      expect(body).toHaveLength(2);
      expect(() => appendTransientStoreInstructions(transientStore, sandwiched)).toThrow(
        'must not open or close the transient store',
      );
    }
    // A new transaction with the same sponsor deliberately reuses the address, not its contents.
    expect((await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS })).address).toBe(
      transientStore.address,
    );
    const other = await prepareTransientStore({
      payer: createNoopSigner(INSTRUCTIONS_SYSVAR_ADDRESS),
      host: ZAMA_HOST_PROGRAM_ADDRESS,
    });
    expect(other.address).not.toBe(transientStoreAddress);
  });

  it('rejects a hand-built object that was never prepared', () => {
    expect(() => appendTransientStoreInstructions({ address: transientStoreAddress }, [])).toThrow(
      'requires the TransientStore returned by prepareTransientStore',
    );
  });

  it('appends the sandwiched instructions onto a Kit message', async () => {
    const transientStore = await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
    const body: Instruction = {
      programAddress: address('11111111111111111111111111111111'),
      data: new Uint8Array([7]),
    };
    const message = setTransactionMessageFeePayerSigner(payer, createTransactionMessage({ version: 0 }));
    const withStore = appendTransientStoreInstructions(transientStore, [body], message);
    const expected = appendTransactionMessageInstructions(
      appendTransientStoreInstructions(transientStore, [body]),
      message,
    );
    expect(withStore.instructions).toEqual(expected.instructions);
    expectTypeOf(withStore.feePayer).toEqualTypeOf(message.feePayer);
    expect(() => appendTransientStoreInstructions(transientStore, [body], withStore)).toThrow(
      'must not open or close the transient store',
    );
  });
});
