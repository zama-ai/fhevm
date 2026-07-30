import { signature, type Signature } from '@solana/kit';

import type { DemoSession } from './demoSession';

export type TransactionEvidenceRecord = {
  readonly label: string;
  readonly signature: Signature;
};

export type DecryptionEvidenceRecord = {
  readonly label: 'cShares' | 'cUSDC';
  readonly handle: string;
  readonly jobId: string;
  readonly queueToResponseMs: number;
  readonly totalElapsedMs: number;
  readonly completedAt: number;
};

const MAX_EVIDENCE_RECORDS = 64;
const MAX_DECRYPTION_RECORDS = 16;
const HANDLE_PATTERN = /^0x[0-9a-f]{64}$/;

const evidenceKey = (session: DemoSession): string =>
  `fhevm-solana-demo:evidence:${session.config.demoBootId}:${session.config.chainId}:${session.signer.address}`;
const decryptionEvidenceKey = (session: DemoSession): string =>
  `fhevm-solana-demo:decrypt-evidence:${session.config.demoBootId}:${session.config.chainId}:${session.signer.address}`;

export const readTransactionEvidence = (session: DemoSession): readonly TransactionEvidenceRecord[] => {
  try {
    const raw = localStorage.getItem(evidenceKey(session));
    if (raw === null) return [];
    const parsed = JSON.parse(raw) as unknown;
    if (!Array.isArray(parsed)) return [];
    return parsed.flatMap((entry): TransactionEvidenceRecord[] => {
      if (
        typeof entry !== 'object' ||
        entry === null ||
        typeof (entry as Record<string, unknown>).label !== 'string' ||
        typeof (entry as Record<string, unknown>).signature !== 'string'
      ) {
        return [];
      }
      try {
        return [{
          label: (entry as Record<string, unknown>).label as string,
          signature: signature((entry as Record<string, unknown>).signature as string),
        }];
      } catch {
        return [];
      }
    });
  } catch {
    return [];
  }
};

export const recordTransactionEvidence = (
  session: DemoSession,
  record: TransactionEvidenceRecord,
): void => {
  try {
    const records = readTransactionEvidence(session).filter(({ signature }) => signature !== record.signature);
    localStorage.setItem(evidenceKey(session), JSON.stringify([record, ...records].slice(0, MAX_EVIDENCE_RECORDS)));
  } catch {
    // Public evidence persistence is best-effort and must never affect an on-chain action.
  }
};

export const readDecryptionEvidence = (session: DemoSession): readonly DecryptionEvidenceRecord[] => {
  try {
    const raw = localStorage.getItem(decryptionEvidenceKey(session));
    if (raw === null) return [];
    const parsed = JSON.parse(raw) as unknown;
    if (!Array.isArray(parsed)) return [];
    return parsed.flatMap((entry): DecryptionEvidenceRecord[] => {
      if (typeof entry !== 'object' || entry === null) return [];
      const record = entry as Record<string, unknown>;
      if (
        (record.label !== 'cShares' && record.label !== 'cUSDC') ||
        typeof record.handle !== 'string' ||
        !HANDLE_PATTERN.test(record.handle) ||
        typeof record.jobId !== 'string' ||
        record.jobId.length === 0 ||
        typeof record.queueToResponseMs !== 'number' ||
        !Number.isFinite(record.queueToResponseMs) ||
        record.queueToResponseMs < 0 ||
        typeof record.totalElapsedMs !== 'number' ||
        !Number.isFinite(record.totalElapsedMs) ||
        record.totalElapsedMs < record.queueToResponseMs ||
        typeof record.completedAt !== 'number' ||
        !Number.isSafeInteger(record.completedAt) ||
        record.completedAt <= 0
      ) {
        return [];
      }
      return [record as DecryptionEvidenceRecord];
    });
  } catch {
    return [];
  }
};

export const recordDecryptionEvidence = (
  session: DemoSession,
  record: DecryptionEvidenceRecord,
): void => {
  try {
    const records = readDecryptionEvidence(session).filter(({ jobId }) => jobId !== record.jobId);
    localStorage.setItem(
      decryptionEvidenceKey(session),
      JSON.stringify([record, ...records].slice(0, MAX_DECRYPTION_RECORDS)),
    );
  } catch {
    // Job ids, handles, and timings are public evidence; persistence remains best-effort.
  }
};
