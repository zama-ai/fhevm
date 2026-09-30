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

/**
 * The leaf-appending events of `encryptedStore`, oldest first. Each write carries the leaf count it
 * appended at, so events land at their position whatever order the RPC lists transactions in.
 * Throws when two writes claim one leaf or no write claims one, rather than returning a history
 * whose proofs would fail on chain.
 */
export async function fetchSolanaStoreHistory(
  rpc: SolanaRpc,
  encryptedStore: Address,
  programAddress: Address,
): Promise<SolanaStoreHistoryEvent[]> {
  const events: Array<SolanaStoreHistoryEvent | undefined> = [];
  const place = (previousLeafCount: bigint, appended: readonly SolanaStoreHistoryEvent[]): void => {
    appended.forEach((event, offset) => {
      const index = Number(previousLeafCount) + offset;
      if (events[index] !== undefined) throw new Error(`two writes to ${encryptedStore} claim leaf ${index}`);
      events[index] = event;
    });
  };

  for (const signature of await storeSignatures(rpc, encryptedStore)) {
    const instructions = await hostInstructions(rpc, signature, programAddress);
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
            if (key === undefined)
              throw new Error(`an fhe_execute in ${signature} allows a key outside its dictionary`);
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

  // `Array.from` visits the holes a missed write leaves.
  return Array.from(events, (event, index) => {
    if (event === undefined) throw new Error(`no confirmed write to ${encryptedStore} records leaf ${index}`);
    return event;
  });
}

/** Every successful transaction that touched `address`, paged until the RPC has no older one. */
async function storeSignatures(rpc: SolanaRpc, address: Address): Promise<Signature[]> {
  const signatures: Signature[] = [];
  for (let before: Signature | undefined; ; ) {
    const page = await rpc
      .getSignaturesForAddress(address, { commitment: 'confirmed', ...(before ? { before } : {}) })
      .send();
    signatures.push(...page.filter((entry) => entry.err === null).map((entry) => entry.signature));
    const last = page.at(-1);
    if (last === undefined) return signatures;
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
        // The generated parsers name accounts by position and never read their roles.
        return { address: key, role: AccountRole.READONLY };
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
