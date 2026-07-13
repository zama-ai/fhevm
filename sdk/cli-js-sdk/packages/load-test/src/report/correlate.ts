import type { StageRow } from "../collectors/stage-rows";
import type { RequestRecord } from "../flows/types";
import type { FlowKind } from "../relayer/types";
import { statsOf } from "./histogram";
import type { CorrelationReport } from "./schema";

/**
 * Reconciles client-observed latency with server-measured latency.
 *
 * The client only learns a request completed on its next poll, so client e2e
 * is quantized up to one poll interval above the truth. The relayer's
 * `created_at → completed_at` is poll-free. Joining the two by job id
 * quantifies exactly how much overhead polling added — turning the report's
 * "e2e is quantized" caveat into a measured number.
 */
export const buildCorrelation = (
  records: readonly RequestRecord[],
  stageRows: readonly StageRow[],
): CorrelationReport[] => {
  const serverE2eByJob = new Map<string, number>();
  for (const row of stageRows) {
    if (!row.createdAt || !row.completedAt) continue;
    const ms = Date.parse(row.completedAt) - Date.parse(row.createdAt);
    if (Number.isFinite(ms) && ms >= 0) serverE2eByJob.set(row.externalJobId, ms);
  }
  if (serverE2eByJob.size === 0) return [];

  const byFlow = new Map<
    FlowKind,
    { client: number[]; server: number[]; overhead: number[] }
  >();
  for (const record of records) {
    if (record.outcome !== "succeeded" || record.jobId === undefined) continue;
    if (record.e2eLatencyMs === undefined) continue;
    const serverMs = serverE2eByJob.get(record.jobId);
    if (serverMs === undefined) continue;
    const bucket =
      byFlow.get(record.flow) ?? { client: [], server: [], overhead: [] };
    bucket.client.push(record.e2eLatencyMs);
    bucket.server.push(serverMs);
    // Clamp at 0: clocks can disagree slightly; negative overhead is noise.
    bucket.overhead.push(Math.max(0, record.e2eLatencyMs - serverMs));
    byFlow.set(record.flow, bucket);
  }

  return [...byFlow.entries()].map(([flow, b]) => ({
    flow,
    matched: b.client.length,
    clientE2e: statsOf(b.client),
    serverE2e: statsOf(b.server),
    pollOverhead: statsOf(b.overhead),
  }));
};
