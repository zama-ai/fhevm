import {
  createSolanaRpc,
  getAddressEncoder,
  type Address,
  type TransactionSigner,
} from '@solana/kit';
import {
  LOOKUP_TABLE_DEACTIVATION_COOLDOWN_SLOTS,
  LOOKUP_TABLE_STILL_ACTIVE,
  decodeLookupTableDeactivationSlot,
  getCloseLookupTableInstruction,
  getBatchByIndex,
  getCurrentBatch,
  getDeactivateLookupTableInstruction,
  getExtendLookupTableInstructions,
  openBatchForBatcher,
} from './vault/index.js';

import { BatchStatus, isBatchFinished, type VaultDirection } from './batchTypes';
import type { DemoConfig } from './demoConfig';
import { sendTransaction } from './sendTransaction';
import { vaultRoots } from './vaultRoots';

const PROVISIONING_COMPUTE_UNIT_LIMIT = 800_000;
const LOOKUP_TABLE_COMPUTE_UNIT_LIMIT = 300_000;
const LOOKUP_TABLE_HEADER_BYTES = 56;
const LOOKUP_TABLE_PROGRAM = 'AddressLookupTab1e1111111111111111111111111';

const addressEncoder = getAddressEncoder();

const lookupTablePrefixLength = async (
  config: DemoConfig,
  lookupTable: Address,
  expectedAddresses: readonly Address[],
): Promise<number | null> => {
  const account = await createSolanaRpc(config.rpcUrl)
    .getAccountInfo(lookupTable, { commitment: 'confirmed', encoding: 'base64' })
    .send();
  if (account.value === null || account.value.owner !== LOOKUP_TABLE_PROGRAM) return null;
  const encoded = account.value.data as readonly [string, 'base64'];
  const bytes = Buffer.from(encoded[0], 'base64');
  if (bytes.length < LOOKUP_TABLE_HEADER_BYTES || (bytes.length - LOOKUP_TABLE_HEADER_BYTES) % 32 !== 0) return null;
  const addressCount = (bytes.length - LOOKUP_TABLE_HEADER_BYTES) / 32;
  if (addressCount > expectedAddresses.length) return null;
  for (let index = 0; index < addressCount; index += 1) {
    const expected = addressEncoder.encode(expectedAddresses[index]!);
    const actual = bytes.subarray(LOOKUP_TABLE_HEADER_BYTES + index * 32, LOOKUP_TABLE_HEADER_BYTES + (index + 1) * 32);
    if (!actual.equals(Buffer.from(expected))) return null;
  }
  return addressCount;
};

/**
 * One batch's lookup-table record. `table` is the table settlement will use; `retired` holds every
 * address this batch derived before it — a table whose create landed while a later step did not,
 * or one whose contents no longer match what settlement needs. Those are still keeper-owned and
 * still paying rent, so they stay recorded until the crank has deactivated and closed them.
 * Dropping them from the record is how a table becomes unreclaimable: nothing else remembers the
 * address, because it is derived from a slot that has passed.
 */
type BatchLookupEntry = {
  readonly table: string;
  readonly retired?: readonly string[];
};
type BatchLookupRegistry = Record<string, BatchLookupEntry>;

const registryKey = (
  config: DemoConfig,
  direction: VaultDirection,
  batchIndex: bigint,
  batch: Address,
): string => `${config.chainId}:${config.batchers[direction].batcher}:${batchIndex.toString()}:${batch}`;

/** `chainId:batcher:batchIndex:batch` — the batch index is the third field. */
const batchIndexFromKey = (key: string): bigint | null => {
  const field = key.split(':')[2];
  if (field === undefined || !/^\d+$/.test(field)) return null;
  return BigInt(field);
};

const normalizeEntry = (value: unknown): BatchLookupEntry | null => {
  // A record written before the entry grew a `retired` list is a bare address string.
  if (typeof value === 'string') return { table: value };
  if (typeof value !== 'object' || value === null || Array.isArray(value)) return null;
  const candidate = value as { table?: unknown; retired?: unknown };
  if (typeof candidate.table !== 'string') return null;
  if (candidate.retired !== undefined) {
    if (!Array.isArray(candidate.retired) || candidate.retired.some((entry) => typeof entry !== 'string')) {
      return null;
    }
  }
  const retired = (candidate.retired ?? []) as readonly string[];
  return retired.length === 0 ? { table: candidate.table } : { table: candidate.table, retired };
};

/**
 * Reads the registry, treating only "the file does not exist yet" as an empty one.
 *
 * Every other failure throws, because the caller writes what this returns straight back: a
 * permission error, a truncated file, or one unreadable entry read as "no tables recorded" would be
 * persisted over the record of every table this direction still owns. Those tables are rent-bearing
 * and their addresses are derived from a slot that has already passed, so nothing else can name them
 * afterwards — the leak the crank exists to prevent. Refusing to provision is the recoverable
 * outcome: the file is still there to be repaired.
 */
const readRegistry = async (registryPath: string): Promise<BatchLookupRegistry> => {
  const fs = await import('node:fs/promises');
  let contents: string;
  try {
    contents = await fs.readFile(registryPath, 'utf8');
  } catch (error) {
    if ((error as { code?: unknown }).code === 'ENOENT') return {};
    throw new Error(`Cannot read the batch lookup registry at ${registryPath}: ${String(error)}`);
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(contents) as unknown;
  } catch (error) {
    throw new Error(`The batch lookup registry at ${registryPath} is not valid JSON: ${String(error)}`);
  }
  if (typeof parsed !== 'object' || parsed === null || Array.isArray(parsed)) {
    throw new Error(`The batch lookup registry at ${registryPath} is not a JSON object`);
  }
  const entries: [string, BatchLookupEntry][] = [];
  for (const [key, value] of Object.entries(parsed)) {
    const entry = normalizeEntry(value);
    if (entry === null) {
      throw new Error(`The batch lookup registry at ${registryPath} has an unreadable entry for ${key}`);
    }
    entries.push([key, entry]);
  }
  return Object.fromEntries(entries);
};

const writeRegistry = async (registryPath: string, registry: BatchLookupRegistry): Promise<void> => {
  const fs = await import('node:fs/promises');
  const temporaryPath = `${registryPath}.tmp`;
  await fs.writeFile(temporaryPath, `${JSON.stringify(registry, null, 2)}\n`, { mode: 0o600 });
  await fs.rename(temporaryPath, registryPath);
};

/**
 * The retiring half of the table lifecycle (create -> extend -> deactivate -> close). Run on every
 * `prepareNextBatch`, over every table this direction has ever recorded:
 *
 *   - a table whose batch is settled or canceled and that is still active gets deactivated;
 *   - a table whose deactivation cooldown has elapsed gets closed, refunding the rent;
 *   - a table that is already gone is pruned from the record.
 *
 * `settleVaultBatch` also deactivates eagerly on the happy path, and both routes are safe together
 * because deactivating twice is an on-chain error and both check the table's state first. The
 * crank is what makes deactivation *recoverable*: settlement and deactivation are separate
 * transactions, so the process can exit in between, the second can fail on its own, and a canceled
 * batch never settles at all. Without this pass any of those three leaves a table active and
 * rented with nothing left to retry it — the leak class #1859 section 6d exists to close.
 *
 * A batch that is still pending or dispatched keeps its table: dispatch has not settled yet, and
 * settlement needs the table to build its v0 transaction.
 *
 * Every step is best-effort. A transaction that cannot land yet is retried by a later crank, which
 * is the whole point of deriving the work from on-chain state instead of remembering it.
 */
const retireFinishedLookupTables = async (
  config: DemoConfig,
  keeper: TransactionSigner,
  direction: VaultDirection,
  registryPath: string,
  activeLookupTable: Address,
): Promise<void> => {
  const rpc = createSolanaRpc(config.rpcUrl);
  const roots = vaultRoots(config, direction);
  const registry = await readRegistry(registryPath);
  const keyPrefix = `${config.chainId}:${config.batchers[direction].batcher}:`;
  // Every table this direction owns, each carrying the batch it belongs to so its status can be
  // read. A retired table shares its entry's batch index: it was derived for that batch, and that
  // batch is what decides whether the table can still be needed. The batch-0 table is
  // seed-provisioned into the demo-config rather than recorded here.
  const candidates = new Map<string, bigint | null>([[config.batchers[direction].lookupTable, 0n]]);
  for (const [key, entry] of Object.entries(registry)) {
    if (!key.startsWith(keyPrefix)) continue;
    const batchIndex = batchIndexFromKey(key);
    for (const table of [entry.table, ...(entry.retired ?? [])]) candidates.set(table, batchIndex);
  }
  candidates.delete(activeLookupTable);
  const currentSlot = await rpc.getSlot({ commitment: 'finalized' }).send();
  const closed = new Set<string>();
  // One read per batch, not per table: a batch with several retired tables is common after a
  // partial provisioning run.
  const finishedByIndex = new Map<string, boolean>();
  const batchIsFinished = async (batchIndex: bigint | null): Promise<boolean> => {
    if (batchIndex === null) return false;
    const memoKey = batchIndex.toString();
    const memoized = finishedByIndex.get(memoKey);
    if (memoized !== undefined) return memoized;
    let finished = false;
    try {
      const batch = await getBatchByIndex(rpc, roots, batchIndex, { commitment: 'confirmed' });
      finished = isBatchFinished(batch.state.status);
    } catch {
      // An unreadable batch account is not evidence that settlement is over, so the table stays.
      finished = false;
    }
    finishedByIndex.set(memoKey, finished);
    return finished;
  };
  for (const [table, batchIndex] of candidates) {
    try {
      const account = await rpc
        .getAccountInfo(table as Address, { commitment: 'confirmed', encoding: 'base64' })
        .send();
      if (account.value === null || account.value.owner !== LOOKUP_TABLE_PROGRAM) {
        closed.add(table);
        continue;
      }
      const encoded = account.value.data as readonly [string, 'base64'];
      const deactivationSlot = decodeLookupTableDeactivationSlot(Buffer.from(encoded[0], 'base64'));
      if (deactivationSlot === LOOKUP_TABLE_STILL_ACTIVE) {
        // Reading the state first is what makes this idempotent: the deactivate instruction fails
        // on an already-deactivated table, so an eager deactivation by `settleVaultBatch` and this
        // crank cannot collide.
        if (!(await batchIsFinished(batchIndex))) continue;
        await sendTransaction(
          config,
          keeper,
          [getDeactivateLookupTableInstruction({ lookupTable: table as Address, authority: keeper })],
          LOOKUP_TABLE_COMPUTE_UNIT_LIMIT,
        );
        // Closing has to wait out the cooldown; the next crank picks it up.
        continue;
      }
      if (currentSlot <= deactivationSlot + LOOKUP_TABLE_DEACTIVATION_COOLDOWN_SLOTS) continue;
      await sendTransaction(
        config,
        keeper,
        [getCloseLookupTableInstruction({ lookupTable: table as Address, authority: keeper, recipient: keeper.address })],
        LOOKUP_TABLE_COMPUTE_UNIT_LIMIT,
      );
      closed.add(table);
    } catch (error) {
      console.warn(
        `retiring lookup table ${table} failed (a later prepare retries): ${error instanceof Error ? error.message : String(error)}`,
      );
    }
  }
  if (closed.size > 0) {
    // Only closed tables leave the record. A deactivated-but-not-yet-closed one stays, because it
    // still owns rent and this file is the only thing that knows its address.
    const pruned: BatchLookupRegistry = {};
    for (const [key, entry] of Object.entries(registry)) {
      const retired = (entry.retired ?? []).filter((table) => !closed.has(table));
      if (key.startsWith(keyPrefix) && closed.has(entry.table)) {
        // The primary table is gone; keep the entry only while it still tracks live retired ones.
        if (retired.length > 0) pruned[key] = { table: retired[0]!, retired: retired.slice(1) };
        continue;
      }
      pruned[key] = retired.length === 0 ? { table: entry.table } : { table: entry.table, retired };
    }
    await writeRegistry(registryPath, pruned);
  }
};

export type PreparedBatch = {
  readonly batchIndex: bigint;
  readonly batch: Address;
  readonly lookupTable: Address;
};

export const prepareNextBatch = async (
  config: DemoConfig,
  keeper: TransactionSigner,
  direction: VaultDirection,
  registryPath: string,
): Promise<PreparedBatch> => {
  const rpc = createSolanaRpc(config.rpcUrl);
  const roots = vaultRoots(config, direction);
  const current = await getCurrentBatch(rpc, roots, { commitment: 'confirmed' });
  if (current.state.status === BatchStatus.Dispatched) {
    throw new Error(`The current ${direction} batch is still settling`);
  }

  const batchIndex = current.state.status === BatchStatus.Pending ? current.index : current.index + 1n;
  const recentSlot = await rpc.getSlot({ commitment: 'finalized' }).send();
  const prepared = await openBatchForBatcher({
    roots,
    batchIndex,
    payer: keeper,
    recentSlot,
    authorityFundingLamports: BigInt(config.authorityFundingLamports),
  });

  if (current.state.status !== BatchStatus.Pending) {
    await sendTransaction(config, keeper, [prepared.instructions[0]!], PROVISIONING_COMPUTE_UNIT_LIMIT);
  }

  const registry = await readRegistry(registryPath);
  const batch =
    current.state.status === BatchStatus.Pending
      ? current.addresses.batch
      : (await getCurrentBatch(rpc, roots)).addresses.batch;
  const key = registryKey(config, direction, batchIndex, batch);
  const recorded = registry[key];
  const candidateLookupTable = (
    batchIndex === 0n ? config.batchers[direction].lookupTable : recorded?.table ?? prepared.lookupTableAddress
  ) as Address;
  const candidatePrefix = await lookupTablePrefixLength(
    config,
    candidateLookupTable,
    prepared.lookupTableAddresses,
  );
  const lookupTable = candidatePrefix === null ? prepared.lookupTableAddress : candidateLookupTable;
  const prefixLength = candidatePrefix ?? 0;
  const remaining = prepared.lookupTableAddresses.slice(prefixLength);

  // Write-ahead, before any create or extend is sent. The table address is derived from a recent
  // slot, so once that slot has passed the address cannot be re-derived: if the create landed and
  // the process then died, this file is the only thing that could still name the table, and a
  // record written afterwards would never exist. Anything the previous record named that this run
  // is not going to use is retired rather than overwritten, so the crank can still reclaim it.
  const retired = [
    ...new Set(
      [...(recorded?.retired ?? []), ...(recorded === undefined ? [] : [recorded.table])].filter(
        (table) => table !== lookupTable,
      ),
    ),
  ];
  registry[key] = retired.length === 0 ? { table: lookupTable } : { table: lookupTable, retired };
  await writeRegistry(registryPath, registry);

  // The vault builder chunks the extend at the wire limit so no instruction is unsendable; the
  // table's create (when the table is fresh) rides with the first chunk.
  const extendInstructions =
    remaining.length === 0
      ? []
      : getExtendLookupTableInstructions({
          lookupTable,
          authority: keeper,
          payer: keeper,
          addresses: remaining,
        });
  for (const [index, extend] of extendInstructions.entries()) {
    const create = candidatePrefix === null && index === 0 ? prepared.instructions[1] : undefined;
    await sendTransaction(
      config,
      keeper,
      create === undefined ? [extend] : [create, extend],
      LOOKUP_TABLE_COMPUTE_UNIT_LIMIT,
    );
  }

  // Rent hygiene: retire this direction's finished tables while we are here.
  await retireFinishedLookupTables(config, keeper, direction, registryPath, lookupTable);
  return {
    batchIndex,
    batch,
    lookupTable,
  };
};

export const lookupTableForBatch = async (
  config: DemoConfig,
  direction: VaultDirection,
  position: { readonly batchIndex: bigint; readonly batch: Address },
  registryPath: string,
): Promise<Address> => {
  if (position.batchIndex === 0n) return config.batchers[direction].lookupTable;
  const registry = await readRegistry(registryPath);
  const entry = registry[registryKey(config, direction, position.batchIndex, position.batch)];
  if (entry === undefined) throw new Error(`No settlement lookup table is ready for ${position.batch}`);
  // Settlement gets `table`, never a retired one: those exist only so the crank can reclaim them.
  return entry.table as Address;
};
