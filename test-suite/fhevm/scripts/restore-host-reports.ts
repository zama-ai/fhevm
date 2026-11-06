#!/usr/bin/env bun
import { hostReportOriginals } from "../src/consensus/host-report-journal";

const file = process.argv[2];
if (!file) throw new Error("host report recovery journal required");
const originals = hostReportOriginals(await Bun.file(file).json());
for (const row of originals) {
  // The journaled key is already scoped to the upgrade attempt that published
  // the divergent object (state_hash.rs:state_hash_key).
  const key = `${row.bucket}/${row.key}`;
  // The coprocessor buckets accept anonymous writes (see object-store-docker-compose.yml).
  const restored = await fetch(`http://127.0.0.1:9000/${key}`, {
    method: "PUT", headers: { "x-amz-meta-block-hash": row.blockHash }, body: Buffer.from(row.original, "hex"),
    signal: AbortSignal.timeout(60_000),
  });
  if (!restored.ok) throw new Error("host report restore failed; retain journal and discard stack until repaired");
  const response = await fetch(`http://127.0.0.1:9000/${key}`, { signal: AbortSignal.timeout(10_000) });
  if (!response.ok || response.headers.get("x-amz-meta-block-hash") !== row.blockHash ||
      Buffer.from(await response.arrayBuffer()).toString("hex") !== row.original) {
    throw new Error("restored host commitment or metadata differs");
  }
}
console.log(`Restored and read back ${originals.length} host commitments.`);
