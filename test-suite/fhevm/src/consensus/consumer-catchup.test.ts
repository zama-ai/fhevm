import { expect, test } from "bun:test";
import { requireCompletedConsumerCatchup } from "../../../e2e/test/consensus/failureEvidence";

test("offline replay requires a new completed non-finalized request covering every receipt", () => {
  const blocks = Array.from({ length: 12 }, (_, i) => 100 + i);
  const row = { id: "new-request", status: "DONE", finalized: false, blockStart: 0, blockEnd: 120 };
  expect(() => requireCompletedConsumerCatchup(row, "old-request", blocks)).not.toThrow();
  for (const invalid of [
    { ...row, id: "old-request" },
    { ...row, id: undefined },
    { ...row, status: "ONGOING" },
    { ...row, finalized: true },
    { ...row, blockStart: 101 },
    { ...row, blockEnd: 110 },
    { ...row, blockEnd: undefined },
  ]) {
    expect(() => requireCompletedConsumerCatchup(invalid, "old-request", blocks)).toThrow();
  }
  expect(() => requireCompletedConsumerCatchup(row, undefined, blocks)).toThrow();
  expect(() => requireCompletedConsumerCatchup(row, "old-request", blocks.slice(1))).toThrow();
});
