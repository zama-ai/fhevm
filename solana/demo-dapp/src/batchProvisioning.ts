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
  getCurrentBatch,
  getExtendLookupTableInstructions,
  openBatchForBatcher,
} from './vault/index.js';

import type { VaultDirection } from './batchTypes';
import type { DemoConfig } from './demoConfig';
import { sendTransaction } from './sendTransaction';
import { vaultRoots } from './vaultRoots';

const BATCH_PENDING = 0;
const BATCH_DISPATCHED = 1;
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

type BatchLookupRegistry = Record<string, string>;

const registryKey = (
  config: DemoConfig,
  direction: VaultDirection,
  batchIndex: bigint,
  batch: Address,
): string => `${config.chainId}:${config.batchers[direction].batcher}:${batchIndex.toString()}:${batch}`;

const readRegistry = async (registryPath: string): Promise<BatchLookupRegistry> => {
  try {
    const parsed = JSON.parse(await (await import('node:fs/promises')).readFile(registryPath, 'utf8')) as unknown;
    if (typeof parsed !== 'object' || parsed === null || Array.isArray(parsed)) return {};
    return Object.fromEntries(
      Object.entries(parsed).filter((entry): entry is [string, string] => typeof entry[1] === 'string'),
    );
  } catch {
    return {};
  }
};

const writeRegistry = async (registryPath: string, registry: BatchLookupRegistry): Promise<void> => {
  const fs = await import('node:fs/promises');
  const temporaryPath = `${registryPath}.tmp`;
  await fs.writeFile(temporaryPath, `${JSON.stringify(registry, null, 2)}\n`, { mode: 0o600 });
  await fs.rename(temporaryPath, registryPath);
};

/**
 * The close half of the table lifecycle (create -> extend -> deactivate -> close):
 * `settleVaultBatch` deactivates a batch's table right after settlement, and this crank — run on
 * every `prepareNextBatch` — closes any table of this direction whose deactivation cooldown has
 * elapsed, refunding the rent to the keeper. Already-closed tables are pruned from the registry.
 * Every step is best-effort: a close that cannot land yet is simply retried by a later crank.
 */
const closeCooledLookupTables = async (
  config: DemoConfig,
  keeper: TransactionSigner,
  direction: VaultDirection,
  registryPath: string,
  activeLookupTable: Address,
): Promise<void> => {
  const rpc = createSolanaRpc(config.rpcUrl);
  const registry = await readRegistry(registryPath);
  const keyPrefix = `${config.chainId}:${config.batchers[direction].batcher}:`;
  const directionEntries = Object.entries(registry).filter(([key]) => key.startsWith(keyPrefix));
  const candidates = new Set<string>([
    // The seed-provisioned batch-0 table lives in the demo-config, not the registry.
    config.batchers[direction].lookupTable,
    ...directionEntries.map(([, table]) => table),
  ]);
  candidates.delete(activeLookupTable);
  const currentSlot = await rpc.getSlot({ commitment: 'finalized' }).send();
  const closed = new Set<string>();
  for (const table of candidates) {
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
      if (deactivationSlot === LOOKUP_TABLE_STILL_ACTIVE) continue;
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
        `closing cooled lookup table ${table} failed (a later prepare retries): ${error instanceof Error ? error.message : String(error)}`,
      );
    }
  }
  if (closed.size > 0) {
    const pruned = Object.fromEntries(
      Object.entries(registry).filter(([key, table]) => !(key.startsWith(keyPrefix) && closed.has(table))),
    );
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
  if (current.state.status === BATCH_DISPATCHED) {
    throw new Error(`The current ${direction} batch is still settling`);
  }

  const batchIndex = current.state.status === BATCH_PENDING ? current.index : current.index + 1n;
  const recentSlot = await rpc.getSlot({ commitment: 'finalized' }).send();
  const prepared = await openBatchForBatcher({
    roots,
    batchIndex,
    payer: keeper,
    recentSlot,
    authorityFundingLamports: BigInt(config.authorityFundingLamports),
  });

  if (current.state.status !== BATCH_PENDING) {
    await sendTransaction(config, keeper, [prepared.instructions[0]!], PROVISIONING_COMPUTE_UNIT_LIMIT);
  }

  const registry = await readRegistry(registryPath);
  const key = registryKey(config, direction, batchIndex, current.state.status === BATCH_PENDING
    ? current.addresses.batch
    : (await getCurrentBatch(rpc, roots)).addresses.batch);
  const recordedLookupTable = registry[key] as Address | undefined;
  const candidateLookupTable =
    batchIndex === 0n
      ? config.batchers[direction].lookupTable
      : recordedLookupTable ?? prepared.lookupTableAddress;
  const candidatePrefix = await lookupTablePrefixLength(
    config,
    candidateLookupTable,
    prepared.lookupTableAddresses,
  );
  const lookupTable = candidatePrefix === null ? prepared.lookupTableAddress : candidateLookupTable;
  const prefixLength = candidatePrefix ?? 0;
  const remaining = prepared.lookupTableAddresses.slice(prefixLength);
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

  const batch = current.state.status === BATCH_PENDING ? current.addresses.batch : (await getCurrentBatch(rpc, roots)).addresses.batch;
  registry[registryKey(config, direction, batchIndex, batch)] = lookupTable;
  await writeRegistry(registryPath, registry);
  // Rent hygiene: close this direction's cooled-down deactivated tables while we are here.
  await closeCooledLookupTables(config, keeper, direction, registryPath, lookupTable);
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
  const lookupTable = registry[registryKey(config, direction, position.batchIndex, position.batch)];
  if (lookupTable === undefined) throw new Error(`No settlement lookup table is ready for ${position.batch}`);
  return lookupTable as Address;
};
