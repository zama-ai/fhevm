import type { RelayerLegRecord, RequestRecord } from "../flows/types";
import type { FlowKind } from "../relayer/types";
import { JsonlWriter } from "../shared/jsonl";
import { safeArtifactText } from "../shared/safe-artifact";

export type TargetRequestRecord = Pick<
  RequestRecord,
  "flow" | "index" | "loadStage" | "startedAtMs" | "sentRequestId"
> &
  Readonly<{ relayerTarget: "A" | "B" }> &
  RelayerLegRecord;

export type RecorderOptions = Readonly<{
  relayerAPath?: string;
  relayerBPath?: string;
}>;

/**
 * Collects per-request records: awaited to requests.jsonl as they complete,
 * fsynced on orderly close, and kept in memory for report building. A hard
 * process/power failure may lose the operating system's most recent writes. Optional target files
 * contain normalized per-relayer legs for easier ad hoc analysis.
 */
export class Recorder {
  private writer: JsonlWriter<RequestRecord> | undefined;
  private relayerAWriter: JsonlWriter<TargetRequestRecord> | undefined;
  private relayerBWriter: JsonlWriter<TargetRequestRecord> | undefined;
  private readonly all: RequestRecord[] = [];

  static async open(path: string, options: RecorderOptions = {}): Promise<Recorder> {
    const recorder = new Recorder();
    try {
      recorder.writer = await JsonlWriter.open<RequestRecord>(path);
      recorder.relayerAWriter = options.relayerAPath
        ? await JsonlWriter.open<TargetRequestRecord>(options.relayerAPath)
        : undefined;
      recorder.relayerBWriter = options.relayerBPath
        ? await JsonlWriter.open<TargetRequestRecord>(options.relayerBPath)
        : undefined;
      return recorder;
    } catch (error) {
      try {
        await recorder.close();
      } catch (cleanupError) {
        throw new AggregateError(
          [error, cleanupError],
          "Recorder initialization and cleanup both failed",
          { cause: error },
        );
      }
      throw error;
    }
  }

  async record(record: RequestRecord): Promise<void> {
    // Request records are durable operator artifacts. Treat every executor
    // error as untrusted, including SDK and transport errors that may embed
    // credentials, signatures, or request bodies. Labels remain stable for
    // aggregation; only free-form messages are redacted and bounded.
    const sanitized: RequestRecord = {
      ...record,
      errorMessage: safeArtifactText(record.errorMessage),
      errorMessageB: safeArtifactText(record.errorMessageB),
    };
    this.all.push(sanitized);
    const writes: Promise<void>[] = [];
    if (this.writer) writes.push(this.writer.write(sanitized));
    const aRecord = targetRecord(sanitized, "A");
    if (aRecord && this.relayerAWriter) writes.push(this.relayerAWriter.write(aRecord));
    const bRecord = targetRecord(sanitized, "B");
    if (bRecord && this.relayerBWriter) writes.push(this.relayerBWriter.write(bRecord));
    const settled = await Promise.allSettled(writes);
    const errors = settled.flatMap((result) =>
      result.status === "rejected" ? [result.reason] : [],
    );
    if (errors.length > 0) throw new AggregateError(errors, "Failed to persist request record");
  }

  get records(): readonly RequestRecord[] {
    return this.all;
  }

  byFlow(): ReadonlyMap<FlowKind, RequestRecord[]> {
    const map = new Map<FlowKind, RequestRecord[]>();
    for (const record of this.all) {
      const list = map.get(record.flow) ?? [];
      list.push(record);
      map.set(record.flow, list);
    }
    return map;
  }

  /** Job ids of every request that reached the relayer queue. */
  jobIds(): string[] {
    return this.all.flatMap((record) =>
      [record.jobId, record.jobIdB].filter(
        (jobId): jobId is string => jobId !== undefined,
      ),
    );
  }

  async close(): Promise<void> {
    const writers = [this.writer, this.relayerAWriter, this.relayerBWriter].filter(
      (writer): writer is JsonlWriter<RequestRecord> | JsonlWriter<TargetRequestRecord> =>
        writer !== undefined,
    );
    this.writer = undefined;
    this.relayerAWriter = undefined;
    this.relayerBWriter = undefined;
    const settled = await Promise.allSettled(writers.map((writer) => writer.close()));
    const errors = settled.flatMap((result) =>
      result.status === "rejected" ? [result.reason] : [],
    );
    if (errors.length > 0) throw new AggregateError(errors, "Failed to close request recorder");
  }
}

const baseTargetRecord = (
  record: RequestRecord,
): Pick<
  TargetRequestRecord,
  "flow" | "index" | "loadStage" | "startedAtMs" | "sentRequestId"
> => ({
  flow: record.flow,
  index: record.index,
  loadStage: record.loadStage,
  startedAtMs: record.startedAtMs,
  sentRequestId: record.sentRequestId,
});

const targetRecord = (
  record: RequestRecord,
  target: "A" | "B",
): TargetRequestRecord | undefined => {
  if (target === "A") {
    return {
      ...baseTargetRecord(record),
      relayerTarget: "A",
      echoedRequestId: record.echoedRequestId,
      jobId: record.jobId,
      submitHttpStatus: record.submitHttpStatus,
      submitLatencyMs: record.submitLatencyMs,
      firstRetryAfterMs: record.firstRetryAfterMs,
      pollCount: record.pollCount,
      outcome: record.outcome,
      errorLabel: record.errorLabel,
      errorMessage: record.errorMessage,
      e2eLatencyMs: record.e2eLatencyMs,
      verified: record.verified,
    };
  }

  if (record.outcomeB === undefined) return undefined;
  return {
    ...baseTargetRecord(record),
    relayerTarget: "B",
    echoedRequestId: record.echoedRequestIdB,
    jobId: record.jobIdB,
    submitHttpStatus: record.submitHttpStatusB,
    submitLatencyMs: record.submitLatencyMsB,
    firstRetryAfterMs: record.firstRetryAfterMsB,
    pollCount: record.pollCountB ?? 0,
    outcome: record.outcomeB,
    errorLabel: record.errorLabelB,
    errorMessage: record.errorMessageB,
    e2eLatencyMs: record.e2eLatencyMsB,
    verified: record.verifiedB,
  };
};
