/**
 * Shared kms-connector database inspection for the KMS acceptance profiles. Each party runs its own
 * connector (gw-listener + kms-worker + tx-sender) backed by its own database in the shared
 * postgres container; these helpers run a scalar query against one party's database and poll a
 * query across every party until it settles on an expected value.
 */
import { PreflightError } from "./errors";
import { kmsConnectorDbName, kmsPartyIds } from "./kms-party";
import { uint256LeHex } from "./kms-onchain";
import { COPROCESSOR_DB_CONTAINER, DEFAULT_POSTGRES_PASSWORD, DEFAULT_POSTGRES_USER } from "./layout";
import { run } from "./utils/process";

/** Default bound for the connector to ingest an event, forward it to KMS Core, and settle. */
const CONNECTOR_SETTLE_TIMEOUT_MS = 240_000;
const CONNECTOR_POLL_MS = 2_000;

const connectorDbRuntime = () => ({
  container: process.env.POSTGRES_CONTAINER ?? COPROCESSOR_DB_CONTAINER,
  user: process.env.POSTGRES_USER ?? DEFAULT_POSTGRES_USER,
  password: process.env.POSTGRES_PASSWORD ?? DEFAULT_POSTGRES_PASSWORD,
});

/** Runs a scalar query against one party's kms-connector database. */
const connectorQuery = async (dbName: string, sql: string) => {
  const db = connectorDbRuntime();
  const result = await run([
    "docker", "exec", "-e", `PGPASSWORD=${db.password}`, db.container,
    "psql", "-U", db.user, "-d", dbName, "-t", "-A", "-c", sql,
  ]);
  return result.stdout.trim();
};

/** A bytea literal for a U256 id in the little-endian form the connector stores. */
const byteaLiteral = (id: bigint) => `decode('${uint256LeHex(id)}','hex')`;

/** A column's value as text, or 'missing' when the row does not exist. Booleans read as 't'/'f'. */
export const columnQuery = (table: string, idColumn: string, column: string, id: bigint) =>
  `SELECT COALESCE((SELECT ${column}::text FROM ${table} WHERE ${idColumn} = ${byteaLiteral(id)}), 'missing')`;

/** Polls `columnValueQuery` in each listed party's connector DB until it returns one of
 * `expectedValues` everywhere, or throws after the timeout with the last value seen.
 * `parties` is a party count (poll parties 1..N) or an explicit party-id list — the profiles use a
 * list when different parties expect different outcomes (e.g. a spare that never held a context).
 * `checkDescription` names the check in logs and in the timeout error. */
export const checkConnectorsDbColumn = async (
  parties: number | number[],
  checkDescription: string,
  columnValueQuery: string,
  expectedValues: string[],
  opts?: { timeoutMs?: number; pollMs?: number },
) => {
  const partyIds = Array.isArray(parties) ? parties : kmsPartyIds(parties);
  const timeoutMs = opts?.timeoutMs ?? CONNECTOR_SETTLE_TIMEOUT_MS;
  const pollMs = opts?.pollMs ?? CONNECTOR_POLL_MS;
  const settledValues: string[] = [];
  for (const party of partyIds) {
    const dbName = kmsConnectorDbName(party);
    const deadline = Date.now() + timeoutMs;
    let lastValue = await connectorQuery(dbName, columnValueQuery);
    while (!expectedValues.includes(lastValue)) {
      const isCheckTimedOut = Date.now() >= deadline;
      if (isCheckTimedOut) {
        throw new PreflightError(
          `${checkDescription}: db "${dbName}" returned ${JSON.stringify(lastValue)} (expected one of ${expectedValues.join("/")}) after ${timeoutMs / 1000}s — query: ${columnValueQuery}`,
        );
      }
      await Bun.sleep(pollMs);
      lastValue = await connectorQuery(dbName, columnValueQuery);
    }
    settledValues.push(lastValue);
  }
  console.log(
    `[connector] check OK on ${partyIds.length} db(s) (parties ${partyIds.join(",")}): ${checkDescription} -> ${settledValues.join(", ")}`,
  );
};
