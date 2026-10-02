// Merkle record check: once the scenarios have run, the Merkle indexer's record holds the same
// leaf count and peaks for every EncryptedStore as the chain. A record that drifted serves proofs
// the connector rejects against the on-chain peaks, so a green run also proves the record.

import { createHash } from "node:crypto";

import { type Base58EncodedBytes, createSolanaRpc, getAddressDecoder, getBase58Decoder } from "@solana/kit";

import { SOLANA_MERKLE_DATABASE } from "../layout";
import { run } from "../utils/process";
import { until } from "../utils/until";
import { sdkVerifyModule } from "./lazy-modules";

// The indexer follows the same confirmed stream the scenarios just wrote to; it is seconds behind.
const RECORD_CATCH_UP_TIMEOUT_MS = 60_000;

export type StoreCursor = { readonly leafCount: bigint; readonly peaks: readonly string[] };

/** Anchor's account discriminator: `sha256("account:EncryptedStore")[..8]`. */
const encryptedStoreDiscriminator = (): Uint8Array =>
  createHash("sha256").update("account:EncryptedStore").digest().subarray(0, 8);

const hex = (bytes: Uint8Array): string => Buffer.from(bytes).toString("hex");

/**
 * Compares both directions. A store on chain without leaves may be absent from the record, which
 * adds a store with its first leaf. Only the preview wipe closes EncryptedStores, and the preview
 * reset recreates the record after it, so a store only the record holds is drift too.
 */
export const recordMismatches = (
  chain: ReadonlyMap<string, StoreCursor>,
  record: ReadonlyMap<string, StoreCursor>,
): string[] => {
  const onChain = [...chain].flatMap(([store, cursor]) => {
    const recorded = record.get(store);
    if (recorded === undefined) {
      return cursor.leafCount === 0n ? [] : [`${store}: ${cursor.leafCount} leaves on chain, absent from the record`];
    }
    if (recorded.leafCount !== cursor.leafCount) {
      return [`${store}: ${cursor.leafCount} leaves on chain, ${recorded.leafCount} in the record`];
    }
    return recorded.peaks.join() === cursor.peaks.join() ? [] : [`${store}: the peaks differ at ${cursor.leafCount} leaves`];
  });
  const recordOnly = [...record]
    .filter(([store]) => !chain.has(store))
    .map(([store, recorded]) => `${store}: ${recorded.leafCount} leaves in the record, no account on chain`);
  return [...onChain, ...recordOnly];
};

type RecordRow = { readonly store: string; readonly leafCount: string; readonly peaks: readonly string[] };

const RECORD_QUERY = `SELECT coalesce(json_agg(json_build_object(
  'store', encode(encrypted_store, 'hex'),
  'leafCount', leaf_count::text,
  'peaks', (SELECT coalesce(json_agg(encode(peak, 'hex')), '[]') FROM unnest(peaks) AS peak))), '[]')
FROM encrypted_stores`;

/**
 * Lists the zama-host program's EncryptedStores, waits for the indexer's checkpoint to reach the
 * slot of that listing, and compares the two, retrying until they agree or the deadline passes.
 * `psql` is the command prefix that opens the coprocessor database; the Merkle database is on the
 * same server, and psql takes the last `-d`.
 */
export const assertMerkleRecordMatchesChain = async (input: {
  readonly rpcUrl: string;
  readonly aclProgram: `0x${string}`;
  readonly coprocessorDbPsql: readonly string[];
}): Promise<void> => {
  const { decodeSolanaEncryptedStore } = await sdkVerifyModule();
  const rpc = createSolanaRpc(input.rpcUrl);
  const addressDecoder = getAddressDecoder();
  const program = addressDecoder.decode(Buffer.from(input.aclProgram.slice(2), "hex"));
  const psql = [...input.coprocessorDbPsql, "-d", SOLANA_MERKLE_DATABASE, "-tAc"];
  const discriminator = getBase58Decoder().decode(encryptedStoreDiscriminator()) as Base58EncodedBytes;

  const compared = await until(
    async () => {
      const { context, value: accounts } = await rpc
        .getProgramAccounts(program, {
          commitment: "confirmed",
          encoding: "base64",
          withContext: true,
          filters: [{ memcmp: { offset: 0n, bytes: discriminator, encoding: "base58" } }],
        })
        .send();
      const checkpoint = (await run([...psql, "SELECT slot FROM checkpoint"])).stdout.trim();
      if (checkpoint === "" || BigInt(checkpoint) < context.slot) {
        throw new Error(`record checkpoint ${checkpoint || "absent"} is behind slot ${context.slot}; see the Merkle indexer logs`);
      }
      const chain = new Map(
        accounts.map(({ pubkey, account }): [string, StoreCursor] => {
          const store = decodeSolanaEncryptedStore(Buffer.from(account.data[0], "base64"), pubkey);
          return [pubkey, { leafCount: store.leafCount, peaks: store.peaks.map(hex) }];
        }),
      );
      const rows = JSON.parse((await run([...psql, RECORD_QUERY])).stdout) as RecordRow[];
      const record = new Map(
        rows.map((row): [string, StoreCursor] => [
          addressDecoder.decode(Buffer.from(row.store, "hex")),
          { leafCount: BigInt(row.leafCount), peaks: row.peaks },
        ]),
      );
      const mismatches = recordMismatches(chain, record);
      if (mismatches.length > 0) {
        throw new Error(`the Merkle record disagrees with the chain at slot ${context.slot}:\n${mismatches.join("\n")}`);
      }
      const withLeaves = [...chain.values()].filter((cursor) => cursor.leafCount > 0n).length;
      if (withLeaves === 0) throw new Error(`no EncryptedStore holds a leaf at slot ${context.slot}; nothing was compared`);
      return { slot: context.slot, withLeaves };
    },
    { timeoutMs: RECORD_CATCH_UP_TIMEOUT_MS, intervalMs: 1_000, description: "Merkle record matching the chain" },
  );
  console.log(`Merkle record matches the chain for ${compared.withLeaves} EncryptedStore(s) with leaves at slot ${compared.slot}`);
};
