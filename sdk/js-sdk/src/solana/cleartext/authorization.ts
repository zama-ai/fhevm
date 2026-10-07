// The KMS Connector's authorization of a Solana user or public decryption
// (kms-connector/crates/kms-worker/src/core/solana/pipeline.rs and public_decrypt.rs), over the same
// host accounts, read in the same order and judged by the same rules, and the leaf proofs of one
// leaf record, asked and verified as the Connector asks and verifies a coprocessor's. The relayer's
// delegation pre-check, which runs before the Connector sees a request, is here too.
//
// solana/test-fixtures/authorization/decrypt_cases_v1.json holds the Connector's verdicts on a set
// of cases; authorization.test.ts holds this module to them.
import {
  getAddressDecoder,
  getAddressEncoder,
  type Address,
  type MaybeEncodedAccount,
  type ProgramDerivedAddress,
} from '@solana/kit';
import { findDelegationRecordPda, findEncryptedStorePda, findInvalidationPda } from '@fhevm/solana-zama-host';
import { getSysvarClockDecoder, SYSVAR_CLOCK_ADDRESS } from '@solana/sysvars';
import type { SolanaPermitFields } from '../permit/types.js';
import type { SolanaUserDecryptHandleEntry } from '../userDecrypt/index.js';
import type { SolanaMerkleProofOutcome, SolanaMerkleProofReader, SolanaLeafQuery } from './merkleProofs.js';
import { bytesToHex, concatBytes } from '../../core/base/bytes.js';
import { verifySolanaPermitSignature } from '../permit/envelope.js';
import {
  decodeSolanaEncryptedStore,
  isSolanaEncryptedStoreData,
  type SolanaEncryptedStore,
} from '../encryptedStore.js';
import { solanaPermitInvalidationWatermark } from '../actions/revokePermits.js';
import {
  decodeSolanaUserDecryptionDelegation,
  isSolanaUserDecryptionDelegationLiveAt,
  SOLANA_WILDCARD_APP,
  type SolanaDelegationApplication,
} from '../actions/userDecryptionDelegation.js';
import { mmrMountainHeight, verifyHistoricalAccessProof, verifyPublicDecryptProof } from './mmr.js';

////////////////////////////////////////////////////////////////////////////////

/** One read of the host accounts at `keys`, at a slot no older than `minContextSlot`. */
export type SolanaHostAccountsReader = (
  keys: readonly Address[],
  minContextSlot?: bigint,
) => Promise<{ readonly slot: bigint; readonly accounts: readonly MaybeEncodedAccount[] }>;

/**
 * Whether the Connector may clear each failure on a later attempt (`failure.rs`). A failed account
 * or Merkle proof read is not listed: the cleartext client throws on one instead of judging.
 */
export const CONNECTOR_FAILURE_RECOVERABLE = {
  Signature: false,
  'Window::NotYetValid': true,
  'Window::Expired': false,
  ProgramIdMismatch: false,
  'Watermark::Invalidated': false,
  'Watermark::InvalidHostRecord': false,
  'EncryptedStore::Absent': true,
  'EncryptedStore::ForeignOwner': false,
  'EncryptedStore::NotAnEncryptedStore': false,
  'EncryptedStore::AddressMismatch': false,
  'EncryptedStore::InvalidHostRecord': false,
  ScopeNotAllowed: false,
  'HandleBinding::NoLeaf': true,
  'HandleBinding::ProofRecordBehind': true,
  'HandleBinding::AccountUnknownToProofRecord': true,
  'HandleBinding::ProofDoesNotVerify': true,
  'HandleBinding::LeafIndexOutOfRange': true,
  'Delegation::NoLiveDelegation': true,
  'Delegation::InvalidHostRecord': false,
} as const satisfies Record<string, boolean>;

export type ConnectorFailure = keyof typeof CONNECTOR_FAILURE_RECOVERABLE;

export type ConnectorVerdict =
  | { readonly authorized: true }
  | {
      readonly authorized: false;
      readonly failure: ConnectorFailure;
      /** The entry the failure names, for a per-entry rule. */
      readonly entry?: number;
      readonly message: string;
    };

class ConnectorRefusal extends Error {
  constructor(
    readonly failure: ConnectorFailure,
    message: string,
    readonly entry?: number,
  ) {
    super(entry === undefined ? message : `entry ${entry}: ${message}`);
  }
}

const refuse = (failure: ConnectorFailure, message: string, entry?: number): never => {
  throw new ConnectorRefusal(failure, message, entry);
};

const SYSTEM_PROGRAM_ADDRESS = '11111111111111111111111111111111' as Address;

const decodeAddress = (bytes: Uint8Array): Address => getAddressDecoder().decode(bytes);
const encodeAddress = (address: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(address));

/**
 * The owner and data of an account the host program may have written, or `undefined` for none: a
 * bare transfer to a derivable address leaves a System-owned empty account, which says nothing
 * about host state.
 */
function initialized(account: MaybeEncodedAccount): { owner: Address; data: Uint8Array } | undefined {
  if (!account.exists) return undefined;
  if (account.programAddress === SYSTEM_PROGRAM_ADDRESS && account.data.length === 0) return undefined;
  return { owner: account.programAddress, data: account.data };
}

/** `resolve_encrypted_store`: the store an entry names, checked as the host program writes one. */
async function resolveStore(
  account: MaybeEncodedAccount,
  programAddress: Address,
  key: Address,
  entry: number,
): Promise<SolanaEncryptedStore> {
  const found = initialized(account);
  if (found === undefined) {
    return refuse('EncryptedStore::Absent', `encrypted store ${key} does not exist`, entry);
  }
  if (found.owner !== programAddress) {
    return refuse('EncryptedStore::ForeignOwner', `encrypted store ${key} is owned by ${found.owner}`, entry);
  }
  if (!isSolanaEncryptedStoreData(found.data)) {
    return refuse('EncryptedStore::NotAnEncryptedStore', `account ${key} is not an encrypted store`, entry);
  }
  let store: SolanaEncryptedStore;
  try {
    store = decodeSolanaEncryptedStore(found.data, key);
  } catch (error) {
    return refuse('EncryptedStore::InvalidHostRecord', String(error), entry);
  }
  const [derived, bump] = await findEncryptedStorePda(store, { programAddress });
  if (derived !== key || bump !== store.bump) {
    return refuse(
      'EncryptedStore::AddressMismatch',
      `encrypted store ${key} has bump ${store.bump}; its fields derive ${derived} with bump ${bump}`,
      entry,
    );
  }
  // With the sentinel as program, the store's delegation row would be the wildcard row itself.
  if (store.program === SOLANA_WILDCARD_APP.program || store.scope === SOLANA_WILDCARD_APP.scope) {
    return refuse('EncryptedStore::InvalidHostRecord', `encrypted store ${key} names the wildcard application`, entry);
  }
  return store;
}

/** A delegation row: its derived address and bump, and the application it is for. */
type DelegationRow = SolanaDelegationApplication & { readonly pda: ProgramDerivedAddress };

/** The two rows that can authorize a delegated entry: its store's application, then the wildcard. */
async function delegationRows(
  delegator: Address,
  delegate: Address,
  store: SolanaEncryptedStore,
  programAddress: Address,
): Promise<readonly [DelegationRow, DelegationRow]> {
  const row = async ({ program, scope }: SolanaDelegationApplication): Promise<DelegationRow> => ({
    pda: await findDelegationRecordPda({ delegator, delegate, program, scope }, { programAddress }),
    program,
    scope,
  });
  return [await row(store), await row(SOLANA_WILDCARD_APP)];
}

/** `judge_delegation_row`: what one row says at host time `now`. */
function judgeRow(
  account: MaybeEncodedAccount,
  row: DelegationRow,
  delegator: Address,
  delegate: Address,
  programAddress: Address,
  now: bigint,
): 'live' | 'dead' | 'invalid' {
  const found = initialized(account);
  if (found === undefined) return 'dead';
  if (found.owner !== programAddress) return 'invalid';
  try {
    const record = decodeSolanaUserDecryptionDelegation(found.data, row.pda[0]);
    if (
      record.delegator !== delegator ||
      record.delegate !== delegate ||
      record.program !== row.program ||
      record.scope !== row.scope ||
      record.bump !== row.pda[1]
    ) {
      return 'invalid';
    }
    return isSolanaUserDecryptionDelegationLiveAt(record, now) ? 'live' : 'dead';
  } catch {
    return 'invalid';
  }
}

/** Hands out the answers of one read in the order they were asked for, as the Connector's `observe` does. */
function cursor<T>(answers: readonly T[], source: string): () => T {
  let position = 0;
  return () => {
    const answer = answers[position];
    if (answer === undefined) throw new Error(`${source} returned ${answers.length} answers, fewer than asked`);
    position += 1;
    return answer;
  };
}

/**
 * `verify_proofs` over one leaf record: every leaf asked in one read, then judged in order, so the
 * first entry whose leaf is not proven is the one refused.
 */
async function proveLeaves(
  readMerkleProofs: SolanaMerkleProofReader,
  leaves: ReadonlyArray<{ readonly store: SolanaEncryptedStore; readonly query: SolanaLeafQuery }>,
): Promise<void> {
  const outcomes = await readMerkleProofs(leaves.map(({ query }) => query));
  if (outcomes.length !== leaves.length) {
    throw new Error(`the leaf record answered ${outcomes.length} proofs for ${leaves.length} queries`);
  }
  const next = cursor(outcomes, 'the leaf record');
  leaves.forEach(({ store, query }, index) => {
    checkLeaf(store, query, next(), index);
  });
}

/**
 * `check_leaf`: whether the record's answer proves the queried leaf against the observed store. A
 * proof is verified before the record's age is considered, since one built at a larger leaf count
 * still verifies whenever the leaf's mountain has not merged since. Without a proof, a record at
 * least as long as the store holds no such leaf, and a shorter one may catch up.
 */
function checkLeaf(
  store: SolanaEncryptedStore,
  { encryptedStore, handle, key }: SolanaLeafQuery,
  outcome: SolanaMerkleProofOutcome,
  index: number,
): void {
  const live = store.leafCount;
  switch (outcome.status) {
    case 'notFound':
      if (outcome.leafCount >= live) {
        return refuse(
          'HandleBinding::NoLeaf',
          `no leaf for this key and handle in ${outcome.leafCount} of ${live} leaves of ${encryptedStore}`,
          index,
        );
      }
      return refuse(
        'HandleBinding::ProofRecordBehind',
        `the leaf record is behind the chain (${outcome.leafCount} of ${live} leaves of ${encryptedStore})`,
        index,
      );
    case 'unknownAccount':
      return refuse(
        'HandleBinding::AccountUnknownToProofRecord',
        `the leaf record does not know ${encryptedStore}`,
        index,
      );
    case 'found':
      break;
  }
  const { leafIndex, siblings } = outcome;
  if (leafIndex >= live) {
    return refuse(
      'HandleBinding::LeafIndexOutOfRange',
      `leaf index ${leafIndex} is not below the observed leaf count ${live}`,
      index,
    );
  }
  // `MmrProof::for_leaf_count`: as many siblings as reach the leaf's peak at the observed count.
  const proof = { leafIndex, siblings: siblings.slice(0, mmrMountainHeight(leafIndex, live)) };
  const storeBytes = encodeAddress(encryptedStore);
  const verifies =
    key === undefined
      ? verifyPublicDecryptProof(storeBytes, store.peaks, live, handle, proof)
      : verifyHistoricalAccessProof(storeBytes, store.peaks, live, handle, encodeAddress(key), proof);
  if (!verifies) {
    refuse(
      'HandleBinding::ProofDoesNotVerify',
      `the leaf proof does not verify against the observed peaks (record ${outcome.leafCount} leaves, chain ${live})`,
      index,
    );
  }
}

function verdictOf(error: unknown): ConnectorVerdict {
  if (!(error instanceof ConnectorRefusal)) throw error;
  return {
    authorized: false,
    failure: error.failure,
    ...(error.entry === undefined ? {} : { entry: error.entry }),
    message: error.message,
  };
}

/**
 * The relayer's advisory pre-check of delegated entries (`relayer/src/host/solana_delegation_precheck.rs`),
 * run before it spends a gateway transaction: why an entry's two delegation rows are both dead at
 * the host's Clock, or `undefined`. What it cannot judge, a store that does not resolve or a row
 * the host could not have written, passes for the Connector to decide.
 */
export async function solanaRelayerDelegationRefusal({
  programAddress,
  fields,
  entries,
  readAccounts,
}: {
  readonly programAddress: Address;
  readonly fields: SolanaPermitFields;
  readonly entries: readonly SolanaUserDecryptHandleEntry[];
  readonly readAccounts: SolanaHostAccountsReader;
}): Promise<string | undefined> {
  const delegate = decodeAddress(fields.userAddress);
  const delegated = entries
    .map((entry) => ({
      key: decodeAddress(entry.encryptedStore),
      delegator: decodeAddress(entry.ownerAddress),
    }))
    .filter(({ delegator }) => delegator !== delegate);
  if (delegated.length === 0) return undefined;

  const first = await readAccounts(delegated.map(({ key }) => key));
  const nextStore = cursor(first.accounts, 'the host read');
  const planned = [];
  for (const [index, { key, delegator }] of delegated.entries()) {
    const account = nextStore();
    let store: SolanaEncryptedStore;
    try {
      store = await resolveStore(account, programAddress, key, index);
    } catch (error) {
      if (error instanceof ConnectorRefusal) continue;
      throw error;
    }
    planned.push({ delegator, rows: await delegationRows(delegator, delegate, store, programAddress) });
  }
  if (planned.length === 0) return undefined;

  const rowKeys = planned.flatMap(({ rows }) => rows.map((row) => row.pda[0]));
  const second = await readAccounts([...rowKeys, SYSVAR_CLOCK_ADDRESS], first.slot);
  const nextRow = cursor(second.accounts, 'the host read');
  const judged = planned.map((entry) => ({ ...entry, accounts: [nextRow(), nextRow()] as const }));
  const clock = nextRow();
  if (!clock.exists) return undefined;
  const now = getSysvarClockDecoder().decode(clock.data).unixTimestamp;
  for (const { delegator, rows, accounts } of judged) {
    const verdicts = [
      judgeRow(accounts[0], rows[0], delegator, delegate, programAddress, now),
      judgeRow(accounts[1], rows[1], delegator, delegate, programAddress, now),
    ];
    if (verdicts.every((verdict) => verdict === 'dead')) {
      return `${delegator} has no live delegation to ${delegate} at the host's clock ${now}`;
    }
  }
  return undefined;
}

/**
 * `check_public_decrypt`: every handle's store, read in one snapshot and judged before any leaf, and
 * then each handle's public leaf proven against its store.
 */
export async function judgeSolanaPublicDecryption({
  programAddress,
  handles,
  readAccounts,
  readMerkleProofs,
}: {
  readonly programAddress: Address;
  readonly handles: ReadonlyArray<{ readonly handle: Uint8Array; readonly encryptedStore: Address }>;
  readonly readAccounts: SolanaHostAccountsReader;
  readonly readMerkleProofs: SolanaMerkleProofReader;
}): Promise<ConnectorVerdict> {
  try {
    const { accounts } = await readAccounts(handles.map(({ encryptedStore }) => encryptedStore));
    const next = cursor(accounts, 'the host read');
    const leaves = [];
    for (const [index, { handle, encryptedStore }] of handles.entries()) {
      leaves.push({
        store: await resolveStore(next(), programAddress, encryptedStore, index),
        query: { encryptedStore, handle },
      });
    }
    await proveLeaves(readMerkleProofs, leaves);
    return { authorized: true };
  } catch (error) {
    return verdictOf(error);
  }
}

/**
 * The Connector's verdict on a user decryption: `authorized`, or the first failure in its order of
 * checks. `now` is the Connector's own clock, which bounds the permit's validity window; delegation
 * rows are judged at the host's Clock, read in the same snapshot as the rows.
 */
export async function judgeSolanaUserDecryption({
  programAddress,
  now,
  fields,
  signature,
  entries,
  readAccounts,
  readMerkleProofs,
}: {
  readonly programAddress: Address;
  readonly now: bigint;
  readonly fields: SolanaPermitFields;
  readonly signature: Uint8Array;
  readonly entries: readonly SolanaUserDecryptHandleEntry[];
  readonly readAccounts: SolanaHostAccountsReader;
  readonly readMerkleProofs: SolanaMerkleProofReader;
}): Promise<ConnectorVerdict> {
  try {
    try {
      verifySolanaPermitSignature(fields, signature);
    } catch (error) {
      refuse('Signature', `the permit signature does not verify: ${String(error)}`);
    }
    const end = fields.startTimestamp + fields.durationSeconds;
    if (fields.startTimestamp > now) {
      refuse('Window::NotYetValid', `the permit starts at ${fields.startTimestamp}, later than now ${now}`);
    }
    if (now > end) refuse('Window::Expired', `the permit expired at ${end}, now ${now}`);
    const signedProgram = decodeAddress(fields.verifyingProgramId);
    if (signedProgram !== programAddress) {
      refuse('ProgramIdMismatch', `the permit names program ${signedProgram}, this host is ${programAddress}`);
    }

    // A delegation row's address depends on the application of the entry's store, so a delegated
    // request needs a second read, no older than the first. The first read only locates the rows;
    // every rule that authorizes is judged against the last read.
    const signer = decodeAddress(fields.userAddress);
    const watermarkPda = await findInvalidationPda({ user: signer }, { programAddress });
    const claims = entries.map((entry, index) => ({
      index,
      handle: entry.handle,
      storeKey: decodeAddress(entry.encryptedStore),
      owner: decodeAddress(entry.ownerAddress),
    }));
    const storeKeys = claims.map(({ storeKey }) => storeKey);
    const first = await readAccounts([watermarkPda[0], ...storeKeys]);
    const firstRead = cursor(first.accounts, 'the host read');
    // The watermark is judged on the last read.
    firstRead();
    const located = [];
    for (const claim of claims) {
      const account = firstRead();
      const rows =
        claim.owner === signer
          ? undefined
          : await delegationRows(
              claim.owner,
              signer,
              await resolveStore(account, programAddress, claim.storeKey, claim.index),
              programAddress,
            );
      located.push({ ...claim, rows });
    }

    const rowKeys = located.flatMap(({ rows }) => (rows ?? []).map((row) => row.pda[0]));
    const delegated = rowKeys.length > 0;
    const last = delegated
      ? await readAccounts([watermarkPda[0], SYSVAR_CLOCK_ADDRESS, ...storeKeys, ...rowKeys], first.slot)
      : first;
    const lastRead = cursor(last.accounts, 'the host read');
    const watermarkAccount = lastRead();
    let hostNow = 0n;
    if (delegated) {
      const clock = lastRead();
      if (!clock.exists) throw new Error('the host read returned no Clock sysvar');
      hostNow = getSysvarClockDecoder().decode(clock.data).unixTimestamp;
    }
    // The read lists every store before any row, so each claim takes its store, then its rows.
    const withStores = located.map((claim) => ({ ...claim, account: lastRead() }));
    const judged = withStores.map(({ rows, ...claim }) => ({
      ...claim,
      delegation: rows === undefined ? undefined : { rows, accounts: [lastRead(), lastRead()] as const },
    }));

    let watermark = 0n;
    try {
      watermark = solanaPermitInvalidationWatermark(watermarkAccount, watermarkPda, signer, programAddress);
    } catch (error) {
      refuse('Watermark::InvalidHostRecord', String(error));
    }
    if (fields.startTimestamp < watermark) {
      refuse(
        'Watermark::Invalidated',
        `the permit starts at ${fields.startTimestamp}, before ${signer} revoked permits at ${watermark}`,
      );
    }

    // Every host rule is judged before any leaf.
    const allowedScopes = new Set(fields.allowedScopes.map((scope) => bytesToHex(scope)));
    const resolved = [];
    for (const { index, handle, storeKey, owner, account, delegation } of judged) {
      const store = await resolveStore(account, programAddress, storeKey, index);
      const application = bytesToHex(concatBytes(encodeAddress(store.program), encodeAddress(store.scope)));
      if (allowedScopes.size > 0 && !allowedScopes.has(application)) {
        refuse('ScopeNotAllowed', `application (${store.program}, ${store.scope}) is outside the signed scope`, index);
      }
      if (delegation !== undefined) {
        const { rows, accounts } = delegation;
        const verdicts = [
          judgeRow(accounts[0], rows[0], owner, signer, programAddress, hostNow),
          judgeRow(accounts[1], rows[1], owner, signer, programAddress, hostNow),
        ];
        if (verdicts.includes('invalid')) {
          refuse(
            'Delegation::InvalidHostRecord',
            'a delegation row holds a record the host could not have written',
            index,
          );
        }
        if (!verdicts.includes('live')) {
          refuse('Delegation::NoLiveDelegation', `${owner} has no live delegation to ${signer} at ${hostNow}`, index);
        }
      }
      // The leaf must name the entry's owner: the signer for a direct entry, the delegator for a
      // delegated one.
      resolved.push({ store, query: { encryptedStore: storeKey, handle, key: owner } });
    }
    await proveLeaves(readMerkleProofs, resolved);
    return { authorized: true };
  } catch (error) {
    return verdictOf(error);
  }
}
