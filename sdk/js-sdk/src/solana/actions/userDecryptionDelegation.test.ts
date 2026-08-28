import { describe, expect, it } from 'vitest';
import { AccountRole, address, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import {
  SOLANA_WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
  buildDelegateForUserDecryptionInstruction,
  buildRevokeDelegationForUserDecryptionInstruction,
  decodeSolanaUserDecryptionDelegation,
  fetchSolanaUserDecryptionDelegation,
  isSolanaUserDecryptionDelegationLiveAt,
  solanaDelegationWarnings,
  solanaUserDecryptionDelegationAddress,
} from './userDecryptionDelegation.js';
import type { SolanaRpc } from '../encryptedValueAccount.js';
import { findHostConfigPda } from '../internal/generated/zamaHost/pdas/index.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../internal/generated/zamaHost/programAddress.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

function hex(bytes: Iterable<number>): string {
  return Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, '0'))
    .join('');
}

// The fixture tuple of the Rust cross-pin (`sdk_fixture_delegation_address_and_instruction_bytes`
// in solana/runtime-tests/tests/user_decryption_delegation_mollusk.rs). The literals below are
// asserted there against the host program's own derivation and codec, so a drift on either side
// breaks both suites on the same bytes.
const delegator = addr(0x11);
const delegate = addr(0x22);
const authority = addr(0x33);
const payer = addr(0x55);
const RECORD_ADDRESS = '5bK6ZBSpCgC13c5JT5g2LHjRfTjBM6Fjaybcv8tQqUUX';
const GRANT_DATA =
  'f0f8d7586df401672222222222222222222222222222222222222222222222222222222222222222' +
  '3333333333333333333333333333333333333333333333333333333333333333f401000000000000';
const REVOKE_DATA = '931b7e35412576e1';
const SYSTEM_PROGRAM = '11111111111111111111111111111111';

describe('solanaUserDecryptionDelegationAddress', () => {
  it('derives the canonical record address the host program derives', async () => {
    const derived = await solanaUserDecryptionDelegationAddress({
      delegator,
      delegate,
      encryptedValueAccountAuthority: authority,
    });
    expect(derived).toBe(RECORD_ADDRESS);
  });
});

describe('buildDelegateForUserDecryptionInstruction', () => {
  const build = () =>
    buildDelegateForUserDecryptionInstruction({
      payer,
      delegator,
      delegate,
      encryptedValueAccountAuthority: authority,
      expirationSlot: 500n,
    });

  it('builds the exact bytes the host program decodes', async () => {
    const instruction = await build();
    expect(instruction.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect(hex(instruction.data!)).toBe(GRANT_DATA);
  });

  it('names the five accounts in program order with their roles', async () => {
    const instruction = await build();
    const [hostConfig] = await findHostConfigPda();
    expect(instruction.accounts?.map((account) => [account.address, account.role])).toEqual([
      [payer, AccountRole.WRITABLE_SIGNER],
      [delegator, AccountRole.READONLY_SIGNER],
      [hostConfig, AccountRole.READONLY],
      [RECORD_ADDRESS, AccountRole.WRITABLE],
      [SYSTEM_PROGRAM, AccountRole.READONLY],
    ]);
  });
});

describe('buildRevokeDelegationForUserDecryptionInstruction', () => {
  const build = () =>
    buildRevokeDelegationForUserDecryptionInstruction({
      delegator,
      delegate,
      encryptedValueAccountAuthority: authority,
    });

  it('builds the exact bytes the host program decodes', async () => {
    const instruction = await build();
    expect(instruction.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect(hex(instruction.data!)).toBe(REVOKE_DATA);
  });

  it('names the three accounts in program order with their roles', async () => {
    const instruction = await build();
    const [hostConfig] = await findHostConfigPda();
    expect(instruction.accounts?.map((account) => [account.address, account.role])).toEqual([
      [delegator, AccountRole.READONLY_SIGNER],
      [hostConfig, AccountRole.READONLY],
      [RECORD_ADDRESS, AccountRole.WRITABLE],
    ]);
  });
});

describe('solanaDelegationWarnings', () => {
  it('exports the sentinel the wildcard row carries in place of an authority', () => {
    expect(SOLANA_WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY).toBe(addr(0xff));
  });

  it('flags a wildcard-authority grant', () => {
    const warnings = solanaDelegationWarnings({
      encryptedValueAccountAuthority: SOLANA_WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
    });
    expect(warnings).toHaveLength(1);
    expect(warnings[0]!.code).toBe('WildcardAuthority');
    expect(warnings[0]!.message).toContain('every');
  });

  it('is silent for an authority-scoped grant', () => {
    expect(solanaDelegationWarnings({ encryptedValueAccountAuthority: authority })).toEqual([]);
  });
});

// The record bytes as the host program serializes them — the same literal is pinned in the Rust
// cross-pin against the program's own serializer: discriminator, the tuple, expiration 500,
// counter 7, last update slot 400, not revoked, bump 254.
const RECORD_BYTES_HEX =
  '25058b21493501f8' +
  '11'.repeat(32) +
  '22'.repeat(32) +
  '33'.repeat(32) +
  'f401000000000000' +
  '0700000000000000' +
  '9001000000000000' +
  '00' +
  'fe';

function bytesFromHex(hexString: string): Uint8Array {
  const out = new Uint8Array(hexString.length / 2);
  for (let index = 0; index < out.length; index += 1) {
    out[index] = Number.parseInt(hexString.slice(index * 2, index * 2 + 2), 16);
  }
  return out;
}

describe('decodeSolanaUserDecryptionDelegation', () => {
  it('decodes the exact bytes the host program writes', () => {
    const record = decodeSolanaUserDecryptionDelegation(bytesFromHex(RECORD_BYTES_HEX), 'the fixture record');
    expect(record.delegator).toBe(delegator);
    expect(record.delegate).toBe(delegate);
    expect(record.encryptedValueAccountAuthority).toBe(authority);
    expect(record.expirationSlot).toBe(500n);
    expect(record.delegationCounter).toBe(7n);
    expect(record.lastUpdateSlot).toBe(400n);
    expect(record.revoked).toBe(false);
    expect(record.bump).toBe(254);
  });

  it('rejects an account of another type by its discriminator', () => {
    const data = bytesFromHex(RECORD_BYTES_HEX);
    data[0] = data[0]! ^ 0xff;
    expect(() => decodeSolanaUserDecryptionDelegation(data, 'the fixture record')).toThrow('discriminator');
  });

  it('rejects a record of the wrong size', () => {
    const data = bytesFromHex(RECORD_BYTES_HEX).slice(0, -1);
    expect(() => decodeSolanaUserDecryptionDelegation(data, 'the fixture record')).toThrow('130');
  });

  it('rejects a revoked byte that is not a borsh bool, exactly as the Rust twin does', () => {
    const data = bytesFromHex(RECORD_BYTES_HEX);
    data[128] = 2;
    expect(() => decodeSolanaUserDecryptionDelegation(data, 'the fixture record')).toThrow('not a borsh bool');
  });
});

describe('isSolanaUserDecryptionDelegationLiveAt', () => {
  const record = decodeSolanaUserDecryptionDelegation(bytesFromHex(RECORD_BYTES_HEX), 'the fixture record');

  it('is live through the expiration slot, inclusive — the Connector boundary', () => {
    expect(isSolanaUserDecryptionDelegationLiveAt(record, 500n)).toBe(true);
    expect(isSolanaUserDecryptionDelegationLiveAt(record, 501n)).toBe(false);
  });

  it('is dead once revoked, whatever the expiration says', () => {
    const revoked = { ...record, revoked: true };
    expect(isSolanaUserDecryptionDelegationLiveAt(revoked, 100n)).toBe(false);
  });
});

describe('fetchSolanaUserDecryptionDelegation', () => {
  const tuple = { delegator, delegate, encryptedValueAccountAuthority: authority };

  function rpcWith(
    accounts: Readonly<Record<string, string | { data: string; owner: string }>>,
  ): SolanaRpc {
    return {
      getAccountInfo: (accountAddress: string) => ({
        send: () => {
          const entry = accounts[accountAddress];
          const account = typeof entry === 'string' ? { data: entry, owner: ZAMA_HOST_PROGRAM_ADDRESS } : entry;
          return Promise.resolve({
            context: { slot: 0n },
            value:
              account === undefined
                ? null
                : {
                    data: [Buffer.from(bytesFromHex(account.data)).toString('base64'), 'base64'],
                    executable: false,
                    lamports: 1_000_000n,
                    owner: account.owner,
                    rentEpoch: 0n,
                    space: BigInt(account.data.length / 2),
                  },
          });
        },
      }),
    } as unknown as SolanaRpc;
  }

  it('reads the authority-specific row', async () => {
    const rows = await fetchSolanaUserDecryptionDelegation(rpcWith({ [RECORD_ADDRESS]: RECORD_BYTES_HEX }), tuple);
    expect(rows.exact?.delegationCounter).toBe(7n);
    expect(rows.wildcard).toBeNull();
  });

  it('reads the wildcard row an authority-specific miss falls back to', async () => {
    const wildcardAddress = await solanaUserDecryptionDelegationAddress({
      ...tuple,
      encryptedValueAccountAuthority: SOLANA_WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
    });
    const rows = await fetchSolanaUserDecryptionDelegation(rpcWith({ [wildcardAddress]: RECORD_BYTES_HEX }), tuple);
    expect(rows.exact).toBeNull();
    expect(rows.wildcard?.delegationCounter).toBe(7n);
  });

  it('reports the delegation as absent when neither row exists', async () => {
    const rows = await fetchSolanaUserDecryptionDelegation(rpcWith({}), tuple);
    expect(rows.exact).toBeNull();
    expect(rows.wildcard).toBeNull();
  });

  // Anyone can create a system account at the canonical address by transferring lamports to it;
  // no delegation record exists there, and a third party must not be able to make this read throw.
  it('reads a foreign-owned account at the record address as absent, not as an error', async () => {
    const rows = await fetchSolanaUserDecryptionDelegation(
      rpcWith({ [RECORD_ADDRESS]: { data: '', owner: SYSTEM_PROGRAM } }),
      tuple,
    );
    expect(rows.exact).toBeNull();
    expect(rows.wildcard).toBeNull();
  });
});
