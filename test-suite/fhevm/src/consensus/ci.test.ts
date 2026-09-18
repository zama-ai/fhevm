import { expect, test } from "bun:test";
import { failedSelectedJobs } from "./ci";

test("a failed final validity gate rejects otherwise passing case coverage", () => {
  expect(failedSelectedJobs(["byte-agreement"], {
    plan: {result: "success"}, harness: {result: "success"}, consensus: {result: "failure"},
  })).toEqual(["consensus: failure"]);
});

test("stack-only selections also require their prerequisite harness", () => {
  for (const leg of ["byte-agreement", "fork", "gpu"]) {
    for (const result of ["failure", "skipped", "cancelled"]) {
      expect(failedSelectedJobs([leg], {
        plan: {result: "success"}, harness: {result},
        consensus: {result: "success"}, gpu: {result: "success"},
      })).toEqual([`harness: ${result}`]);
    }
  }
});

test("unselected skipped jobs do not invalidate a partial selection", () => {
  expect(failedSelectedJobs(["harness"], {
    plan: {result: "success"}, harness: {result: "success"},
    consensus: {result: "skipped"}, gpu: {result: "skipped"},
  })).toEqual([]);
});

test("selected missing, skipped and cancelled jobs cannot pass", () => {
  expect(failedSelectedJobs(["harness", "rust-regression", "gpu", "fork"], {
    plan: {result: "success"}, harness: {result: "cancelled"},
    gpu: {result: "skipped"}, consensus: {result: "success"},
  })).toEqual(["harness: cancelled", "rust-regression: missing", "gpu: skipped"]);
});
