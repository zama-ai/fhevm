import { mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { expect, test } from "bun:test";
import { missingIngestedReceiptBlocks, offlineCatchupRange, requireCompletedConsumerCatchup, requireRelativeConsumerCatchup, requireCatchupIngestionReceipts } from "../../../e2e/test/consensus/failureEvidence";

test("offline recovery uses the saved height and current RPC head like the legacy poller", () => {
  const blocks = Array.from({ length: 12 }, (_, i) => 2764 + i * 2);
  expect(offlineCatchupRange(2733, 2788, blocks)).toEqual({ start: 2734, end: 2788 });
  expect(offlineCatchupRange(null, 2788, blocks)).toEqual({ start: 1, end: 2788 });
  // The queued pre-outage LIVE reference from CI cannot bound this workload.
  expect(() => offlineCatchupRange(2733, 2732, blocks)).toThrow("all twelve");
  expect(() => offlineCatchupRange(2733, 2785, blocks)).toThrow("all twelve");
  expect(() => offlineCatchupRange(2764, 2788, blocks)).toThrow("all twelve");
});

test("offline replay requires a new completed non-finalized request overlapping workload receipts", () => {
  const blocks = Array.from({ length: 12 }, (_, i) => 100 + i);
  const row = { id: "new-request", status: "DONE", finalized: false, blockStart: 0, blockEnd: 120 };
  expect(requireCompletedConsumerCatchup(row, "old-request", blocks)).toEqual(blocks);
  expect(requireCompletedConsumerCatchup({ ...row, blockEnd: 103 }, "old-request", blocks)).toEqual([100, 101, 102, 103]);
  expect(requireCompletedConsumerCatchup({ ...row, blockStart: 111 }, "old-request", blocks)).toEqual([111]);
  for (const invalid of [
    { ...row, id: "old-request" },
    { ...row, id: undefined },
    { ...row, status: "ONGOING" },
    { ...row, finalized: true },
    { ...row, blockStart: 112 },
    { ...row, blockEnd: 99 },
    { ...row, blockEnd: undefined },
  ]) {
    expect(() => requireCompletedConsumerCatchup(invalid, "old-request", blocks)).toThrow();
  }
  expect(() => requireCompletedConsumerCatchup(row, undefined, blocks)).toThrow();
  expect(() => requireCompletedConsumerCatchup(row, "old-request", blocks.slice(1))).toThrow();
});

test("database coverage requires every exact receipt block, including those outside the replay", () => {
  const blocks = Array.from({ length: 12 }, (_, i) => 100 + i);
  const hashes = blocks.map(block => `0x${block.toString(16).padStart(64, "0")}`);
  const ingested = blocks.map((blockNumber, index) => ({
    blockNumber, blockHash: hashes[index], blockStatus: index % 2 ? "pending" : "finalized",
  }));
  expect(missingIngestedReceiptBlocks(blocks, hashes, [...ingested].reverse())).toEqual([]);
  // A later ingested block cannot hide a missing block in the middle.
  expect(missingIngestedReceiptBlocks(blocks, hashes, ingested.filter(block => block.blockNumber !== 105))).toEqual([105]);
  expect(missingIngestedReceiptBlocks(blocks, hashes, [ingested[0], ...ingested.filter(block => block.blockNumber !== 105)])).toEqual([105]);
  const partial = requireCompletedConsumerCatchup({
    id: "new-request", status: "DONE", finalized: false, blockStart: 0, blockEnd: 103,
  }, "old-request", blocks);
  expect(missingIngestedReceiptBlocks(blocks, hashes, ingested.filter(block => partial.includes(block.blockNumber))))
    .toEqual(blocks.slice(4));
  expect(missingIngestedReceiptBlocks(blocks, hashes, ingested.map(block => block.blockNumber === 105
    ? { ...block, blockHash: hashes[0] } : block))).toEqual([105]);
  for (const blockStatus of ["orphaned", "unknown"]) {
    expect(missingIngestedReceiptBlocks(blocks, hashes, ingested.map(block => block.blockNumber === 105
      ? { ...block, blockStatus } : block))).toEqual([105]);
  }
  for (const invalid of [undefined, hashes.slice(1), hashes.map(() => "invalid")]) {
    expect(() => missingIngestedReceiptBlocks(blocks, invalid, ingested)).toThrow("exact hash");
  }
});

test("relative-only recovery rejects a completed replay bounded by an old LIVE delivery", () => {
  const blocks = Array.from({ length: 12 }, (_, i) => 2764 + i * 2);
  const row = { id: "relative-request", finalized: false, requestedFrom: -1024, requestedTo: -1, blockStart: 1709, blockEnd: 2732 };
  expect(() => requireRelativeConsumerCatchup(row, "previous-request", blocks)).toThrow("first queued LIVE reference may be stale");
  expect(() => requireRelativeConsumerCatchup({ ...row, blockStart: 1764, blockEnd: 2788 }, "previous-request", blocks)).not.toThrow();
  // A separately seeded absolute request cannot substitute for the CLI request.
  expect(() => requireRelativeConsumerCatchup({ ...row, requestedFrom: 2734, requestedTo: 2788, blockStart: 2734, blockEnd: 2788 }, "previous-request", blocks)).toThrow("CLI-created");
  expect(() => requireRelativeConsumerCatchup({ ...row, id: "previous-request" }, "previous-request", blocks)).toThrow("CLI-created");
});

test("catchup attribution requires the exact request, block hash and original transaction after successful ingestion", () => {
  const blocks = Array.from({ length: 12 }, (_, i) => 100 + i * 2);
  const hashes = blocks.map(block => `0x${block.toString(16).padStart(64, "0")}`);
  const transactions = blocks.map(block => `0x${(block + 1000).toString(16).padStart(64, "0")}`);
  const receipts = blocks.map((block_number, i) => ({ flow: "CATCHUP", catchup_id: "requested", block_number, block_hash: hashes[i], transactions: [{ hash: transactions[i] }] }));
  const verify = (observed: typeof receipts) => requireCatchupIngestionReceipts("requested", blocks, hashes, transactions, observed);
  expect(() => verify([...receipts, receipts[0]].reverse())).not.toThrow();
  for (const changed of [
    receipts.slice(1),
    receipts.map(receipt => ({ ...receipt, flow: "LIVE" })),
    receipts.map(receipt => ({ ...receipt, flow: "FINAL" })),
    receipts.map(receipt => ({ ...receipt, catchup_id: "different-request" })),
    receipts.map(receipt => ({ ...receipt, block_hash: hashes[0] })),
    receipts.map(receipt => ({ ...receipt, transactions: [] })),
    receipts.map(receipt => ({ ...receipt, transactions: [{ hash: transactions[0] }] })),
  ]) expect(() => verify(changed)).toThrow("no successful ingestion receipt");
});

test("the evidence command rejects expired isolation and stale scenario artifacts", () => {
  const root = mkdtempSync(path.join(tmpdir(), "catchup-evidence-"));
  try {
    const blocks = Array.from({ length: 12 }, (_, i) => 100 + i * 2);
    const hashes = blocks.map(block => `0x${block.toString(16).padStart(64, "0")}`);
    const transactions = blocks.map(block => `0x${(block + 1000).toString(16).padStart(64, "0")}`);
    const write = (file: string, value: unknown) => writeFileSync(path.join(root, file), JSON.stringify(value));
    write("control.json", { until: Math.floor(Date.now() / 1000) + 60 });
    write("workload.json", { payload: { caseId: "FM-HOST-RELATIVE-CATCHUP", detail: { blocks, blockHashes: hashes }, transactionHashes: transactions } });
    write("request.json", { payload: { caseId: "FM-HOST-RELATIVE-CATCHUP", catchupId: "new" } });
    writeFileSync(path.join(root, "receipts.jsonl"), blocks.map((block_number, i) => JSON.stringify({
      flow: "CATCHUP", catchup_id: "new", block_number, block_hash: hashes[i], transactions: [{ hash: transactions[i] }],
    })).join("\n"));
    const run = () => Bun.spawnSync([process.execPath, path.resolve(import.meta.dir, "../../scripts/verify-catchup-isolation.ts"), root]);
    const valid = run();
    expect(valid.exitCode, valid.stderr.toString()).toBe(0);
    expect(JSON.parse(valid.stdout.toString()).successfulReplayTransactions).toBe(12);
    write("control.json", { until: Math.floor(Date.now() / 1000) - 1 });
    const expired = run();
    expect(expired.exitCode).not.toBe(0);
    expect(expired.stderr.toString()).toContain("remain active");
    write("control.json", { until: Math.floor(Date.now() / 1000) + 60 });
    write("request.json", { payload: { caseId: "another-case", catchupId: "new" } });
    const stale = run();
    expect(stale.exitCode).not.toBe(0);
    expect(stale.stderr.toString()).toContain("another scenario");
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("supervisor cleanup retains gate ownership after a failed clear and verifies a successful retry", () => {
  const root = mkdtempSync(path.join(tmpdir(), "catchup-gate-cleanup-"));
  try {
    writeFileSync(path.join(root, "catchup-isolation-owner"), "coprocessor1-host-listener-consumer\n");
    const helper = path.resolve(import.meta.dir, "../../scripts/lib/catchup-isolation.sh");
    const script = `source "$1"
      SP_RUNTIME_DIR="$2"
      sc_start() { :; }
      docker() { [[ "$*" != *" $FAIL_ACTION" ]]; }
      restore_catchup_isolation`;
    for (const [fail, exit] of [["clear", 1], ["empty", 1], ["none", 0]] as const) {
      const result = Bun.spawnSync(["bash", "-c", script, "--", helper, root], { env: { ...process.env, FAIL_ACTION: fail } });
      expect(result.exitCode, result.stderr.toString()).toBe(exit);
      expect(Bun.spawnSync(["test", "-f", path.join(root, "catchup-isolation-owner")]).exitCode).toBe(fail === "none" ? 1 : 0);
    }
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
