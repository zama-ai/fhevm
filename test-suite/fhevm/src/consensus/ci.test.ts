import { expect, test } from "bun:test";
import { failedSelectedJobs } from "./ci";

test("a failed final validity gate rejects otherwise passing case coverage", () => {
  expect(failedSelectedJobs(["byte-agreement"], {
    plan: {result: "success"}, consensus: {result: "failure"},
  })).toEqual(["consensus: failure"]);
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
