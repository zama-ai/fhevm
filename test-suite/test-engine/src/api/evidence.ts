import type { Redactor } from '../secrets/redactor.js';

/** Media type of the Cucumber attachments that carry evidence entries. */
export const EVIDENCE_MEDIA_TYPE = 'application/vnd.fhevm.evidence+json';

/** Payload of one evidence attachment (stored as plain text, one JSON object per attachment). */
export interface EvidencePayload {
  kind: string;
  label?: string;
  data: Record<string, unknown>;
  timestamp: string;
}

export interface RecordOptions {
  /** Human-readable label shown in reports. */
  label?: string;
}

/** Cucumber's `World.attach`, restricted to the form the evidence module uses. */
export type AttachFunction = (data: string, options: { mediaType: string }) => void | Promise<void>;

const KIND_PATTERN = /^[a-z0-9]+(?:[.-][a-z0-9]+)*$/;
const NOT_RUNNING = 'Cannot attach when a step/hook is not running';

/**
 * Records structured evidence as a Cucumber attachment on the running step or hook. The report
 * extracts these attachments into `evidence.json`; there is no second channel to keep in sync.
 */
export class Evidence {
  constructor(
    private readonly attach: AttachFunction,
    private readonly redactor: Redactor,
    private readonly now: () => Date = () => new Date(),
  ) {}

  /**
   * @param kind short namespaced kind, e.g. `engine.note`, later `chain.tx`
   * @param data JSON object; secret values are redacted before it is attached
   */
  record(kind: string, data: Record<string, unknown>, options: RecordOptions = {}): void {
    if (!KIND_PATTERN.test(kind)) {
      throw new Error(`evidence kind '${kind}' is invalid: use lowercase words separated by '.' or '-'`);
    }
    if (data === null || typeof data !== 'object' || Array.isArray(data)) {
      throw new Error('evidence data must be a JSON object');
    }
    const payload: EvidencePayload = {
      kind,
      ...(options.label !== undefined ? { label: options.label } : {}),
      data,
      timestamp: this.now().toISOString(),
    };
    const body = JSON.stringify(this.redactor.redactDeep(payload));
    try {
      void this.attach(body, { mediaType: EVIDENCE_MEDIA_TYPE });
    } catch (error) {
      if (error instanceof Error && error.message.includes(NOT_RUNNING)) {
        throw new Error('evidence.record() must be called (and awaited) inside a step or hook');
      }
      throw error;
    }
  }
}
