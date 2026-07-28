import { signature, type Signature } from '@solana/kit';

import type { DemoSession } from './demoSession';

export type TransactionEvidenceRecord = {
  readonly label: string;
  readonly signature: Signature;
};

const MAX_EVIDENCE_RECORDS = 64;

const evidenceKey = (session: DemoSession): string =>
  `fhevm-solana-demo:evidence:${session.config.demoBootId}:${session.config.chainId}:${session.signer.address}`;

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
