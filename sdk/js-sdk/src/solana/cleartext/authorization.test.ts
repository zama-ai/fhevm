import { readFileSync } from 'node:fs';
import { address, getAddressDecoder, lamports, type Address, type MaybeEncodedAccount } from '@solana/kit';
import { describe, expect, it, vi } from 'vitest';
import { hexToBytes } from '../../core/base/bytes.js';
import { decodeSolanaPermitFields } from '../permit/validate.js';
import type { SolanaLeafProofOutcome, SolanaLeafProofReader, SolanaLeafQuery } from './leafProofs.js';
import {
  CONNECTOR_FAILURE_RECOVERABLE,
  judgeSolanaPublicDecryption,
  judgeSolanaUserDecryption,
  solanaRelayerDelegationRefusal,
  type ConnectorVerdict,
  type SolanaHostAccountsReader,
} from './authorization.js';

// The Connector's verdicts, rendered by kms-connector/crates/kms-worker/tests/solana_authorization_cases.rs.
type Accounts = readonly { readonly address: string; readonly owner: string; readonly data_base64: string }[];
type LeafRead = readonly { readonly query: unknown; readonly outcome: unknown }[] | null;
type Verdict =
  | { readonly authorized: true }
  | { readonly authorized: false; readonly failure: string; readonly entry: number | null };
type UserCase = {
  readonly name: string;
  readonly now: string;
  readonly permit: {
    readonly user_address: string;
    readonly allowed_scopes: readonly string[];
    readonly start_timestamp: string;
    readonly duration_seconds: string;
    readonly verifying_program_id: string;
    readonly chain_id: string;
    readonly extra_data: string;
  };
  readonly signature: string;
  readonly entries: readonly {
    readonly handle: string;
    readonly owner_address: string;
    readonly encrypted_store: string;
  }[];
  readonly accounts: Accounts;
  readonly leaf_read: LeafRead;
  readonly verdict: Verdict & { readonly recoverable?: boolean };
};
type PublicCase = {
  readonly name: string;
  readonly handles: readonly { readonly handle: string; readonly encrypted_store: string }[];
  readonly accounts: Accounts;
  readonly leaf_read: LeafRead;
  readonly verdict: Verdict & { readonly recoverable?: boolean };
};
type Fixture = {
  readonly host_program: string;
  readonly slot: string;
  readonly transport_key: string;
  readonly rust_only: readonly { readonly failure: string }[];
  readonly user_decrypt_cases: readonly UserCase[];
  readonly public_decrypt_cases: readonly PublicCase[];
};

const fixture = JSON.parse(
  readFileSync(
    new URL('../../../../../solana/test-fixtures/authorization/decrypt_cases_v1.json', import.meta.url),
    'utf8',
  ),
) as Fixture;

const bytes = (hex: string): Uint8Array => hexToBytes(`0x${hex}`);
const addressOf = (hex: string): Address => getAddressDecoder().decode(bytes(hex));
const programAddress = address(fixture.host_program);
const SLOT = BigInt(fixture.slot);

/** A read of these accounts, at the fixture's slot; any other key is absent. */
function accountsReader(accounts: Accounts): SolanaHostAccountsReader {
  const byAddress = new Map<Address, MaybeEncodedAccount>(
    accounts.map((account) => {
      const data = new Uint8Array(Buffer.from(account.data_base64, 'base64'));
      return [
        address(account.address),
        {
          exists: true,
          address: address(account.address),
          programAddress: address(account.owner),
          data,
          executable: false,
          lamports: lamports(1n),
          space: BigInt(data.length),
        },
      ];
    }),
  );
  return (keys, minContextSlot) => {
    // A delegated request's second read may not be older than its first.
    if (minContextSlot !== undefined) expect(minContextSlot).toBe(SLOT);
    return Promise.resolve({
      slot: SLOT,
      accounts: keys.map((key) => byAddress.get(key) ?? { exists: false, address: key }),
    });
  };
}

/** A recorded query as the leaf it asks for. */
function queryOf(wire: unknown): SolanaLeafQuery {
  const { encryptedStore, handle, key } = wire as { encryptedStore: string; handle: string; key?: string };
  const query = { encryptedStore: getAddressDecoder().decode(bytes(encryptedStore)), handle: bytes(handle) };
  return key === undefined ? query : { ...query, key: getAddressDecoder().decode(bytes(key)) };
}

/** A recorded answer as the outcome it is. */
function outcomeOf(wire: unknown): SolanaLeafProofOutcome {
  const { status, leafIndex, leafCount, siblings } = wire as Record<string, unknown>;
  return (
    status === 'found'
      ? {
          status,
          leafIndex: BigInt(leafIndex as number),
          leafCount: BigInt(leafCount as number),
          siblings: (siblings as string[]).map(bytes),
        }
      : status === 'notFound'
        ? { status, leafCount: BigInt(leafCount as number) }
        : { status }
  ) as SolanaLeafProofOutcome;
}

/**
 * The leaf record of a case: it answers the batch the Connector asked for, and only that batch, in
 * the Connector's order. `asked` counts the reads.
 */
function leafRecord(leafRead: LeafRead): { readLeafProofs: SolanaLeafProofReader; asked: () => number } {
  let reads = 0;
  return {
    readLeafProofs: (queries) => {
      reads += 1;
      if (leafRead === null) throw new Error('the Connector reads no leaf proof in this case');
      expect(queries).toEqual(leafRead.map(({ query }) => queryOf(query)));
      return Promise.resolve(leafRead.map(({ outcome }) => outcomeOf(outcome)));
    },
    asked: () => reads,
  };
}

/** A case's signed permit and handle entries. */
function requestOf({ permit, signature, entries }: UserCase) {
  return {
    fields: decodeSolanaPermitFields({
      userAddress: bytes(permit.user_address),
      transportKey: bytes(fixture.transport_key),
      allowedScopes: permit.allowed_scopes.map(bytes),
      startTimestamp: permit.start_timestamp,
      durationSeconds: permit.duration_seconds,
      verifyingProgramId: bytes(permit.verifying_program_id),
      chainId: permit.chain_id,
      extraData: bytes(permit.extra_data),
    }),
    signature: bytes(signature),
    entries: entries.map((entry) => ({
      handle: bytes(entry.handle),
      ownerAddress: bytes(entry.owner_address),
      encryptedStore: bytes(entry.encrypted_store),
    })),
  };
}

const userCase = (name: string): UserCase => {
  const found = fixture.user_decrypt_cases.find((testCase) => testCase.name === name);
  if (found === undefined) throw new Error(`no case named ${name}`);
  return found;
};

function expectVerdict(verdict: ConnectorVerdict, expected: Verdict): void {
  if (expected.authorized) {
    expect(verdict).toEqual({ authorized: true });
    return;
  }
  expect(verdict).toMatchObject({ authorized: false, failure: expected.failure });
  if (!verdict.authorized) expect(verdict.entry ?? null).toBe(expected.entry);
}

describe('the cleartext client judges a user decryption as the KMS Connector does', () => {
  for (const testCase of fixture.user_decrypt_cases) {
    it(testCase.name, async () => {
      const record = leafRecord(testCase.leaf_read);
      const verdict = await judgeSolanaUserDecryption({
        programAddress,
        now: BigInt(testCase.now),
        ...requestOf(testCase),
        readAccounts: accountsReader(testCase.accounts),
        readLeafProofs: record.readLeafProofs,
      });
      expectVerdict(verdict, testCase.verdict);
      expect(record.asked()).toBe(testCase.leaf_read === null ? 0 : 1);
      if (!verdict.authorized) {
        expect(CONNECTOR_FAILURE_RECOVERABLE[verdict.failure]).toBe(testCase.verdict.recoverable);
      }
    });
  }

  it('knows exactly the failures the cases produce, and none the Connector keeps to itself', () => {
    const produced = new Set(
      fixture.user_decrypt_cases.flatMap((c) => (c.verdict.authorized ? [] : [c.verdict.failure])),
    );
    expect(new Set(Object.keys(CONNECTOR_FAILURE_RECOVERABLE))).toEqual(produced);
    for (const { failure } of fixture.rust_only) expect(produced.has(failure)).toBe(false);
  });

  it('judges every rule on the last read, which may be newer than the first', async () => {
    // The first read shows a live delegation; the host revokes it before the second.
    const live = accountsReader(userCase("a delegated entry through the application's row").accounts);
    const revoked = userCase('a revoked delegation');
    const readAccounts = vi.fn<SolanaHostAccountsReader>(live);
    readAccounts.mockImplementation((keys, minContextSlot) =>
      readAccounts.mock.calls.length === 1 ? live(keys) : accountsReader(revoked.accounts)(keys, minContextSlot),
    );
    const verdict = await judgeSolanaUserDecryption({
      programAddress,
      now: BigInt(revoked.now),
      ...requestOf(revoked),
      readAccounts,
      readLeafProofs: leafRecord(revoked.leaf_read).readLeafProofs,
    });
    expect(verdict).toMatchObject({ failure: 'Delegation::NoLiveDelegation', entry: 0 });
    expect(readAccounts.mock.calls.map(([, minContextSlot]) => minContextSlot)).toEqual([undefined, SLOT]);
  });
});

describe('the cleartext client judges a public decryption as the KMS Connector does', () => {
  for (const testCase of fixture.public_decrypt_cases) {
    it(testCase.name, async () => {
      const record = leafRecord(testCase.leaf_read);
      const verdict = await judgeSolanaPublicDecryption({
        programAddress,
        handles: testCase.handles.map(({ handle, encrypted_store }) => ({
          handle: bytes(handle),
          encryptedStore: addressOf(encrypted_store),
        })),
        readAccounts: accountsReader(testCase.accounts),
        readLeafProofs: record.readLeafProofs,
      });
      expectVerdict(verdict, testCase.verdict);
      expect(record.asked()).toBe(testCase.leaf_read === null ? 0 : 1);
      if (!verdict.authorized) {
        expect(CONNECTOR_FAILURE_RECOVERABLE[verdict.failure]).toBe(testCase.verdict.recoverable);
      }
    });
  }
});

// The relayer judges delegation rows by the Connector's rules, over the same worlds, but refuses
// only what the Connector could never authorize.
describe("the relayer's delegation pre-check", () => {
  const precheck = (name: string) => {
    const testCase = userCase(name);
    const { fields, entries } = requestOf(testCase);
    return solanaRelayerDelegationRefusal({
      programAddress,
      fields,
      entries,
      readAccounts: accountsReader(testCase.accounts),
    });
  };

  it('refuses a delegated entry whose rows are both dead at the host Clock', async () => {
    for (const name of [
      'a delegated entry with no delegation row',
      'a revoked delegation',
      "a delegation that has ended by the host's Clock but not by the local clock",
    ]) {
      await expect(precheck(name), name).resolves.toMatch(/has no live delegation .* at the host's clock/);
    }
  });

  it('passes a live row, a row the host could not have written, and a store it cannot resolve', async () => {
    for (const name of [
      'a delegated entry through the wildcard row beside a revoked application row',
      'a delegation row held by another program beside a live wildcard row',
      'a wildcard row held by another program beside a live application row',
      "a delegated entry's store is resolved before the watermark",
    ]) {
      await expect(precheck(name), name).resolves.toBeUndefined();
    }
  });

  it('reads nothing for direct entries', async () => {
    const { fields, entries } = requestOf(userCase("a direct entry with the signer's allow leaf"));
    const readAccounts = vi.fn<SolanaHostAccountsReader>();
    await expect(
      solanaRelayerDelegationRefusal({ programAddress, fields, entries, readAccounts }),
    ).resolves.toBeUndefined();
    expect(readAccounts).not.toHaveBeenCalled();
  });
});
