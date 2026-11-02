/** Evidence as it travels from the scenario worker (Cucumber attachments) to the reports. */

/** Media type of the Cucumber attachments that carry evidence entries. */
export const EVIDENCE_MEDIA_TYPE = 'application/vnd.fhevm.evidence+json';

/** Payload of one evidence attachment (stored as plain text, one JSON object per attachment). */
export interface EvidencePayload {
  kind: string;
  label?: string;
  data: Record<string, unknown>;
  timestamp: string;
}
