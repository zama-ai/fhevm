/**
 * Normalized server-side stage timestamps for one request.
 *
 * The PostgreSQL collector that originally produced these rows is intentionally
 * not part of the recovered package. The type remains at the reporting boundary
 * so a future implementation-specific observability adapter can provide the
 * same evidence without coupling the runner to a relayer database schema.
 */
export type StageRow = Readonly<{
  flow: string;
  externalJobId: string;
  status: string;
  createdAt: string;
  readinessClaimedAt?: string;
  readyAt?: string;
  claimedAt?: string;
  broadcastedAt?: string;
  gatewayRequestConfirmedAt?: string;
  completedAt?: string;
  readinessAttemptCount: number;
  broadcastAttemptCount: number;
}>;
