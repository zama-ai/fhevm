import { readFileSync } from 'node:fs';
import { address, getAddressDecoder, lamports, type Address, type MaybeEncodedAccount } from '@solana/kit';
import { describe, expect, it, vi } from 'vitest';
import { hexToBytes } from '../../core/base/bytes.js';
import { decodeSolanaPermitFields } from '../permit/validate.js';
import type { SolanaStoreHistoryEvent } from '../proof.js';
import type { SolanaStoreHistoryReader } from './storeHistory.js';
import {
  CONNECTOR_FAILURE_RECOVERABLE,
  judgeSolanaPublicDecryption,
  judgeSolanaUserDecryption,
  solanaRelayerDelegationRefusal,
  type SolanaHostAccountsReader,
} from './authorization.js';

// The Connector's verdicts, rendered by kms-connector/crates/kms-worker/tests/solana_authorization_cases.rs.
type Fixture = {
  readonly host_program: string;
  readonly rust_only: readonly { readonly failure: string }[];
  readonly cases: readonly {
    readonly name: string;
    readonly now: string;
    readonly permit: {
      readonly user_address: string;
      readonly transport_key: string;
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
    readonly accounts: readonly { readonly address: string; readonly owner: string; readonly data_base64: string }[];
    readonly records: readonly {
      readonly encrypted_store: string;
      readonly leaves: readonly {
        readonly kind: 'allowed' | 'public';
        readonly handle: string;
        readonly key?: string;
      }[];
    }[];
    readonly verdict:
      | { readonly authorized: true }
      | {
          readonly authorized: false;
          readonly failure: string;
          readonly entry: number | null;
          readonly recoverable: boolean;
        };
  }[];
};

const fixture = JSON.parse(
  readFileSync(
    new URL('../../../../../solana/test-fixtures/authorization/user_decrypt_cases_v1.json', import.meta.url),
    'utf8',
  ),
) as Fixture;

const bytes = (hex: string): Uint8Array => hexToBytes(`0x${hex}`);
const programAddress = address(fixture.host_program);
const SLOT = 100n;

/** The host state and leaf record of a case, as the two readers a verdict takes. */
function hostOf(testCase: Fixture['cases'][number]): {
  readAccounts: SolanaHostAccountsReader;
  readHistory: SolanaStoreHistoryReader;
} {
  const accounts = new Map<Address, MaybeEncodedAccount>(
    testCase.accounts.map((account) => {
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
  const records = new Map<Address, SolanaStoreHistoryEvent[]>(
    testCase.records.map((record) => [
      address(record.encrypted_store),
      record.leaves.map(
        (leaf): SolanaStoreHistoryEvent =>
          leaf.kind === 'allowed'
            ? { kind: 'allowed', handle: bytes(leaf.handle), key: bytes(leaf.key ?? '') }
            : { kind: 'markedPublic', handle: bytes(leaf.handle) },
      ),
    ]),
  );
  return {
    readAccounts: (keys, minContextSlot) => {
      // A delegated request's second read may not be older than its first.
      if (minContextSlot !== undefined) expect(minContextSlot).toBe(SLOT);
      return Promise.resolve({
        slot: SLOT,
        accounts: keys.map((key) => accounts.get(key) ?? { exists: false, address: key }),
      });
    },
    readHistory: (store) => {
      const record = records.get(store);
      if (record === undefined) throw new Error(`the Connector reads no leaf of ${store} in this case`);
      return Promise.resolve(record);
    },
  };
}

const caseNamed = (name: string): Fixture['cases'][number] => {
  const found = fixture.cases.find((testCase) => testCase.name === name);
  if (found === undefined) throw new Error(`no case named ${name}`);
  return found;
};

/** A case's signed permit and handle entries. */
function requestOf({ permit, signature, entries }: Fixture['cases'][number]) {
  return {
    fields: decodeSolanaPermitFields({
      userAddress: bytes(permit.user_address),
      transportKey: bytes(permit.transport_key),
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

describe('the cleartext client judges a user decryption as the KMS Connector does', () => {
  for (const testCase of fixture.cases) {
    it(testCase.name, async () => {
      const verdict = await judgeSolanaUserDecryption({
        programAddress,
        now: BigInt(testCase.now),
        ...requestOf(testCase),
        ...hostOf(testCase),
      });

      const expected = testCase.verdict;
      if (expected.authorized) {
        expect(verdict).toEqual({ authorized: true });
        return;
      }
      expect(verdict).toMatchObject({
        authorized: false,
        failure: expected.failure,
        ...(expected.entry === null ? {} : { entry: expected.entry }),
      });
      if (!verdict.authorized) {
        expect(verdict.entry ?? null).toBe(expected.entry);
        expect(CONNECTOR_FAILURE_RECOVERABLE[verdict.failure]).toBe(expected.recoverable);
      }
    });
  }

  it('knows exactly the failures the cases produce, and none the Connector keeps to itself', () => {
    const produced = new Set(fixture.cases.flatMap((c) => (c.verdict.authorized ? [] : [c.verdict.failure])));
    expect(new Set(Object.keys(CONNECTOR_FAILURE_RECOVERABLE))).toEqual(produced);
    for (const { failure } of fixture.rust_only) expect(produced.has(failure)).toBe(false);
  });
});

// The relayer judges delegation rows by the Connector's rules, over the same worlds, but refuses
// only what the Connector could never authorize.
describe("the relayer's delegation pre-check", () => {
  const precheck = (name: string) => {
    const testCase = caseNamed(name);
    const { fields, entries } = requestOf(testCase);
    return solanaRelayerDelegationRefusal({
      programAddress,
      fields,
      entries,
      readAccounts: hostOf(testCase).readAccounts,
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
      "a delegated entry's store is resolved before the watermark",
    ]) {
      await expect(precheck(name), name).resolves.toBeUndefined();
    }
  });

  it('reads nothing for direct entries', async () => {
    const { fields, entries } = requestOf(caseNamed("a direct entry with the signer's allow leaf"));
    const readAccounts = vi.fn<SolanaHostAccountsReader>();
    await expect(
      solanaRelayerDelegationRefusal({ programAddress, fields, entries, readAccounts }),
    ).resolves.toBeUndefined();
    expect(readAccounts).not.toHaveBeenCalled();
  });
});

// The Connector judges a public decryption by the same store and leaf rules, which these cases' stores exercise.
describe('the cleartext client judges a public decryption as the KMS Connector does', () => {
  const judgePublic = (name: string) => {
    const testCase = caseNamed(name);
    const [entry] = testCase.entries;
    if (entry === undefined) throw new Error(`${name} has no entry`);
    return judgeSolanaPublicDecryption({
      programAddress,
      encryptedStore: getAddressDecoder().decode(bytes(entry.encrypted_store)),
      handle: bytes(entry.handle),
      ...hostOf(testCase),
    });
  };

  it('decrypts a handle whose public leaf the store sealed', async () => {
    await expect(judgePublic('an allow leaf sealed after the handle was made public')).resolves.toEqual({
      authorized: true,
    });
  });

  it('refuses a handle the store never made public', async () => {
    await expect(judgePublic("a direct entry with the signer's allow leaf")).resolves.toMatchObject({
      failure: 'HandleBinding::NoLeaf',
    });
  });

  it('refuses a store the host could not have written', async () => {
    await expect(judgePublic('a store naming the wildcard application')).resolves.toMatchObject({
      failure: 'EncryptedStore::InvalidHostRecord',
    });
  });
});
