// The leaf history of an `EncryptedStore`, rebuilt from the confirmed host transactions that wrote
// to it, the way the coprocessor's host-listener records it (`solana_grpc_listener.rs`,
// reconstruct). With no coprocessor, this is where a cleartext stack's leaf proofs come from.
import {
  AccountRole,
  getBase58Encoder,
  type AccountMeta,
  type Address,
  type ReadonlyUint8Array,
  type Signature,
} from '@solana/kit';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaStoreHistoryEvent } from '../proof.js';
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

////////////////////////////////////////////////////////////////////////////////

const startsWith = (data: ReadonlyUint8Array, prefix: ReadonlyUint8Array): boolean =>
  data.length >= prefix.length && prefix.every((byte, index) => data[index] === byte);

/** Reads the leaf-appending events of an `EncryptedStore`, oldest first. */
export type SolanaStoreHistoryReader = (encryptedStore: Address) => Promise<SolanaStoreHistoryEvent[]>;

/** The leaf-appending events of one store, as far as its newest transaction seen so far. */
type SeenHistory = { readonly newest: Signature; readonly events: ReadonlyArray<SolanaStoreHistoryEvent | undefined> };

/**
 * Reads the leaf-appending events of each `EncryptedStore`, oldest first. The reader keeps what it
 * read, so a later read fetches only the transactions newer than the last one it saw; a validator
 * reset, which drops that transaction, makes it read the store from the start again.
 *
 * Each write carries the leaf count it appended at, so events land at their position whatever order
 * the RPC lists transactions in. A read throws when two writes claim one leaf or no write claims
 * one, rather than returning a history whose proofs would fail on chain.
 */
export function createSolanaStoreHistoryReader(rpc: SolanaRpc, programAddress: Address): SolanaStoreHistoryReader {
  const seen = new Map<Address, SeenHistory>();
  // Reads of one store run in turn, so each extends the history the one before it kept.
  const reads = new Map<Address, Promise<unknown>>();

  const extend = async (encryptedStore: Address): Promise<SolanaStoreHistoryEvent[]> => {
    let base = seen.get(encryptedStore);
    if (base !== undefined) {
      const { value } = await rpc.getSignatureStatuses([base.newest], { searchTransactionHistory: true }).send();
      if (value[0] === null) base = undefined;
    }
    const { newest, successful } = await storeSignatures(rpc, encryptedStore, base?.newest);
    const events = [...(base?.events ?? [])];
    const place = (previousLeafCount: bigint, appended: readonly SolanaStoreHistoryEvent[]): void => {
      appended.forEach((event, offset) => {
        const index = Number(previousLeafCount) + offset;
        if (events[index] !== undefined) throw new Error(`two writes to ${encryptedStore} claim leaf ${index}`);
        events[index] = event;
      });
    };
    for (const signature of successful) {
      placeWrites(await hostInstructions(rpc, signature, programAddress), signature, encryptedStore, place);
    }
    // `Array.from` visits the holes a missed write leaves. A history with one is never kept, since
    // later reads would not look behind its newest transaction again.
    const history = Array.from(events, (event, index) => {
      if (event === undefined) throw new Error(`no confirmed write to ${encryptedStore} records leaf ${index}`);
      return event;
    });
    const last = newest ?? base?.newest;
    if (last !== undefined) seen.set(encryptedStore, { newest: last, events });
    return history;
  };

  return (encryptedStore) => {
    const previous = reads.get(encryptedStore) ?? Promise.resolve();
    const read = previous.catch(() => undefined).then(() => extend(encryptedStore));
    reads.set(encryptedStore, read);
    return read;
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

/** The host program's instructions in `signature`, top-level and inner, in execution order. */
async function hostInstructions(
  rpc: SolanaRpc,
  signature: Signature,
  programAddress: Address,
): Promise<HostInstruction[]> {
  const transaction = await rpc
    .getTransaction(signature, { commitment: 'confirmed', encoding: 'json', maxSupportedTransactionVersion: 0 })
    .send();
  if (transaction === null) throw new Error(`transaction ${signature} is not available from the RPC`);
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
