import type { LoadTestEnv } from "../env";
import { RelayerClient } from "../relayer/client";
import { logger } from "../shared/logger";

export type RelayerReadinessClients = Readonly<{
  primary: RelayerClient;
  candidate?: RelayerClient;
}>;

export type RelayerReadinessOptions = Readonly<{
  env: LoadTestEnv;
  connections?: number;
  skipReadiness?: boolean;
  signal?: AbortSignal;
  /** Existing clients remain owned by the caller and are not closed here. */
  clients?: RelayerReadinessClients;
}>;

const createClients = (options: RelayerReadinessOptions): RelayerReadinessClients => ({
  primary: new RelayerClient({
    baseUrl: options.env.relayerUrl,
    apiPrefix: options.env.relayerApiPrefix,
    connections: options.connections,
    apiKey: process.env.ZAMA_FHEVM_API_KEY,
  }),
  ...(options.env.relayerBUrl
    ? {
        candidate: new RelayerClient({
          baseUrl: options.env.relayerBUrl,
          apiPrefix: options.env.relayerBApiPrefix ?? options.env.relayerApiPrefix,
          connections: options.connections,
          apiKey: process.env.ZAMA_FHEVM_API_KEY,
        }),
      }
    : {}),
});

/** Reusable A/B readiness gate used before workload execution or pool writes. */
export const assertRelayerReadiness = async (
  options: RelayerReadinessOptions,
): Promise<void> => {
  options.signal?.throwIfAborted();
  if (options.skipReadiness) {
    logger.warn(
      "Skipping readiness check (--skip-readiness); assuming the relayer's v2 routes are live.",
    );
    return;
  }

  const clients = options.clients ?? createClients(options);
  const ownsClients = options.clients === undefined;
  let readinessError: unknown;
  try {
    if (!(await clients.primary.isReady())) {
      throw new Error(
        `Relayer at ${options.env.relayerUrl} failed the readiness check (GET /health/readiness). ` +
          "Older relayers expose health elsewhere (e.g. /liveness, /healthz); pass --skip-readiness to proceed.",
      );
    }
    options.signal?.throwIfAborted();
    if (clients.candidate && !(await clients.candidate.isReady())) {
      throw new Error(
        `Candidate relayer at ${options.env.relayerBUrl ?? "<unset>"} failed the readiness check (GET /health/readiness). ` +
          "Older relayers expose health elsewhere (e.g. /liveness, /healthz); pass --skip-readiness to proceed.",
      );
    }
    options.signal?.throwIfAborted();
    logger.success("Relayer readiness check passed.");
  } catch (error) {
    readinessError = error;
  }

  const cleanupErrors: unknown[] = [];
  if (ownsClients) {
    const settled = await Promise.allSettled([
      clients.primary.close(),
      ...(clients.candidate ? [clients.candidate.close()] : []),
    ]);
    cleanupErrors.push(...settled.flatMap((result) =>
      result.status === "rejected" ? [result.reason] : [],
    ));
  }
  if (readinessError && cleanupErrors.length > 0) {
    throw new AggregateError(
      [readinessError, ...cleanupErrors],
      "Relayer readiness failed and client cleanup also failed",
      { cause: readinessError },
    );
  }
  if (readinessError) throw readinessError;
  if (cleanupErrors.length > 0) {
    throw new AggregateError(cleanupErrors, "Relayer readiness client cleanup failed");
  }
};
