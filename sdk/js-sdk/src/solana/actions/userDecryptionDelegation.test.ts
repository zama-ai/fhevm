import { describe, expect, it } from 'vitest';
import {
  AccountRole,
  address,
  generateKeyPairSigner,
  getAddressEncoder,
  getProgramDerivedAddress,
  type Address,
  type TransactionSigner,
} from '@solana/kit';
import { base58 } from '@scure/base';

import {
  SOLANA_USER_DECRYPTION_DELEGATION_SEED,
  SOLANA_WILDCARD_APP,
  buildDelegateForUserDecryptionInstruction,
  buildRevokeDelegationForUserDecryptionInstruction,
  decodeSolanaUserDecryptionDelegation,
  fetchSolanaUserDecryptionDelegation,
  isSolanaUserDecryptionDelegationLiveAt,
  solanaDelegationWarnings,
  solanaUserDecryptionDelegationAddress,
} from './userDecryptionDelegation.js';
import type { SolanaRpc } from '../encryptedStore.js';
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
const application = { program: addr(0x33), scope: new Uint8Array(32).fill(0x44) };
const payer = addr(0x55);
const RECORD_ADDRESS = address('GkmqVNMzqxopBjPkSkZvuLuDE6Jze3iA3Mq5ZHr6SrtJ');
// The delegator's wildcard row, pinned by the relayer's cross-pin in the same Rust suite.
const WILDCARD_RECORD_ADDRESS = 'J4BMamYLJvJroFATJp48L6AeQJDQqv86YAQyPqvBcKq1';
const GRANT_DATA =
  'f0f8d7586df401672222222222222222222222222222222222222222222222222222222222222222' +
  '33'.repeat(32) +
  '44'.repeat(32) +
  'f401000000000000';
const REVOKE_DATA = '931b7e35412576e1';
const SYSTEM_PROGRAM = '11111111111111111111111111111111';

// A second deployment is a different host program id.
const OTHER_PROGRAM = addr(0x66);
const HOST = { programAddress: ZAMA_HOST_PROGRAM_ADDRESS };

describe('solanaUserDecryptionDelegationAddress', () => {
  it('derives the canonical record address the host program derives', async () => {
    const derived = await solanaUserDecryptionDelegationAddress(
      { delegator, delegate, ...application },
      { programAddress: ZAMA_HOST_PROGRAM_ADDRESS },
    );
    expect(derived).toBe(RECORD_ADDRESS);
  });

  it('derives the wildcard row from the wildcard application', async () => {
    const derived = await solanaUserDecryptionDelegationAddress(
      { delegator, delegate, ...SOLANA_WILDCARD_APP },
      { programAddress: ZAMA_HOST_PROGRAM_ADDRESS },
    );
    expect(derived).toBe(WILDCARD_RECORD_ADDRESS);
  });

  it('derives another row for another scope of the same program', async () => {
    const derived = await solanaUserDecryptionDelegationAddress(
      { delegator, delegate, ...application, scope: new Uint8Array(32).fill(0x45) },
      { programAddress: ZAMA_HOST_PROGRAM_ADDRESS },
    );
    expect(derived).not.toBe(RECORD_ADDRESS);
  });

  it('refuses a scope that is not 32 bytes', async () => {
    await expect(
      solanaUserDecryptionDelegationAddress(
        { delegator, delegate, ...application, scope: new Uint8Array(31).fill(0x44) },
        { programAddress: ZAMA_HOST_PROGRAM_ADDRESS },
      ),
    ).rejects.toThrow('delegation scope must be 32 bytes, got 31');
  });

  it('derives under the configured program id', async () => {
    const derived = await solanaUserDecryptionDelegationAddress(
      { delegator, delegate, ...application },
      { programAddress: OTHER_PROGRAM },
    );
    expect(derived).not.toBe(RECORD_ADDRESS);
  });
});

describe('buildDelegateForUserDecryptionInstruction', () => {
  const build = () =>
    buildDelegateForUserDecryptionInstruction({
      programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
      payer,
      delegator,
      delegate,
      ...application,
      expiresAt: 500n,
    });

  it('builds the exact bytes the host program decodes', async () => {
    const instruction = await build();
    expect(instruction.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect(hex(instruction.data!)).toBe(GRANT_DATA);
  });

  it('refuses a short scope even when the record address is given', async () => {
    await expect(
      buildDelegateForUserDecryptionInstruction({
        programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
        payer,
        delegator,
        delegate,
        ...application,
        scope: new Uint8Array(31),
        delegationRecord: RECORD_ADDRESS,
        expiresAt: 500n,
      }),
    ).rejects.toThrow('delegation scope must be 32 bytes, got 31');
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

  // The meta of a signer meta: `signer` is present exactly when the account must sign, and it
  // is what the kit's signing pipeline signs with.
  type SignerMeta = { readonly address: Address; readonly signer?: TransactionSigner };

  it('carries a passed TransactionSigner through to the signer metas', async () => {
    const delegatorSigner = await generateKeyPairSigner();
    const instruction = await buildDelegateForUserDecryptionInstruction({
      programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
      payer: delegatorSigner,
      delegator: delegatorSigner,
      delegate,
      ...application,
      expiresAt: 500n,
    });
    const [payerMeta, delegatorMeta] = instruction.accounts as readonly SignerMeta[];
    expect(payerMeta?.signer).toBe(delegatorSigner);
    expect(delegatorMeta?.signer).toBe(delegatorSigner);
  });

  it('mixes a signing payer with an address-only delegator, deriving from the address', async () => {
    const payerSigner = await generateKeyPairSigner();
    const instruction = await buildDelegateForUserDecryptionInstruction({
      programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
      payer: payerSigner,
      delegator,
      delegate,
      ...application,
      expiresAt: 500n,
    });
    const [payerMeta, delegatorMeta] = instruction.accounts as readonly SignerMeta[];
    expect(payerMeta?.signer).toBe(payerSigner);
    // The delegator meta still demands a signature, but through a noop placeholder — the
    // proposal/CPI form.
    expect(delegatorMeta?.signer?.address).toBe(delegator);
    expect(delegatorMeta?.signer).not.toBe(payerSigner);
    // The record PDA derives from the delegator's address regardless of the form it came in.
    expect(instruction.accounts?.[3]?.address).toBe(RECORD_ADDRESS);
  });

  it('targets an overridden program id with every derived account under it', async () => {
    const instruction = await buildDelegateForUserDecryptionInstruction({
      payer,
      delegator,
      delegate,
      ...application,
      expiresAt: 500n,
      programAddress: OTHER_PROGRAM,
    });
    const [hostConfig] = await findHostConfigPda({ programAddress: OTHER_PROGRAM });
    const record = await solanaUserDecryptionDelegationAddress(
      { delegator, delegate, ...application },
      { programAddress: OTHER_PROGRAM },
    );
    expect(instruction.programAddress).toBe(OTHER_PROGRAM);
    expect(instruction.accounts?.[2]?.address).toBe(hostConfig);
    expect(instruction.accounts?.[3]?.address).toBe(record);
  });
});

describe('buildRevokeDelegationForUserDecryptionInstruction', () => {
  const build = () =>
    buildRevokeDelegationForUserDecryptionInstruction({
      programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
      delegator,
      delegate,
      ...application,
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

  it('carries a passed TransactionSigner through to the delegator meta', async () => {
    const delegatorSigner = await generateKeyPairSigner();
    const instruction = await buildRevokeDelegationForUserDecryptionInstruction({
      programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
      delegator: delegatorSigner,
      delegate,
      ...application,
    });
    const meta = instruction.accounts?.[0] as { address: Address; signer?: TransactionSigner } | undefined;
    expect(meta?.signer).toBe(delegatorSigner);
  });

  it('targets an overridden program id with every derived account under it', async () => {
    const instruction = await buildRevokeDelegationForUserDecryptionInstruction({
      delegator,
      delegate,
      ...application,
      programAddress: OTHER_PROGRAM,
    });
    const [hostConfig] = await findHostConfigPda({ programAddress: OTHER_PROGRAM });
    const record = await solanaUserDecryptionDelegationAddress(
      { delegator, delegate, ...application },
      { programAddress: OTHER_PROGRAM },
    );
    expect(instruction.programAddress).toBe(OTHER_PROGRAM);
    expect(instruction.accounts?.[1]?.address).toBe(hostConfig);
    expect(instruction.accounts?.[2]?.address).toBe(record);
  });
});

describe('solanaDelegationWarnings', () => {
  it('exports 0xff×32 in both positions as the wildcard application', () => {
    expect(SOLANA_WILDCARD_APP.program).toBe(addr(0xff));
    expect(hex(SOLANA_WILDCARD_APP.scope)).toBe('ff'.repeat(32));
  });

  it('flags a wildcard grant', () => {
    const warnings = solanaDelegationWarnings(SOLANA_WILDCARD_APP);
    expect(warnings).toHaveLength(1);
    expect(warnings[0]!.code).toBe('WildcardApp');
    expect(warnings[0]!.message).toContain('every application');
  });

  it('is silent for a grant over one application', () => {
    expect(solanaDelegationWarnings(application)).toEqual([]);
  });

  it('is silent for a scope that only starts with the wildcard bytes', () => {
    expect(
      solanaDelegationWarnings({ program: SOLANA_WILDCARD_APP.program, scope: new Uint8Array(33).fill(0xff) }),
    ).toEqual([]);
  });

  it('is silent for a half-wildcard, which the host refuses rather than widens', () => {
    expect(solanaDelegationWarnings({ ...application, program: SOLANA_WILDCARD_APP.program })).toEqual([]);
  });
});

// The record bytes as the host program serializes them — the same literal is pinned in the Rust
// cross-pin against the program's own serializer: discriminator, the tuple, expiry 500, counter
// 7, last update slot 400, bump 255 (the canonical bump of the record address — the only value
// the program ever stores, and the one the fetch validates).
const RECORD_BYTES_HEX =
  '25058b21493501f8' +
  '11'.repeat(32) +
  '22'.repeat(32) +
  '33'.repeat(32) +
  '44'.repeat(32) +
  'f401000000000000' +
  '0700000000000000' +
  '9001000000000000' +
  'ff';

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
    expect(record.program).toBe(application.program);
    expect(hex(record.scope)).toBe('44'.repeat(32));
    expect(record.expiresAt).toBe(500n);
    expect(record.delegationCounter).toBe(7n);
    expect(record.lastUpdateSlot).toBe(400n);
    expect(record.bump).toBe(255);
  });

  it('rejects an account of another type by its discriminator', () => {
    const data = bytesFromHex(RECORD_BYTES_HEX);
    data[0] = data[0]! ^ 0xff;
    expect(() => decodeSolanaUserDecryptionDelegation(data, 'the fixture record')).toThrow('discriminator');
  });

  it('rejects a record of the wrong size', () => {
    const data = bytesFromHex(RECORD_BYTES_HEX).slice(0, -1);
    expect(() => decodeSolanaUserDecryptionDelegation(data, 'the fixture record')).toThrow('161');
  });
});

describe('isSolanaUserDecryptionDelegationLiveAt', () => {
  const record = decodeSolanaUserDecryptionDelegation(bytesFromHex(RECORD_BYTES_HEX), 'the fixture record');

  it('is live until its expiry second, exclusive — the Connector boundary', () => {
    expect(isSolanaUserDecryptionDelegationLiveAt(record, 499n)).toBe(true);
    expect(isSolanaUserDecryptionDelegationLiveAt(record, 500n)).toBe(false);
  });

  it('is dead once revoked, which zeroes the expiry', () => {
    const revoked = { ...record, expiresAt: 0n };
    expect(isSolanaUserDecryptionDelegationLiveAt(revoked, 0n)).toBe(false);
  });
});

describe('fetchSolanaUserDecryptionDelegation', () => {
  const tuple = { delegator, delegate, ...application };

  function rpcWith(accounts: Readonly<Record<string, string | { data: string; owner: string }>>): SolanaRpc {
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

  it('reads the row of the application', async () => {
    const rows = await fetchSolanaUserDecryptionDelegation(
      rpcWith({ [RECORD_ADDRESS]: RECORD_BYTES_HEX }),
      tuple,
      HOST,
    );
    expect(rows.exact?.delegationCounter).toBe(7n);
    expect(rows.wildcard).toBeNull();
  });

  // The wildcard row as the host program would write it: the wildcard application in the tuple,
  // and the canonical bump of the wildcard address in the trailing byte.
  async function wildcardRowFixture(): Promise<{ address: Address; bytesHex: string }> {
    const encoder = getAddressEncoder();
    const [wildcardAddress, bump] = await getProgramDerivedAddress({
      programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
      seeds: [
        SOLANA_USER_DECRYPTION_DELEGATION_SEED,
        encoder.encode(delegator),
        encoder.encode(delegate),
        encoder.encode(SOLANA_WILDCARD_APP.program),
        SOLANA_WILDCARD_APP.scope,
      ],
    });
    const bytesHex =
      '25058b21493501f8' +
      '11'.repeat(32) +
      '22'.repeat(32) +
      'ff'.repeat(64) +
      'f401000000000000' +
      '0700000000000000' +
      '9001000000000000' +
      bump.toString(16).padStart(2, '0');
    return { address: wildcardAddress, bytesHex };
  }

  it('reads the wildcard row a miss on the application row falls back to', async () => {
    const wildcardRow = await wildcardRowFixture();
    const rows = await fetchSolanaUserDecryptionDelegation(
      rpcWith({ [wildcardRow.address]: wildcardRow.bytesHex }),
      tuple,
      HOST,
    );
    expect(rows.exact).toBeNull();
    expect(rows.wildcard?.delegationCounter).toBe(7n);
    expect(rows.wildcard?.program).toBe(SOLANA_WILDCARD_APP.program);
  });

  // The Connector's rule, mirrored: the address is not taken as proof of what the record says.
  // Only the host program can write these bytes, so a contradiction is its defect — an error,
  // not a delegation of the queried tuple and not a silent absence.
  it('throws on a record naming a tuple other than the one its address derives from', async () => {
    // The application record (program 0x33, scope 0x44) sitting at the wildcard address.
    await expect(
      fetchSolanaUserDecryptionDelegation(rpcWith({ [WILDCARD_RECORD_ADDRESS]: RECORD_BYTES_HEX }), tuple, HOST),
    ).rejects.toThrow('tuple other than');
  });

  // The scope is the one field compared as bytes rather than as an address.
  it('throws on a record naming another scope of the same program', async () => {
    const otherScope = RECORD_BYTES_HEX.replace('44'.repeat(32), '45'.repeat(32));
    await expect(
      fetchSolanaUserDecryptionDelegation(rpcWith({ [RECORD_ADDRESS]: otherScope }), tuple, HOST),
    ).rejects.toThrow('tuple other than');
  });

  it('throws on a record storing a bump that is not the canonical one of its address', async () => {
    const withWrongBump = RECORD_BYTES_HEX.slice(0, -2) + 'fd';
    await expect(
      fetchSolanaUserDecryptionDelegation(rpcWith({ [RECORD_ADDRESS]: withWrongBump }), tuple, HOST),
    ).rejects.toThrow('canonical bump');
  });

  it('reports the delegation as absent when neither row exists', async () => {
    const rows = await fetchSolanaUserDecryptionDelegation(rpcWith({}), tuple, HOST);
    expect(rows.exact).toBeNull();
    expect(rows.wildcard).toBeNull();
  });

  // Anyone can create a system account at the canonical address by transferring lamports to it;
  // no delegation record exists there, and a third party must not be able to make this read throw.
  it('reads a foreign-owned account at the record address as absent, not as an error', async () => {
    const rows = await fetchSolanaUserDecryptionDelegation(
      rpcWith({ [RECORD_ADDRESS]: { data: '', owner: SYSTEM_PROGRAM } }),
      tuple,
      HOST,
    );
    expect(rows.exact).toBeNull();
    expect(rows.wildcard).toBeNull();
  });

  it('reads a deployment at an overridden program id: its addresses, its ownership', async () => {
    const encoder = getAddressEncoder();
    // Derived by hand rather than through the module, bump included: the record the other
    // deployment's program would write carries the canonical bump of ITS address.
    const [overriddenRecord, overriddenBump] = await getProgramDerivedAddress({
      programAddress: OTHER_PROGRAM,
      seeds: [
        SOLANA_USER_DECRYPTION_DELEGATION_SEED,
        encoder.encode(delegator),
        encoder.encode(delegate),
        encoder.encode(application.program),
        application.scope,
      ],
    });
    const overriddenRecordBytes = RECORD_BYTES_HEX.slice(0, -2) + overriddenBump.toString(16).padStart(2, '0');
    const accounts = {
      // The row of the overridden deployment, owned by it.
      [overriddenRecord]: { data: overriddenRecordBytes, owner: OTHER_PROGRAM },
      // A canonical-deployment row must not satisfy a read scoped to the other deployment.
      [RECORD_ADDRESS]: RECORD_BYTES_HEX,
    };
    const rows = await fetchSolanaUserDecryptionDelegation(rpcWith(accounts), tuple, {
      programAddress: OTHER_PROGRAM,
    });
    expect(rows.exact?.delegationCounter).toBe(7n);
    expect(rows.wildcard).toBeNull();
  });
});
