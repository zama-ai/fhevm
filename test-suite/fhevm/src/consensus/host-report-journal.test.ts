import { expect, test } from "bun:test";
import { hostReportOriginals } from "./host-report-journal";

test("host report recovery cannot target arbitrary buckets, paths or malformed bytes", () => {
  const key = "state_hash/proposal=" + "00".repeat(31) + "01/attempt=10/chain=12345/block=150.bin";
  const row = { chain: "12345", block: "150", original: "42".repeat(32), blockHash: "0x" + "ab".repeat(32), bucket: "coproc-1", key };
  expect(hostReportOriginals({ "150": row })).toEqual([row]);
  expect(hostReportOriginals({})).toEqual([]); // Aborted before any upload.
  for (const change of [{ bucket: "coproc-0" }, { chain: "../other" }, { block: "151" },
    { original: "" }, { blockHash: "0x12" },
    // The key must be attempt-scoped and stay inside this row's (chain, block).
    { key: "state_hash/chain=12345/block=150.bin" },
    { key: key.replace("/chain=12345/", "/chain=67890/") },
    { key: key.replace("/block=150.bin", "/block=151.bin") },
    { key: "../" + key }, { key: undefined as unknown as string }]) {
    expect(() => hostReportOriginals({ "150": { ...row, ...change } })).toThrow("unscoped or damaged");
  }
  expect(() => hostReportOriginals([])).toThrow("invalid");
});
