import { readFile } from "node:fs/promises";

import { logger } from "../shared/logger";
import { redactConfigText } from "../shared/safe-artifact";

/**
 * Relayer config snapshot. Throughput is config-capped
 * (`max_broadcasts_in_flight`, readiness `max_concurrency`, pool size, ...),
 * so a report without the config in effect is uninterpretable. The relayer
 * exposes no config endpoint; the file is read from a caller-supplied path
 * (e.g. a checkout of the deploy repo) and embedded after secret redaction.
 */
export type RelayerConfigSnapshot = Readonly<{
  path: string;
  raw: string;
}>;

export const snapshotRelayerConfig = async (
  path: string | undefined,
): Promise<RelayerConfigSnapshot | undefined> => {
  if (!path) return undefined;
  try {
    return { path, raw: redactConfigText(await readFile(path, "utf8")) };
  } catch (error) {
    logger.warn(`Could not snapshot relayer config at ${path}: ${(error as Error).message}`);
    return undefined;
  }
};
