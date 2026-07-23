/**
 * Pluggable run-time collectors. Every collector is optional: failure to
 * start or a mid-run error degrades the report, never the run.
 */
export interface Collector {
  readonly name: string;
  start(): Promise<void>;
  stop(): Promise<void>;
}

/** One queue-depth observation, used by reports and saturation feedback. */
export type QueueDepthSample = Readonly<{
  /** Epoch ms. */
  tMs: number;
  /** Rows per flow/status. */
  byFlowStatus: Readonly<Record<string, number>>;
  /** Sum of non-terminal rows across flows. */
  pendingTotal: number;
}>;

export interface QueueDepthSource {
  readonly samples: readonly QueueDepthSample[];
}
