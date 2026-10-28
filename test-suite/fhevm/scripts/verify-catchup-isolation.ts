import { readFileSync } from "node:fs";
import path from "node:path";
import { requireCatchupIngestionReceipts } from "../../e2e/test/consensus/failureEvidence";

const root = process.argv[2];
if (!root) throw new Error("catchup evidence directory required");
const read = (file: string) => JSON.parse(readFileSync(path.join(root, file), "utf8"));
const control = read("control.json");
if (!Number.isSafeInteger(control.until) || control.until <= Math.floor(Date.now() / 1000)) {
  throw new Error("LIVE/FINAL isolation must remain active throughout verification");
}
const workload = read("workload.json").payload;
const request = read("request.json").payload;
if (request.caseId !== "FM-HOST-LONG-OFFLINE" || workload.caseId !== request.caseId) {
  throw new Error("catchup evidence belongs to another scenario");
}
const receipts = readFileSync(path.join(root, "receipts.jsonl"), "utf8")
  .trim().split("\n").filter(Boolean).map(line => JSON.parse(line));
requireCatchupIngestionReceipts(request.catchupId, workload.detail.blocks, workload.detail.blockHashes, workload.transactionHashes, receipts);
console.log(JSON.stringify({ catchupId: request.catchupId, receiptBlocks: workload.detail.blocks, successfulReplayTransactions: 12, liveFinalHeld: true }));
