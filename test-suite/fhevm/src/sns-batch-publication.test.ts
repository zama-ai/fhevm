import { expect, test } from "bun:test";

import { hasCompetingBatches, snsBatchEvidence } from "./commands/sns-batch-publication";

function event(ms: number, message: string, fields: Record<string, unknown> = {}, target = "sns") {
  return JSON.stringify({ timestamp: new Date(ms).toISOString(), fields: { message, ...fields }, span: { target } });
}

test("SNS evidence attributes victim retries to the correct service and counts full batches", () => {
  const deadlock = { error: "error returned from database: deadlock detected" };
  const lines = [
    event(1, "S3 migration is disabled"),
    event(2, "Fetched SnS tasks", { count: 0 }),
    event(3, "Fetched SnS tasks", { count: 20 }),
    event(4, "Service error; retrying...", deadlock, "upload"),
    event(5, "Database deadlock; retrying immediately", deadlock),
    event(6, "Fetched SnS tasks", { count: 20 }),
    event(7, "Service error; retrying...", deadlock),
    event(8, "Fetched SnS tasks", { count: 5 }),
    event(9, "more tasks to process, continuing"),
    event(10, "Fetched SnS tasks", { count: 20 }),
    event(11, "more tasks to process, continuing"),
  ];
  expect(snsBatchEvidence(lines.reverse().join("\n"))).toEqual({
    migrationDisabled: true,
    idle: true,
    delayed: 1,
    immediate: 1,
    intervals: [
      { start: 3, end: 5 },
      { start: 6, end: 7 },
      { start: 10, end: 11 },
    ],
  });
});

test("full batch intervals must overlap on distinct SNS workers", () => {
  const worker = {
    intervals: [
      { start: 1, end: 4 },
      { start: 6, end: 10 },
    ],
  };
  expect(hasCompetingBatches([worker])).toBe(false);
  expect(hasCompetingBatches([worker, { intervals: [{ start: 4, end: 6 }] }])).toBe(false);
  expect(hasCompetingBatches([worker, { intervals: [{ start: 8, end: 12 }] }])).toBe(true);
});
