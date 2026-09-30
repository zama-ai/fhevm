// The KMS Connector's authorization of a Solana user or public decryption
// (kms-connector/crates/kms-worker/src/core/solana/pipeline.rs and public_decrypt.rs), over the same
// host accounts, read in the same order and judged by the same rules. The cleartext client asks no coprocessor: the
// history it rebuilds from a store's transactions stands in for the coprocessors' leaf record, and
// a leaf from it is proven against the observed store as the Connector proves a coprocessor's.
//
// solana/test-fixtures/authorization/user_decrypt_cases_v1.json holds the Connector's verdicts on a
// set of cases; authorization.test.ts holds this module to them.
import {
  getAddressDecoder,
  getAddressEncoder,
  getProgramDerivedAddress,
  isOffCurveAddress,
  type Address,
  type MaybeEncodedAccount,
  type ProgramDerivedAddress,
} from '@solana/kit';
import { getSysvarClockDecoder, SYSVAR_CLOCK_ADDRESS } from '@solana/sysvars';
import { sha256 } from '@noble/hashes/sha2.js';
import type { SolanaPermitFields } from '../permit/types.js';
import type { SolanaUserDecryptHandleEntry } from '../userDecrypt/index.js';
import type { SolanaStoreHistoryEvent } from '../proof.js';
import type { SolanaStoreHistoryReader } from './storeHistory.js';
import { bytesToHex, concatBytes } from '../../core/base/bytes.js';
import { verifySolanaPermitSignature } from '../permit/envelope.js';
import {
  decodeSolanaEncryptedStore,
  ENCRYPTED_STORE_DISCRIMINATOR,
  SOLANA_ENCRYPTED_STORE_SEED,
  type SolanaEncryptedStore,
} from '../encryptedStore.js';
import { SOLANA_PERMIT_INVALIDATION_SEED, solanaPermitInvalidationWatermark } from '../actions/revokePermits.js';
import {
  decodeSolanaUserDecryptionDelegation,
  isSolanaUserDecryptionDelegationLiveAt,
  SOLANA_USER_DECRYPTION_DELEGATION_SEED,
  SOLANA_WILDCARD_APP,
} from '../actions/userDecryptionDelegation.js';
import {
  mmrBuildProof,
  reconstructSolanaStoreHistory,
  verifyHistoricalAccessProof,
  verifyPublicDecryptProof,
} from '../proof.js';

////////////////////////////////////////////////////////////////////////////////

/** One read of the host accounts at `keys`, at a slot no older than `minContextSlot`. */
export type SolanaHostAccountsReader = (
  keys: readonly Address[],
  minContextSlot?: bigint,
) => Promise<{ readonly slot: bigint; readonly accounts: readonly MaybeEncodedAccount[] }>;

/**
 * Whether the Connector may clear each failure on a later attempt (`failure.rs`). The failures the
 * cleartext client cannot produce, such as a coprocessor that is down, are not listed.
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
  'HandleBinding::ProofDoesNotVerify': true,
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
const PDA_MARKER = new TextEncoder().encode('ProgramDerivedAddress');

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

/** Solana's `create_program_address`: the PDA of `seeds` and `bump`, or `undefined` when they give none. */
function createProgramAddress(
  seeds: readonly Uint8Array[],
  bump: number,
  programAddress: Address,
): Address | undefined {
  const candidate = decodeAddress(
    sha256(concatBytes(...seeds, Uint8Array.of(bump), encodeAddress(programAddress), PDA_MARKER)),
  );
  return isOffCurveAddress(candidate) ? candidate : undefined;
}

/** `resolve_encrypted_store`: the store an entry names, checked as the host program writes one. */
function resolveStore(
  account: MaybeEncodedAccount,
  programAddress: Address,
  key: Address,
  entry: number,
): SolanaEncryptedStore {
  const found = initialized(account);
  if (found === undefined) {
    return refuse('EncryptedStore::Absent', `encrypted store ${key} does not exist`, entry);
  }
  if (found.owner !== programAddress) {
    return refuse('EncryptedStore::ForeignOwner', `encrypted store ${key} is owned by ${found.owner}`, entry);
  }
  if (!ENCRYPTED_STORE_DISCRIMINATOR.every((byte, index) => found.data[index] === byte)) {
    return refuse('EncryptedStore::NotAnEncryptedStore', `account ${key} is not an encrypted store`, entry);
  }
  let store: SolanaEncryptedStore;
  try {
    store = decodeSolanaEncryptedStore(found.data, key);
  } catch (error) {
    return refuse('EncryptedStore::InvalidHostRecord', String(error), entry);
  }
  const derived = createProgramAddress(
    [SOLANA_ENCRYPTED_STORE_SEED, ...[store.program, store.authority, store.scope].map(encodeAddress)],
    store.bump,
    programAddress,
  );
  if (derived !== key) {
    return refuse(
      'EncryptedStore::AddressMismatch',
      `encrypted store ${key} does not live at the address its fields derive (${derived ?? 'none'})`,
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
type DelegationRow = { readonly pda: ProgramDerivedAddress; readonly program: Address; readonly scope: Address };

async function delegationRow(
  delegator: Address,
  delegate: Address,
  { program, scope }: { readonly program: Address; readonly scope: Address },
  programAddress: Address,
): Promise<DelegationRow> {
  const pda = await getProgramDerivedAddress({
    programAddress,
    seeds: [SOLANA_USER_DECRYPTION_DELEGATION_SEED, ...[delegator, delegate, program, scope].map(encodeAddress)],
  });
  return { pda, program, scope };
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

/** Hands out the accounts of one read in key order, as the Connector's `observe` does. */
function cursor(accounts: readonly MaybeEncodedAccount[]): () => MaybeEncodedAccount {
  let position = 0;
  return () => {
    const account = accounts[position];
    if (account === undefined) throw new Error(`the host read returned ${accounts.length} accounts, fewer than asked`);
    position += 1;
    return account;
  };
}

/** The leaf a decryption needs: an allow of `key` on `handle`, or `handle` made public. */
type RequiredLeaf = { readonly handle: Uint8Array; readonly key?: Uint8Array };

/** `check_handle_binding` and `check_public_binding`, with the rebuilt history as the leaf record. */
function proveLeaf(
  storeKey: Address,
  store: SolanaEncryptedStore,
  history: readonly SolanaStoreHistoryEvent[],
  { handle, key }: RequiredLeaf,
  index: number,
): void {
  // The history is read after the store and can run past it: only the leaves the observed store
  // had sealed are proven against it.
  const sealed = history.slice(0, Number(store.leafCount));
  const handleHex = bytesToHex(handle);
  const leafIndex = sealed.findIndex((event) =>
    key === undefined
      ? event.kind === 'markedPublic' && bytesToHex(event.handle) === handleHex
      : event.kind === 'allowed' && bytesToHex(event.handle) === handleHex && bytesToHex(event.key) === bytesToHex(key),
  );
  if (leafIndex < 0) {
    if (BigInt(sealed.length) >= store.leafCount) {
      refuse(
        'HandleBinding::NoLeaf',
        key === undefined
          ? `handle ${handleHex} was not made public in ${storeKey}`
          : `no leaf allows ${decodeAddress(key)} on handle ${handleHex} in ${storeKey}`,
        index,
      );
    }
    refuse(
      'HandleBinding::ProofRecordBehind',
      `the history holds ${sealed.length} of ${store.leafCount} leaves`,
      index,
    );
  }
  const storeBytes = encodeAddress(storeKey);
  const proof = mmrBuildProof(reconstructSolanaStoreHistory(storeBytes, sealed).leaves, BigInt(leafIndex));
  const verifies =
    proof !== undefined &&
    (key === undefined
      ? verifyPublicDecryptProof(storeBytes, store.peaks, store.leafCount, handle, proof)
      : verifyHistoricalAccessProof(storeBytes, store.peaks, store.leafCount, handle, key, proof));
  if (!verifies) {
    refuse('HandleBinding::ProofDoesNotVerify', `the leaf does not verify against the peaks of ${storeKey}`, index);
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

/** `check_public_decrypt` for one handle: the store it names, and its public leaf proven against it. */
export async function judgeSolanaPublicDecryption({
  programAddress,
  encryptedStore,
  handle,
  readAccounts,
  readHistory,
}: {
  readonly programAddress: Address;
  readonly encryptedStore: Address;
  readonly handle: Uint8Array;
  readonly readAccounts: SolanaHostAccountsReader;
  readonly readHistory: SolanaStoreHistoryReader;
}): Promise<ConnectorVerdict> {
  try {
    const { accounts } = await readAccounts([encryptedStore]);
    const store = resolveStore(cursor(accounts)(), programAddress, encryptedStore, 0);
    proveLeaf(encryptedStore, store, await readHistory(encryptedStore), { handle }, 0);
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
  readHistory,
}: {
  readonly programAddress: Address;
  readonly now: bigint;
  readonly fields: SolanaPermitFields;
  readonly signature: Uint8Array;
  readonly entries: readonly SolanaUserDecryptHandleEntry[];
  readonly readAccounts: SolanaHostAccountsReader;
  readonly readHistory: SolanaStoreHistoryReader;
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
    const watermarkPda = await getProgramDerivedAddress({
      programAddress,
      seeds: [SOLANA_PERMIT_INVALIDATION_SEED, encodeAddress(signer)],
    });
    const storeKeys = entries.map((entry) => decodeAddress(entry.encryptedStore));
    const first = await readAccounts([watermarkPda[0], ...storeKeys]);
    const firstAccount = cursor(first.accounts);
    firstAccount();
    const located = [];
    for (const [index, entry] of entries.entries()) {
      const key = decodeAddress(entry.encryptedStore);
      const owner = decodeAddress(entry.ownerAddress);
      const account = firstAccount();
      let rows: readonly [DelegationRow, DelegationRow] | undefined;
      if (owner !== signer) {
        const store = resolveStore(account, programAddress, key, index);
        rows = [
          await delegationRow(owner, signer, store, programAddress),
          await delegationRow(owner, signer, SOLANA_WILDCARD_APP, programAddress),
        ];
      }
      located.push({ index, entry, key, owner, rows });
    }

    const rowKeys = located.flatMap(({ rows }) => (rows ?? []).map((row) => row.pda[0]));
    const delegated = rowKeys.length > 0;
    const last = delegated
      ? await readAccounts([watermarkPda[0], SYSVAR_CLOCK_ADDRESS, ...storeKeys, ...rowKeys], first.slot)
      : first;
    const next = cursor(last.accounts);
    const watermarkAccount = next();
    let hostNow = 0n;
    if (delegated) {
      const clock = next();
      if (!clock.exists) throw new Error('the host read returned no Clock sysvar');
      hostNow = getSysvarClockDecoder().decode(clock.data).unixTimestamp;
    }
    const observed = located.map((claim) => ({ ...claim, account: next() }));
    const judged = observed.map((claim) => ({
      ...claim,
      rowAccounts: claim.rows === undefined ? undefined : ([next(), next()] as const),
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
    const resolved = judged.map(({ index, entry, key, owner, rows, account, rowAccounts }) => {
      const store = resolveStore(account, programAddress, key, index);
      const application = bytesToHex(concatBytes(encodeAddress(store.program), encodeAddress(store.scope)));
      if (allowedScopes.size > 0 && !allowedScopes.has(application)) {
        refuse('ScopeNotAllowed', `application (${store.program}, ${store.scope}) is outside the signed scope`, index);
      }
      if (rows !== undefined && rowAccounts !== undefined) {
        const verdicts = [
          judgeRow(rowAccounts[0], rows[0], owner, signer, programAddress, hostNow),
          judgeRow(rowAccounts[1], rows[1], owner, signer, programAddress, hostNow),
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
      return { index, entry, key, store };
    });

    // The leaf must name the entry's owner: the signer for a direct entry, the delegator for a
    // delegated one.
    const histories = new Map<Address, readonly SolanaStoreHistoryEvent[]>();
    for (const { index, entry, key, store } of resolved) {
      const history = histories.get(key) ?? (await readHistory(key));
      histories.set(key, history);
      proveLeaf(key, store, history, { handle: entry.handle, key: entry.ownerAddress }, index);
    }
    return { authorized: true };
  } catch (error) {
    return verdictOf(error);
  }
}
