import { expect, test } from "bun:test";
import path from "node:path";

const library = path.resolve(import.meta.dir, "../../scripts/lib/retired-writer-evidence.sh");
const observed = (role: string, error: string) => {
  const result = Bun.spawnSync(["bash", "-c", 'source "$1"; rw_rejection_observed "$2" "$3"', "probe", library, role, JSON.stringify({ fields: { error } })]);
  return result.exitCode === 0;
};

test("accepts the real release TFHE and SNS retirement errors", () => {
  expect(observed("tfhe-worker", 'Coprocessor db error: Configuration(StaleStackError { binary: "0.14.0", live: "0.15.0" })')).toBe(true);
  expect(observed("sns-worker", "DB: error with configuration: stack version 0.14.0 is older than live stack 0.15.0; access denied (retired stack)")).toBe(true);
});

test("does not mistake generic retries or another stack version for retirement", () => {
  for (const error of ["Error in background worker, retrying shortly", "database connection refused", 'StaleStackError { binary: "0.15.0", live: "0.14.0" }', 'StaleStackError { binary: "0.14.0", live: "0.15.01" }']) {
    expect(observed("tfhe-worker", error)).toBe(false);
  }
  expect(observed("tfhe-worker", "pausing into no-op mode")).toBe(false);
  expect(observed("sns-worker", 'StaleStackError { binary: "0.14.0", live: "0.15.0" }')).toBe(false);
});
