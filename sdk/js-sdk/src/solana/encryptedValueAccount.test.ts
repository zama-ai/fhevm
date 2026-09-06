// The hand-rolled EncryptedValue account decoder, pinned byte by byte.
//
// There is no IDL to generate this from, so the layout lives in two places by construction: the
// crate's struct and this decoder. What these tests pin is everything that keeps that duplication
// honest — the field order, the realloc rule (trailing capacity only in whole 32-byte elements),
// and the MMR invariant (as many peaks as the leaf count has set bits), which is checked
// independently so a misaligned decode fails loudly instead of returning shifted fields.

import { describe, expect, it } from 'vitest';
import { getProgramDerivedAddress, type Address } from '@solana/kit';
import { base58 } from '@scure/base';
import {
  SOLANA_ENCRYPTED_VALUE_SEED,
  decodeSolanaEncryptedValueState,
  fetchSolanaEncryptedValueState,
  solanaEncryptedValueAccountAddress,
  type SolanaRpc,
} from './encryptedValueAccount.js';

////////////////////////////////////////////////////////////////////////////////
// Account bytes, built the way the program writes them
////////////////////////////////////////////////////////////////////////////////

const bytes32 = (fill: number): Uint8Array => new Uint8Array(32).fill(fill);

/** `sha256("account:EncryptedValue")[..8]` — the same pin the decoder matches. */
const ENCRYPTED_VALUE_DISCRIMINATOR = new Uint8Array([0x9b, 0x03, 0x95, 0x3a, 0x84, 0x67, 0xc8, 0xa1]);

const u32LE = (value: number): Uint8Array => {
  const out = new Uint8Array(4);
  new DataView(out.buffer).setUint32(0, value, true);
  return out;
};

const u64LE = (value: bigint): Uint8Array => {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, value, true);
  return out;
};

function concat(...parts: readonly Uint8Array[]): Uint8Array {
  const out = new Uint8Array(parts.reduce((length, part) => length + part.length, 0));
  let offset = 0;
  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }
  return out;
}

/** Serializes an account exactly as borsh does, plus optional realloc trailing capacity. */
function accountData(state: {
  readonly leafCount?: bigint;
  readonly peaks?: readonly Uint8Array[];
  readonly trailingBytes?: number;
}): Uint8Array {
  const leafCount = state.leafCount ?? 3n;
  const peaks = state.peaks ?? [bytes32(0x71), bytes32(0x72)];
  return concat(
    ENCRYPTED_VALUE_DISCRIMINATOR, // matched by the decoder before anything else
    bytes32(0x11), // program
    bytes32(0x22), // encrypted value account authority
    bytes32(0x33), // scope
    bytes32(0x44), // label
    bytes32(0x55), // current handle
    u64LE(leafCount),
    u32LE(peaks.length),
    ...peaks,
    new Uint8Array([0xfe]), // bump
    new Uint8Array(state.trailingBytes ?? 0).fill(0x00),
  );
}

////////////////////////////////////////////////////////////////////////////////

describe('decoding an EncryptedValue account', () => {
  it('returns every field of a well-formed account', () => {
    const state = decodeSolanaEncryptedValueState(accountData({}), 'the fixture account');

    expect(state.program).toBe(base58.encode(bytes32(0x11)));
    expect(state.encryptedValueAccountAuthority).toBe(base58.encode(bytes32(0x22)));
    expect(state.scope).toEqual(bytes32(0x33));
    expect(state.label).toEqual(bytes32(0x44));
    expect(state.currentHandle).toEqual(bytes32(0x55));
    expect(state.leafCount).toBe(3n);
    expect(state.peaks).toEqual([bytes32(0x71), bytes32(0x72)]);
    expect(state.bump).toBe(0xfe);
  });

  // The account realloc-grows and never shrinks: a shorter live value leaves stale capacity after
  // it, always in whole 32-byte vector elements.
  it('accepts trailing realloc capacity in whole 32-byte elements', () => {
    const state = decodeSolanaEncryptedValueState(accountData({ trailingBytes: 64 }), 'the fixture account');
    expect(state.leafCount).toBe(3n);
  });

  it('rejects trailing capacity that is not whole elements — the layout has drifted', () => {
    expect(() => decodeSolanaEncryptedValueState(accountData({ trailingBytes: 31 }), 'the fixture account')).toThrow(
      'drifted',
    );
  });

  // The MMR invariant is checked independently of the borsh walk: as many peaks as the leaf count
  // has set bits. A decoder that misaligned on a field would fail here instead of returning
  // shifted fields as if they were real.
  it('rejects a peak count that does not match the leaf count', () => {
    const data = accountData({ leafCount: 3n, peaks: [bytes32(0x71)] });
    expect(() => decodeSolanaEncryptedValueState(data, 'the fixture account')).toThrow('drifted');
  });

  it('rejects an account of another type by its discriminator', () => {
    const data = accountData({});
    data[0] = data[0]! ^ 0xff;
    expect(() => decodeSolanaEncryptedValueState(data, 'the fixture account')).toThrow('discriminator');
  });
});

describe('the account address', () => {
  const HOST_PROGRAM = bytes32(0x99);

  it('is the PDA of the tag and the four identity fields, in that order', async () => {
    const seeds = {
      program: bytes32(0x11),
      encryptedValueAccountAuthority: bytes32(0x22),
      scope: bytes32(0x33),
      label: bytes32(0x44),
    };
    const [expected] = await getProgramDerivedAddress({
      programAddress: base58.encode(HOST_PROGRAM) as Address,
      seeds: [
        SOLANA_ENCRYPTED_VALUE_SEED,
        seeds.program,
        seeds.encryptedValueAccountAuthority,
        seeds.scope,
        seeds.label,
      ],
    });
    expect(await solanaEncryptedValueAccountAddress(HOST_PROGRAM, seeds)).toBe(expected);
    expect(new TextDecoder().decode(SOLANA_ENCRYPTED_VALUE_SEED)).toBe('encrypted-value');
  });
});

describe('fetching an EncryptedValue account', () => {
  const HOST_PROGRAM = 'HostProgram1111111111111111111111111111111';
  const SYSTEM_PROGRAM = '11111111111111111111111111111111';

  function rpcWithAccount(owner: string): SolanaRpc {
    return {
      getAccountInfo: () => ({
        send: () =>
          Promise.resolve({
            context: { slot: 0n },
            value: {
              data: [Buffer.from(accountData({})).toString('base64'), 'base64'],
              executable: false,
              lamports: 1_000_000n,
              owner,
              rentEpoch: 0n,
              space: BigInt(accountData({}).length),
            },
          }),
      }),
    } as unknown as SolanaRpc;
  }

  it('names the account that does not exist', async () => {
    const rpc = {
      getAccountInfo: () => ({
        send: () => Promise.resolve({ context: { slot: 0n }, value: null }),
      }),
    } as unknown as SolanaRpc;

    await expect(
      fetchSolanaEncryptedValueState(rpc, 'Missing111111111111111111111111111111111111' as never),
    ).rejects.toThrow('Missing111111111111111111111111111111111111');
  });

  // Anyone can create a system account at the canonical address by transferring lamports to it;
  // with the owner pinned, the read reports that honestly instead of failing deeper in the
  // decoder as a phantom layout drift.
  it('refuses a foreign-owned account when the expected owner is pinned', async () => {
    await expect(
      fetchSolanaEncryptedValueState(
        rpcWithAccount(SYSTEM_PROGRAM),
        'Dusted11111111111111111111111111111111111111' as never,
        undefined,
        HOST_PROGRAM as never,
      ),
    ).rejects.toThrow('not the zama-host program');
  });

  it('decodes a host-owned account when the expected owner is pinned', async () => {
    const state = await fetchSolanaEncryptedValueState(
      rpcWithAccount(HOST_PROGRAM),
      'Dusted11111111111111111111111111111111111111' as never,
      undefined,
      HOST_PROGRAM as never,
    );
    expect(state.leafCount).toBe(3n);
  });
});
