/** Result of the preflight checks (`src/boot/preflight.ts`), shared by the CLI and the report. */

export interface PreflightCheck {
  /** `chain.rpc`, `relayer.keyurl` or `relayer.fhe-key`. */
  name: string;
  /** What was checked (a URL; secrets redacted before it is printed or stored). */
  target: string;
  status: 'passed' | 'failed' | 'skipped';
  durationMs: number;
  /** Why it failed or was skipped, or a short note when it passed. */
  detail?: string;
}

export interface PreflightResult {
  status: 'passed' | 'failed' | 'skipped';
  /** Why the preflight was skipped. */
  reason?: string;
  checks: PreflightCheck[];
}
