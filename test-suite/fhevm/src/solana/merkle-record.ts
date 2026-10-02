// Merkle record check: once the scenarios have run, the Merkle indexer's cursor of every
// EncryptedStore holding leaves equals the account on chain. A record that drifted serves proofs
// the connector rejects against the on-chain peaks, so a green run also proves the record.

import { createHash } from "node:crypto";

import { type Base58EncodedBytes, createSolanaRpc, getAddressDecoder, getBase58Decoder } from "@solana/kit";

import { run } from "../utils/process";
import { until } from "../utils/until";

/** The Merkle proof service's database, on the coprocessor's Postgres server. */
const MERKLE_DATABASE = "solana_merkle";
// The indexer follows the same confirmed stream the scenarios just wrote to; it is seconds behind.
const RECORD_CATCH_UP_TIMEOUT_MS = 60_000;

export type StoreCursor = { readonly leafCount: bigint; readonly peaks: readonly string[] };

/** Anchor's account discriminator: `sha256("account:EncryptedStore")[..8]`. */
const encryptedStoreDiscriminator = (): Uint8Array =>
  createHash("sha256").update("account:EncryptedStore").digest().subarray(0, 8);

const hex = (bytes: Uint8Array): string => Buffer.from(bytes).toString("hex");

/**
 * Every store on chain with at least one leaf must appear in the record with the same leaf count
 * and peaks. Stores only in the record are ones the run closed, which the record keeps.
 */
export const recordMismatches = (
  chain: ReadonlyMap<string, StoreCursor>,
  record: ReadonlyMap<string, StoreCursor>,
): string[] =>
  [...chain]
    .filter(([, cursor]) => cursor.leafCount > 0n)
    .flatMap(([store, cursor]) => {
      const recorded = record.get(store);
      if (recorded === undefined) return [`${store}: ${cursor.leafCount} leaves on chain, absent from the record`];
      if (recorded.leafCount !== cursor.leafCount || recorded.peaks.join() !== cursor.peaks.join()) {
        return [`${store}: chain has ${cursor.leafCount} leaves, the record ${recorded.leafCount}, or their peaks differ`];
      }
      return [];
    });

/** Parses `encrypted_store leaf_count peak,peak,...` rows, each column hex but the count. */
export const parseRecordRows = (stdout: string): Map<string, StoreCursor> => {
  const addressDecoder = getAddressDecoder();
  return new Map(
    stdout
      .split("\n")
      .filter((line) => line.trim() !== "")
      .map((line): [string, StoreCursor] => {
        const [store, leafCount, peaks = ""] = line.trim().split(" ");
        return [
          addressDecoder.decode(Buffer.from(store!, "hex")),
          { leafCount: BigInt(leafCount!), peaks: peaks === "" ? [] : peaks.split(",") },
        ];
      }),
  );
};

/**
 * Lists the zama-host program's EncryptedStores at a confirmed slot, waits for the indexer's
 * checkpoint to reach that slot, and compares the two. `psql` is the command prefix that opens
 * the coprocessor database; the Merkle database is on the same server.
 */
export const assertMerkleRecordMatchesChain = async (input: {
  readonly rpcUrl: string;
  readonly aclProgram: `0x${string}`;
  readonly coprocessorDbPsql: readonly string[];
}): Promise<void> => {
  // The SDK loads only from its build, which the pure helpers above do not need.
  const { decodeSolanaEncryptedStore } = await import("@fhevm/sdk/solana");
  const rpc = createSolanaRpc(input.rpcUrl);
  const program = getAddressDecoder().decode(Buffer.from(input.aclProgram.slice(2), "hex"));
  const slot = await rpc.getSlot({ commitment: "confirmed" }).send();
  const accounts = await rpc
    .getProgramAccounts(program, {
      commitment: "confirmed",
      encoding: "base64",
      minContextSlot: slot,
      filters: [
        {
          memcmp: {
            offset: 0n,
            bytes: getBase58Decoder().decode(encryptedStoreDiscriminator()) as Base58EncodedBytes,
            encoding: "base58",
          },
        },
      ],
    })
    .send();
  const chain = new Map(
    accounts.map(({ pubkey, account }): [string, StoreCursor] => {
      const store = decodeSolanaEncryptedStore(Buffer.from(account.data[0], "base64"), pubkey);
      return [pubkey, { leafCount: store.leafCount, peaks: store.peaks.map(hex) }];
    }),
  );

  const psql = [...input.coprocessorDbPsql, "-d", MERKLE_DATABASE, "-tAc"];
  await until(
    async () => {
      const result = await run([...psql, "SELECT slot FROM checkpoint"], { allowFailure: true });
      return result.code === 0 && result.stdout.trim() !== "" && BigInt(result.stdout.trim()) >= slot;
    },
    { timeoutMs: RECORD_CATCH_UP_TIMEOUT_MS, intervalMs: 1_000, description: `Merkle record at slot ${slot}` },
  );
  const rows = await run([
    ...psql,
    "SELECT encode(encrypted_store, 'hex') || ' ' || leaf_count || ' ' || " +
      "array_to_string(ARRAY(SELECT encode(peak, 'hex') FROM unnest(peaks) AS peak), ',') FROM encrypted_stores",
  ]);
  const mismatches = recordMismatches(chain, parseRecordRows(rows.stdout));
  if (mismatches.length > 0) {
    throw new Error(`the Merkle record disagrees with ${mismatches.length} EncryptedStore(s):\n${mismatches.join("\n")}`);
  }
  console.log(`Merkle record matches the chain for ${chain.size} EncryptedStore(s) at slot ${slot}`);
};
