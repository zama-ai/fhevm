import { run } from "./utils/process";

export const DRIFT_INSTALL_SQL = `CREATE TABLE IF NOT EXISTS drift_injection_state (
  id BOOLEAN PRIMARY KEY DEFAULT TRUE,
  enabled BOOLEAN NOT NULL,
  consumed BOOLEAN NOT NULL DEFAULT FALSE,
  injected_handle BYTEA
);

INSERT INTO drift_injection_state (id, enabled, consumed, injected_handle)
VALUES (TRUE, TRUE, FALSE, NULL)
ON CONFLICT (id) DO UPDATE
SET enabled = EXCLUDED.enabled,
    consumed = EXCLUDED.consumed,
    injected_handle = EXCLUDED.injected_handle;

CREATE OR REPLACE FUNCTION inject_ciphertext_drift_once()
RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
  should_inject BOOLEAN;
BEGIN
  SELECT enabled AND NOT consumed
  INTO should_inject
  FROM drift_injection_state
  WHERE id = TRUE;

  IF NOT COALESCE(should_inject, FALSE) THEN
    RETURN NEW;
  END IF;

  IF NEW.txn_is_sent = FALSE
     AND NEW.ciphertext IS NOT NULL
     AND NEW.ciphertext128 IS NOT NULL
     AND (OLD.ciphertext IS NULL OR OLD.ciphertext128 IS NULL)
     AND EXISTS (SELECT 1 FROM computations WHERE output_handle = NEW.handle) THEN
    NEW.ciphertext := set_byte(NEW.ciphertext, 0, get_byte(NEW.ciphertext, 0) # 1);

    UPDATE drift_injection_state
    SET consumed = TRUE,
        injected_handle = NEW.handle
    WHERE id = TRUE;
  END IF;

  RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS ciphertext_drift_injector ON ciphertext_digest;

CREATE TRIGGER ciphertext_drift_injector
BEFORE UPDATE ON ciphertext_digest
FOR EACH ROW
EXECUTE FUNCTION inject_ciphertext_drift_once();
`;

export const DRIFT_CLEANUP_SQL = `DROP TRIGGER IF EXISTS ciphertext_drift_injector ON ciphertext_digest;
DROP FUNCTION IF EXISTS inject_ciphertext_drift_once();
DROP TABLE IF EXISTS drift_injection_state;
`;

/** Parses a coprocessor instance index from env or CLI input. */
export const parseDriftInstanceIndex = (value: string) => {
  if (!/^\d+$/.test(value)) {
    throw new Error("instance index must be a non-negative integer");
  }
  return Number(value);
};

/** Parses a positive integer environment setting used by the drift test. */
export const parsePositiveInteger = (value: string, name: string) => {
  if (!/^\d+$/.test(value) || Number(value) <= 0) {
    throw new Error(`${name} must be a positive integer`);
  }
  return Number(value);
};

// Passive drift observation. The two warnings below are emitted by gw-listener during normal
// operation, with no injection involved: the first whenever it sees peers submit differing
// digests for one handle, the second when a coprocessor's own digest loses to consensus. The
// `ciphertext-drift` profiles force these by corrupting a ciphertext; the scan here never
// touches the stack, it only reads what already happened. That distinction is the point — a
// rollout wants to know whether a version-mixed fleet diverges on its own, which an injected
// fault cannot answer.
const PEER_DIVERGENCE_WARNING = '"message":"Drift detected: observed multiple digest variants for handle"';
const CONSENSUS_MISMATCH_WARNING = '"message":"Drift detected: local digest does not match consensus"';
const DRIFT_NEEDLES = [
  { needle: PEER_DIVERGENCE_WARNING, kind: "peer-divergence" as const },
  { needle: CONSENSUS_MISMATCH_WARNING, kind: "consensus-mismatch" as const },
];
const DRIFT_HANDLE_PATTERN = /"handle":"0x([0-9a-f]+)"/i;
const GW_LISTENER_NAME_PATTERN = /^coprocessor(\d+)?-gw-listener$/;

export type DriftKind = (typeof DRIFT_NEEDLES)[number]["kind"];

export interface DriftObservation {
  readonly container: string;
  readonly kind: DriftKind;
  readonly handle: string | undefined;
  readonly line: string;
}

export interface DriftScanResult {
  readonly containers: readonly string[];
  readonly observations: readonly DriftObservation[];
}

/** Extracts every drift warning present in one container's log output. */
export const findDriftObservations = (container: string, output: string): DriftObservation[] => {
  const observations: DriftObservation[] = [];
  for (const line of output.split(/\r?\n/)) {
    for (const { needle, kind } of DRIFT_NEEDLES) {
      if (!line.includes(needle)) {
        continue;
      }
      observations.push({ container, kind, handle: line.match(DRIFT_HANDLE_PATTERN)?.[1], line: line.trim() });
    }
  }
  return observations;
};

/** Filters `docker ps` output down to the running gateway-listener containers. */
export const gwListenerContainerNames = (dockerPsOutput: string): string[] =>
  dockerPsOutput
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => GW_LISTENER_NAME_PATTERN.test(line));

/**
 * Reads every running gateway listener's log since `since` and collects the drift warnings.
 *
 * A scan with no listeners running is reported as such rather than as a clean result: "nobody
 * was watching" and "everybody agreed" are the same empty list, and only one of them is evidence.
 */
export const scanForDrift = async (since: string): Promise<DriftScanResult> => {
  const listed = await run(["docker", "ps", "--format", "{{.Names}}"], { allowFailure: true });
  if (listed.code !== 0) {
    throw new Error(listed.stderr.trim() || "docker ps failed");
  }
  const containers = gwListenerContainerNames(listed.stdout);
  if (containers.length === 0) {
    throw new Error("drift scan found no running gateway listeners; a clean scan would prove nothing");
  }
  const observations: DriftObservation[] = [];
  for (const container of containers) {
    const logs = await run(["docker", "logs", "--since", since, container], { allowFailure: true });
    observations.push(...findDriftObservations(container, logs.stdout + logs.stderr));
  }
  return { containers, observations };
};

/** Renders a scan result as the one line a rollout receipt carries. */
export const formatDriftScan = (label: string, result: DriftScanResult) => {
  const scope = `${result.containers.length} listener${result.containers.length === 1 ? "" : "s"}`;
  if (result.observations.length === 0) {
    return `drift scan ${label}: clean across ${scope}`;
  }
  const handles = [...new Set(result.observations.map((entry) => entry.handle ?? "unknown"))];
  return `drift scan ${label}: ${result.observations.length} warning(s) across ${scope}, handles ${handles.join(", ")}`;
};
