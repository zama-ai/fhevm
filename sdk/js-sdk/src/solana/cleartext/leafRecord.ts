// The cleartext stack's leaf record: the leaves of every `EncryptedStore` the host writes, rebuilt
// from the validator's confirmed transactions the way the coprocessors' host listener records them
// (`solana_grpc_listener.rs`, reconstruct), kept in memory for as long as the stack runs, and
// served over the listener's wire by `test-suite/fhevm/src/solana/cleartext-leaf-proofs.ts`.
import {
  AccountRole,
  fetchEncodedAccount,
  getAddressEncoder,
  getBase58Encoder,
  type AccountMeta,
  type Address,
  type ReadonlyUint8Array,
  type Signature,
} from '@solana/kit';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaStoreHistoryEvent } from '../proof.js';
import type { SolanaLeafProofOutcome, SolanaLeafProofReader } from './leafProofs.js';
import { bytesToHex } from '../../core/base/bytes.js';
import { decodeSolanaEncryptedStore, isSolanaEncryptedStoreData } from '../encryptedStore.js';
import {
  createRetainedMmr,
  historicalAccessLeafCommitment,
  publicDecryptLeafCommitment,
  type RetainedMmr,
} from '../proof.js';
import {
  FHE_EXECUTE_DISCRIMINATOR,
  parseFheExecuteInstruction,
} from '../internal/generated/zamaHost/instructions/fheExecute.js';
import {
  MAKE_STORE_HANDLE_PUBLIC_DISCRIMINATOR,
  parseMakeStoreHandlePublicInstruction,
} from '../internal/generated/zamaHost/instructions/makeStoreHandlePublic.js';
import { getFheExecutedEventDecoder } from '../internal/generated/zamaHost/types/fheExecutedEvent.js';
import { EVENT_IX_TAG, EVENT_VERSION, FHE_EXECUTED_EVENT_DISCRIMINATOR } from './hostConstants.js';

////////////////////////////////////////////////////////////////////////////////

type HostInstruction = {
  readonly programAddress: Address;
  readonly accounts: readonly AccountMeta[];
  readonly data: Uint8Array;
};

/** One store's sealed leaves, as far as `newest`, the newest of its transactions read so far. */
type StoreRecord = {
  newest: Signature | undefined;
  readonly tree: RetainedMmr;
  /** The first leaf of each `(handle, key)` or public `handle`, as the listener answers. */
  readonly firstLeaf: Map<string, number>;
};

/**
 * A store's record as one catch-up left it. A later catch-up may append to the record while a read
 * still answers from this one, so the read answers from the first `leafCount` leaves only, which no
 * catch-up changes.
 */
type CaughtUp = { readonly record: StoreRecord; readonly leafCount: number };

////////////////////////////////////////////////////////////////////////////////

const startsWith = (data: ReadonlyUint8Array, prefix: ReadonlyUint8Array): boolean =>
  data.length >= prefix.length && prefix.every((byte, index) => data[index] === byte);

const leafKey = (handle: Uint8Array, key?: Uint8Array): string =>
  key === undefined ? `public:${bytesToHex(handle)}` : `allowed:${bytesToHex(handle)}:${bytesToHex(key)}`;

/**
 * The leaf record of the stores the host at `programAddress` writes. Each read first brings every
 * store it names up to the confirmed chain, reading only the transactions newer than the last one
 * it kept, then answers from the record.
 *
 * - A store extends only by a gap-free run of new leaves that reaches the leaf count of the account,
 *   which is read before the listing. While a transaction is not yet available, or the listing or a
 *   transaction does not show a write the account holds, the record stays where it was, and the
 *   Connector reads its shorter leaf count as a record behind the chain.
 * - Once the record holds as many leaves as the store's account, its peaks at that count must be the
 *   account's.
 * - Peaks that differ, two writes that claim one leaf, a write the parser cannot read or a failed RPC
 *   call make the read throw and drop the store's record. The next read rebuilds it from the store's
 *   first transaction.
 *
 * Catch-ups of one store run in turn; different stores catch up independently. A read answers from
 * the leaf count its own catch-up checked.
 */
export function createSolanaLeafRecord(rpc: SolanaRpc, programAddress: Address): SolanaLeafProofReader {
  const records = new Map<Address, StoreRecord>();
  const catchUps = new Map<Address, Promise<CaughtUp | undefined>>();

  /** The record of `encryptedStore` up to the chain, or `undefined` when no host store lives there. */
  const catchUp = async (encryptedStore: Address): Promise<CaughtUp | undefined> => {
    const account = await fetchEncodedAccount(rpc, encryptedStore, { commitment: 'confirmed' });
    if (!account.exists || account.programAddress !== programAddress) return undefined;
    if (!isSolanaEncryptedStoreData(account.data)) return undefined;
    const live = decodeSolanaEncryptedStore(account.data, encryptedStore);

    const record = records.get(encryptedStore) ?? {
      newest: undefined,
      tree: createRetainedMmr(),
      firstLeaf: new Map(),
    };
    records.set(encryptedStore, record);
    const covered = Number(live.leafCount);
    try {
      await extend(record, encryptedStore, covered);
      if (record.tree.leafCount() >= covered) {
        const peaks = record.tree.peaks(covered).map(bytesToHex);
        if (peaks.length !== live.peaks.length || peaks.some((peak, index) => peak !== bytesToHex(live.peaks[index]))) {
          throw new Error(`the leaves rebuilt for ${encryptedStore} do not match its peaks at leaf count ${covered}`);
        }
      }
    } catch (error) {
      records.delete(encryptedStore);
      throw error;
    }
    return { record, leafCount: record.tree.leafCount() };
  };

  /**
   * Appends the leaves of the transactions newer than `record.newest`, all of them or none, and
   * none while they stop short of the `covered` leaves the account holds.
   */
  const extend = async (record: StoreRecord, encryptedStore: Address, covered: number): Promise<void> => {
    const { newest, successful } = await storeSignatures(rpc, encryptedStore, record.newest);
    if (newest === undefined) return;
    const sealed = record.tree.leafCount();
    // Each write carries the leaf count it appended at, so its leaves land at their position
    // whatever order the RPC lists transactions in.
    const appended: Array<SolanaStoreHistoryEvent | undefined> = [];
    const place = (previousLeafCount: bigint, events: readonly SolanaStoreHistoryEvent[]): void => {
      events.forEach((event, offset) => {
        const index = Number(previousLeafCount) + offset;
        if (index < sealed || appended[index - sealed] !== undefined) {
          throw new Error(`two writes to ${encryptedStore} claim leaf ${index}`);
        }
        appended[index - sealed] = event;
      });
    };
    for (const signature of successful) {
      const instructions = await hostInstructions(rpc, signature, programAddress);
      if (instructions === undefined) return;
      placeWrites(instructions, signature, encryptedStore, place);
    }
    // `Array.from` visits the holes a write the listing does not show yet leaves.
    const run = Array.from(appended);
    if (!run.every((event): event is SolanaStoreHistoryEvent => event !== undefined)) return;
    if (sealed + run.length < covered) return;

    const storeBytes = new Uint8Array(getAddressEncoder().encode(encryptedStore));
    run.forEach((event, offset) => {
      const index = sealed + offset;
      const key = event.kind === 'allowed' ? event.key : undefined;
      record.tree.append(
        key === undefined
          ? publicDecryptLeafCommitment(storeBytes, BigInt(index), event.handle)
          : historicalAccessLeafCommitment(storeBytes, BigInt(index), event.handle, key),
      );
      const found = leafKey(event.handle, key);
      if (!record.firstLeaf.has(found)) record.firstLeaf.set(found, index);
    });
    record.newest = newest;
  };

  const catchUpInTurn = (encryptedStore: Address): Promise<CaughtUp | undefined> => {
    const previous = catchUps.get(encryptedStore) ?? Promise.resolve(undefined);
    const next = previous.catch(() => undefined).then(() => catchUp(encryptedStore));
    catchUps.set(encryptedStore, next);
    return next;
  };

  return async (queries) => {
    const stores = [...new Set(queries.map(({ encryptedStore }) => encryptedStore))];
    const caught = new Map(
      await Promise.all(stores.map(async (store) => [store, await catchUpInTurn(store)] as const)),
    );
    return queries.map(({ encryptedStore, handle, key }): SolanaLeafProofOutcome => {
      const store = caught.get(encryptedStore);
      if (store === undefined) return { status: 'unknownAccount' };
      const { record, leafCount } = store;
      const leafIndex = record.firstLeaf.get(
        leafKey(handle, key === undefined ? undefined : new Uint8Array(getAddressEncoder().encode(key))),
      );
      if (leafIndex === undefined || leafIndex >= leafCount) {
        return { status: 'notFound', leafCount: BigInt(leafCount) };
      }
      return {
        status: 'found',
        leafIndex: BigInt(leafIndex),
        leafCount: BigInt(leafCount),
        siblings: record.tree.proof(leafIndex, leafCount).siblings,
      };
    });
  };
}

/** Places the events every host instruction of `signature` appended to `encryptedStore`. */
function placeWrites(
  instructions: readonly HostInstruction[],
  signature: Signature,
  encryptedStore: Address,
  place: (previousLeafCount: bigint, appended: readonly SolanaStoreHistoryEvent[]) => void,
): void {
  instructions.forEach((instruction, position) => {
    if (startsWith(instruction.data, FHE_EXECUTE_DISCRIMINATOR)) {
      const { accounts, data: execution } = parseFheExecuteInstruction(instruction);
      // An effect's `storeIndex` counts the accounts after the fixed ones the parser names.
      const stores = instruction.accounts.slice(Object.keys(accounts).length);
      const results = executedResults(instructions, position);
      for (const effect of execution.effects) {
        if (stores[effect.storeIndex]?.address !== encryptedStore) continue;
        const handle = results[effect.result.stepIndex];
        if (effect.result.outputIndex !== 0 || handle === undefined) {
          throw new Error(`an fhe_execute in ${signature} writes a result its event does not carry`);
        }
        const allowed = [...effect.allowIndexes].map((index): SolanaStoreHistoryEvent => {
          const key = execution.dictionary[index];
          if (key === undefined) throw new Error(`an fhe_execute in ${signature} allows a key outside its dictionary`);
          return { kind: 'allowed', handle, key: new Uint8Array(key) };
        });
        place(effect.previousLeafCount, [
          ...allowed,
          ...(effect.makePublic ? [{ kind: 'markedPublic', handle } as const] : []),
        ]);
      }
    } else if (startsWith(instruction.data, MAKE_STORE_HANDLE_PUBLIC_DISCRIMINATOR)) {
      const { accounts, data } = parseMakeStoreHandlePublicInstruction(instruction);
      if (accounts.encryptedStore.address !== encryptedStore) return;
      place(data.previousLeafCount, [{ kind: 'markedPublic', handle: new Uint8Array(data.handle) }]);
    }
  });
}

/**
 * The successful transactions that touched `address` after `until` (all of them without it), and
 * the newest transaction listed, failed or not, paged until the RPC has no older one.
 */
async function storeSignatures(
  rpc: SolanaRpc,
  address: Address,
  until: Signature | undefined,
): Promise<{ readonly newest: Signature | undefined; readonly successful: Signature[] }> {
  const successful: Signature[] = [];
  let newest: Signature | undefined;
  for (let before: Signature | undefined; ; ) {
    const page = await rpc
      .getSignaturesForAddress(address, {
        commitment: 'confirmed',
        ...(before ? { before } : {}),
        ...(until ? { until } : {}),
      })
      .send();
    newest ??= page[0]?.signature;
    successful.push(...page.filter((entry) => entry.err === null).map((entry) => entry.signature));
    const last = page.at(-1);
    if (last === undefined) return { newest, successful };
    before = last.signature;
  }
}

/**
 * The host program's instructions in `signature`, top-level and inner, in execution order, or
 * `undefined` while the RPC does not serve the transaction yet.
 */
async function hostInstructions(
  rpc: SolanaRpc,
  signature: Signature,
  programAddress: Address,
): Promise<HostInstruction[] | undefined> {
  const transaction = await rpc
    .getTransaction(signature, { commitment: 'confirmed', encoding: 'json', maxSupportedTransactionVersion: 0 })
    .send();
  if (transaction === null) return undefined;
  const { message } = transaction.transaction;
  const loaded = transaction.meta?.loadedAddresses;
  const keys = [...message.accountKeys, ...(loaded?.writable ?? []), ...(loaded?.readonly ?? [])];
  const roles = transactionRoles(message.header, message.accountKeys.length, loaded?.writable.length ?? 0);
  const base58 = getBase58Encoder();
  const resolve = (instruction: {
    readonly programIdIndex: number;
    readonly accounts: readonly number[];
    readonly data: string;
  }): HostInstruction | undefined => {
    if (keys[instruction.programIdIndex] !== programAddress) return undefined;
    return {
      programAddress,
      data: new Uint8Array(base58.encode(instruction.data)),
      accounts: instruction.accounts.map((index) => {
        const key = keys[index];
        if (key === undefined) throw new Error(`transaction ${signature} names account ${index} it does not load`);
        return { address: key, role: roles(index) };
      }),
    };
  };
  return message.instructions.flatMap((instruction, index) =>
    [
      instruction,
      ...(transaction.meta?.innerInstructions ?? [])
        .filter((inner) => inner.index === index)
        .flatMap((inner) => inner.instructions),
    ].flatMap((candidate) => resolve(candidate) ?? []),
  );
}

/**
 * The role of each account in a transaction, by its index in the static keys followed by the
 * writable and readonly lookup-table keys. An inner instruction may hold an account with fewer
 * privileges than the transaction grants it; the parsers read positions, never roles.
 */
function transactionRoles(
  header: {
    readonly numRequiredSignatures: number;
    readonly numReadonlySignedAccounts: number;
    readonly numReadonlyUnsignedAccounts: number;
  },
  staticCount: number,
  loadedWritableCount: number,
): (index: number) => AccountRole {
  return (index) => {
    if (index < header.numRequiredSignatures) {
      return index < header.numRequiredSignatures - header.numReadonlySignedAccounts
        ? AccountRole.WRITABLE_SIGNER
        : AccountRole.READONLY_SIGNER;
    }
    if (index < staticCount) {
      return index < staticCount - header.numReadonlyUnsignedAccounts ? AccountRole.WRITABLE : AccountRole.READONLY;
    }
    return index < staticCount + loadedWritableCount ? AccountRole.WRITABLE : AccountRole.READONLY;
  };
}

/**
 * The result handles of the `fhe_execute` at `position`, from the one `FheExecutedEvent` it emits
 * before the next execution. Only the host can sign its event authority, so a host instruction
 * carrying the event tag came from the host.
 */
function executedResults(instructions: readonly HostInstruction[], position: number): readonly Uint8Array[] {
  const next = instructions.findIndex(
    (instruction, index) => index > position && startsWith(instruction.data, FHE_EXECUTE_DISCRIMINATOR),
  );
  const emitted = instructions
    .slice(position + 1, next === -1 ? undefined : next)
    .filter(
      ({ data }) =>
        startsWith(data, EVENT_IX_TAG) &&
        startsWith(data.subarray(EVENT_IX_TAG.length), FHE_EXECUTED_EVENT_DISCRIMINATOR),
    );
  const [only] = emitted;
  if (emitted.length !== 1 || only === undefined) {
    throw new Error(`an fhe_execute is followed by ${emitted.length} FheExecutedEvents, expected 1`);
  }
  const event = getFheExecutedEventDecoder().decode(only.data, EVENT_IX_TAG.length);
  if (event.version !== EVENT_VERSION) {
    throw new Error(`FheExecutedEvent version ${event.version} is not ${EVENT_VERSION}`);
  }
  return event.results.map((result) => new Uint8Array(result));
}
