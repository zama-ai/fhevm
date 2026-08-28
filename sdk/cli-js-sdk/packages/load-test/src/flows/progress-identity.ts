type ProgressIdentity = Readonly<{ requestId: string; jobId: string }>;

type SdkProgress = Readonly<{
  type: string;
  method?: string;
  requestId?: string;
  jobId?: string;
}>;

/** Captures only the first POST acceptance; queued GETs are polling events. */
export const captureInitialPostIdentity = (
  current: ProgressIdentity | undefined,
  progress: SdkProgress,
): ProgressIdentity | undefined => {
  if (current || progress.type !== "queued" || progress.method !== "POST") return current;
  return typeof progress.requestId === "string" && typeof progress.jobId === "string"
    ? { requestId: progress.requestId, jobId: progress.jobId }
    : undefined;
};

/**
 * A successful SDK result is trusted only when it belongs to the accepted job.
 * `requestId` identifies an individual HTTP exchange: some relayers echo a
 * caller-provided correlation id, while others generate a fresh id per
 * response. The POST's `jobId` is the stable asynchronous operation identity.
 */
export const sdkTerminalIdentityError = (
  initial: ProgressIdentity | undefined,
  terminal: SdkProgress | undefined,
): string | undefined => {
  if (!initial) return "SDK did not report an initial POST acceptance identity.";
  if (terminal?.type !== "succeeded") {
    return "SDK returned clear values without terminal success provenance.";
  }
  if (typeof terminal.requestId !== "string") {
    return "SDK terminal success did not report an HTTP request identity.";
  }
  if (terminal.jobId !== initial.jobId) {
    return "SDK terminal progress job identity did not match the initial POST acceptance.";
  }
  return undefined;
};
