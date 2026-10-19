import { expect, test } from "bun:test";
import { missingIngestedReceiptBlocks, requireCompletedConsumerCatchup } from "../../../e2e/test/consensus/failureEvidence";

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
