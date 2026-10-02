import { expect, test } from "bun:test";
import { hostReportOriginals } from "./host-report-journal";

test("host report recovery cannot target arbitrary buckets, paths or malformed bytes", () => {
  const row = { chain: "12345", block: "150", original: "42".repeat(32), blockHash: "0x" + "ab".repeat(32), bucket: "coproc-1" };
  expect(hostReportOriginals({ "150": row })).toEqual([row]);
  expect(hostReportOriginals({})).toEqual([]); // Aborted before any upload.
  for (const change of [{ bucket: "coproc-0" }, { chain: "../other" }, { block: "151" },
    { original: "" }, { blockHash: "0x12" }]) {
    expect(() => hostReportOriginals({ "150": { ...row, ...change } })).toThrow("unscoped or damaged");
  }
  expect(() => hostReportOriginals([])).toThrow("invalid");
});
