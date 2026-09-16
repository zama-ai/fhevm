/**
 * Structured evidence recording for the `kms-context-qa-tests` profile.
 *
 * Every observable action a QA case takes — a transaction, a view call, a decoded event, a wait, a
 * container manipulation, a test probe, an assertion — is recorded as one {@link EvidenceEntry} and
 * printed twice: progressively, one greppable line as it completes, and again in a final table.
 *
 * The rationale for recording rather than logging ad hoc: a QA profile's job is to produce a
 * defensible account of what the protocol did, not just a pass/fail. Wall-clock durations, on-chain
 * transaction hashes, block numbers and decoded ids are the difference between "the rotation
 * worked" and "the rotation landed in tx 0x9f… at block 1421 and activated 94s later, after all
 * four committee nodes reported a completed reshare".
 *
 * The CLI already has two private timing helpers — `timedLabel`/`runLogged` in
 * `src/commands/test.ts` and `timed` in `src/flow/runtime-compose.ts` — but neither is exported and
 * neither records structured fields. {@link EvidenceRecorder.step} replaces both for this profile.
 */
import type { Receipt } from "../kms-onchain";

/** Log prefix shared by every line this profile emits. */
export const EVIDENCE_PREFIX = "kms-context-qa";

/**
 * The kind of action an entry describes. Used only for display grouping and greppability, never for
 * control flow, so adding a kind is a non-breaking change.
 *
 * - `tx`     — a state-changing transaction sent by the profile.
 * - `call`   — a read-only contract view.
 * - `event`  — a decoded log emitted by a transaction.
 * - `wait`   — a poll loop against on-chain or container state.
 * - `node`   — a container start/stop.
 * - `probe`  — an e2e test run inside the test-suite container.
 * - `assert` — an invariant check that produced a verdict.
 * - `note`   — a recorded fact with no duration.
 */
export type EvidenceKind = "tx" | "call" | "event" | "wait" | "node" | "probe" | "assert" | "note";

/** One recorded action. `fields` is free-form so cases can attach whatever identifies the step. */
export type EvidenceEntry = {
  /** Monotonic across the whole profile run, so the table order is unambiguous. */
  readonly seq: number;
  readonly caseId: string;
  readonly kind: EvidenceKind;
  readonly label: string;
  /** `Date.now()` at the start of the step. */
  readonly startedAtMs: number;
  /** Wall-clock duration; `0` for notes. */
  readonly durationMs: number;
  /** False when the step threw. The entry is still recorded, then the error is rethrown. */
  readonly ok: boolean;
  readonly fields: Readonly<Record<string, string>>;
};

/**
 * `cast send --json` returns `transactionHash`, `blockNumber`, `gasUsed` and `effectiveGasPrice`
 * alongside the `status`/`logs` that {@link Receipt} models. The exported type deliberately narrows
 * them away, so widen structurally at the read site rather than changing the shared type in
 * `src/kms-onchain.ts`.
 */
type CastSendReceiptDetails = Receipt &
  Partial<{
    transactionHash: string;
    blockNumber: string | number;
    gasUsed: string | number;
    effectiveGasPrice: string | number;
  }>;

/**
 * Renders a numeric receipt field as decimal, accepting the hex (`0x…`) or decimal forms `cast`
 * may emit. Returns undefined for anything unparsable, so evidence never fails a run.
 */
const decimalOrUndefined = (value: string | number | undefined): string | undefined => {
  if (value === undefined || value === null) return undefined;
  if (typeof value === "number") return Number.isFinite(value) ? String(value) : undefined;
  const trimmed = value.trim();
  if (!trimmed) return undefined;
  try {
    return BigInt(trimmed).toString();
  } catch {
    return undefined;
  }
};

/**
 * Extracts printable transaction evidence from a `cast send` receipt.
 *
 * Pure and total: missing or malformed fields are omitted rather than throwing, because evidence
 * gathering must never be the reason a QA run fails. Exported for unit testing.
 */
export const txEvidenceFields = (receipt: Receipt): Record<string, string> => {
  const details = receipt as CastSendReceiptDetails;
  const fields: Record<string, string> = {};
  if (details.transactionHash) fields.txHash = details.transactionHash;
  const block = decimalOrUndefined(details.blockNumber);
  if (block) fields.block = block;
  const gasUsed = decimalOrUndefined(details.gasUsed);
  if (gasUsed) fields.gasUsed = gasUsed;
  const gasPrice = decimalOrUndefined(details.effectiveGasPrice);
  if (gasPrice) fields.gasPrice = gasPrice;
  if (details.status) fields.status = details.status;
  fields.logCount = String(receipt.logs?.length ?? 0);
  return fields;
};

/**
 * Renders a duration the way the rest of the CLI does: whole seconds with one decimal, and
 * sub-second values in milliseconds so fast view calls stay legible. Pure; exported for testing.
 */
export const formatDuration = (durationMs: number): string =>
  durationMs < 1000 ? `${Math.round(durationMs)}ms` : `${(durationMs / 1000).toFixed(1)}s`;

/** Renders a field map as `key=value` pairs in insertion order. Pure; exported for testing. */
export const formatFields = (fields: Readonly<Record<string, string>>): string =>
  Object.entries(fields)
    .map(([key, value]) => `${key}=${value}`)
    .join(" ");

/** Renders one entry as a single greppable line. Pure; exported for testing. */
export const formatEvidenceLine = (entry: EvidenceEntry): string => {
  const verdict = entry.ok ? "ok" : "FAILED";
  const timing = entry.kind === "note" ? "" : ` ${formatDuration(entry.durationMs)}`;
  const fields = formatFields(entry.fields);
  return (
    `[${EVIDENCE_PREFIX}][${entry.caseId}][${entry.kind}] ${entry.label} ${verdict}${timing}` +
    (fields ? ` ${fields}` : "")
  );
};

/**
 * Renders the final evidence table: one line per entry, numbered, with durations and fields.
 *
 * Pure and independent of the wall clock — durations come from the entries — so it can be unit
 * tested against fixtures. Exported for testing.
 */
export const formatEvidenceTable = (entries: readonly EvidenceEntry[]): string => {
  if (!entries.length) return "(no evidence recorded)";
  const seqWidth = String(entries[entries.length - 1]!.seq).length;
  return entries
    .map((entry) => {
      const seq = String(entry.seq).padStart(seqWidth, " ");
      const verdict = entry.ok ? "ok    " : "FAILED";
      const duration = (entry.kind === "note" ? "-" : formatDuration(entry.durationMs)).padStart(7, " ");
      const fields = formatFields(entry.fields);
      return `  ${seq}  ${verdict}  ${duration}  ${entry.caseId}/${entry.kind}  ${entry.label}${fields ? `  ${fields}` : ""}`;
    })
    .join("\n");
};

/**
 * Per-case view of the recorder. Cases receive one of these and never see the global recorder, so a
 * case cannot accidentally attribute evidence to another case.
 */
export type CaseEvidence = {
  /**
   * Times `task`, records it, prints one line, and returns the task's value.
   *
   * On throw the entry is still recorded with `ok: false` and printed before the error propagates —
   * a failed run must leave the same audit trail as a successful one, up to the point of failure.
   */
  step<T>(
    kind: EvidenceKind,
    label: string,
    fields: Record<string, string>,
    task: () => Promise<T>,
  ): Promise<T>;
  /** Records a zero-duration fact, such as a decoded value or a resolved address. */
  note(kind: EvidenceKind, label: string, fields?: Record<string, string>): void;
};

/**
 * Collects evidence for a whole profile run.
 *
 * Construct one per run, call {@link EvidenceRecorder.forCase} per case, and
 * {@link EvidenceRecorder.renderSummary} at the end (including on failure).
 */
export class EvidenceRecorder {
  readonly #entries: EvidenceEntry[] = [];
  #seq = 0;

  /** Every entry recorded so far, in order. */
  get entries(): readonly EvidenceEntry[] {
    return this.#entries;
  }

  /** Entries belonging to one case, in order. */
  entriesForCase(caseId: string): readonly EvidenceEntry[] {
    return this.#entries.filter((entry) => entry.caseId === caseId);
  }

  /** Total wall-clock time recorded for one case, summed over its timed steps. */
  durationForCase(caseId: string): number {
    return this.entriesForCase(caseId).reduce((total, entry) => total + entry.durationMs, 0);
  }

  #record(entry: Omit<EvidenceEntry, "seq">): EvidenceEntry {
    const recorded: EvidenceEntry = { ...entry, seq: ++this.#seq };
    this.#entries.push(recorded);
    console.log(formatEvidenceLine(recorded));
    return recorded;
  }

  /** Returns a case-scoped recorder that stamps every entry with `caseId`. */
  forCase(caseId: string): CaseEvidence {
    return {
      step: async <T>(
        kind: EvidenceKind,
        label: string,
        fields: Record<string, string>,
        task: () => Promise<T>,
      ): Promise<T> => {
        const startedAtMs = Date.now();
        try {
          const result = await task();
          this.#record({
            caseId,
            kind,
            label,
            startedAtMs,
            durationMs: Date.now() - startedAtMs,
            ok: true,
            fields,
          });
          return result;
        } catch (error) {
          this.#record({
            caseId,
            kind,
            label,
            startedAtMs,
            durationMs: Date.now() - startedAtMs,
            ok: false,
            fields,
          });
          throw error;
        }
      },
      note: (kind: EvidenceKind, label: string, fields: Record<string, string> = {}) => {
        this.#record({
          caseId,
          kind,
          label,
          startedAtMs: Date.now(),
          durationMs: 0,
          ok: true,
          fields,
        });
      },
    };
  }

  /** The full evidence table, ready to print at the end of a run. */
  renderSummary(): string {
    return [
      `[${EVIDENCE_PREFIX}] evidence (${this.#entries.length} step(s)):`,
      formatEvidenceTable(this.#entries),
    ].join("\n");
  }
}
