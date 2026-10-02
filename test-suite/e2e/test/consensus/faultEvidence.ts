import { InvalidRunError } from './validity';

/** A missing listener row is missing evidence, not an unchanged cursor. */
export function requireGatewayWatermark(value: number | null, operator: number): number {
  if (value === null || !Number.isSafeInteger(value) || value < 0) {
    throw new InvalidRunError(`operator ${operator} has no readable gateway watermark`);
  }
  return value;
}

export function assertGatewayWatermarkStopped(before: number | null, after: number | null, operator: number): void {
  const start = requireGatewayWatermark(before, operator);
  const end = requireGatewayWatermark(after, operator);
  if (end !== start) throw new Error(`operator ${operator} changed its gateway watermark while supposedly offline (${start} -> ${end})`);
}

/** The interrupted process cannot legitimately retain any lease after recovery.
 * Scope to its recorded identity: an unrelated live worker's expired lease is
 * merely eligible for production reclamation, not proof of a crash orphan.
 */
export const INTERRUPTED_WORKER_LOCKS_SQL = `
SELECT encode(dependence_chain_id, 'hex') AS chain
FROM dependence_chain WHERE worker_id = $1::uuid`;
