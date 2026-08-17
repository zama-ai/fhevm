import type { RelayerLegRecord } from "./types";

export const interruptedLeg = (
  signal: AbortSignal,
  progressType?: string,
): Pick<RelayerLegRecord, "outcome" | "errorLabel"> | undefined => {
  if (signal.aborted || progressType === "abort") {
    return { outcome: "aborted", errorLabel: "client_aborted" };
  }
  if (progressType === "timeout") {
    return { outcome: "timed_out", errorLabel: "client_poll_deadline_exceeded" };
  }
  return undefined;
};
